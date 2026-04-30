# Renfield (DM0005) — Librarian queue

Five parts referenced by Renfield's spec are not yet in the registry /
vendored set. The librarian should add each entry with the standard
manifest + `Part(...)` sourcing metadata, prefer **QFN / WSON / leadless**
packages where multiple options exist, and confirm Digi-Key + Mouser
US stock with lead time ≤ 4 weeks. All packages must be reflow-compatible
and T&R-packed. No application circuitry is required at this stage —
we'll grow that into reference designs ourselves.

## Required parts

### 1. TCPP01-M12 — USB-C port protection
- **STMicroelectronics**, **QFN-12 3×3 mm**.
- Adjustable VBUS OVP 5–22 V via external divider, 6 V CC OVP, IEC
  61000-4-2 L4 ESD, integrated N-FET gate driver + charge pump,
  integrated dead-battery Rd.
- DigiKey 497-TCPP01-M12CT-ND. ~14 k stocked, ~$0.50/100.

### 2. CSD17318Q2 — VBUS gating MOSFET
- **TI NexFET**, 30 V, **2×2 WSON-6 ("Q2")**, 25 A rated.
- 15.1 mΩ @ VGS=8 V; effective ~20 mΩ at TCPP01's regulated VGS ≈ 5.5 V.
  At 5 A continuous → ~500 mW dissipation in a 16 W (Tc) package.
- ±10 V VGS rating is fine — TCPP01-M12 caps gate drive at 6 V max per
  DS12900 Table 6 (regulated, **not** "VBUS+5 V" charge pump).
- ~$0.50/100, ~9 k stocked at Digi-Key.

### 3. TPSM33606S5QRDNRQ1 — buck module (VBUS → 5 V)
- **TI**, 36 V Vin, 0.6 A, fixed 5 V output, **HotRod QFN with
  integrated AEC-Q200 shielded inductor + boot cap**. AEC-Q100 grade 1.
  2.2 MHz fixed.
- Mouser 595-SM33606S5QRDNRQ1, 8.6 k+ in stock.
- Backup if unavailable: **LMR36006FBQDDAR** (WSON-8 HotRod) + external
  Coilcraft XAL4030-104MEC inductor per the LMR36006 reference design.

### 4. IN-PI15TAT5R5G5B — V-bargraph LEDs (×8 per board)
- **Inolux**, 1.5×1.5 mm WS2812-protocol addressable RGB, top-emitting,
  5 mA per channel, integrated controller, transparent lens.
- Slightly larger and more visible than the 1010 alternative and
  friendlier for AOI / PnP yield on a sprawling bench board.

### 5. WAGO 2060-452/998-404 — test-load output connector
- 2-pole, 4 mm pitch, push-in cage clamp, **SMD reflow** (260 °C peak
  qualified), 6 A @ AWG 24-18, T&R packed.
- DigiKey 2073-2060-452/998-404TR-ND.
- Sibling SKUs (2060-451 1-pole, 2060-453 3-pole) acceptable as
  variants of the same package family.

## Already in the registry / vendored — no action needed

| Function | Path | Notes |
|---|---|---|
| MCU | `components/STMicroelectronics/STM32G0B1KxUxN@0.1.2` | `memory_size="512KB"` → STM32G0B1KEU6N (UFQFPN-32) |
| USB-C receptacle | `connectors/UsbC16P@0.1.1` | GCT USB4105-GF-A, 16-pin USB 2.0 |
| D+/D- ESD | `components/TPD4E05U06QDQARQ1@0.4.5` | IEC 61000-4-2 L4 |
| Load eFuse | `reference/TPS25948x@0.3.2` | Reference design, exposes OVLO/ILIM/UVLO |
| LDO 5V→3V3 | `components/Texas_Instruments/TPS74x01P@0.1.1` | |
| Tactile buttons | `components/B3U-1000P@0.2.1` | Omron SMT |
| 4-pos DIP | `components/DS04-254-1-04BK-SMT@0.3.7` | SMT 2.54 mm |
| SWD pads | `connectors/TagConnect` | TC2030-IDC-NL |
| Stdlib generics | `Capacitor`, `Resistor`, `Led`, `TestPoint` (`Pad_1.5x1.5mm`), `PinHeader` (2×4, 1×4) | |
