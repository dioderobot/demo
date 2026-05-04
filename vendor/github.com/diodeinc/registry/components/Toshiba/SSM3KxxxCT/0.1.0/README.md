# SSM3KxxxCT — Toshiba N-Ch MOSFETs in CST3 (SOT-883)

Selector package for Toshiba's small-signal and load-switch N-channel MOSFET
family in the CST3 (SOT-883, SC-101) 1.0 × 0.6 mm package. One `.zen`, one
`.kicad_sym`, one shared footprint.

## Parts

| MPN | Vds | Id | Rds(on) | Vgs(th) | Note |
|-----|-----|-----|---------|---------|------|
| SSM3K16CT  | 20 V | 100 mA | 3.0 Ω @ 10 mA, 4 V    | 1.1 V | π-MOSIV small-signal |
| SSM3K35CT  | 20 V | 180 mA | 3.0 Ω @ 50 mA, 4 V    | 1.0 V | older small-signal |
| SSM3K37CT  | 20 V | 200 mA | 2.2 Ω @ 100 mA, 4.5 V | 1.0 V | U-MOSIII |
| SSM3K56CT  | 20 V | 800 mA | 235 mΩ @ 800 mA, 4.5 V | 1.0 V | U-MOSVII-H load switch |
| SSM3K56ACT | 20 V | 1.4 A  | 235 mΩ @ 800 mA, 4.5 V | 1.0 V | higher-Id bin of K56 |
| SSM3K15ACT | 30 V | 100 mA | 3.6 Ω @ 10 mA, 4 V    | 1.5 V | needs ≥2.5 V gate drive |

All variants share pinout (1 = Gate, 2 = Source, 3 = Drain) and footprint
(CST3 / SOT-883). MPNs match Digi-Key's comma-suffixed orderable form
(e.g. `SSM3K35CT,L3F`).

## Selection

Two config knobs:

- `vds` (`"20V"` | `"30V"`) — drain-to-source voltage class.
- `drain_current` (Current) — minimum continuous Id required.

The smallest-sufficient current tier at the requested `vds` becomes the
primary part; larger tiers are emitted as BOM alternates.

## Pinout

| Pin | Function |
|-----|----------|
| 1 | Gate |
| 2 | Source |
| 3 | Drain |

## Examples

```zen
SSM3KxxxCT = Module("@github/diodeinc/registry/components/Toshiba/SSM3KxxxCT/SSM3KxxxCT.zen")

# Default: smallest 20 V part that handles ≥100 mA → SSM3K16CT
SSM3KxxxCT(name="Q1", G=GATE, S=GND, D=LOAD)

# 20 V, 500 mA load switch — picks SSM3K56CT, alt = SSM3K56ACT
SSM3KxxxCT(name="Q_SW", vds="20V", drain_current="500mA",
           G=EN, S=GND, D=LOAD)

# 30 V small-signal — picks SSM3K15ACT
SSM3KxxxCT(name="Q2", vds="30V", G=GATE, S=GND, D=VBAT)
```

## Package

**CST3 / SOT-883 / SC-101** — 1.0 × 0.6 × 0.38 mm, 3-pin leadless DFN.
JEDEC/JEITA standard; second-sourced by NXP, Nexperia, Diodes, etc. Not
footprint-compatible with Toshiba's shrunk `CST3C` (0.8 × 0.6 mm) — a
different package lives in its own registry entry.
