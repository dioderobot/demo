# Renfield (DM0005) — Design Specification

USB-C Power Delivery **sink** dev board for **STM32G0B1 UCPD firmware
development**. Negotiates up to 20 V / 5 A from a USB-PD source and
passes the negotiated rail through a firmware-gated FET to a WAGO
terminal block. Round SMD test pads expose the signals a PD-firmware
developer actually probes.

The board consumes its master's power on demand. Hence Renfield.

---

## 1. Purpose

A standalone bench-top dev/eval board for the STM32G0B1's UCPD-based
USB-PD sink stack. It is **not a product** — it is the platform on
which we develop and debug PD sink firmware before deploying it on a
production board.

Primary use cases:

1. Negotiate contracts up to 20 V / 5 A (60 W design point) against
   any USB-PD source.
2. Observe CC BMC traffic live with a Saleae Logic Pro 8 on the CC
   test pads.
3. Flash / debug over SWD (Tag-Connect); printf-trace over UART pads.
4. Stress-test firmware by deliberately misbehaving mid-negotiation —
   TCPP01-M12 + OVP FET hardware-clamp VBUS at 22 V regardless of
   firmware state.

---

## 2. Requirements summary

| ID  | Requirement | Priority |
|-----|-------------|----------|
| R1  | USB-C 16P receptacle, sink role, 5/9/15/20 V up to 5 A | P0 |
| R2  | STM32G0B1KBU6N (UFQFPN-32, N-pinout, 128 KB) runs the PD stack on UCPD1 | P0 |
| R3  | TCPP01-M12 + external NexFET: hardware OVP on CC + VBUS, IEC 61000-4-2 L4 ESD on CC | P0 |
| R4  | Hardware VBUS OVP = 22 V; CC OVP = 6 V (TCPP01 internal) | P0 |
| R5  | TPS70933 LDO (VBUS_RAW → 3V3, 30 V Vin, 150 mA) powers the MCU directly | P0 |
| R6  | Load rail gated OFF at reset; firmware enables after PD contract | P0 |
| R7  | WAGO 2060-452 SMD push-in 2-pole terminal block as test-load output | P0 |
| R8  | Tag-Connect TC2030-IDC-NL SWD pads (no connector populated) | P0 |
| R9  | Round Ø1.0 mm SMD test pads for CC, UART, scope trigger, OVP sense, GND | P0 |
| R10 | Crystal-less USB via HSI48 + CRS | P0 |
| R11 | 4-layer, all-SMT, single-pass reflow, T&R, US-stocked | P0 |
| R12 | 60 W maximum design point (20 V × 3 A or 12 V × 5 A — never both) | P0 |

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
           ├──► TCPP01 IN_GD           CC1, CC2 ◄── UCPD1
           │     │                                   │
           │     │ GATE ──► CSD17318Q2 ──► VBUS_PROT ├──► WAGO (load)
           │     │            ▲  (gated)             │
           │     │            │                      │
           │     │            └── Q_KILL ◄── PA4 ── firmware load-enable
           │     │                 (default-off)     │
           │     │                                   │
           │     └──► FLT/ open-drain ──────────────►│  fault LED + PA9
           │                                         │
           │                                         │  PA10 ── TCPP01 DB/
           │                                         │  PA13/PA14 SWD
           │                                         │  PA2/PA3 UART
           │                                         │  PD0 SCOPE_MARKER
           │                                         │  PB0/PB1/PA7 LEDs
           └──► D+/D- ─────────────────── USB FS ────┘
```

**Split MCU / load domain** (DS12900 §6.1). The MCU sits on VBUS_RAW,
so it boots the instant the cable is live and stays powered through
any OVP event for fault reporting. The FET gates the **load rail
only**. Hardware OVP protects the load against source overvoltage;
firmware enables the rail after PD contract via `Q_KILL` (PA4).

---

## 4. Power

### Input

| Parameter | Value |
|-----------|-------|
| Source | USB VBUS via USB-C receptacle (sink role) |
| Voltage | 5 V default; up to 20 V after PD negotiation |
| Maximum draw | 5 A continuous (limited by FET / WAGO rating) |
| Hardware OVP backstop | 22 V at TCPP01-M12 |

### Rails

| Rail | Voltage | Source | Budget |
|------|---------|--------|--------|
| VBUS_RAW | 5–22 V | USB-C receptacle (always hot) | up to 5 A |
| VBUS_PROT | 5–22 V | Post-FET (TCPP01 OVP + firmware gate) → WAGO | up to 5 A |
| 3V3 | 3.3 V ±2 % | TPS70933 LDO from VBUS_RAW | ~40 mA peak |

### 3V3 budget

- MCU @ 64 MHz with USB active: ~25 mA
- TCPP01 VCC: ~120 µA
- LEDs (4 × ~2 mA): ~8 mA
- Pull-ups / misc: ~5 mA
- **Total**: ~40 mA. LDO has 3.7× margin. Sustained 40 mA at 20 V Vin
  dissipates ~670 mW — above the 2×2 mm WSON's free-air envelope but
  bounded by actual draw (typically ~10 mA ≈ 170 mW).

### Boot sequence

1. Cable plugged in → source sees TCPP01's passive dead-battery Rd on
   CC and applies 5 V to VBUS.
2. VBUS_RAW live → LDO starts → V3V3 rises. The 10 kΩ pull-up on
   `Q_KILL`'s gate drags it on before TCPP01 exits UVLO, so the main
   FET stays off regardless of TCPP01 state.
3. MCU comes out of reset; GPIO PA4 is input-floating, so `Q_KILL`
   stays on. WAGO is dead.
4. Firmware writes `SYSCFG->CFGR1 |= UCPD1_STROBE` to release the
   G0's internal CC Rd, configures UCPD1, then drives TCPP01 DB/ high
   (PA10) to close TCPP01's CC switches. PD contract negotiation runs.
5. After an acceptable contract, firmware drives PA4 **low** → `Q_KILL`
   releases → TCPP01 drives the main FET gate → VBUS_PROT follows
   VBUS_RAW → WAGO is live.
6. On fault, firmware drives PA4 high (or returns it to high-Z) to
   re-gate the load off.

Throughout, TCPP01's 22 V OVP still forces the main FET off in
hardware whenever VBUS exceeds 22 V — independent of PA4.

---

## 5. Interfaces

### USB-C

GCT USB4105-GF-A, 16-pin USB-2.0 receptacle, sink role. Connector-side
dead-battery Rd + CC OVP + ESD come from TCPP01-M12 (the CC Rd is
passive and works with the chip unpowered). D+/D- routed as 90 Ω
differential to STM32 USB FS pins.

### Test load output

**WAGO 2060-452/998-404** SMD push-in terminal block, 2-pole, 4 mm
pitch, 6 A. Fed from VBUS_PROT (post-OVP-FET). Silk: `LOAD +` /
`LOAD −`. Bring-up workflow: plug Renfield in, wait for
`PD_CONTRACT` LED, then push the load lead in.

### SWD

Tag-Connect TC2030-IDC-NL footprint, **pads only**. VSENSE tied to
V3V3; SWO pad is NC (Cortex-M0+ has no ITM). Printf-trace goes over
the UART pads.

### Test pads

Round Ø1.0 mm SMD pads (stdlib `TestPoint` variant `Pad_D1.0mm`) for
connector-side CC1/CC2 (raw BMC capture, before TCPP01's OVP
switches), UART RX/TX, a firmware-driven scope trigger, and the
TCPP01 OVP divider tap (`OVP` — scales VBUS by ~1/17; safe 1× probe
point). VBUS_RAW and VBUS_PROT are intentionally not tapped — probe
them via the USB-C housing or the WAGO terminal.

### Status LEDs

Four 0402 LEDs. Three are firmware-driven active-high (GPIO → LED →
resistor → GND): blue PD-contract-held, blue USB-enumerated, yellow
MCU heartbeat. The red TCPP01 fault LED is hardware-driven
(`V3V3 → R → LED → TCPP01.FLT/`) so it lights on any TCPP01 fault
regardless of firmware state.

---

## 6. Key components

| Function | Part | Package |
|---|---|---|
| MCU | STM32G0B1KBU6N (128 KB) | UFQFPN-32, N-pinout |
| USB-C connector | GCT USB4105-GF-A | 16-pin USB-2.0 |
| USB-C port protection | STMicroelectronics TCPP01-M12 | QFN-12 (3×3) |
| Main VBUS FET | TI CSD17318Q2 NexFET | WSON-6 (2×2) |
| Load-enable kill FET | Toshiba SSM3K15ACT | SOT-883 (1.0×0.6) |
| 3V3 LDO | TI TPS70933QDRVRQ1 | WSON-6 (2×2) |
| Load output | WAGO 2060-452/998-404 | SMD push-in |
| SWD | Tag-Connect TC2030-IDC-NL | pads only |

`Renfield.zen` is authoritative for part values, passives, and net
assignments.

---

## 7. Mechanical & environmental

- Form factor: A8 (52 × 74 mm), rectangular.
- Stack-up: 1.6 mm FR4, 4 layers, 1 oz Cu.
- Mounting: 4× M3 corner holes.
- USB-C on one short edge, WAGO on the opposite short edge, SWD +
  test pads clustered in the center.
- Operating temperature: 0 °C – 50 °C (indoor, dry, no conformal).

---

## 8. Manufacturing & assembly

- 4 layers. 6/6 mil trace/space, 0.25 mm min finished via.
- Smallest package: 0402 passives, UFQFPN-32 MCU, SOT-883 (kill FET).
- In-house single-pass SMT reflow, no through-hole.
- Prototype qty ~5–10. US assembly. Not ITAR.

---

## 9. Regulatory & compliance

Not a sold product. FCC / CE / UL / USB-IF not targeted.

---

## 10. Design notes

1. **The STM32 runs the PD stack.** Using G0B1's UCPD peripheral
   instead of a dedicated PD controller IC is the whole point of the
   board. TCPP01-M12 is a *protection* chip, not a PD controller.

2. **Split MCU / load domain.** DS12900 §6.1. MCU on VBUS_RAW (always
   hot), load on VBUS_PROT (gated). MCU survives OVP events for
   fault reporting. Load rail has two independent gates: TCPP01's
   hardware OVP clamp (22 V absolute backstop) and `Q_KILL`
   (firmware enable).

3. **Firmware-gated load rail.** `Q_KILL` (SSM3K15ACT) shunts the
   main FET's gate to GND by default. The 10 kΩ pull-up on its gate
   activates it before TCPP01 exits UVLO, so the load stays off
   through boot with zero firmware involvement. Firmware drives PA4
   low to enable. This lets firmware refuse a contract, reject
   unexpected voltages, or de-energise the load during reset without
   relying on physical disconnect. Hardware OVP still operates
   independently.

4. **LDO on VBUS_RAW.** TCPP01's gate driver needs VCC = 3.0–3.6 V to
   close the FET, so the MCU's 3V3 rail cannot be downstream of the
   FET (circular dependency). LDO over buck because ~10 mA steady
   draw doesn't justify the inductor/ripple overhead on a bench tool.

5. **Tag-Connect, not a populated SWD header.** SWD is a one-off
   programming step; anyone doing serious debug plugs in the cable.

6. **Crystalless USB.** HSI48 + CRS trim against USB SOF. No HSE.

7. **128 KB flash** is plenty for a sink-only PD stack.

### Firmware-facing pinout notes

Exact pin assignments live in `Renfield.zen`; these are
decisions firmware must be aware of.

- **UCPD1 CC pair is swapped** on the TCPP01→MCU side (TCPP01 CC1 →
  MCU UCPD1_CC2, TCPP01 CC2 → MCU UCPD1_CC1). Safe because the
  channels are symmetric; firmware reads the UCPD orientation bit at
  runtime. Silk and test pads track the physical connector, not the
  MCU pin.
- **DBCC pins unconnected.** TCPP01 provides connector-side Rd, so
  the AN5225 §11.3.1 DBCC-to-CC short is omitted. **Firmware must
  write `SYSCFG_CFGR1.UCPD1_STROBE` before driving TCPP01 DB/
  (PA10) high**, otherwise the G0's internal Rd would parallel
  TCPP01 once TCPP01's CC switches close.
- **UCPD2 strobe required.** `SCOPE_MARKER` on PD0 (TIM16_CH1) has
  an internal Rd at reset; firmware must write
  `SYSCFG_CFGR1.UCPD2_STROBE` before configuring the timer output.
- **Load-enable (PA4) is active-LOW.** Drive low after PD contract
  to energise VBUS_PROT; drive high (or leave high-Z) to gate off.
  Reset-default floating keeps the load off.
- **No SWO.** Cortex-M0+ has no ITM / TPIU. Printf over UART.
- **USB FS native pins.** PA11/PA12 with PA11_RMP / PA12_RMP cleared.
