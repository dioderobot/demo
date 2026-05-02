# /// script
# requires-python = ">=3.10"
# dependencies = ["marimo", "matplotlib", "numpy", "scienceplots"]
# ///
"""TPS259482 OVLO/UVLO Protection — marimo simulation analysis.

Self-contained: runs `pcb sim --netlist` + ngspice in a tempdir, asserts
behavior across the four-phase VIN sweep, and plots inline.

    uvx marimo edit --sandbox sim_analyze.py   # interactive
    uvx marimo run  --sandbox sim_analyze.py   # read-only app
    uv  run                  sim_analyze.py   # headless / CI
"""

import marimo

__generated_with = "0.23.4"
app = marimo.App(width="medium")


@app.cell
def _():
    import marimo as mo

    return (mo,)


@app.cell
def _(mo):
    mo.md("""
    # TPS259482 OVLO/UVLO Protection — Simulation Analysis
    """)
    return


@app.cell
def _():
    import matplotlib.pyplot as plt
    import numpy as np
    import scienceplots  # noqa: F401  (registers matplotlib styles)

    plt.style.use(["science", "no-latex", "grid"])
    return np, plt


@app.cell
def _(np):
    """Run pcb sim → ngspice in a tempdir; load wrdata CSV into memory."""
    import subprocess
    import tempfile
    from pathlib import Path

    TB = Path(__file__).resolve().parent / "test_startup.zen"

    netlist = subprocess.run(
        ["pcb", "sim", "--netlist", str(TB)],
        capture_output=True, text=True, check=True,
    ).stdout

    with tempfile.TemporaryDirectory() as _td:
        tmp = Path(_td)
        (tmp / "output").mkdir()  # netlist writes to `output/startup.csv`
        (tmp / "tb.cir").write_text(netlist)
        subprocess.run(["ngspice", "-b", "tb.cir"], cwd=tmp, check=True, capture_output=True)
        csv = tmp / "output" / "startup.csv"
        with csv.open() as f:
            header = f.readline().split()
        data = np.loadtxt(csv, skiprows=1)

    t = data[:, 0]
    cols = {name: data[:, i] for i, name in enumerate(header)}
    print(f"{len(t)} samples · {t[-1] * 1e3:.0f} ms · signals {header[1:]}")
    return cols, t


@app.cell
def _():
    # (t0, t1, label, vin_expected, vout_should_be_off, splygd_should_be_high)
    PHASES = [
        (3e-3,  5e-3,  "normal-1", 12.0, False, True),
        (7e-3,  10e-3, "OV fault", 24.0, True,  False),
        (12e-3, 15e-3, "normal-2", 12.0, False, True),
        (17e-3, 20e-3, "UV fault",  4.0, True,  False),
    ]
    return (PHASES,)


@app.cell
def _(PHASES, cols, t):
    """Assert each phase. Function-scoped so loop vars don't leak to cell top-level."""

    def _check(t0, t1, vin_exp, vout_off, splygd_hi):
        sl = (t >= t0) & (t <= t1)
        v_in, v_out, v_sg = (cols[k][sl].mean() for k in ("v(VIN)", "v(VOUT)", "v(SPLYGD)"))
        problems = []
        if abs(v_in - vin_exp) > 0.1:
            problems.append(f"VIN drift ({v_in:.2f}V)")
        if vout_off and v_out > 1.0:
            problems.append(f"VOUT not off ({v_out:.2f}V)")
        if not vout_off and abs(v_out - v_in) > 0.5:
            problems.append(f"VOUT≠VIN ({v_out:.2f}V)")
        if splygd_hi and v_sg < 3.0:
            problems.append(f"SPLYGD low ({v_sg:.2f}V)")
        if not splygd_hi and v_sg > 0.5:
            problems.append(f"SPLYGD high ({v_sg:.2f}V)")
        return v_in, v_out, v_sg, problems

    def _run():
        fails = 0
        print(f"{'phase':<10}  {'VIN':>7}  {'VOUT':>7}  {'SPLYGD':>7}  result")
        for t0, t1, label, vin_exp, vout_off, splygd_hi in PHASES:
            v_in, v_out, v_sg, problems = _check(t0, t1, vin_exp, vout_off, splygd_hi)
            fails += bool(problems)
            status = "PASS" if not problems else "FAIL — " + ", ".join(problems)
            print(f"{label:<10}  {v_in:6.2f}V  {v_out:6.2f}V  {v_sg:6.2f}V  {status}")
        assert fails == 0, f"{fails} phase(s) failed"
        print("All phases pass ✓")

    _run()
    return


@app.cell
def _(PHASES, cols, mo, plt, t):
    """Two-row figure with phase shading; rendered via mo.mpl.interactive for pan/zoom."""

    def _render():
        fig, (a1, a2) = plt.subplots(2, 1, figsize=(9, 5.5), sharex=True)
        tm = t * 1e3

        a1.plot(tm, cols["v(VIN)"],  color="C3", lw=1.4, label="VIN")
        a1.plot(tm, cols["v(VOUT)"], color="C0", lw=1.4, label="VOUT")
        a1.axhline(22, ls="--", color="0.4", lw=0.7, label="OVLO 22 V")
        a1.axhline(6,  ls=":",  color="0.4", lw=0.7, label="UVLO 6 V")
        a1.set(ylabel="Power rails (V)", title="TPS259482 OVLO/UVLO response")
        a1.legend(loc="upper right", ncol=2)

        a2.plot(tm, cols["v(EN)"],          color="C1", lw=1.2, label="EN/UVLO tap")
        a2.plot(tm, cols["v(EFUSE.OVLO)"],  color="C2", lw=1.2, label="OVLO tap")
        a2.plot(tm, cols["v(SPLYGD)"],      color="C4", lw=1.2, label="SPLYGD")
        a2.axhline(1.2, ls="--", color="0.4", lw=0.7, label="1.2 V threshold")
        a2.set(ylabel="Logic / divider (V)", xlabel="Time (ms)")
        a2.legend(loc="upper right", ncol=2)

        for t0, t1, _, _, vout_off, _ in PHASES:
            shade = "#fde7e7" if vout_off else "#e7f5ea"
            for ax in (a1, a2):
                ax.axvspan(t0 * 1e3, t1 * 1e3, color=shade, alpha=0.7, zorder=-1)

        fig.tight_layout()
        return fig

    mo.mpl.interactive(_render())
    return


if __name__ == "__main__":
    app.run()
