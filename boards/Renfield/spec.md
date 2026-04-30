# Renfield (DM0005) — Design Specification

A USB-C Power Delivery **sink** development board built around the
STM32G0B1, designed first and foremost for **firmware development on
the PD sink stack**. Negotiates up to 20 V / 5 A from a USB-PD source and
delivers the negotiated rail to a downstream test load over an SMD
push-in terminal block. Optimized for live observability — every signal
the firmware engineer cares about is on a probe header, a Tag-Connect
debug pad, a UART header, or a colored solder-loop test point.

The board "consumes" its master's power on demand for whatever load you
attach. Hence Renfield.

---

## 1. Purpose

A standalone bench-top dev/eval board for the STM32G0B1's UCPD-based
USB-PD sink stack (X-CUBE-USB-PD3 / X-CUBE-TCPP class firmware). It is
**not a product** — it is the platform on which we develop and debug PD
sink firmware before deploying it on a Feign-class production board.

Primary use cases:

1. Plug Renfield into any USB-PD source and run firmware that negotiates
   contracts up to 20 V / 5 A (60 W maximum design point).
2. Observe the BMC traffic on CC1/CC2 in real time on a Saleae Logic Pro 8
   (analog inputs).
3. Watch live VBUS voltage on an 8-LED RGB bargraph without external
   instruments. Current measurement is intentionally **not on this
   board** — use an external smart load (DC e-load, USB-PD analyzer,
   etc.) for current/power telemetry.
4. Reconfigure firmware behavior at runtime via 4 DIP switches without
   reflashing.
5. Stress-test firmware by intentionally crashing it mid-negotiation —
   the board's hardware OVP and load-side eFuse protect both the
   attached load and the board itself.
6. Reflash firmware over USB DFU (BOOT0 + RESET buttons), SWD via Tag-
   Connect, or printf-trace over the dedicated UART header.

---

## 2. Requirements summary

| ID  | Requirement | Priority |
|-----|-------------|----------|
| R1  | USB-C 16P receptacle, sink role, supports negotiated 5/9/15/20 V at up to 5 A | P0 |
| R2  | STM32G0B1KEU6N (UFQFPN-32, N-pinout, 512 KB flash) runs the entire PD sink stack on UCPD1 | P0 |
| R3  | TCPP01-M12 + external NexFET on VBUS provide hardware OVP/CC OVP/IEC ESD on CC + VBUS | P0 |
| R4  | TPD4E05U06QDQARQ1 provides IEC 61000-4-2 L4 ESD on D+/D- (TCPP01-M12 covers CC only) | P0 |
| R5  | Hardware VBUS OVP threshold = 22 V; CC OVP = 6 V (TCPP01-M12 internal) | P0 |
| R6  | Load-side TPS25948x eFuse with EN default-low, OVLO = 22 V, latch-on-fault | P0 |
| R7  | Auto-retry mode for eFuse selectable via DNP cap footprint (silk-labeled) | P1 |
| R8  | VBUS÷10 divider → MCU ADC for voltage. **No on-board current measurement** — user supplies external smart load if current/power telemetry is needed. | P0 |
| R9  | One 8-segment RGB bargraph (V: 0–20 V) using IN-PI15 LEDs | P0 |
| R10 | 7 discrete 0603 status LEDs: 2 rail-direct (5V, 3V3), 2 hardware-driven from open-drain fault pins (TCPP01_FLG, eFuse_FLT), 3 MCU GPIO-driven (PD_CONTRACT, USB_ENUM, HEARTBEAT) | P0 |
| R11 | 4-position SMD DIP switch direct-driven on MCU GPIOs (no I²C expander) | P0 |
| R12 | Two side-by-side Saleae-friendly 2×4 0.1″ probe headers (8 signal lines total) | P0 |
| R13 | Dedicated 1×4 0.1″ UART debug header (GND/TX/RX/3V3) | P0 |
| R14 | Tag-Connect TC2030-IDC-NL SWD pads with SWO trace (no connector populated) | P0 |
| R15 | SMD test pads for nets not on a probe/UART header (no Keystone loops) | P0 |
| R16 | USB DFU bootloader entry via BOOT0 + RESET tactile buttons | P0 |
| R17 | Crystal-less USB via HSI48 + CRS (no HSE) | P0 |
| R18 | TPSM33606S5 buck (VBUS→5 V, integrated inductor) feeding TPS74x01P LDO (5 V→3V3) | P0 |
| R19 | WAGO 2060-452 SMD push-in 2-pole terminal block as test-load output | P0 |
| R20 | A7 form factor (74 × 105 mm), 4-layer board, 4× M3 corner mounting holes | P0 |
| R21 | Fully SMT, single-pass reflow, all parts T&R, US-stocked | P0 |
| R22 | 60 W maximum design point (20 V × 3 A or 12 V × 5 A — never both) | P0 |

P0 = must have. P1 = should have.

---

## 3. System architecture

```
USB-C 16P (sink, 5–20 V / 0–5 A)
   │
   │ VBUS  D+  D-  CC1  CC2  GND
   ▼
┌──────────────────┐                       ┌─────────────────┐
│   TCPP01-M12     │  CC1, CC2 ──────────► │  STM32G0B1KxUxN │
│   - CC OVP 6 V   │                       │  UFQFPN-32, 512K│
│   - VBUS OVP 22V │  GATE→ ┌──────────┐   │  N-pinout       │
│   - ESD L4 (CC)  │ ──────►│CSD17318Q2│   │                 │
│   - Dead-batt Rd │        │ N-FET 2x2│   │  Peripherals    │
│   - FLG/ OD      │ ──────►│ ~20 mΩ   │   │  (pin assign    │
└──────────────────┘        └────┬─────┘   │   deferred to   │
        │                        │ VBUS_PROT│  capture):     │
        │ FLG/ → LED + MCU       │          │                 │
        │                        ├──► to buck (TPSM33606S5)  │
        │                        │      → 5V → LDO → 3V3    │
D+/D-   │                        │                           │
  │  TPD4E05U06 ──► STM32 USB FS                             │
  │  (IEC ESD L4)                                            │
  │                             ▼                            │
  │                  ┌──────────────────┐                    │
  │                  │ TPS259482 eFuse  │  VBUS_OUT_DIV (÷10)│
  │                  │ 3.5–23 V, 8 A    │───────────────────►├─► ADC_V
  │                  │ EN default low   │                    │
  │                  │ OVLO 22 V        │                    │
  │                  │ ILIM ~6 A        │                    │
  │                  │ FLT/ OD →LED+MCU │                    │
  │                  └────────┬─────────┘                    │
  │                           │ VBUS_OUT                     │
  │                           ▼                              │
  │                       load+ ──────► WAGO 2060-452 ──► load
  │                       load− ──────► WAGO 2060-452 ──► load
  │                       (current/power telemetry: external smart load)
  │                                                          │
  └─── BOOT0 button ──► PA14 (BOOT0)   SWD: PA13/PA14/PA15  │
       RESET button ──► NRST           UART: TX, RX (header)│
                                                            │
       4-pos DIP ────────────────► 4× GPIO inputs           │
       7 status LEDs:                                       │
         - 5V_RAIL, 3V3_RAIL: rail-direct                   │
         - TCPP01_FLG_LED, eFuse_FLT_LED: hardware-driven   │
           from open-drain pins                             │
         - PD_CONTRACT, USB_ENUM, HEARTBEAT: MCU GPIO       │
       1 V-bargraph (8× IN-PI15 WS2812 RGB) → SPI MOSI ─────┘
       Probe headers (2× 2×4) — see §5
       UART header (1×4), Tag-Connect SWD pads, SMD test pads
```

---

## 4. Power

### Input

| Parameter | Value |
|-----------|-------|
| Source | USB VBUS via USB-C receptacle (sink role) |
| Voltage | 5 V default; up to 20 V after PD negotiation |
| Maximum draw | 5 A continuous (limited by FET / eFuse rating, not USB-PD) |
| Worst-case OVP backstop | 22 V at TCPP01-M12 (hardware), 22 V at TPS25948x OVLO |

### Rails

| Rail | Voltage | Source | Budget |
|------|---------|--------|--------|
| VBUS_RAW | 5–20 V | USB-C, before TCPP01 | up to 5 A |
| VBUS_PROT | 5–20 V | After TCPP01-M12 + NexFET, OVP-clamped at 22 V | up to 5 A |
| VBUS_OUT | 5–20 V | After TPS25948x eFuse (current-limited, OVLO-protected) | up to 5 A |
| 5V | 5 V ±2 % | TPSM33606S5 module from VBUS_PROT | ~250 mA peak |
| 3V3 | 3.3 V ±2 % | TPS74x01P LDO from 5 V | ~80 mA peak |

### Budgets

**5 V rail**:
- 8× WS2812 LEDs (V bargraph only): peak ~120 mA at full white, but
  firmware caps brightness ≤ 30 % → ~75 mA typical.
- LDO input: ~80 mA (×3.3/5 ÷ η ≈ 60 mA).
- **Total**: ~135 mA typical, ~320 mA peak. TPSM33606S5 0.6 A part has
  ~2× margin.

**3V3 rail**:
- STM32G0B1KEU6N @ 64 MHz with USB peripheral: ~25 mA peak.
- 7× discrete status LEDs @ 2 mA (3 GPIO + 2 rail-direct + 2 hw-driven): 14 mA.
- TCPP01 VCC + pull-ups + button pull-ups: ~5 mA.
- **Total**: ~45 mA peak. TPS74x01P (500 mA capable) has ~11× margin.

### MCU survives 22 V VBUS via the 5 V rail

The buck input sits on VBUS_PROT (post-TCPP01-FET). At 22 V → 5 V, ~150 mA
the TPSM33606S5 dissipates ~130 mW. Module package handles it with its
exposed pad on standard copper. No heat-sinking required.

### Boot sequence

1. Source attaches and applies default 5 V to VBUS.
2. TCPP01-M12 dead-battery Rd makes the source see a sink.
3. TCPP01-M12 charge pump turns on the NexFET (5 V is below OVP).
4. VBUS_PROT goes live → TPSM33606S5 starts → 5 V rail up.
5. TPS74x01P starts → 3V3 rail up → MCU comes out of reset.
6. MCU asserts TCPP01_VCC, takes UCPD1 control, releases dead-battery Rd
   per AN5225 strobe sequence, starts PD stack.
7. Firmware reads DIP switches directly via 4 GPIO inputs, decides PDO
   request strategy.
8. Firmware enables TPS25948x eFuse (default-disabled at boot).
9. PD contract → VBUS goes to negotiated voltage → eFuse passes it to
   the WAGO output → bargraphs light up.

---

## 5. Interfaces

### USB-C (input side)

GCT USB4105-GF-A, 16-pin USB-2.0 USB-C receptacle. Data role is
**DFP-data-capable** so we can do USB DFU / CDC over the
same connector — but power role is sink-only and dead-battery Rd is
controlled exclusively by the STM32G0B1 UCPD peripheral. No external
5.1 kΩ Rd resistors (UCPD dead-battery does it).

D+/D- routed as 90 Ω differential to STM32 USB FS pins.

ESD on D+/D- and CC1/CC2 is provided by **TCPP01-M12** (IEC 61000-4-2
Level 4: ±8 kV contact / ±15 kV air on CC; VBUS clamping via gated FET).

### Test load output

**WAGO 2060-452/998-404** SMD push-in terminal block, 2-pole, 4 mm pitch,
6 A continuous rated, reflow-able. Strip the end of any bench-eLoad lead
and push it in — no soldering required.

Silk: `LOAD +` and `LOAD −` arrows, plus a polarity warning rectangle.

### Main probe headers (Saleae Logic Pro 8 grommets)

**Two 2×4 0.1″ vertical pin headers**, side by side, signal-and-GND
alternating columns. One Saleae 8-channel grommet plugs onto each
header.

**Header A — "PD" (analog-friendly)**

| Pin | Signal | Notes |
|---|---|---|
| 1 | CC1_MCU (post-TCPP01) | Primary BMC capture, ~1.2 V analog |
| 2 | GND | |
| 3 | CC2_MCU (post-TCPP01) | Other CC line |
| 4 | GND | |
| 5 | SCOPE_MARKER | Firmware-toggled trigger output |
| 6 | GND | |
| 7 | (spare) | DNF — routed to a spare MCU GPIO pad |
| 8 | GND | |

**Header B — "POWER" (analog VBUS sense + digital fault flags)**

| Pin | Signal | Notes |
|---|---|---|
| 1 | ADC_V | VBUS_OUT ÷10 divider, 0–2 V analog |
| 2 | GND | |
| 3 | (spare) | DNF — routed to a spare MCU GPIO pad for future expansion |
| 4 | GND | |
| 5 | eFuse FLT/ | TPS25948x fault edges (open-drain) |
| 6 | GND | |
| 7 | TCPP01 FLG/ | TCPP01-M12 fault edges (open-drain) |
| 8 | GND | |

Header A signals belong on Saleae **analog** inputs (CC swing is ~1.2 V,
below standard digital threshold). Header B carries one analog signal
(ADC_V) and two digital open-drain signals (FLT/, FLG/); all three
work on either Saleae analog or digital channels.

### UART debug header

**1 × 4 0.1″ pin header**, FTDI-Basic-style, no modem lines:

| Pin | Net |
|---|---|
| 1 | GND |
| 2 | UART_TX (MCU output, 3.3 V CMOS) |
| 3 | UART_RX (MCU input, 3.3 V CMOS) |
| 4 | 3V3 |

Pinout chosen so a stock FTDI USB-to-UART cable's GND/TX/RX/VCC mate
directly. Header is intended for printf-trace only — modem lines are
firmware-defined GPIOs at best, not provided here.

### SWD / SWO

Tag-Connect TC2030-IDC-NL footprint, **pads only**, no connector
populated. Six-pin Tag-Connect signal mapping:

| Tag-Connect pin | Signal |
|---|---|
| 1 (Vcc ref) | 3V3 |
| 2 (TMS / SWDIO) | SWDIO |
| 3 (NRST) | NRST |
| 4 (TCK / SWCLK) | SWCLK |
| 5 (GND) | GND |
| 6 (TDO / SWO) | SWO |

The G0B1 supports SWO via SWO pin (PA15 default), giving live ITM trace
without the USB or UART being involved.

### Test pads

**Flat SMD test pads** (~1.5 × 1.5 mm) for nets that aren't on a probe
header but still want a clean probe-tip touchpoint. Silk labels nearby;
**no colored solder loops** (no Keystone parts — too much BOM pain for
the value).

Minimum set (silkscreen labels per pad):

| Pad | Net | Silk warning |
|---|---|---|
| TP1 | VBUS_RAW | `HV ≤ 22V` |
| TP2 | VBUS_PROT | `HV ≤ 22V` |
| TP3 | VBUS_OUT | `HV ≤ 22V` |
| TP4 | VBUS_OUT_DIV (÷10) | none (safe analog) |
| TP5 | CC1_RAW (pre-TCPP01) | `HV ≤ 22V` |
| TP6 | CC2_RAW (pre-TCPP01) | `HV ≤ 22V` |
| TP7 | 5V | none |
| TP8 | 3V3 | none |
| TP9 | TCPP01_VCC | none (also TCPP01 enable) |
| TP10 | eFuse_EN | none |
| TP11 | eFuse IMON | analog out (not routed to MCU; free scope point provided by the eFuse for users wanting on-board current visibility) |
| TP12–15 | GND | none (distributed for probe clips) |

HV pads should be physically separated from low-voltage pads so a probe
clip can't accidentally bridge VBUS to GND or to a 3V3 net.

### User I/O

| Element | Function |
|---|---|
| BOOT0 button | Tactile, pulls BOOT0 high for DFU bootloader entry |
| RESET button | Tactile, pulls NRST low |
| 4-pos DIP switch | Direct-driven on 4 MCU GPIOs; semantics firmware-defined |
| V bargraph | 8 IN-PI15 LEDs, addressable, 0–20 V at 2.5 V/LED |
| Status LEDs | 7 discrete 0603 (see below) — 2 rail-direct + 2 hardware-driven from FLG//FLT/ + 3 MCU GPIO-driven |

### Status LEDs

Never on the WS2812 chain so they remain useful when firmware is broken.
Seven 0603 LEDs total, three driver styles.

| LED | Color | Driver | Behavior |
|-----|-------|--------|----------|
| 5V_RAIL | Green | **Rail-direct** (5 V → R → LED → GND) | On whenever 5 V is present |
| 3V3_RAIL | Green | **Rail-direct** (3V3 → R → LED → GND) | On whenever 3V3 is present |
| TCPP01_FLG_LED | Red | **Hardware-driven** (3V3 → R → LED → FLG/ open-drain) | On when TCPP01-M12 latches a fault |
| eFuse_FLT_LED | Red | **Hardware-driven** (3V3 → R → LED → FLT/ open-drain) | On when TPS259482 latches a fault |
| PD_CONTRACT | Blue | MCU GPIO | Contract negotiated and held |
| USB_ENUM | Blue | MCU GPIO | USB host has enumerated us |
| HEARTBEAT | White | MCU GPIO | Slow blink = MCU alive |

**Why rail-direct, not PG-direct.** The TPSM33606S5 PGOOD pin is
open-drain — it would need an external pull-up and would drive an LED
on *fault*, not on *good*. The TPS74x01P LDO has push-pull PG, but
mixing the two patterns is confusing. Driving both rail LEDs from the
rail itself through a current-limit resistor gives consistent "rail is
up" telemetry with no PG-pin sourcing concerns.

**Why hardware-driven fault LEDs.** Both TCPP01-M12 FLG/ and TPS259482
FLT/ are open-drain outputs that go low on a latched fault. Wiring an
LED between 3V3 and the fault pin (with a current-limit resistor)
gives a per-chip latched-fault indicator that's independent of MCU
state — the LED lights even if firmware has crashed. The MCU reads
the same pin in parallel for software fault handling. Standard
latching-fault pattern.

The three GPIO-driven status LEDs are 0603 with discrete current-limit
resistors (~330 Ω for ~2 mA at VOL ≈ 0).

---

## 6. Key components

| Function | Part | Package |
|----------|------|---------|
| MCU | STMicroelectronics STM32G0B1KEU6N | UFQFPN-32, N-pinout |
| USB-C connector | GCT USB4105-GF-A | 16-pin USB-2.0 receptacle |
| USB-C port protection (CC + VBUS) | STMicroelectronics TCPP01-M12 | QFN-12 (3×3) |
| D+/D- ESD | TI TPD4E05U06QDQARQ1 | USON-10 |
| VBUS gating FET | TI CSD17318Q2 NexFET | WSON-6 (2×2) |
| Buck (VBUS→5 V) | TI TPSM33606S5QRDNRQ1 | HotRod QFN module, integrated inductor |
| LDO (5 V→3.3 V) | TI TPS74x01P | SON-6 |
| Load eFuse | TI TPS259482AYWPR | POWERWCSP (YWP), DSBGA-12 |
| DIP switch | DS04-254-1-04BK-SMT, 4-pos | SMT, 2.54 mm pitch |
| V-bargraph LEDs | Inolux IN-PI15TAT5R5G5B (×8) | 1.5×1.5 mm, 4-pad |
| Status LEDs | 7× generic 0603 (green ×2, red ×2, blue ×2, white ×1) | 0603 |
| Buttons | Omron B3U-1000P (×2) | SMT tactile |
| SWD | Tag-Connect TC2030-IDC-NL | pads only, no connector |
| Test pads | flat SMD pad (~1.5×1.5 mm) | SMT |
| Load output | WAGO 2060-452/998-404 | SMD push-in, 2-pole, 4 mm pitch |
| Probe headers | 2× 2×4 0.1″ pin header | SMT or THT |
| UART header | 1×4 0.1″ pin header | SMT or THT |

### Support passives (commodity, must be present)

- **MCU decoupling**: 100 nF 0402 on each VDD pin (VDD ×2, VDDA, VDDIO2,
  VDDUSB) + 1× ≥ 1 µF bulk near MCU.
- **NRST filter**: 100 nF 0402, NRST → GND.
- **BOOT0 pull-down**: 10 kΩ (so the button defines the only 1-state).
- **VBAT tie**: short to VDD; RTC backup unused.
- **VBUS_RAW bulk**: 4.7–10 µF ceramic near USB-C receptacle (USB 2.0
  spec).
- **VBUS_PROT bulk**: 22 µF X7R ceramic, 50 V (eFuse app-note value).
- **VBUS_OUT bulk**: 22 µF X7R ceramic, 50 V (eFuse app-note value).
- **TCPP01 OVP divider**: external resistor pair sets 22 V threshold.
- **TPS25948x OVLO divider**: external resistor pair sets 22 V threshold.
- **TPS25948x ILIM resistor**: sized for ~6 A current limit (1.2× the
  5 A continuous spec).
- **TPS25948x RETRY pin**: pulled to GND through 0 Ω resistor (latch-off
  by default). **Footprint for an alternative cap to GND is provided
  but DNP**, silkscreened "POPULATE FOR AUTO-RETRY" per R7.
- **TPS25948x EN pull-down**: 100 kΩ to GND (default-off).
- **TCPP01 FLG/, eFuse FLT/ pulled to 3V3 via the status LED**: each
  open-drain pin sees `3V3 → R (~1.5 kΩ) → LED → pin`. The series R
  doubles as both the fault-LED current limit and the open-drain
  pull-up the MCU input needs. EE may add a parallel high-Z pull-up
  (e.g. 100 kΩ) if MCU input timing requires faster recovery.
- **WS2812 chain decoupling**: 100 nF per LED (best practice for
  addressable RGB chains).
- **DIP-switch pull-ups**: 100 kΩ each to 3V3 (or use MCU internal
  pull-ups if firmware enables them — EE choice). Switch shorts to GND
  when closed.
- **VBUS_OUT_DIV ÷10**: 90 kΩ + 10 kΩ from VBUS_OUT to GND — routed to
  an MCU ADC channel (ADC_V) and exposed at TP4 + Header B pin 1.
  Optional 100 Ω + 1 nF RC filter on the ADC tap to attenuate noise.

---

## 7. Mechanical & environmental

- **Form factor**: A7 (74 × 105 mm), single rectangular outline.
- **Stack-up**: 1.6 mm FR4, **4 layers**, 1 oz copper.
- **Layer use** (suggested, EE may revise during layout):
  - Top: signal + components
  - Inner 1: GND plane (continuous, especially under buck switch node)
  - Inner 2: power (5 V + 3V3, with VBUS islands)
  - Bottom: signal + return
- **Mounting**: 4× M3 holes at the corners.
- **Connector layout**:
  - USB-C receptacle on one short edge.
  - WAGO output on the **opposite** short edge.
  - User I/O (DIP, buttons, LEDs) clustered along the centre, oriented
    for desk-top viewing.
  - Probe header(s) placed for easy access without crossing the high-
    current path.
- **Operating temperature**: 0 °C – 50 °C (bench use).
- **Environmental**: indoor dry use, no conformal coating.

---

## 8. Manufacturing & assembly

- Layer count: **4**.
- Min trace/space: 6/6 mil. Min via: 0.25 mm finished.
- Min package: 0402 passives, UFQFPN-32 MCU, 1.5×1.5 mm RGB LEDs,
  QFN-12 (TCPP01-M12), 2×2 WSON-6 (CSD17318Q2). The TPS259482
  POWERWCSP (12-bump, 0.5 mm pitch) is the only BGA-style part on
  the board; standard reflow handles it.
- Assembly: in-house, single-pass SMT reflow. **No through-hole / hand-
  solder steps.** All connectors (USB-C, WAGO, pin headers) are SMT.
- Prototype quantity: ~5–10.
- US assembly: yes. ITAR: no.

---

## 9. Regulatory & compliance

- Not a sold product; FCC / CE / UL not targeted.
- USB-IF certification not pursued. Device will not claim USB-IF logo
  compliance.
- The TCPP01-M12 has been USB-IF-certified as a sink via the
  X-NUCLEO-SNK1M1 (TID 5205) — that's the chip's compliance, not ours.

---

## 10. Open items (non-blocking, deferred to capture / firmware)

- **Pin assignment** is deliberately not specified. EE picks during
  schematic capture against the STM32G0B1KEU6N (UFQFPN-32, N-pinout),
  with the constraints that:
  - **UCPD1 dead-battery wiring** per AN5225 §11.3.1: PA9-physical
    shorted to PA8 (CC1), PA10-physical shorted to PB15 (CC2). These
    physical pins also carry USB DM/DP after firmware sets the
    `SYSCFG_CFGR1.PA11_RMP / PA12_RMP` bits, so PA9/PA10 cannot be
    used as general GPIOs.
  - The chosen USART must not collide with USB FS, DBCC, or UCPD pins.
    USART2 (PA2/PA3) or LPUART1 (PA2/PA3 alt) are the leading
    candidates; final pick by EE.
  - SWD on PA13/PA14, SWO on PA15.
  - I/O-pin demand against the 32-pin G0B1's user pins
    (after VDD/VSS/SWD/NRST/BOOT0/UCPD1 dead-battery overlap):
    - **Alt-func, mandatory**: UCPD1 (CC1, CC2, DBCC1, DBCC2),
      USB FS (DM, DP via PA11/PA12 remap), USART (TX, RX),
      SPI MOSI → WS2812.
    - **Analog**: 1× ADC for ADC_V.
    - **Timer alt-func**: 1× SCOPE_MARKER output.
    - **GPIO inputs (6)**: 4× DIP switch, TCPP01 FLG/, eFuse FLT/.
    - **GPIO outputs (6)**: TCPP01 VCC, TCPP01 DB, eFuse EN,
      PD_CONTRACT LED, USB_ENUM LED, HEARTBEAT LED.
  - EE confirms during pin assignment that the multiplexing fits,
    using the dead-battery release strobe (SYSCFG) per AN5225
    so DBCC pins free up post-startup. If the budget is tight,
    the candidate cuts in priority order are: SCOPE_MARKER,
    HEARTBEAT LED, TCPP01 DB pin (use FLG/ alone).
  - FT_c 5-V-tolerant pins (PA8, PB15, PD0, PD2) are not strictly
    required for any Renfield net (no externally-powered targets), but
    EE may still prefer FT_c for fault-flag inputs as cheap insurance.
- **DIP-switch semantics** are firmware concerns (which bit means what)
  and are not specified in the hardware spec.
- **PD PDO advertisement** content is firmware concern.
- **Default LED brightness / animation** is firmware concern.
- **TPS25948x exact ILIM resistor value** — EE selects per datasheet
  using a target of ~6 A current limit.
- **TCPP01 OVP divider exact resistor values** — EE selects per
  datasheet for 22 V threshold.
- **VBUS_OUT_DIV resistor values** — EE selects, target ÷10
  attenuation for safe analog probing of full-scale VBUS at TP4.
- **USB DFU VID:PID** — firmware concern; default to a development
  VID:PID (e.g. pid.codes) consistent with Feign.

---

## 11. Design notes

1. **The STM32 runs the PD stack itself.** Using the G0B1's UCPD
   peripheral instead of a dedicated PD controller IC (TPS25750,
   TPS26750, etc.) is the explicit point of the board. The
   TCPP01-M12 is a *protection* chip, not a PD controller.

2. **No on-board current measurement.** Current/power telemetry is
   delegated to an external smart load (DC e-load, USB-PD analyzer
   like the Total Phase Power Delivery Analyzer, etc.) at the WAGO
   terminals — those instruments do higher-quality measurement than
   anything reasonable on a 4-layer bench board, and Renfield's job
   is PD-stack firmware development, not power instrumentation.
   **VBUS voltage** is still measured on-board via the VBUS_OUT ÷10
   divider → ADC_V (one resistor pair, useful for AN4879 attach
   detection). The eFuse's IMON pin is exposed at TP11 as a free
   analog scope point for users wanting a quick current visual.

3. **The 5 V rail exists for the WS2812 LEDs.** It is not strictly
   needed by anything else. We don't power the MCU from 5 V because
   the LDO needs at least 1 V of headroom — TPS74x01P from 5 V to
   3.3 V is well within spec at all loads.

4. **The V bargraph is addressable, the status LEDs are not.**
   The two technologies are intentionally split. The bargraph is an
   *information* surface where richness matters (color zones,
   brightness, animation). The status LEDs are *diagnostic* — they
   must work when firmware is broken. A stuck DMA buffer or a blown
   WS2812 silently breaks a unified RGB ladder; a GPIO and a 0603
   LED won't. The **rail LEDs are wired straight to the rails** and
   the **fault LEDs are wired straight off the open-drain fault
   pins** — no MCU, no PG pin, no firmware involvement. The MCU
   reads the fault edges in parallel for software handling.
   Latching faults that survive an MCU crash are the most useful
   kind to be visually obvious.

5. **Latch-off by default, auto-retry by re-stuff.** TPS25948x latches
   off on fault by default, requiring firmware to re-enable. This is
   the correct dev-board behavior: a fault during firmware development
   should stop the world, not silently retry. We provide a **DNP cap
   footprint** silk-labeled `POPULATE FOR AUTO-RETRY` so the auto-retry
   variant is one rework away when needed.

6. **Hardware OVP at 22 V is the absolute backstop.** TCPP01-M12's OVP
   threshold and the eFuse OVLO are fixed by external resistor dividers
   to 22 V. This allows full-spec 20 V negotiation with 2 V margin and
   protects against a defective source delivering more than 20 V on
   VBUS without firmware involvement. A buggy firmware request for >22 V
   would simply be clamped by the hardware.

7. **eFuse defaults to off.** EN is pulled to GND so the load output
   is dead until firmware explicitly turns it on. This avoids the
   "stick a load in, plug in USB, and immediately get 5 V at the load"
   behavior — which sounds friendly but is wrong for a board that will
   be intentionally crashed. The board should not deliver power to its
   output without the firmware affirmatively asking it to.

8. **VBUS not on probe headers, divided-down sense IS.** The high-side
   VBUS rails (RAW/PROT/OUT) stay off the probe headers — they go to
   silk-warned test pads only. Header B carries the safe ÷10 ADC_V
   analog signal and both open-drain fault flags. Keeping the
   high-current noisy nets off the probe-header GND grommet preserves
   CC1/CC2 BMC capture quality on Header A.

9. **CC1_RAW / CC2_RAW are exposed as test loops, deliberately.** A
   PD developer occasionally wants to see what the source is putting
   on the CC line *before* TCPP01-M12 clamps it — e.g. to diagnose
   a defective source. The raw CC test loops are silk-labeled
   `HV — UP TO 22 V` so probe-tip choice is informed.

10. **UCPD2 is unused** but its pins (PD0/PD1/PD2/PD3) may carry a
    dead-battery Rd at boot per Feign §10 design note 14. Firmware
    must release UCPD2 dead-battery early in startup
    (`SYSCFG->CFGR1 |= UCPD2_STROBE`) before those pins can be used
    for anything else.

11. **Crystalless USB.** STM32G0B1 integrates HSI48 + CRS, trimming
    HSI48 against USB SOF. No HSE crystal on the BOM. Same as Feign.

12. **D+ pull-up is on-die.** STM32G0B1 has the 1.5 kΩ D+ pull-up
    integrated. No external pull-up.

13. **Tag-Connect over a populated SWD header** because the board is
    deliberately small (A7 = 74×105 mm) and SWD is a one-off
    programming step. Anyone doing serious SWD work plugs in the
    Tag-Connect cable; nobody needs a 10-pin Cortex header sticking
    up off the board permanently.

14. **No external flash.** Sink-only PD firmware fits comfortably in
    G0B1's 512 KB internal flash with room for trace buffers and
    DFU dual-bank.

15. **No active cooling.** Steady-state dissipation at 5 A worst
    case (TPSM33606S5 ~150 mW + CSD17318Q2 ~500 mW + TPS25948x
    ~830 mW + TPS74x01P ~140 mW + LEDs + MCU ≈ 1.7 W) is fine in a
    4-layer A7 board with reasonable copper pours. A typical session
    at 9 V × 1 A is well under 0.5 W board-wide.
