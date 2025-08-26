# TCAN33x 3.3V CAN Transceivers with CAN FD (Flexible Data Rate) 

## 1 Features

- 3.3V Single supply operation
- Data rates up to 5Mbps (TCAN33xG devices)
- Compatible with ISO 11898-2
- SOIC-8 and SOT-23 package options
- Operating Modes:
- Normal mode (all devices)
- Low power standby mode with wake (TCAN334)
- Silent mode (TCAN330, TCAN337)
- Shutdown mode (TCAN330, TCAN334)
- Wide common mode range of operation $\pm 12 \mathrm{~V}$
- Bus pin fault protection of $\pm 14 \mathrm{~V}$
- Total loop delay < 135ns
- Wide ambient operation temperature range: $-40^{\circ} \mathrm{C}$ to $125^{\circ} \mathrm{C}$
- Optimized behavior when unpowered:
- Bus and logic pins are high impedance (no load to operating bus or application)
- Power up / down glitch free operation
- Excellent EMC performance
- Protection features:
- ESD Protection of bus terminals
- HBM ESD Protection exceeds $\pm 25 \mathrm{kV}$
- IEC61000-4-2 ESD Contact discharge protection exceeds $\pm 12 \mathrm{kV}$
- Driver dominant time out (TXD DTO)
- Receiver dominant time out (RXD DTO)
- Fault output pin (TCAN337 only)
- Undervoltage protection on $\mathrm{V}_{\mathrm{CC}}$
- Thermal shutdown protection
- Current limiting on bus pins


## 2 Applications

- 5-Mbps operation in CAN with flexible data rate networks (TCAN33xG devices)
- 1-Mbps operation in highly loaded can networks
- Industrial automation, control, sensors and drive systems
- Building, security and climate control automation
- Telecom base station status and control
- CAN Bus standards such as CANopen, DeviceNet, NMEA2000, ARINC825, ISO11783, CANaerospace


## 3 Description

The TCAN33x family of devices is compatible with the ISO 11898 High Speed CAN (Controller Area Network) Physical Layer standard. TCAN330, TCAN332, TCAN334 and TCAN337 are specified for data rates up to 1 Mbps . Pending the release of the updated version of ISO 11898-2 including CAN FD, additional timing parameters defining loop delay symmetry are specified for the TCAN330G, TCAN332G, TCAN334G and TCAN337G devices. The devices include many protection features including driver and receiver Dominant Time Out (DTO) providing CAN network robustness. Integrated 12kV IEC-61000-4-2 ESD Contact Discharge protection eliminates the need of additional components for system level robustness.

The use of single 3.3 V supply enables the transceivers to directly interface with 3.3 V CAN controllers/MCUs. In addition, these devices are fully compatible with other 5V CAN transceivers on the same bus.

These devices have excellent EMC performance due to matched Dominant and Recessive Common Modes. Ultra low power Shutdown and Standby modes make these devices attractive for battery powered applications.

This family of devices is available in standard 8-pin SOIC packages for drop-in compatibility and in small SOT-23 packages for space-constrained applications.

## Package Information

| PART NUMBER | PACKAGE $^{(1)}$ | PACKAGE SIZE ${ }^{(2)}$ |
| :-- | :-- | :-- |
| TCAN330/G | SOIC (8) | $4.9 \mathrm{~mm} \times 6 \mathrm{~mm}$ |
| TCAN332/G |  |  |
| TCAN334/G | SOT-23 (8) | $2.9 \mathrm{~mm} \times 1.6 \mathrm{~mm}$ |
| TCAN337/G |  |  |

(1) For all available packages, see Section 9.
(2) The package size (length $\times$ width) is a nominal value and includes pins, where applicable.![img-0.jpeg](img-0.jpeg)

A: Sleep Receiver and Wake Detect are device dependent options and are only available in TCAN334.
B: Fault Logic are only available in TCAN337.
C: Pin 5 and 8 functions are device dependent. Refer to Device Comparison Table.
Block Diagram# Table of Contents 

1 Features ..... 1
2 Applications ..... 1
3 Description ..... 1
4 Pin Configuration and Functions ..... 4
5 Specifications ..... 5
5.1 Absolute Maximum Ratings ..... 5
5.2 ESD Ratings ..... 5
5.3 Recommended Operating Conditions ..... 5
5.4 Thermal Information ..... 6
5.5 Electrical Characteristics ..... 6
5.6 Switching Characteristics ..... 9
5.7 Typical Characteristics ..... 11
5.8 Typical Characteristics, TCAN330 Receiver ..... 12
5.9 Typical Characteristics, TCAN330 Driver ..... 12

6 Detailed Description ..... 18
6.1 Overview ..... 18
6.2 Functional Block Diagram ..... 18
6.3 Feature Description ..... 18
6.4 Device Functional Modes ..... 22
7 Application Information Disclaimer ..... 26
7.1 Application Information ..... 26
7.2 Typical Application ..... 26
7.3 System Examples ..... 28
7.4 Power Supply Recommendations ..... 29
7.5 Layout ..... 29
8 Revision History ..... 31
9 Mechanical, Packaging, and Orderable Information. ..... 32

## Device Options

| DEVICE | PIN 5 | PIN 8 | DERATE | DESCRIPTION |
| :--: | :--: | :--: | :--: | :--: |
| TCAN330 | SHDN | S | 1 Mbps | Shutdown and silent modes |
| TCAN332 | NC | NC | 1 Mbps | Normal mode only |
| TCAN334 | SHDN | STB | 1 Mbps | Shutdown and standby with wake |
| TCAN337 | FAULT | S | 1 Mbps | Fault output and silent mode |
| TCAN330G | SHDN | S | 5 Mbps | Shutdown and silent modes |
| TCAN332G | NC | NC | 5 Mbps | Normal mode only |
| TCAN334G | SHDN | STB | 5 Mbps | Shutdown and standby with wake |
| TCAN337G | FAULT | S | 5 Mbps | Fault output and silent mode |# 4 Pin Configuration and Functions 

![img-1.jpeg](img-1.jpeg)

Figure 4-1. TCAN330 D, DCN Packages, 8-Pin SOIC, SOT-23, (Top View)
![img-2.jpeg](img-2.jpeg)

Figure 4-3. TCAN334 D, DCN Packages, 8-Pin SOIC, SOT-23, (Top View)
![img-3.jpeg](img-3.jpeg)

Figure 4-2. TCAN332 D, DCN Packages, 8-Pin SOIC, SOT-23, (Top View)
![img-4.jpeg](img-4.jpeg)

Figure 4-4. TCAN337 D, DCN Packages, 8-Pin SOIC, SOT-23, (Top View)

Table 4-1. Pin Functions

| PIN |  |  |  |  | I/O | DESCRIPTION |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| NAME | TCAN330 | TCAN332 | TCAN334 | TCAN337 |  |  |
| TXD | 1 | 1 | 1 | 1 | I | CAN transmit data input (LOW for dominant and HIGH for recessive bus states), integrated pull up |
| GND | 2 | 2 | 2 | 2 | GND | Ground connection |
| $\mathrm{V}_{\mathrm{CC}}$ | 3 | 3 | 3 | 3 | Supply | 3.3-V supply voltage |
| RXD | 4 | 4 | 4 | 4 | 0 | CAN receive data output (LOW for dominant and HIGH for recessive bus states), tri-state |
| SHDN | 5 | - | 5 | - | I | Drive high for shutdown mode. Internal pull-down. |
| NC | - | 5 | - | - | NC | No Connect - Not internally connected |
| FAULT | - | - | - | 5 | 0 | Open drain fault output pin. |
| CANL | 6 | 6 | 6 | 6 | I/O | Low level CAN bus line |
| CANH | 7 | 7 | 7 | 7 | I/O | High level CAN bus line |
| S | 8 | - | - | 8 | I | Drive high for silent mode, integrated pull down |
| NC | - | 8 | - | - | NC | No Connect - Not internally connected |
| STB | - | - | 8 | - | I | Drive high for low power standby mode, integrated pull down |# 5 Specifications 

### 5.1 Absolute Maximum Ratings

over operating free-air temperature range (unless otherwise noted) ${ }^{(1)(2)}$

|  |  | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: |
| Supply Voltage range, $\mathrm{V}_{\mathrm{CC}}$ |  | $-0.3$ | 5 | V |
| Voltage at any bus terminal (CANH or CANL), $\mathrm{V}_{\text {(BUS) }}$ |  | $-14$ | 14 | V |
| Logic input terminal voltage range $V_{\text {(Logic, Input) }}$ |  | $-0.3$ | 5 | V |
| Logic output terminal voltage range, $V_{\text {(Logic, Output) }}$ |  | $-0.3$ | 5 | V |
| Logic output current, $\mathrm{I}_{\text {(SLOGIC) }}$ |  |  | 8 | mA |
| Operating junction temperature range, $T_{J}$ |  | $-40$ | 150 | ${ }^{\circ} \mathrm{C}$ |
| Storage temperature, $T_{\text {stg }}$ |  |  | 150 | ${ }^{\circ} \mathrm{C}$ |

(1) Stresses beyond those listed under Absolute Maximum Ratings may cause permanent damage to the device. These are stress ratings only, which do not imply functional operation of the device at these or any other conditions beyond those indicated under Recommended Operating Conditions. Exposure to absolute-maximum-rated conditions for extended periods may affect device reliability.
(2) All voltage values, except differential I/O bus voltages, are with respect to ground terminal.

### 5.2 ESD Ratings

|  |  |  | VALUE | UNIT |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {(ESD) }}$ | Electrostatic discharge | Human-body model (HBM), per ANSI/ESDA/ JEDEC JS-001 ${ }^{(1)}$ | All pins except CANH and CANL | $\pm 4000$ |
|  |  |  | Pins CANH and CANL | $\pm 25000$ |
|  |  | Charged-device model (CDM), per JEDEC specification JESD22-C101 ${ }^{(2)}$ | All pins | $\pm 1500$ |
|  |  | IEC 61400-4-2 Contact Discharge | CANH and CANL terminals to GND | $\pm 12000$ |

(1) JEDEC document JEP155 states that 500 V HBM allows safe manufacturing with a standard ESD control process. .
(2) JEDEC document JEP157 states that 250 V CDM allows safe manufacturing with a standard ESD control process.

### 5.3 Recommended Operating Conditions

over operating free-air temperature range (unless otherwise noted)

|  |  | MIN | NOM | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{CC}}$ | Supply voltage | 3 |  | 3.6 | V |
| $\mathrm{I}_{\text {(OH(LOGIC) }}$ | Logic terminal HIGH level output current | $-2$ |  |  | mA |
| $\mathrm{I}_{\text {(SLOGIC) }}$ | Logic terminal LOW level output current |  |  | 2 |  |
| $T_{A}$ | Operational free-air temperature | $-40$ |  | 125 | ${ }^{\circ} \mathrm{C}$ |# 5.4 Thermal Information 

| THERMAL METRIC ${ }^{(1)}$ |  |  | TCAN33x | TCAN33x | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | D (SOIC) | DCN (SOT-23) |  |
|  |  |  | 8 PINS | 8 PINS |  |
| $R_{8, \text { IA }}$ | Junction-to-ambient thermal resistance |  | 114.4 | 154.4 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{8, \text { ( } O \text { (top) }}$ | Junction-to-case (top) thermal resistance |  | 58.7 | 76.6 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{8, B}$ | Junction-to-board thermal resistance |  | 55.2 | 49.2 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\psi_{J T}$ | Junction-to-top characterization parameter |  | 11.7 | 11.9 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\psi_{J B}$ | Junction-to-board characterization parameter |  | 54.6 | 49.2 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{8, C(\text { bot) }}$ | Junction-to-case (bottom) thermal resistance |  | N/A | N/A | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $P_{D}$ | Average power dissipation | $\begin{aligned} & \mathrm{V}_{\mathrm{CC}}=3.3 \mathrm{~V}, \mathrm{~T}_{\mathrm{J}}=27^{\circ} \mathrm{C}, \mathrm{R}_{\mathrm{L}}=60 \Omega, \mathrm{SHDN}, \mathrm{S} \text { and STB } \\ & \text { at } 0 \mathrm{~V}, \text { Input to } \mathrm{TXD} \text { at } 500 \mathrm{kHz}, 50 \% \text { duty cycle square } \\ & \text { wave, } \mathrm{C}_{\mathrm{L}(\mathrm{RXD})}=15 \mathrm{pF} \end{aligned}$ | 65 | 65 | mW |
| $T_{S D}$ | Thermal shutdown temperature |  | 175 | 175 | ${ }^{\circ} \mathrm{C}$ |
| $T_{\text {HYS }}$ | Thermal shutdown hysteresis |  | 5 | 5 | ${ }^{\circ} \mathrm{C}$ |

(1) For more information about traditional and new thermal metrics, see the Semiconductor and IC Package Thermal Metrics application report.

### 5.5 Electrical Characteristics

over operating free-air temperature range, $\mathrm{T}_{\mathrm{J}}=-40^{\circ} \mathrm{C}$ to $150^{\circ} \mathrm{C}$. All typical values are at $25^{\circ} \mathrm{C}$ and supply voltages of $\mathrm{V}_{\mathrm{CC}}=$ $3.3 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=60 \Omega$, (unless otherwise noted)

| PARAMETER |  |  | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Supply |  |  |  |  |  |  |  |
| $I_{C C}$ | Supply current Normal Mode | Dominant | See Figure 6-1, TXD $=0 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=60 \Omega$, $\mathrm{C}_{\mathrm{L}}=$ open, S, STB and SHDN $=0 \mathrm{~V}$. Typical Bus Load. |  |  | 55 | mA |
|  |  |  | See Figure 6-1, TXD $=0 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=50 \Omega$, $\mathrm{C}_{\mathrm{L}}=$ open, S, STB and SHDN $=0 \mathrm{~V}$. High Bus Load. |  |  | 60 |
|  |  | Dominant with bus fault | See Figure 6-1, TXD $=0 \mathrm{~V}, \mathrm{~S}$, STB and SHDN $=0 \mathrm{~V}, \mathrm{CANH}=-12 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=$ open, $\mathrm{C}_{\mathrm{L}}=$ open |  |  | 180 |  |
|  |  | Recessive | See Figure 6-1, TXD $=\mathrm{V}_{\mathrm{CC}}, \mathrm{R}_{\mathrm{L}}=50 \Omega$, $\mathrm{C}_{\mathrm{L}}=$ open, S, STB and SHDN $=0 \mathrm{~V}$ |  |  | 3.5 |  |
|  | Supply Current: Silent Mode |  | See Figure 6-1, TXD $=\mathrm{V}_{\mathrm{CC}}, \mathrm{R}_{\mathrm{L}}=50 \Omega$, $\mathrm{C}_{\mathrm{L}}=$ open, $\mathrm{S}=\mathrm{V}_{\mathrm{CC}}$ |  |  | 2.5 |  |
|  | Supply Current: Standby Mode |  | $\mathrm{T}_{\mathrm{A}}<85^{\circ} \mathrm{C}$, STB at $\mathrm{V}_{\mathrm{CC}}$, RXD floating, TXD at $\mathrm{V}_{\mathrm{CC}}$ |  |  | 15 | $\mu \mathrm{A}$ |
|  |  |  | STB at $\mathrm{V}_{\mathrm{CC}}$, RXD floating, TXD at $\mathrm{V}_{\mathrm{CC}}$ |  |  | 20 |  |
|  | Supply Current: Shutdown Mode |  | $\mathrm{T}_{\mathrm{A}}<85^{\circ} \mathrm{C}$, SHDN at $\mathrm{V}_{\mathrm{CC}}$, RXD floating, TXD at $\mathrm{V}_{\mathrm{CC}}$ |  |  | 1 |  |
|  |  |  | SHDN $=\mathrm{V}_{\mathrm{CC}}$, RXD floating, TXD at $\mathrm{V}_{\mathrm{CC}}$ |  |  | 2.5 |  |
| $\mathrm{UV}_{(\mathrm{VCC})}$ | Rising under voltage detection on $\mathrm{V}_{\mathrm{CC}}$ for protected mode |  |  |  | 2.2 | 2.6 | V |
|  | Falling under voltage detection on $\mathrm{V}_{\mathrm{CC}}$ for protected mode |  |  | 1.65 | 2 | 2.5 |  |
| $\mathrm{V}_{\text {HYS(UVVCC) }}$ | Hysteresis voltage on $\mathrm{UV}_{(\mathrm{VCC})}$ |  |  |  | 200 |  | mV |
| Driver |  |  |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{O}(\mathrm{D})}$ | Bus output voltage (dominant) | CANH | See Figure 6-3 and Figure 6-2, TXD = | 2.45 |  | $\mathrm{V}_{\mathrm{CC}}$ | V |
|  |  | CANL | $\begin{aligned} & 0 \mathrm{~V}, \mathrm{~S}, \text { STB and SHDN }=0 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=60 \Omega \text {, } \\ & \mathrm{C}_{\mathrm{L}}=\text { open } \end{aligned}$ | 0.5 |  | 1.25 |  |
| $\mathrm{V}_{\mathrm{O}(\mathrm{R})}$ | Bus output voltage (recessive) |  | See Figure 6-3 and Figure 6-2, TXD = $\mathrm{V}_{\mathrm{CC}}, \mathrm{STB}, \mathrm{SHDN}=0 \mathrm{~V}, \mathrm{~S}=0 \mathrm{~V}$ or $\mathrm{V}_{\mathrm{CC}}$ <br> (1), $\mathrm{R}_{\mathrm{L}}=$ open (no load) |  | 1.85 |  | V |# 5.5 Electrical Characteristics (continued) 

over operating free-air temperature range, $\mathrm{T}_{\mathrm{J}}=-40^{\circ} \mathrm{C}$ to $150^{\circ} \mathrm{C}$. All typical values are at $25^{\circ} \mathrm{C}$ and supply voltages of $\mathrm{V}_{\mathrm{CC}}=$ $3.3 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=60 \Omega$, (unless otherwise noted)

| PARAMETER |  | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {OD(D) }}$ | Differential output voltage (dominant) | See Figure 6-3 and Figure 6-2, TXD = 0V, S, STB and SHDN $=0 \mathrm{~V}, 50 \Omega \leq \mathrm{R}_{\mathrm{L}} \leq$ $65 \Omega, \mathrm{C}_{\mathrm{L}}=$ open | 1.6 |  | 3 | V |
|  |  | See Figure 6-3 and Figure 6-2, TXD = 0V, S, STB and SHDN $=0 \mathrm{~V}, 45 \Omega \leq \mathrm{R}_{\mathrm{L}}<$ $50 \Omega, \mathrm{C}_{\mathrm{L}}=$ open | 1.5 |  | 3 |  |
| $\mathrm{V}_{\text {OD(R) }}$ | Differential output voltage (recessive) | See Figure 6-3 and Figure 6-2, TXD = $\mathrm{V}_{\mathrm{CC}}, \mathrm{S}$, STB and SHDN $=0 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=60 \Omega$, $\mathrm{C}_{\mathrm{L}}=$ open | $-120$ |  | 12 | mV |
|  |  | $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}$, See Figure 6-3 and Figure 6-2, TXD $=\mathrm{V}_{\mathrm{CC}}, \mathrm{S}$, STB and SHDN $=$ $0 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=$ open (no load), $\mathrm{C}_{\mathrm{L}}=$ open | $-50$ |  | 50 |  |
|  |  | See Figure 6-3 and Figure 6-2, TXD = $\mathrm{V}_{\mathrm{CC}}, \mathrm{S}$, STB and SHDN $=0 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=$ open (no load), $\mathrm{C}_{\mathrm{L}}=$ open | $-50$ |  | 100 |  |
| $\mathrm{V}_{(\mathrm{SYM})}$ | Output symmetry (dominant and recessive) $\left(\mathrm{CANH}_{\text {REC }}+\mathrm{CANL}_{\text {REC }}-\mathrm{CANH}_{\text {DOM }}-\mathrm{CANL}_{\text {DOM }}\right)$ | See Figure 6-3 and Figure 6-2, S, STB and $\mathrm{SHDN}=0 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=60 \Omega, \mathrm{C}_{\mathrm{L}}=$ open | $-400$ |  | 400 | mV |
| $\mathrm{I}_{\text {OS(DOM) }}$ | Short-circuit steady-state output current, Dominant | See Figure 6-9, $\mathrm{V}_{\text {(CANH) }}=-12 \mathrm{~V}$, CANL $=$ open, TXD $=0 \mathrm{~V}$ | $-200$ |  |  | mA |
|  |  | See Figure 6-9, $\mathrm{V}_{\text {(CANL) }}=12 \mathrm{~V}$, CANH $=$ open, TXD $=0 \mathrm{~V}$ |  |  | 200 |  |
| $\mathrm{I}_{\text {OS(REC) }}$ | Short-circuit steady-state output current, Recessive | See Figure 6-9, $-12 \mathrm{~V} \leq \mathrm{V}_{\text {BUS }} \leq 12 \mathrm{~V}$, $\mathrm{V}_{\text {BUS }}=\mathrm{CANH}=\mathrm{CANL}, \mathrm{TXD}=\mathrm{V}_{\mathrm{CC}}$ | $-5$ |  | 5 | mA |# 5.5 Electrical Characteristics (continued) 

over operating free-air temperature range, $\mathrm{T}_{\mathrm{J}}=-40^{\circ} \mathrm{C}$ to $150^{\circ} \mathrm{C}$. All typical values are at $25^{\circ} \mathrm{C}$ and supply voltages of $\mathrm{V}_{\mathrm{CC}}=$ $3.3 \mathrm{~V}, \mathrm{R}_{\mathrm{L}}=60 \Omega$, (unless otherwise noted)

| PARAMETER |  | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Receiver |  |  |  |  |  |  |
| $V_{I T}$ | Input threshold voltage, normal modes and selective wake modes | See Figure 6-3 and Table 6-7 | 500 |  | 900 | mV |
| $\mathrm{V}_{\text {VITS }}$ | Hysteresis voltage for input threshold, normal modes and selective wake modes |  |  | 120 |  |  |
| $\mathrm{V}_{\mathrm{CM}}$ | Common Mode Range: normal and silent modes |  | $-12$ |  | 12 | V |
| $\mathrm{V}_{\text {IT(STB) }}$ | Input Threshold, standby mode | $-2 \mathrm{~V}<\mathrm{V}_{\mathrm{CM}}<7 \mathrm{~V}$ <br> See Figure 6-3 and Table 6-7 | 400 |  | 1150 | mV |
|  |  | $-12 \mathrm{~V}<\mathrm{V}_{\mathrm{CM}}<12 \mathrm{~V}$ <br> See Figure 6-3 and Table 6-7 | 400 |  | 1350 | mV |
| $\mathrm{I}_{\text {(OFF) (LKG) }}$ | Power-off (unpowered) bus input leakage current | $\mathrm{T}_{\mathrm{A}}<85^{\circ} \mathrm{C}, \mathrm{CANH}=\mathrm{CANL}=3.3 \mathrm{~V}, \mathrm{~V}_{\mathrm{CC}}$ to GND via $0 \Omega$ and $47 \mathrm{k} \Omega$ resistor |  |  | 6 | $\mu \mathrm{A}$ |
|  |  | CANH = CANL $=3.3 \mathrm{~V}, \mathrm{~V}_{\mathrm{CC}}$ to GND via $0 \Omega$ and $47 \mathrm{k} \Omega$ resistor |  |  | 12 |  |
| $\mathrm{C}_{\mathrm{I}}$ | Input capacitance to ground (CANH or CANL) |  |  |  | 20 | pF |
| $\mathrm{C}_{\text {ID }}$ | Differential input capacitance |  |  |  | 10 |  |
| $R_{I D}$ | Differential input resistance | TXD $=\mathrm{V}_{\mathrm{CC}}$, Normal Mode | 30 |  | 80 | $\mathrm{k} \Omega$ |
| $R_{I N}$ | Input resistance (CANH or CANL) | TXD $=\mathrm{V}_{\mathrm{CC}}$, Normal mode | 15 |  | 40 |  |
| $R_{(N M)}$ | Input resistance matching: $\left[1-\left(R_{(N(C A N H)}\right) /\right.$ $\left.R_{(N(C A N L)} \| \neq 100 \%\right)$ | $\mathrm{V}_{\text {(CANH) }}=\mathrm{V}_{\text {(CANL) }}$ | $-3 \%$ |  | $3 \%$ |  |
| TXD Terminal (CAN Transmit Data Input) |  |  |  |  |  |  |
| $V_{I H}$ | HIGH level input voltage |  | 2 |  |  | V |
| $V_{I L}$ | LOW level input voltage |  |  |  | 0.8 | V |
| $I_{I H}$ | HIGH level input leakage current | TXD $=\mathrm{V}_{\mathrm{CC}}=3.6 \mathrm{~V}$ | $-2.5$ | 0 | 3 | $\mu \mathrm{A}$ |
| $I_{I L}$ | LOW level input leakage current | TXD $=0 \mathrm{~V}, \mathrm{~V}_{\mathrm{CC}}=3.6 \mathrm{~V}$ | $-4$ | 0 | 0 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {LKG(OFF) }}$ | Unpowered leakage current | TXD $=3.6 \mathrm{~V}, \mathrm{~V}_{\mathrm{CC}}=0 \mathrm{~V}$ | $-2$ | 0 | 2.5 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {(CAP) }}$ | Input Capacitance |  |  | 2.5 |  | pF |
| RXD Terminal (CAN Receive Data Output) |  |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{OH}}$ | HIGH level output voltage | See Figure 6-3, $\mathrm{I}_{\mathrm{O}}=-2 \mathrm{~mA}$ | $0.8 \times \mathrm{V}_{\mathrm{CC}}$ |  |  | V |
| $\mathrm{V}_{\mathrm{OL}}$ | LOW level output voltage | See Figure 6-3, $\mathrm{I}_{\mathrm{O}}=2 \mathrm{~mA}$ |  | 0.2 | 0.4 | V |
| $\mathrm{I}_{\text {LKG(OFF) }}$ | Unpowered leakage current | RXD $=3.6 \mathrm{~V}, \mathrm{~V}_{\mathrm{CC}}=0 \mathrm{~V}$ | $-1$ | 0 | 1 | $\mu \mathrm{A}$ |
| STB/S/SHDN Terminals |  |  |  |  |  |  |
| $V_{I H}$ | HIGH level input voltage |  | 2 |  |  | V |
| $V_{I L}$ | LOW level input voltage |  |  |  | 0.8 | V |
| $I_{I H}$ | HIGH level input leakage current | STB, S, SHDN $=\mathrm{V}_{\mathrm{CC}}=3.6 \mathrm{~V}$ | $-3$ | 0 | 10 | $\mu \mathrm{A}$ |
| $I_{I L}$ | LOW level input leakage current | STB, S, SHDN $=0 \mathrm{~V}, \mathrm{~V}_{\mathrm{CC}}=3.6 \mathrm{~V}$ | $-4$ | 0 | 1 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {LKG(OFF) }}$ | Unpowered leakage current | STB, S, SHDN $=3.6 \mathrm{~V}, \mathrm{~V}_{\mathrm{CC}}=0 \mathrm{~V}$ | $-3$ | 0 | 5 | $\mu \mathrm{A}$ |
| FAULT Pin (Fault Output), TCAN337 only |  |  |  |  |  |  |
| $\mathrm{I}_{\mathrm{CH}}$ | Output current high level | FAULT $=\mathrm{V}_{\mathrm{CC}}$, See Figure 6-11 | $-10$ |  |  | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\mathrm{CL}}$ | Output current low level | FAULT $=0.4 \mathrm{~V}$, See Figure 6-11 | 4 | 12 |  | mA |

(1) The bus output voltage (recessive) is the same if the device is in normal mode with $S$ terminal LOW, or if the device is in silent mode with the $S$ terminal is HIGH.# 5.6 Switching Characteristics 

over operating free-air temperature range (unless otherwise noted)

| PARAMETER |  | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Device Switching Characteristics |  |  |  |  |  |  |
| $t_{\text {PROP(LOOP) }}$ | Total loop delay, driver input (TXD) to receiver output (RXD), recessive to dominant and dominant to recessive | See Figure 6-6, S, STB and SHDN $=0 \mathrm{~V}$, $R_{L}=60 \Omega, C_{L}=100 \mathrm{pF}, C_{\mathrm{L(RXD)}}=15 \mathrm{pF}$ |  | 100 | 135 | ns |
| $t_{\text {PROP(LOOP) }}$ | Total Loop delay in highly loaded network | See Figure 6-6, S, STB and SHDN $=0 \mathrm{~V}$, $R_{L}=120 \Omega, C_{L}=200 \mathrm{pF}$, $C_{\text {L(RXD) }}=15 \mathrm{pF}$ |  | 120 | 180 | ns |
| $t_{\text {BUS_SYM_2 }}$ | 2 Mbps transmitted recessive bit width | See Figure 6-7, S or STB $=0 \mathrm{~V}, R_{L}=$ $60 \Omega, C_{L}=100 \mathrm{pF}, C_{\mathrm{L(RXD)}}=15 \mathrm{pF}$, $t_{\text {BIT }}=500 \mathrm{~ns}$ | 435 |  | 530 | ns |
| $t_{\text {REC_SYM_2 }}$ | 2 Mbps received recessive bit width |  | 400 |  | 550 | ns |
| $\Delta t_{\text {SYM_2 }}$ | 2 Mbps receiver timing symmetry $\left(t_{\text {REC_SYM_2 }}-t_{\text {BUS_SYM_2 }}\right)$ | TCAN330G, TCAN332G, TCAN334G and TCAN337G only | $-65$ |  | 40 | ns |
| $t_{\text {BUS_SYM_5 }}$ | 5 Mbps transmitted recessive bit width | See Figure 6-7, S or STB $=0 \mathrm{~V}, R_{L}=$ $60 \Omega, C_{L}=100 \mathrm{pF}, C_{\mathrm{L(RXD)}}=15 \mathrm{pF}$, $t_{\text {BIT }}=200 \mathrm{~ns}$ | 155 |  | 210 | ns |
| $t_{\text {REC_SYM_5 }}$ | 5 Mbps received recessive bit width |  | 120 |  | 220 | ns |
| $\Delta t_{\text {SYM_5 }}$ | 5 Mbps receiver timing symmetry $\left(t_{\text {REC_SYM_5 }}-t_{\text {BUS_SYM_5 }}\right)$ | TCAN330G, TCAN332G, TCAN334G and TCAN337G only | $-45$ |  | 15 | ns |
| $t_{\text {MODE }}$ | Mode change time | See Figure 6-4 and Figure 6-5. $R_{L}=60 \Omega, C_{L}=100 \mathrm{pF}$, $C_{\text {L(RXD) }}=15 \mathrm{pF}$ |  | 5 | 10 | $\mu \mathrm{s}$ |
| $t_{\text {UV_RE-ENABLE }}$ | Re-enable time after UV event | Time for device to return to normal operation from $\mathrm{UV}_{(\mathrm{VCC})}$ under voltage event |  |  | 1000 | $\mu \mathrm{s}$ |
| $t_{\text {WK_FILTER }}$ | Bus time to meet Filtered Bus Requirements for Wake Up Request | See Figure 6-5, Standby mode. $-12 \mathrm{~V}<\mathrm{V}_{\mathrm{CM}}<12 \mathrm{~V}$ | 0.5 |  | 4 | $\mu \mathrm{s}$ |
| Driver Switching Characteristics |  |  |  |  |  |  |
| $t_{\text {pHR }}$ | Propagation delay time, HIGH TXD to Driver Recessive | See Figure 6-2, S, STB and SHDN $=0 \mathrm{~V}$. $R_{L}=60 \Omega, C_{L}=100 \mathrm{pF}$, |  | 25 |  | ns |
| $t_{\text {pLD }}$ | Propagation delay time, LOW TXD to Driver Dominant |  |  | 20 |  |  |
| $t_{\text {tWQO }}$ | Pulse skew $\left(\left|t_{\text {pHR }}-\right|_{\text {pLD }}\right)$ |  |  | 5 |  |  |
| $t_{r}$ | Differential output signal rise time |  |  | 17 |  |  |
| $t_{f}$ | Differential output signal fall time |  |  | 9 |  |  |
| $t_{\text {TXD_DTO }}$ | Driver dominant time out ${ }^{(1)}$ | See Figure 6-8, $R_{L}=60 \Omega, C_{L}=100 \mathrm{pF}$ |  | 1.2 | 2.6 | 3.8 | ms |
| Receiver Switching Characteristics |  |  |  |  |  |  |
| $t_{\text {pRH }}$ | Propagation delay time, bus recessive input to high RXD output | See Figure 6-3, $\mathrm{C}_{\mathrm{L(RXD)}}=15 \mathrm{pF}$ CANL $=$ 1.5 V , CANH $=3.5 \mathrm{~V}$ |  | 62 |  | ns |
| $t_{\text {pOL }}$ | Propagation delay time, bus dominant input to RXD low output |  |  | 56 |  |  |
| $t_{r}$ | Output signal rise time (RXD) |  |  | 7 |  |  |
| $t_{f}$ | Output signal fall time (RXD) |  |  | 6 |  |  |
| $t_{\text {RXD_DTO }}$ | Receiver dominant time out ${ }^{(2)}$ | See Figure 6-10, $\mathrm{C}_{\mathrm{L(RXD)}}=15 \mathrm{pF}$ |  | 1.6 | 3 | 5 | ms |

(1) The TXD dominant time out ( $t_{\text {TXD_DTO }}$ ) disables the driver of the transceiver once the TXD has been dominant longer than $t_{\text {TXD_DT }}$ O, which releases the bus lines to recessive, preventing a local failure from locking the bus dominant. The driver may only transmit dominant again after TXD has been returned HIGH (recessive). While this protects the bus from local faults, locking the bus dominant, it limits the minimum data rate possible. The CAN protocol allows a maximum of eleven successive dominant bits (on TXD) for the worst case, where five successive dominant bits are followed immediately by an error frame. This, along with the $t_{\text {TXD_DTO }}$ minimum, limits the minimum bit rate. The minimum bit rate may be calculated by: Minimum Bit Rate $=11 / t_{\text {TXD_DTO }}=11$ bits $/ 1.2 \mathrm{~ms}=9.2 \mathrm{kbps}$.
(2) The RXD timeout ( $t_{\text {RXD_DTO }}$ ) disables the RXD output in the case that the bus has been dominant longer than $t_{\text {RXD_DTO }}$, which releases RXD pin to the recessive state (high), thus preventing a dominant bus failure from permanently keeping the RXD pin low. The RXD pin will automatically resume normal operation once the bus has been returned to a recessive state. While this protects the protocol controller from a permanent dominant state, it limits the minimum data rate possible. The CAN protocol allows a maximum of eleven successive dominant bits (on RXD) for the worst case, where five successive dominant bits are followed immediately by an error frame. This, along with the $t_{\text {RXD_DTO }}$ minimum, limits the minimum bit rate. The minimum bit rate may be calculated by: Minimum Bit Rate $=11 / t_{\text {RXD_DTO }}=11$ bits $/ 1.6 \mathrm{~ms}=6.9 \mathrm{kbps}$.![img-5.jpeg](img-5.jpeg)

Figure 5-1. Example Timing Diagram for TXD DTO and FAULT Pin
![img-6.jpeg](img-6.jpeg)

Figure 5-2. Example Timing Diagram for RXD DTO and FAULT Pin# 5.7 Typical Characteristics 

![img-7.jpeg](img-7.jpeg)

Figure 5-3. Supply Current (RSM) vs Frequency
![img-8.jpeg](img-8.jpeg)

Figure 5-5. Driver High-Level Output Current vs High-level Output Voltage
![img-9.jpeg](img-9.jpeg)

Figure 5-4. Driver Low-Level Output Current vs Low-level Output Voltage
![img-10.jpeg](img-10.jpeg)

Figure 5-6. Dominant Voltage ( $\mathrm{V}_{\mathrm{DD}}$ ) vs Free-Air Temperature# 5.8 Typical Characteristics, TCAN330 Receiver 

![img-11.jpeg](img-11.jpeg)

Figure 5-7. Receiver Bus Recessive Input to High RXD Output Propagation Delay Time vs Free-Air Temperature
![img-12.jpeg](img-12.jpeg)

Figure 5-9. Receiver Rise Time vs Free-Air Temperature
![img-13.jpeg](img-13.jpeg)

Figure 5-8. Receiver Bus Dominant Input to Low RXD Output Propagation Delay Time vs Free-Air Temperature
![img-14.jpeg](img-14.jpeg)

Figure 5-10. Receiver Fall Time vs Free-Air Temperature

### 5.9 Typical Characteristics, TCAN330 Driver

![img-15.jpeg](img-15.jpeg)

Figure 5-11. Driver High TXD Input to Driver Recessive Output Propagation Delay Time vs Free-Air Temperature
![img-16.jpeg](img-16.jpeg)

Figure 5-12. Driver Low TXD Input to Driver Dominant Output Propagation Delay Time vs Free-Air Temperature# 5.9 Typical Characteristics, TCAN330 Driver (continued) 

![img-17.jpeg](img-17.jpeg)

Figure 5-13. Differential Output Signal Rise Time vs Free-Air Temperature
![img-18.jpeg](img-18.jpeg)

Figure 5-15. Pulse Skew $\left(\left|t_{p H R}-t_{p L O}\right|\right)$ vs Free-Air Temperature
![img-19.jpeg](img-19.jpeg)

Figure 5-14. Differential Output Signal Fall Time vs Free-Air Temperature
![img-20.jpeg](img-20.jpeg)

Figure 5-16. Total Loop Delay Recessive to Dominant $\mathrm{t}_{\text {PROP(LOOP1) }}$ vs Free-Air Temperature
![img-21.jpeg](img-21.jpeg)

Figure 5-17. Total Loop Delay Dominant to Recessive $\mathrm{t}_{\text {PROP(LOOP2) }}$ vs Free-Air TemperatureParameter Measurement Information
![img-22.jpeg](img-22.jpeg)

Figure 6-1. Supply Test Circuit
![img-23.jpeg](img-23.jpeg)

Figure 6-2. Driver Test Circuit and Measurement
![img-24.jpeg](img-24.jpeg)

Figure 6-3. Receiver Test Circuit and Measurement
![img-25.jpeg](img-25.jpeg)

Figure 6-4. $\mathbf{t}_{\text {MODE }}$ Test Circuit and Measurement, from Normal to Shutdown, Standby or Silent Mode![img-26.jpeg](img-26.jpeg)

Figure 6-5. $\mathbf{t}_{\text {MODE }}$ Test Circuit and Measurement, from Shutdown, Standby or Silent to Normal Mode
![img-27.jpeg](img-27.jpeg)

Figure 6-6. $\mathbf{t}_{\text {PROP(LOOP) }}$ Test Circuit and Measurement![img-28.jpeg](img-28.jpeg)

Figure 6-7. Loop Delay Symmetry Test Circuit and Measurement
![img-29.jpeg](img-29.jpeg)

Figure 6-8. TXD Dominant Time Out Test Circuit and Measurement![img-30.jpeg](img-30.jpeg)

Figure 6-9. Driver Short-Circuit Current Test and Measurement
![img-31.jpeg](img-31.jpeg)

Figure 6-10. RXD Dominant Timeout Test Circuit and Measurement
![img-32.jpeg](img-32.jpeg)

Figure 6-11. FAULT Test and Measurement# 6 Detailed Description 

### 6.1 Overview

This family of CAN transceivers is compatible with the ISO11898-2 High-Speed CAN (controller area network) physical layer standard. The devices are designed to interface between the differential bus lines in CAN and the CAN protocol controller.

### 6.2 Functional Block Diagram

![img-33.jpeg](img-33.jpeg)

Copyright © 2016, Texas Instruments Incorporated
A. Sleep Receiver and Wake Detect are device dependent options and are only available in TCAN334.
B. Fault Logic is only available in TCAN337.
C. Pin 5 and 8 functions are device dependent. Refer to Section Device Options.

### 6.3 Feature Description

### 6.3.1 TXD Dominant Timeout (TXD DTO)

During normal mode (the only mode where the CAN driver is active), the TXD DTO circuit prevents the transceiver from blocking network communication in the event of a hardware or software failure where TXD is held dominant longer than the timeout period $t_{\text {TXD_DTO }}$. The DTO circuit timer starts on a falling edge on TXD. The DTO circuit disables the CAN bus driver if no rising edge is seen before the timeout period expires. This frees the bus for communication between other nodes on the network. The CAN driver is re-activated when arecessive signal is seen on TXD pin, thus clearing the TXD DTO condition. The receiver and RXD pin still reflect the CAN bus, and the bus pins are biased to recessive level during a TXD dominant timeout.

# 6.3.2 RXD Dominant Timeout (RXD DTO) 

All devices have a RXD DTO circuit that prevents a bus stuck dominant fault from permanently driving the RXD output dominant (low) when the bus is held dominant longer than the timeout period $t_{\text {RXD_DTO }}$. The RXD DTO timer starts on a falling edge on RXD (bus going dominant). If no rising edge (bus returning recessive) is seen before the timeout constant of the circuit expires ( $\mathrm{t}_{\text {RXD_DTO }}$ ), the RXD pin returns high (recessive). The RXD output is re-activated to mirror the bus receiver output when a recessive signal is seen on the bus, clearing the RXD dominant timeout. The CAN bus pins are biased to the recessive level during a RXD DTO.

### 6.3.3 Thermal Shutdown

If the junction temperature of the device exceeds the thermal shutdown threshold, the device turns off the CAN driver circuits thus blocking the TXD-to-bus transmission path. The shutdown condition is cleared when the junction temperature of the device drops below the thermal shutdown temperature of the device. If the fault condition that caused the thermal shutdown is still present, the temperature may rise again and the device will enter thermal shut down again. Prolonged operation with thermal shutdown conditions may affect device reliability. The thermal shutdown circuit includes hysteresis to avoid oscillation of the driver output.

During thermal shutdown the CAN bus drivers are turned off, thus no transmission is possible from TXD to the bus. The CAN bus pins are biased to recessive level during a thermal shutdown and the receiver to RXD path remains operational.

### 6.3.4 Undervoltage Lockout and Unpowered Device

The $\mathrm{V}_{\mathrm{CC}}$ supply terminal has under voltage detection which will place the device in protected mode if the supply drops below the UVLO threshold. This protects the bus during an under voltage event on $\mathrm{V}_{\mathrm{CC}}$ by placing the bus into a high impedance biased to ground state and the RXD terminal into a tri-stated (high impedance) state. During undervoltage the device does not pass any signals from the bus. If the device is in normal mode and $\mathrm{V}_{\mathrm{CC}}$ supply is lost the device will transition to a protected mode.

The device is designed to be an "ideal passive" or "no load" to the CAN bus if the device is unpowered. The bus terminals (CANH, CANL) have low leakage currents when the device is unpowered, so the device does not load the bus. This is critical if some nodes of the network are unpowered while the rest of the of network remains operational. Logic pins also have low leakage currents when the device is unpowered, so the device does not load other circuits which may remain powered.

Table 6-1. Undervoltage Protection 3.3-V Single Supply Devices

| $\mathbf{V}_{\mathbf{C C}}$ | DEVICE STATE | BUS | RXD |
| :--: | :--: | :--: | :--: |
| GOOD | Operational | Per Operating Mode | Per Operating Mode |
| BAD | Protected | Common mode bias to GND | High Impedance |
| UNPOWERED | Unpowered | High Impedance (no load) | High Impedance |# 6.3.5 Fault Pin (TCAN337) 

If one or more of the faults (TXD-Dominant Timeout, RXD dominant Timeout, Thermal Shutdown or Undervoltage Lockout) occurs, the FAULT pin (open-drain) turns off, resulting in a high level when externally pulled up to $\mathrm{V}_{\mathrm{CC}}$ supply.
![img-34.jpeg](img-34.jpeg)

Figure 6-1. FAULT Pin Function Diagram and Application

### 6.3.6 Floating Pins

The device has internal pull ups and pull downs on critical terminals to place the device into known states if the pin floats. See Table 6-1 for details on pin bias conditions.

Table 6-2. Pin Bias

| PIN | PULL UP or PULL DOWN | COMMENT |
| :--: | :--: | :--: |
| TXD | Pull up | Weakly biases TXD toward recessive to prevent bus blockage or TXD DTO <br> triggering. |
| STB | Pull down | Weakly biases STB terminal towards normal mode. |
| S | Pull down | Weakly biases S terminal towards normal mode. |
| SHDN | Pull down | Weakly biases SHDN terminal towards normal mode. |

The internal bias should not be relied on by design, especially in noisy environments, but should be considered a fall back protection. Special care needs to be taken when the device is used with MCUs using open drain outputs. TXD is weakly internally pulled up. The TXD pull up strength and CAN bit timing require special consideration when this device is used with an open drain TXD output on the microprocessor's CAN controller. An adequate external pull up resistor must be used to ensure that the TXD output of the microprocessor maintains adequate bit timing input to the CAN transceiver.

### 6.3.7 CAN Bus Short Circuit Current Limiting

The device has several protection features that limit the short circuit current when a CAN bus line is shorted. These include CAN driver current limiting (dominant and recessive). The device has TXD dominant time out which prevents permanently having the higher short circuit current of dominant state in case of a system fault. During CAN communication the bus switches between dominant and recessive states, thus the short circuit current may be viewed either as the current during each bus state or as a DC average current. For system current and power considerations in the termination resistors and common mode choke ratings the average short circuit current should be used. The percentage dominant is limited by the TXD dominant time out and CAN protocol which has forced state changes and recessive bits such as bit stuffing, control fields, and interframespace. These ensure there is a minimum recessive amount of time on the bus even if the data field contains a high percentage of dominant bits.

The short circuit current of the bus depends on the ratio of recessive to dominant bits and their respective short circuit currents. The average short circuit current may be calculated with the following formula:

$$
I_{\text {OS(AVG) }}=\% \text { Transmit } \times\left[\left(\% \text { REC_Bits } \times I_{\text {OS(SS)_REC }}\right)+\left(\% \text { DOM_Bits } \times I_{\text {OS(SS)_DOM }}\right)\right]+\left[\% \text { Receive } \times I_{\text {OS(SS)_REC }}\right]
$$

Where:

- $\left.I_{\text {OS(AVG) }}\right)$ is the average short circuit current
- \%Transmit is the percentage the node is transmitting CAN messages
- \%Receive is the percentage the node is receiving CAN messages
- \%REC_Bits is the percentage of recessive bits in the transmitted CAN messages
- \%DOM_Bits is the percentage of dominant bits in the transmitted CAN messages
- $\mathrm{I}_{\text {OS(SS)_REC }}$ is the recessive steady state short circuit current
- $\mathrm{I}_{\text {OS(SS)_DOM }}$ is the dominant steady state short circuit current

The short circuit current and possible fault cases of the network should be taken into consideration when sizing the power ratings of the termination resistance and other network components.

# 6.3.8 ESD Protection 

The bus pins of the TCAN33x family possess on-chip ESD protection against $\pm 25 \mathrm{kV}$ human body model (HBM) and $\pm 12 \mathrm{kV}$ IEC61000-4-2 contact discharge. The IEC-ESD test is far more severe than the HBM-ESD test. The $50 \%$ higher charge capacitance, $\mathrm{C}_{\mathrm{S}}$, and $78 \%$ lower discharge resistance, $\mathrm{R}_{\mathrm{D}}$ of the IEC model produce significantly higher discharge currents than the HBM-model.

As stated in the IEC 61000-4-2 standard, contact discharge is the preferred test method; although IEC air-gap testing is less repeatable than contact testing, air discharge protection levels are inferred from the contact discharge test results.
![img-35.jpeg](img-35.jpeg)

Figure 6-2. HBM and IEC-ESD Models and Currents in Comparison (HBM Values in Parenthesis)

### 6.3.9 Digital Inputs and Outputs

All the devices in this family are single 3.3 V nominal supply devices. The digital logic input and output levels for these devices have TTL threshold levels.# 6.4 Device Functional Modes 

### 6.4.1 CAN Bus States

The CAN bus has two logical states during operation: recessive and dominant. See Figure 6-3 and Figure 6-4.
Recessive bus state is when the high resistive internal input resistors of each node receiver bias the bus to a common mode of about 1.85 V across the bus termination resistors. Recessive is equivalent to logic high and is typically a differential voltage on the bus of about 0 V . Recessive state is also the idle state.

Dominant bus state is when the bus is driven differentially by one or more drivers. Current is induced to flow through the termination resistors and generate a differential voltage on the bus. Dominant is equivalent to logic low and is a differential voltage on the bus greater than the minimum threshold for a CAN dominant. A dominant state overwrites the recessive state.

During arbitration, multiple CAN nodes may transmit a dominant bit at the same time. In this case, the differential voltage of the bus is greater than the differential voltage of a single driver.

The host microprocessor of the CAN node uses the TXD terminal to drive the bus, and receives data from the bus on the RXD pin.

Transceivers with low power Standby Mode have a third bus state where the bus terminals are weakly biased to ground via the high resistance internal resistors of the receiver. See Figure 6-3 and Figure 6-4.
![img-36.jpeg](img-36.jpeg)

The devices have four main operating modes:

1. Normal mode (all devices)
2. Silent mode (TCAN330, TCAN337)
3. Standby mode with wake (TCAN334)
4. Shutdown mode (TCAN330, TCAN334)

Table 6-3. CAN Transceivers with Silent Mode

| S | Device MODE | DRIVER | RECEIVER | RXD PIN |
| :--: | :--: | :--: | :--: | :--: |
| HIGH | Reduced Power Silent <br> (Listen) Mode | Disabled (OFF) ${ }^{(2)}$ | Enabled (ON) | Mirrors Bus State ${ }^{(1)}$ |
| LOW/NC | Normal Mode | Enabled (ON) | Enabled (ON) |  |

(1) Mirrors bus state: low if CAN bus is dominant, high if CAN bus is recessive.
(2) See Figure 6-3 for bus state.Table 6-4. CAN Transceivers with Standby Mode with Wake

| STB | Device MODE | DRIVER | RECEIVER | RXD Terminal |
| :--: | :--: | :--: | :--: | :--: |
| HIGH | Ultra Low Current Standby Mode | Disabled (OFF) ${ }^{(2)}$ | Low Power Receiver and Bus Monitor Enabled (ON) | High (Recessive) until WUP, then filtered mirrors of Bus State ${ }^{(1)}$ |
| LOW/NC | Normal Mode | Enabled (ON) | Enabled (ON) | Mirrors Bus State ${ }^{(1)}$ |

(1) Standby Mode RXD behavior: See Figure 6-5.

Table 6-5. CAN Transceivers with Shutdown Mode

| SHDN | Device MODE | DRIVER | RECEIVER | RXD Terminal |
| :--: | :--: | :--: | :--: | :--: |
| HIGH | Lowest Current | Disabled (OFF) ${ }^{(2)}$ | Disabled (OFF) | High (Recessive) |
| LOW/NC | Normal Mode | Enabled (ON) | Enabled (ON) | Mirrors Bus State ${ }^{(1)}$ |

# 6.4.2 Normal Mode 

This is the normal operating mode of the device. The CAN driver and receiver are fully operational and CAN communication is bi-directional. The driver is translating a digital input on TXD to a differential output on CANH and CANL. The receiver is translating the differential signal from CANH and CANL to a digital output on RXD.

### 6.4.3 Silent Mode

This is the silent or receive only mode of the device. The CAN driver is disabled but the receiver is fully operational. CAN communication is unidirectional and only flows from the CAN bus through the receive path of the transceiver to the CAN protocol controller via the RXD output pin. The receiver is translating the differential signal from CANH and CANL to a digital output on RXD.

### 6.4.4 Standby Mode with Wake

This is the low power mode of the device. The CAN driver and main receiver are turned off and bi-directional CAN communication is not possible. The low power receiver and bus monitor are enabled to allow for RXD Wake Requests via the CAN bus. A wake up request will be output to RXD (driven low) as shown in Figure 6-5. The local CAN protocol microprocessor should monitor RXD for transitions (high to low) and reactivate the device to normal mode based on the RXD Wake Request. The CAN bus pins are weakly pulled to GND during this mode, see Figure 6-4.

### 6.4.5 Bus Wake via RXD Request (BWRR) in Standby Mode

The TCAN334 with low power standby mode, offers a wake up from the CAN bus mechanism called bus wake via RXD Request (BWRR) to indicate to a host microprocessor that the bus is active and it should wake up and return to normal CAN communication.

This device uses the multiple filtered dominant wake-up pattern (WUP) from ISO11898-5 to qualify bus traffic into a request to wake the host microprocessor. The bus wake request is signaled to the microprocessor by a falling edge and low corresponding to a "filtered" bus dominant on the RXD terminal (BWRR).

The wake up pattern (WUP) consists of a filtered dominant bus, then a filtered recessive bus time followed by a second filtered bus time. Once the WUP is detected the device will start issuing wake up requests (BWRR) on the RXD terminal every time a filtered dominant time is received from the bus. The first filtered dominant initiates the WUP and the bus monitor waits on a filtered recessive; other bus traffic does not reset the bus monitor. Once a filtered recessive is received, the bus monitor waits on a filtered dominant and again; other bus traffic does not reset the bus monitor. Immediately upon receiving of the second filtered dominant, the bus monitor recognizes the WUP and transitions to BWRR mode. In this mode, RXD is driven low for all dominant bits lasting for longer than $t_{\text {WK_FILTER. }}$ The RXD output during BWRR matches the classical 8-pin CAN devices, such as the TCANA1040A-Q1 device, that used the single filtered dominant on the bus as the wake up request mechanism from ISO11898-5.

For a dominant or recessive to be considered filtered, the bus must be in that state for more than $t_{\text {WK_FILTER }}$ time. Due to variability in the $t_{\text {WK_FILTER }}$ the following scenarios are applicable. Bus state times less than $t_{\text {WK_FILTER(MIN) }}$ are never detected as part of a WUP and thus no BWRR is generated. Bus state times between$t_{\text {WK_FILTER(MIN) }}$ and $t_{\text {WK_FILTER(MAX) }}$ may be detected as part of a WUP and a BWRR may be generated. Bus state times more than $t_{\text {WK_FILTER(MAX) }}$ are always detected as part of a WUP and thus a BWRR is always generated.
See Figure 6-5 for the timing diagram of the WUP. The pattern, $t_{\text {WK_FILTER }}$ time used for the WUP and BWRR prevent noise and bus stuck dominant faults from causing false wake requests. If the device is switched to normal mode, or an under voltage event occurs on $\mathrm{V}_{\mathrm{CC}}$ the BWRR will be lost.
![img-37.jpeg](img-37.jpeg)

Figure 6-5. Wake Up Pattern (WUP) and Bus Wake via RXD Request (BWRR)

# 6.4.6 Shutdown Mode 

This is the lowest power mode of all of the devices. The CAN driver and receiver are turned off and bi-directional CAN communication is not possible. It is not possible to receive a remote wake request via the CAN bus in this mode. The CAN bus pins are pulled to GND during this mode as shown in Figure 6-3.# 6.4.7 Driver and Receiver Function Tables 

Table 6-6. Driver Function Table

| DEVICE MODE | TXD $^{(1)}$ INPUT | BUS OUTPUTS ${ }^{(2)}$ |  | DRIVEN BUS STATE ${ }^{(3)}$ |
| :--: | :--: | :--: | :--: | :--: |
|  |  | CANH | CANL |  |
| Normal | L | H | L | Dominant |
|  | H or Open | Z | Z | Biased Recessive |
| Silent | X | Z | Z | Biased Recessive |
| Standby | X | Z | Z | Weak Pull to GND |
| Shutdown | X | Z | Z | Weak Pull to GND |

(1) $\mathrm{H}=$ high level, $\mathrm{L}=$ low level, $\mathrm{X}=$ irrelevant.
(2) $\mathrm{H}=$ high level, $\mathrm{L}=$ low level, $\mathrm{Z}=$ high Z receiver bias.
(3) For Bus state and bias see Figure 6-3 and Figure 6-4.

Table 6-7. Receiver Function Table Normal and Standby Modes

| DEVICE MODE | CAN DIFFERENTIAL INPUTS $\mathrm{V}_{(\mathrm{ID})}=\mathrm{V}_{\text {(CANH) }}-\mathrm{V}_{\text {(CANL) }}$ | BUS STATE | RXD PIN ${ }^{(1)}$ |
| :--: | :--: | :--: | :--: |
| Normal or Silent | $\mathrm{V}_{(\mathrm{ID})} \geq 0.9 \mathrm{~V}$ | Dominant | L |
|  | $0.5 \mathrm{~V}<\mathrm{V}_{(\mathrm{ID})}<0.9 \mathrm{~V}$ | ? | ? |
|  | $\mathrm{V}_{(\mathrm{ID})} \leq 0.5 \mathrm{~V}$ | Recessive | H |
| Standby | $\mathrm{V}_{(\mathrm{ID})} \geq 1.15 \mathrm{~V}$ | Dominant | See Figure 6-5 |
|  | $0.4 \mathrm{~V}<\mathrm{V}_{(\mathrm{ID})}<1.15 \mathrm{~V}$ | ? |  |
|  | $\mathrm{V}_{(\mathrm{ID})} \leq 0.4 \mathrm{~V}$ | Recessive |  |
| Shutdown | Any | Recessive | H |
| Any | Open $\left(\mathrm{V}_{(\mathrm{ID})} \approx 0 \mathrm{~V}\right)$ | Open | H |

(1) $\mathrm{I}=$ high level, $\mathrm{L}=$ low level, $?=$ indeterminate.# 7 Application Information Disclaimer 

## Note

Information in the following applications sections is not part of the TI component specification, and TI does not warrant its accuracy or completeness. TI's customers are responsible for determining suitability of components for their purposes, as well as validating and testing their design implementation to confirm system functionality.

### 7.1 Application Information

### 7.1.1 Bus Loading, Length and Number of Nodes

The ISO 11898 standard specifies a data rate up to 1 Mbps , maximum CAN bus cable length of 40 m , maximum drop line (stub) length of 0.3 m and a maximum of 30 nodes. However, with careful network design, the system may have longer cables, longer stub lengths, and many more nodes to a bus. Many CAN organizations and standards have scaled the use of CAN for applications outside the original ISO 11898 standard. They have made system level trade-offs for data rate, cable length, and parasitic loading of the bus. Examples of some of these specifications are ARINC825, CANopen, CAN Kingdom, DeviceNet and NMEA200.

A high number of nodes requires a transceiver with high input impedance and wide common mode range such as the TCAN33x CAN family. ISO 11898-2 specifies the driver differential output with a $60 \Omega$ load (two $120 \Omega$ termination resistors in parallel) and the differential output must be greater than 1.5 V . The TCAN33x devices are specified to meet the 1.5 V requirement with a $50 \Omega$ load across a common mode range of -12 V to 12 V through a $330 \Omega$ coupling network. This network represents the bus loading of 120 TCAN33x transceivers based on their minimum differential input resistance of $40 \mathrm{k} \Omega$.

For CAN network design, margin must be given for signal loss across the system and cabling, parasitic loadings, network imbalances, ground offsets and signal integrity, thus a practical maximum number of nodes may be lower. Bus length may also be extended beyond the original ISO 11898 standard of 40 m by careful system design and data rate tradeoffs. For example, CANopen network design guidelines allow the network to be up to 1 km with changes in the termination resistance, cabling, number of nodes and data rate.

This flexibility in CAN network design is one of the key strengths of the various extensions and additional standards that have been built on the original ISO 11898 CAN standard.

### 7.2 Typical Application

![img-38.jpeg](img-38.jpeg)

Copyright © 2016, Texas Instruments Incorporated
Figure 7-1. Typical 3.3V Application# 7.2.1 Design Requirements 

### 7.2.1.1 CAN Termination

The ISO 11898 standard specifies the interconnect to be a twisted-pair cable (shielded or unshielded) with $120 \Omega$ characteristic impedance $\left(\mathrm{Z}_{\mathrm{O}}\right)$. Resistors equal to the characteristic impedance of the line should be used to terminate both ends of the cable to prevent signal reflections. Unterminated drop lines (stubs) connecting nodes to the bus should be kept as short as possible to minimize signal reflections. The termination may be on the cable or in a node, but if nodes may be removed from the bus the termination must be carefully placed so that it is not removed from the bus.

### 7.2.2 Detailed Design Procedure

Termination is typically a $120 \Omega$ resistor at each end of the bus. If filtering and stabilization of the common mode voltage of the bus is desired, then split termination may be used. Split termination uses two $60 \Omega$ resistors with a capacitor in the middle of these resistors to ground. Split termination improves the electromagnetic emissions behavior of the network by eliminating fluctuations in the bus common mode voltages at the start and end of message transmissions.

Care should be taken in the power ratings of the termination resistors used. Typically the worst case condition would be if the system power supply was shorted across the termination resistance to ground. In most cases, the current flow through the resistor in this condition is much higher than the transceiver current limit.
![img-39.jpeg](img-39.jpeg)

Figure 7-2. Typical CAN Bus
Standard Termination
![img-40.jpeg](img-40.jpeg)

Split Termination
![img-41.jpeg](img-41.jpeg)

Figure 7-3. CAN Bus Termination Concepts# 7.2.3 Application Curves 

![img-42.jpeg](img-42.jpeg)

### 7.3 System Examples

### 7.3.1 ISO11898 Compliance of TCAN33x Family of 3.3V CAN Transceivers Introduction

Many users value the low power consumption of operating their CAN transceivers from a 3.3 V supply. However, some are concerned about the interoperability with 5 V supplied transceivers on the same bus. This report analyzes this situation to address those concerns.# 7.3.2 Differential Signal 

CAN is a differential bus where complementary signals are sent over two wires and the voltage difference between the two wires defines the logical state of the bus. The differential CAN receiver monitors this voltage difference and outputs the bus state with a single ended logic level output signal.
![img-43.jpeg](img-43.jpeg)

Figure 7-6. Typical Differential Output Waveform
The CAN driver creates the differential voltage between CANH and CANL in the dominant state. The dominant differential output of the TCAN33x is greater than 1.5 V and less than 3 V across a $60 \Omega$ load as defined by the ISO11898 standard. These are the same limiting values for 5 V supplied CAN transceivers. The bus termination resistors drive the recessive bus state and not the CAN driver.

A CAN receiver is required to output a recessive state when less than 500 mV of differential voltage exists on the bus, and a dominant state when more than 900 mV of differential voltage exists on the bus. The CAN receiver must do this with common-mode input voltages from -2 V to 7 V . The TCAN33x family receivers meet these same input specifications as 5 V supplied receivers.

### 7.3.3 Common-Mode Signal and EMC Performance

A common-mode signal is an average voltage of the two signal wires that the differential receiver rejects. The common-mode signal comes from the CAN driver, ground noise, and coupled bus noise. Since the bias voltage of the recessive state of the device is dependent on $\mathrm{V}_{\mathrm{CC}}$, any noise present or variation of $\mathrm{V}_{\mathrm{CC}}$ has an effect on this bias voltage seen by the bus. The TCAN33x family has the recessive bias voltage set higher than $0.5 \times \mathrm{V}_{\mathrm{CC}}$ to match common mode in recessive mode to dominant mode. This results in superior EMC performance.

### 7.4 Power Supply Recommendations

For reliable operation at all data rates and supply voltages, each supply should be decoupled with a 100 nF ceramic capacitor located as close to the $\mathrm{V}_{\mathrm{CC}}$ supply pins as possible. The TPS76333 is a linear voltage regulator suitable for the 3.3 V supply.

### 7.5 Layout

### 7.5.1 Layout Guidelines

TCAN33x family of devices incorporates integrated IEC 61000-4-2 ESD protection. Should the system requires additional protection against ESD, EFT or surge, additional external protection and filtering circuitry may be needed.

In order for the PCB design to be successful, start with design of the protection and filtering circuitry. Because ESD and EFT transients have a wide frequency bandwidth from approximately 3 MHz to 3 GHz , high frequency layout techniques must be applied during PCB design.Design the bus protection components in the direction of the signal path. Do not force the transient current to divert from the signal path to reach the protection device. Below is a list of layout recommendations when designing a CAN transceiver into an application.

- Transient Protection on CANH and CANL: Transient Voltage Suppression (TVS) and capacitors (D1, C5 and C7 shown in Figure 7-7) can be used for additional system level protection. These devices must be placed as close to the connector as possible. This prevents the transient energy and noise from penetrating into other nets on the board.
- Bus Termination on CANH and CANL: Figure 7-7 shows split termination where the termination is split into two resistors, R5 and R6, with the center or split tap of the termination connected to ground through capacitor C6. Split termination provides common mode filtering for the bus. When termination is placed on the board instead of directly on the bus, care must be taken to ensure the terminating node is not removed from the bus, as this causes signal integrity issues if the bus is not properly terminated on both ends.
- Decoupling Capacitors on $\mathrm{V}_{\mathrm{CC}}$ : Bypass and bulk capacitors must be placed as close as possible to the supply pins of transceiver (examples are C 2 and C 3 ).
- Ground and power connections: Use at least two vias for $\mathrm{V}_{\mathrm{CC}}$ and ground connections of bypass capacitors and protection devices to minimize trace and via inductance.
- Digital inputs and outputs: To limit current of digital lines, serial resistors may be used. Examples are R1, R2, R3 and R4.
- Filtering noise on digital inputs and outputs: To filter noise on the digital I/O lines, a capacitor may be used close to the input side of the I/O as shown by C1, C8 and C4.
- Fault Output Pin (TCAN337 only): Because the FAULT output pin is an open drain output, an external pullup resistor is required to pull the pin voltage high for normal operation (R7).
- TXD input pin: If an open-drain host processor is used to drive the TXD pin of the device, an external pullup resistor between $1 \mathrm{k} \Omega$ and $10 \mathrm{k} \Omega$ must be used to help drive the recessive input state of the device (weak internal pullup resistor).


# 7.5.2 Layout Example 

![img-44.jpeg](img-44.jpeg)

Figure 7-7. Layout Example# Device and Documentation Support 

## 1 Receiving Notification of Documentation Updates

To receive notification of documentation updates, navigate to the device product folder on ti.com. Click on Notifications to register and receive a weekly digest of any product information that has changed. For change details, review the revision history included in any revised document.

## 2 Support Resources

TI E2E ${ }^{\text {TM }}$ support forums are an engineer's go-to source for fast, verified answers and design help - straight from the experts. Search existing answers or ask your own question to get the quick design help you need.
Linked content is provided "AS IS" by the respective contributors. They do not constitute TI specifications and do not necessarily reflect TI's views; see TI's Terms of Use.

## 3 Trademarks

TI E2E ${ }^{\text {TM }}$ is a trademark of Texas Instruments.
All trademarks are the property of their respective owners.

## 4 Electrostatic Discharge Caution

This integrated circuit can be damaged by ESD. Texas Instruments recommends that all integrated circuits be handled with appropriate precautions. Failure to observe proper handling and installation procedures can cause damage.
ESD damage can range from subtle performance degradation to complete device failure. Precision integrated circuits may be more susceptible to damage because very small parametric changes could cause the device not to meet its published specifications.

## 5 Glossary

TI Glossary This glossary lists and explains terms, acronyms, and definitions.

## 8 Revision History

Changes from Revision E (December 2019) to Revision F (May 2025) Page

- Changed the Device Information table to the Package Information table. 1

Changes from Revision D (April 2016) to Revision E (December 2019) Page

- Changed the Pin Configuration image appearance. 4
- Changed the titles of Figure 6-4 and Figure 6-5 3

Changes from Revision C (April 2016) to Revision D (April 2016) Page

- Changed From: ARNIC825 To ARINC825 in the Section 2 list. 1

Changes from Revision B (April 2016) to Revision C (April 2016) Page

- Removed the Preview Note from TCAN337 and TCAN337G in the Device Options table. 3

Changes from Revision A (January 2016) to Revision B (April 2016) Page

- Removed the Preview Note from all device except for TCAN337 and TCAN337G in the Device Comparison table. 3- Changed FAULT Pin $\mathrm{I}_{\mathrm{CL}}$ MIN value From: 5 mA To: 4 mA in the Section 5.5 6

Changes from Revision * (December 2015) to Revision A (January 2016)
Page

- Changed Section 1 From: "Total Loop Delay < 150ns" To: "Total loop delay < 135ns" .................................... 1
- Changed $\mathrm{V}_{\text {IT(SLEEP) }}$ To: $\mathrm{V}_{\text {IT(STB) }}$ and added Test conditions in the Section 5.5 ................................................ 6
- Added $-12 \mathrm{~V}<\mathrm{V}_{\mathrm{CM}}<12 \mathrm{~V}$ to $\mathrm{t}_{\text {WK_FILTER }}$ in the Test Conditions of Section 5.6 ................................................ 9


# 9 Mechanical, Packaging, and Orderable Information 

The following pages include mechanical, packaging, and orderable information. This information is the most current data available for the designated devices. This data is subject to change without notice and revision of this document. For browser-based versions of this data sheet, refer to the left-hand navigation.# PACKAGE OPTION ADDENDUM

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
|  TCAN330D | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330D.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330DCNR | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 330  |
|  TCAN330DCNR.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 330  |
|  TCAN330DCNT | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 330  |
|  TCAN330DCNT.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 330  |
|  TCAN330DR | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330DR.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330GD | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330GD.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330GDCNR | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 330  |
|  TCAN330GDCNR.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 330  |
|  TCAN330GDCNT | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 330  |
|  TCAN330GDCNT.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 330  |
|  TCAN330GDG4 | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330GDG4.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330GDR | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN330GDR.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC330  |
|  TCAN332D | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332D.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332DCNR | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332DCNR.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332DCNT | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332DCNT.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332DCNTG4 | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332DCNTG4.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332DR | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332DR.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332DRG4 | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  ||  Orderable part number | Status
(1) | Material type
(2) | Package | Pins | Package qty | Carrier | RoHS
(3) | Lead finish/
Ball material
(4) | MSL rating/
Peak reflow
(5) | Op temp ( ${ }^{\circ} \mathrm{C}$ ) | Part marking
(6)  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  TCAN332DRG4.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332GD | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332GD.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332GDCNR | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332GDCNR.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332GDCNRG4 | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332GDCNRG4.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332GDCNT | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332GDCNT.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 332  |
|  TCAN332GDR | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332GDR.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332GDRG4 | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN332GDRG4.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC332  |
|  TCAN334D | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334D.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334DCNR | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334DCNR.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334DCNRG4 | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334DCNRG4.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334DCNT | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334DCNT.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334DR | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334DR.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334DRG4 | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334DRG4.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334GD | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334GD.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334GDCNR | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334GDCNR.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334GDCNT | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334GDCNT.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  ||  Orderable part number | Status
(1) | Material type
(2) | Package | Pins | Package qty | Carrier | RoHS
(3) | Lead finish/
Ball material
(4) | MSL rating/
Peak reflow
(5) | Op temp ( ${ }^{\circ} \mathrm{C}$ ) | Part marking
(6)  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  TCAN334GDCNTG4 | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334GDCNTG4.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 334  |
|  TCAN334GDR | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN334GDR.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC334  |
|  TCAN337D | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337D.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337DCNR | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337DCNR.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337DCNT | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337DCNT.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337DR | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337DR.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337GD | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337GD.B | Active | Production | SOIC (D) | 8 | 75 | TUBE | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337GDCNR | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337GDCNR.B | Active | Production | SOT-23 (DCN) | 8 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337GDCNT | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337GDCNT.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337GDCNTG4 | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337GDCNTG4.B | Active | Production | SOT-23 (DCN) | 8 | 250 | SMALL T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | 337  |
|  TCAN337GDR | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337GDR.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337GDRG4 | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |
|  TCAN337GDRG4.B | Active | Production | SOIC (D) | 8 | 2500 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 125 | TC337  |

${ }^{(1)}$ Status: For more details on status, see our product life cycle. ${ }^{(2)}$ Material type: When designated, preproduction parts are prototypes/experimental devices, and are not yet approved or released for full production. Testing and final process, including without limitation quality assurance, reliability performance testing, and/or process qualification, may not yet be complete, and this item is subject to further changes or possible discontinuation. If available for ordering, purchases will be subject to an additional waiver at checkout, and are intended for early internal evaluation purposes only. These items are sold without warranties of any kind. ${ }^{(3)}$ RoHS values: Yes, No, RoHS Exempt. See the TI RoHS Statement for additional information and value definition.${ }^{(4)}$ Lead finish/Ball material: Parts may have multiple material finish options. Finish options are separated by a vertical ruled line. Lead finish/Ball material values may wrap to two lines if the finish value exceeds the maximum column width.
${ }^{(5)}$ MSL rating/Peak reflow: The moisture sensitivity level ratings and peak solder (reflow) temperatures. In the event that a part has multiple moisture sensitivity ratings, only the lowest level per JEDEC standards is shown. Refer to the shipping label for the actual reflow temperature that will be used to mount the part to the printed circuit board.
${ }^{(6)}$ Part marking: There may be an additional marking, which relates to the logo, the lot trace code information, or the environmental category of the part.

Multiple part markings will be inside parentheses. Only one part marking contained in parentheses and separated by a "-" will appear on a part. If a line is indented then it is a continuation of the previous line and the two combined represent the entire part marking for that device.

Important Information and Disclaimer:The information provided on this page represents TI's knowledge and belief as of the date that it is provided. TI bases its knowledge and belief on information provided by third parties, and makes no representation or warranty as to the accuracy of such information. Efforts are underway to better integrate information from third parties. TI has taken and continues to take reasonable steps to provide representative and accurate information but may not have conducted destructive testing or chemical analysis on incoming materials and chemicals. TI and TI suppliers consider certain information to be proprietary, and thus CAS numbers and other limited information may not be available for release.

In no event shall TI's liability arising out of such information exceed the total purchase price of the TI part(s) at issue in this document sold by TI to Customer on an annual basis.# TAPE AND REEL INFORMATION 

![img-45.jpeg](img-45.jpeg)

TAPE DIMENSIONS
![img-46.jpeg](img-46.jpeg)

| A0 | Dimension designed to accommodate the component width |
| :-- | :-- |
| B0 | Dimension designed to accommodate the component length |
| K0 | Dimension designed to accommodate the component thickness |
| W | Overall width of the carrier tape |
| P1 | Pitch between successive cavity centers |

QUADRANT ASSIGNMENTS FOR PIN 1 ORIENTATION IN TAPE
![img-47.jpeg](img-47.jpeg)
*All dimensions are nominal

| Device | Package <br> Type | Package <br> Drawing | Pins | SPQ | Reel <br> Diameter <br> (mm) | Reel <br> Width <br> W1 (mm) | A0 <br> (mm) | B0 <br> (mm) | K0 <br> (mm) | P1 <br> (mm) | W <br> (mm) | Pin1 <br> Quadrant |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TCAN330DCNR | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN330DCNT | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN330DR | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN330GDCNR | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN330GDCNT | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN330GDR | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN332DCNR | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN332DCNT | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN332DCNTG4 | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN332DR | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN332DRG4 | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN332GDCNR | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN332GDCNRG4 | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN332GDCNT | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN332GDR | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN332GDRG4 | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 || Device | Package <br> Type | Package <br> Drawing | Pins | SPQ | Reel <br> Diameter <br> (mm) | Reel <br> Width <br> W1 (mm) | A0 <br> (mm) | B0 <br> (mm) | K0 <br> (mm) | P1 <br> (mm) | W <br> (mm) | Pin1 <br> Quadrant |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TCAN334DCNR | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN334DCNRG4 | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN334DCNT | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN334DR | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN334DRG4 | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN334GDCNR | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN334GDCNT | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN334GDCNTG4 | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN334GDR | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN337DCNR | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN337DCNT | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN337DR | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN337GDCNR | SOT-23 | DCN | 8 | 3000 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN337GDCNT | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN337GDCNTG4 | SOT-23 | DCN | 8 | 250 | 180.0 | 8.4 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| TCAN337GDR | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |
| TCAN337GDRG4 | SOIC | D | 8 | 2500 | 330.0 | 12.4 | 6.4 | 5.2 | 2.1 | 8.0 | 12.0 | Q1 |# **PACKAGE MATERIALS INFORMATION**

![img-48.jpeg](img-48.jpeg)

|  *All dimensions are nominal |  |  |  |  |  |  |   |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  Device | Package Type | Package Drawing | Pins | SPQ | Length (mm) | Width (mm) | Height (mm)  |
|  TCAN330DCNR | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0  |
|  TCAN330DCNT | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0  |
|  TCAN330DR | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0  |
|  TCAN330GDCNR | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0  |
|  TCAN330GDCNT | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0  |
|  TCAN330GDR | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0  |
|  TCAN332DCNR | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0  |
|  TCAN332DCNT | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0  |
|  TCAN332DCNTG4 | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0  |
|  TCAN332DR | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0  |
|  TCAN332DRG4 | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0  |
|  TCAN332GDCNR | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0  |
|  TCAN332GDCNRG4 | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0  |
|  TCAN332GDCNT | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0  |
|  TCAN332GDR | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0  |
|  TCAN332GDRG4 | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0  |
|  TCAN334DCNR | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0  |
|  TCAN334DCNRG4 | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0  || Device | Package Type | Package Drawing | Pins | SPQ | Length (mm) | Width (mm) | Height (mm) |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TCAN334DCNT | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0 |
| TCAN334DR | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0 |
| TCAN334DRG4 | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0 |
| TCAN334GDCNR | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0 |
| TCAN334GDCNT | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0 |
| TCAN334GDCNTG4 | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0 |
| TCAN334GDR | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0 |
| TCAN337DCNR | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0 |
| TCAN337DCNT | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0 |
| TCAN337DR | SOIC | D | 8 | 2500 | 340.5 | 336.1 | 25.0 |
| TCAN337GDCNR | SOT-23 | DCN | 8 | 3000 | 202.0 | 201.0 | 28.0 |
| TCAN337GDCNT | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0 |
| TCAN337GDCNTG4 | SOT-23 | DCN | 8 | 250 | 202.0 | 201.0 | 28.0 |
| TCAN337GDR | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0 |
| TCAN337GDRG4 | SOIC | D | 8 | 2500 | 353.0 | 353.0 | 32.0 |TUBE
![img-49.jpeg](img-49.jpeg)
"All dimensions are nominal

| Device | Package Name | Package Type | Pins | SPQ | L (mm) | W (mm) | T ( $\mu \mathrm{m}$ ) | B (mm) |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TCAN330D | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN330D.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN330GD | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN330GD.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN330GD4 | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN330GD4.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN332D | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN332D.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN332GD | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN332GD.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN334D | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN334D.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN334GD | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN334GD.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN337D | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN337D.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN337GD | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |
| TCAN337GD.B | D | SOIC | 8 | 75 | 507 | 8 | 3940 | 4.32 |![img-50.jpeg](img-50.jpeg)

# NOTES: 

1. Linear dimensions are in inches [millimeters]. Dimensions in parenthesis are for reference only. Controlling dimensions are in inches. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.
3. This dimension does not include mold flash, protrusions, or gate burrs. Mold flash, protrusions, or gate burrs shall not exceed $.006[0.15]$ per side.
4. This dimension does not include interlead flash.
5. Reference JEDEC registration MS-012, variation AA.![img-51.jpeg](img-51.jpeg)

NOTES: (continued)
6. Publication IPC-7351 may have alternate designs.
7. Solder mask tolerances between and around signal pads can vary based on board fabrication site.![img-52.jpeg](img-52.jpeg)

NOTES: (continued)
8. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.
9. Board assembly site may have different recommendations for stencil design.# DCN (R-PDSO-G8) PLASTIC SMALL-OUTLINE PACKAGE (DIE DOWN) 

![img-53.jpeg](img-53.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Package outline exclusive of metal burr \& dambar protrusion/intrusion.
D. Package outline inclusive of solder plating.
E. A visual index feature must be located within the Pin 1 index area.
F. Falls within JEDEC MO-178 Variation BA.
G. Body dimensions do not include flash or protrusion. Mold flash and protrusion shall not exceed 0.25 per side.# DCN (R-PDSO-G8) PLASTIC SMALL-OUTLINE PACKAGE (DIE DOWN) 

![img-54.jpeg](img-54.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Publication IPC-7351 is recommended for alternate designs.
D. Laser cutting apertures with trapezoidal walls and also rounding corners will offer better paste release. Customers should contact their board assembly site for stencil design recommendations. Refer to IPC-7525.
E. Customers should contact their board fabrication site for solder mask tolerances between and around signal pads.# IMPORTANT NOTICE AND DISCLAIMER 

TI PROVIDES TECHNICAL AND RELIABILITY DATA (INCLUDING DATA SHEETS), DESIGN RESOURCES (INCLUDING REFERENCE DESIGNS), APPLICATION OR OTHER DESIGN ADVICE, WEB TOOLS, SAFETY INFORMATION, AND OTHER RESOURCES "AS IS" AND WITH ALL FAULTS, AND DISCLAIMS ALL WARRANTIES, EXPRESS AND IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE OR NON-INFRINGEMENT OF THIRD PARTY INTELLECTUAL PROPERTY RIGHTS.
These resources are intended for skilled developers designing with TI products. You are solely responsible for (1) selecting the appropriate TI products for your application, (2) designing, validating and testing your application, and (3) ensuring your application meets applicable standards, and any other safety, security, regulatory or other requirements.
These resources are subject to change without notice. TI grants you permission to use these resources only for development of an application that uses the TI products described in the resource. Other reproduction and display of these resources is prohibited. No license is granted to any other TI intellectual property right or to any third party intellectual property right. TI disclaims responsibility for, and you will fully indemnify TI and its representatives against, any claims, damages, costs, losses, and liabilities arising out of your use of these resources.
TI's products are provided subject to TI's Terms of Sale or other applicable terms available either on ti.com or provided in conjunction with such TI products. TI's provision of these resources does not expand or otherwise alter TI's applicable warranties or warranty disclaimers for TI products.
TI objects to and rejects any additional or different terms you may have proposed.
Mailing Address: Texas Instruments, Post Office Box 655303, Dallas, Texas 75265
Copyright © 2025, Texas Instruments Incorporated