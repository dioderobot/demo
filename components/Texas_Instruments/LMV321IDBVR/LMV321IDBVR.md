# LMV321 Single Low-Voltage Rail-to-Rail Output Operational Amplifier

## 1 Features

- 2.7-V and 5-V performance
- –40°C to +125°C operation
- No crossover distortion
- Low supply current: 130 μA (typical)
- Rail-to-rail output swing
- Unity-gain stable
- 1-MHz unity-gain bandwidth
- 1-V/μs slew rate
- ESD protection exceeds JESD 22
  - 2000-V human-body model
  - 1000-V charged-device model

## 2 Applications

- Desktop PCs
- HVAC: heating, ventilating, and air conditioning
- Motor control: AC induction
- Portable media players
- Professional audio mixers
- Power: telecom DC/DC module

## 3 Description

The LMV321 is a single low-voltage (2.7 V to 5.5 V) operational amplifier with rail-to-rail output swing. It is designed specifically for low-voltage (2.7 V to 5 V) operation, with performance specifications meeting or exceeding the LM358 and LM324 devices that operate from 5 V to 30 V.

The common-mode input voltage range includes ground, providing a 1-MHz unity-gain bandwidth and 1-V/μs slew rate.

For an upgraded version with enhanced performance, refer to LMV321A.

## 4 Device Information

| PART NUMBER | PACKAGE | PACKAGE SIZE |
|---|---|---|
| LMV321IDBVR | SOT-23 (DBV), 5-Pin | 2.90 mm × 2.80 mm |
| LMV321IDCKR | SC-70 (DCK), 5-Pin | 2.00 mm × 2.10 mm |

## 5 Pin Configuration (SOT-23-5, DBV Package)

| PIN | NAME | TYPE | DESCRIPTION |
|---|---|---|---|
| 1 | IN+ | I | Noninverting input |
| 2 | GND | — | Negative supply |
| 3 | IN– | I | Inverting input |
| 4 | OUT | O | Output |
| 5 | VCC+ | — | Positive supply |

## 6 Specifications

### 6.1 Absolute Maximum Ratings

| PARAMETER | MIN | MAX | UNIT |
|---|---|---|---|
| VCC Supply voltage | | 5.5 | V |
| VID Differential input voltage | ±5.5 | | V |
| VI Input voltage range | –0.2 | 5.7 | V |
| TJ Operating junction temperature | | 150 | °C |
| Tstg Storage temperature range | –65 | 150 | °C |

### 6.2 Recommended Operating Conditions

| PARAMETER | MIN | MAX | UNIT |
|---|---|---|---|
| VCC Supply voltage | 2.7 | 5.5 | V |
| TA Operating temperature | –40 | 125 | °C |

### 6.3 Thermal Information (LMV321)

| THERMAL METRIC | DBV (SOT-23) | DCK (SC-70) | UNIT |
|---|---|---|---|
| RθJA Junction-to-ambient | 232.9 | 239.6 | °C/W |

### 6.4 Electrical Characteristics (VCC+ = 2.7 V, TA = 25°C)

| PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
|---|---|---|---|---|---|
| VIO Input offset voltage | | | 1.7 | 7 | mV |
| IIB Input bias current | | | 11 | 250 | nA |
| IIO Input offset current | | | 5 | 50 | nA |
| CMRR Common-mode rejection ratio | VCM = 0 to 1.7 V | 50 | 63 | | dB |
| kSVR Supply-voltage rejection ratio | VCC = 2.7 V to 5 V | 50 | 60 | | dB |
| VICR Common-mode input voltage range | CMRR ≥ 50 dB | –0.2 | | 1.9 | V |
| VO Output swing (high) | RL = 10 kΩ | | VCC – 10 | VCC – 100 | mV |
| VO Output swing (low) | RL = 10 kΩ | | 60 | 180 | mV |
| ICC Supply current | | | 80 | 170 | μA |
| B1 Unity-gain bandwidth | CL = 200 pF | | 1 | | MHz |
| Φm Phase margin | | | 60 | | deg |
| Gm Gain margin | | | 10 | | dB |
| Vn Input noise voltage | f = 1 kHz | | 46 | | nV/√Hz |

### 6.5 Electrical Characteristics (VCC+ = 5 V, TA = 25°C)

| PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
|---|---|---|---|---|---|
| VIO Input offset voltage | | | 1.7 | 7 | mV |
| IIB Input bias current | | | 15 | 250 | nA |
| CMRR Common-mode rejection ratio | VCM = 0 to 4 V | 50 | 65 | | dB |
| VO Output swing (high) | RL = 10 kΩ | | VCC – 10 | VCC – 100 | mV |
| VO Output swing (low) | RL = 10 kΩ | | 65 | 180 | mV |
| AVD Large-signal voltage gain | RL = 2 kΩ | 15 | 100 | | V/mV |
| ICC Supply current | | | 130 | 250 | μA |
| B1 Unity-gain bandwidth | CL = 200 pF | | 1 | | MHz |
| SR Slew rate | | | 1 | | V/μs |
| Vn Input noise voltage | f = 1 kHz | | 39 | | nV/√Hz |

## 7 Power Supply Recommendations

- Operates from 2.7 V to 5.5 V single supply
- Place 0.1-μF bypass capacitors close to the power-supply pins to reduce errors from noisy or high-impedance supplies

## 8 Layout Guidelines

- Connect low-ESR, 0.1-μF ceramic bypass capacitors between each supply pin and ground, placed as close to the device as possible
- Separate grounding for analog and digital portions of circuitry
- Run input traces as far away from supply or output traces as possible
- Place external components as close to the device as possible
- Keep input trace lengths as short as possible

## 9 Orderable Information

| ORDERABLE PART NUMBER | STATUS | PACKAGE | PINS | PACKAGE QTY |
|---|---|---|---|---|
| LMV321IDBVR | Active | SOT-23 (DBV) | 5 | 3000 (T&R) |
| LMV321IDBVT | Active | SOT-23 (DBV) | 5 | 250 (T&R) |
| LMV321IDCKR | Active | SC-70 (DCK) | 5 | 3000 (T&R) |
| LMV321IDCKT | Active | SC-70 (DCK) | 5 | 250 (T&R) |

---

**Document**: SLOS263Y – AUGUST 1999 – REVISED AUGUST 2023
**Source**: [Texas Instruments LMV321 Datasheet](https://www.ti.com/lit/ds/symlink/lmv321.pdf)
**Copyright**: © 2023 Texas Instruments Incorporated
