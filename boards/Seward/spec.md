# Seward — Design Specification

A small, reliable CMSIS-DAP debug probe based on the STM32G0B1. Exposes
a 10-pin ARM Cortex-Debug target header with bidirectional SWD and an
integrated UART bridge. Target-side I/O rails auto-range to support
1.8 V, 3.3 V, and 5 V targets on a single probe.

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
| R4 | Target-side I/O rails auto-range over 1.8–5.0 V (VTref-driven, nRESET clamp) | P0 |
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
 │         │   │  VCCB = VTref (auto 1.8–5.0 V)       │                    │
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

| Rail | Voltage | Source | Budget |
|---|---|---|---|
| VBUS | 5 V USB | USB-C receptacle | < 100 mA total |
| 3V3 | 3.3 V | TPS74x01P LDO from VBUS | ~30 mA peak |
| VTref | 1.8–5.0 V | **Target-driven** (header pin 1) | ≤ 100 µA target-sourced |

Probe is bus-powered; never sources power onto the target header. VTref
feeds only the level-shifter VCCB rails — the probe does not sense,
buffer, or regulate it.

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
| 10 | nRESET | probe → target (open-drain) | **Direct to MCU FT_c GPIO**, no shifter; drive-only |

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
(5 V tolerant regardless of VDD) GPIO as open-drain: drive low to hold
target in reset, Hi-Z otherwise. The probe does not read target nRESET
state — the MCU input threshold is referenced to the 3V3 probe rail,
which wouldn't register a 1.1/1.8 V target-driven high. CMSIS-DAP only
needs to assert/release reset.

FT_c pin assignment: **PD0**. Only four FT_c pins on the UFQFPN-32
'N' package (PA8, PB15, PD0, PD2, per DS13560 Table 15); PA8/PB15 are
UCPD CC1/CC2, PD2 is reserved as a spare FT_c GPIO, leaving PD0 for
nRESET. FT_c input voltage limit is 5.0 V regardless of VDD per
DS13560 §6.3.15.

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

| LED | Color | Function |
|---|---|---|
| D_STATUS  | green  | Probe heartbeat        |
| D_DAP     | amber  | SWD activity           |
| D_UART_TX | blue   | Host → target byte     |
| D_UART_RX | blue   | Target → host byte     |

---

## 6. Key components

| Function | Part | Package |
|---|---|---|
| MCU | STM32G0B1KBU6N (128 KB, N-pinout) | UFQFPN-32 |
| USB-C receptacle | GCT USB4105-GF-A via `UsbC16P` | 16-pin SMT |
| USB/CC ESD | TPD4E05U06QDQARQ1 | USON-10 |
| VBUS TVS | 5–6 V standoff unidirectional (stdlib `Tvs` generic) | SMF |
| 3V3 LDO | TPS74x01P | SON-6 |
| Level shifters (×4) | SN74LXC1T45-family (LXC selected via `SN74x1T45-DRY` module) | USON-6 |
| Target header | FTSH-105-01-L-DV-K-A-P-TR | SMT, keyed |
| Buttons (×2) | Omron B3U-1000P | SMT |
| LEDs | stdlib `Led` generic | 0402 |
| SWD footprint | Tag-Connect TC2030-IDC-NL | pads only |

Support passives (pull-ups/downs, decoupling, series damping) are
managed in `Seward.zen` rather than enumerated here — that file is the
single source of truth for the parts list.

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

- **Production VID:PID** — firmware currently defaults to pid.codes
  `0x1209 / 0x0001`. Replace once firmware stabilizes.
- **Layout-stage controlled-impedance review** — confirm 90 Ω
  differential USB on L1 after first layout pass.

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

14. **SWD bit-bang performance / turnaround.** USB FS is the
    throughput bottleneck long before the Cortex-M0+ bit-bang loop.
    Target ~10 MHz SWCLK; LXC1T45 supports much faster push-pull
    translation. `SWD_DIR` toggles in lock-step with the MCU GPIO
    direction around ADI v5 turnaround cycles to avoid internal LS
    contention. Firmware concern; no extra silicon.

15. **No target power from probe.** Seward never sources power onto
    the 10-pin header. Pin 1 is an *input* (VTref sense) only.

16. **Spare GPIOs.** ~8 unused pins (PA7, PB0, PA15, PB3, PB4, PB5,
    PD1, PD2) can be broken out to test points for future firmware
    features. Exact set is at EE discretion during layout.

---

## 13. Bring-up / verification checklist

### Power / USB

- [ ] VBUS = 5 V ± 5 %, 3V3 = 3.30 V ± 2 %, USB idle < 30 mA.
- [ ] D_STATUS green LED heartbeating.
- [ ] Host enumerates Vendor + CDC-ACM composite; WinUSB auto-binds.
- [ ] `pyOCD list` / `probe-rs list` shows a unique serial (G0B1 UID).

### Target header, no target

- [ ] All B-side signals float, VTref = 0 V, nRESET floats.

### Back-feed test (R8 verification)

1. Unplug USB from probe.
2. Apply 3.3 V to header pin 1 (VTref) from bench supply, 1 mA limit.
3. Apply 3.3 V to header pin 10 through 10 kΩ to bench supply.
4. Probe 3V3 rail < 100 mV (DMM).
5. Bench supply current < 200 µA.
6. Repeat at VTref = 5.0 V — same pass criteria.

### SWD / UART at 1.8 V, 3.3 V, 5 V targets

- [ ] Known Cortex-M target: correct IDCODE; scope SWCLK full p-p at
      VTref; flash + readback round-trip OK.
- [ ] UART loopback at 115200 and 921600 bps.

### Self-programming

- [ ] Hold BOOT0 + tap NRST → STM32 DFU enumerates at `0483:df11`.
- [ ] `dfu-util` flashes a blink test.

---
