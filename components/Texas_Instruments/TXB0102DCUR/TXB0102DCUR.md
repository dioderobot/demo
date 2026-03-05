# TXB0102 2-Bit Bidirectional Voltage-Level Translator

## 1 Features

- 2-bit bidirectional voltage-level translator
- Auto-direction sensing — no direction-control signal needed
- A port: 1.2 V to 3.6 V (VCCA)
- B port: 1.65 V to 5.5 V (VCCB), VCCA ≤ VCCB
- VCC isolation: if either VCC is at GND, all outputs go high-impedance
- OE (output-enable) input referenced to VCCA
- Ultra-low power consumption: 4 µA max ICC
- Ioff supports partial-power-down mode
- Latch-up performance exceeds 100 mA per JESD 78, Class II
- ESD protection:
  - A port: ±2500 V HBM, ±1500 V CDM
  - B port: ±15 kV HBM, ±1500 V CDM
- Available in NanoFree (DSBGA) package

## 2 Applications

- Handsets and smartphones
- Tablets
- Desktop PCs
- SWD/JTAG debug level shifting
- General push-pull CMOS level translation

## 3 Description

The TXB0102 is a 2-bit noninverting translator that uses two separate configurable power-supply rails. The A port tracks VCCA (1.2 V to 3.6 V), and the B port tracks VCCB (1.65 V to 5.5 V). This allows universal low-voltage bidirectional translation between 1.2 V, 1.5 V, 1.8 V, 2.5 V, 3.3 V, and 5 V nodes.

The architecture uses output one-shot edge-rate accelerators on both rising and falling edges to improve data rates. When a transition is detected, low-impedance PMOS (rising) or NMOS (falling) transistors momentarily turn on to speed up the edge. Typical output impedance during transition is 40–70 Ω depending on supply voltage.

When OE is low, all outputs are placed in high-impedance state. The device also supports partial-power-down via Ioff circuitry, which prevents current backflow when the device is unpowered.

**Important:** This device is designed for push-pull CMOS outputs only. It must NOT be used with open-drain drivers (I2C, 1-Wire). For open-drain applications, use the TXS0102 instead.

## 4 Package Options

| Part Number | Package | Body Size |
|-------------|---------|-----------|
| TXB0102DCU/DCUR | VSSOP (8) | 2.30 mm × 2.00 mm |
| TXB0102YZP/YZPR | DSBGA (8) | 0.90 mm × 1.80 mm |

## 5 Pin Configuration (DCU / VSSOP-8)

| Pin | Name | Type | Description |
|-----|------|------|-------------|
| 1 | B2 | I/O | Bidirectional data, referenced to VCCB |
| 2 | GND | S | Ground |
| 3 | VCCA | S | A-port supply (1.2 V to 3.6 V) |
| 4 | A2 | I/O | Bidirectional data, referenced to VCCA |
| 5 | A1 | I/O | Bidirectional data, referenced to VCCA |
| 6 | OE | I | Output enable (active high, referenced to VCCA) |
| 7 | VCCB | S | B-port supply (1.65 V to 5.5 V) |
| 8 | B1 | I/O | Bidirectional data, referenced to VCCB |

## 6 Absolute Maximum Ratings

| Parameter | Min | Max | Unit |
|-----------|-----|-----|------|
| VCCA supply voltage | –0.5 | 4.6 | V |
| VCCB supply voltage | –0.5 | 6.5 | V |
| A port input voltage | –0.5 | 4.6 | V |
| B port input voltage | –0.5 | 6.5 | V |
| Continuous output current | — | ±50 | mA |
| Continuous VCC/GND current | — | ±100 | mA |
| Junction temperature | — | 150 | °C |
| Storage temperature | –65 | 150 | °C |

## 7 Recommended Operating Conditions

| Parameter | Min | Max | Unit |
|-----------|-----|-----|------|
| VCCA | 1.2 | 3.6 | V |
| VCCB | 1.65 | 5.5 | V |
| Operating temperature | –40 | 85 | °C |
| Input transition rate (A port) | — | 40 | ns/V |
| Input transition rate (B port) | — | 30–40 | ns/V |

## 8 Key Electrical Characteristics (TA = 25°C)

| Parameter | Condition | Typ/Max | Unit |
|-----------|-----------|---------|------|
| ICC (total) | Outputs enabled, no load | 3.5 (typ) / 4 (max) | µA |
| ICC (OE disabled) | Outputs hi-Z | ~3.35 | µA |
| Ioff leakage | Partial power-down | ±1 | µA |
| IOZ (hi-Z leakage) | OE = GND | ±1 | µA |
| Cio (A port) | — | 5 | pF |
| Cio (B port) | — | 11 | pF |
| Ci (OE) | — | 2.5 | pF |

## 9 Data Rates by VCCA Voltage

| VCCA | Max Data Rate | Min Pulse Width |
|------|---------------|-----------------|
| 1.2 V | 20 Mbps | 50 ns |
| 1.5 V | 40 Mbps | 25 ns |
| 1.8 V | 60 Mbps | 17 ns |
| 2.5 V | 100 Mbps | 10 ns |
| 3.3 V | 100 Mbps | 10 ns |

## 10 Switching Characteristics (VCCA = 1.8 V, typ)

| Parameter | A→B | B→A | Unit |
|-----------|-----|-----|------|
| Propagation delay (tpd), VCCB = 3.3 V | 4.7 ns | 5.2 ns | ns |
| Propagation delay (tpd), VCCB = 5 V | 4.5 ns | 4.5 ns | ns |
| Rise time (A port) | 3.2 ns | — | ns |
| Rise time (B port) | — | 1.2 ns | ns |
| OE enable time (ten) | 1 µs | 1 µs | µs |
| OE disable time (tdis) | ~14 ns | ~16 ns | ns |

## 11 Design Considerations

### Input Driver Requirements
- The external driver must provide at least **±2 mA** drive strength
- Push-pull CMOS outputs only (NOT open-drain)

### Output Load
- Max recommended capacitive load: **70 pF**
- Keep PCB trace lengths short to avoid excessive capacitance
- One-shot circuits stay on for ~10 ns; heavy loads may cause incomplete transitions

### Pull-up/Pull-down Resistors
- External pull-up or pull-down resistors on data I/Os must be **> 50 kΩ**
- Lower values will contend with the weak DC output drivers

### OE Pin
- Must be pulled low through a resistor during power-up/power-down to ensure hi-Z state
- Referenced to VCCA

### Power Supply
- VCCA must always be ≤ VCCB
- No power-supply sequencing requirement (either can come up first)
- Place 0.1 µF bypass capacitors close to both VCCA and VCCB pins

## 12 Reference Documents

- Datasheet: [TXB0102](https://www.ti.com/lit/ds/symlink/txb0102.pdf) (SCES641E, May 2007, Rev Oct 2023)
- App note: [A Guide to Voltage Translation With TXB-Type Translators](https://www.ti.com/lit/an/scea043/scea043.pdf) (SCEA043)
- Product folder: [TXB0102](https://www.ti.com/product/TXB0102)
