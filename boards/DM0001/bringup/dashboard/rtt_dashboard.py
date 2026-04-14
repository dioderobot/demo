#!/usr/bin/env python3
from __future__ import annotations

import argparse
import curses
import json
import os
import queue
import re
import signal
import subprocess
import sys
import threading
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent.parent
DEFAULT_ELF = ROOT / "firmware/target/thumbv7em-none-eabihf/release/dm0001-bringup"
DEFAULT_PSU_MODULE = Path(
    os.environ.get("DPS150_API_DIR", str(Path.home() / "GitHub" / "dps150_api"))
)
DEFAULT_PSU_PORT = "/dev/cu.usbmodem065AD9D205B31"
DEFAULT_POLL_S = 2.0
TELEMETRY_RE = re.compile(r"telemetry=(\{.*\})")


@dataclass
class DashboardState:
    telemetry: dict[str, Any] | None = None
    telemetry_ts: float = 0.0
    raw_line: str = ""
    probe_status: str = "starting"
    probe_cmd: str = ""
    psu: dict[str, Any] = field(default_factory=dict)
    psu_ts: float = 0.0
    message: str = ""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="DM0001 RTT telemetry dashboard")
    parser.add_argument("--elf", type=Path, default=DEFAULT_ELF, help="Path to the flashed ELF")
    parser.add_argument(
        "--mode",
        choices=("attach", "run"),
        default="attach",
        help="Use `attach` for an already running board or `run` to flash and reset before viewing",
    )
    parser.add_argument("--probe", default="probe-rs", help="probe-rs executable")
    parser.add_argument("--chip", default="STM32G431C8", help="Target chip")
    parser.add_argument("--probe-speed", type=int, default=None, help="Optional SWD speed in kHz")
    parser.add_argument("--plain", action="store_true", help="Print parsed updates instead of the curses UI")
    parser.add_argument("--duration", type=float, default=None, help="Optional auto-exit timeout in seconds")
    parser.add_argument("--psu", action="store_true", help="Enable DPS-150 polling for live supply power data")
    parser.add_argument("--psu-port", default=DEFAULT_PSU_PORT, help="DPS-150 serial device path")
    parser.add_argument("--psu-module-path", type=Path, default=DEFAULT_PSU_MODULE, help="Path to dps150_api repo")
    return parser.parse_args()


def parse_atom(text: str) -> Any:
    text = text.strip()
    if text == "true":
        return True
    if text == "false":
        return False
    if text.startswith("[") and text.endswith("]"):
        inner = text[1:-1].strip()
        if not inner:
            return []
        return [parse_atom(part) for part in split_top_level(inner)]
    if re.fullmatch(r"-?\d+", text):
        return int(text)
    return text


def split_top_level(text: str) -> list[str]:
    parts: list[str] = []
    depth = 0
    start = 0
    for idx, ch in enumerate(text):
        if ch in "[{":
            depth += 1
        elif ch in "]}":
            depth -= 1
        elif ch == "," and depth == 0:
            parts.append(text[start:idx].strip())
            start = idx + 1
    parts.append(text[start:].strip())
    return [part for part in parts if part]


def parse_telemetry(text: str) -> dict[str, Any]:
    body = text.strip()
    if not (body.startswith("{") and body.endswith("}")):
        raise ValueError(f"unexpected telemetry body: {body!r}")
    items = split_top_level(body[1:-1])
    parsed: dict[str, Any] = {}
    for item in items:
        key, value = item.split(":", 1)
        parsed[key.strip()] = parse_atom(value)
    return parsed


def fmt_mv(value: Any) -> str:
    if not isinstance(value, int):
        return "n/a"
    return f"{value / 1000:.3f} V"


def fmt_ma(value: Any) -> str:
    if not isinstance(value, int):
        return "n/a"
    return f"{value:+d} mA"


def fmt_mc(value: Any) -> str:
    if not isinstance(value, int):
        return "n/a"
    return f"{value / 1000:.1f} C"


def fmt_bool(value: Any) -> str:
    if value is True:
        return "yes"
    if value is False:
        return "no"
    return "n/a"


def list_triplet(value: Any, formatter) -> tuple[str, str, str]:
    if isinstance(value, list) and len(value) == 3:
        return tuple(formatter(v) for v in value)
    return ("n/a", "n/a", "n/a")


def estimate_phase_abs_current_ma(telemetry: dict[str, Any] | None) -> int | None:
    if telemetry is None:
        return None
    currents = telemetry.get("current_ma")
    if not isinstance(currents, list) or len(currents) != 3:
        return None
    return sum(abs(int(v)) for v in currents)


def safe_float(value: Any) -> float | None:
    if isinstance(value, (int, float)):
        return float(value)
    return None


def build_probe_command(args: argparse.Namespace) -> list[str]:
    cmd = [args.probe, args.mode, "--chip", args.chip, "--log-format", "{t} {L} {s}"]
    if args.probe_speed is not None:
        cmd.extend(["--speed", str(args.probe_speed)])
    cmd.append(str(args.elf))
    return cmd


def probe_reader(state: DashboardState, out_queue: queue.Queue[str], stop: threading.Event, args: argparse.Namespace) -> None:
    cmd = build_probe_command(args)
    state.probe_cmd = " ".join(cmd)
    process = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        stdin=subprocess.DEVNULL,
        text=True,
        bufsize=1,
    )
    state.probe_status = "running"
    try:
        assert process.stdout is not None
        for line in process.stdout:
            if stop.is_set():
                break
            out_queue.put(line.rstrip("\n"))
        rc = process.wait(timeout=2)
        if not stop.is_set():
            state.probe_status = f"exited ({rc})"
    except Exception as exc:  # pragma: no cover - defensive
        state.probe_status = f"probe error: {exc}"
    finally:
        if process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                process.kill()
        state.probe_status = "stopped" if stop.is_set() else state.probe_status


def psu_reader(state: DashboardState, stop: threading.Event, args: argparse.Namespace) -> None:
    while not stop.is_set():
        try:
            update_psu_state(state, read_psu_state_subprocess(args))
        except Exception as exc:  # pragma: no cover - hardware dependent
            state.message = f"PSU read failed: {exc}"
        stop.wait(DEFAULT_POLL_S)


def update_psu_state(state: DashboardState, raw: dict[str, Any]) -> None:
    voltage = safe_float(raw.get("output_voltage"))
    current = safe_float(raw.get("output_current"))
    power_w = voltage * current if voltage is not None and current is not None else None
    state.psu = {
        "output_on": raw.get("output_on"),
        "output_voltage": voltage,
        "output_current": current,
        "power_w": power_w,
        "protection": raw.get("protection"),
        "current_setpoint": safe_float(raw.get("current_setpoint")),
        "voltage_setpoint": safe_float(raw.get("voltage_setpoint")),
    }
    state.psu_ts = time.time()
    state.message = ""


def read_psu_state_subprocess(args: argparse.Namespace) -> dict[str, Any]:
    script = (
        "import json, sys\n"
        f"sys.path.insert(0, {str(args.psu_module_path)!r})\n"
        "from dps150 import DPS150\n"
        f"psu = DPS150({args.psu_port!r})\n"
        "psu.connect()\n"
        "try:\n"
        "    state = psu.read_state() or {}\n"
        "finally:\n"
        "    psu.close()\n"
        "print(json.dumps(state))\n"
    )
    proc = subprocess.run(
        [sys.executable, "-c", script],
        capture_output=True,
        text=True,
        timeout=6,
        check=True,
    )
    return json.loads(proc.stdout)


def render_line(stdscr: curses.window, row: int, text: str, attr: int = 0) -> int:
    height, width = stdscr.getmaxyx()
    if 0 <= row < height:
        stdscr.addnstr(row, 0, text.ljust(width - 1), width - 1, attr)
    return row + 1


def render_dashboard(stdscr: curses.window, state: DashboardState) -> None:
    stdscr.erase()
    now = time.time()
    telemetry = state.telemetry
    seq = telemetry.get("seq") if telemetry else "n/a"
    age = f"{now - state.telemetry_ts:0.2f}s" if state.telemetry_ts else "n/a"
    row = 0
    row = render_line(stdscr, row, "DM0001 RTT Dashboard", curses.A_BOLD)
    row = render_line(
        stdscr,
        row,
        f"probe={state.probe_status}  seq={seq}  age={age}  cmd={state.probe_cmd}",
    )
    if state.message:
        row = render_line(stdscr, row, f"note={state.message}")
    row += 1

    state_name = telemetry.get("state", "n/a") if telemetry else "n/a"
    arm_ready = fmt_bool(telemetry.get("arm_ready")) if telemetry else "n/a"
    row = render_line(
        stdscr,
        row,
        f"state={state_name}  arm_ready={arm_ready}  bemf_gpio={fmt_bool(telemetry.get('bemf_gpio') if telemetry else None)}  hall={telemetry.get('hall', 'n/a') if telemetry else 'n/a'}",
    )

    row = render_line(
        stdscr,
        row,
        f"bus={fmt_mv(telemetry.get('bus_mv') if telemetry else None)}  vdda={fmt_mv(telemetry.get('vdda_mv') if telemetry else None)}  ntc={fmt_mv(telemetry.get('ntc_mv') if telemetry else None)}  ntc_r={telemetry.get('ntc_ohms', 'n/a') if telemetry else 'n/a'} ohm  mcu={fmt_mc(telemetry.get('mcu_temp_mc') if telemetry else None)}",
    )

    current_triplet = list_triplet(telemetry.get("current_ma") if telemetry else None, fmt_ma)
    current_mv_triplet = list_triplet(telemetry.get("current_out_mv") if telemetry else None, fmt_mv)
    bemf_triplet = list_triplet(telemetry.get("bemf_mv") if telemetry else None, fmt_mv)
    row = render_line(
        stdscr,
        row,
        f"current A/B/C={current_triplet[0]}  {current_triplet[1]}  {current_triplet[2]}",
    )
    row = render_line(
        stdscr,
        row,
        f"current_out A/B/C={current_mv_triplet[0]}  {current_mv_triplet[1]}  {current_mv_triplet[2]}",
    )
    row = render_line(
        stdscr,
        row,
        f"bemf A/B/C={bemf_triplet[0]}  {bemf_triplet[1]}  {bemf_triplet[2]}",
    )

    phase_abs_ma = estimate_phase_abs_current_ma(telemetry)
    psu = state.psu
    supply_v = psu.get("output_voltage")
    supply_i = psu.get("output_current")
    supply_p = psu.get("power_w")
    row += 1
    row = render_line(stdscr, row, "Power / Motion", curses.A_BOLD)
    row = render_line(
        stdscr,
        row,
        f"psu_on={fmt_bool(psu.get('output_on'))}  psu_v={f'{supply_v:.3f} V' if supply_v is not None else 'n/a'}  psu_i={f'{supply_i * 1000:.1f} mA' if supply_i is not None else 'n/a'}  psu_p={f'{supply_p:.3f} W' if supply_p is not None else 'n/a'}  protection={psu.get('protection', 'n/a')}",
    )
    row = render_line(
        stdscr,
        row,
        f"duty=n/a  rpm=n/a  commutation={state_name if state_name in ('armed', 'disarmed') else 'n/a'}  abs_phase_current={f'{phase_abs_ma} mA' if phase_abs_ma is not None else 'n/a'}",
    )
    row = render_line(stdscr, row, f"raw={state.raw_line[: max(0, stdscr.getmaxyx()[1] - 6)]}")
    row += 1
    render_line(stdscr, row, "Controls: q to quit", curses.A_DIM)
    stdscr.refresh()


def main() -> int:
    args = parse_args()
    if not args.elf.exists():
        print(f"ELF not found: {args.elf}", file=sys.stderr)
        return 2

    state = DashboardState()
    stop = threading.Event()
    out_queue: queue.Queue[str] = queue.Queue()
    deadline = time.time() + args.duration if args.duration is not None else None

    probe_thread = threading.Thread(target=probe_reader, args=(state, out_queue, stop, args), daemon=True)
    probe_thread.start()

    psu_thread: threading.Thread | None = None
    if args.psu:
        try:
            update_psu_state(state, read_psu_state_subprocess(args))
        except Exception as exc:
            state.message = f"PSU startup read failed: {exc}"
        psu_thread = threading.Thread(target=psu_reader, args=(state, stop, args), daemon=True)
        psu_thread.start()

    def _handle_signal(_signum: int, _frame: Any) -> None:
        stop.set()

    signal.signal(signal.SIGINT, _handle_signal)
    signal.signal(signal.SIGTERM, _handle_signal)

    def drain_lines() -> None:
        while True:
            try:
                line = out_queue.get_nowait()
            except queue.Empty:
                return
            state.raw_line = line
            match = TELEMETRY_RE.search(line)
            if match:
                try:
                    state.telemetry = parse_telemetry(match.group(1))
                    state.telemetry_ts = time.time()
                except Exception as exc:
                    state.message = f"parse error: {exc}"

    try:
        if args.plain:
            last_seq: int | None = None
            last_message = ""
            while not stop.is_set():
                drain_lines()
                if deadline is not None and time.time() >= deadline:
                    break
                if state.message and state.message != last_message:
                    print(f"note={state.message}", flush=True)
                    last_message = state.message
                telemetry = state.telemetry
                if telemetry:
                    seq = telemetry.get("seq")
                    if isinstance(seq, int) and seq != last_seq:
                        last_seq = seq
                        psu = state.psu
                        psu_v = psu.get("output_voltage")
                        psu_i = psu.get("output_current")
                        print(
                            "seq={seq} state={state} arm_ready={arm} bus={bus} currents={currents} ntc={ntc} mcu={mcu} psu={pv}/{pi}".format(
                                seq=seq,
                                state=telemetry.get("state"),
                                arm=telemetry.get("arm_ready"),
                                bus=telemetry.get("bus_mv"),
                                currents=telemetry.get("current_ma"),
                                ntc=telemetry.get("ntc_mv"),
                                mcu=telemetry.get("mcu_temp_mc"),
                                pv=f"{psu_v:.3f}V" if psu_v is not None else "n/a",
                                pi=f"{psu_i * 1000:.1f}mA" if psu_i is not None else "n/a",
                            )
                            ,
                            flush=True,
                        )
                time.sleep(0.1)
        else:
            def wrapped(stdscr: curses.window) -> None:
                curses.curs_set(0)
                stdscr.nodelay(True)
                while not stop.is_set():
                    drain_lines()
                    render_dashboard(stdscr, state)
                    if deadline is not None and time.time() >= deadline:
                        break
                    key = stdscr.getch()
                    if key in (ord("q"), ord("Q")):
                        break
                    time.sleep(0.1)

            curses.wrapper(wrapped)
    finally:
        stop.set()
        probe_thread.join(timeout=2)
        if psu_thread is not None:
            psu_thread.join(timeout=2)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
