# Feign (DM0004) — Design Specification

A compact USB-C to UART bridge built on the STM32G0B1. Enumerates as a
standard USB CDC-ACM virtual serial port; an optional firmware build
adds software FTDI UART-mode emulation for `pyftdi` compatibility.

---

## 1. Purpose

A USB-C to 3.3 V UART bridge replacing common FT232R / CP2102 / CH340
cables with an open-firmware alternative. A Cortex-M0+ runs the USB
device stack and the protocol translator.

Primary use cases:

1. Plug into a dev board's serial header and open a terminal.
2. Power the attached target from USB VBUS (5 V, current-limited).
3. Toggle modem-control lines (DTR / RTS) for boards that auto-reset.
4. Be reflashed in-system over USB via the STM32 ROM DFU bootloader.

---

## 2. Requirements summary

| ID | Requirement | Priority |
|----|-------------|----------|
| R1 | USB 2.0 FS device, bus-powered, USB-C receptacle | P0 |
| R2 | Crystal-less USB via HSI48 + CRS | P0 |
| R3 | Enumerate as USB CDC-ACM virtual serial port | P0 |
| R4 | Optional firmware: software FTDI UART-mode emulation for `pyftdi` | P1 |
| R5 | UART signals: TX, RX, plus 2× firmware-defined modem lines, all 3.3 V CMOS | P0 |
| R6 | VCC output: 5 V, ~400 mA current-limited (USB 500 mA − ~100 mA board), firmware-enable, fault flag to MCU | P0 |
| R7 | LEDs: power (driven by LDO PG), TX activity, RX activity | P1 |
| R8 | Firmware update via on-board USB DFU (ROM bootloader) + BOOT0/RESET buttons | P0 |
| R9 | Tag-Connect SWD footprint (unpopulated) for factory / recovery | P1 |
| R10 | ESD: IEC 61000-4-2 Level 4 on D+/D-/CC1/CC2 + VBUS TVS | P0 |
| R11 | CC termination via STM32G0B1 UCPD dead-battery Rd (no external 5.1 kΩ) | P0 |
| R12 | Load-switch fault behavior: auto-retry | P0 |
| R13 | Single channel, single UART (no MPSSE, no multi-channel) | P0 |
| R14 | 1×6 SMD right-angle pin header, SparkFun FTDI Basic pinout (legacy compatibility) | P0 |

P0 = must have. P1 = should have.

---

## 3. System architecture

```
 ┌──────────────────────────────────────────────────────────────────┐
 │                                                                  │
 │      USB-C receptacle  (USB 2.0 FS, sink)                        │
 │            │                                                     │
 │            │  VBUS  D+  D-  CC1  CC2  GND                        │
 │            │                                                     │
 │            ▼                                                     │
 │   ┌─────────────────────────┐                                    │
 │   │  UsbCSink16P module     │  Connector + TPD4E05U06 ESD        │
 │   │  (CC Rd DNP, TVS pop'd) │  + VBUS TVS                        │
 │   └─────────────────────────┘                                    │
 │            │                                                     │
 │  VBUS (5V) ┼──────────┬───────────────────────────┐              │
 │            │          │                           │              │
 │            ▼          ▼                           ▼              │
 │     ┌────────────┐  ┌────────────────┐    ┌─────────────────┐    │
 │     │  TPS74x01P │  │   TPS2553DRV   │    │ STM32G0B1KxUxN  │    │
 │     │  ULDO 3V3  │  │  load switch   │    │  UFQFPN-32 'N'  │    │
 │     │  + PG out  │  │  400 mA, ARet  │    │  Cortex-M0+     │    │
 │     └────────────┘  └────────────────┘    │                 │    │
 │            │                │  │          │   USB PA11/PA12 │    │
 │            ▼                │  ▼          │   UART → header │    │
 │       ┌─────────┐           │ /FLT        │   UCPD CC pins  │    │
 │       │ PWR LED │           │             │   PG → input    │    │
 │       └─────────┘           │             │   EN/FLT GPIOs  │    │
 │           (3V3 to MCU,      │             │   BOOT0, NRST   │    │
 │            LEDs, pull-ups)  │             │   SWD pads      │    │
 │                             ▼             │                 │    │
 │                       VCC_OUT 5V          └─────────────────┘    │
 │                             │                  │       │         │
 │                             ▼                  ▼       ▼         │
 │              ┌────────────────────────────────────────────────┐  │
 │              │  1×6 SMD right-angle pin header (Harwin M20)   │  │
 │              │  GND  CTS  VCC  TXD  RXD  DTR                  │  │
 │              └────────────────────────────────────────────────┘  │
 │                                                                  │
 │   Buttons: BOOT0 (PA14), RESET (NRST)                            │
 │   SWD: Tag-Connect TC2030-IDC-NL footprint, pads only            │
 │                                                                  │
 └──────────────────────────────────────────────────────────────────┘
```

### USB enumeration

Default firmware: **CDC-ACM** single virtual serial port.

Optional firmware build: vendor-class interface emulating the FTDI
UART-mode protocol. Mode selection is a firmware concern; no board
impact.

---

## 4. Power

### Input

| Parameter | Value |
|-----------|-------|
| Source | USB VBUS via USB-C receptacle (bus-powered) |
| Voltage | 4.75 V – 5.25 V (USB 2.0 tolerance) |
| Max draw | 500 mA total (USB default device budget) |

### Rails

| Rail | Voltage | Source | Budget |
|------|---------|--------|--------|
| VBUS | 5 V (USB) | USB-C, ESD-clamped, VBUS-TVS protected | 500 mA total |
| 3V3 | 3.3 V ±2% | TPS74x01P ULDO from VBUS | ~30 mA peak |
| VCC_OUT | 5 V | TPS2553DRV load switch from VBUS | ≤ 400 mA, current-limited |

### 3 V3 budget

| Consumer | Current |
|---|---|
| STM32G0B1 @ 64 MHz + USB peripheral | ~20 mA peak |
| 3× LEDs @ 2 mA each | 6 mA |
| Pull-ups, LDO/switch IQ | < 1 mA |
| **Total** | **~25–30 mA peak** |

LDO (500 mA capable) has > 15× margin. Dropout 5 V → 3.3 V at 30 mA
dissipates ~50 mW, trivial in DRV (SON-6).

### VBUS sensing

Not implemented. Bus-powered device — USB attach is equivalent to
power-on, so AN4879's VBUS divider is unnecessary.

### Target power control (VCC_OUT)

VCC_OUT is gated by a current-limited load switch driven by an MCU GPIO,
so firmware can power-cycle the target over USB and a target short does
not brown out the host.

| Parameter | Value |
|---|---|
| Switch | TPS2553DRV (auto-retry variant) |
| Input | VBUS (5 V) |
| Output | VCC pin on target header |
| Current limit | ~400 mA (set by ILIM resistor; leaves ~100 mA for the board itself within the 500 mA USB budget) |
| Enable | MCU GPIO (polarity per chosen part) |
| Fault | Open-drain `/FLT` to MCU GPIO with 10 kΩ pull-up to 3V3 |
| Soft-start | Built into TPS2553 |
| Reverse-current block | Yes (TPS2553 feature) |

---

## 5. Interfaces

### USB-C (host side)

Implemented via `github.com/diodeinc/registry/modules/UsbCSink16P@0.1.4`
(GCT USB4105-GF-A 16-pin USB 2.0-only USB-C, TPD4E05U06 ESD on D+/D-/CC1/CC2,
VBUS TVS).

Module config:

| Config | Value | Rationale |
|---|---|---|
| `cc_resistors` | `False` | STM32G0B1 'N' UCPD dead-battery Rd handles termination from POR. External 5.1 kΩ pulldowns redundant. Footprints DNP for debug/rework. |
| `tvs` | `True` | Keep VBUS TVS for defective-source protection. |

D+/D- routed as 90 Ω differential to the STM32 (PA11/PA12).

### Target header (target side)

**1×6 male SMD right-angle pin header**, pins exiting the short edge
opposite the USB-C receptacle. Part: **Harwin M20-8890645** (registry,
in-house). Pinout matches the SparkFun FTDI Basic / Adafruit FTDI
Friend convention for drop-in compatibility with existing dev-board
headers:

| Pin | Silk | Default function | Direction (default) |
|----:|------|------------------|---------------------|
| 1 | GND | Ground | — |
| 2 | CTS | Firmware-defined GPIO; default = CTS | Input |
| 3 | VCC | 5 V, current-limited, firmware-enabled | Output |
| 4 | TXD | UART TX from board → target RX | Output |
| 5 | RXD | UART RX to board ← target TX | Input |
| 6 | DTR | Firmware-defined GPIO; default = DTR (Arduino auto-reset) | Output |

Notes:

- Pins 2 and 6 are plain 3.3 V GPIOs. Silkscreen labels reflect the
  default firmware role; firmware can reassign to RTS / DSR / DCD / RI /
  plain GPIO at runtime based on CDC modem-line state or vendor
  requests.
- All inputs (RXD, CTS) are routed to STM32 **FT_c** pins (PD2, PD0).
  These tolerate 5 V from an externally-powered target while Feign is
  unpowered. Plain FT pins are only V_DD + 4 V max, insufficient when
  V_DD = 0.
- Optional ≈ 100 Ω series resistors on TXD / DTR for short-circuit
  protection — EE judgment.

### Debug

- **Tag-Connect TC2030-IDC-NL** SWD footprint (SWDIO/SWCLK/NRST/3V3/GND).
  No connector populated. Pads only.
- **RESET button** (Omron B3U-1000P): tactile SMT, NRST → GND. 100 nF
  filter cap from NRST to GND per AN4879.
- **BOOT0 button** (Omron B3U-1000P): tactile SMT, PA14/BOOT0 → 3V3 with
  10 kΩ pull-down keeping the pin defined when the button is released.
  Hold BOOT0 + tap RESET → enter STM32 ROM USB DFU bootloader.

### User I/O

| LED | Color (suggested) | Driver | Behavior |
|---|---|---|---|
| Power | Green | TPS74x01P PG output (push-pull) directly | Lights only when 3V3 LDO is in regulation |
| TX | Amber | MCU GPIO | Pulsed by firmware on host → target traffic |
| RX | Blue | MCU GPIO | Pulsed by firmware on target → host traffic |

All via stdlib `Led` generic at ~2 mA each.

---

## 6. Key components

| Function | Part | Package | Source |
|----------|------|---------|--------|
| MCU | STM32G0B1 'N' SKU (alt-pinout, UCPD on 32-pin) | UFQFPN-32 | `components/STMicroelectronics/STM32G0B1KxUxN@0.1.0` |
| USB-C front-end | UsbCSink16P module (connector + ESD + VBUS TVS) | Module | `modules/UsbCSink16P@0.1.4` |
| 3V3 LDO | TPS74x01P ULDO, adjustable, PG output | DRV (SON-6) | `components/Texas_Instruments/TPS74x01P@0.1.1` |
| VCC load switch | TPS2553DRV, auto-retry, adjustable ILIM | SOT-23-6 | `components/Texas_Instruments/TPS2553DRV@0.1.0` |
| Pin header | Harwin M20-8890645 | SMD right-angle | `components/Harwin/M20-8890645` (in-house) |
| Tactile buttons | Omron B3U-1000P, 1.5 N | SMT | `components/B3U-1000P@0.2.1` |
| LEDs | stdlib `Led` generic | 0402 | Generic |
| SWD footprint | Tag-Connect TC2030-IDC-NL | THT pads, no part | Footprint only |

### Support passives (commodity, must be on the board)

- **MCU decoupling**: 100 nF 0402 on each VDD pin (VDD ×2, VDDA, VDDIO2/VDDUSB) + 1× ≥ 1 µF bulk near MCU. Per AN4879.
- **NRST filter**: 100 nF 0402, NRST → GND. AN4879 glitch immunity.
- **BOOT0 pull-down**: 10 kΩ, PA14/BOOT0 → GND.
- **VBAT tie**: short to VDD; RTC backup unused.
- **VBUS bulk**: 4.7–10 µF ceramic near USB-C receptacle (USB 2.0 spec, not provided by `UsbCSink16P`).
- **Load switch support**: 1× ILIM resistor (≈ 64.9 kΩ → 400 mA), 1× 100 kΩ /FLT pull-up to 3V3, 1× 100 kΩ EN pull-down to GND (deterministic-off during MCU reset), 1× ≈ 22 µF ceramic on VCC_OUT.
- **LDO feedback divider**: 2× resistors for 3.3 V (handled by `TPS745x` reference module config).
- **CC Rd pulldowns**: 2× 5.1 kΩ inside the `UsbCSink16P` footprint, DNP — UCPD dead-battery Rd replaces them.

---

## 7. Mechanical & environmental

- Form factor: USB dongle, target ~ **18 mm × 40 mm**. USB-C at one short edge, 1×6 header at the opposite short edge. Final dimensions at EE discretion.
- Stack-up: 1.6 mm FR4, 2 layers, 1 oz copper.
- Operating temperature: 0 °C – 70 °C (commercial use).
- Mounting: none (free-floating dongle).
- Environmental: indoor dry use, no conformal coating.

---

## 8. Manufacturing & assembly

- Layer count: **2**.
- Min trace/space: 6/6 mil. Min via: 0.25 mm finished.
- Min package: 0402 passives, SOT-23 actives, UFQFPN-32 MCU.
- Assembly: in-house, single-pass SMT reflow. No through-hole / hand-solder steps.
- Prototype quantity: ~5–10.
- US assembly: no. ITAR: no.

---

## 9. Regulatory & compliance

- Not a sold product; FCC / CE / UL not targeted.
- USB-IF certification: not pursued. Device will not claim USB-IF logo compliance.
- FTDI VID (0x0403) **not used**. Default firmware uses a development VID:PID (e.g. pid.codes). Users who need stock pyftdi bindings can either pass `--vid/--pid` to pyftdi or rebuild firmware locally.

---

## 10. Open items (non-blocking)

- TPS2553DRV ILIM resistor exact value (≈ 64.9 kΩ for 400 mA per datasheet R_ILIM = 25.95 kV / I_OS).
- USART peripheral / pin assignment on the G0B1 — multiple valid options. EE picks during schematic capture, keeping USB on PA11/PA12 and routing length minimal.
- Whether to populate ~100 Ω series resistors on TXD / DTR for short-circuit protection.
- VID:PID assignment for production firmware.

---

## Design notes

1. **Modem lines are firmware-defined.** Pin 2 (CTS silk) and pin 6 (DTR silk) go to plain 3.3 V GPIOs (PD0 and PD1); firmware decides direction and meaning. The default assignment matches the SparkFun FTDI Basic pinout for cable-level drop-in compatibility (and Arduino auto-reset on DTR). Hardware UART CTS/RTS is not used on this board — USART5 is plain TX/RX only.

2. **VID:PID policy.** Released firmware will not ship with the FTDI VID (0x0403). This is a deliberate trademark / driver-blocklist guardrail. Users who need stock pyftdi bindings can either pass `--vid/--pid` or rebuild firmware locally.

3. **Crystal-less USB.** STM32G0B1 integrates HSI48 + CRS. CRS trims HSI48 against USB SOF packets to well under USB 2.0 FS's ±500 ppm requirement and far exceeds UART accuracy needs. No HSE crystal on the BOM.

4. **D+ pull-up.** STM32G0B1 integrates the 1.5 kΩ D+ pull-up on-die (AN4879 §3.1.1). No external pull-up needed.

5. **VBUS sensing skipped.** Bus-powered device. Per AN4879, the VBUS-sensing divider is not required in this configuration.

6. **ESD / VBUS protection.** Fully handled by `UsbCSink16P`: TPD4E05U06 4-channel ESD on D+/D-/CC1/CC2 (IEC 61000-4-2 Level 4: ±8 kV contact / ±15 kV air) plus a VBUS TVS with 5–6 V standoff. This exceeds the G0B1's intrinsic IEC 61000-4-2 Level 2B (≈ ±4 kV) functional EMS rating and brings the port to USB-IF expectations.

7. **CC termination via UCPD dead-battery Rd.** STM32G0B1 'N' variant has UCPD with dead-battery Rd active from POR — host sees a sink before firmware boots. External 5.1 kΩ Rd resistors in the `UsbCSink16P` module are DNP'd (`cc_resistors=False`). Firmware must not disable dead-battery Rd at runtime unless explicitly configuring UCPD as a sink.

8. **Logic-level vs. signal-level VCC.** VCC pin on the target header is 5 V (VBUS-derived, fused). All UART signals (TX/RX/CTS/DTR) are 3.3 V CMOS. Targets must accept 3.3 V high logic — same as modern serial-cable conventions.

9. **5 V tolerance on inputs (FT_c).** RX and CTS are assigned to FT_c pins (PD2 and PD0) so they tolerate up to 5.5 V regardless of V_DD. Plain FT pins are only V_DD + 4 V max — insufficient when V_DD = 0 and target is externally driving 5 V. On the 32-pin 'N' package, only PA8, PB15, PD0, and PD2 are FT_c-class; PA8/PB15 are taken by UCPD CC1/CC2, leaving PD0/PD2 for the user-facing inputs. This forces the UART onto USART5 (PD2 RX, PD3 TX) instead of USART1.

13. **UCPD dead-battery wiring.** Per AN5225 §11.3.1, the dead-battery Rd on PA8/PB15 only exposes itself when the matching DBCC pin is shorted to it externally: PA9 → PA8 (CC1) and PA10 → PB15 (CC2). The schematic implements these shorts. PA9/PA10 are therefore reserved and cannot be used for any other signal. The 5.1 kΩ external CC pull-downs in `UsbCSink16P` are DNP'd (`cc_resistors=False`).

10. **Power LED truthfulness.** Power LED is driven by the TPS74x01P push-pull PG output rather than a 3V3 pull-up, so it lights only when the LDO is in regulation. Saves an MCU GPIO and gives a real "rail healthy" indication.

11. **Firmware update path.** Primary: USB DFU via STM32 system memory bootloader. Hold BOOT0 + tap RESET → host sees an STM32 DFU device → `dfu-util` / `STM32CubeProgrammer` flashes firmware. Tag-Connect SWD is the factory programming and recovery path; no SWD connector populated on shipped boards.

14. **UCPD2 dead-battery firmware mitigation.** PD0/PD1/PD2/PD3 are also UCPD2's CC1/DBCC1/CC2/DBCC2 pins. At reset, UCPD2 hardware can enable a ~5.1 kΩ internal pull-down on PD0 (CTS) or PD2 (RX) when the matching DBCC pin (PD1 / PD3) sees a high level from a powered target. Firmware must release UCPD2 dead-battery early in startup by setting `SYSCFG->CFGR1 |= SYSCFG_CFGR1_UCPD2_STROBE`. Do **not** set `UCPD1_STROBE` until UCPD1 has been brought up properly — that disables the dead-battery Rd we rely on for USB-C presence detection.

12. **GPIO budget.** Used pins: USB (2) + UART TX/RX (2) + 2 modem GPIOs + 3 LEDs + 2 buttons + load-switch EN + load-switch /FLT + LDO PG (sense-only) + UCPD CC1/CC2 + SWD (2) = 18 pins. Comfortable in the UFQFPN-32 'N' package.
