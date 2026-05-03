# TPS709-Q1 — TI 30 V, 150 mA low-Iq automotive LDO (WSON-6 / DRV)

Selector package for Texas Instruments' TPS709-Q1 AEC-Q100 linear
regulator family. All nine orderables share pinout, package, datasheet,
and performance — they differ only in the factory-trim output voltage.
Select by `output_voltage`; the matching silicon is picked automatically.

## Parts

| MPN | Vout | Max Dropout (150 mA) |
|-----|------|----------------------|
| TPS70912QDRVRQ1 | 1.2 V | — |
| TPS70915QDRVRQ1 | 1.5 V | — |
| TPS70918QDRVRQ1 | 1.8 V | — |
| TPS70925QDRVRQ1 | 2.5 V | — |
| TPS70927QDRVRQ1 | 2.7 V | — |
| TPS70928QDRVRQ1 | 2.8 V | — |
| TPS70930QDRVRQ1 | 3.0 V | 1.54 V |
| TPS70933QDRVRQ1 | 3.3 V | 1.54 V |
| TPS70950QDRVRQ1 | 5.0 V | 1.20 V |

All 9 share: 2.7–30 V Vin, 150 mA Iout, ~1 µA Iq, 80 dB PSRR at 10 Hz,
overcurrent / overtemperature / reverse-current / UVLO protection,
AEC-Q100 qualification.

## Selection

One config knob:

- `output_voltage` (Voltage) — factory-trim output. Must exactly match one
  of `"1.2V"`, `"1.5V"`, `"1.8V"`, `"2.5V"`, `"2.7V"`, `"2.8V"`, `"3V"`,
  `"3.3V"`, `"5V"`. Default: `"3.3V"`.

## Decoupling

Instantiated with the IC:

- `C_IN` — 4.7 µF / 50 V X7R / 0805. Sized for full 30 V Vin including
  automotive load-dump transients. Not required for stability per
  datasheet §7.1.1, but strongly recommended for ripple rejection.
- `C_OUT` — 4.7 µF / 16 V X7R / 0603. Meets the datasheet §7.1.1
  stability floor (≥ 2 µF effective for Vout < 1.5 V, ≥ 1.5 µF for
  Vout ≥ 1.5 V) across all 9 trims after tolerance + DC-bias derating.

## EN

Exposed as an optional `io(Net, ...)` with a default `NotConnected()` —
the datasheet explicitly allows EN to float (internal pull-up enables the
device). If driving externally, keep V_EN ≤ 6.5 V — do not tie EN to VIN
if VIN can exceed 6.5 V.

## Example

```zen
TPS709_Q1 = Module("@github/diodeinc/registry/components/Texas_Instruments/TPS709-Q1/TPS709-Q1.zen")

# Default 3.3 V (TPS70933QDRVRQ1)
TPS709_Q1(name="U_3V3", VIN=VBAT, VOUT=V3V3, GND=GND)

# Explicit 5 V rail, EN driven by an MCU GPIO (must be ≤ 6.5 V)
TPS709_Q1(
    name="U_5V",
    output_voltage="5V",
    VIN=VBAT,
    VOUT=V5,
    GND=GND,
    EN=SYS_EN,
)
```

## Package

**DRV / WSON-6** — 2 × 2 × 0.8 mm, 6-pin leadless DFN with exposed thermal
pad. Footprint is KiCad's stock `WSON-6-1EP_2x2mm_P0.65mm_EP1x1.6mm`
(IPC-7351 nominal density); electrically equivalent to TI's recommended
DRV land pattern. STEP 3D model is the KiCad-bundled file, embedded in
the `.kicad_mod` for self-contained use.
