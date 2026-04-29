# Renfield (DM0005) — Registry parts to request from the librarian

These parts are referenced in `exploration.md` but are **not yet in the
registry / vendored set**. The librarian should add (or upgrade) each entry,
prefer **QFN / QFN-equivalent (WSON, HotRod, leadless) packages** where the
part exists in multiple options, and confirm sourcing on Digi-Key + Mouser
(US-stocked).

## Required for Renfield

### MCU
- **No new part needed.** Use registry's
  `components/STMicroelectronics/STM32G0B1KxUxN@0.1.2` with config
  `memory_size="512KB"` → instantiates STM32G0B1KEU6N (32-pin
  UFQFPN-32 N-pinout, 512 KB flash). Same family/peripherals as the
  48-pin part; we cut diagnostic LEDs / spare ADC channels to fit the
  GPIO budget (see spec §3 and §10).

### USB D+/D- ESD
- **No new part needed.** Use registry's
  `components/TPD4E05U06QDQARQ1@0.4.5` (already vendored, used by
  Feign's `UsbCSink16P` module). 4-channel ESD array, IEC 61000-4-2
  Level 4. TCPP01-M12 covers CC + VBUS only — D+/D- still need this
  separate part.

### USB-C port protection
- **TCPP01-M12** — STMicroelectronics, USB Type-C port protection,
  **QFN-12 3×3 mm**. Adjustable VBUS OVP 5–22 V via external divider,
  6 V CC OVP, IEC 61000-4-2 L4 ESD, integrated N-FET gate driver +
  charge pump, integrated dead-battery Rd. DigiKey 497-TCPP01-M12CT-ND.
  Sourcing: 14 k+ stocked at Digi-Key, ~$0.50/100.

### VBUS gating MOSFET (driven by TCPP01-M12)
User preference: **TI NexFET, 2×2 package**. Sized for **5 A continuous
through-current**.

- **CSD17318Q2** — TI NexFET, 30 V, **2×2 WSON-6 ("Q2")**, 25 A rated.
  Spec'd 15.1 mΩ @ VGS=8 V; effective ~20 mΩ at TCPP01's regulated
  VGS ≈ 5.5 V. At 5 A continuous → ~500 mW dissipation in a 16 W (Tc)
  package. ±10 V VGS rating is fine — TCPP01-M12 caps gate drive at
  6 V max per DS12900 Table 6. ~$0.50 / 100, ~9 k stocked at Digi-Key.
  **Default pick.**

Note on gate drive: per the TCPP01-M12 datasheet (DS12900 Rev 7,
Table 6), VGS is **internally regulated** to typ 5.5 V (5.0–6.0 V max)
regardless of VBUS level — it's a regulated gate driver, not a
"VBUS+5 V" charge pump. Pick FETs spec'd for low RDSon at VGS=4.5 V,
or confirm enhancement-region behaviour at VGS≈5.5 V from the FET's
RDSon-vs-VGS curve.

### Load-side eFuse
- The registry already has `reference/TPS25948x@0.3.1` covering a TPS25948
  family eFuse. Confirm the exact AYWPR / AYRPR sub-variant for our
  3.5–22 V, 5 A, programmable OVLO + ILIM use case. **No new part needed
  unless that reference doesn't expose OVLO/ILIM config nicely.**

### V/I monitor
- **INA236AIDSGR** — TI, 48 V, 16-bit, I²C bidirectional current/power
  monitor, **WSON-10 (1.4×1.8 mm, QFN-equivalent)**. DigiKey
  296-INA236AIDSGRCT-ND. Replaces our use of the 20-bit INA228 (vendored
  INA228 is fine to keep; add INA236 alongside as the smaller, cheaper
  option for visualization-grade measurement).

### Buck (VBUS → 5 V) — integrated-inductor module
- **TPSM33606S5QRDNRQ1** — TI, 36 V Vin, 0.6 A, fixed 5 V output,
  **HotRod QFN with integrated AEC-Q200 shielded inductor + boot cap**.
  AEC-Q100 grade 1. 2.2 MHz fixed. Replaces LMR36006 + discrete
  inductor with a single module; smaller BOM, smaller PCB area, no
  inductor footprint to spec. Mouser 595-SM33606S5QRDNRQ1, 8.6 k+ in
  stock.
- Backup if module unavailable: **LMR36006FBQDDAR** (WSON-8 HotRod) +
  external shielded inductor (e.g. Coilcraft XAL4030-104MEC) per the
  LMR36006 reference design.

### LDO (5 V → 3.3 V)
- Registry has `components/Texas_Instruments/TPS74x01P@0.1.1` — should
  cover this directly. **No new part needed.**

### Addressable LEDs (V & I bargraph)
- **IN-PI15TAT5R5G5B** — **Inolux**, 1.5×1.5 mm WS2812-protocol
  addressable RGB LED, top-emitting, 5 mA per channel, integrated
  controller, transparent lens. Slightly larger and more visible than
  the 1010 alternative and friendlier for AOI / PnP yield on a sprawling
  bench board. Replaces the previously-suggested `XL-1010RGBC-WS2812B`.
- Quantity needed per board: TBD in spec (~20–24).

### Test-load output connector
- **WAGO 2060-452/998-404** — 2-pole, 4 mm pitch, push-in cage clamp,
  **SMD reflow** (260 °C peak qualified), 6 A @ AWG 24-18, T&R packed.
  DigiKey 2073-2060-452/998-404TR-ND. Same family parts also acceptable:
  2060-451 (1-pole) or 2060-453 (3-pole) — please add the 2-pole as
  default with siblings as variants.

### Test pads
- **No new part needed.** Use stdlib `TestPoint` generic with
  `Pad_1.5x1.5mm` variant for flat SMD pads. Dropped Keystone solder
  loops — not worth the BOM complexity; silkscreen labels next to flat
  pads do the job.

### Saleae-friendly probe headers
- **No new part needed.** Two side-by-side **2×4 0.1″ (2.54 mm) pin
  headers** (8 signal lines total). Use the stdlib `PinHeader` /
  `PinHeader_2x4` generic when scaffolding `Renfield.zen`. If the
  workspace doesn't yet have a 2×4 variant under stdlib, add one then
  — not a librarian task.

### UART debug header
- **No new part needed.** 1×4 0.1″ pin header (GND, TX, RX, 3V3).
  Stdlib generic.

### DIP switch — **8-pos** (upgrade from 4)
- Need an **8-position SMT DIP switch**, 2.54 mm or 1.27 mm pitch,
  T&R packed, reflow-rated. Suggested candidates:
  - **CTS Electrocomponents 219-8MSTR** (8-pos, 2.54 mm SMT)
  - **C&K SDA08H1SBR** (8-pos, 1.27 mm SMT)
  - **Same Sky DSHP08TSGER** (8-pos, 2.54 mm SMT)
  - Librarian: pick a well-stocked Digi-Key part; we don't care about
    pitch as long as it's SMT/T&R.

### I²C GPIO expander (DIP-switch reader)
- **PCA9554APWR** or **PCA9554ABS** — NXP / TI, **8-bit I²C GPIO
  expander**. Preferred package: **HVQFN-16 (4×4 mm)** if available
  (PCA9554ABS), else TSSOP-16 (PCA9554APWR). 8 hardware-selectable
  addresses. Reads the 8 DIP switches over the same I²C bus that runs
  INA236 — zero extra MCU GPIOs.
  - DigiKey 568-1077-1-ND (PCA9554APWR, TSSOP).

### Tactile buttons (RESET, BOOT0)
- Registry has `components/B3U-1000P@0.2.1` (Omron, SMT). **No new part
  needed.**

### SWD footprint
- Registry has `connectors/TagConnect` (TC2030-IDC-NL pads). **No new
  part needed.**

### USB-C receptacle
- Registry has `connectors/UsbC16P@0.1.1` (GCT USB4105-GF-A, 16-pin USB
  2.0). **No new part needed** — we don't need 24-pin since we don't run
  alt-mode / USB 3.0.

## Optional / nice-to-have

- **PCA9955B** (NXP, 16-channel constant-current LED driver, **QFN-32**) —
  fallback if we abandon addressable LEDs for shift-register-driven
  monochrome.
- **TPD1E04U04DPYR** or similar tiny single-line ESD on the BOOT0/RESET
  buttons if we decide to be extra defensive. Probably skip.

## Notes for the librarian

- Prefer parts on **Digi-Key tape-and-reel** with lead time ≤ 4 weeks.
- All packages must be **reflow-compatible** (no through-hole, no manual-
  solder-only parts).
- For each new component, the standard manifest with sourcing metadata
  in `Part(...)` is sufficient — no application circuitry required at
  this stage; we'll grow application circuitry into reference designs
  ourselves where appropriate.
