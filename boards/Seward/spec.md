# Seward — Design Specification

A small, reliable CMSIS-DAP debug probe based on the STM32G0B1. Exposes
a 10-pin ARM Cortex-Debug target header with bidirectional SWD and an
integrated UART bridge. Target-side I/O rails auto-range from 1.1 V to
5.0 V so one probe handles 1.8 V / 3.3 V / 5 V targets.

---

## 1. Purpose

A minimal, open-firmware USB-to-SWD debug probe. Drop-in replacement for
a CMSIS-DAP v2 probe for bringing up and flashing Cortex-M targets.

Primary use cases:

1. Host-side CMSIS-DAP v2 (WinUSB bulk) over USB-C for `pyOCD` /
   `OpenOCD` / `probe-rs` / `Keil` / IAR.
2. USB-CDC virtual UART for target serial console, independent from SWD.
3. Power-off target while probe remains plugged into USB (target
   powered externally) without back-feed from probe into target.
4. Probe itself is reprogrammable over USB DFU (STM32 ROM bootloader)
   via a BOOT0 + RESET button combination; a Tag-Connect footprint is
   available for factory/recovery SWD.

Named for Dr. John Seward — the asylum director who kept Dracula's
victims under methodical, if ineffective, observation.

---

## 2. Requirements summary

| ID | Requirement | Priority |
|----|-------------|----------|
| R1 | USB 2.0 FS device, bus-powered, USB-C receptacle | P0 |
| R2 | Crystal-less USB via HSI48 + CRS | P0 |
| R3 | Enumerate as CMSIS-DAP v2 (WinUSB) + CDC-ACM composite | P0 |
| R4 | Target-side I/O rails auto-range over 1.1–5.0 V (VTref-driven, nRESET clamp) | P0 |
| R5 | Bidirectional SWDIO via directional level shifter, firmware-controlled direction | P0 |
| R6 | Target UART bridge, same voltage range as SWD | P0 |
| R7 | Target nRESET: open-drain drive, 5 V tolerant when probe is unpowered | P0 |
| R8 | No back-feed from target into probe when USB is disconnected | P0 |
| R9 | 10-pin 0.05" ARM Cortex-Debug target header, keyed/shrouded | P0 |
| R10 | Four status LEDs (activity / heartbeat); none on power rails | P0 |
| R11 | Probe self-flashable via USB DFU (BOOT0 + RESET buttons) | P0 |
| R12 | Tag-Connect SWD footprint (unpopulated) for factory / recovery | P1 |
| R13 | ESD: IEC 61000-4-2 Level 4 on D+/D-/CC1/CC2 + VBUS TVS | P0 |
| R14 | CC termination via STM32G0B1 UCPD dead-battery Rd (no external 5.1 kΩ) | P0 |

P0 = must have, P1 = should have.

---

## 3. System architecture

```
 ┌─────────────────────────────────────────────────────────────────────────┐
 │  USB-C receptacle (GCT USB4105-GF-A, USB 2.0, 16 pin)                   │
 │         │                                                               │
 │         │ VBUS  D+/D-  CC1 CC2  GND                                     │
 │         ▼                                                               │
 │  TPD4E05U06 (4-ch TVS)  ── D+/D-/CC1/CC2                                │
 │         │                                                               │
 │  VBUS ──┼──────────────┐                                                │
 │         │              ▼                                                │
 │         │        ┌──────────────┐                                       │
 │         │        │  TPS74x01P   │  3V3                                  │
 │         │        │  ULDO 500 mA │                                       │
 │         │        └──────┬───────┘                                       │
 │         │               │                                               │
 │         │         ┌─────▼──────────────────┐                            │
 │         │         │ STM32G0B1KxUxN         │                            │
 │         │         │ UFQFPN-32 (N-pinout)   │                            │
 │         │         │ USB + UCPD dead-batt   │                            │
 │         │         │ CMSIS-DAP v2 firmware  │                            │
 │         │         │                        │                            │
 │         │         │ SWD_A   SWCLK_A        │                            │
 │         │         │ SWD_DIR                │                            │
 │         │         │ UART_TX_A UART_RX_A    │                            │
 │         │         │ nRST (FT_c, 5V tol.)   │                            │
 │         │         │ 4× LED GPIOs           │                            │
 │         │         │ BOOT0, NRST buttons    │                            │
 │         │         └─┬──────────────────────┘                            │
 │         │           │ (3V3 A-side)                                      │
 │         │           ▼                                                   │
 │         │   ┌──────────────────────────────────────┐                    │
 │         │   │ 4× SN74LXC1T45QDRYRQ1 (USON)        │                    │
 │         │   │  • SWDIO  (DIR = SWD_DIR, firmware)  │                    │
 │         │   │  • SWCLK  (DIR = VCCA, A→B)          │                    │
 │         │   │  • UART TX (DIR = VCCA, A→B)         │                    │
 │         │   │  • UART RX (DIR = GND, B→A)          │                    │
 │         │   │  VCCB = VTref (auto 1.1–5.0 V)       │                    │
 │         │   └──────────────┬───────────────────────┘                    │
 │         │                  │ (B-side, target domain)                    │
 │         │                  │    ┌──── nRESET (direct, MCU open-drain,   │
 │         │                  │    │                    FT_c 5V tol.)      │
 │         ▼                  ▼    ▼                                       │
 │  ┌───────────────────────────────────────────────────────┐              │
 │  │  Samtec FTSH-105-01-L-DV-K-A-P-TR  (10-pin, keyed)    │              │
 │  │   1:VTref  2:SWDIO  3:GND  4:SWCLK  5:GND             │              │
 │  │   6:UART_RX  7:NC(key)  8:UART_TX  9:GNDDetect 10:nRST│              │
 │  └───────────────────────────────────────────────────────┘              │
 │                                                                         │
 │  Buttons: BOOT0 (PA14/BOOT0 → 3V3)  RESET (NRST → GND)                  │
 │  Tag-Connect TC2030-IDC-NL: SWD of the G0B1 itself (unpopulated)        │
 │                                                                         │
 │  LEDs (all GPIO-driven, 3 colors):                                      │
 │    STATUS (green, heartbeat)                                            │
 │    DAP    (amber, SWD activity)                                         │
 │    UART_TX (blue, host→target byte)                                     │
 │    UART_RX (blue, target→host byte)                                     │
 └─────────────────────────────────────────────────────────────────────────┘
```

### USB enumeration

Composite USB device, single configuration:

| Interface | Class | Endpoints | Purpose |
|---|---|---|---|
| 0 | Vendor (WinUSB, MS OS 2.0 descriptors) | `EP1 IN bulk`, `EP1 OUT bulk`, optional `EP2 IN bulk` (SWO) | CMSIS-DAP v2 |
| 1 | CDC-ACM control | `EP3 IN interrupt` | CDC notifications |
| 2 | CDC-ACM data | `EP4 IN bulk`, `EP4 OUT bulk` | Target UART |

- VID:PID: `0x1209 / 0x0001` (pid.codes community default) during
  bringup; production PID assignment tracked in §11.
- MS OS 2.0 descriptors so Windows auto-loads WinUSB on interface 0.
  No INF, no zadig. Compatible with `pyOCD` / `probe-rs` / `OpenOCD`
  CMSIS-DAP v2 driver out of the box.
- iSerialNumber derived from G0B1 96-bit UID → 16-char ASCII so
  `pyOCD list` / `probe-rs list` distinguish multiple probes.
- The SWO streaming endpoint (EP2 IN bulk) is reserved in the
  descriptor but Cortex-M0+ has no SWO output, so it's a no-op in
  this firmware build.
- Firmware concern; no board impact beyond USB D+ pull-up (on-die,
  see Design Note 10).

---

## 4. Power

### Input

| Parameter | Value |
|-----------|-------|
| Source | USB VBUS via USB-C receptacle (bus-powered) |
| Voltage | 4.75 V – 5.25 V |
| Max draw | ≤ 100 mA (probe only; no target power from probe) |

### Rails

| Rail | Voltage | Source | Budget |
|------|---------|--------|--------|
| VBUS | 5 V (USB) | USB-C + VBUS TVS + TPD4E05U06 on D+/D-/CC1/CC2 | n/a |
| 3V3 | 3.3 V ±2% | TPS74x01P ULDO from VBUS | ~30 mA peak |
| VTref | 1.1–5.0 V | **From target** (header pin 1) | ≤ 100 µA target-sourced |

### 3 V3 budget

| Consumer | Current |
|---|---|
| STM32G0B1 @ 64 MHz + USB | ~20 mA peak |
| 4× LXC1T45 A-side quiescent | ~40 µA |
| 4 LEDs @ 2 mA each (typically only one on) | ≤ 8 mA |
| Pull-ups, LDO IQ | < 1 mA |
| **Total** | **~25–30 mA peak** |

LDO has >15× margin. Dropout 5 V → 3.3 V at 30 mA dissipates ~50 mW.

### VTref handling

VTref (target VCC sense, pin 1 of the 10-pin header) is wired directly
to the VCCB rail of each LXC1T45, with a 100 nF 0402 decap per shifter.
No buffer, no ADC, no regulator. Total target-sourced current is ~40 µA
quiescent plus transient A→B driver current during activity.

The probe does not measure VTref. Firmware assumes the user configured
the level shifter for the connected target by virtue of plugging in.

### VBUS sensing

Not implemented — bus-powered device.

---

## 5. Interfaces

### USB-C (host)

Direct instantiation of:

- `connectors/UsbC16P/UsbC16P.zen` — GCT USB4105-GF-A 16-pin USB 2.0
  receptacle.
- `components/TPD4E05U06QDQARQ1/TPD4E05U06QDQARQ1.zen` — 4-channel TVS.

Wiring (per explicit user requirement):

| TPD4E05U06 pin | Net |
|----------------|-----|
| D2M | USB_C.D.P |
| D2P | USB_C.D.N |
| D1P | USB_C.CC2 |
| D1M | USB_C.CC1 |

VBUS bulk: 4.7–10 µF ceramic near the receptacle (USB 2.0 spec; not
provided by the raw `UsbC16P` connector module — it's part of the host
design, unlike `UsbCSink16P`).

VBUS TVS: 1 × SMAJ5.0A (or equivalent 5 V standoff TVS) across VBUS/GND
near the receptacle.

**CC resistors**: not placed. STM32G0B1 'N' variant UCPD dead-battery Rd
provides the Rd termination from POR. Per AN5225 §11.3.1, DBCC pins are
shorted to matching CC pins externally:
- PA9 (UCPD1_DBCC1) ↔ PA8 (UCPD1_CC1) ↔ USB_C.CC1
- PA10 (UCPD1_DBCC2) ↔ PB15 (UCPD1_CC2) ↔ USB_C.CC2

Firmware must not disable UCPD1 dead-battery until it's configured as a
sink (see Design Note 5).

D+/D- routed 90 Ω differential to PA11/PA12.

### Target header (10-pin Cortex-Debug, modified for UART)

**Samtec FTSH-105-01-L-DV-K-A-P-TR** — 2×5, 0.05" pitch, keyed/shrouded,
SMT, reuses the DM0002 package. Pinout matches the ARM Cortex Debug
standard with SWO/TDI repurposed for UART RX/TX (common convention on
probes with integrated serial):

| Pin | Function | Direction | Notes |
|----:|---|---|---|
| 1 | VTref | target → probe | Powers level-shifter B-side |
| 2 | SWDIO | bidirectional | Via LXC1T45, DIR = `SWD_DIR` |
| 3 | GND | — | |
| 4 | SWCLK | probe → target | Via LXC1T45, DIR = VCCA (always A→B) |
| 5 | GND | — | |
| 6 | UART_RX | target → probe | Via LXC1T45, DIR = GND (B→A) |
| 7 | NC (key) | — | Polarization |
| 8 | UART_TX | probe → target | Via LXC1T45, DIR = VCCA (A→B) |
| 9 | GNDDetect | — | Short to GND on the probe |
| 10 | nRESET | bidirectional open-drain | **Direct to MCU FT_c GPIO**, no shifter |

### Level shifter bank

Four `SN74LXC1T45QDRYRQ1` (USON-6, 1.45×1 mm, Ioff + V₀₀ disconnect,
1.1–5.5 V both ports). All share:
- VCCA = 3V3 (probe side)
- VCCB = VTref (target side)
- 100 nF 0402 on each VCCA and VCCB pin

| Shifter | DIR wiring | A (MCU side) | B (target side) |
|---|---|---|---|
| U_LS_SWDIO | GPIO `SWD_DIR` | PA4 (`SWD_A`) | Header pin 2 |
| U_LS_SWCLK | tied to VCCA (always A→B) | PA5 (`SWCLK_A`) | Header pin 4 |
| U_LS_UTX | tied to VCCA (always A→B) | PB6 (USART1_TX) | Header pin 8 |
| U_LS_URX | tied to GND (always B→A) | PB7 (USART1_RX) | Header pin 6 |

Firmware controls `SWD_DIR`:
- High → MCU drives SWDIO (write/address phase).
- Low → MCU samples SWDIO (read/ack phase).

The SWD turnaround timing is deterministic per the ADI v5 spec, so this
is a trivial bit-bang sequence in the CMSIS-DAP SWD loop.

### nRESET handling

No level shifter. MCU drives header pin 10 directly from a **FT_c**
(5 V tolerant regardless of VDD) GPIO, configured as:
- **Output**: open-drain, drive low to hold target in reset; Hi-Z
  otherwise.
- **Input**: read to observe external resets.

FT_c pin assignment: **PD0**. Only four FT_c pins on the UFQFPN-32
'N' package (PA8, PB15, PD0, PD2, per DS13560 Table 15); PA8/PB15 are
UCPD CC1/CC2, PD2 is reserved as a spare FT_c GPIO, leaving PD0 for
nRESET. FT_c input voltage limit is 5.0 V regardless of VDD per
DS13560 §6.3.15.

DNP pull-up (10 kΩ) from header pin 10 to VTref for targets without an
internal reset pull-up.

### Debug (probe self-programming)

- **Tag-Connect TC2030-IDC-NL** SWD footprint on the G0B1's SWD
  (PA13/PA14). Unpopulated; pads only.
- **NRST button** (B3U-1000P): NRST → GND. 100 nF filter cap NRST→GND
  per AN4879.
- **BOOT0 button** (B3U-1000P): PA14/BOOT0 → 3V3 with 10 kΩ pull-down
  keeping the pin low when released. Hold BOOT0 + tap NRST → STM32 ROM
  USB DFU bootloader for self-reflash.

### User I/O — LEDs

All driven by MCU GPIO (current-sourcing: GPIO → LED anode → series R →
GND, active-high). Three colors total (no white):

| LED | Color | Function | Default behavior | Typ Vf @ I | Series R |
|---|---|---|---|---|---|
| D_STATUS  | green  | Probe alive        | 1 Hz heartbeat         | 1.9 V @ 2 mA | 680 Ω |
| D_DAP     | amber  | SWD activity       | 5 ms pulse per DAP xfer| 1.9 V @ 2 mA | 680 Ω |
| D_UART_TX | blue   | Host → target byte | 5 ms pulse per byte    | 2.6 V @ 2 mA | 330 Ω |
| D_UART_RX | blue   | Target → host byte | 5 ms pulse per byte    | 2.6 V @ 2 mA | 330 Ω |

All resistors 0402, 1 %. Sizing is `(3V3 − Vf) / 2 mA` rounded to E24;
final values may be trimmed during bringup for equal perceived
brightness across colors.

Follow the repo's `Led` generic pattern (LED + 0402 series resistor
per LED).

---

## 6. Key components

| Function | Part | Package | Source |
|---|---|---|---|
| MCU | STM32G0B1KBU6N (128 KB, N-pinout) | UFQFPN-32 (5×5) | `components/STMicroelectronics/STM32G0B1KxUxN` |
| USB-C receptacle | GCT USB4105-GF-A via `UsbC16P` | 16-pin SMT | `connectors/UsbC16P/UsbC16P.zen` |
| USB/CC ESD | TPD4E05U06QDQARQ1 | USON-10 (2.5×1) | `components/TPD4E05U06QDQARQ1` |
| VBUS TVS | SMAJ5.0A or equivalent | SMA | stdlib / EE choice |
| 3V3 LDO | TPS74x01P | SON-6 (DRV) | `components/Texas_Instruments/TPS74x01P` |
| Level shifters (×4) | SN74LXC1T45QDRYRQ1 | USON-6 (1.45×1) | Not yet in registry — import via `pcb new component` during implementation |
| Target header | FTSH-105-01-L-DV-K-A-P-TR | SMT, keyed | Registry (to be published by user) |
| Buttons (×2) | Omron B3U-1000P | SMT | `components/B3U-1000P` |
| LEDs | stdlib `Led` generic | 0402 | stdlib |
| SWD footprint | Tag-Connect TC2030-IDC-NL | pads only | `connectors/TagConnect/TC2030-NL_SWD.zen` |

### Support passives (commodity, must be on the board)

- **MCU decoupling**: 100 nF 0402 on each VDD pin (VDD ×2, VDDA,
  VDDIO2/VDDUSB) + 1 × ≥ 1 µF bulk near MCU, per AN4879.
- **NRST filter**: 100 nF 0402, NRST → GND (AN4879 glitch immunity).
- **BOOT0 pull-down**: 10 kΩ, PA14/BOOT0 → GND.
- **VBAT tie**: short VBAT to VDD (no RTC backup).
- **VBUS bulk**: 4.7–10 µF ceramic near USB-C receptacle.
- **LXC1T45 decoupling (×4)**: 100 nF 0402 on VCCA and VCCB of each
  shifter.
- **SWD_DIR idle state**: 10 kΩ pull-down from `SWD_DIR` to GND so the
  shifter defaults to B→A (read) when MCU is in reset — neutral state
  that never back-drives the target.
- **Target nRESET DNP pull-up**: 10 kΩ header-pin-10 → VTref, DNP.
- **LED current-limit resistors**: per-color sized in the Zener source
  for roughly equal brightness at 3.3 V.

---

## 7. Mechanical & environmental

### Form factor

- Dongle-style PCBA, target **~25 mm × 55 mm**, corners 1 mm radius.
- USB-C receptacle on one short edge, flush with board edge.
- FTSH-105 10-pin header on the opposite short edge, with the
  connector locator slot per Samtec drawing 145-0032.
- Top-side components only except optional decaps (EE discretion); no
  bottom-side connectors.
- No mounting holes.
- Silkscreen: board name + version + pin-1 indicators on the 10-pin
  header and USB-C; LED labels adjacent to each LED.

### Stack-up

4-layer, 1.6 mm FR4, 1 oz copper all layers:

| Layer | Purpose |
|---|---|
| L1 (top) | Signals + components |
| L2 | Solid GND reference |
| L3 | 3V3 pour + VTref pour (split by domain), short signal runs OK |
| L4 (bottom) | Signals + test points; secondary GND pour |

Rationale: L2 solid GND gives a clean reference for the 90 Ω USB
differential pair on L1 and level-shifter return currents; L3 split
pour isolates the target voltage domain from the probe rail.

### Controlled impedance

- **USB D+/D-**: 90 Ω ±10 % differential on L1 referenced to L2 GND.
  Keep < 50 mm from receptacle to MCU; no stubs; length-match within
  ± 0.5 mm.
- All other signals: default netclass.

### Environmental

- Operating: 0 °C – 70 °C, indoor dry use.
- Storage: −20 °C – 85 °C.
- Mounting: free-floating dongle.
- No conformal coating.

---

## 8. Manufacturing & assembly

- Layer count: **4**.
- Min trace/space: 6/6 mil. Min via: 0.25 mm finished.
- Min package: 0402 passives; USON (1.45×1 mm) actives; USON-10 and
  UFQFPN-32 largest ICs.
- Assembly: single-pass SMT reflow. No through-hole.
- Prototype quantity: ~5–10.
- US assembly: no. ITAR: no.

---

## 9. Regulatory & compliance

- Not a sold product. FCC / CE / UL not targeted.
- USB-IF certification not pursued. VID:PID: development range
  (pid.codes) for initial firmware.

---

## 10. Back-feed / unpowered-probe resilience

Requirement R8: the probe must not back-feed current into the target
when USB is unplugged but the 10-pin header is connected to a live
target.

Audit of every target → probe path:

| Path | Mitigation |
|---|---|
| VTref → LXC1T45 VCCB | Shifter draws ~10 µA quiescent per chip = ~40 µA total. Acceptable target load. No path into probe 3V3 because VCCA is 0 V on all four shifters. |
| SWDIO (target out) → MCU GPIO | LXC1T45 has **Ioff + V₀₀ disconnect**: when VCCA drops below 100 mV, the A-side I/O is briefly pulled low then goes Hi-Z. No back-drive into MCU or 3V3. |
| SWCLK / UART / SWD_DIR | Shifters Ioff for A-side; direction is MCU→target so target never drives these B-sides in the first place. |
| nRESET → MCU PD0 | PD0 is **FT_c** (5 V tolerant regardless of VDD per DS13557 §6.3.15) — no ESD clamp to VDD, so a target-driven high on nRESET does not charge 3V3. |
| VTref → 3V3 rail | No direct electrical path. No op-amp, no voltage divider, no diode. |
| USB D+/D- | USB disconnected; no path. |

**Result**: no backfeed. No bleeder resistor, no PMOS isolator, no
Schottky — not needed once the ADC buffer and BSS138 nRESET shifter are
eliminated.

Verification during bringup: apply 5 V to VTref and 5 V pull-up to
nRESET with USB unplugged; confirm probe 3V3 stays < 100 mV and probe
draws < 100 µA from target.

---

## 11. Open items (non-blocking)

- **Series damping on SWDIO/SWCLK**: 33 Ω 0402 footprints in-line
  between the level-shifter B-side and the header, populated by
  default. At 5 V B-side the LXC1T45 driver impedance is ~20 Ω; 33 Ω
  series + typical 10 pF cable gives ~0.4 ns rise-time with minimal
  ringing for 10–20 cm flying leads. Can be shorted with 0 Ω on bench
  setups with short stubs.
- **VBUS TVS part**: SMAJ5.0A (SMA) vs. PESD5V0S1BA (SOD-323). Either
  satisfies R13; EE picks by layout real-estate.
- **Production VID:PID**: replace pid.codes default once firmware
  stabilizes.

---

## 12. Design notes

1. **SWD level shifter direction control.** SN74LXC1T45 is directional
   with an external DIR pin, 1.1–5.5 V on both VCCA and VCCB (covers
   1.8 V / 3.3 V / 5 V targets in one part). For SWDIO we route DIR to
   a dedicated GPIO (`SWD_DIR`) that firmware toggles around SWD
   turnaround phases per ADI v5. For SWCLK and UART_TX, DIR is
   hard-tied to VCCA (permanently A→B). For UART_RX, DIR is
   hard-tied to GND (permanently B→A). This gives deterministic,
   high-bandwidth, push-pull level translation with none of the
   flakiness of auto-direction parts (TXB/TXS series).

2. **VTref powers only the level shifters.** The probe does not measure
   VTref. The target provides the B-side rail via header pin 1 and
   signals automatically scale with whatever voltage the target runs.
   Total current draw from target is ~40 µA quiescent. No buffer / ADC.

3. **Backfeed resilience through part selection, not added circuitry.**
   LXC1T45 Ioff + V₀₀-disconnect, and FT_c nRESET, eliminate every
   path from target to probe 3V3 rail when USB is unplugged. No
   bleeder resistor, no PMOS isolator. See §10.

4. **FT_c pin for nRESET.** The target can hold nRESET at up to 5.0 V
   (weak pull-up on a 5 V target). MCU GPIO on nRESET must tolerate
   5 V even when VDD = 0 (probe unpowered, target live). Only PA8,
   PB15, PD0, PD2 are FT_c / FT_cs on the UFQFPN-32 'N' package.
   PA8/PB15 are UCPD CC1/CC2; PD2 is kept as a spare FT_c for future
   use. **PD0** is assigned to target nRESET. DS13560 §6.3.15
   guarantees operating V_IN from −0.3 V to 5.0 V on FT_c; targets
   whose nRESET pull-up ties to > 5.0 V (rare) are out of spec.

5. **UCPD dead-battery.** STM32G0B1 'N' variant has UCPD1 dead-battery
   Rd active from POR. External 5.1 kΩ Rd resistors are **not** placed
   (CC pins go directly to USB_C.CC1 / USB_C.CC2). Per AN5225 §11.3.1,
   DBCC pins are externally shorted to matching CC pins: PA9↔PA8
   (CC1), PA10↔PB15 (CC2). Firmware must not touch `UCPD1_STROBE`
   unless explicitly configuring UCPD1 as a sink.

6. **UCPD2 dead-battery mitigation.** PD0/PD1/PD2/PD3 double as UCPD2
   CC/DBCC. PD0 drives the target nRESET line, which a live target
   can hold at VTref while the probe is still running — UCPD2
   hardware may interpret that as a CC wake and engage ~5.1 kΩ
   internal pull-downs on PD0/PD2 at reset (DS13560 Table 12 note 4:
   the Rd on PD0 is gated by PD1; the Rd on PD2 is gated by PD3).
   Hardware mitigation: 100 kΩ pull-downs on PD1 (`R_UCPD2_DBCC1`)
   and PD3 (`R_UCPD2_DBCC2`) to GND force both DBCC inputs low at
   POR, so neither internal Rd is enabled. Firmware must still
   release UCPD2 dead-battery early after boot:
   `SYSCFG->CFGR1 |= SYSCFG_CFGR1_UCPD2_STROBE`. Do not set
   `UCPD1_STROBE` unless UCPD1 has been explicitly configured as a
   sink.

7. **SWD_DIR idle.** `SWD_DIR` is pulled to GND (10 kΩ) at the MCU so,
   during MCU reset, the SWDIO shifter is in B→A (read) mode — Hi-Z
   on the target side (A-side Ioff during reset is N/A because VCCA
   is present; but with DIR = B→A the A-side is an input and the
   B-side output is effectively Hi-Z with respect to a driven target).

8. **No BSS138 on nRESET.** DM0002 uses a BSS138 FET shifter for
   nRESET. Seward eliminates it by using an FT_c MCU GPIO directly in
   open-drain mode — one fewer FET, no pull-up on the MCU side, and
   eliminates the body-diode backfeed path the BSS138 topology would
   otherwise create when the probe is unpowered and the target pulls
   nRESET high.

9. **Crystal-less USB.** G0B1 HSI48 + CRS trim against USB SOF packets.
   No HSE crystal.

10. **D+ pull-up.** G0B1 integrates the 1.5 kΩ D+ pull-up on-die
    (AN4879 §3.1.1). No external pull-up.

11. **Power LED omitted on purpose.** No LED indicates 3V3 or VBUS —
    per explicit requirement. `D_STATUS` (green heartbeat) is the
    nearest equivalent and actually indicates firmware is running, not
    just that a rail is up.

12. **Firmware update path.** Primary: USB DFU via STM32 system memory
    bootloader. Hold BOOT0 + tap NRST → host sees STM32 DFU device →
    `dfu-util` / `STM32CubeProgrammer`. Tag-Connect SWD is factory /
    recovery only; no connector populated on shipped boards.

13. **GPIO budget.** USB (2) + UCPD CC/DBCC (4) + SWD (2) + target
    SWDIO/SWCLK/SWD_DIR (3) + target UART TX/RX (2) + nRESET (1) +
    4 LEDs + BOOT0 reserved = 19 pins. Comfortable in UFQFPN-32.

14. **SWD bit-bang performance.** CMSIS-DAP over USB FS (12 Mbps) is
    host-throughput-limited well before MCU bit-bang speed on a
    64 MHz Cortex-M0+. Target 10 MHz SWCLK out of the level shifter;
    LXC1T45 supports ≥ 100 MHz push-pull translation. No
    peripheral-assist (SPI/TIM) required for v1 firmware.

15. **SWD turnaround sequence.** With `SWD_DIR` as a separate GPIO:
    - Write: `SWD_DIR = 1` → clock 32 data + 1 parity.
    - Turnaround: set `SWD_DIR = 0` during the turnaround bit
      (1 SWCLK cycle) before the read phase.
    - Read: `SWD_DIR = 0` → sample SWDIO on each SWCLK rising edge.

    Matches ADI v5 §4.4 SWD protocol exactly. The MCU's `SWD_A` GPIO
    must be set to input when `SWD_DIR = 0` (SWDIO sourced by the
    target through B→A); the LS A-side goes Hi-Z when the B→A driver
    is inactive which is fine, but leaving the MCU GPIO as an output
    while DIR=0 causes contention internal to the LXC1T45. Firmware
    reconfigures the GPIO direction in lock-step with `SWD_DIR`.

16. **No target power from probe.** Unlike Feign (which current-limits
    5 V to the UART header), Seward never sources power onto the
    10-pin header. Pin 1 is an *input* (VTref sense) only. Target is
    always externally powered.

17. **Spare GPIOs broken out to test points.** PA7, PB0, PD1, PD2,
    PA15, PB3, PB4, PB5 land on L4 pads. Enables future firmware
    features (current-sense ADC hook, JTAG TDI/TDO, extra LED,
    LPUART1 modem lines) without re-spinning the board.

---

## 13. Pin assignment (detailed)

STM32G0B1KxUxN, UFQFPN-32 'N' variant. Pin numbers per DS13560
Table 15, LQFP32/UFQFPN32-N column.

| # | Port | I/O struct | Net | Role |
|---:|---|---|---|---|
| 1 | PC14 | FT | — | Spare (cfg as analog in FW) |
| 2 | PC15 | FT | — | Spare |
| 3 | VDD | — | 3V3 | Core + I/O supply |
| 4 | PF2-NRST | — | NRST | MCU reset (button + 100 nF cap) |
| 5 | PA0 | FT | LED_STATUS | Active-high green LED |
| 6 | PA1 | FT | LED_DAP | Active-high amber LED |
| 7 | PA2 | FT_a | LED_UART_TX | Active-high blue LED |
| 8 | PA3 | FT_ea | LED_UART_RX | Active-high blue LED |
| 9 | PA4 | FT_a | SWD_A | SWDIO A-side to U_LS_SWDIO |
| 10 | PA5 | FT_a | SWCLK_A | SWCLK A-side to U_LS_SWCLK |
| 11 | PA6 | FT_a | SWD_DIR | Direction control for U_LS_SWDIO |
| 12 | PA7 | FT_fa | — | Spare → TP |
| 13 | PB0 | FT_fa | — | Spare → TP |
| 14 | VDDA/VSSA | — | 3V3 / GND | Tie VDDA to 3V3 through a BLM15 ferrite; VSSA to GND |
| 15 | PA8 | FT_c | USB_C.CC1 | UCPD1_CC1 |
| 16 | PA9 | FT_a | USB_C.CC1 | UCPD1_DBCC1, shorted to PA8 |
| 17 | PA10 | FT_a | USB_C.CC2 | UCPD1_DBCC2, shorted to PB15 |
| 18 | PA11 | FT | USB_C.D.N | USB_DM |
| 19 | PA12 | FT | USB_C.D.P | USB_DP |
| 20 | PA13 | FT_a | SWD.SWDIO | MCU self-SWD (Tag-Connect) |
| 21 | PA14/BOOT0 | FT_a | SWD.SWCLK / BOOT0 | SWCLK + BOOT0 button |
| 22 | PA15 | FT_fa | — | Spare → TP |
| 23 | PB3 | FT_fa | — | Spare → TP |
| 24 | PB4 | FT_fa | — | Spare → TP |
| 25 | PB5 | FT_fa | — | Spare → TP |
| 26 | PD0 | FT_cs | TGT_nRESET | Target nRESET, open-drain, **5 V tolerant** |
| 27 | PD1 | FT | — | Spare → TP |
| 28 | PD2 | FT_cs | — | Spare (reserve FT_c for future) → TP |
| 29 | PB6 | FT_fa | UART_TX_A | USART1_TX to U_LS_UTX |
| 30 | PB7 | FT_fa | UART_RX_A | USART1_RX from U_LS_URX |
| 31 | PB15 | FT_c | USB_C.CC2 | UCPD1_CC2 |
| 32 | VSS + EP | — | GND | Ground + exposed pad (thermal + electrical) |

### Decoupling (per AN4879 §3.2)

| Pin | Cap |
|---|---|
| VDD × {each instance} | 100 nF 0402 within 2 mm |
| VDDA | 100 nF 0402 + 1 µF 0603, tied to 3V3 through BLM15 ferrite |
| 3V3 bulk | 4.7 µF 0805 near LDO output + 1 µF 0603 near MCU |
| VBUS | 4.7 µF 0805 ceramic near USB-C receptacle |
| LXC1T45 VCCA (×4) | 100 nF 0402 each, placed on the MCU side of the LS |
| LXC1T45 VCCB (×4) | 100 nF 0402 each, placed on the target side of the LS |

---

## 14. Bring-up / verification checklist

### Visual / continuity (no power)

- [ ] VBUS / 3V3 / GND not shorted to each other.
- [ ] CC1/CC2 not shorted to GND (no 5.1 kΩ populated).
- [ ] Header pin 1 (VTref) isolated from 3V3.
- [ ] Header pin 7 (key) not connected to a signal.

### Power-up (USB, no target)

- [ ] VBUS = 5 V ± 5 %.
- [ ] 3V3 = 3.30 V ± 2 %.
- [ ] USB idle current < 30 mA.
- [ ] D_STATUS green LED begins heartbeat blink.

### USB enumeration

- [ ] Host enumerates Vendor + CDC-ACM composite device.
- [ ] Windows auto-binds WinUSB via MS OS 2.0 descriptors.
- [ ] `pyOCD list` / `probe-rs list` shows a unique serial.

### Target header, no target connected

- [ ] All B-side signals float (Hi-Z on DMM).
- [ ] VTref = 0 V (no back-feed from probe).
- [ ] nRESET floats (PD0 input mode).

### Back-feed test (R8 verification)

1. Unplug USB from probe.
2. Apply 3.3 V to header pin 1 (VTref) from bench supply, 1 mA limit.
3. Apply 3.3 V to header pin 10 through 10 kΩ to bench supply.
4. Probe 3V3 rail < 100 mV (DMM).
5. Bench supply current < 200 µA.
6. Repeat at VTref = 5.0 V — same pass criteria.

### SWD functional at 3.3 V

- [ ] Known Cortex-M target: `pyOCD cmd` returns correct IDCODE.
- [ ] Scope SWCLK: 10 MHz, full 3.3 V p-p during DAP transactions.
- [ ] Scope SWDIO: bidirectional, no contention spikes at turnaround.
- [ ] Flash a test blob, read back, verify.

### SWD at 1.8 V and 5 V

- [ ] 1.8 V target: SWCLK amplitude = 1.8 V p-p, IDCODE correct.
- [ ] 5 V target: SWCLK amplitude = 5 V p-p, IDCODE correct.

### UART bridge

- [ ] `/dev/ttyACM*` enumerates alongside CMSIS-DAP.
- [ ] Loopback jumper at header pins 6–8: echoed bytes at 115200 and
      921600 bps.
- [ ] Verify amplitude translation at 1.8 V, 3.3 V, 5 V VTref.

### Self-programming

- [ ] Hold BOOT0 + tap NRST → STM32 DFU enumerates at `0483:df11`.
- [ ] Flash a blink test via `dfu-util`.

---

## 15. Test points

0.8 mm L4 pads, labelled in silk:

| Net | Label | Purpose |
|---|---|---|
| VBUS | TP_VBUS | USB 5 V sanity |
| 3V3 | TP_3V3 | Probe rail |
| GND | TP_GND | DMM / scope return |
| VTref | TP_VTREF | Target rail sense |
| SWDIO (B) | TP_SWDIO | Scope / protocol analyzer on target side |
| SWCLK (B) | TP_SWCLK | Scope |
| SWD_DIR | TP_DIR | Watch DIR toggling during bringup |
| nRESET | TP_nRST | Target reset line |
| UART_TX (B) | TP_UTX | Target-side UART |
| UART_RX (B) | TP_URX | Target-side UART |

Plus spare-GPIO test points PA7, PB0, PA15, PB3, PB4, PB5, PD1, PD2.

---

