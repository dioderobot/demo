# TLV9001 Low-Power RRIO 1-MHz Operational Amplifier

## 1 Features

- Scalable CMOS amplifier for low-cost applications
- Rail-to-rail input and output (RRIO)
- Low input offset voltage: ±0.4 mV (typ), ±1.6 mV (max)
- Unity-gain bandwidth: 1 MHz
- Low broadband noise: 27 nV/√Hz at 10 kHz
- Low input bias current: 5 pA
- Low quiescent current: 60 µA/channel (typ)
- Unity-gain stable (phase margin 78° at G=1)
- Internal RFI and EMI rejection filter
- Operational at supply voltages as low as 1.8 V
- Easier to stabilize with higher capacitive load due to resistive open-loop output impedance
- Extended temperature range: –40°C to 125°C
- No phase reversal in overdrive conditions

## 2 Applications

- Sensor signal conditioning
- Power modules
- Active filters
- Low-side current sensing
- Smoke detectors / motion detectors
- Wearable devices
- Large and small appliances
- Barcode scanners
- Personal electronics
- HVAC
- Motor control

## 3 Description

The TLV900x family includes single (TLV9001), dual (TLV9002), and quad-channel (TLV9004) low-voltage (1.8 V to 5.5 V) operational amplifiers with rail-to-rail input and output swing capabilities. These op amps provide a cost-effective solution for space-constrained applications where low-voltage operation and high capacitive-load drive are required. The capacitive-load drive is 500 pF, and the resistive open-loop output impedance makes stabilization easier with much higher capacitive loads.

The TLV900x devices are designed specifically for low-voltage operation (1.8 V to 5.5 V) with performance specifications similar to the TLV600x devices. The robust design features unity-gain stability, an integrated RFI/EMI rejection filter, and no-phase reversal in overdrive conditions.

Shutdown variants (TLV9001S, TLV9002S, TLV9004S) allow switching to standby mode with < 1 µA current consumption.

## 4 Package Options (TLV9001)

| Package | Pins | Body Size |
|---------|------|-----------|
| SOT-23 (DBV) | 5 | 1.60 mm × 2.90 mm |
| SC70 (DCK) | 5 | 1.25 mm × 2.00 mm |
| SOT-553 (DRL) | 5 | 1.65 mm × 1.20 mm |
| X2SON (DPW) | 5 | 0.80 mm × 0.80 mm |

## 5 Pin Configuration (TLV9001 DBV / SOT-23-5)

| Pin | Name | I/O | Description |
|-----|------|-----|-------------|
| 1 | IN+ | I | Non-inverting input |
| 2 | V– (GND) | I/— | Negative supply or ground |
| 3 | IN– | I | Inverting input |
| 4 | OUT | O | Output |
| 5 | V+ | I | Positive supply |

## 6 Absolute Maximum Ratings

| Parameter | Min | Max | Unit |
|-----------|-----|-----|------|
| Supply voltage (V+) – (V–) | — | 7 | V |
| Signal input voltage (common-mode) | (V–) – 0.5 | (V+) + 0.5 | V |
| Signal input voltage (differential) | — | (V+) – (V–) + 0.2 | V |
| Signal input current | –10 | 10 | mA |
| Operating temperature | –55 | 150 | °C |
| Junction temperature | — | 150 | °C |
| Storage temperature | –65 | 150 | °C |

## 7 Recommended Operating Conditions

| Parameter | Min | Max | Unit |
|-----------|-----|-----|------|
| Supply voltage | 1.8 | 5.5 | V |
| Operating temperature | –40 | 125 | °C |

## 8 Key Electrical Characteristics (VS = 1.8–5.5 V, TA = 25°C)

| Parameter | Condition | Min | Typ | Max | Unit |
|-----------|-----------|-----|-----|-----|------|
| Input offset voltage (VOS) | VS = 5 V | — | ±0.4 | ±1.6 | mV |
| VOS temperature drift | –40°C to 125°C | — | ±0.6 | — | µV/°C |
| PSRR | VS = 1.8–5.5 V | 80 | 105 | — | dB |
| Common-mode voltage range | RRIO | (V–) – 0.1 | — | (V+) + 0.1 | V |
| CMRR | VS = 5.5 V, full range | 63 | 77 | — | dB |
| Input bias current | VS = 5 V | — | ±5 | — | pA |
| Noise density (en) | f = 1 kHz | — | 30 | — | nV/√Hz |
| Noise density (en) | f = 10 kHz | — | 27 | — | nV/√Hz |
| Open-loop gain (AOL) | VS = 5.5 V, RL = 10 kΩ | 104 | 117 | — | dB |
| Gain-bandwidth product | VS = 5 V | — | 1 | — | MHz |
| Phase margin | VS = 5.5 V, G = 1 | — | 78 | — | ° |
| Slew rate | VS = 5 V | — | 2 | — | V/µs |
| Output swing from rails | VS = 5.5 V, RL = 10 kΩ | — | 10 | 20 | mV |
| Short-circuit current | VS = 5.5 V | — | ±40 | — | mA |
| Open-loop output impedance | VS = 5 V, f = 1 MHz | — | 1200 | — | Ω |
| Quiescent current (per ch) | IO = 0, VS = 5.5 V | — | 60 | 77 | µA |

## 9 ESD Ratings

| Model | Rating |
|-------|--------|
| Human Body Model (HBM) | ±2000 V |
| Charged Device Model (CDM) | ±1000 V |

## 10 Input and ESD Protection

The TLV900x incorporates internal ESD protection circuits on all pins. For input and output pins, this protection consists of current-steering diodes connected between the input and power-supply pins. These diodes provide in-circuit input overdrive protection as long as current is limited to 10 mA.

A series input resistor can be added to limit input current during overdrive conditions. The added resistor contributes thermal noise and should be minimized in noise-sensitive applications.

**Key for back-power protection:** When the supply is unpowered, the internal ESD clamp diodes are the primary concern for back-powering. With a series resistor (e.g., 10 kΩ) on the input, any leakage current through the clamp diodes is limited to a safe level, preventing the op amp from phantom-powering downstream circuits.

## 11 Power Supply Recommendations

- Specified for 1.8 V to 5.5 V operation (supply voltages > 6 V may permanently damage the device)
- Place 0.1 µF low-ESR ceramic bypass capacitors close to the power-supply pins
- For single-supply applications, a single bypass capacitor from V+ to ground is adequate

## 12 Layout Guidelines

- Place low-ESR 0.1 µF ceramic bypass capacitors between each supply pin and ground, as close to the device as possible
- Separate analog and digital grounds
- Run input traces as far from supply/output traces as possible; cross at 90° if necessary
- Place external components close to the device
- Keep input trace lengths as short as possible
- Consider a driven guard ring around critical traces to reduce leakage

## 13 Ordering Information

| Part Number | Package | Pins |
|-------------|---------|------|
| TLV9001IDBVR | SOT-23 (DBV) | 5 |
| TLV9001IDCKR | SC70 (DCK) | 5 |
| TLV9001IDRLR | SOT-553 (DRL) | 5 |
| TLV9001IDPWR | X2SON (DPW) | 5 |

## 14 Reference Documents

- Datasheet: [TLV9001, TLV9002, TLV9004](https://www.ti.com/lit/ds/symlink/tlv9001.pdf) (SBOS833R, Oct 2017, Rev Nov 2021)
- Product folder: [TLV9001](https://www.ti.com/product/TLV9001)
