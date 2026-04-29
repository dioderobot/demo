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
3. Watch live VBUS voltage and current on two 8-LED RGB bargraphs without
   external instruments.
4. Reconfigure firmware behavior at runtime via 8 DIP switches without
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
| R2  | STM32G0B1KxUxN (UFQFPN-32 N-pinout, `memory_size="512KB"` → STM32G0B1KEU6N) runs the entire PD sink stack on UCPD1 | P0 |
| R3  | TCPP01-M12 + external NexFET on VBUS provide hardware OVP/CC OVP/IEC ESD on CC + VBUS | P0 |
| R4  | TPD4E05U06QDQARQ1 provides IEC 61000-4-2 L4 ESD on D+/D- (TCPP01-M12 covers CC only) | P0 |
| R5  | Hardware VBUS OVP threshold = 22 V; CC OVP = 6 V (TCPP01-M12 internal) | P0 |
| R6  | Load-side TPS25948x eFuse with EN default-low, OVLO = 22 V, latch-on-fault | P0 |
| R7  | Auto-retry mode for eFuse selectable via DNP cap footprint (silk-labeled) | P1 |
| R8  | INA236A + 5 mΩ shunt for I²C bus voltage and current measurement | P0 |
| R9  | Two 8-segment RGB bargraphs (V: 0–20 V, I: 0–5 A) using IN-PI15 LEDs | P0 |
| R10 | 6 discrete 0603 status LEDs: 2 rail-direct (5V, 3V3), 4 MCU GPIO-driven (PD_CONTRACT, USB_ENUM, HEARTBEAT, FAULT) | P0 |
| R11 | 8-position SMD DIP switch read via PCA9554A I²C expander (no MCU GPIOs) | P0 |
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
 ┌──────────────────────────────────────────────────────────────────────┐
 │                                                                      │
 │   USB-C 16P receptacle (sink, 5–20 V / 0–5 A)                        │
 │      │                                                               │
 │      │  VBUS  D+  D-  CC1  CC2  GND                                  │
 │      │                                                               │
 │      ▼                                                               │
 │   ┌───────────────┐         CC1 ──┐                                  │
 │   │  TCPP01-M12   │         CC2 ──┤  to STM32 UCPD1 (PA8/PB15)       │
 │   │ CC OVP 6 V    │  gate         │                                  │
 │   │ VBUS OVP 22 V │  driver       │  D+/D-                           │
 │   │ ESD L4 (CC)   │  ┌────────────▼────────────┐                     │
 │   │ Dead-batt Rd  ├─►│  CSD17578Q3A (3×3 SON)  │  VBUS_PROT          │
 │   │ FLG/          │  │  N-FET, 4 mΩ @ 10 V     ├────┬────────────►   │
 │   └───┬───────────┘  └─────────────────────────┘    │                │
 │       │                                             │                │
 │       │           D+/D- ──► TPD4E05U06 ──► STM32 USB FS              │
 │       │                     (IEC ESD L4)                             │
 │       │                                             │                │
 │       │ FLG/ → MCU                       ┌──────────▼─────────┐      │
 │       │                                  │  TPSM33606S5       │      │
 │       │                                  │  buck (VBUS→5 V)   │      │
 │       │                                  │  HotRod QFN, 0.6 A │      │
 │       │                                  │  integrated L      │      │
 │       │                                  └──────────┬─────────┘      │
 │       │                                             │ 5 V rail       │
 │       │                                             ├─► WS2812 chain │
 │       │                                             ▼                │
 │       │                                  ┌────────────────────┐      │
 │       │                                  │  TPS74x01P LDO     │      │
 │       │                                  │  (5 V → 3.3 V)     │      │
 │       │                                  └──────────┬─────────┘      │
 │       │                                             │ 3V3 rail       │
 │       │                                             ▼                │
 │   ┌───┴─────────────────────────────────────────────────────────┐    │
 │   │                                                             │    │
 │   │   STM32G0B1KxUxN (UFQFPN-32 N-pinout, 512 KB via config)    │    │
 │   │   Peripherals (pin assignment deferred to capture):         │    │
 │   │     - UCPD1: CC1 (PA8), CC2 (PB15), DBCC1, DBCC2            │    │
 │   │     - USB FS: DM (PA11→PA9 remap), DP (PA12→PA10 remap)     │    │
 │   │     - 1× USART (UART debug header, 3.3 V CMOS)              │    │
 │   │     - 1× I²C (INA236, PCA9554A, exposed on probe header)    │    │
 │   │     - 1× SPI MOSI w/ DMA → WS2812 chain (1 GPIO out)        │    │
 │   │     - 1× ADC (eFuse IMON only; INA236 covers VBUS digitally)│    │
 │   │     - 2× timer outputs → scope_marker_0/_1 (probe pins)     │    │
 │   │     - GPIOs: 4 status LEDs, TCPP01 EN+VCC+FLG/, eFuse EN+FLT│    │
 │   │     - BOOT0 (PA14), NRST                                    │    │
 │   │     - SWD: PA13 (SWDIO) / PA14 (SWCLK) / PA15 (SWO)         │    │
 │   │     17 GPIOs used out of ~20 available; 3 spare             │    │
 │   └───────────────────────────────────────────────────────┬─────┘    │
 │                                                            │ I²C     │
 │                  ┌─────────────────────────────────────────┤         │
 │                  │                                         │         │
 │                  ▼                                         ▼         │
 │         ┌───────────────────┐                   ┌─────────────────┐  │
 │         │  PCA9554A         │                   │  INA236A        │  │
 │         │  8-bit I²C GPIO   │                   │  V/I monitor    │  │
 │         │  reads 8× DIP sw  │                   │  + 5 mΩ shunt   │  │
 │         └───────────────────┘                   └────────┬────────┘  │
 │                                                          │           │
 │   VBUS_PROT ───► [shunt 5 mΩ] ───► VBUS_OUT ─┬───────────┘           │
 │                                              │                       │
 │                                              ▼                       │
 │                                  ┌────────────────────────┐          │
 │                                  │  TPS259482 eFuse       │          │
 │                                  │  3.5–23 V, 8 A         │          │
 │                                  │  EN default low        │          │
 │                                  │  OVLO 22 V             │          │
 │                                  │  ILIM ~6 A             │          │
 │                                  │  IMON → MCU ADC        │          │
 │                                  │  FLT/ → MCU GPIO       │          │
 │                                  └───────────┬────────────┘          │
 │                                              │                       │
 │                                              ▼                       │
 │                              WAGO 2060-452 (load + / load −)         │
 │                                                                      │
 │   User I/O on the top face:                                          │
 │     - 8 V-bargraph + 8 I-bargraph WS2812 RGB LEDs                    │
 │     - 6 discrete 0603 status LEDs (2 rail-direct, 4 GPIO-driven)     │
 │     - 8-position SMD DIP switch                                      │
 │     - BOOT0 button, RESET button (Omron B3U-1000P)                   │
 │                                                                      │
 │   Probe / debug headers:                                             │
 │     - 2× side-by-side 2×4 probe headers (8 signal lines, Saleae)     │
 │     - 1×4 UART debug header                                          │
 │     - Tag-Connect TC2030-IDC-NL SWD pads (incl. SWO)                 │
 │     - ~14 SMD test pads (Pad_1.5x1.5mm) for non-header nets          │
 │                                                                      │
 └──────────────────────────────────────────────────────────────────────┘
```

---

## 4. Power

### Input

| Parameter | Value |
|-----------|-------|
| Source | USB VBUS via USB-C receptacle (sink role) |
| Voltage | 5 V default; up to 20 V after PD negotiation |
| Maximum draw | 5 A continuous (limited by FET / shunt / eFuse rating, not USB-PD) |
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
- 16× WS2812 LEDs (V + I bargraphs): peak ~240 mA at full white, but
  firmware caps brightness ≤ 30 % → ~75 mA typical.
- LDO input: ~80 mA (×3.3/5 ÷ η ≈ 60 mA).
- **Total**: ~135 mA typical, ~320 mA peak. TPSM33606S5 0.6 A part has
  ~2× margin.

**3V3 rail**:
- STM32G0B1KEU6N @ 64 MHz with USB peripheral: ~25 mA peak.
- 6× discrete status LEDs @ 2 mA (4 GPIO-driven, 2 rail-direct): 12 mA.
- INA236, PCA9554A, TCPP01 VCC, pull-ups, button pull-ups: ~10 mA.
- **Total**: ~55 mA peak. TPS74x01P (500 mA capable) has ~9× margin.

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
7. Firmware reads DIP switches via PCA9554A, decides PDO request strategy.
8. Firmware enables TPS25948x eFuse (default-disabled at boot).
9. PD contract → VBUS goes to negotiated voltage → eFuse passes it to
   the WAGO output → bargraphs light up.

---

## 5. Interfaces

### USB-C (input side)

GCT USB4105-GF-A 16-pin USB-2.0 USB-C (registry `connectors/UsbC16P`).
Data role is **DFP-data-capable** so we can do USB DFU / CDC over the
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
| 5 | SCOPE_MARKER_0 | Firmware-toggled trigger A |
| 6 | GND | |
| 7 | SCOPE_MARKER_1 | Firmware-toggled trigger B |
| 8 | GND | |

**Header B — "BUS" (digital-friendly)**

| Pin | Signal | Notes |
|---|---|---|
| 1 | I²C SDA | Snoop INA236 / PCA9554A traffic |
| 2 | GND | |
| 3 | I²C SCL | Snoop INA236 / PCA9554A traffic |
| 4 | GND | |
| 5 | eFuse FLT/ | TPS25948x fault edges |
| 6 | GND | |
| 7 | TCPP01 FLG/ | TCPP01-M12 fault edges |
| 8 | GND | |

Header A signals belong on Saleae **analog** inputs (CC swing is ~1.2 V,
below standard digital threshold). Header B signals are 3.3 V CMOS,
fine on Saleae digital channels.

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

**Flat SMD test pads** (stdlib `TestPoint` generic, `Pad_1.5x1.5mm`
variant) for nets that aren't on a probe header but still want a clean
probe-tip touchpoint. Silk labels nearby; **no colored solder loops**
(no Keystone parts — too much BOM pain for the value).

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
| TP9 | TCPP01_EN | none |
| TP10 | eFuse_EN | none |
| TP11 | IMON | analog out |
| TP12–15 | GND | none (distributed for probe clips) |

HV pads should be physically separated from low-voltage pads so a probe
clip can't accidentally bridge VBUS to GND or to a 3V3 net.

### User I/O

| Element | Function |
|---|---|
| BOOT0 button | Tactile, pulls BOOT0 high for DFU bootloader entry |
| RESET button | Tactile, pulls NRST low |
| 8-pos DIP switch | Read via PCA9554A I²C expander; semantics firmware-defined |
| V bargraph | 8 IN-PI15 LEDs, addressable, 0–20 V at 2.5 V/LED |
| I bargraph | 8 IN-PI15 LEDs, addressable, 0–5 A at 0.625 A/LED |
| Status LEDs | 6 discrete 0603 (see §6) — 2 rail-direct (5V, 3V3), 4 MCU GPIO-driven |

### Status LEDs

Never on the WS2812 chain so they remain useful when firmware is broken.

| LED | Color | Driver | Behavior |
|-----|-------|--------|----------|
| 5V_RAIL | Green | **Rail-direct** (5 V → R → LED → GND) | Steady on whenever 5 V is present |
| 3V3_RAIL | Green | **Rail-direct** (3V3 → R → LED → GND) | Steady on whenever 3V3 is present |
| PD_CONTRACT | Blue | MCU GPIO | Contract negotiated and held |
| USB_ENUM | Blue | MCU GPIO | USB host has enumerated us |
| HEARTBEAT | White | MCU GPIO | Slow blink = MCU alive |
| FAULT | Red | MCU GPIO | Aggregated fault (FLT/ or FLG/ or firmware) |

**Why rail-direct, not PG-direct.** The TPSM33606S5 PGOOD pin is
open-drain — it would need an external pull-up and would drive an LED
on *fault*, not on *good*. The TPS74x01P LDO has push-pull PG, but
mixing the two patterns is confusing. Driving both LEDs from the rail
through a current-limit resistor gives consistent "rail is up"
telemetry with no PG-pin sourcing concerns. If the rail comes up out
of regulation, INA236 catches it and firmware drives FAULT.

The four GPIO-driven status LEDs are 0603 with discrete current-limit
resistors (~330 Ω for ~2 mA at VOL ≈ 0).

---

## 6. Key components

| Function | Part | Package | Source |
|----------|------|---------|--------|
| MCU | STM32G0B1KxUxN (`memory_size="512KB"` → STM32G0B1KEU6N) | UFQFPN-32 N-pinout | `components/STMicroelectronics/STM32G0B1KxUxN` (registry) |
| USB-C connector | GCT USB4105-GF-A | 16-pin USB-2.0 receptacle | `connectors/UsbC16P` |
| USB-C port protection (CC + VBUS) | TCPP01-M12 | QFN-12 (3×3) | Librarian (new) |
| D+/D- ESD | TPD4E05U06QDQARQ1 | DQA USON-10 | `components/TPD4E05U06QDQARQ1` (registry, vendored) |
| VBUS gating FET | CSD17578Q3A | SON-8 (3×3) | Librarian (new) |
| Buck (VBUS→5 V) | TPSM33606S5QRDNRQ1 | HotRod QFN module | Librarian (new) |
| LDO (5 V→3.3 V) | TPS74x01P | DRV (SON-6) | `components/Texas_Instruments/TPS74x01P` |
| V/I monitor | INA236A | WSON-10 | Librarian (new) |
| Shunt | 5 mΩ ±1 % 1 W | 2512 | Generic |
| Load eFuse | TPS259482AYWPR | LQFN-23 | `reference/TPS25948x` |
| I²C GPIO expander (DIP reader) | PCA9554A | HVQFN-16 (4×4) preferred, TSSOP-16 fallback | Librarian (new) |
| DIP switch | 8-pos SMD DIP | SMT 2.54 / 1.27 mm | Librarian (new) |
| Bargraph LEDs | IN-PI15TAT5R5G5B (×16) | 1.5×1.5 mm 4-pad | Librarian (new) |
| Status LEDs | Generic 0603 (6× — green ×2, blue ×2, white ×1, red ×1) | 0603 | Generic |
| Buttons | Omron B3U-1000P | SMT tactile | `components/B3U-1000P` |
| SWD | Tag-Connect TC2030-IDC-NL | Pads only | `connectors/TagConnect` |
| Test pads | stdlib `TestPoint` generic, `Pad_1.5x1.5mm` variant | flat SMD pad | stdlib |
| Load output | WAGO 2060-452/998-404 | SMD push-in 2-pole | Librarian (new) |
| Probe headers | 2× side-by-side 2×4 0.1″ pin headers (stdlib generic) | SMT or THT | stdlib |
| UART header | 1×4 0.1″ pin header (stdlib generic) | SMT or THT | stdlib |

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
- **TCPP01 FLG/, eFuse FLT/ pull-ups**: 100 kΩ to 3V3.
- **WS2812 chain decoupling**: 100 nF per LED (best practice for
  addressable RGB chains).
- **PCA9554A address pins**: tied to suit a non-conflicting I²C address.
- **I²C bus pull-ups**: 4.7 kΩ to 3V3 on SDA and SCL.
- **VBUS_OUT divider for analog observation (TP4)**: 90 kΩ + 10 kΩ from
  VBUS_OUT to GND, providing a ÷10 attenuated copy on the test-point
  loop. Optional: also routed to a spare MCU ADC channel.

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
- Min package: 0402 passives, SOT-23 small actives, UFQFPN-32 MCU, 1.5×1.5
  RGB LEDs, QFN-12 (TCPP01-M12), WSON-10 (INA236A).
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
  schematic capture against the G0B1KxUxN 32-pin N-pinout, with the
  constraints that:
  - **UCPD1 dead-battery wiring** per AN5225 §11.3.1: PA9-physical
    shorted to PA8 (CC1), PA10-physical shorted to PB15 (CC2). These
    physical pins also carry USB DM/DP after firmware sets the
    `SYSCFG_CFGR1.PA11_RMP / PA12_RMP` bits, so PA9/PA10 cannot be
    used as general GPIOs.
  - The chosen USART must not collide with USB FS, DBCC, or UCPD pins.
    USART2 (PA2/PA3) or LPUART1 (PA2/PA3 alt) are the leading
    candidates; final pick by EE.
  - SWD on PA13/PA14, SWO on PA15.
  - GPIO budget: ~17 of ~20 free GPIOs used. 3 spare. See architecture
    diagram §3 for the demand list.
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

1. **Why the STM32 runs the PD stack itself.** Using the G0B1's UCPD
   peripheral instead of a dedicated PD controller IC (TPS25750,
   TPS26750, etc.) is the explicit point of the board. Renfield is
   the platform on which we develop UCPD-based firmware, so any
   integrated controller would defeat the purpose. The TCPP01-M12 is
   a *protection* chip, not a PD controller.

1a. **Why the 32-pin part.** The 32-pin UFQFPN-32 N-pinout
    (`STM32G0B1KxUxN` already in registry) supports everything
    Renfield needs with 3 GPIOs of headroom. The 48-pin part offered
    no functional advantage for this board — only more spares — and
    keeping the 32-pin part means we reuse a vendored package that
    Feign already validates. Cuts vs an unconstrained design: status
    LED count (10 → 6), MCU-side ADC channels (3 → 1), and dropping
    the dedicated scope-marker LEDs (markers stay as probe-header
    pins). All cuts are diagnostic-cosmetic, not functional.

2. **The 5 V rail exists for the WS2812 LEDs.** It is not strictly
   needed by anything else. We don't power the MCU from 5 V because
   the LDO needs at least 1 V of headroom — the TPS74x01P from 5 V
   to 3.3 V is well within spec at all loads.

3. **Why the bargraphs are addressable but the status LEDs are not.**
   The two technologies are intentionally split: the bargraph display
   is an *information* surface where we want richness (color zones,
   brightness, animation), while the status LEDs are *diagnostic* —
   they must continue to work when firmware is broken. A blown WS2812
   driver, a corrupted DMA buffer, or a stuck SPI clock would silently
   break a unified RGB ladder. A GPIO and a 0603 LED won't. Going
   further: the **two power-rail LEDs are wired straight to the rails**
   through current-limit resistors, no MCU and no PG pin involvement —
   so they light correctly even when the MCU is dead and even when the
   regulator's PG pin is open-drain (TPSM33606S5) or push-pull
   (TPS74x01P). Tradeoff: we lose the "rail is in regulation"
   distinction (vs "rail is merely present"); INA236 catches that case
   and firmware drives FAULT.

4. **Why an I²C GPIO expander reads the DIP switch.** Eight SMT DIP
   switches consume eight MCU GPIOs. Eight LEDs would consume eight
   more. The PCA9554A is a $0.50 single chip on the I²C bus we
   already have for INA236, costs zero extra GPIOs, and (bonus) the
   DIP-switch states are observable on the probe-header SDA/SCL pins
   for free.

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

8. **Probe header carries I²C, not VBUS sense.** The 2×9 main probe
   header is for low-voltage signals only — CC, I²C, fault flags,
   firmware markers. VBUS observation goes through the **separate
   VBUS_OUT_DIV (÷10) test loop** at TP4. Keeping the high-current
   noisy nets off the probe-header GND grommet preserves CC1/CC2 BMC
   capture quality.

9. **CC1_RAW / CC2_RAW are exposed as test loops, deliberately.** A
   PD developer occasionally wants to see what the source is putting
   on the CC line *before* TCPP01-M12 clamps it — for example to
   diagnose a defective source. The raw CC test loops are silk-labeled
   `HV — UP TO 22 V` so probe-tip choice is informed.

10. **UCPD2 is unused** but its pins (PD0/PD1/PD2/PD3) may carry a
    dead-battery Rd at boot per Feign §10 design note 14. Firmware
    must release UCPD2 dead-battery early in startup
    (`SYSCFG->CFGR1 |= UCPD2_STROBE`) before those pins can be used
    for anything else, regardless of whether we end up using them.

11. **Crystalless USB.** STM32G0B1 integrates HSI48 + CRS, trimming
    HSI48 against USB SOF. No HSE crystal on the BOM. Same as Feign.

12. **D+ pull-up is on-die.** STM32G0B1 has the 1.5 kΩ D+ pull-up
    integrated. No external pull-up.

13. **VBUS sensing via divider.** Per AN4879 §3.2, a VBUS divider into
    an MCU ADC is recommended for proper attach detection. Renfield's
    `VBUS_OUT_DIV` net (÷10) serves this role and is also exposed at
    TP4 for analog scope observation.

14. **Tag-Connect over a populated SWD header** because the board is
    deliberately small (A7 = 74×105 mm) and SWD is a one-off
    programming step. Anyone doing serious SWD work plugs in the
    Tag-Connect cable; nobody needs a 10-pin Cortex header sticking
    up off the board permanently.

15. **No external flash.** Sink-only PD firmware fits comfortably in
    G0B1's 512 KB internal flash with room for trace buffers and
    DFU dual-bank.

16. **The board has no fan, no heatsink, no thermal mat.** Steady-state
    dissipation (TPSM33606S5 ~150 mW + CSD17578Q3A ~100 mW + TPS25948x
    ~830 mW + TPS74x01P ~140 mW + LEDs + MCU = ~1.5 W absolute worst
    case) is fine in a 4-layer A7 board with reasonable copper
    pours. This is the bound; a typical session at 9 V × 1 A is well
    under 0.5 W board-wide.
