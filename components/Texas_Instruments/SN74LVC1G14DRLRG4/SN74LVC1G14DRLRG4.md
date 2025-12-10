# SN74LVC1G14 Single Schmitt-Trigger Inverter 

## 1 Features

- Latch-up performance exceeds 100 mA per JESD 78, Class II
- ESD protection exceeds JESD 22
- 2000V human-body model (A114-A)
- 200V machine model (A115-A)
- 1000V charged-device model (C101)
- Available in the Texas Instruments NanoFree ${ }^{\text {TM }}$ package
- Supports $5 \mathrm{~V} \mathrm{~V}_{\mathrm{CC}}$ operation
- Inputs accept voltages to 5.5 V
- Maximum $t_{\text {pd }}$ of 4.6 ns at 3.3 V
- Low power consumption, $10 \mu \mathrm{~A}$ maximum $\mathrm{I}_{\mathrm{CC}}$
- $\pm 24 \mathrm{~mA}$ output drive at 3.3 V
- $\mathrm{I}_{\text {off }}$ supports partial-power-down mode operation


## 2 Applications

- AV receiver
- Audio dock: portable
- Blu-ray player and home theater
- Embedded PC
- MP3 player/recorder (portable audio)
- Personal Digital Assistant (PDA)
- Power: telecom/server AC/DC supply: single controller: analog and digital
- Solid State Drive (SSD): client and enterprise
- TV: LCD/Digital and High-Definition (HDTV)
- Tablet: enterprise
- Video analytics: server
- Wireless headset, keyboard, and mouse
![img-0.jpeg](./images/img-0.jpeg)

Logic Diagram (Positive Logic)
(DBV, DCK, DRL, DRY, DPW, and YZP Package)

## 3 Description

This single Schmitt-trigger inverter is designed for 1.65 V to $5.5 \mathrm{~V} \mathrm{~V}_{\mathrm{CC}}$ operation.

The SN74LVC1G14 device contains one inverter and performs the Boolean function $Y=\bar{A}$. The device functions as an independent inverter with Schmitttrigger inputs, so the device has different input threshold levels for positive-going $\left(V_{T+}\right)$ and negativegoing $\left(V_{T-}\right)$ signals to provide hysteresis $\left(\Delta V_{T}\right)$ which makes the device tolerant to slow or noisy input signals.

NanoFree ${ }^{\text {TM }}$ package technology is a major breakthrough in IC packaging concepts, using the die as the package.

This device is fully specified for partial-power-down applications using $\mathrm{I}_{\text {off. }}$. The $\mathrm{I}_{\text {off }}$ circuitry disables the outputs when the device is powered down. This inhibits current backflow into the device which prevents damage to the device.

Package Information

| PART NUMBER | PACKAGE ${ }^{(1)}$ | PACKAGE <br> SIZE ${ }^{(2)}$ | BODY SIZE (NOM) ${ }^{(3)}$ |
| :--: | :--: | :--: | :--: |
| SN74LVC1G14 | DBV (SOT-23, 5) | $\begin{aligned} & 2.90 \mathrm{~mm} * \\ & 2.80 \mathrm{~mm} \end{aligned}$ | $2.90 \mathrm{~mm} * 1.60 \mathrm{~mm}$ |
|  | DCK (SC70, 5) | $\begin{aligned} & 2.00 \mathrm{~mm} * \\ & 2.10 \mathrm{~mm} \end{aligned}$ | $2.00 \mathrm{~mm} * 1.25 \mathrm{~mm}$ |
|  | DRL (SOT-5X3, <br> 5) | $\begin{aligned} & 1.60 \mathrm{~mm} * \\ & 1.60 \mathrm{~mm} \end{aligned}$ | $1.60 \mathrm{~mm} * 1.20 \mathrm{~mm}$ |
|  | DRY (USON, 6) | $\begin{aligned} & 1.45 \mathrm{~mm} * \\ & 1.00 \mathrm{~mm} \end{aligned}$ | $1.45 \mathrm{~mm} * 1.00 \mathrm{~mm}$ |
|  | DSF (X2SON, 6) | $\begin{aligned} & 1.00 \mathrm{~mm} * \\ & 1.00 \mathrm{~mm} \end{aligned}$ | $1.00 \mathrm{~mm} * 1.00 \mathrm{~mm}$ |
|  | YZP (DSBGA, 5) | $\begin{aligned} & 1.75 \mathrm{~mm} * \\ & 1.75 \mathrm{~mm} \end{aligned}$ | $1.39 \mathrm{~mm} * 0.89 \mathrm{~mm}$ |
|  | YZV (DSBGA, 4) | $\begin{aligned} & 1.25 \mathrm{~mm} * \\ & 1.25 \mathrm{~mm} \end{aligned}$ | $0.89 \mathrm{~mm} * 0.89 \mathrm{~mm}$ |
|  | DPW (X2SON, 5) | $\begin{aligned} & 0.80 \mathrm{~mm} * \\ & 0.80 \mathrm{~mm} \end{aligned}$ | $0.80 \mathrm{~mm} * 0.80 \mathrm{~mm}$ |

(1) For all available packages, see the orderable addendum at the end of the data sheet.
(2) The package size (length $\times$ width) is a nominal value and includes pins, where applicable.
(3) The body size (length $\times$ width) is a nominal value and does not include pins.
![img-1.jpeg](./images/img-1.jpeg)

Logic Diagram (Positive Logic)
(YZV Package)# Table of Contents 

1 Features ..... 1
2 Applications ..... 1
3 Description ..... 1
4 Pin Configuration and Functions ..... 3
5 Specifications ..... 5
5.1 Absolute Maximum Ratings ..... 5
5.2 ESD Ratings ..... 5
5.3 Recommended Operating Conditions ..... 6
5.4 Thermal Information ..... 6
5.5 Electrical Characteristics ..... 7
5.6 Switching Characteristics: $-40^{\circ} \mathrm{C}$ to $85^{\circ} \mathrm{C}$ ..... 9
5.7 Switching Characteristics: $-40^{\circ} \mathrm{C}$ to $125^{\circ} \mathrm{C}$ ..... 9
5.8 Operating Characteristics ..... 9
5.9 Typical Characteristics ..... 9
6 Detailed Description ..... 11
6.1 Overview ..... 11
6.2 Functional Block Diagrams ..... 11
6.3 Feature Description ..... 11
6.4 Device Functional Modes ..... 12
7 Application and Implementation ..... 13
7.1 Application Information ..... 13
7.2 Typical Application ..... 13
7.3 Power Supply Recommendations ..... 14
7.4 Layout ..... 14
8 Device and Documentation Support ..... 16
8.1 Documentation Support ..... 16
8.2 Receiving Notification of Documentation Updates ..... 16
8.3 Support Resources ..... 16
8.4 Trademarks ..... 16
8.5 Electrostatic Discharge Caution ..... 16
8.6 Glossary ..... 16
9 Revision History ..... 16
10 Mechanical, Packaging, and Orderable Information ..... 17# 4 Pin Configuration and Functions 

![img-2.jpeg](./images/img-2.jpeg)

Figure 4-1. DBV Package 5-Pin SOT-23 Top View
![img-3.jpeg](./images/img-3.jpeg)

Figure 4-2. DCK Package 5-Pin SC70 Top View
![img-4.jpeg](./images/img-4.jpeg)

Figure 4-3. DRL Package 5-Pin SOT-5X3 Top View
![img-5.jpeg](./images/img-5.jpeg)

Figure 4-4. DRY Package 6-Pin SON Top View
![img-6.jpeg](./images/img-6.jpeg)

Figure 4-5. DPW Package 5-Pin X2SON Top View
![img-7.jpeg](./images/img-7.jpeg)

See mechanical drawings for dimensions.
N.C. - No internal connection

Figure 4-6. DSF Package 6-Pin SON Top View![img-8.jpeg](./images/img-8.jpeg)

DNU - Do not use

Figure 4-7. YZP Package 5-Pin DSBGA Bottom View
![img-9.jpeg](./images/img-9.jpeg)

Figure 4-8. YZV Package 4-Pin DSBGA Bottom View
Table 4-1. Pin Functions

| PIN |  |  |  |  | I/O | DESCRIPTION |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| NAME | DBV, DCK, <br> DRL, DPW | DRY, DSF | YZP | YZV |  |  |
| A | 2 | 2 | B1 | A1 | I | Signal Input |
| GND | 3 | 3 | C1 | B1 | — | Ground |
| N.C. | 1 | 1,5 | - | - | - | No internal connection ${ }^{(1)}$ |
| DNU | - | - | A1 | - | - | Do not use ${ }^{(2)}$ |
| $\mathrm{V}_{\mathrm{CC}}$ | 5 | 6 | A2 | A2 | - | Positive Supply |
| Y | 4 | 4 | C2 | B2 | 0 | Signal Output |

(1) Pins labeled N.C. can be connected to any signal or voltage source, including ground. They should always be soldered to the board.
(2) Pins labeled DNU should not be connected to any signal or voltage source, including ground. They should always be soldered to the board.# 5 Specifications 

### 5.1 Absolute Maximum Ratings

over operating free-air temperature range (unless otherwise noted) ${ }^{(1)}$

|  |  |  | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{CC}}$ | Supply voltage |  | $-0.5$ | 6.5 | V |
| $\mathrm{V}_{\mathrm{I}}$ | Input voltage ${ }^{(2)}$ |  | $-0.5$ | 6.5 | V |
| $\mathrm{V}_{\mathrm{O}}$ | Voltage range applied to any output in the high-impedance or power-off state ${ }^{(2)}$ |  | $-0.5$ | 6.5 | V |
| $\mathrm{V}_{\mathrm{O}}$ | Voltage range applied to any output in the high or low state ${ }^{(2)(3)}$ |  | $-0.5$ | $\mathrm{V}_{\mathrm{CC}}+0.5$ | V |
| $\mathrm{I}_{\text {IK }}$ | Input clamp current | $V_{I}<0$ |  | $-50$ | mA |
| $\mathrm{I}_{\mathrm{OK}}$ | Output clamp current | $V_{O}<0$ |  | $-50$ | mA |
| $\mathrm{I}_{\mathrm{O}}$ | Continuous output current |  |  | $\pm 50$ | mA |
|  | Continuous current through $\mathrm{V}_{\mathrm{CC}}$ or GND |  |  | $\pm 100$ | mA |
| $T_{j}$ | Maximum junction temperature |  |  | 150 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\text {stg }}$ | Storage temperature |  | $-65$ | 150 | ${ }^{\circ} \mathrm{C}$ |

(1) Stresses beyond those listed under Absolute Maximum Ratings may cause permanent damage to the device. These are stress ratings only, which do not imply functional operation of the device at these or any other conditions beyond those indicated under Recommended Operating Conditions. Exposure to absolute-maximum-rated conditions for extended periods may affect device reliability.
(2) The input and output negative-voltage ratings may be exceeded if the input and output current ratings are observed.
(3) The value of $\mathrm{V}_{\mathrm{CC}}$ is provided in the recommended operating conditions table.

### 5.2 ESD Ratings

|  |  |  | VALUE | UNIT |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {(ESD) }}$ | Electrostatic discharge | Human-body model (HBM), per ANSI/ESDA/JEDEC JS-001 ${ }^{(1)}$ | 2000 | V |
|  |  | Charged-device model (CDM), per JEDEC specification JESD22-C101 ${ }^{(2)}$ | 1000 |  |
|  |  | Machine Model (A115-A) | 200 |  |

(1) JEDEC document JEP155 states that 500-V HBM allows safe manufacturing with a standard ESD control process.
(2) JEDEC document JEP157 states that 250-V CDM allows safe manufacturing with a standard ESD control process.# 5.3 Recommended Operating Conditions 

over operating free-air temperature range (unless otherwise noted) ${ }^{(1)}$

|  |  |  |  | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{CC}}$ | Supply voltage | Operating |  | 1.65 | 5.5 | V |
|  |  | Data retention only |  | 1.5 |  |  |
| $\mathrm{V}_{\mathrm{I}}$ | Input voltage |  |  | 0 | 5.5 | V |
| $\mathrm{V}_{\mathrm{O}}$ | Output voltage |  |  | 0 | $\mathrm{V}_{\mathrm{CC}}$ | V |
| $\mathrm{I}_{\mathrm{OH}}$ | High-level output current | $\mathrm{V}_{\mathrm{CC}}=1.65 \mathrm{~V}$ |  |  | $-4$ | mA |
|  |  | $\mathrm{V}_{\mathrm{CC}}=2.3 \mathrm{~V}$ |  |  | $-8$ |  |
|  |  | $\mathrm{V}_{\mathrm{CC}}=3 \mathrm{~V}$ |  |  | $-16$ |  |
|  |  |  |  |  | $-24$ |  |
|  |  | $\mathrm{V}_{\mathrm{CC}}=4.5 \mathrm{~V}$ |  |  | $-32$ |  |
| $\mathrm{I}_{\mathrm{OL}}$ | Low-level output current | $\mathrm{V}_{\mathrm{CC}}=1.65 \mathrm{~V}$ |  |  | 4 | mA |
|  |  | $\mathrm{V}_{\mathrm{CC}}=2.3 \mathrm{~V}$ |  |  | 8 |  |
|  |  | $\mathrm{V}_{\mathrm{CC}}=3 \mathrm{~V}$ |  |  | 16 |  |
|  |  |  |  |  | 24 |  |
|  |  | $\mathrm{V}_{\mathrm{CC}}=4.5 \mathrm{~V}$ |  |  | 32 |  |
| $\mathrm{T}_{\mathrm{A}}$ | Operating free-air temperature | YZP, YZV, and DPW packages |  |  | $-40$ | 85 | ${ }^{\circ} \mathrm{C}$ |

(1) All unused inputs of the device must be held at $\mathrm{V}_{\mathrm{CC}}$ or GND to assure proper device operation. See Implications of Slow or Floating CMOS Inputs.

### 5.4 Thermal Information

| THERMAL METRIC ${ }^{(1)}$ |  | SN74LVC1G14 |  |  |  |  |  | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | DBV <br> (SOT-23) | DCK <br> (SC70) | DRL <br> (SOT-5X3) | DRY <br> (SON) | DPW <br> (X2SON) | YZV <br> (DSBGA) |  |
|  |  | 5 PINS | 5 PINS | 5 PINS | 5 PINS | 5 PINS | 4 PINS | 5 PINS |
| $\mathrm{R}_{\text {SJA }}$ | Junction-to-ambient thermal resistance | 357.1 | 276.1 | 296.2 | 369.6 | 522.9 | 168.2 | 146.2 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{R}_{\text {SJC(top) }}$ | Junction-to-case (top) thermal resistance | 263.7 | 178.9 | 137.3 | 257.6 | 250.5 | 2.1 | 1.4 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{R}_{\text {SJB }}$ | Junction-to-board thermal resistance | 264.4 | 70.9 | 145.3 | 230.8 | 384.0 | 55.9 | 39.8 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\psi_{\text {JT }}$ | Junction-to-top characterization parameter | 195.6 | 47.0 | 14.7 | 77.2 | 46.5 | 1.1 | 0.7 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\psi_{\text {JB }}$ | Junction-to-board characterization parameter | 262.2 | 69.3 | 145.9 | 231.0 | 382.8 | 56.3 | 39.3 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{R}_{\text {SJC(bot) }}$ | Junction-to-case (bottom) thermal resistance | N/A | N/A | N/A | N/A | 174.1 | N/A | N/A | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |

(1) For more information about traditional and new thermal metrics, see the Semiconductor and IC Package Thermal Metrics application note.# 5.5 Electrical Characteristics 

over recommended operating free-air temperature range (unless otherwise noted)

| PARAMETE <br> R | TEST CONDITIONS | $\mathrm{V}_{\mathrm{CC}}$ | $-40^{\circ} \mathrm{C}$ to $85^{\circ} \mathrm{C}$ |  |  | $-40^{\circ} \mathrm{C}$ to $125^{\circ} \mathrm{C}^{(2)}$ |  |  | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | MIN | TYP ${ }^{(1)}$ | MAX | MIN | TYP | MAX |  |
| $\mathrm{V}_{\mathrm{T}}$ <br> Positivegoing input threshold voltage |  | 1.65 V | 0.79 |  | 1.16 | .79 |  | 1.16 | V |
|  |  | 2.3 V | 1.11 |  | 1.56 | 1.11 |  | 1.56 |  |
|  |  | 3 V | 1.5 |  | 1.87 | 1.5 |  | 1.87 |  |
|  |  | 4.5 V | 2.16 |  | 2.74 | 2.16 |  | 2.74 |  |
|  |  | 5.5 V | 2.61 |  | 3.33 | 2.61 |  | 3.33 |  |
| $\mathrm{V}_{\mathrm{T}}$ - <br> Negativegoing input threshold voltage | DBV, DCK, DRL, DRY, DSF, YZV and YZP packages | 1.65 V | 0.39 |  | 0.62 | .39 |  | .64 | V |
|  |  | 2.3 V | 0.58 |  | 0.87 | .58 |  | .89 |  |
|  |  | 3 V | 0.84 |  | 1.14 | .84 |  | 1.16 |  |
|  |  | 4.5 V | 1.41 |  | 1.79 | 1.41 |  | 1.79 |  |
|  |  | 5.5 V | 1.87 |  | 2.29 | 1.87 |  | 2.29 |  |
| $\mathrm{V}_{\mathrm{T}}$ - <br> Negativegoing input threshold voltage | DPW package | 1.65 V | 0.44 |  | 0.67 |  |  |  | V |
|  |  | 2.3 V | 0.63 |  | 0.92 |  |  |  |  |
|  |  | 3 V | 0.89 |  | 1.19 |  |  |  |  |
|  |  | 4.5 V | 1.46 |  | 1.84 |  |  |  |  |
|  |  | 5.5 V | 1.92 |  | 2.34 |  |  |  |  |
| $\Delta \mathrm{V}_{\mathrm{T}}$ <br> Hysteresis $\left(\mathrm{V}_{\mathrm{T}+}-\mathrm{V}_{\mathrm{T}-}\right)$ |  | 1.65 V | 0.37 |  | 0.62 | 0.37 |  | 0.62 | V |
|  |  | 2.3 V | 0.48 |  | 0.77 | 0.48 |  | 0.77 |  |
|  |  | 3 V | 0.56 |  | 0.87 | 0.56 |  | 0.87 |  |
|  |  | 4.5 V | 0.71 |  | 1.04 | 0.71 |  | 1.04 |  |
|  |  | 5.5 V | 0.71 |  | 1.11 | 0.71 |  | 1.11 |  |
| $\mathrm{V}_{\mathrm{OH}}$ | $\mathrm{I}_{\mathrm{OL}}=-100 \mu \mathrm{~A}$ | 1.65 V to <br> 4.5 V | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}- \\ 0.1 \end{gathered}$ |  |  | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}- \\ 0.1 \end{gathered}$ |  |  | V |
|  |  | $\mathrm{I}_{\mathrm{OL}}=-4 \mathrm{~mA}$ | 1.65 V | 1.2 |  | 1.2 |  |  |  |
|  |  | $\mathrm{I}_{\mathrm{OL}}=-8 \mathrm{~mA}$ | 2.3 V | 1.9 |  | 1.9 |  |  |  |
|  |  | $\mathrm{I}_{\mathrm{OL}}=-16 \mathrm{~mA}$ | 3 V | 2.4 |  | 2.4 |  |  |  |
|  |  | $\mathrm{I}_{\mathrm{OL}}=-24 \mathrm{~mA}$ |  | 2.3 |  | 2.3 |  |  |  |
|  |  | $\mathrm{I}_{\mathrm{OL}}=-32 \mathrm{~mA}$ | 4.5 V | 3.8 |  | 3.8 |  |  |  |
| $\mathrm{V}_{\mathrm{OL}}$ | $\mathrm{I}_{\mathrm{OL}}=100 \mu \mathrm{~A}$ | 1.65 V to <br> 4.5 V |  |  | 0.1 |  |  | 0.1 | V |
|  |  | $\mathrm{I}_{\mathrm{OL}}=4 \mathrm{~mA}$ | 1.65 V |  | 0.45 |  |  | 0.45 |  |
|  |  | $\mathrm{I}_{\mathrm{OL}}=8 \mathrm{~mA}$ | 2.3 V |  | 0.3 |  |  | 0.3 |  |
|  |  | $\mathrm{I}_{\mathrm{OL}}=16 \mathrm{~mA}$ | 3 V |  | 0.4 |  |  | 0.4 |  |
|  |  | $\mathrm{I}_{\mathrm{OL}}=24 \mathrm{~mA}$ |  |  | 0.55 |  |  | 0.55 |  |
|  |  | $\mathrm{I}_{\mathrm{OL}}=32 \mathrm{~mA}$ | 4.5 V |  | 0.55 |  |  | 0.7 |  |
| $\mathrm{I}_{\mathrm{I}}$ | A input | $\mathrm{V}_{\mathrm{I}}=5.5 \mathrm{~V}$ or GND | 0 to 5.5 <br> V |  | $\pm 5$ |  |  | $\pm 5$ | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {off }}$ |  | $\mathrm{V}_{\mathrm{I}}$ or $\mathrm{V}_{\mathrm{O}}=5.5 \mathrm{~V}$ | 0 |  | $\pm 10$ |  |  | $\pm 10$ | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\mathrm{CC}}$ |  | $\mathrm{V}_{\mathrm{I}}=5.5 \mathrm{~V}$ or GND, $\mathrm{I}_{\mathrm{O}}=0$ | 1.65 V to <br> 5.5 V |  | 10 |  |  | 10 | $\mu \mathrm{A}$ |
| $\Delta \mathrm{I}_{\mathrm{CC}}$ |  | One input at Other inputs at $\mathrm{V}_{\mathrm{CC}}$ $\mathrm{V}_{\mathrm{CC}}-0.6 \mathrm{~V}$, or GND | 3 V to <br> 5.5 V |  | 500 |  |  | 500 | $\mu \mathrm{A}$ |
| $\mathrm{C}_{\mathrm{i}}$ |  | $\mathrm{V}_{\mathrm{I}}=\mathrm{V}_{\mathrm{CC}}$ or GND | 3.3 V |  | 4.5 |  |  | 4.5 |  |

(1) All typical values are at $\mathrm{V}_{\mathrm{CC}}=3.3 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=25^{\circ} \mathrm{C}$.(2) These specifications do not apply to DPW, YZV and YZP packages. DPW, YZV and YZP have a recommended operating free-air temperature range of $-40^{\circ} \mathrm{C}$ to $85^{\circ} \mathrm{C}$.# 5.6 Switching Characteristics: $-40^{\circ} \mathrm{C}$ to $85^{\circ} \mathrm{C}$ 

over recommended operating free-air temperature range, $\left(-40^{\circ} \mathrm{C}\right.$ to $85^{\circ} \mathrm{C}$ unless otherwise noted)

| PARAMETER | FROM <br> (INPUT) | TO <br> (OUTPUT) | $\mathrm{V}_{\mathrm{CC}}$ | $\mathrm{C}_{\mathrm{L}}=15 \mathrm{pF}$ |  | $\mathrm{C}_{\mathrm{L}}=30 \mathrm{pF}$ or 50 pF |  | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | MIN | MAX | MIN | MAX |  |
| $t_{p d}$ | A | $Y$ | $1.8 \mathrm{~V} \pm 0.15 \mathrm{~V}$ | 2.8 | 9.9 | 3.8 | 11 | ns |
|  |  |  | $2.5 \mathrm{~V} \pm 0.2 \mathrm{~V}$ | 1.6 | 5.5 | 2 | 6.5 |  |
|  |  |  | $3.3 \mathrm{~V} \pm 0.3 \mathrm{~V}$ | 1.5 | 4.6 | 1.8 | 5.5 |  |
|  |  |  | $5 \mathrm{~V} \pm 0.5 \mathrm{~V}$ | 0.9 | 4.4 | 1.2 | 5 |  |

5.7 Switching Characteristics: $-40^{\circ} \mathrm{C}$ to $125^{\circ} \mathrm{C}$
over operating free-air temperature range, $\left(-40^{\circ} \mathrm{C}\right.$ to $125^{\circ} \mathrm{C}$ unless otherwise noted)

| PARAMETER | FROM <br> (INPUT) | TO <br> (OUTPUT) | $\mathrm{V}_{\mathrm{CC}}$ | $\mathrm{C}_{\mathrm{L}}=30 \mathrm{pF}$ <br> or 50 pF |  | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | MIN | MAX |  |
| $t_{p d}$ | A | $Y$ | $1.8 \mathrm{~V} \pm 0.15 \mathrm{~V}$ | 3.8 | 13 | ns |
|  |  |  | $2.5 \mathrm{~V} \pm 0.2 \mathrm{~V}$ | 2 | 8 |  |
|  |  |  | $3.3 \mathrm{~V} \pm 0.3 \mathrm{~V}$ | 1.8 | 6.5 |  |
|  |  |  | $5 \mathrm{~V} \pm 0.5 \mathrm{~V}$ | 1.2 | 6 |  |

### 5.8 Operating Characteristics

$T_{A}=25^{\circ} \mathrm{C}$

| PARAMETER | TEST CONDITIONS | $\mathrm{V}_{\mathrm{CC}}$ | TYP | UNIT |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{C}_{\mathrm{pd}}$ Power dissipation capacitance | $\mathrm{f}=10 \mathrm{MHz}$ | 1.8 V | 20 | pF |
|  |  | 2.5 V | 21 |  |
|  |  | 3.3 V | 22 |  |
|  |  | 5 V | 25 |  |

### 5.9 Typical Characteristics

$T_{A}=25^{\circ} \mathrm{C}$
![img-10.jpeg](./images/img-10.jpeg)# Parameter Measurement Information 

- Input pulse is supplied by generator having the following characteristics: $\mathrm{PRR} \leq 10 \mathrm{MHz} . \mathrm{Z}_{\mathrm{O}}=50 \Omega$.
- The outputs are measured one at a time, with one transition per measurement.
![img-11.jpeg](./images/img-11.jpeg)
A. $\quad C_{L}$ includes probe and jig capacitance.

Figure 6-1. Load Circuit
Table 6-1. Parameter Measurement Conditions

| $\mathbf{V}_{\mathbf{c c}}$ | INPUTS |  | $\mathbf{V}_{\mathbf{M}}$ | $\mathbf{V}_{\text {LOAD }}$ | $\mathbf{C}_{\mathbf{L}}$ | $\mathbf{R}_{\mathbf{L}}$ | $\mathbf{V}_{\mathbf{D}}$ |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | $\mathbf{V}_{\mathbf{I}}$ | $\mathbf{t}_{\mathbf{r}} / \mathbf{t}_{\mathbf{f}}$ |  |  |  |  |  |
| $1.8 \mathrm{~V} \pm 0.15 \mathrm{~V}$ | $\mathbf{V}_{\mathbf{c c}}$ | $\leq 2 \mathrm{~ns}$ | $\mathrm{V}_{\mathrm{cc}} / 2$ | $2 \times \mathrm{V}_{\mathrm{cc}}$ | 15 pF | $1 \mathrm{M} \Omega$ | 0.15 V |
|  |  |  |  |  | 30 pF | $1 \mathrm{k} \Omega$ |  |
| $2.5 \mathrm{~V} \pm 0.2 \mathrm{~V}$ | $\mathbf{V}_{\mathbf{c c}}$ | $\leq 2 \mathrm{~ns}$ | $\mathrm{V}_{\mathrm{cc}} / 2$ | $2 \times \mathrm{V}_{\mathrm{cc}}$ | 15 pF | $1 \mathrm{M} \Omega$ | 0.15 V |
|  |  |  |  |  | 30 pF | $500 \Omega$ |  |
| $3.3 \mathrm{~V} \pm 0.3 \mathrm{~V}$ | 3 V | $\leq 2.5 \mathrm{~ns}$ | 1.5 V | 6 V | 15 pF | $1 \mathrm{M} \Omega$ | 0.3 V |
|  |  |  |  |  | 50 pF | $500 \Omega$ |  |
| $5 \mathrm{~V} \pm 0.5 \mathrm{~V}$ | $\mathbf{V}_{\mathbf{c c}}$ | $\leq 2.5 \mathrm{~ns}$ | $\mathrm{V}_{\mathrm{cc}} / 2$ | $2 \times \mathrm{V}_{\mathrm{cc}}$ | 15 pF | $1 \mathrm{M} \Omega$ | 0.3 V |
|  |  |  |  |  | 50 pF | $500 \Omega$ |  |

![img-12.jpeg](./images/img-12.jpeg)
A. The maximum value of $t_{p d}$ is the worst case of $t_{\mathrm{PLH}}$ or $t_{\mathrm{PHL}}$

Figure 6-2. Voltage Waveforms, Propagation Delay Times, Inverting and Non-Inverting Outputs# 6 Detailed Description 

### 6.1 Overview

The SN74LVC1G14 single Schmitt-trigger inverter is designed for 1.65 V to 5.5 V operation and performs the Boolean function $Y=\bar{A}$. This device is fully specified for partial-power-down applications using $I_{\text {off }}$. The $I_{\text {off }}$ circuitry disables the outputs when the device is powered down. This inhibits current backflow into the device which prevents damage to the device.

### 6.2 Functional Block Diagrams

![img-13.jpeg](./images/img-13.jpeg)

Figure 6-1. Logic Diagram (Positive Logic)
(DBV, DCK, DRL, DRY, DPW, and YZP Package)
![img-14.jpeg](./images/img-14.jpeg)

Figure 6-2. Logic Diagram (Positive Logic) (YZV Package)

### 6.3 Feature Description

### 6.3.1 Balanced High-Drive CMOS Push-Pull Outputs

A balanced output allows the device to sink and source similar currents. The high drive capability of this device creates fast edges into light loads so routing and load conditions should be considered to prevent ringing. Additionally, the outputs of this device are capable of driving larger currents than the device can sustain without being damaged. It is important for the power output of the device to be limited to avoid thermal runaway and damage due to over-current. The electrical and thermal limits defined the in the Absolute Maximum Ratings
Absolute Maximum Ratings must be followed at all times.

### 6.3.2 CMOS Schmitt-Trigger Inputs

Standard CMOS inputs are high impedance and are typically modeled as a resistor in parallel with the input capacitance given in the Electrical Characteristics. The worst case resistance is calculated with the maximum input voltage, given in the Absolute Maximum Ratings, and the maximum input leakage current, given in the Electrical Characteristics, using ohm's law $(R=V+I)$.
The Schmitt-trigger input architecture provides hysteresis as define in the Electrical Characteristics, which makes this device extremely tolerant to slow or noisy inputs. While the inputs can be driven much slower than standard CMOS inputs, it is still recommended to properly terminate unused inputs. Driving the inputs slowly will also increase dynamic current consumption of the device.# 6.3.3 Clamp Diodes 

The inputs and outputs to this device have negative clamping diodes.

## CAUTION

Voltages beyond the values specified in the Absolute Maximum Ratings table can cause damage to the device. The input negative-voltage and output negative-voltage ratings may be exceeded if the input and output clamp-current ratings are observed.
![img-15.jpeg](./images/img-15.jpeg)

Figure 6-3. Electrical Placement of Clamping Diodes for Each Input and Output

### 6.3.4 Partial Power Down ( $I_{\text {off }}$ )

The inputs and outputs for this device enter a high impedance state when the supply voltage is 0 V . The maximum leakage into or out of any input or output pin on the device is specified by $I_{\text {off }}$ in the Electrical Characteristics.

### 6.3.5 Over-Voltage Tolerant Inputs

Input signals to this device can be driven above the supply voltage so long as they remain below the maximum input voltage value specified in the Absolute Maximum Ratings.

### 6.4 Device Functional Modes

Table 6-1 lists the functional modes of the SN74LVC1G14 device.
Table 6-1. Function Table

| INPUT <br> A | OUTPUT <br> Y |
| :--: | :--: |
| H | L |
| L | H |# 7 Application and Implementation 

## Note

Information in the following applications sections is not part of the TI component specification, and TI does not warrant its accuracy or completeness. TI's customers are responsible for determining suitability of components for their purposes, as well as validating and testing their design implementation to confirm system functionality.

### 7.1 Application Information

Mechanical input elements, such as push buttons or rotary knobs, offer simple ways to interact with electronic systems. Typically, these elements have recoil or bouncing, where the mechanical element makes and breaks contact multiple times during human interaction. This bouncing can cause one or more repeated signals to be passed, triggering multiple actions when only a single input was intended. One potential solution to mitigating these multiple inputs is by utilizing a Schmitt-trigger to create a debounce circuit. Figure 7-1 shows an example of this solution.

### 7.2 Typical Application

The input due to the push button switches multiple times, causing the output of a non Schmitt-trigger device to trigger multiple times, while the Schmitt-trigger input device with RC delay limits the output pulse to a single pulse desired by the user. The separated positive and negative input voltage threshold values, see Figure 7-2, prevent multiple triggers from occurring.
![img-16.jpeg](./images/img-16.jpeg)

Figure 7-1. Push Button Debounce Circuit Schematic

### 7.2.1 Design Requirements

This device uses CMOS technology and has balanced output drive. Take care to avoid bus contention because it can drive currents that would exceed maximum limits. The high drive also creates fast edges into light loads so routing and load conditions should be considered to prevent ringing.

### 7.2.2 Detailed Design Procedure

1. Recommended Input Conditions:

- For specified high and low levels, see $\left(\mathrm{V}_{\mathrm{T}+}\right.$ and $\left.\mathrm{V}_{\mathrm{T}-}\right)$ in the Recommended Operating Conditions table.
- Inputs are overvoltage tolerant allowing them to go as high as ( $\mathrm{V}_{\mathrm{I}}$ max) in the Recommended Operating Conditions table at any valid $\mathrm{V}_{\mathrm{CC}}$.

2. Recommended Output Conditions:

- Load currents should not exceed ( $I_{O}$ max) per output and should not exceed (Continuous current through $\mathrm{V}_{\mathrm{CC}}$ or GND) total current for the part. These limits are located in the Absolute Maximum Ratings table.# 7.2.3 Application Curve 

Figure 7-2 is created from the values given in the Electrical Characteristics. Linear interpolation shows the values between each given point.
![img-17.jpeg](./images/img-17.jpeg)

Figure 7-2. Interpolated Threshold Voltages vs. $\mathbf{V}_{\mathbf{C C}}$

### 7.3 Power Supply Recommendations

The power supply can be any voltage between the minimum and maximum supply voltage rating located in the Recommended Operating Conditions table.
The $\mathrm{V}_{\mathrm{CC}}$ pin must have a good bypass capacitor to prevent power disturbance. A $0.1-\mu \mathrm{F}$ capacitor is recommended, and it is ok to parallel multiple bypass caps to reject different frequencies of noise. $0.1-\mu \mathrm{F}$ and $1-\mu \mathrm{F}$ capacitors are commonly used in parallel. The bypass capacitor must be installed as close to the power pin as possible for best results.

### 7.4 Layout

### 7.4.1 Layout Guidelines

Even low data rate digital signals can contain high-frequency signal components due to fast edge rates. When a printed-circuit board (PCB) trace turns a corner at a $90^{\circ}$ angle, a reflection can occur. A reflection occurs primarily because of the change of width of the trace. At the apex of the turn, the trace width increases to 1.414 times the width. This increase upsets the transmission-line characteristics, especially the distributed capacitance and self-inductance of the trace which results in the reflection. Not all PCB traces can be straight and therefore some traces must turn corners.

An example layout is given in Figure 7-3 for the DPW (X2SON-5) package. This example layout includes a 0402 (metric) capacitor and uses the measurements found in the example board layout appended to this end of this datasheet. A via of diameter 0.1 mm ( 3.973 mil ) is placed directly in the center of the device. This via can be used to trace out the center pin connection through another board layer, or it can be left out of the layout# 7.4.2 Layout Example 

![img-18.jpeg](./images/img-18.jpeg)

Figure 7-3. Example Layout With DPW (X2SON-5) Package# 8 Device and Documentation Support 

### 8.1 Documentation Support

### 8.1.1 Related Documentation

For related documentation see the following:

- Texas Instruments, Implications of Slow or Floating CMOS Inputs application note


### 8.2 Receiving Notification of Documentation Updates

To receive notification of documentation updates, navigate to the device product folder on ti.com. Click on Notifications to register and receive a weekly digest of any product information that has changed. For change details, review the revision history included in any revised document.

### 8.3 Support Resources

TI E2E ${ }^{\text {TM }}$ support forums are an engineer's go-to source for fast, verified answers and design help - straight from the experts. Search existing answers or ask your own question to get the quick design help you need.
Linked content is provided "AS IS" by the respective contributors. They do not constitute TI specifications and do not necessarily reflect TI's views; see TI's Terms of Use.

### 8.4 Trademarks

NanoFree ${ }^{\text {TM }}$ and TI E2E ${ }^{\text {TM }}$ are trademarks of Texas Instruments.
All trademarks are the property of their respective owners.

### 8.5 Electrostatic Discharge Caution

This integrated circuit can be damaged by ESD. Texas Instruments recommends that all integrated circuits be handled with appropriate precautions. Failure to observe proper handling and installation procedures can cause damage.
ESD damage can range from subtle performance degradation to complete device failure. Precision integrated circuits may be more susceptible to damage because very small parametric changes could cause the device not to meet its published specifications.

### 8.6 Glossary

TI Glossary This glossary lists and explains terms, acronyms, and definitions.

## 9 Revision History

NOTE: Page numbers for previous revisions may differ from page numbers in the current version.
Changes from Revision Y (November 2018) to Revision Z (June 2025)
Page

- Updated the numbering format for tables, figures, and cross-references throughout the document................. 1
- Changed Device Information table to Package Information ................................................................................. 1
- Changed Junction-to-ambient thermal resistance value for DBV package from: $247.2^{\circ} \mathrm{C} / \mathrm{W}$ to: $357.1^{\circ} \mathrm{C} / \mathrm{W}$..... 6
- Changed Junction-to-case (top) thermal resistance value for DBV package from: $154.5^{\circ} \mathrm{C} / \mathrm{W}$ to: $263.7^{\circ} \mathrm{C} / \mathrm{W}$... 6
- Changed Junction-to-board thermal resistance value for DBV package from: $86.8^{\circ} \mathrm{C} / \mathrm{W}$ to: $264.4^{\circ} \mathrm{C} / \mathrm{W}$............ 6
- Changed Junction-to-top characterization value for DBV package from: $58.0^{\circ} \mathrm{C} / \mathrm{W}$ to: $195.6^{\circ} \mathrm{C} / \mathrm{W}$.................. 6
- Changed Junction-to-board characterization value for DBV package from: $86.4^{\circ} \mathrm{C} / \mathrm{W}$ to: $262.2^{\circ} \mathrm{C} / \mathrm{W}$............... 6

Changes from Revision X (August 2017) to Revision Y (November 2018)
Page

- Changed New package pinout added to Pin Functions table. Multiple Pin Functions tables condensed to one............................................................................................................................................................................... 3
- Changed $T_{j}$ and $T_{\text {stg }}$ lines switched for consistency with other devices............................................................ 5
- Added differentiated ROC temperatures for DPW, YZP and YZV packages .................................................... 6
- Changed format of Switching Characteristics tables to include columns for different $\mathrm{C}_{\mathrm{L}}$ conditions ................ 9
- Added temperature range to Conditions statement for Switching Characteristics tables ................................ 9- Replaced PMI section with updated load circuit and relevant waveform figures. Collapsed parameter measurement values into one table. 10


# 10 Mechanical, Packaging, and Orderable Information 

The following pages include mechanical packaging and orderable information. This information is the most current data available for the designated devices. This data is subject to change without notice and revision of this document. For browser-based versions of this data sheet, refer to the left-hand navigation.# PACKAGE OPTION ADDENDUM

|  Orderable part number | Status
(1) | Material type
(2) | Package | Pins | Package qty | Carrier | RoHS
(3) | Lead finish/
Ball material
(4) | MSL rating/
Peak reflow
(5) | Op temp ( ${ }^{\circ} \mathrm{C}$ ) | Part marking
(6)  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  SN74LVC1G14DBVR | Active | Production | SOT-23 (DBV) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | SN | Level-1-260C-UNLIM | $-40$ to 125  |
|  SN74LVC1G14DBVR.A | Active | Production | SOT-23 (DBV) | 5 | 3000 | LARGE T\&R | Yes | SN | Level-1-260C-UNLIM | $-40$ to 125 | (C145, C14F, C14J,
C14K, C14R)
(C14H, C14S)  |
|  SN74LVC1G14DBVR.B | Active | Production | SOT-23 (DBV) | 5 | 3000 | LARGE T\&R | Yes | SN | Level-1-260C-UNLIM | $-40$ to 125 | (C145, C14F, C14J,
C14K, C14R)
(C14H, C14S)  |
|  SN74LVC1G14DBVRE4 | Active | Production | SOT-23 (DBV) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | C14F  |
|  SN74LVC1G14DBVRG4 | Active | Production | SOT-23 (DBV) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | C14F  |
|  SN74LVC1G14DBVRG4.A | Active | Production | SOT-23 (DBV) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | C14F  |
|  SN74LVC1G14DBVRG4.B | Active | Production | SOT-23 (DBV) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | C14F  |
|  SN74LVC1G14DBVT | Active | Production | SOT-23 (DBV) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | SN | NIPDAU Level-1-260C-UNLIM | $-40$ to 125  |
|  SN74LVC1G14DBVT.B | Active | Production | SOT-23 (DBV) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | (C145, C14F, C14J,
C14K, C14R)
(C14H, C14S)  |
|  SN74LVC1G14DBVTE4 | Active | Production | SOT-23 (DBV) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | C14F  |
|  SN74LVC1G14DBVTG4 | Active | Production | SOT-23 (DBV) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | C14F  |
|  SN74LVC1G14DBVTG4.B | Active | Production | SOT-23 (DBV) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | C14F  |
|  SN74LVC1G14DCKR | Active | Production | SC70 (DCK) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | SN | NIPDAU Level-1-260C-UNLIM | $-40$ to 125  |
|  SN74LVC1G14DCKR.A | Active | Production | SC70 (DCK) | 5 | 3000 | LARGE T\&R | Yes | SN | Level-1-260C-UNLIM | $-40$ to 125 | (CF5, CFF, CFJ, CF
K, CFR, CFT)
(CFH, CFS)  |
|  SN74LVC1G14DCKR.B | Active | Production | SC70 (DCK) | 5 | 3000 | LARGE T\&R | Yes | SN | Level-1-260C-UNLIM | $-40$ to 125 | (CF5, CFF, CFJ, CF
K, CFR, CFT)
(CFH, CFS)  |
|  SN74LVC1G14DCKRE4 | Active | Production | SC70 (DCK) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF5
CFS  ||  Orderable part number | Status
(1) | Material type
(2) | Package | Pins | Package qty | Carrier | RoHS
(3) | Lead finish/
Ball material
(4) | MSL rating/
Peak reflow
(5) | Op temp ( ${ }^{\circ} \mathrm{C}$ ) | Part marking
(6)  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  SN74LVC1G14DCKRG4.A | Active | Production | SC70 (DCK) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF5
CF5  |
|  SN74LVC1G14DCKRG4.B | Active | Production | SC70 (DCK) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF5
CF5  |
|  SN74LVC1G14DCKT | Active | Production | SC70 (DCK) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | SN | NIPDAU | Level-1-260C-UNLIM  |
|  SN74LVC1G14DCKT.B | Active | Production | SC70 (DCK) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | (CF5, CFF, CFJ, CF
K, CFR, CFT)
(CFH, CFS)  |
|  SN74LVC1G14DCKTE4 | Active | Production | SC70 (DCK) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF5
CF5  |
|  SN74LVC1G14DCKTG4 | Active | Production | SC70 (DCK) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF5
CF5  |
|  SN74LVC1G14DCKTG4.B | Active | Production | SC70 (DCK) | 5 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF5
CF5  |
|  SN74LVC1G14DPWR | Active | Production | X2SON (DPW) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 9 H  |
|  SN74LVC1G14DPWR.B | Active | Production | X2SON (DPW) | 5 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 9 H  |
|  SN74LVC1G14DRLR | Active | Production | SOT-5X3 (DRL) | 5 | 4000 | LARGE T\&R | Yes | NIPDAUAG | Level-1-260C-UNLIM | $-40$ to 125 | (CF7, CFR)  |
|  SN74LVC1G14DRLR.B | Active | Production | SOT-5X3 (DRL) | 5 | 4000 | LARGE T\&R | Yes | NIPDAUAG | Level-1-260C-UNLIM | $-40$ to 125 | (CF7, CFR)  |
|  SN74LVC1G14DRLRG4 | Active | Production | SOT-5X3 (DRL) | 5 | 4000 | LARGE T\&R | Yes | NIPDAUAG | Level-1-260C-UNLIM | $-40$ to 125 | (CF7, CFR)  |
|  SN74LVC1G14DRYR | Active | Production | SON (DRY) | 6 | 5000 | LARGE T\&R | Yes | NIPDAU | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125  |
|  SN74LVC1G14DRYR.B | Active | Production | SON (DRY) | 6 | 5000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF  |
|  SN74LVC1G14DRYRG4.B | Active | Production | SON (DRY) | 6 | 5000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF  |
|  SN74LVC1G14DSFR | Active | Production | SON (DSF) | 6 | 5000 | LARGE T\&R | Yes | NIPDAU | NIPDAUAG | Level-1-260C-UNLIM | $-40$ to 125  |
|  SN74LVC1G14DSFR.B | Active | Production | SON (DSF) | 6 | 5000 | LARGE T\&R | Yes | NIPDAUAG | Level-1-260C-UNLIM | $-40$ to 125 | CF  |
|  SN74LVC1G14DSFRG4 | Active | Production | SON (DSF) | 6 | 5000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF  |
|  SN74LVC1G14DSFRG4.B | Active | Production | SON (DSF) | 6 | 5000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | CF  |
|  SN74LVC1G14YZPR | Active | Production | DSBGA (YZP) | 5 | 3000 | LARGE T\&R | Yes | SNAGCU | Level-1-260C-UNLIM | $-40$ to 85 | (CF7, CFN)  |
|  SN74LVC1G14YZPR.B | Active | Production | DSBGA (YZP) | 5 | 3000 | LARGE T\&R | Yes | SNAGCU | Level-1-260C-UNLIM | $-40$ to 85 | (CF7, CFN)  |
|  SN74LVC1G14YZVR | Active | Production | DSBGA (YZV) | 4 | 3000 | LARGE T\&R | Yes | SNAGCU | Level-1-260C-UNLIM | $-40$ to 85 | CF
(7, N)  |
|  SN74LVC1G14YZVR.B | Active | Production | DSBGA (YZV) | 4 | 3000 | LARGE T\&R | Yes | SNAGCU | Level-1-260C-UNLIM | $-40$ to 85 | CF
(7, N)  |# PACKAGE OPTION ADDENDUM 

(1) Status: For more details on status, see our product life cycle.
${ }^{(2)}$ Material type: When designated, preproduction parts are prototypes/experimental devices, and are not yet approved or released for full production. Testing and final process, including without limitation quality assurance, reliability performance testing, and/or process qualification, may not yet be complete, and this item is subject to further changes or possible discontinuation. If available for ordering, purchases will be subject to an additional waiver at checkout, and are intended for early internal evaluation purposes only. These items are sold without warranties of any kind.
${ }^{(3)}$ RoHS values: Yes, No, RoHS Exempt. See the TI RoHS Statement for additional information and value definition.
${ }^{(4)}$ Lead finish/Ball material: Parts may have multiple material finish options. Finish options are separated by a vertical ruled line. Lead finish/Ball material values may wrap to two lines if the finish value exceeds the maximum column width.
${ }^{(5)}$ MSL rating/Peak reflow: The moisture sensitivity level ratings and peak solder (reflow) temperatures. In the event that a part has multiple moisture sensitivity ratings, only the lowest level per JEDEC standards is shown. Refer to the shipping label for the actual reflow temperature that will be used to mount the part to the printed circuit board.
${ }^{(6)}$ Part marking: There may be an additional marking, which relates to the logo, the lot trace code information, or the environmental category of the part.

Multiple part markings will be inside parentheses. Only one part marking contained in parentheses and separated by a "-" will appear on a part. If a line is indented then it is a continuation of the previous line and the two combined represent the entire part marking for that device.

Important Information and Disclaimer:The information provided on this page represents TI's knowledge and belief as of the date that it is provided. TI bases its knowledge and belief on information provided by third parties, and makes no representation or warranty as to the accuracy of such information. Efforts are underway to better integrate information from third parties. TI has taken and continues to take reasonable steps to provide representative and accurate information but may not have conducted destructive testing or chemical analysis on incoming materials and chemicals. TI and TI suppliers consider certain information to be proprietary, and thus CAS numbers and other limited information may not be available for release.

In no event shall TI's liability arising out of such information exceed the total purchase price of the TI part(s) at issue in this document sold by TI to Customer on an annual basis.

## OTHER QUALIFIED VERSIONS OF SN74LVC1G14 :

- Automotive : SN74LVC1G14-Q1
- Enhanced Product : SN74LVC1G14-EP

NOTE: Qualified Version Definitions:

- Automotive - Q100 devices qualified for high-reliability automotive applications targeting zero defects
- Enhanced Product - Supports Defense, Aerospace and Medical Applications# TAPE AND REEL INFORMATION 

![img-19.jpeg](./images/img-19.jpeg)

TAPE DIMENSIONS
![img-20.jpeg](./images/img-20.jpeg)

| A0 | Dimension designed to accommodate the component width |
| :-- | :-- |
| B0 | Dimension designed to accommodate the component length |
| K0 | Dimension designed to accommodate the component thickness |
| W | Overall width of the carrier tape |
| P1 | Pitch between successive cavity centers |

QUADRANT ASSIGNMENTS FOR PIN 1 ORIENTATION IN TAPE
![img-21.jpeg](./images/img-21.jpeg)
*All dimensions are nominal

| Device | Package <br> Type | Package <br> Drawing | Pins | SPQ | Reel <br> Diameter <br> (mm) | Reel <br> Width <br> W1 (mm) | A0 <br> (mm) | B0 <br> (mm) | K0 <br> (mm) | P1 <br> (mm) | W <br> (mm) | Pin1 <br> Quadrant |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 178.0 | 8.4 | 3.2 | 3.2 | 1.4 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 180.0 | 8.4 | 3.2 | 3.2 | 1.4 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DBVRG4 | SOT-23 | DBV | 5 | 3000 | 178.0 | 9.0 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DBVT | SOT-23 | DBV | 5 | 250 | 178.0 | 9.0 | 3.3 | 3.2 | 1.4 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DBVTG4 | SOT-23 | DBV | 5 | 250 | 178.0 | 9.0 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DCKR | SC70 | DCK | 5 | 3000 | 180.0 | 8.4 | 2.3 | 2.5 | 1.2 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DCKT | SC70 | DCK | 5 | 250 | 178.0 | 9.0 | 2.4 | 2.5 | 1.2 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DCKTG4 | SC70 | DCK | 5 | 250 | 178.0 | 9.2 | 2.4 | 2.4 | 1.22 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DPWR | X2SON | DPW | 5 | 3000 | 178.0 | 8.4 | 0.91 | 0.91 | 0.5 | 2.0 | 8.0 | Q3 |
| SN74LVC1G14DRLR | SOT-5X3 | DRL | 5 | 4000 | 180.0 | 8.4 | 1.98 | 1.78 | 0.69 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DRYR | SON | DRY | 6 | 5000 | 180.0 | 9.5 | 1.2 | 1.65 | 0.7 | 4.0 | 8.0 | Q1 |
| SN74LVC1G14DSFR | SON | DSF | 6 | 5000 | 180.0 | 9.5 | 1.16 | 1.16 | 0.5 | 4.0 | 8.0 | Q2 |
| SN74LVC1G14DSFRG4 | SON | DSF | 6 | 5000 | 180.0 | 9.5 | 1.16 | 1.16 | 0.5 | 4.0 | 8.0 | Q2 |
| SN74LVC1G14YZPR | DSBGA | YZP | 5 | 3000 | 178.0 | 9.2 | 1.02 | 1.52 | 0.63 | 4.0 | 8.0 | Q1 |
| SN74LVC1G14YZVR | DSBGA | YZV | 4 | 3000 | 178.0 | 9.2 | 1.0 | 1.0 | 0.63 | 4.0 | 8.0 | Q1 |# **PACKAGE MATERIALS INFORMATION**

![img-22.jpeg](./images/img-22.jpeg)

|  *All dimensions are nominal |  |  |  |  |  |  |   |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  Device | Package Type | Package Drawing | Pins | SPQ | Length (mm) | Width (mm) | Height (mm)  |
|  SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 208.0 | 191.0 | 35.0  |
|  SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 210.0 | 185.0 | 35.0  |
|  SN74LVC1G14DBVRG4 | SOT-23 | DBV | 5 | 3000 | 180.0 | 180.0 | 18.0  |
|  SN74LVC1G14DBVT | SOT-23 | DBV | 5 | 250 | 180.0 | 180.0 | 18.0  |
|  SN74LVC1G14DBVTG4 | SOT-23 | DBV | 5 | 250 | 180.0 | 180.0 | 18.0  |
|  SN74LVC1G14DCKR | SC70 | DCK | 5 | 3000 | 210.0 | 185.0 | 35.0  |
|  SN74LVC1G14DCKT | SC70 | DCK | 5 | 250 | 180.0 | 180.0 | 18.0  |
|  SN74LVC1G14DCKTG4 | SC70 | DCK | 5 | 250 | 180.0 | 180.0 | 18.0  |
|  SN74LVC1G14DPWR | X2SON | DPW | 5 | 3000 | 205.0 | 200.0 | 33.0  |
|  SN74LVC1G14DRLR | SOT-5X3 | DRL | 5 | 4000 | 202.0 | 201.0 | 28.0  |
|  SN74LVC1G14DRYR | SON | DRY | 6 | 5000 | 189.0 | 185.0 | 36.0  |
|  SN74LVC1G14DSFR | SON | DSF | 6 | 5000 | 184.0 | 184.0 | 19.0  |
|  SN74LVC1G14DSFRG4 | SON | DSF | 6 | 5000 | 184.0 | 184.0 | 19.0  |
|  SN74LVC1G14YZPR | DSBGA | YZP | 5 | 3000 | 220.0 | 220.0 | 35.0  |
|  SN74LVC1G14YZVR | DSBGA | YZV | 4 | 3000 | 220.0 | 220.0 | 35.0  |![img-23.jpeg](./images/img-23.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.
3. Reference JEDEC MO-178.
4. Body dimensions do not include mold flash, protrusions, or gate burrs. Mold flash, protrusions, or gate burrs shall not exceed 0.25 mm per side.
5. Support pin may differ or may not be present.![img-24.jpeg](./images/img-24.jpeg)

NOTES: (continued)
6. Publication IPC-7351 may have alternate designs.
7. Solder mask tolerances between and around signal pads can vary based on board fabrication site.![img-25.jpeg](./images/img-25.jpeg)

NOTES: (continued)
8. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.
9. Board assembly site may have different recommendations for stencil design.Images above are just a representation of the package family, actual package may vary. Refer to the product data sheet for package details.![img-26.jpeg](./images/img-26.jpeg)
![img-27.jpeg](./images/img-27.jpeg)
![img-28.jpeg](./images/img-28.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.![img-29.jpeg](./images/img-29.jpeg)

NOTES: (continued)
3. For more information, see QFN/SON PCB application report in literature No. SLUA271 (www.ti.com/lit/slua271).![img-30.jpeg](./images/img-30.jpeg)

SOLDER PASTE EXAMPLE
BASED ON 0.075 - 0.1 mm THICK STENCIL SCALE:40X
NOTES: (continued)
4. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.![img-31.jpeg](./images/img-31.jpeg)
![img-32.jpeg](./images/img-32.jpeg)
![img-33.jpeg](./images/img-33.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.![img-34.jpeg](./images/img-34.jpeg)

NOTES: (continued)
3. For more information, see QFN/SON PCB application report in literature No. SLUA271 (www.ti.com/lit/slua271).![img-35.jpeg](./images/img-35.jpeg)

SOLDER PASTE EXAMPLE
BASED ON 0.075 - 0.1 mm THICK STENCIL SCALE:40X
NOTES: (continued)
4. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.![img-36.jpeg](./images/img-36.jpeg)
![img-37.jpeg](./images/img-37.jpeg)
![img-38.jpeg](./images/img-38.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.
3. Reference JEDEC registration MO-287, variation X2AAF.EXAMPLE BOARD LAYOUT
DSF0006A
X2SON - 0.4 mm max height
PLASTIC SMALL OUTLINE - NO LEAD
![img-39.jpeg](./images/img-39.jpeg)

NOTES: (continued)
4. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/lit/slua271).![img-40.jpeg](./images/img-40.jpeg)

4220597/B 06/2022
4. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.![img-41.jpeg](./images/img-41.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.
3. This dimension does not include mold flash, protrusions, or gate burrs. Mold flash, protrusions, or gate burrs shall not exceed 0.15 mm per side.
4. Reference JEDEC registration MO-293 Variation UAAD-1![img-42.jpeg](./images/img-42.jpeg)

NOTES: (continued)
5. Publication IPC-7351 may have alternate designs.
6. Solder mask tolerances between and around signal pads can vary based on board fabrication site.![img-43.jpeg](./images/img-43.jpeg)

NOTES: (continued)
7. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.
8. Board assembly site may have different recommendations for stencil design.![img-44.jpeg](./images/img-44.jpeg)

Images above are just a representation of the package family, actual package may vary. Refer to the product data sheet for package details.![img-45.jpeg](./images/img-45.jpeg)
![img-46.jpeg](./images/img-46.jpeg)
![img-47.jpeg](./images/img-47.jpeg)
![img-48.jpeg](./images/img-48.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.
3. The size and shape of this feature may vary.![img-49.jpeg](./images/img-49.jpeg)

NOTES: (continued)
4. This package is designed to be soldered to a thermal pad on the board. For more information, refer to QFN/SON PCB application note in literature No. SLUA271 (www.ti.com/lit/slua271).![img-50.jpeg](./images/img-50.jpeg)

NOTES: (continued)
5. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.![img-51.jpeg](./images/img-51.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.![img-52.jpeg](./images/img-52.jpeg)

NOTES: (continued)
3. Final dimensions may vary due to manufacturing tolerance considerations and also routing constraints. For more information, see Texas Instruments literature number SNVA009 (www.ti.com/lit/snva009).![img-53.jpeg](./images/img-53.jpeg)

NOTES: (continued)
4. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release.YZV (S-XBGA-N4)
DIE-SIZE BALL GRID ARRAY
![img-54.jpeg](./images/img-54.jpeg)

NOTES: A. All linear dimensions are in millimeters. Dimensioning and tolerancing per ASME Y14.5M-1994.
B. This drawing is subject to change without notice.
C. NanoFree ${ }^{\text {TM }}$ package configuration.![img-55.jpeg](./images/img-55.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.
3. Reference JEDEC MO-203.
4. Support pin may differ or may not be present.
5. Lead width does not comply with JEDEC.
6. Body dimensions do not include mold flash, protrusions, or gate burrs. Mold flash, protrusions, or gate burrs shall not exceed 0.25 mm per side![img-56.jpeg](./images/img-56.jpeg)

NOTES: (continued)
7. Publication IPC-7351 may have alternate designs.
8. Solder mask tolerances between and around signal pads can vary based on board fabrication site.![img-57.jpeg](./images/img-57.jpeg)

NOTES: (continued)
9. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.
10. Board assembly site may have different recommendations for stencil design.# IMPORTANT NOTICE AND DISCLAIMER 

TI PROVIDES TECHNICAL AND RELIABILITY DATA (INCLUDING DATA SHEETS), DESIGN RESOURCES (INCLUDING REFERENCE DESIGNS), APPLICATION OR OTHER DESIGN ADVICE, WEB TOOLS, SAFETY INFORMATION, AND OTHER RESOURCES "AS IS" AND WITH ALL FAULTS, AND DISCLAIMS ALL WARRANTIES, EXPRESS AND IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE OR NON-INFRINGEMENT OF THIRD PARTY INTELLECTUAL PROPERTY RIGHTS.
These resources are intended for skilled developers designing with TI products. You are solely responsible for (1) selecting the appropriate TI products for your application, (2) designing, validating and testing your application, and (3) ensuring your application meets applicable standards, and any other safety, security, regulatory or other requirements.
These resources are subject to change without notice. TI grants you permission to use these resources only for development of an application that uses the TI products described in the resource. Other reproduction and display of these resources is prohibited. No license is granted to any other TI intellectual property right or to any third party intellectual property right. TI disclaims responsibility for, and you will fully indemnify TI and its representatives against, any claims, damages, costs, losses, and liabilities arising out of your use of these resources.
TI's products are provided subject to TI's Terms of Sale or other applicable terms available either on ti.com or provided in conjunction with such TI products. TI's provision of these resources does not expand or otherwise alter TI's applicable warranties or warranty disclaimers for TI products.
TI objects to and rejects any additional or different terms you may have proposed.
Mailing Address: Texas Instruments, Post Office Box 655303, Dallas, Texas 75265
Copyright © 2025, Texas Instruments Incorporated