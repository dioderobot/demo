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
populated. VSENSE tied to V3V3; SWO pad (pin 6) is NC because
Cortex-M0+ has no ITM / TPIU. Printf-trace goes over the UART test
pads instead.

### Test pads

Round Ø1.0 mm SMD pads (stdlib `TestPoint` variant `Pad_D1.0mm`)
expose the signals a PD-firmware developer actually probes: both CC
lines on the **connector** side (raw BMC capture, before TCPP01's OVP
switches), UART RX/TX, a firmware-driven hardware scope trigger, and
the TCPP01 OVP divider tap (`OVP` — scales VBUS by ~1/17, so it's a
safe 1×-probe point for VBUS events without a HV probe). VBUS_RAW
and VBUS_PROT are not tapped; probe them via the USB-C housing or
the WAGO terminal instead.

See `Renfield.zen` for the exact net list and silk labels.

### Status LEDs

Four 0402 LEDs: red TCPP01 fault (hardware-driven so it works
regardless of firmware state), blue PD-contract-held, blue
USB-enumerated, yellow MCU heartbeat. All firmware-driven LEDs sink
current from a 3V3 pull-up through the MCU GPIO. The fault LED is
wired `3V3 → R → LED → TCPP01.FLT/` so it lights whenever TCPP01
latches a fault, independent of firmware.

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

`Renfield.zen` is the authoritative BOM; all per-part values,
reference-design passives, and net assignments live there.

---

## 7. Mechanical & environmental

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

## 8. Manufacturing & assembly

- **4** layers. Min trace/space: 6/6 mil. Min via: 0.25 mm finished.
- Min package: 0402 passives, UFQFPN-32 MCU, QFN-12 (TCPP01), WSON-6
  (CSD17318Q2). No BGA-style parts.
- Assembly: in-house, single-pass SMT reflow. No through-hole.
- Prototype qty: ~5–10. US assembly. ITAR: no.

---

## 9. Regulatory & compliance

Not a sold product. FCC / CE / UL / USB-IF not targeted.

---

## 10. Design notes

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

### Firmware-facing pinout notes

These are decisions that firmware must be aware of. Exact pin
assignments live in `Renfield.zen`.

- **UCPD1 CC pair is swapped** on the TCPP01→MCU side for layout
  routing (TCPP01 CC1 → MCU UCPD1_CC2, TCPP01 CC2 → MCU UCPD1_CC1).
  Safe because the two channels are symmetric; firmware reads the
  UCPD orientation bit at runtime regardless. Silk and test pads
  still track the physical USB-C connector.
- **DBCC pins unconnected.** TCPP01 provides connector-side
  dead-battery Rd, so the AN5225 §11.3.1 DBCC-to-CC short is
  omitted. **Firmware must write `SYSCFG_CFGR1.UCPD1_STROBE`
  before driving TCPP01 DB/ high**, otherwise the G0's internal Rd
  would parallel TCPP01's live CC driver after the switches close.
- **UCPD2 strobe also required.** `SCOPE_MARKER` is on PD0
  (TIM16_CH1). PD0/PD2 carry an internal Rd at reset, so firmware
  must write `SYSCFG_CFGR1.UCPD2_STROBE` before configuring the
  timer output.
- **No SWO.** Cortex-M0+ has no ITM / TPIU; printf-trace goes over
  UART.
- **USB FS without remap.** PA11/PA12 carry USB DM/DP at their
  native pin positions; PA11_RMP / PA12_RMP bits stay cleared.
