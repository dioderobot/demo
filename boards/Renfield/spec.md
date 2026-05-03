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
| R2  | STM32G0B1KBU6N (UFQFPN-32, N-pinout, 128 KB flash) runs the PD sink stack on UCPD1 | P0 |
| R3  | TCPP01-M12 + external NexFET provide hardware OVP on CC + VBUS, IEC 61000-4-2 L4 ESD on CC | P0 |
| R4  | Hardware VBUS OVP = 22 V; CC OVP = 6 V (TCPP01-M12 internal) | P0 |
| R5  | TPS70933 LDO (VBUS_RAW → 3V3, 30 V Vin, 150 mA) powers the MCU directly | P0 |
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
 VBUS_RAW ─┬──► TPS70933 LDO ────► V3V3 ──► MCU + TCPP01 VCC
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
           │     └──► FLT/ open-drain ──────────────►│  fault LED + PA9
           │                                         │
           │                                         │  PA10 ── TCPP01 DB/
           │                                         │  PA13/PA14 SWD
           │                                         │  PA2/PA3 UART
           │                                         │  PD0 SCOPE_MARKER
           │                                         │  PB0/PB1/PA7 LEDs
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
| 3V3 | 3.3 V ±2 % | TPS70933 LDO from VBUS_RAW | ~50 mA peak |

### 3V3 budget

- STM32G0B1 @ 64 MHz with USB peripheral: ~25 mA peak
- TCPP01 VCC: ~120 µA
- 4× status LEDs @ ~2 mA: ~8 mA
- Misc pull-ups: ~5 mA
- **Total**: ~40 mA peak. TPS70933 (150 mA) has ~3.7× margin.
  Worst-case LDO dissipation at 20 V Vin × 40 mA = 670 mW is outside
  the 2×2 mm WSON-6 thermal envelope; steady-state is bounded by
  actual draw (~10 mA, ~170 mW) and derated airflow on a 4-layer
  pour. Keep sustained USB activity + all LEDs on at 20 V out of
  scope — dev board, not product.

### Boot sequence

1. Cable plugged in → source sees TCPP01's passive dead-battery Rd on
   CC and applies 5 V to VBUS.
2. VBUS_RAW live → LDO starts → V3V3 up → MCU out of reset; TCPP01
   VCC powers up at the same time (VCC tied directly to V3V3).
3. TCPP01 gate driver now has its supply → FET closes autonomously
   (VBUS < 22 V) → VBUS_PROT live → WAGO passes 5 V to the load.
4. MCU releases UCPD1 internal dead-battery Rd
   (`SYSCFG->CFGR1 |= UCPD1_STROBE`) **before** driving TCPP01 DB/
   high. Once DB/ is high, TCPP01 closes its CC switches and the
   G0 becomes the CC driver.
5. Firmware runs PD contract negotiation. Source steps VBUS to the
   negotiated voltage; FET stays closed under 22 V; load follows.

**Why the LDO sits on VBUS_RAW.** The TCPP01 gate driver needs VCC =
3.0–3.6 V to close the FET (DS12900 Table 6 conditions VGS specs on
VCC). If the LDO were on VBUS_PROT, V3V3 would need the FET closed,
and the FET would need V3V3 — circular. The split-domain topology
resolves this and gives the MCU survival through OVP events.

---

## 5. Interfaces

### USB-C (input side)

GCT USB4105-GF-A, 16-pin USB-2.0 USB-C receptacle. Data role is
DFP-data-capable (USB DFU / CDC over the same connector). Power role
is sink-only. Connector-side dead-battery Rd is provided by TCPP01-M12
(passive, works with the chip unpowered). CC OVP + ESD at the
connector come from TCPP01-M12. D+/D- routed as 90 Ω differential to
STM32 USB FS pins.

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

| Pad | Silk | Net | Notes |
|---|---|---|---|
| TP_OVP_SENSE | `OVP` | TCPP01 VBUS_CTRL divider tap | ~VBUS/17; safe 1× probe of the OVP input |
| TP_UART_TX | `TX` | PA2 (USART2_TX) | debug serial out, 3V3 CMOS |
| TP_UART_RX | `RX` | PA3 (USART2_RX) | debug serial in, 3V3 CMOS |
| TP_CC1 | `CC1` | USB_C.CC1 (connector-side) | raw BMC capture |
| TP_CC2 | `CC2` | USB_C.CC2 (connector-side) | raw BMC capture |
| TP_SCOPE_MARKER | `TRG` | PD0 (TIM16_CH1) | firmware scope trigger |
| TP_GND | `GND` | GND | single shared ground |

CC pads are tapped on the **connector** side (before TCPP01's OVP
switches) so a Logic Pro captures true wire signals — including any
TCPP01-initiated CC cutoff event. VBUS and VBUS_PROT are not test
pads; probe them via the USB-C housing or the WAGO terminal instead.
OVP sense is the safe high-side scope point: it scales 22 V to
~1.3 V at the divider tap.

### Status LEDs

Four 0402 LEDs. The fault indicator is firmware-independent so it
stays useful when firmware is broken.

| LED | Color | Driver | Behavior |
|---|---|---|---|
| TCPP01_FLT | Red | 3V3 → R → LED → FLT/ (open-drain from PA9) | On when TCPP01-M12 latches a fault |
| PD_CONTRACT | Blue | MCU GPIO (PB1) | Contract negotiated and held |
| USB_ENUM | Blue | MCU GPIO (PA7) | USB host has enumerated us |
| HEARTBEAT | Yellow | MCU GPIO (PB0) | Slow blink = MCU alive |

---

## 6. Key components

| Function | Part | Package |
|---|---|---|
| MCU | STMicroelectronics STM32G0B1KBU6N (128 KB) | UFQFPN-32, N-pinout |
| USB-C connector | GCT USB4105-GF-A | 16-pin USB-2.0 receptacle |
| USB-C port protection (CC + VBUS) | STMicroelectronics TCPP01-M12 | QFN-12 (3×3) |
| VBUS gating FET | TI CSD17318Q2 NexFET | WSON-6 (2×2) |
| 3V3 LDO (VBUS_RAW → 3V3) | TI TPS70933QDRVRQ1 | WSON-6 (2×2), 30 V Vin, 150 mA |
| Load output | WAGO 2060-452/998-404 | SMD push-in, 2-pole, 4 mm pitch |
| Status LEDs | 4× 0402 (red, yellow, 2× blue) | 0402 |
| SWD | Tag-Connect TC2030-IDC-NL | pads only |
| Test pads | stdlib `TestPoint` variant `Pad_D1.0mm` | round, SMT |
| Fiducials | 3× stdlib `Fiducial` | SMT |

Reference-design passives (MCU decoupling, TCPP01 VCC bypass +
IN_GD ESD cap + OVP divider + FLT pull-up, LDO input/output,
NRST filter, CC line caps, BOOT0 pull-down) live inside the
respective registry components or the board-level `.zen`.
`Renfield.zen` is the authoritative BOM.

---

## 7. Pinout (STM32G0B1KBU6N, UFQFPN-32, N-pinout)

UCPD1 CC pair is swapped on the TCPP01→MCU side for layout routing
convenience (the two channels are symmetric; firmware reads the
orientation bit at runtime). DBCC pins are left unconnected —
TCPP01 provides connector-side dead-battery Rd, and firmware
releases the G0's internal Rd via `SYSCFG_CFGR1.UCPD1_STROBE`.

| Pin | Net | Function | I/O |
|---:|---|---|---|
| 1 | — | spare | PB9 |
| 2 | — | spare | PC14 (LSE) |
| 3 | — | spare | PC15 (LSE) |
| 4 | V3V3 | VDD / VDDA | supply |
| 5 | GND | VSS / VSSA | supply |
| 6 | NRST | reset (Tag-Connect + 100 nF filter) | NRST |
| 7 | — | spare | PA0 |
| 8 | — | spare | PA1 |
| 9 | UART_TX | debug printf out | PA2 / USART2_TX AF1 |
| 10 | UART_RX | debug input | PA3 / USART2_RX AF1 |
| 11 | — | spare | PA4 |
| 12 | — | spare | PA5 |
| 13 | — | spare | PA6 |
| 14 | USB_ENUM_LED | USB-enumerated indicator | PA7 / GPIO out |
| 15 | HEARTBEAT_LED | MCU "alive" indicator | PB0 / GPIO out |
| 16 | PD_CONTRACT_LED | contract-held indicator | PB1 / GPIO out |
| 17 | UCPD1_CC2 *(carries connector CC1)* | post-TCPP01, swapped | PB15 / UCPD1_CC2 |
| 18 | UCPD1_CC1 *(carries connector CC2)* | post-TCPP01, swapped | PA8 / UCPD1_CC1 |
| 19 | TCPP01_FLT | TCPP01 fault flag (open-drain) | PA9 / GPIO in |
| 20 | V3V3 | VDDIO2 | supply |
| 21 | TCPP01_DB | TCPP01 dead-battery release (active-high) | PA10 / GPIO out |
| 22 | USB_DM | USB D− | PA11 / USB_DM (no remap) |
| 23 | USB_DP | USB D+ | PA12 / USB_DP (no remap) |
| 24 | SWDIO | Tag-Connect pin 2 | PA13 / SWDIO |
| 25 | SWCLK / BOOT0 | Tag-Connect pin 4 + 10 kΩ pull-down | PA14 / SWCLK / BOOT0 |
| 26 | SCOPE_MARKER | hardware scope-trigger output | PD0 / TIM16_CH1 |
| 27 | — | spare | PD1 |
| 28 | — | spare | PD2 |
| 29 | — | spare | PD3 |
| 30 | — | spare | PB6 |
| 31 | — | spare | PB7 |
| 32 | — | spare | PB8 |

*EP (exposed thermal pad) ties to GND.*

### Notes

- **UCPD1 CC swap (TCPP01→MCU side only).** TCPP01 CC1 drives MCU
  PB15 (UCPD1_CC2); TCPP01 CC2 drives MCU PA8 (UCPD1_CC1). Saves a
  trace crossing; safe because CC1/CC2 channels are symmetric and
  firmware uses the runtime orientation bit anyway. Connector-side
  labels (silk, test pads) still track the receptacle.
- **DBCC pins unconnected.** No AN5225 §11.3.1 short-to-CC. TCPP01
  provides connector-side Rd; firmware must write
  `SYSCFG_CFGR1.UCPD1_STROBE` before driving TCPP01 DB/ high,
  otherwise the G0's internal Rd would parallel TCPP01 once the
  switches close.
- **USB FS without remap.** PA11 / PA12 carry USB DM / DP at native
  pin positions; PA11_RMP / PA12_RMP not used.
- **No SWO.** Cortex-M0+ has no ITM / TPIU — trace goes over UART.
- **UCPD2 strobe on PD0–PD3.** PD0 is actively used (SCOPE_MARKER /
  TIM16_CH1), so firmware must write
  `SYSCFG_CFGR1.UCPD2_STROBE` at boot to release the PD0/PD2
  internal dead-battery Rd before configuring timer output.

### GPIO usage

| Class | Count | Pins |
|---|---:|---|
| UCPD1 CC | 2 | PA8, PB15 |
| USB FS | 2 | PA11, PA12 |
| SWD | 2 | PA13, PA14 |
| USART2 | 2 | PA2, PA3 |
| Timer (SCOPE_MARKER) | 1 | PD0 |
| TCPP01 control / status | 2 | PA9 (in), PA10 (out) |
| GPIO outputs (LEDs) | 3 | PB0, PB1, PA7 |
| **Used** | **14** | |
| Spare | 14 | PA0, PA1, PA4, PA5, PA6, PB6–PB9, PC14, PC15, PD1–PD3 |

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

4. **LDO direct to 3V3 (no intermediate 5 V rail).** TPS70933 takes
   3–30 V in and produces 3.3 V out with ~1 µA Iq. At ~10 mA
   steady load (MCU + TCPP01 + one or two LEDs) the 170 mW dissipation
   at 20 V Vin is fine on the 2×2 mm WSON-6 with a 4-layer pour.
   A dedicated buck would be more efficient but the simplicity
   (no inductor, no ripple) wins on a bench tool.

5. **Tag-Connect, not a populated SWD header.** Board is deliberately
   small; SWD is a one-off programming step. Anyone doing serious
   SWD plugs in the Tag-Connect cable.

6. **Crystalless USB.** STM32G0B1 integrates HSI48 + CRS, trimming
   HSI48 against USB SOF. No HSE.

7. **No external flash.** Sink-only PD firmware fits comfortably in
   the G0B1's 128 KB internal flash.

8. **No active cooling.** Worst-case dissipation at 5 A (LDO ~170 mW
   + CSD17318Q2 ~500 mW + MCU + LEDs ≈ 800 mW) is fine on a 4-layer
   board with reasonable copper pours.
