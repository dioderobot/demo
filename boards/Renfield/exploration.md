# Renfield (DM0005) — STM32G0B1 USB PD Sink Dev Board (exploration)

> **Status: superseded by `spec.md`.** This file is the early exploration
> journal kept for posterity. Several decisions captured below were
> later refined; the authoritative design document is `spec.md` in
> this directory. Major changes vs this exploration:
>
> - MCU is **STM32G0B1KxUxN (UFQFPN-32)** with `memory_size="512KB"`
>   config (reuses existing registry package). ~17 GPIOs used.
> - **No on-board current measurement.** All current/power telemetry
>   is delegated to an external smart load attached at the WAGO
>   terminals. INA236 + I²C bus and the later op-amp + shunt + ADC
>   plan are both dropped.
> - **VBUS voltage** is still measured on-board via a ÷10 resistor
>   divider → MCU ADC (free, useful for AN4879 attach detection).
> - **DIP switch dropped from 8 to 4 positions**, direct-driven on MCU
>   GPIOs (no PCA9554A expander — the I²C bus has no peripherals).
> - **VBUS gate FET** is CSD17318Q2 in 2×2 WSON-6 (not the 3×3
>   CSD17578Q3A originally listed) — TCPP01-M12 datasheet confirmed
>   regulated VGS ≈5.5 V so the smaller FET works fine.
> - **7 status LEDs total**: 2 rail-direct, **2 hardware-driven** off
>   open-drain FLG//FLT/ pins (latching-fault-survives-MCU-crash
>   pattern), 3 GPIO-driven (PD_CONTRACT, USB_ENUM, HEARTBEAT).
> - Single SCOPE_MARKER (was 2).
> - Probe surface is **two side-by-side 2×4 0.1″ headers** (8 signal
>   lines), not a single 2×9. Header B carries ADC_V + FLT/ + FLG/
>   (no current sense).
> - **V bargraph only (8 IN-PI15 LEDs)** — the I bargraph went away
>   with on-board current sensing.
> - eFuse IMON pin still exposed at a test pad as a free analog
>   current scope point for users wanting a quick visual.
> - Test points are **flat SMD pads** (`Pad_1.5x1.5mm`), not Keystone
>   solder loops.
> - Power-rail LEDs are **rail-direct**, not driven by LDO PG / buck
>   PGOOD pins.
> - D+/D- ESD added via the registry's `TPD4E05U06QDQARQ1` (TCPP01-M12
>   does not cover the data lines).
>
> Read `spec.md` for the locked design.

## Decisions locked in (round 1 — partially superseded)

### Architecture
- **Role:** PD **sink** only. No DRP/source. No PPS for now.
- **MCU:** **STM32G0B1CEU6** — UFQFPN-48, 512 KB flash, 144 KB RAM.
- **Port protection:** **TCPP01-M12** + external **CSD17318Q2** NexFET
  (30 V, 2×2 WSON-6, 25 A rated, ~20 mΩ at TCPP01's regulated VGS≈5.5 V)
  sized for 5 A continuous. *(Was originally CSD17578Q3A in 3×3; swapped
  to 2×2 once the TCPP01-M12 datasheet confirmed VGS is regulated to
  5–6 V regardless of VBUS, making the smaller package's higher RDSon
  irrelevant.)*
- **USB-C strategy:** single connector for both PD-under-test and USB
  host (DFU + CDC trace optional). UART is the primary independent debug.
- **Power:** **TPSM33606S5** module (VBUS→5 V, integrated inductor,
  HotRod QFN) → **TPS74x01P** LDO 5 V→3 V3 (registry).
- **No on-board current sensing.** Current/power telemetry is the
  external smart load's job. *(Was originally INA236A over I²C, then
  briefly a low-side shunt + TLV9001 op-amp + MCU ADC; both dropped
  because power instrumentation isn't this board's job — it's a PD
  firmware development platform, and external smart loads do better
  measurement anyway.)*
- **Voltage sense:** VBUS_OUT ÷10 divider → MCU ADC (one resistor
  pair, useful for AN4879 attach detection).
- **Load eFuse:** **TPS25948x** (registry `reference/TPS25948x@0.3.1`),
  programmed for **EN default-OFF, latch-off on fault**, with optional
  DNP cap footprint to convert to auto-retry (silk-labeled).
- **DIP switch:** **4-pos** SMT, direct-driven on MCU GPIOs. Switch
  semantics deferred to firmware. *(Was 8-pos behind a PCA9554A I²C
  expander; cut once INA236 was dropped and the I²C bus became
  empty — 4 positions cover realistic profile-selection use cases.)*
- **Form factor:** **A7 (74×105 mm)**, 4-layer.

### LEDs
- **8× IN-PI15TAT5R5G5B** (Inolux, 1.5×1.5 mm, WS2812 protocol) for V
  bargraph (0–20 V, 2.5 V/LED).
- **8× IN-PI15TAT5R5G5B** for I bargraph (0–5 A, 0.625 A/LED).
- **~10× discrete 0603 LEDs**, GPIO-driven, for status: 3V3_PG, 5V_PG,
  PD_CONTRACT, USB_ENUM, HEARTBEAT, FAULT, SCOPE_MARKER_0,
  SCOPE_MARKER_1, DEBUG_0, DEBUG_1.

### Probe / debug interfaces
- **Main probe header**, 2×9 0.1" with signal-GND alternating
  (Saleae-grommet-friendly):

  | Pin | Signal |
  |---|---|
  | 1 | CC1_MCU |
  | 2 | CC2_MCU |
  | 3 | SDA |
  | 4 | SCL |
  | 5 | scope_marker_0 |
  | 6 | scope_marker_1 |
  | 7 | eFuse FLT/ |
  | 8 | TCPP01 FLG/ |

- **UART header**, 1×4 0.1" — GND, TX, RX, 3V3 (FTDI-ish, no modem lines).
- **SWD via Tag-Connect TC2030-IDC-NL** pads, includes SWO trace pin.
- **Flat SMD test pads** (stdlib `TestPoint` `Pad_1.5x1.5mm` variant)
  for everything not on the headers: VBUS_RAW, VBUS_PROT, VBUS_OUT,
  VBUS_OUT_DIV (÷10), CC1_RAW, CC2_RAW, IMON, TCPP01_EN, eFuse_EN,
  3V3, 5V, distributed GNDs. HV pads carry `HV ≤ 22V` silkscreen
  warnings; physically separated from low-voltage pads to prevent
  probe-clip bridging.

### TCPP01 / eFuse hardware backstop
- TCPP01-M12 OVP threshold: **22 V** (allows full 20 V negotiation
  with margin).
- TPS25948x OVLO: **22 V** (matched).
- Hardware fault behavior: **latch off, MCU re-enables**. Auto-retry
  cap footprint placed but **DNP**, silkscreened
  "POPULATE FOR AUTO-RETRY".

Parts that aren't yet in the registry are tracked in
[`librarian-requests.md`](./librarian-requests.md).

---

## 0. What you asked for, in my words

A USB-C **PD sink** development board — an "engineer's dev board" where the
**STM32G0B1 itself runs the PD sink firmware** (no dedicated PD-controller IC
like TPS25750). It plugs into a PD source (charger / PSU) over USB-C, the MCU
negotiates a power profile up to **20 V / 5 A (60 W cap)**, and the negotiated
VBUS is delivered to a downstream **test-load connector**.

Everything is optimized for *firmware development on the PD stack*, not for
production:

- All critical signals brought out for probing.
- Live, glanceable voltage/current visualization.
- DIP-switchable PD profile selection so you don't have to reflash to swap
  between e.g. "5 V only", "9 V max", "15 V max", "20 V max", current cap,
  etc.
- Robust port + load-side protection so a misbehaving firmware doesn't kill
  itself or the connected test load.
- Easy in-house pick-and-place SMT assembly. No through-hole if avoidable.
- All parts sourceable in the US (Digi-Key / Mouser stocked).

> ⚠ **Wording check:** you wrote "USB PD Sync" but everything else (downstream
> load, test load generators, 60 W cap, e-fuse on the load side) describes a
> PD **sink**. Treating it as **PD sink** unless corrected.

---

## 1. Research summary

### 1.1 STM32G0B1 SKU / flash sizing

`STM32G0B1<pins><flash><pkg>` — confirmed against DS13560 Rev 6:

| Suffix | Flash | Notes |
|---|---|---|
| B  | 128 KB | UCPD only on "N"-pinout 32-pin |
| C  | 256 KB | "  |
| **E** | **512 KB** | **selected** |

Pin-count letters: K=32, **C=48** (selected), R=64, M=80, V=100. RAM is
always 144 KB.

ST's reference USB-PD sink stack (X-CUBE-USB-PD3 / X-CUBE-TCPP) runs in
~30–60 KB flash for a sink-only PD3.0 application. **512 KB is generous**
for development: lots of room for trace logging, multiple PDO profile
tables, USB DFU dual-bank, and growth. ~$4.71 in low qty at Digi-Key.
**No external flash needed.**

> **Locked: `STM32G0B1CEU6`** — UFQFPN-48, 512 KB. 48 pins is comfortable
> for the GPIO budget below; we don't need the "N"-pinout because the
> 48-pin standard-pinout part already exposes PA8/PB15 UCPD1 CC1/CC2 plus
> a separate VDDIO2 supply pin.

### 1.2 PD signal frequencies — scope vs. logic analyzer

Confirmed from the USB PD spec and field reports (element14 community CC
decoding write-up, Saleae docs):

| Signal | Rate / edge | Voltage |
|---|---|---|
| CC1 / CC2 BMC packet | 300 kbit/s ±10 % data; ~1 µs edges | ~1.2 V swing (Rp/Rd dependent) |
| PD timing budget | 4–30 ms message intervals | — |
| VBUS contract slew (5→20 V) | ≥ 30 ms (tSrcReady) | up to 20 V |
| eFuse fault response | ~1–10 µs | up to 22 V |
| I²C (V/I monitor) | 400 kHz–1 MHz | 3.3 V |

**Bottom line: a Saleae Logic Pro 8 (50 MS/s analog, 12-bit, 8 ch) is
sufficient for protocol-level PD work.** Two key caveats:

1. CC swing is ~1.2 V — **below standard logic-analyzer digital
   thresholds**. You must use the analog inputs (Saleae Pro / Logic 8 has 4
   analog @ 10 MS/s 12-bit; Logic Pro 8 has 8 analog @ 50 MS/s 12-bit).
   Pulseview and Logic 2 both have a USB PD BMC decoder.
2. **A scope is still useful** — but only for VBUS slew/ringing analysis and
   fault-path debug (eFuse trip behaviour, FET turn-on glitches). Any
   modern 100 MHz scope (Rigol DHO804 / Siglent SDS800X HD / etc.) is
   plenty. The board does **not** need to be designed around scope-only
   probing — Saleae-type access is the primary expectation.

The board should still **expose CC1/CC2 raw and post-protection**, plus
firmware-controlled "scope marker" GPIOs to make analog cross-trigger
trivial. See §3.7.

### 1.3 USB-C port protection — the "specific IC"

You're remembering the **TCPP01-M12** from ST. This is the exact part used in
ST's `X-NUCLEO-SNK1M1` shield, the standard companion to STM32G0/G4/L5
UCPD-based sinks (AN5418).

| Property | TCPP01-M12 | Notes |
|---|---|---|
| VBUS OVP threshold | 5 – 22 V, externally programmable | Perfect for our 20 V max |
| CC1/CC2 OVP | 6 V (against short-to-VBUS) | "  |
| ESD | IEC 61000-4-2 Level 4 (±8 kV contact) on CC + VBUS | "  |
| Dead-battery Rd | Integrated | "  |
| VBUS gate driver | Internal charge pump for **external N-MOSFET** | We pick the FET to suit current/Rds(on) |
| Quiescent | 0 nA when no cable attached | "  |
| Package | QFN-12, 3 × 3 mm | SMT, T&R |
| Sourcing | Digi-Key 14 k+ in stock @ ~$0.99/1, ~$0.50/100 | US-stocked |
| USB-IF | TID 5205 (sink) verified on the Nucleo shield | "  |

**Alternative considered: TI TPD4S480** (48 V EPR rated) — overkill for our
20 V max scope, larger BOM (needs separate VBUS gate), targets EPR/240 W
docking applications. Skip unless we ever extend to EPR.

**Alternative considered: TI TPD8S300** — 24 V tolerant, focused on data-pin
ESD as well as CC OVP. Useful when D+/D– go through the protector, but adds
parts we don't need; TCPP01-M12 + a small TVS on D+/D– is cheaper.

> **Pick: TCPP01-M12** + external 30 V N-MOSFET (TBD specifics, see §1.4).

### 1.4 VBUS gating MOSFET (downstream of TCPP01)

Sized for: VDS ≥ 30 V, ID continuous ≥ 8 A (50 % margin over 5 A target),
RDSon at VGS = 4.5 V to keep IR drop and self-heating low. The TCPP01-M12
charge-pump drives a single high-side N-FET on the VBUS line.

Candidates (all SMT, T&R, stocked):

| MPN | Pkg | VDS | ID | RDSon @ 4.5 V | $/100 |
|---|---|---|---|---|---|
| AON7400A      | DFN3.3×3.3 | 30 V | 30 A | 4.6 mΩ | ~$0.40 |
| CSD18540Q5B   | SON5×6     | 60 V | 100 A | 1.5 mΩ | ~$1.20 |
| Si7113DN      | PowerPak1212 | 30 V | 14 A | 6 mΩ | ~$1.00 |

For our 5 A peak and engineering margin, **AON7400A** (or comparable 30 V
class) is more than enough — copper area is the limit, not the FET.

### 1.5 Voltage / current monitoring

**TI INA228** (already vendored: `reference/INA228AQDGSRQ1@0.7.3`) — 85 V
common-mode, 20-bit, I²C, integrated shunt-voltage and bus-voltage. Single
chip, single I²C address, MCU does the math and drives bargraphs. No need
for an op-amp + ADC discrete chain.

Shunt: 5 mΩ at 5 A → 25 mV full-scale. INA228 measures down to nV. Use a
2512 5 mΩ ±1 % resistor (1 W rating, derated to ~0.7 W ≪ part rating).

**Alternative: INA236** (48 V, 16-bit, smaller) is half the cost and more
than enough resolution for an LED bargraph readout. Either works; INA228 is
already in the registry, INA236 saves ~$1. Open question §4.4.

### 1.6 Load-side eFuse / protection

**TI TPS25948x** family — already in the registry as
`reference/TPS25948x@0.3.1`. 3.5–23 V, 8 A, integrated back-to-back FETs,
adjustable ILIM, OVLO/UVLO, slew-rate control, fault flag, IMON.

Why on the load side specifically: even though TCPP01-M12 protects the
*input*, a buggy firmware build can negotiate 20 V into a 5 V test load and
fry it before you notice. Putting a configurable eFuse downstream lets you
hard-cap output voltage (OVLO resistor divider) and current (ILIM resistor)
**in hardware** independent of MCU state, and the FLT pin gives the MCU an
unambiguous "you screwed up" signal.

### 1.7 LED visualization (locked)

**Two roles, two technologies.**

**Bargraphs:** **addressable WS2812-protocol RGB**, two 8-LED bars
(Inolux IN-PI15TAT5R5G5B, 1.5×1.5 mm). Single GPIO data line drives
both bars in series via SPI-as-WS2812 with DMA. Rationale:
- 1 GPIO instead of 16.
- Color-coded zones (green/yellow/red) on the same bar replace separate
  "warning" LEDs.
- Over-limit flashes red instead of lighting statically.
- Animation during negotiation gives visual feedback before V/I are
  meaningful.

Mapping:
- V bar: 8 LEDs × 2.5 V/step = 0–20 V.
- I bar: 8 LEDs × 0.625 A/step = 0–5 A.

**Status LEDs:** **discrete 0603 LEDs**, each driven by a dedicated MCU
GPIO (or PG signal). Rationale: a debug LED you can scope-trigger on, or
simply slap your hand over and instantly see whether the MCU is alive,
should be a **dumb single-color LED on a single GPIO** — not a smart pixel
that depends on the firmware's LED stack working correctly. Fault
indicators that ride a working WS2812 driver are useless when the chain
breaks.

10 status LEDs:
- **3V3_PG** — from LDO PG output if available, else MCU GPIO
- **5V_PG** — from TPSM33606 PGOOD
- **PD_CONTRACT** — MCU GPIO
- **USB_ENUM** — MCU GPIO
- **HEARTBEAT** — MCU GPIO
- **FAULT** — MCU GPIO (firmware-aggregated from FLT/FLG)
- **SCOPE_MARKER_0** / **_1** — MCU GPIO, mirror probe-header signals
- **DEBUG_0** / **_1** — MCU GPIO, dev-reserved

Downside of WS2812: needs 4–5.3 V supply → we have a 5 V rail anyway
(see §1.10).

### 1.8 PD profile selection — DIP switch

Registry has `DS04-254-1-04BK-SMT` (4-position SMT DIP, 2.54 mm pitch). 4
bits gives 16 distinct configurations — comfortable for:

- Bit 0–1: max negotiated voltage (5 / 9 / 15 / 20 V)
- Bit 2: max current cap (3 A vs 5 A, regardless of cable e-marker)
- Bit 3: behavior flag (e.g. PPS request / fixed PDO request, or
  hard-reset stress-test mode)

Or we may want 6–8 positions; open question §4.5.

### 1.9 Downstream (test-load) connector

Constraints: 20 V / 5 A continuous, fully reflowable on PnP, no through-hole,
mates with bench DC eLoad leads.

Candidate ranking:

| Part | Type | A | Reflow? | Notes |
|---|---|---|---|---|
| **WAGO 2060-452/998-404** | Push-in cage clamp, 2-pole, 4 mm pitch, T&R | 6 A @ 24-18 AWG | Yes (260 °C peak) | Strip-and-stuff wire entry, no tool needed; ideal for "stick a banana-to-bare-wire lead in" |
| WAGO 2060-451 (×2)        | 1-pole, push-in | 6 A | Yes | Same as above, two singles for clearer +/− separation |
| Phoenix Contact PTSM 0,5/2-2,5 SMD | SMD pluggable terminal block | 6 A | Yes | Removable plug; nicer feel but ~3× the cost |
| Molex Micro-Fit SMT       | Plug + crimp | 8.5 A | Yes | Needs a custom mating cable assembly |
| SMT banana jack           | — | varies | usually no | Most "PCB banana jacks" are TH; SMT ones aren't well-stocked |
| XT30 SMT                  | — | 30 A | rare | Ecosystem is RC/drone, not bench |

> **Pick: WAGO 2060-452 (2-pole)** as primary. Visually clear, accepts any
> bench eLoad lead with a stripped end, fully SMT/PnP-friendly, in stock at
> Digi-Key. **No banana jacks needed** — eLoads usually ship with banana-
> to-alligator-or-stripped leads which insert directly into the WAGO.

### 1.10 Power architecture (locked)

VBUS varies 5–22 V. We need both 5 V (for the WS2812 chain) and 3.3 V
(for the MCU + INA236 + TCPP01 VCC + DIP switch pull-ups + buttons).
Clean topology:

```
VBUS (5–22 V) ──► sync buck ──► 5 V rail ──► WS2812B chain (≤200 mA peak)
                                  │
                                  └──► LDO ──► 3V3 rail (~150 mA peak)
```

**Buck pick: TI `LMR36006FBQDDAR`** — WSON-8 HotRod (QFN-equivalent),
3.6–60 V Vin, 0.6 A out. Critical: supports **100 % duty cycle** so it
passes 5 V → 5 V cleanly when VBUS is at PD default 5 V. 60 V Vin tolerance
gives huge surge headroom over our 22 V max. Datasheet ref design is
copy-pasteable.

**LDO pick: registry's `Texas_Instruments/TPS74x01P`** — already vendored
and used on Feign. 5 V → 3.3 V at ~150 mA = 0.25 W dissipation. Easy.

Boot sequence: source plugs in, applies default 5 V on VBUS. TCPP01-M12
turns on the VBUS gate FET (5 V is below OVP threshold). 5 V flows to the
buck → 5 V rail → LDO → 3.3 V → MCU boots, takes UCPD control, starts
negotiation. Standard X-NUCLEO-SNK1M1 sequence per AN5418.

---

## 2. Block diagram (proposed)

```
┌────────────────────────────────────────────────────────────────────────────┐
│                                                                            │
│   USB-C 16P/24P (sink, GCT USB4105 / 24P equivalent)                       │
│   │                                                                        │
│   ├─ VBUS_RAW ──┬─────────────────────────────┐                            │
│   │             │                             │                            │
│   │             ▼                             ▼                            │
│   │    ┌───────────────┐              ┌──────────────┐                     │
│   │    │  TCPP01-M12   │   gate dr.   │  N-MOSFET    │   VBUS_PROT         │
│   │    │  CC OVP+ESD   │─────────────►│  (e.g. AON-  │────────┬─────────►  │
│   │    │  VBUS OVP     │              │   7400A)     │        │            │
│   │    │  Dead-batt Rd │              └──────────────┘        │            │
│   │    └───┬─────┬─────┘                                      │            │
│   │        │ CC1 │ CC2                                        │            │
│   │        ▼     ▼                                            │            │
│   │     (to MCU UCPD1)                                        ▼            │
│   │                                                  ┌────────────────┐    │
│   ├─ D+/D- ──────► STM32 USB FS (DFU + COM trace)    │ Buck/LDO 3V3   │    │
│   │                                                   │ (input 5–22V) │   │
│   │                                                  └──────┬─────────┘    │
│   │                                                         ▼              │
│   │                                                       3V3 ───► MCU,    │
│   │                                                              INA228,   │
│   │                                                              TCPP01,   │
│   │                                                              LEDs,     │
│   │                                                              dipsw     │
│   │                                                                        │
│   │                                                  VBUS_PROT             │
│   │                                                         │              │
│   │                                                         ▼              │
│   │                                                  ┌────────────┐       │
│   │                                                  │  Shunt 5mΩ │       │
│   │                                                  └──────┬─────┘       │
│   │                                                         │ V/I → INA228 │
│   │                                                         ▼              │
│   │                                                  ┌────────────────┐   │
│   │                                                  │ TPS25948x      │   │
│   │                                                  │ eFuse / OVLO / │   │
│   │                                                  │ ILIM / IMON    │   │
│   │                                                  └──────┬─────────┘   │
│   │                                                         ▼  VBUS_OUT   │
│   │                                                                        │
│   │                                                  WAGO 2060-452 (+ / −) │
│   │                                                                        │
│   │   STM32G0B1CCU6 (UFQFPN-48, 256 KB)                                    │
│   │   ├─ UCPD1: CC1, CC2 (post-TCPP01)                                     │
│   │   ├─ USB FS: PA11/PA12 (D+/D-)                                         │
│   │   ├─ I²C: INA228, plus brought-out header                              │
│   │   ├─ GPIO ×20: two 10-LED bargraphs (V & I)                            │
│   │   ├─ GPIO ×4: DIP switch (with pull-ups)                               │
│   │   ├─ GPIO: TCPP01 EN, TCPP01 FLG/, eFuse EN, eFuse FLT, eFuse IMON ADC │
│   │   ├─ GPIO ×2: scope-marker outputs (firmware-toggled triggers)         │
│   │   ├─ UART: brought to 0.1" header for printf trace                     │
│   │   └─ SWD: Tag-Connect TC2030-IDC-NL pads                               │
│   │                                                                        │
│   │   Test points / probe header: VBUS_RAW, VBUS_PROT, VBUS_OUT,           │
│   │   CC1_RAW, CC2_RAW, CC1_MCU, CC2_MCU, 3V3, GND ×4,                     │
│   │   FLG_TCPP, FLT_FUSE, IMON, scope_marker_0/1                           │
│   │                                                                        │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Tentative requirements (P0 / P1)

| ID | Requirement | Priority |
|---|---|---|
| R1 | USB-C receptacle, sink role, supports up to 20 V / 5 A negotiated | P0 |
| R2 | STM32G0B1**CEU6** (UFQFPN-48, 512 KB) runs USB-PD sink stack | P0 |
| R3 | TCPP01-M12 + ext 30 V N-FET for VBUS OVP, CC OVP, IEC ESD L4 | P0 |
| R4 | Bus-side V & I measurement via **INA236A** (I²C, 48 V, 16-bit, WSON-10) | P0 |
| R5 | **20× WS2812B-1010 addressable RGB chain** for V/I bargraphs + state telltales | P0 |
| R6 | Load-side eFuse (TPS25948x family) with hardware OVLO + ILIM + fault flag | P0 |
| R7 | DIP-switch input, 4 positions, for runtime PD profile selection | P0 |
| R8 | All critical nets on probe-friendly test points / 0.1" header | P0 |
| R9 | USB DFU bootloader path (BOOT0 + RESET buttons) for firmware update | P0 |
| R10 | SWD via Tag-Connect TC2030 footprint (pads only) | P1 |
| R11 | UART trace header (3.3 V CMOS) for PD-stack printf debugging | P0 |
| R12 | Two firmware-controlled "scope-marker" GPIOs (pin header + 1 LED each) | P1 |
| R13 | Downstream load connector: **WAGO 2060-452** SMD push-in | P0 |
| R14 | Fully SMT, single-pass reflow, all parts T&R, US-stocked | P0 |
| R15 | 60 W max design point (20 V × 3 A or 12 V × 5 A — never both simultaneously) | P0 |
| R16 | Two-layer PCB acceptable; 4-layer if it materially helps EMI/ground | P1 |
| R17 | Power: VBUS→5 V buck (LMR36006) → 5 V→3.3 V LDO (TPS74x01P from registry) | P0 |

---

## 4. Open questions — all resolved

All architecture-shaping questions are now closed. Remaining items are
pinmap / firmware concerns that the spec will lock in writing.

| Item | Resolution |
|---|---|
| Role | Sink only |
| PPS | No |
| USB data role | Single USB-C + UART debug |
| MCU | STM32G0B1CEU6 (UFQFPN-48, 512 KB) |
| Current sensor | INA236A + 5 mΩ shunt |
| DIP-switch width | 8-pos via PCA9554A I²C expander |
| Buck topology | Sync module VBUS→5 V (TPSM33606S5) → LDO 5 V→3 V3 (TPS74x01P) |
| MCU | STM32G0B1KxUxN (registry, `memory_size="512KB"` → KEU6N) |
| LED bargraph | 2× 8 IN-PI15 addressable RGB |
| Status LEDs | 6 discrete 0603 (2 rail-direct, 4 GPIO-driven) |
| Form factor | A7 (74×105 mm), 4-layer |
| FET | CSD17318Q2 (2×2 WSON-6, 25 A rated, ~500 mW at 5 A cont) |
| OVP threshold | 22 V on both TCPP01 and eFuse OVLO |
| eFuse default | EN low (off until firmware enables), latch-off, DNP cap for auto-retry |
| Probe headers | 2× side-by-side 2×4 0.1″ (8 signal lines total) |
| UART header | 1×4 GND/TX/RX/3V3 |
| SWD | Tag-Connect TC2030 + SWO |
| Test points | Flat SMD pads (stdlib `Pad_1.5x1.5mm`) |
| Mounting | 4× M3 corner holes |
| Crystal | None — HSI48 + CRS |

---

## 5. Not on the board (intentionally)

- **EPR / 28 V or 48 V support.** Out of scope (max 20 V / 60 W). Would
  need TPD4S480 + 60 V FETs + new buck.
- **D+/D− alt-mode signal routing** (DisplayPort, Thunderbolt). Pure
  PD-power dev board.
- **VCONN sourcing** (powering active e-marked cables). Only relevant if
  we go DRP/source.
- **External flash for firmware.** Not needed — sink-only PD stack fits
  comfortably in 256 KB internal flash.

---

## 6. Registry parts available for reuse

| Need | Registry path | Notes |
|---|---|---|
| INA236 alternative (high-precision) | `reference/INA228AQDGSRQ1@0.7.3` | Pin-compat-ish family; keep available |
| Load-side eFuse | `reference/TPS25948x@0.3.1` | Drop-in for OVLO + ILIM + IMON |
| USB-C 16P front-end | `connectors/UsbC16P@0.1.1` | Connector only; we replace the ESD layer with TCPP01-M12 |
| 5V → 3.3V LDO | `components/Texas_Instruments/TPS74x01P@0.1.1` | From Feign |
| WS2812B addressable LED | `components/XINGLIGHT/XL-1010RGBC-WS2812B@0.3.1` | 1×1 mm RGB |
| DIP switch | `components/DS04-254-1-04BK-SMT` | 4-pos, SMT |
| Tag-Connect SWD | `connectors/TagConnect` | SWD pads |
| Tactile buttons | `components/B3U-1000P@0.2.1` | Reset / BOOT0 |

**Parts to be added by the librarian** are tracked in
[`librarian-requests.md`](./librarian-requests.md):

- `STM32G0B1CEU6` — UFQFPN-48, 512 KB (sibling of vendored 32-pin part)
- `TCPP01-M12` — ST QFN-12 USB-C port protector
- `AON7400A` (or equivalent 30 V N-FET in QFN-class pkg) — VBUS gate FET
- `INA236AIDSGR` — TI WSON-10 V/I monitor
- `LMR36006FBQDDAR` — TI WSON-8 HotRod buck
- `WAGO 2060-452/998-404` — SMD push-in terminal block, 2-pole

---

## 7. What's next

1. **Send `librarian-requests.md` to the librarian** to get the new
   components added to the registry.
2. Answer (or default) §4.A–E above. None are blockers if you just say
   "your call on all of them".
3. Once parts are vendored, scaffold the board properly with `pcb new
   board DM0005` and promote this exploration into a real `spec.md` (the
   Feign one is a good template) including pin assignments, voltage
   checks, and the exact module decomposition.
4. Then implement, build, layout, bring up.
