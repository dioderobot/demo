# Renfield (DM0005) — Design Specification

A USB-C Power Delivery **sink** development board built around the
STM32G0B1, for **firmware development on the PD sink stack**. Negotiates
up to 20 V / 5 A from a USB-PD source and passes the negotiated rail
through an OVP-gated FET to a WAGO terminal block. Round SMD test pads
expose the signals a PD-firmware developer actually probes.

The board "consumes" its master's power on demand for whatever load you
attach. Hence Renfield.

---

## 1. Purpose

A standalone bench-top dev/eval board for the STM32G0B1's UCPD-based
USB-PD sink stack. It is **not a product** — it is the platform on
which we develop and debug PD sink firmware before deploying it on a
production board.

Primary use cases:

1. Plug Renfield into any USB-PD source and run firmware that negotiates
   contracts up to 20 V / 5 A (60 W maximum design point).
2. Observe BMC traffic on CC1/CC2 in real time with a Saleae Logic Pro 8
   connected to the CC1 / CC2 test pads.
3. Flash / debug over SWD via Tag-Connect; printf-trace over UART test
   pads.
4. Stress-test firmware by deliberately misbehaving mid-negotiation —
   TCPP01-M12 + gated FET hardware-clamp VBUS at 22 V regardless of
   firmware state.

---

## 2. Requirements summary

| ID  | Requirement | Priority |
|-----|-------------|----------|
| R1  | USB-C 16P receptacle, sink role, supports negotiated 5/9/15/20 V at up to 5 A | P0 |
| R2  | STM32G0B1KEU6N (UFQFPN-32, N-pinout, 512 KB flash) runs the PD sink stack on UCPD1 | P0 |
| R3  | TCPP01-M12 + external NexFET provide hardware OVP on CC + VBUS, IEC 61000-4-2 L4 ESD on CC | P0 |
| R4  | Hardware VBUS OVP = 22 V; CC OVP = 6 V (TCPP01-M12 internal) | P0 |
| R5  | TPSM33606S3 buck (VBUS_RAW → 3V3, integrated inductor) powers the MCU directly | P0 |
| R6  | WAGO 2060-452 SMD push-in 2-pole terminal block as test-load output, fed from the post-FET rail | P0 |
| R7  | Tag-Connect TC2030-IDC-NL SWD pads (no connector populated; Cortex-M0+ has no SWO — pin 6 is NC) | P0 |
| R8  | Round Ø1.0 mm SMD test pads for the signals a PD-firmware developer probes (rails, CC, UART, scope trigger, fault flag) | P0 |
| R9  | Crystal-less USB via HSI48 + CRS (no HSE) | P0 |
| R10 | 4-layer board, all-SMT, single-pass reflow, all parts T&R, US-stocked | P0 |
| R11 | 60 W maximum design point (20 V × 3 A or 12 V × 5 A — never both) | P0 |

P0 = must have.

---

## 3. System architecture

```
USB-C 16P (sink, 5–20 V)
    │
    │ VBUS  D+  D-  CC1  CC2  GND
    ▼
 VBUS_RAW ─┬──► TPSM33606S3 buck ──► V3V3 ──► MCU + TCPP01 VCC
           │                                       │
           │    always hot when cable is           │  (controller
           │    live — MCU boots directly          │   domain)
           │    off the receptacle                 │
           │                                       │
           ├──► TCPP01-M12 IN_GD        CC1,CC2 ◄─ UCPD1
           │     │                                   │
           │     │ GATE ──► CSD17318Q2 ──► VBUS_PROT ├──► WAGO (load)
           │     │          N-FET (gated)            │
           │     │                                   │
           │     └──► FLT/ open-drain ──────────────►│  fault LED + PB6
           │                                         │
           │                                         │  PB1 ── TCPP01 DB/
           │                                         │  PA13/PA14 SWD
           │                                         │  PA2/PA3 UART
           │                                         │  PA6 SCOPE_MARKER
           │                                         │  PA1/PA4/PA5 LEDs
           └──► D+/D- ──────────────────── USB FS ──┘
```

**Split MCU / load domain** (DS12900 §6.1). The MCU sits on VBUS_RAW,
so it boots as soon as the cable is live and stays powered through an
OVP trip for fault reporting. The FET gates the **load rail only**. On
a 22 V OVP event, WAGO goes dead; MCU keeps running.

---

## 4. Power

### Input

| Parameter | Value |
|-----------|-------|
| Source | USB VBUS via USB-C receptacle (sink role) |
| Voltage | 5 V default; up to 20 V after PD negotiation |
| Maximum draw | 5 A continuous (limited by FET / WAGO rating, not USB-PD) |
| Hardware OVP backstop | 22 V at TCPP01-M12 |

### Rails

| Rail | Voltage | Source | Budget |
|------|---------|--------|--------|
| VBUS_RAW | 5–22 V | USB-C receptacle (always hot) | up to 5 A |
| VBUS_PROT | 5–22 V | Post-FET (TCPP01 OVP-gated) → WAGO | up to 5 A |
| 3V3 | 3.3 V ±2 % | TPSM33606S3 buck from VBUS_RAW | ~50 mA peak |

### 3V3 budget

- STM32G0B1KEU6N @ 64 MHz with USB peripheral: ~25 mA peak
- TCPP01 VCC: ~120 µA
- 5× status LEDs @ ~2 mA: ~10 mA
- Misc pull-ups: ~5 mA
- **Total**: ~45 mA peak. TPSM33606S3 (0.6 A) has ~13× margin.

### Boot sequence

1. Cable plugged in → source sees TCPP01's passive dead-battery Rd on
   CC and applies 5 V to VBUS.
2. VBUS_RAW live → buck starts → V3V3 up → MCU out of reset, TCPP01
   VCC powered at the same time (VCC tied directly to V3V3).
3. TCPP01 gate driver now has its supply → FET closes → VBUS_PROT live
   → WAGO passes 5 V to the load.
4. MCU releases UCPD1 dead-battery (`SYSCFG->CFGR1 |= UCPD1_STROBE`),
   drives TCPP01 DB/ high, runs PD contract negotiation.
5. Source steps VBUS to the negotiated voltage; FET stays closed (under
   22 V OVP); load follows.

**Why the buck sits on VBUS_RAW.** The TCPP01 gate driver needs VCC =
3.0–3.6 V to close the FET (DS12900 Table 6 conditions VGS specs on
VCC). If the buck were on VBUS_PROT, V3V3 would need the FET closed,
and the FET would need V3V3 — circular. The split-domain topology
resolves this and gives the MCU survival through OVP events.

---

## 5. Interfaces

### USB-C (input side)

GCT USB4105-GF-A, 16-pin USB-2.0 USB-C receptacle. Data role is
DFP-data-capable (USB DFU / CDC over the same connector). Power role
is sink-only; dead-battery Rd is provided by the STM32G0B1 UCPD
peripheral at boot via its internal hardware (no external 5.1 kΩ
resistors). CC OVP + ESD at the connector come from TCPP01-M12.

D+/D- routed as 90 Ω differential to STM32 USB FS pins.

### Test load output

**WAGO 2060-452/998-404** SMD push-in terminal block, 2-pole, 4 mm pitch,
6 A continuous, reflow-able. Fed from VBUS_PROT (post-OVP-FET). Strip the
end of any bench-eLoad lead and push it in.

Silk: `LOAD +` / `LOAD −`.

### SWD

Tag-Connect TC2030-IDC-NL footprint, **pads only**, no connector
populated:

| TC pin | Signal |
|---|---|
| 1 (VSENSE) | V3V3 |
| 2 (SWDIO) | PA13 |
| 3 (NRST) | NRST |
| 4 (SWCLK) | PA14 |
| 5 (GND) | GND |
| 6 (SWO) | NC — Cortex-M0+ has no SWO/ITM |

Debug is single-wire (SWD) only; printf-trace goes out the UART pads.

### Test pads

Round Ø1.0 mm SMD pads (stdlib `TestPoint` variant `Pad_D1.0mm`) for
everything a PD-firmware developer needs to probe:

| Pad | Net | Notes |
|---|---|---|
| TP_VBUS_RAW | VBUS_RAW | `HV ≤ 22V` |
| TP_VBUS_PROT | VBUS_PROT | `HV ≤ 22V` (load-side, post-FET) |
| TP_FET_GATE | FET_GATE (TCPP01 GATE) | `≤ 28V` — rides to VBUS_PROT + VGS |
| TP_V3V3 | V3V3 | MCU rail reference |
| TP_UART_TX | PA2 (USART2_TX) | debug serial out, 3V3 CMOS |
| TP_UART_RX | PA3 (USART2_RX) | debug serial in, 3V3 CMOS |
| TP_CC1 | CC1_MCU (post-TCPP01) | BMC capture, analog |
| TP_CC2 | CC2_MCU (post-TCPP01) | BMC capture, analog |
| TP_SCOPE_MARKER | PA6 (TIM3_CH1) | firmware scope trigger |
| TP_TCPP01_FLT | PB6 / FLT/ open-drain | fault edges |
| TP_GND_HV | GND | near the HV cluster |
| TP_GND_LV | GND | near the LV signals |

Two GND pads — one near each logical cluster — so a scope ground
pigtail lands close to the signal being probed.

### Status LEDs

Five 0402 LEDs. Firmware-independent rail and fault indicators stay
useful when firmware is broken.

| LED | Color | Driver | Behavior |
|---|---|---|---|
| 3V3_RAIL | Green | Rail-direct (V3V3 → R → LED → GND) | On whenever V3V3 is up |
| TCPP01_FLT | Red | 3V3 → R → LED → FLT/ (open-drain) | On when TCPP01-M12 latches a fault |
| PD_CONTRACT | Blue | MCU GPIO (PA4) | Contract negotiated and held |
| USB_ENUM | Blue | MCU GPIO (PA5) | USB host has enumerated us |
| HEARTBEAT | Yellow | MCU GPIO (PA1) | Slow blink = MCU alive |

---

## 6. Key components

| Function | Part | Package |
|---|---|---|
| MCU | STMicroelectronics STM32G0B1KEU6N | UFQFPN-32, N-pinout |
| USB-C connector | GCT USB4105-GF-A | 16-pin USB-2.0 receptacle |
| USB-C port protection (CC + VBUS) | STMicroelectronics TCPP01-M12 | QFN-12 (3×3) |
| VBUS gating FET | TI CSD17318Q2 NexFET | WSON-6 (2×2) |
| Buck (VBUS_RAW → 3V3) | TI TPSM33606S3QRDNRQ1 | HotRod QFN module, integrated inductor |
| Load output | WAGO 2060-452/998-404 | SMD push-in, 2-pole, 4 mm pitch |
| Status LEDs | 5× 0402 (green, red, yellow, 2× blue) | 0402 |
| SWD | Tag-Connect TC2030-IDC-NL | pads only |
| Test pads | stdlib `TestPoint` variant `Pad_D1.0mm` | round, SMT |
| Fiducials | 3× stdlib `Fiducial` | SMT |

Reference-design passives (MCU decoupling, TCPP01 VCC bypass +
IN_GD ESD cap + OVP divider + FLT pull-up, buck input/output/feedback,
NRST filter, CC line caps, BOOT0 pull-down) live inside the respective
registry components or the board-level `.zen`. `Renfield.zen` is the
authoritative BOM.

---

## 7. Pinout (STM32G0B1KEU6N, UFQFPN-32, N-pinout)

Fixed-function pins (UCPD1, USB FS, SWD, power, reset) follow AN5225
§11.3.1 dead-battery wiring. Free pins are assigned per this spec.

| Pin | Net | Function | AF / I/O type |
|---:|---|---|---|
| 1 | — | spare | PB9 |
| 2 | — | spare | PC14 (LSE) |
| 3 | — | spare | PC15 (LSE) |
| 4 | V3V3 | VDD / VDDA | supply |
| 5 | GND | VSS / VSSA | supply |
| 6 | NRST | reset (Tag-Connect + 100 nF filter) | NRST |
| 7 | — | spare | PA0 |
| 8 | HEARTBEAT_LED | MCU "alive" indicator | PA1 / GPIO out |
| 9 | UART_TX | debug printf out | PA2 / USART2_TX AF1 |
| 10 | UART_RX | debug input | PA3 / USART2_RX AF1 |
| 11 | PD_CONTRACT_LED | contract-held indicator | PA4 / GPIO out |
| 12 | USB_ENUM_LED | USB-enumerated indicator | PA5 / GPIO out |
| 13 | SCOPE_MARKER | hardware scope-trigger output | PA6 / TIM3_CH1 AF1 |
| 14 | — | spare | PA7 |
| 15 | — | spare | PB0 |
| 16 | TCPP01_DB | TCPP01 dead-battery release (active-high) | PB1 / GPIO out |
| 17 | UCPD1_CC2 | post-TCPP01 CC2 | PB15 / UCPD1_CC2 |
| 18 | UCPD1_CC1 | post-TCPP01 CC1 | PA8 / UCPD1_CC1 |
| 19 | UCPD1_DBCC1 | dead-battery sense — short ext. to PA8 | PA9 / UCPD1_DBCC1 |
| 20 | V3V3 | VDDIO2 | supply |
| 21 | UCPD1_DBCC2 | dead-battery sense — short ext. to PB15 | PA10 / UCPD1_DBCC2 |
| 22 | USB_DM | USB D− | PA11 / USB_DM (no remap) |
| 23 | USB_DP | USB D+ | PA12 / USB_DP (no remap) |
| 24 | SWDIO | Tag-Connect pin 2 | PA13 / SWDIO |
| 25 | SWCLK / BOOT0 | Tag-Connect pin 4 + 10 kΩ pull-down | PA14 / SWCLK / BOOT0 |
| 26 | — | spare | PD0 (UCPD2 strobe) |
| 27 | — | spare | PD1 (UCPD2 strobe) |
| 28 | — | spare | PD2 (UCPD2 strobe) |
| 29 | — | spare | PD3 (UCPD2 strobe) |
| 30 | TCPP01_FLT | TCPP01 fault flag (open-drain) | PB6 / GPIO in |
| 31 | — | spare | PB7 |
| 32 | — | spare | PB8 |

*EP (exposed thermal pad) ties to GND.*

### Notes

- **UCPD1 dead-battery wiring (AN5225 §11.3.1).** PA9 shorted to PA8
  and PA10 shorted to PB15 on the PCB so the dead-battery sense
  circuits see the CC lines. Firmware releases the internal Rd via
  `SYSCFG->CFGR1 |= UCPD1_STROBE`.
- **USB FS without remap.** PA11 / PA12 carry USB DM / DP at native
  pin positions; PA11_RMP / PA12_RMP not used.
- **No SWO.** Cortex-M0+ has no ITM / TPIU — trace goes over UART.
- **UCPD2 strobe on PD0–PD3.** Even though those pins are unused,
  firmware should still write `SYSCFG->CFGR1 |= UCPD2_STROBE` at
  boot to release the internal dead-battery Rd on PD0 / PD2.

### GPIO usage

| Class | Count | Pins |
|---|---:|---|
| Fixed peripherals (UCPD1 / USB / SWD) | 7 | PA8, PA9, PA10, PA11, PA12, PA13, PA14, PB15 |
| USART2 | 2 | PA2, PA3 |
| Timer (SCOPE_MARKER) | 1 | PA6 |
| TCPP01 control / status | 2 | PB1 (out), PB6 (in) |
| GPIO outputs (LEDs) | 3 | PA1, PA4, PA5 |
| **Used** | **15 / 20** | |
| Spare | 5 | PA0, PA7, PB0, PB9, + 4× PDn |

---

## 8. Mechanical & environmental

- **Form factor**: A8 (52 × 74 mm), single rectangular outline.
- **Stack-up**: 1.6 mm FR4, **4 layers**, 1 oz copper.
- **Mounting**: 4× M3 holes at the corners.
- **Connector layout**:
  - USB-C receptacle on one short edge.
  - WAGO output on the opposite short edge.
  - Tag-Connect + test pads clustered along the center for bench use.
- **Operating temperature**: 0 °C – 50 °C (bench use).
- **Environmental**: indoor dry, no conformal coating.

---

## 9. Manufacturing & assembly

- **4** layers. Min trace/space: 6/6 mil. Min via: 0.25 mm finished.
- Min package: 0402 passives, UFQFPN-32 MCU, QFN-12 (TCPP01), WSON-6
  (CSD17318Q2). No BGA-style parts.
- Assembly: in-house, single-pass SMT reflow. No through-hole.
- Prototype qty: ~5–10. US assembly. ITAR: no.

---

## 10. Regulatory & compliance

Not a sold product. FCC / CE / UL / USB-IF not targeted.

---

## 11. Design notes

1. **The STM32 runs the PD stack itself.** Using the G0B1's UCPD
   peripheral instead of a dedicated PD controller IC is the whole
   point of the board. TCPP01-M12 is a *protection* chip, not a
   PD controller.

2. **Split MCU / load domain.** DS12900 §6.1: "separate the low
   voltage MCU domain from the high-voltage power path." MCU powered
   from VBUS_RAW; load gated by TCPP01 on VBUS_PROT. MCU survives
   OVP events; load is protected at 22 V in hardware regardless of
   firmware state.

3. **Hardware OVP at 22 V is the absolute backstop.** TCPP01-M12's
   internal OVP divider is programmed to 22 V via the registry
   component's `vbus_ovp_threshold` config. Allows full-spec 20 V
   negotiation with 2 V headroom. A buggy firmware request for > 22 V
   is clamped in silicon.

4. **Buck direct to 3V3 (no intermediate 5 V rail).** The TPSM33606S3
   fixed-trim part takes 3–36 V in and produces 3.3 V out. No LDO
   stage. No 5 V rail because nothing consumes 5 V on this board.

5. **Tag-Connect, not a populated SWD header.** Board is deliberately
   small; SWD is a one-off programming step. Anyone doing serious
   SWD plugs in the Tag-Connect cable.

6. **Crystalless USB.** STM32G0B1 integrates HSI48 + CRS, trimming
   HSI48 against USB SOF. No HSE.

7. **No external flash.** Sink-only PD firmware fits comfortably in
   the G0B1's 512 KB internal flash.

8. **No active cooling.** Worst-case dissipation at 5 A (buck ~130 mW
   + CSD17318Q2 ~500 mW + MCU + LEDs ≈ 800 mW) is fine on a 4-layer
   board with reasonable copper pours.
