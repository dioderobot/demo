#!/usr/bin/env python3
import argparse
import os
import re
import shutil
import struct
import subprocess
import sys
import time


STATE_NAMES = {
    0: "IDLE",
    2: "ALIGNMENT",
    4: "START",
    6: "RUN",
    8: "STOP",
    10: "FAULT_NOW",
    11: "FAULT_OVER",
    12: "ICLWAIT",
    16: "CHARGE_BOOT_CAP",
    17: "OFFSET_CALIB",
    19: "SWITCH_OVER",
    20: "WAIT_STOP_MOTOR",
}


def run(cmd: list[str]) -> str:
    return subprocess.check_output(cmd, text=True)


def tool(name: str, env_var: str, fallback: str | None = None) -> str:
    if env_var in os.environ:
        return os.environ[env_var]
    found = shutil.which(name)
    if found:
        return found
    if fallback:
        return fallback
    raise SystemExit(f"missing required tool: {name}")


def symbol_addresses(elf: str) -> dict[str, int]:
    nm = tool("arm-none-eabi-nm", "ARM_NM")
    out = run(
        [
            nm,
            "-n",
            elf,
        ]
    )
    wanted = {
        "STO_PLL_M1",
        "VirtualSpeedSensorM1",
        "SpeednTorqCtrlM1",
        "Mci",
    }
    addrs: dict[str, int] = {}
    for line in out.splitlines():
        m = re.match(r"^([0-9a-fA-F]+)\s+\w\s+(\w+)$", line.strip())
        if not m:
            continue
        name = m.group(2)
        if name in wanted:
            addrs[name] = int(m.group(1), 16)
    missing = wanted - addrs.keys()
    if missing:
        raise SystemExit(f"missing symbols in elf: {sorted(missing)}")
    return addrs


def probe_read(chip: str, address: int, count: int, speed_khz: int) -> bytes:
    out = run(
        [
            "probe-rs",
            "read",
            "--chip",
            chip,
            "--protocol",
            "swd",
            "--speed",
            str(speed_khz),
            "b8",
            hex(address),
            str(count),
        ]
    )
    vals = [int(tok, 16) for tok in out.strip().split()]
    return bytes(vals)


def u16(buf: bytes, off: int) -> int:
    return struct.unpack_from("<H", buf, off)[0]


def i16(buf: bytes, off: int) -> int:
    return struct.unpack_from("<h", buf, off)[0]


def i32(buf: bytes, off: int) -> int:
    return struct.unpack_from("<i", buf, off)[0]


def rpm_from_speed_unit(speed_unit: int) -> float:
    return speed_unit * 60.0 / 10.0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--elf", required=True)
    ap.add_argument("--chip", default="STM32G431C8")
    ap.add_argument("--duration", type=float, default=6.0)
    ap.add_argument("--period", type=float, default=0.1)
    ap.add_argument("--attach-timeout", type=float, default=3.0)
    ap.add_argument("--speed-khz", type=int, default=100)
    args = ap.parse_args()

    addrs = symbol_addresses(args.elf)
    base = min(addrs.values())
    end = max(addrs.values()) + 64
    span = end - base

    sto_off = addrs["STO_PLL_M1"] - base
    vss_off = addrs["VirtualSpeedSensorM1"] - base
    stc_off = addrs["SpeednTorqCtrlM1"] - base
    mci_off = addrs["Mci"] - base

    start = time.monotonic()
    print(
        "t_s,state,obs_speed_unit,obs_rpm,forced_speed_unit,forced_rpm,ref_speed_unit,ref_rpm,fault_now,fault_past"
    )

    attach_deadline = start + args.attach_timeout

    while True:
        now = time.monotonic()
        if now - start > args.duration:
            break

        try:
            blob = probe_read(args.chip, base, span, args.speed_khz)
        except subprocess.CalledProcessError as exc:
            if now < attach_deadline:
                time.sleep(min(args.period, 0.1))
                continue
            print(f"{now-start:.3f},READ_ERROR,,,,,,,,", file=sys.stderr)
            raise SystemExit(exc.returncode) from exc

        obs_speed = i16(blob, sto_off + 12)
        forced_speed = i16(blob, vss_off + 12)
        ref_speed = i32(blob, stc_off + 8) >> 16
        state = blob[mci_off + 31]
        fault_now = u16(blob, mci_off + 32)
        fault_past = u16(blob, mci_off + 34)

        print(
            f"{now-start:.3f},"
            f"{STATE_NAMES.get(state, str(state))},"
            f"{obs_speed},{rpm_from_speed_unit(obs_speed):.1f},"
            f"{forced_speed},{rpm_from_speed_unit(forced_speed):.1f},"
            f"{ref_speed},{rpm_from_speed_unit(ref_speed):.1f},"
            f"0x{fault_now:04x},0x{fault_past:04x}"
        )

        sleep_time = args.period - (time.monotonic() - now)
        if sleep_time > 0:
            time.sleep(sleep_time)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
