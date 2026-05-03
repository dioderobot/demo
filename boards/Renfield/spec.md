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
| R6  | Load-side TPS259482L (LatchOff) eFuse, self-enabling via UVLO/OVLO divider (UVLO = 4.5 V, OVLO = 22 V, ILIM = 6 A), with firmware kill switch via active-high DISABLE input | P0 |
| R7  | Auto-retry behavior available by swapping to TPS259482A (drop-in same package) — silicon SKU choice | P1 |
| R8  | VBUS÷10 divider → MCU ADC for voltage. **No on-board current measurement** — user supplies external smart load if current/power telemetry is needed. | P0 |
| R9  | One 8-segment RGB bargraph (V: 0–20 V) using IN-PI15 LEDs | P0 |
| R10 | 7 discrete 0402 status LEDs: 2 rail-direct (5V, 3V3), 2 hardware-driven from open-drain pins (TCPP01 FLT, TPS259482 SPLYGD), 3 MCU GPIO-driven (PD_CONTRACT, USB_ENUM, HEARTBEAT) | P0 |
| R11 | 4-position SMD DIP switch direct-driven on MCU GPIOs (no I²C expander) | P0 |
| R12 | One 2×4 2.54 mm SMT header for PD-dev signals (CC1/CC2/ADC_V/SCOPE_MARKER + 2 fault flags + 2 GND) — hookup-wire breakout to Saleae Logic Pro 8 | P0 |
| R13 | Dedicated 1×4 0.1″ UART debug header (GND/TX/RX/3V3) | P0 |
| R14 | Tag-Connect TC2030-IDC-NL SWD pads (no connector populated; Cortex-M0+ has no SWO — pin 6 is NC) | P0 |
| R15 | SMD test pads for nets not on a probe/UART header (no Keystone loops) | P0 |
| R16 | USB DFU bootloader entry via BOOT0 + RESET tactile buttons | P0 |
| R17 | Crystal-less USB via HSI48 + CRS (no HSE) | P0 |
| R18 | TPSM33606S5 buck (VBUS→5 V, integrated inductor) feeding TPS74x01P LDO (5 V→3V3) | P0 |
| R19 | WAGO 2060-452 SMD push-in 2-pole terminal block as test-load output | P0 |
| R20 | A8 form factor (52 × 74 mm), 4-layer board, 4× M3 corner mounting holes | P0 |
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
│   TCPP01-M12     │  CC1, CC2 ──────────► │ STM32G0B1KEU6N  │
│   - CC OVP 6 V   │                       │ UFQFPN-32, 512K │
│   - VBUS OVP 22V │  GATE→ ┌──────────┐   │ N-pinout        │
│   - ESD L4 (CC)  │ ──────►│CSD17318Q2│   │                 │
│   - Dead-batt Rd │        │ N-FET 2x2│   │ Pinout: see §7  │
│   - FLT  OD      │ ──────►│ ~20 mΩ   │   │                 │
└──────────────────┘        └────┬─────┘   │                 │
        │                        │ VBUS_PROT│                │
        │ FLT → LED + MCU        │          │                 │
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
  │                  │ SPLYGD OD→LED+MCU│                    │
  │                  └────────┬─────────┘                    │
  │                           │ VBUS_OUT                     │
  │                           ▼                              │
  │                       load+ ──────► WAGO 2060-452 ──► load
  │                       load− ──────► WAGO 2060-452 ──► load
  │                       (current/power telemetry: external smart load)
  │                                                          │
  └─── BOOT0 button ──► PA14 (BOOT0)   SWD: PA13/PA14         │
       RESET button ──► NRST           UART: TX, RX (header)│
                                                            │
       4-pos DIP ────────────────► 4× GPIO inputs           │
       7 status LEDs:                                       │
         - 5V_RAIL, 3V3_RAIL: rail-direct                   │
         - TCPP01_FLT_LED, EFUSE_NOT_GOOD_LED: hardware-driven   │
           from TCPP01 FLT and eFuse SPLYGD open-drain pins │
         - PD_CONTRACT, USB_ENUM, HEARTBEAT: MCU GPIO       │
       1 V-bargraph (8× IN-PI15 WS2812 RGB) → SPI MOSI ─────┘
       Probe header (2×4) — see §5
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
| Worst-case OVP backstop | 22 V at TCPP01-M12 (hardware), 22 V at TPS259482 OVLO |

### Rails

| Rail | Voltage | Source | Budget |
|------|---------|--------|--------|
| VBUS_RAW | 5–20 V | USB-C, before TCPP01 | up to 5 A |
| VBUS_PROT | 5–20 V | After TCPP01-M12 + NexFET, OVP-clamped at 22 V | up to 5 A |
| VBUS_OUT | 5–20 V | After TPS259482 eFuse (current-limited, OVLO-protected) | up to 5 A |
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
5. TPS74x01P starts → 3V3 rail up → MCU comes out of reset and TCPP01
   VCC powers up at the same time (VCC tied directly to V3V3).
6. MCU takes UCPD1 control, releases dead-battery Rd per AN5225 strobe
   sequence, drives TCPP01 DB/ high, starts PD stack.
7. Firmware reads DIP switches directly via 4 GPIO inputs, decides PDO
   request strategy.
8. VBUS_PROT → TPS259482 UVLO (4.5 V) crosses → eFuse self-enables
   autonomously, passes VBUS to the WAGO output.
9. PD contract → VBUS steps to negotiated voltage → eFuse continues
   passing it (still within OVLO = 22 V) → load is live.
10. Firmware may at any time drive PB8 (EFUSE_DISABLE) high to force
    the eFuse off — useful for clearing a LatchOff trip (toggle high
    then low) or for test automation.

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

### PD-dev probe header

**One 2×4 2.54 mm SMT vertical male pin header.** Designed for
hookup-wire breakout to Saleae Logic Pro 8 (not the 2-row grommet —
we trade strict signal/GND alternation for an extra signal lane).

| Pin | Signal | Notes |
|---|---|---|
| 1 | CC1_MCU (post-TCPP01) | Primary BMC capture, ~1.2 V analog |
| 2 | CC2_MCU (post-TCPP01) | Orientation-dependent CC line |
| 3 | ADC_V | VBUS_OUT ÷10 divider, 0–2.2 V analog |
| 4 | GND | |
| 5 | GND | |
| 6 | SCOPE_MARKER | Firmware-toggled trigger (PA6 / TIM3_CH1) |
| 7 | TCPP01 FLT | TCPP01-M12 fault edges (open-drain, active-low) |
| 8 | eFuse SPLYGD | TPS259482 supply-good (active-high; LOW on UVLO/OVLO/inrush only — NOT a load-fault flag) |

Analog-class signals (CC1, CC2, ADC_V) grouped on pins 1–3 next to a
GND on pin 4; digital signals (SCOPE_MARKER, fault flags) grouped on
pins 6–8 next to a GND on pin 5. Layout should keep this
analog/digital separation across the trace runs.

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

### SWD

Tag-Connect TC2030-IDC-NL footprint, **pads only**, no connector
populated. Six-pin Tag-Connect signal mapping:

| Tag-Connect pin | Signal |
|---|---|
| 1 (Vcc ref) | 3V3 |
| 2 (TMS / SWDIO) | SWDIO (PA13) |
| 3 (NRST) | NRST |
| 4 (TCK / SWCLK) | SWCLK (PA14) |
| 5 (GND) | GND |
| 6 (TDO / SWO) | NC — Cortex-M0+ has no SWO/ITM trace |

Debug is single-wire (SWD) only; printf-trace is over the UART header.

### Test pads

**Flat SMD test pads** (~1.5 × 1.5 mm) for nets that aren't on a probe
header but still want a clean probe-tip touchpoint. Silk labels nearby;
**no colored solder loops** (no Keystone parts — too much BOM pain for
the value).

Minimum set:

| Pad | Net | Silk |
|---|---|---|
| TP_VBUS_RAW | VBUS_RAW | `HV ≤ 22V` |
| TP_VBUS_PROT | VBUS_PROT | `HV ≤ 22V` |
| TP_FET_GATE | FET_GATE (TCPP01 GATE pin) | `≤ 28V` — rides to VBUS_PROT + VGS |
| TP_EFUSE_IMON | TPS259482 ILM pin | analog current monitor, ~3 V/A |
| TP_GND_HV | GND | near the HV cluster for scope return |
| TP_GND_LV | GND | near the LV area for scope return |

Other candidate signals (VBUS_OUT, V5V, V3V3, NRST, EFUSE_EN/DISABLE,
TCPP01_DB, CC1_RAW / CC2_RAW) are deliberately **not** pads — each is
already reachable via another header, the WAGO terminal, or the MCU
pin directly. Keep pads sparse; add more during layout if a specific
debug case earns one.

HV pads should be physically separated from low-voltage pads so a
scope tip or flying-wire clip can't accidentally bridge VBUS to GND
or to a 3V3 net.

### User I/O

| Element | Function |
|---|---|
| BOOT0 button | Tactile, pulls BOOT0 high for DFU bootloader entry |
| RESET button | Tactile, pulls NRST low |
| 4-pos DIP switch | Direct-driven on 4 MCU GPIOs; semantics firmware-defined |
| V bargraph | 8 IN-PI15 LEDs, addressable, 0–20 V at 2.5 V/LED |
| Status LEDs | 7 discrete 0402 (see below) — 2 rail-direct + 2 hardware-driven on TCPP01 FLT / eFuse SPLYGD + 3 MCU GPIO-driven |

### Status LEDs

Never on the WS2812 chain so they remain useful when firmware is broken.
Seven 0402 LEDs, three driver styles. Current-limit resistors chosen
in the `.zen` to land each LED at comparable brightness.

| LED | Color | Driver | Behavior |
|-----|-------|--------|----------|
| 5V_RAIL | Green | Rail-direct (5 V → R → LED → GND) | On whenever 5 V is present |
| 3V3_RAIL | Green | Rail-direct (3V3 → R → LED → GND) | On whenever 3V3 is present |
| TCPP01_FLT | Red | 3V3 → R → LED → FLT (open-drain) | On when TCPP01-M12 latches a fault |
| EFUSE_NOT_GOOD | Red | 3V3 → R → LED → SPLYGD (open-drain) | On when the rail is not good (UVLO / OVLO / inrush). **Not** a load-fault indicator. |
| PD_CONTRACT | Blue | MCU GPIO | Contract negotiated and held |
| USB_ENUM | Blue | MCU GPIO | USB host has enumerated us |
| HEARTBEAT | Yellow | MCU GPIO | Slow blink = MCU alive |

**Why rail-direct, not PG-direct.** The buck's PGOOD is open-drain
(would need an external pull-up and drive on *fault*); the LDO's PG is
push-pull. Mixing conventions is confusing. Rail-direct gives a
consistent "rail is up" LED with no PG-pin quirks.

**Why hardware-driven fault LEDs.** Driving the LED directly from the
open-drain fault pin gives a firmware-independent latched indicator
— the LED lights even if firmware has crashed. The MCU reads the same
pin in parallel for software handling.

**SPLYGD caveat.** TPS25948 SPLYGD reports rail-good, not load-fault.
Load-side events (overcurrent, short-circuit, overtemperature,
reverse current) leave SPLYGD high. Those events show up instead via
LatchOff behavior (VBUS_OUT drops to 0, firmware sees ADC_V = 0).
Named `EFUSE_NOT_GOOD` rather than `EFUSE_FLT` to reflect this.

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
| Load eFuse | TI TPS259482LYWPR (LatchOff primary; TPS259482A drop-in for AutoRetry) | POWERWCSP (YWP), DSBGA-12 |
| eFuse DISABLE FET | TI CSD13380F3 FemtoFET | 0.6×1.0 mm X2SON-3 |
| DIP switch | DS04-254-1-04BK-SMT, 4-pos | SMT, 2.54 mm pitch |
| V-bargraph LEDs | Inolux IN-PI15TAT5R5G5B (×8) | 1.5×1.5 mm, 4-pad |
| Status LEDs | 7× generic 0402 (green ×2, red ×2, blue ×2, yellow ×1) | 0402 |
| Buttons | Omron B3U-1000P (×2) | SMT tactile |
| SWD | Tag-Connect TC2030-IDC-NL | pads only, no connector |
| Test pads | flat SMD pad (~1.5×1.5 mm, stdlib TestPoint) | SMT |
| Load output | WAGO 2060-452/998-404 | SMD push-in, 2-pole, 4 mm pitch |
| Probe header | 1× 2×4 2.54 mm SMT vertical male (stdlib generic) | SMT |
| UART header | 1×4 2.54 mm SMT vertical male (stdlib generic) | SMT |
| Fiducials | 3× front-side standard 1 mm fiducials (stdlib generic) | SMT |

### Support passives

Most reference-design passives (MCU decoupling, TCPP01 VCC bypass +
ESD shunt + OVP divider + FLT pull-up, buck/LDO input/output caps,
eFuse UVLO/OVLO divider + ILIM + dVdt cap) are supplied inside their
respective registry components.

The board-level `.zen` adds only:

- **VBUS_RAW bulk** near the USB-C receptacle (USB-C compliance + plug-
  event / OVP-trip ride-through).
- **VBUS_OUT bulk** close to the eFuse output (TPS259482 DS §8.8.1).
- **CC line caps** on each post-TCPP01 CC net, sized to land in the
  USB-PD §5.8.6 CC receiver capacitance budget (200–600 pF) once
  TCPP01 and MCU pad contributions are added.
- **VBUS_OUT ÷10 divider + RC filter** to the MCU ADC (ADC_V).
- **NRST 100 nF filter**, **BOOT0 10 kΩ pull-down** on PA14/SWCLK.
- **SPLYGD pull-up** to 3V3 (defined state before firmware configures
  the MCU internal pull-up).
- **DIP-switch pull-ups** to 3V3 (explicit external pulls — defined
  state before firmware touches the UCPD2 strobe on PD0 / PD2).
- **WS2812 DIN series R** for edge shaping on the first pixel.
- **eFuse DISABLE NFET shunt geometry** (CSD13380F3 drain on EN/UVLO,
  source to GND, PB8 gate drive, 100 kΩ pull-down). Required because
  a push-pull GPIO directly on EN/UVLO would clamp the analog divider
  and defeat OVLO (TPS259482 DS §7.1, §8.1.2.1). Default is NFET off;
  driving PB8 high disables the eFuse.
- **Latch-off vs auto-retry** is a silicon SKU choice, not a passive:
  TPS259482L is loaded by default (dev board should stop on fault);
  TPS259482A is a drop-in for auto-retry via the registry component's
  `fault_response` config.

Exact values, packages, and voltage ratings are specified in
`Renfield.zen` — that is the authoritative BOM.

---

## 7. Pinout (STM32G0B1KEU6N, UFQFPN-32, N-pinout)

Fixed-function pins (UCPD1, USB FS, SWD, power, reset) follow the
N-pinout silicon and AN5225 §11.3.1 dead-battery wiring. Free pins
are assigned per the constraints in this spec.

| Pin | Net          | Function                              | AF / I/O type  |
|---:|---------------|---------------------------------------|----------------|
|  1 | —             | spare — expose as test pad             | PB9, FT_f      |
|  2 | —             | spare — expose as test pad             | PC14, FT (LSE) |
|  3 | —             | spare — expose as test pad             | PC15, FT (LSE) |
|  4 | 3V3           | VDD / VDDA                            | supply         |
|  5 | GND           | VSS / VSSA                            | supply         |
|  6 | NRST          | reset (button + 100 nF + Tag-Connect) | NRST           |
|  7 | ADC_V         | VBUS_OUT ÷10 sense                    | PA0  / ADC1_IN0 (FT_a)         |
|  8 | HEARTBEAT_LED | MCU "alive" indicator                 | PA1  / GPIO out (FT_ea)        |
|  9 | UART_TX       | debug printf out                      | PA2  / USART2_TX  AF1 (FT_a)   |
| 10 | UART_RX       | debug input                           | PA3  / USART2_RX  AF1 (FT_ea)  |
| 11 | PD_CONTRACT_LED | contract-held indicator             | PA4  / GPIO out (TT_a)         |
| 12 | USB_ENUM_LED  | USB-enumerated indicator              | PA5  / GPIO out (TT_ea)        |
| 13 | SCOPE_MARKER  | hardware scope-trigger output         | PA6  / TIM3_CH1   AF1 (FT_ea)  |
| 14 | WS2812_DATA   | V-bargraph serial out                 | PA7  / SPI1_MOSI  AF0 (FT_a)   |
| 15 | —             | spare — expose as test pad             | PB0  / GPIO (FT_ea)            |
| 16 | TCPP01_DB     | TCPP01 dead-battery release (active-high) | PB1 / GPIO out (FT_ea)     |
| 17 | UCPD1_CC2     | post-TCPP01 CC2                       | PB15 / UCPD1_CC2  (FT_fcs)     |
| 18 | UCPD1_CC1     | post-TCPP01 CC1                       | PA8  / UCPD1_CC1  (FT_fcs)     |
| 19 | UCPD1_DBCC1   | dead-battery sense — short ext. to PA8 (CC1) | PA9 / UCPD1_DBCC1 (FT_fds) |
| 20 | 3V3           | VDDIO2                                | supply         |
| 21 | UCPD1_DBCC2   | dead-battery sense — short ext. to PB15 (CC2) | PA10 / UCPD1_DBCC2 (FT_fds) |
| 22 | USB_DM        | post-TPD4E05 D−                       | PA11 / USB_DM     (FT_fus, no remap) |
| 23 | USB_DP        | post-TPD4E05 D+                       | PA12 / USB_DP     (FT_fus, no remap) |
| 24 | SWDIO         | Tag-Connect pin 2                     | PA13 / SWDIO      (FT_es)      |
| 25 | SWCLK / BOOT0 | Tag-Connect pin 4 + BOOT0 button + 10 kΩ pull-down | PA14 / SWCLK / BOOT0 (FT_s) |
| 26 | DIP_SW1       | DIP pos 1, GND-close, internal pull-up | PD0 / GPIO in    (FT_cs, UCPD2 strobe) |
| 27 | DIP_SW2       | DIP pos 2                             | PD1  / GPIO in    (FT_ds, UCPD2 strobe) |
| 28 | DIP_SW3       | DIP pos 3                             | PD2  / GPIO in    (FT_cs, UCPD2 strobe) |
| 29 | DIP_SW4       | DIP pos 4                             | PD3  / GPIO in    (FT_ds, UCPD2 strobe) |
| 30 | TCPP01_FLT    | TCPP01 fault flag (open-drain, LED-pulled) | PB6 / GPIO in (FT_fa)      |
| 31 | EFUSE_SPLYGD  | TPS259482 supply-good (open-drain, LED-pulled) | PB7 / GPIO in (FT_fa)  |
| 32 | EFUSE_DISABLE | TPS259482 kill switch (active-high, default-low via on-board NFET shunt on EN/UVLO) | PB8  / GPIO out (FT_f)        |

*EP (exposed thermal pad on package underside) ties to GND.*

### Notes

- **UCPD1 dead-battery wiring (AN5225 §11.3.1).** PA9 is shorted to PA8
  on the PCB so the dead-battery sense circuit on PA9 sees the same
  potential as the CC1 line (PA8). PA10 is shorted to PB15 for CC2.
  The UCPD peripheral drives an internal Rd at boot until firmware
  releases dead-battery via `SYSCFG->CFGR1 |= UCPD1_STROBE`.
- **USB FS without remap.** PA11/PA12 carry USB DM/DP at their native
  pin positions (22, 23). The PA11_RMP / PA12_RMP bits are *not*
  needed because PA9 / PA10 are already used as dedicated UCPD1 DBCC
  pins; the 32-pin N-pinout brings both pin pairs out separately.
- **UCPD2 strobe on PD0–PD3.** Per datasheet note 4, PD0 / PD2 carry an
  internal Rd to GND at reset (FT_cs), and PD1 / PD3 are DBCC sense
  pins. Firmware must write `SYSCFG->CFGR1 |= UCPD2_STROBE` early in
  startup before reading the DIP switches; without it, PD0 / PD2 would
  read low even with the DIP open.
- **PB6 / PB7 (FT_fa) are FM+ I²C tolerant** — reserved here as inputs
  for the open-drain fault flags so the inevitable I²C-on-PB6/PB7
  re-spin remains a one-line firmware change rather than a board
  change.
- **PC14 / PC15 (LSE pins)** are usable as GPIO since Renfield has no
  LSE crystal, but per datasheet notes 1–2 they're current-limited
  (sink-only, ≤2 MHz, ≤30 pF). Useful as inputs / spare test pads;
  unsuitable as LED drivers or signal outputs.
- **No SWO.** Cortex-M0+ has no ITM / TPIU — trace must go over UART
  or USB CDC.

### GPIO budget

| Class | Used | Pins |
|---|---|---|
| Fixed peripherals (UCPD1 / USB / SWD) | 8 | PA8, PA9, PA10, PA11, PA12, PA13, PA14, PB15 |
| ADC analog | 1 | PA0 |
| USART | 2 | PA2, PA3 |
| SPI MOSI (WS2812) | 1 | PA7 |
| Timer (SCOPE_MARKER) | 1 | PA6 |
| GPIO outputs | 5 | PA1, PA4, PA5, PB1, PB8 |
| GPIO inputs | 6 | PB6, PB7, PD0, PD1, PD2, PD3 |
| **Total user-pin GPIOs used** | **16 / 20** | |
| Spare | 4 | PB0, PB9, PC14, PC15 |

---

## 8. Mechanical & environmental

- **Form factor**: A8 (52 × 74 mm), single rectangular outline.
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

## 9. Manufacturing & assembly

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

## 10. Regulatory & compliance

- Not a sold product; FCC / CE / UL not targeted.
- USB-IF certification not pursued. Device will not claim USB-IF logo
  compliance.
- The TCPP01-M12 has been USB-IF-certified as a sink via the
  X-NUCLEO-SNK1M1 (TID 5205) — that's the chip's compliance, not ours.

---

## 11. Open items (non-blocking, deferred to capture / firmware)

- **DIP-switch semantics** are firmware concerns (which bit means what)
  and are not specified in the hardware spec.
- **PD PDO advertisement** content is firmware concern.
- **Default LED brightness / animation** is firmware concern.
- **USB DFU VID:PID** — firmware concern; default to a development
  VID:PID (e.g. pid.codes) consistent with Feign.

(TPS259482 ILIM resistor, TCPP01 OVP divider, VBUS_OUT_DIV, UVLO/OVLO
resistors are now resolved: derived inside the registry components from
the high-level thresholds passed at instantiation. See §6 support
passives.)

## 12. Design notes

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
   detection). The eFuse's ILM pin is exposed at TP_EFUSE_IMON as a
   free analog scope point (~3 V/A) for users wanting a quick current
   visual.

3. **The 5 V rail exists for the WS2812 LEDs.** It is not strictly
   needed by anything else. We don't power the MCU from 5 V because
   the LDO needs at least 1 V of headroom — TPS74x01P from 5 V to
   3.3 V is well within spec at all loads.

4. **The V bargraph is addressable, the status LEDs are not.**
   The two technologies are intentionally split. The bargraph is an
   *information* surface where richness matters (color zones,
   brightness, animation). The status LEDs are *diagnostic* — they
   must work when firmware is broken. A stuck DMA buffer or a blown
   WS2812 silently breaks a unified RGB ladder; a GPIO and a 0402
   LED won't. The **rail LEDs are wired straight to the rails** and
   the **fault LEDs are wired straight off the open-drain fault
   pins** — no MCU, no PG pin, no firmware involvement. The MCU
   reads the fault edges in parallel for software handling.
   Latching faults that survive an MCU crash are the most useful
   kind to be visually obvious.

5. **Latch-off by default; auto-retry by silicon swap.** TPS259482L
   latches off on fault and requires firmware (or a power cycle) to
   re-enable. This is the correct dev-board behavior: a fault during
   firmware development should stop the world, not silently retry. The
   TPS259482A (auto-retry) variant is a drop-in same-package SKU swap
   for users who want hands-off recovery; on TPS259482 the retry
   behavior is baked into the silicon, not selectable via a passive.

6. **Hardware OVP at 22 V is the absolute backstop.** TCPP01-M12's OVP
   threshold and the eFuse OVLO are fixed by external resistor dividers
   to 22 V. This allows full-spec 20 V negotiation with 2 V margin and
   protects against a defective source delivering more than 20 V on
   VBUS without firmware involvement. A buggy firmware request for >22 V
   would simply be clamped by the hardware.

7. **eFuse self-enables; firmware holds a kill switch.** The eFuse is
   a protection IC — its UVLO/OVLO divider is the enable logic by
   design. Gating it behind a firmware vote would leave the rail
   unprotected whenever firmware is crashed, stalled, or unflashed.
   Upstream of the eFuse, VBUS_PROT is already OVP-clamped at 22 V by
   TCPP01-M12 + the external FET, so the eFuse is defense-in-depth on
   the load path, not the master on/off switch. Firmware retains an
   active-high DISABLE input (PB8) wired through an on-board NFET
   shunt on the EN/UVLO node to force the output off on demand —
   primarily for clearing a TPS259482L LatchOff trip without
   unplugging the USB cable. Driving the EN/UVLO pin directly from a
   push-pull GPIO would defeat OVLO, hence the NFET-shunt geometry.

8. **VBUS not on probe header, divided-down sense IS.** The high-side
   VBUS rails (RAW/PROT) stay off the probe header — they go to
   silk-warned test pads only. The header carries the safe ÷10 ADC_V
   analog signal, the CC lines, a firmware scope marker, and two
   open-drain fault flags. Keeping high-current noisy nets off the
   probe-header GND pins preserves CC1/CC2 BMC capture quality.

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
    deliberately small (A8 = 52×74 mm) and SWD is a one-off
    programming step. Anyone doing serious SWD work plugs in the
    Tag-Connect cable; nobody needs a 10-pin Cortex header sticking
    up off the board permanently.

14. **No external flash.** Sink-only PD firmware fits comfortably in
    G0B1's 512 KB internal flash with room for trace buffers and
    DFU dual-bank.

15. **No active cooling.** Steady-state dissipation at 5 A worst
    case (TPSM33606S5 ~150 mW + CSD17318Q2 ~500 mW + TPS259482
    ~830 mW + TPS74x01P ~140 mW + LEDs + MCU ≈ 1.7 W) is fine in a
    4-layer A8 board with reasonable copper pours. A typical session
    at 9 V × 1 A is well under 0.5 W board-wide.
