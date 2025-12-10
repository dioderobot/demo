# SN74LVC1G14 Single Schmitt-Trigger Inverter 

## 1 Features

- Available in the Texas Instruments NanoFree ${ }^{\text {TM }}$ Package
- Supports $5-\mathrm{V} \mathrm{V}_{\mathrm{CC}}$ Operation
- Inputs Accept Voltages to 5.5 V
- Max $t_{\text {pd }}$ of 4.6 ns at 3.3 V
- Low Power Consumption, 10- $\mu \mathrm{A}$ Max $\mathrm{I}_{\mathrm{CC}}$
- $\pm 24-\mathrm{mA}$ Output Drive at 3.3 V
- $\mathrm{I}_{\text {off }}$ Supports Partial-Power-Down Mode Operation
- Latch-Up Performance Exceeds 100 mA Per JESD 78, Class II
- ESD Protection Exceeds JESD 22
- 2000-V Human-Body Model (A114-A)
- 200-V Machine Model (A115-A)
- 1000-V Charged-Device Model (C101)


## 2 Applications

- AV Receiver
- Audio Dock: Portable
- Blu-ray Player and Home Theater
- Embedded PC
- MP3 Player/Recorder (Portable Audio)
- Personal Digital Assistant (PDA)
- Power: Telecom/Server AC/DC Supply: Single Controller: Analog and Digital
- Solid State Drive (SSD): Client and Enterprise
- TV: LCD/Digital and High-Definition (HDTV)
- Tablet: Enterprise
- Video Analytics: Server
- Wireless Headset, Keyboard, and Mouse
![img-0.jpeg](img-0.jpeg)
N.C. - No internal connection

See mechanical drawings for dimensions.

## 3 Description

This single Schmitt-trigger inverter is designed for $1.65-\mathrm{V}$ to $5.5-\mathrm{V} \mathrm{V}_{\mathrm{CC}}$ operation.
The SN74LVC1G14 device contains one inverter and performs the Boolean function $Y=\bar{A}$. The device functions as an independent inverter, but because of Schmitt action, it may have different input threshold levels for positive-going $\left(V_{T+}\right)$ and negative-going $\left(V_{T-}\right)$ signals.
NanoFree ${ }^{\text {TM }}$ package technology is a major breakthrough in IC packaging concepts, using the die as the package.
This device is fully specified for partial-power-down applications using $I_{\text {off }}$. The $I_{\text {off }}$ circuitry disables the outputs, preventing damaging current backflow through the device when it is powered down.

Device Information

| ORDER NUMBER | PACKAGE | BODY SIZE |
| :--: | :--: | :--: |
| SN74LVC1G14DBV | SOT-23 (5) | $2,9 \mathrm{~mm} \times 1,6 \mathrm{~mm}$ |
| SN74LVC1G14DCK | SC70 (5) | $2,0 \mathrm{~mm} \times 1,25 \mathrm{~mm}$ |
| SN74LVC1G14DRL | SOT (5) | $1,6 \mathrm{~mm} \times 1,2 \mathrm{~mm}$ |
| SN74LVC1G14DRY | SON (6) | $1,45 \mathrm{~mm} \times 1,0 \mathrm{~mm}$ |

![img-1.jpeg](img-1.jpeg)# Table of Contents 

1 Features ..... 1
2 Applications ..... 1
3 Description ..... 1
4 Revision History ..... 2
5 Terminal Configuration and Functions ..... 3
6 Specifications ..... 4
6.1 Absolute Maximum Ratings ..... 4
6.2 Handling Ratings ..... 4
6.3 Recommended Operating Conditions ..... 4
6.4 Electrical Characteristics ..... 5
6.5 Switching Characteristics ..... 6
6.6 Switching Characteristics ..... 6
6.7 Operating Characteristics ..... 6
7 Parameter Measurement Information ..... 7
8 Device and Documentation Support ..... 9
8.1 Trademarks ..... 9
8.2 Electrostatic Discharge Caution ..... 9
8.3 Glossary ..... 9
9 Mechanical, Packaging, and Orderable Information ..... 9

## 4 Revision History

Changes from Revision V (November 20112) to Revision W
Page

- Added DPW Package ..... 1
- Added Applications ..... 1
- Moved $\mathrm{T}_{\text {sig }}$ to Handling Ratings table ..... 4# 5 Terminal Configuration and Functions 

![img-2.jpeg](img-2.jpeg)
N.C. - No internal connection

See mechanical drawings for dimensions.

YZP PACKAGE
(TOP VIEW)
![img-3.jpeg](img-3.jpeg)

YZP Package Terminal Assignments

|  | 1 | 2 |
| :--: | :--: | :--: |
| A | DNU | $\mathrm{V}_{\mathrm{CC}}$ |
| B | A | No ball |
| C | GND | Y |

YZV PACKAGE
(TOP VIEW)
![img-4.jpeg](img-4.jpeg)

YZV Package Terminal Assignments

|  | 1 | 2 |
| :--: | :--: | :--: |
| A | A | $\mathrm{V}_{\mathrm{CC}}$ |
| B | GND | Y |

Function Table

| INPUT | OUTPUT |
| :--: | :--: |
| A | Y |
| H | L |
| L | H |

Logic Diagram (Positive Logic)
(DBV, DCK, DRL, DRY, and YZP Package)
Logic Diagram (Positive Logic)
(YZV Package)
A $\xrightarrow{2} \quad \rightarrow \quad \rightarrow \quad$ Y

A $\xrightarrow{1} \rightarrow \rightarrow \rightarrow \quad$ Y# 6 Specifications 

### 6.1 Absolute Maximum Ratings ${ }^{(1)}$

over operating free-air temperature range (unless otherwise noted)

|  |  |  | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{CC}}$ | Supply voltage range |  | $-0.5$ | 6.5 | V |
| $V_{I}$ | Input voltage range ${ }^{(2)}$ |  | $-0.5$ | 6.5 | V |
| $V_{O}$ | Voltage range applied to any output in the high-impedance or power-off state ${ }^{(2)}$ |  | $-0.5$ | 6.5 | V |
| $V_{O}$ | Voltage range applied to any output in the high or low state ${ }^{(2)}$ (3) |  | $-0.5$ | $\mathrm{V}_{\mathrm{CC}}+0.5$ | V |
| $\mathrm{I}_{\text {IK }}$ | Input clamp current | $V_{I}<0$ |  | $-50$ | mA |
| $\mathrm{I}_{\mathrm{OK}}$ | Output clamp current | $V_{O}<0$ |  | $-50$ | mA |
| $\mathrm{I}_{\mathrm{O}}$ | Continuous output current |  |  | $\pm 50$ | mA |
|  | Continuous current through $\mathrm{V}_{\mathrm{CC}}$ or GND |  |  | $\pm 100$ | mA |
| $\theta_{\text {JA }}$ | Package thermal impedance ${ }^{(4)}$ | DBV package |  | 206 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
|  |  | DCK package |  | 252 |  |
|  |  | DRL package |  | 142 |  |
|  |  | DRY package |  | 234 |  |
|  |  | YZP package |  | 132 |  |
|  |  | YZV package |  | 123 |  |

(1) Stresses beyond those listed under "absolute maximum ratings" may cause permanent damage to the device. These are stress ratings only, and functional operation of the device at these or any other conditions beyond those indicated under "recommended operating conditions" is not implied. Exposure to absolute-maximum-rated conditions for extended periods may affect device reliability.
(2) The input and output negative-voltage ratings may be exceeded if the input and output current ratings are observed.
(3) The value of $\mathrm{V}_{\mathrm{CC}}$ is provided in the recommended operating conditions table.
(4) The package thermal impedance is calculated in accordance with JESD 51-7.

### 6.2 Handling Ratings

| PARAMETER |  | DEFINITION | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $T_{\text {stg }}$ |  | Storage temperature range | $-65$ | 150 | ${ }^{\circ} \mathrm{C}$ |

### 6.3 Recommended Operating Conditions ${ }^{(1)}$

|  |  |  | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{CC}}$ | Supply voltage | Operating | 1.65 | 5.5 | V |
|  |  | Data retention only | 1.5 |  |  |
| $V_{I}$ | Input voltage |  | 0 | 5.5 | V |
| $V_{O}$ | Output voltage |  | 0 | $\mathrm{V}_{\mathrm{CC}}$ | V |
| $\mathrm{I}_{\mathrm{OH}}$ | High-level output current | $\mathrm{V}_{\mathrm{CC}}=1.65 \mathrm{~V}$ |  | $-4$ | mA |
|  |  | $\mathrm{V}_{\mathrm{CC}}=2.3 \mathrm{~V}$ |  | $-8$ |  |
|  |  | $\mathrm{V}_{\mathrm{CC}}=3 \mathrm{~V}$ |  | $-16$ |  |
|  |  |  |  | $-24$ |  |
|  |  | $\mathrm{V}_{\mathrm{CC}}=4.5 \mathrm{~V}$ |  | $-32$ |  |
| $\mathrm{I}_{\mathrm{OL}}$ | Low-level output current | $\mathrm{V}_{\mathrm{CC}}=1.65 \mathrm{~V}$ |  | 4 | mA |
|  |  | $\mathrm{V}_{\mathrm{CC}}=2.3 \mathrm{~V}$ |  | 8 |  |
|  |  | $\mathrm{V}_{\mathrm{CC}}=3 \mathrm{~V}$ |  | 16 |  |
|  |  |  |  | 24 |  |
|  |  | $\mathrm{V}_{\mathrm{CC}}=4.5 \mathrm{~V}$ |  | 32 |  |
| $T_{A}$ | Operating free-air temperature |  | $-40$ | 85 | ${ }^{\circ} \mathrm{C}$ |

(1) All unused inputs of the device must be held at $\mathrm{V}_{\mathrm{CC}}$ or GND to ensure proper device operation. Refer to the TI application report, Implications of Slow or Floating CMOS Inputs, literature number SCBA004.# 6.4 Electrical Characteristics 

over recommended operating free-air temperature range (unless otherwise noted)

| PARAMETER | TEST CONDITIONS | $\mathrm{V}_{\mathrm{CC}}$ | MIN | TYP ${ }^{(1)}$ | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{T}_{\mathrm{A}}}$ <br> Positive-going input threshold voltage |  | 1.65 V | 0.79 |  | 1.16 | V |
|  |  | 2.3 V | 1.11 |  | 1.56 |  |
|  |  | 3 V | 1.5 |  | 1.87 |  |
|  |  | 4.5 V | 2.16 |  | 2.74 |  |
|  |  | 5.5 V | 2.61 |  | 3.33 |  |
| $\mathrm{V}_{\mathrm{T}-}$ <br> Negative-going input threshold voltage |  | 1.65 V | 0.39 |  | 0.62 | V |
|  |  | 2.3 V | 0.58 |  | 0.87 |  |
|  |  | 3 V | 0.84 |  | 1.14 |  |
|  |  | 4.5 V | 1.41 |  | 1.79 |  |
|  |  | 5.5 V | 1.87 |  | 2.29 |  |
| $\Delta \mathrm{V}_{\mathrm{T}}$ <br> Hysteresis $\left(\mathrm{V}_{\mathrm{T}_{\mathrm{A}}}-\mathrm{V}_{\mathrm{T}-}\right)$ |  | 1.65 V | 0.37 |  | 0.62 | V |
|  |  | 2.3 V | 0.48 |  | 0.77 |  |
|  |  | 3 V | 0.56 |  | 0.87 |  |
|  |  | 4.5 V | 0.71 |  | 1.04 |  |
|  |  | 5.5 V | 0.71 |  | 1.11 |  |
| $\mathrm{V}_{\mathrm{OH}}$ | $\mathrm{I}_{\mathrm{OL}}=-100 \mu \mathrm{~A}$ | 1.65 V to 4.5 V | $\mathrm{V}_{\mathrm{CC}}-0.1$ |  |  | V |
|  | $\mathrm{I}_{\mathrm{OL}}=-4 \mathrm{~mA}$ | 1.65 V | 1.2 |  |  |  |
|  | $\mathrm{I}_{\mathrm{OL}}=-8 \mathrm{~mA}$ | 2.3 V | 1.9 |  |  |  |
|  | $\mathrm{I}_{\mathrm{OL}}=-16 \mathrm{~mA}$ | 3 V | 2.4 |  |  |  |
|  | $\mathrm{I}_{\mathrm{OL}}=-24 \mathrm{~mA}$ |  | 2.3 |  |  |  |
|  | $\mathrm{I}_{\mathrm{OL}}=-32 \mathrm{~mA}$ | 4.5 V | 3.8 |  |  |  |
| $\mathrm{V}_{\mathrm{OL}}$ | $\mathrm{I}_{\mathrm{OL}}=100 \mu \mathrm{~A}$ | 1.65 V to 4.5 V |  |  | 0.1 | V |
|  | $\mathrm{I}_{\mathrm{OL}}=4 \mathrm{~mA}$ | 1.65 V |  |  | 0.45 |  |
|  | $\mathrm{I}_{\mathrm{OL}}=8 \mathrm{~mA}$ | 2.3 V |  |  | 0.3 |  |
|  | $\mathrm{I}_{\mathrm{OL}}=16 \mathrm{~mA}$ | 3 V |  |  | 0.4 |  |
|  | $\mathrm{I}_{\mathrm{OL}}=24 \mathrm{~mA}$ |  |  |  | 0.55 |  |
|  | $\mathrm{I}_{\mathrm{OL}}=32 \mathrm{~mA}$ | 4.5 V |  |  | 0.55 |  |
| $\mathrm{I}_{\mathrm{I}}$ | A input | $\mathrm{V}_{\mathrm{I}}=5.5 \mathrm{~V}$ or GND | 0 to 5.5 V |  | $\pm 5$ | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {off }}$ |  | $\mathrm{V}_{\mathrm{I}}$ or $\mathrm{V}_{\mathrm{O}}=5.5 \mathrm{~V}$ | 0 |  | $\pm 10$ | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\mathrm{CC}}$ |  | $\mathrm{V}_{\mathrm{I}}=5.5 \mathrm{~V}$ or GND, $\quad \mathrm{I}_{\mathrm{O}}=0$ | 1.65 V to 5.5 V |  | 10 | $\mu \mathrm{A}$ |
| $\Delta \mathrm{I}_{\mathrm{CC}}$ |  | One input at $\mathrm{V}_{\mathrm{CC}}-0.6 \mathrm{~V}$, Other inputs at $\mathrm{V}_{\mathrm{CC}}$ or GND | 3 V to 5.5 V |  | 500 | $\mu \mathrm{A}$ |
| $\mathrm{C}_{\mathrm{i}}$ |  | $\mathrm{V}_{\mathrm{I}}=\mathrm{V}_{\mathrm{CC}}$ or GND | 3.3 V |  | 4.5 | pF |

(1) All typical values are at $\mathrm{V}_{\mathrm{CC}}=3.3 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=25^{\circ} \mathrm{C}$.# 6.5 Switching Characteristics 

over recommended operating free-air temperature range, $\mathrm{C}_{\mathrm{L}}=15 \mathrm{pF}$ (unless otherwise noted) (see Figure 1)

| PARAMETER | FROM <br> (INPUT) | TO <br> (OUTPUT) | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=1.8 \mathrm{~V} \\ \pm 0.15 \mathrm{~V} \\ \hline \end{gathered}$ |  | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=2.5 \mathrm{~V} \\ \pm 0.2 \mathrm{~V} \\ \hline \end{gathered}$ |  | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=3.3 \mathrm{~V} \\ \pm 0.3 \mathrm{~V} \\ \hline \end{gathered}$ |  | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=5 \mathrm{~V} \\ \pm 0.5 \mathrm{~V} \\ \hline \end{gathered}$ |  | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | MIN | MAX | MIN | MAX | MIN | MAX | MIN | MAX |
| $t_{\text {pd }}$ | A | Y | 2.8 | 9.9 | 1.6 | 5.5 | 1.5 | 4.6 | 0.9 | 4.4 | ns |

### 6.6 Switching Characteristics

over recommended operating free-air temperature range, $\mathrm{C}_{\mathrm{L}}=30 \mathrm{pF}$ or 50 pF (unless otherwise noted) (see Figure 2)

| PARAMETER | FROM <br> (INPUT) | TO <br> (OUTPUT) | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=1.8 \mathrm{~V} \\ \pm 0.15 \mathrm{~V} \\ \hline \end{gathered}$ |  | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=2.5 \mathrm{~V} \\ \pm 0.2 \mathrm{~V} \\ \hline \end{gathered}$ |  | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=3.3 \mathrm{~V} \\ \pm 0.3 \mathrm{~V} \\ \hline \end{gathered}$ |  | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=5 \mathrm{~V} \\ \pm 0.5 \mathrm{~V} \\ \hline \end{gathered}$ |  | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | MIN | MAX | MIN | MAX | MIN | MAX | MIN | MAX |
| $t_{\text {pd }}$ | A | Y | 3.8 | 11 | 2 | 6.5 | 1.8 | 5.5 | 1.2 | 5 | ns |

### 6.7 Operating Characteristics

$T_{A}=25^{\circ} \mathrm{C}$

| PARAMETER | TEST CONDITIONS | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=1.8 \mathrm{~V} \\ \text { TYP } \\ \hline \end{gathered}$ | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=2.5 \mathrm{~V} \\ \text { TYP } \\ \hline \end{gathered}$ | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=3.3 \mathrm{~V} \\ \text { TYP } \\ \hline \end{gathered}$ | $\begin{gathered} \mathrm{V}_{\mathrm{CC}}=5 \mathrm{~V} \\ \text { TYP } \\ \hline \end{gathered}$ | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{C}_{\text {pd }}$ Power dissipation capacitance | $\mathrm{f}=10 \mathrm{MHz}$ | 20 | 21 | 22 | 25 | pF |# 7 Parameter Measurement Information 

![img-5.jpeg](img-5.jpeg)

LOAD CIRCUIT
![img-6.jpeg](img-6.jpeg)

NOTES: A. $\mathrm{C}_{\mathrm{L}}$ includes probe and jig capacitance.
B. Waveform 1 is for an output with internal conditions such that the output is low, except when disabled by the output control. Waveform 2 is for an output with internal conditions such that the output is high, except when disabled by the output control.
C. All input pulses are supplied by generators having the following characteristics: PRR $\leq 10 \mathrm{MHz}, \mathrm{Z}_{\mathrm{O}}=50 \Omega$.
D. The outputs are measured one at a time, with one transition per measurement.
E. $\mathrm{t}_{\mathrm{PLZ}}$ and $\mathrm{t}_{\mathrm{PHZ}}$ are the same as $\mathrm{t}_{\text {dis }}$.
F. $\mathrm{t}_{\mathrm{PZL}}$ and $\mathrm{t}_{\mathrm{PZH}}$ are the same as $\mathrm{t}_{\mathrm{am}}$.
G. $\mathrm{t}_{\mathrm{PLH}}$ and $\mathrm{t}_{\mathrm{PHL}}$ are the same as $\mathrm{t}_{\mathrm{pd}}$.
H. All parameters and waveforms are not applicable to all devices.

Figure 1. Load Circuit and Voltage Waveforms# Parameter Measurement Information (continued) 

![img-7.jpeg](img-7.jpeg)

| TEST |  |  | S1 |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{t}_{\mathrm{PLH}} / \mathrm{t}_{\mathrm{PHL}}$ <br> $\mathrm{t}_{\mathrm{PLZ}} / \mathrm{t}_{\mathrm{PZL}}$ <br> $\mathrm{t}_{\mathrm{PHZ}} / \mathrm{t}_{\mathrm{PZH}}$ |  |  | Open <br> VLOAD <br> GND |  |  |  |  |

LOAD CIRCUIT
![img-8.jpeg](img-8.jpeg)

NOTES: A. $\mathrm{C}_{\mathrm{L}}$ includes probe and jig capacitance.
B. Waveform 1 is for an output with internal conditions such that the output is low, except when disabled by the output control. Waveform 2 is for an output with internal conditions such that the output is high, except when disabled by the output control.
C. All input pulses are supplied by generators having the following characteristics: PRR $\leq 10 \mathrm{MHz}, \mathrm{Z}_{\mathrm{O}}=50 \Omega$.
D. The outputs are measured one at a time, with one transition per measurement.
E. $\mathrm{t}_{\mathrm{PLZ}}$ and $\mathrm{t}_{\mathrm{PHZ}}$ are the same as $\mathrm{t}_{\mathrm{dis}}$.
F. $\mathrm{t}_{\mathrm{PZL}}$ and $\mathrm{t}_{\mathrm{PZH}}$ are the same as $\mathrm{t}_{\mathrm{sm}}$.
G. $\mathrm{t}_{\mathrm{PLH}}$ and $\mathrm{t}_{\mathrm{PHL}}$ are the same as $\mathrm{t}_{\mathrm{pd}}$.
H. All parameters and waveforms are not applicable to all devices.

Figure 2. Load Circuit and Voltage Waveforms# 8 Device and Documentation Support 

### 8.1 Trademarks

NanoFree is a trademark of Texas Instruments.

### 8.2 Electrostatic Discharge Caution

These devices have limited built-in ESD protection. The leads should be shorted together or the device placed in conductive foam during storage or handling to prevent electrostatic damage to the MOS gates.

### 8.3 Glossary

SLYZ022 - TI Glossary.
This glossary lists and explains terms, acronyms and definitions.

## 9 Mechanical, Packaging, and Orderable Information

The following pages include mechanical packaging and orderable information. This information is the most current data available for the designated devices. This data is subject to change without notice and revision of this document. For browser-based versions of this data sheet, refer to the left-hand navigation.# PACKAGE OPTION ADDENDUM

|  Orderable Device | Status
(1) | Package Type | Package
Drawing | Pins | Package
Qty | Eco Plan
(2) | Lead/Ball Finish
(6) | MSL Peak Temp
(3) | Op Temp ( ${ }^{\circ} \mathrm{C}$ ) | Device Marking
(4/5) | Samples  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  SN74LVC1G14DBVR | ACTIVE | SOT-23 | DBV | 5 | 3000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (C142 - C145 -
C14F - C14K -
C14R) | Samples  |
|  SN74LVC1G14DBVRE4 | ACTIVE | SOT-23 | DBV | 5 | 3000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (C142 - C145 -
C14F - C14K -
C14R) | Samples  |
|  SN74LVC1G14DBVRG4 | ACTIVE | SOT-23 | DBV | 5 | 3000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (C142 - C145 -
C14F - C14K -
C14R) | Samples  |
|  SN74LVC1G14DBVT | ACTIVE | SOT-23 | DBV | 5 | 250 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (C145 - C14F -
C14K - C14R) | Samples  |
|  SN74LVC1G14DBVTE4 | ACTIVE | SOT-23 | DBV | 5 | 250 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (C145 - C14F -
C14K - C14R) | Samples  |
|  SN74LVC1G14DBVTG4 | ACTIVE | SOT-23 | DBV | 5 | 250 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (C145 - C14F -
C14K - C14R) | Samples  |
|  SN74LVC1G14DCKR | ACTIVE | SC70 | DCK | 5 | 3000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (CF5 - CFF - CFK -
CFR - CFT) | Samples  |
|  SN74LVC1G14DCKRE4 | ACTIVE | SC70 | DCK | 5 | 3000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (CF5 - CFF - CFK -
CFR - CFT) | Samples  |
|  SN74LVC1G14DCKRG4 | ACTIVE | SC70 | DCK | 5 | 3000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (CF5 - CFF - CFK -
CFR - CFT) | Samples  |
|  SN74LVC1G14DCKT | ACTIVE | SC70 | DCK | 5 | 250 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (CF5 - CFF - CFK -
CFR - CFT) | Samples  |
|  SN74LVC1G14DCKTE4 | ACTIVE | SC70 | DCK | 5 | 250 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (CF5 - CFF - CFK -
CFR - CFT) | Samples  |
|  SN74LVC1G14DCKTG4 | ACTIVE | SC70 | DCK | 5 | 250 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (CF5 - CFF - CFK -
CFR - CFT) | Samples  |
|  SN74LVC1G14DRLR | ACTIVE | SOT | DRL | 5 | 4000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (CF7 - CFR) | Samples  |
|  SN74LVC1G14DRLRG4 | ACTIVE | SOT | DRL | 5 | 4000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (CF7 - CFR) | Samples  |
|  SN74LVC1G14DRY2 | PREVIEW | SON | DRY | 6 | 5000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | CF |   |
|  SN74LVC1G14DRYR | ACTIVE | SON | DRY | 6 | 5000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | CF | Samples  ||  Orderable Device | Status
(1) | Package Type | Package
Drawing | Pins | Package
Qty | Eco Plan
(2) | Lead/Ball Finish
(6) | MSL Peak Temp
(3) | Op Temp ( ${ }^{\circ} \mathrm{C}$ ) | Device Marking
(4/5) | Samples  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  SN74LVC1G14DSF2 | PREVIEW | SON | DSF | 6 | 5000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | CF |   |
|  SN74LVC1G14DSFR | ACTIVE | SON | DSF | 6 | 5000 | Green (RoHS
\& no Sb/Br) | CU NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | CF | Samples  |
|  SN74LVC1G14YZPR | ACTIVE | DSBGA | YZP | 5 | 3000 | Green (RoHS
\& no Sb/Br) | SNAGCU | Level-1-260C-UNLIM | $-40$ to 85 | (CF7 - CFN) | Samples  |
|  SN74LVC1G14YZVR | ACTIVE | DSBGA | YZV | 4 | 3000 | Green (RoHS
\& no Sb/Br) | SNAGCU | Level-1-260C-UNLIM | $-40$ to 85 | $\begin{aligned} & \text { CF } \ & (7 \sim N) \end{aligned}$ | Samples  |

${ }^{(1)}$ The marketing status values are defined as follows: ACTIVE: Product device recommended for new designs. LIFEBUY: TI has announced that the device will be discontinued, and a lifetime-buy period is in effect. NRND: Not recommended for new designs. Device is in production to support existing customers, but TI does not recommend using this part in a new design. PREVIEW: Device has been announced but is not in production. Samples may or may not be available. OBSOLETE: TI has discontinued the production of the device. ${ }^{(2)}$ Eco Plan - The planned eco-friendly classification: Pb-Free (RoHS), Pb-Free (RoHS Exempt), or Green (RoHS \& no Sb/Br) - please check http://www.ti.com/productcontent for the latest availability information and additional product content details. TBD: The Pb-Free/Green conversion plan has not been defined. Pb-Free (RoHS): TI's terms "Lead-Free" or "Pb-Free" mean semiconductor products that are compatible with the current RoHS requirements for all 6 substances, including the requirement that lead not exceed $0.1 \%$ by weight in homogeneous materials. Where designed to be soldered at high temperatures, TI Pb-Free products are suitable for use in specified lead-free processes. Pb-Free (RoHS Exempt): This component has a RoHS exemption for either 1) lead-based flip-chip solder bumps used between the die and package, or 2) lead-based die adhesive used between the die and leadframe. The component is otherwise considered Pb-Free (RoHS compatible) as defined above. Green (RoHS \& no Sb/Br): TI defines "Green" to mean Pb-Free (RoHS compatible), and free of Bromine (Br) and Antimony (Sb) based flame retardants (Br or Sb do not exceed $0.1 \%$ by weight in homogeneous material) ${ }^{(3)}$ MSL, Peak Temp. - The Moisture Sensitivity Level rating according to the JEDEC industry standard classifications, and peak solder temperature. ${ }^{(4)}$ There may be additional marking, which relates to the logo, the lot trace code information, or the environmental category on the device. ${ }^{(5)}$ Multiple Device Markings will be inside parentheses. Only one Device Marking contained in parentheses and separated by a "-" will appear on a device. If a line is indented then it is a continuation of the previous line and the two combined represent the entire Device Marking for that device. ${ }^{(6)}$ Lead/Ball Finish - Orderable Devices may have multiple material finish options. Finish options are separated by a vertical ruled line. Lead/Ball Finish values may wrap to two lines if the finish value exceeds the maximum column width.

Important Information and Disclaimer:The information provided on this page represents TI's knowledge and belief as of the date that it is provided. TI bases its knowledge and belief on information provided by third parties, and makes no representation or warranty as to the accuracy of such information. Efforts are underway to better integrate information from third parties. TI has taken and![img-9.jpeg](img-9.jpeg)

# PACKAGE OPTION ADDENDUM

www.ti.com

10-Jun-2014

Continues to take reasonable steps to provide representative and accurate information but may not have conducted destructive testing or chemical analysis on incoming materials and chemicals. TI and TI suppliers consider certain information to be proprietary, and thus CAS numbers and other limited information may not be available for release.

In no event shall TI's liability arising out of such information exceed the total purchase price of the TI part(s) at issue in this document sold by TI to Customer on an annual basis.

## OTHER QUALIFIED VERSIONS OF SN74LVC1G14:

- Enhanced Product: SN74LVC1G14-EP

**NOTE:** Qualified Version Definitions:

- Enhanced Product: Supports Defense, Aerospace and Medical Applications# TAPE AND REEL INFORMATION 

![img-10.jpeg](img-10.jpeg)
![img-11.jpeg](img-11.jpeg)

| A0 | Dimension designed to accommodate the component width |
| :-- | :-- |
| B0 | Dimension designed to accommodate the component length |
| K0 | Dimension designed to accommodate the component thickness |
| W | Overall width of the carrier tape |
| P1 | Pitch between successive cavity centers |

*All dimensions are nominal

| Device | Package <br> Type | Package <br> Drawing | Pins | SPQ | Reel <br> Diameter <br> (mm) | Reel <br> Width <br> W1 (mm) | A0 <br> (mm) | B0 <br> (mm) | K0 <br> (mm) | P1 <br> (mm) | W <br> (mm) | Pin1 <br> Quadrant |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 180.0 | 9.2 | 3.17 | 3.23 | 1.37 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 178.0 | 9.2 | 3.3 | 3.2 | 1.55 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 178.0 | 9.0 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DBVT | SOT-23 | DBV | 5 | 250 | 178.0 | 9.0 | 3.23 | 3.17 | 1.37 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DBVT | SOT-23 | DBV | 5 | 250 | 178.0 | 9.2 | 3.3 | 3.2 | 1.55 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DCKR | SC70 | DCK | 5 | 3000 | 180.0 | 9.2 | 2.3 | 2.55 | 1.2 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DCKR | SC70 | DCK | 5 | 3000 | 178.0 | 9.2 | 2.4 | 2.4 | 1.22 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DCKT | SC70 | DCK | 5 | 250 | 178.0 | 9.2 | 2.4 | 2.4 | 1.22 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DCKT | SC70 | DCK | 5 | 250 | 180.0 | 9.2 | 2.3 | 2.55 | 1.2 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DCKT | SC70 | DCK | 5 | 250 | 178.0 | 9.0 | 2.4 | 2.5 | 1.2 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DRLR | SOT | DRL | 5 | 4000 | 180.0 | 9.5 | 1.78 | 1.78 | 0.69 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DRLR | SOT | DRL | 5 | 4000 | 180.0 | 8.4 | 1.98 | 1.78 | 0.69 | 4.0 | 8.0 | Q3 |
| SN74LVC1G14DRYR | SON | DRY | 6 | 5000 | 179.0 | 8.4 | 1.2 | 1.65 | 0.7 | 4.0 | 8.0 | Q1 |
| SN74LVC1G14DSFR | SON | DSF | 6 | 5000 | 180.0 | 9.5 | 1.16 | 1.16 | 0.5 | 4.0 | 8.0 | Q2 |
| SN74LVC1G14YZPR | DSBGA | YZP | 5 | 3000 | 178.0 | 9.2 | 1.02 | 1.52 | 0.63 | 4.0 | 8.0 | Q1 |
| SN74LVC1G14YZVR | DSBGA | YZV | 4 | 3000 | 178.0 | 9.2 | 1.0 | 1.0 | 0.63 | 4.0 | 8.0 | Q1 |![img-12.jpeg](img-12.jpeg)
*All dimensions are nominal

| Device | Package Type | Package Drawing | Pins | SPQ | Length (mm) | Width (mm) | Height (mm) |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 205.0 | 200.0 | 33.0 |
| SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 180.0 | 180.0 | 18.0 |
| SN74LVC1G14DBVR | SOT-23 | DBV | 5 | 3000 | 180.0 | 180.0 | 18.0 |
| SN74LVC1G14DBVT | SOT-23 | DBV | 5 | 250 | 180.0 | 180.0 | 18.0 |
| SN74LVC1G14DBVT | SOT-23 | DBV | 5 | 250 | 180.0 | 180.0 | 18.0 |
| SN74LVC1G14DBVT | SC70 | DCK | 5 | 3000 | 205.0 | 200.0 | 33.0 |
| SN74LVC1G14DCKR | SC70 | DCK | 5 | 3000 | 180.0 | 180.0 | 18.0 |
| SN74LVC1G14DCKT | SC70 | DCK | 5 | 250 | 180.0 | 180.0 | 18.0 |
| SN74LVC1G14DCKT | SC70 | DCK | 5 | 250 | 205.0 | 200.0 | 33.0 |
| SN74LVC1G14DCKT | SC70 | DCK | 5 | 250 | 180.0 | 180.0 | 18.0 |
| SN74LVC1G14DRLR | SOT | DRL | 5 | 4000 | 184.0 | 184.0 | 19.0 |
| SN74LVC1G14DRLR | SOT | DRL | 5 | 4000 | 202.0 | 201.0 | 28.0 |
| SN74LVC1G14DRYR | SON | DRY | 6 | 5000 | 203.0 | 203.0 | 35.0 |
| SN74LVC1G14DSFR | SON | DSF | 6 | 5000 | 184.0 | 184.0 | 19.0 |
| SN74LVC1G14YZPR | DSBGA | YZP | 5 | 3000 | 220.0 | 220.0 | 35.0 |
| SN74LVC1G14YZVR | DSBGA | YZV | 4 | 3000 | 220.0 | 220.0 | 35.0 |![img-13.jpeg](img-13.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Body dimensions do not include mold flash or protrusion. Mold flash and protrusion shall not exceed 0.15 per side.
D. Falls within JEDEC MO-178 Variation AA.![img-14.jpeg](img-14.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Customers should place a note on the circuit board fabrication drawing not to alter the center solder mask defined pad.
D. Publication IPC-7351 is recommended for alternate designs.
E. Laser cutting apertures with trapezoidal walls and also rounding corners will offer better paste release. Customers should contact their board assembly site for stencil design recommendations. Example stencil design based on a $50 \%$ volumetric metal load solder paste. Refer to IPC-7525 for other stencil recommendations.# DCK (R-PDSO-G5) 

PLASTIC SMALL-OUTLINE PACKAGE
![img-15.jpeg](img-15.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Body dimensions do not include mold flash or protrusion. Mold flash and protrusion shall not exceed 0.15 per side.
D. Falls within JEDEC MO-203 variation AA.![img-16.jpeg](img-16.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Customers should place a note on the circuit board fabrication drawing not to alter the center solder mask defined pad.
D. Publication IPC-7351 is recommended for alternate designs.
E. Laser cutting apertures with trapezoidal walls and also rounding corners will offer better paste release. Customers should contact their board assembly site for stencil design recommendations. Example stencil design based on a $50 \%$ volumetric metal load solder paste. Refer to IPC-7525 for other stencil recommendations.![img-17.jpeg](img-17.jpeg)

NOTES: A. All linear dimensions are in millimeters. Dimensioning and tolerancing per ASME Y14.5M-1994.
B. This drawing is subject to change without notice.

Body dimensions do not include mold flash, interlead flash, protrusions, or gate burrs. Mold flash, interlead flash, protrusions, or gate burrs shall not exceed 0,15 per end or side.
D. JEDEC package registration is pending.![img-18.jpeg](img-18.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Publication IPC-7351 is recommended for alternate designs.
D. Customers should contact their board fabrication site for minimum solder mask web tolerances between signal pads.
E. Maximum stencil thickness $0,127 \mathrm{~mm}$ ( 5 mils). All linear dimensions are in millimeters.
F. Laser cutting apertures with trapezoidal walls and also rounding corners will offer better paste release. Customers should contact their board assembly site for stencil design recommendations. Refer to IPC 7525 for stencil design considerations.
G. Side aperture dimensions over-print land for acceptable area ratio $>0.66$. Customer may reduce side aperture dimensions if stencil manufacturing process allows for sufficient release at smaller opening.![img-19.jpeg](img-19.jpeg)

NOTES: A. All linear dimensions are in millimeters. Dimensioning and tolerancing per ASME Y14.5M-1994.
B. This drawing is subject to change without notice.
C. SON (Small Outline No-Lead) package configuration.

The exposed lead frame feature on side of package may or may not be present due to alternative lead frame designs.
E. This package complies to JEDEC MO-287 variation UFAD.
See the additional figure in the Product Data Sheet for details regarding the pin 1 identifier shape.DRY (R-PUSON-N6) PLASTIC SMALL OUTLINE NO-LEAD
![img-20.jpeg](img-20.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Publication IPC-7351 is recommended for alternate designs.
D. Customers should contact their board fabrication site for minimum solder mask web tolerances between signal pads.
E. Maximum stencil thickness $0,127 \mathrm{~mm}$ ( 5 mils). All linear dimensions are in millimeters.
F. Laser cutting apertures with trapezoidal walls and also rounding corners will offer better paste release. Customers should contact their board assembly site for stencil design recommendations. Refer to IPC 7525 for stencil design considerations.
G. Side aperture dimensions over-print land for acceptable area ratio $>0.66$. Customer may reduce side aperture dimensions if stencil manufacturing process allows for sufficient release at smaller opening.![img-21.jpeg](img-21.jpeg)

NOTES: A. All linear dimensions are in millimeters. Dimensioning and tolerancing per ASME Y14.5M-1994.
B. This drawing is subject to change without notice.
C. SON (Small Outline No-Lead) package configuration.
D. This package complies to JEDEC MO-287 variation X2AAF.![img-22.jpeg](img-22.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Publication IPC-7351 is recommended for alternate designs.
D. Customers should contact their board fabrication site for minimum solder mask web tolerances between signal pads. If 2 mil solder mask is outside PCB vendor capability, it is advised to omit solder mask.
E. Maximum stencil thickness $0,1016 \mathrm{~mm}$ ( 4 mils). All linear dimensions are in millimeters.
F. Laser cutting apertures with trapezoidal walls and also rounding corners will offer better paste release. Customers should contact their board assembly site for stencil design recommendations. Refer to IPC 7525 for stencil design considerations.
G. Suggest stencils cut with lasers such as Fiber Laser that produce the greatest positional accuracy.
H. Component placement force should be minimized to prevent excessive paste block deformation.YZP (R-XBGA-N5)
DIE-SIZE BALL GRID ARRAY
![img-23.jpeg](img-23.jpeg)

NOTES: A. All linear dimensions are in millimeters. Dimensioning and tolerancing per ASME Y14.5M-1994.
B. This drawing is subject to change without notice.
C. NanoFree ${ }^{\text {TM }}$ package configuration.YZV (S-XBGA-N4)
DIE-SIZE BALL GRID ARRAY
![img-24.jpeg](img-24.jpeg)

NOTES: A. All linear dimensions are in millimeters. Dimensioning and tolerancing per ASME Y14.5M-1994.
B. This drawing is subject to change without notice.
C. NanoFree ${ }^{\text {TM }}$ package configuration.# IMPORTANT NOTICE 

Texas Instruments Incorporated and its subsidiaries (TI) reserve the right to make corrections, enhancements, improvements and other changes to its semiconductor products and services per JESD46, latest issue, and to discontinue any product or service per JESD48, latest issue. Buyers should obtain the latest relevant information before placing orders and should verify that such information is current and complete. All semiconductor products (also referred to herein as "components") are sold subject to TI's terms and conditions of sale supplied at the time of order acknowledgment.
TI warrants performance of its components to the specifications applicable at the time of sale, in accordance with the warranty in TI's terms and conditions of sale of semiconductor products. Testing and other quality control techniques are used to the extent TI deems necessary to support this warranty. Except where mandated by applicable law, testing of all parameters of each component is not necessarily performed.
TI assumes no liability for applications assistance or the design of Buyers' products. Buyers are responsible for their products and applications using TI components. To minimize the risks associated with Buyers' products and applications, Buyers should provide adequate design and operating safeguards.
TI does not warrant or represent that any license, either express or implied, is granted under any patent right, copyright, mask work right, or other intellectual property right relating to any combination, machine, or process in which TI components or services are used. Information published by TI regarding third-party products or services does not constitute a license to use such products or services or a warranty or endorsement thereof. Use of such information may require a license from a third party under the patents or other intellectual property of the third party, or a license from TI under the patents or other intellectual property of TI.
Reproduction of significant portions of TI information in TI data books or data sheets is permissible only if reproduction is without alteration and is accompanied by all associated warranties, conditions, limitations, and notices. TI is not responsible or liable for such altered documentation. Information of third parties may be subject to additional restrictions.
Resale of TI components or services with statements different from or beyond the parameters stated by TI for that component or service voids all express and any implied warranties for the associated TI component or service and is an unfair and deceptive business practice. TI is not responsible or liable for any such statements.
Buyer acknowledges and agrees that it is solely responsible for compliance with all legal, regulatory and safety-related requirements concerning its products, and any use of TI components in its applications, notwithstanding any applications-related information or support that may be provided by TI. Buyer represents and agrees that it has all the necessary expertise to create and implement safeguards which anticipate dangerous consequences of failures, monitor failures and their consequences, lessen the likelihood of failures that might cause harm and take appropriate remedial actions. Buyer will fully indemnify TI and its representatives against any damages arising out of the use of any TI components in safety-critical applications.
In some cases, TI components may be promoted specifically to facilitate safety-related applications. With such components, TI's goal is to help enable customers to design and create their own end-product solutions that meet applicable functional safety standards and requirements. Nonetheless, such components are subject to these terms.
No TI components are authorized for use in FDA Class III (or similar life-critical medical equipment) unless authorized officers of the parties have executed a special agreement specifically governing such use.
Only those TI components which TI has specifically designated as military grade or "enhanced plastic" are designed and intended for use in military/aerospace applications or environments. Buyer acknowledges and agrees that any military or aerospace use of TI components which have not been so designated is solely at the Buyer's risk, and that Buyer is solely responsible for compliance with all legal and regulatory requirements in connection with such use.
TI has specifically designated certain components as meeting ISO/TS16949 requirements, mainly for automotive use. In any case of use of non-designated products, TI will not be responsible for any failure to meet ISO/TS16949.

Products
Audio
Amplifiers
Data Converters
DLP® Products
DSP
Clocks and Timers
Interface
Logic
Power Mgmt
Microcontrollers
RFID
OMAP Applications Processors
Wireless Connectivity

## Applications

Automotive and Transportation www.ti.com/automotive
Communications and Telecom www.ti.com/communications
Computers and Peripherals
Consumer Electronics
Energy and Lighting
Industrial
Medical
Security
Space, Avionics and Defense Video and Imaging

T1 E2E Community
www.ti.com/wirelessconnectivity

www.ti.com/computers
www.ti.com/consumer-apps
www.ti.com/energy
www.ti.com/industrial
www.ti.com/medical
www.ti.com/security
www.ti.com/space-avionics-defense
www.ti.com/video
e2e.ti.com

Mailing Address: Texas Instruments, Post Office Box 655303, Dallas, Texas 75265
Copyright © 2014, Texas Instruments Incorporated