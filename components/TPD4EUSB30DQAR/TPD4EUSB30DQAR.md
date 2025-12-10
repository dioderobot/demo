# TPDxEUSB30 2-, 4-Channel ESD Protection for Super-Speed USB 3.0 Interface 

## 1 Features

- Supports USB 3.0 data rates (5 Gbps)
- IEC 61000-4-2 ESD protection (level 4 contact)
- IEC 61000-4-5 surge protection
- 5 A $(8 / 20 \mu \mathrm{~s})$
- Low capacitance
- DRT: 0.7 pF (typical)
- DQA: 0.8 pF (typical)
- Dynamic resistance: $0.6 \Omega$ (typical)
- Space-saving DRT, DQA packages
- Flow-through pin mapping


## 2 Applications

- Notebooks
- Set-top boxes
- DVD players
- Media players
- Portable computers
![img-0.jpeg](img-0.jpeg)


## 3 Description

The TPD2EUSB30, TPD2EUSB30A, and TPD4EUSB30 are 2 and 4 channel Transient Voltage Suppressor (TVS) based Electrostatic Discharge (ESD) protection diode arrays. The TPDxEUSB30/A devices are rated to dissipate ESD strikes at the maximum level specified in the IEC 61000-4-2 international standard (Contact). These devices also offer 5 A $(8 / 20 \mu \mathrm{~s})$ peak pulse current ratings per IEC 61000-4-5 (Surge) specification.
The TPD2EUSB30A offers low 4.5-V DC breakdown voltage. The low capacitance, low break-down voltage, and low dynamic resistance make the TPD2EUSB30A a superior protection device for highspeed differential IOs.
The TPD2EUSB30 and TPD2EUSB30A are offered in space saving DRT ( $1 \mathrm{~mm} \times 1 \mathrm{~mm}$ ) package. The TPD4EUSB30 is offered in space saving DQA (2.5 $\mathrm{mm} \times 1.0 \mathrm{~mm}$ ) package.

Device Information ${ }^{(1)}$

| PART NUMBER | PACKAGE | BODY SIZE (NOM) |
| :-- | :-- | :-- |
| TPD2EUSB30 |  |  |
| TPD2EUSB30A | SOT (3) | $1.00 \mathrm{~mm} \times 0.80 \mathrm{~mm}$ |
| TPD4EUSB30 | USON (10) | $2.50 \mathrm{~mm} \times 1.00 \mathrm{~mm}$ |

(1) For all available packages, see the orderable addendum at the end of the data sheet.
![img-1.jpeg](img-1.jpeg)

TPD2EUSB30/A Circuit# Table of Contents 

1 Features ..... 1
7.4 Device Functional Modes ..... 8
2 Applications ..... 1
8 Application and Implementation ..... 9
3 Description ..... 1
8.1 Application Information ..... 9
4 Revision History ..... 2
8.2 Typical Application ..... 9
5 Pin Configuration and Functions ..... 3
9 Power Supply Recommendations ..... 11
6 Specifications ..... 4
10 Layout ..... 11
6.1 Absolute Maximum Ratings ..... 4
10.1 Layout Guidelines ..... 11
6.2 ESD Ratings ..... 4
10.2 Layout Examples ..... 12
6.3 Recommended Operating Conditions ..... 4
11.1 Receiving Notification of Documentation Updates ..... 14
6.4 Thermal Information ..... 4
11.2 Support Resources ..... 14
6.5 Electrical Characteristics ..... 5
11.3 Trademarks ..... 14
6.6 Typical Characteristics ..... 6
11.4 Electrostatic Discharge Caution ..... 14
7 Detailed Description ..... 8
11.5 Glossary ..... 14
7.1 Overview ..... 8
12 Mechanical, Packaging, and Orderable Information ..... 14
7.2 Functional Block Diagrams ..... 8
Information ..... 14
7.3 Feature Description ..... 8

## 4 Revision History

NOTE: Page numbers for previous revisions may differ from page numbers in the current version.
Changes from Revision F (October 2015) to Revision G (June 2021) Page

- Updated the numbering format for tables, figures, and cross-references throughout the document ..... 1
- Changed the Pin Functions table to clarify pin order and function ..... 3
Changes from Revision E (August 2014) to Revision F (October 2015) Page
- Moved the storage temperature to the Absolute Maximum Ratings table and updated the Handling Ratings table to an ESD Ratings table ..... 4
- Added test condition frequency to capacitance ..... 5
Changes from Revision D (August 2012) to Revision E (July 2014) Page
- Added Handling Rating table, Feature Description section, Device Functional Modes, Application and Implementation section, Power Supply Recommendations section, Layout section, Device and Documentation Support section, and Mechanical, Packaging, and Orderable Information section ..... 1
Changes from Revision C (December 2011) to Revision D (August 2012) Page
- Updated Dynamic Resistance value ..... 1
- Updated Dynamic Resistance value ..... 5
Changes from Revision B (July 2011) to Revision C (December 2011) Page
- Added Insertion Loss graphic to TYPICAL OPERATING CHARACTERISTICS section ..... 6
Changes from Revision A (December 2010) to Revision B (July 2011) Page
- Changed TOP-SIDE MARKING column in the Ordering Information Table ..... 3
Changes from Revision * (August 2010) to Revision A (December 2010) Page
- Added TPS2EUSB30A part to document ..... 1# 5 Pin Configuration and Functions 

![img-2.jpeg](img-2.jpeg)

Figure 5-1. DRT Package 3-Pin SOT Top View
![img-3.jpeg](img-3.jpeg)

Figure 5-2. DQA Package 10-Pin USON Top View
Table 5-1. Pin Functions

| PIN |  | TYPE | DESCRIPTION |
| :--: | :--: | :--: | :--: |
| NAME | DRT | DQA |  |
| D1+ | 1 | 1 | ESD port | High-speed ESD clamp, provides ESD protection to the high-speed differential data lines. |
| D1- | 2 | 2 |  |
| D2+ | - | 4 |  |
| D2- | - | 5 |  |
| GND | 3 | 3, 8 | GND | Ground |
| N.C. | - | $\begin{aligned} & \text { 6, 7, } \\ & \text { 9, } 10 \end{aligned}$ | - | Not normally connected |# 6 Specifications 

### 6.1 Absolute Maximum Ratings

over operating free-air temperature range (unless otherwise noted) ${ }^{(1)}$

|  |  |  | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  | IO voltage (D+ and D- pins) | TPD2EUSB30, TPD4EUSB30 | 0 | 6 | V |
|  |  | TPD2EUSB30A | 0 | 4 |  |
|  | IEC 61000-4-5 surge current $\left(t_{p}=8 / 20 \mu \mathrm{~s}\right)$ | D+, D- pins |  | 5 | A |
|  | IEC 61000-4-5 surge peak power $\left(t_{p}=8 / 20 \mu \mathrm{~s}\right)$ | D+, D- pins |  | 45 | W |
| $\mathrm{T}_{\mathrm{A}}$ | Operating free-air temperature |  | $-40$ | 85 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\text {stg }}$ | Storage temperature |  | $-65$ | 125 | ${ }^{\circ} \mathrm{C}$ |

(1) Stresses beyond those listed under Absolute Maximum Ratings may cause permanent damage to the device. These are stress ratings only, and functional operation of the device at these or any other conditions beyond those indicated in the operational sections of the specifications is not implied. Exposure to absolute maximum-rated conditions for extended periods may affect device reliability.

### 6.2 ESD Ratings

|  |  |  | VALUE | UNIT |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{(\text {ESD) }}$ | Electrostatic discharge | Human body model (HBM), per ANSI/ESDA/JEDEC JS-001, all pins ${ }^{(1)}$ | 2500 | V |
|  |  | Charged device model (CDM), per JEDEC specification JESD22-C101, all pins ${ }^{(2)}$ | 1500 |  |
|  |  | IEC 61000-4-2 Contact Discharge | D+, D- pins | 8000 |
|  |  | IEC 61000-4-2 Air-Gap Discharge (TPD2EUSB30/A) | D+, D- pins | 8000 |
|  |  | IEC 61000-4-2 Air-Gap Discharge (TPD4EUSB30) | D+, D- pins | 9000 |

(1) JEDEC document JEP155 states that 500-V HBM allows safe manufacturing with a standard ESD control process.
(2) JEDEC document JEP157 states that 250-V CDM allows safe manufacturing with a standard ESD control process.

### 6.3 Recommended Operating Conditions

over operating free-air temperature range (unless otherwise noted)

|  |  |  | MIN | MAX | UNIT |
| :-- | :-- | :-- | --: | --: | :--: |
| $\mathrm{T}_{\mathrm{A}}$ operating free-air temperature |  |  | -40 | 85 | ${ }^{\circ} \mathrm{C}$ |
| Operating Voltage |  | TPD2EUSB30, TPD4EUSB30 | 0 | 5.5 | V |
|  |  | TPD2EUSB30A | 0 | 3.6 |  |

### 6.4 Thermal Information

| THERMAL METRIC ${ }^{(1)}$ |  | TPD2EUSB30 | TPD2EUSB30A | TPD4EUSB30 | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | DRT (SOT) | DRT (SOT) | DQA (USON) |  |
|  |  | 3 PINS | 3 PINS | 10 PINS |  |
| $\mathrm{R}_{\text {8JA }}$ | Junction-to-ambient thermal resistance | 610.2 | 610.2 | 162.2 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{R}_{\text {8JC(top) }}$ | Junction-to-case (top) thermal resistance | 288.0 | 288.0 | 128.3 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{R}_{\text {8JB }}$ | Junction-to-board thermal resistance | 118.4 | 118.4 | 56.7 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\psi_{\text {JT }}$ | Junction-to-top characterization parameter | 20.2 | 20.2 | 13.8 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\psi_{\text {JB }}$ | Junction-to-board characterization parameter | 116.4 | 116.4 | 56.6 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{R}_{\text {8JC(bot) }}$ | Junction-to-case (bottom) thermal resistance | N/A | N/A | 8.1 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |

(1) For more information about traditional and new thermal metrics, see the Semiconductor and IC Package Thermal Metrics application report, SPRA953.# 6.5 Electrical Characteristics 

over operating free-air temperature range (unless otherwise noted)

| PARAMETER |  | TEST CONDITIONS |  | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {RWM }}$ | Reverse stand-off voltage ( $D+$ and D- pins) | TPD2EUSB30, TPD4EUSB30 |  |  |  | 5.5 | V |
|  |  | TPD2EUSB30A |  |  |  | 3.6 | V |
| $V_{\text {clamp }}$ | Clamp voltage | $D+, D-$ pins to ground, | $I_{I O}=1 \mathrm{~A}$ |  |  | 8 | V |
| $\mathrm{I}_{\text {IO }}$ | Current from IO port to supply pins | $\mathrm{V}_{\mathrm{IO}}=2.5 \mathrm{~V}$, | $\mathrm{I}_{\mathrm{D}}=8 \mathrm{~mA}$ |  | 0.01 | 0.1 | $\mu \mathrm{A}$ |
| $V_{D}$ | Diode forward voltage | $\begin{aligned} & \mathrm{D}+, \mathrm{D} \text { pins, } \\ & \text { lower clamp diode, } \end{aligned}$ | $\begin{aligned} & V_{I O}=2.5 \mathrm{~V}, \\ & I_{D}=8 \mathrm{~mA} \end{aligned}$ | 0.6 | 0.8 | 0.95 | V |
| $R_{\text {dyn }}$ | Dynamic resistance | $D+, D-$ pins | $I=1 \mathrm{~A}$ |  | 0.6 |  | $\Omega$ |
| $\mathrm{C}_{\text {IO-IO }}$ | Capacitance IO to IO | $D+, D-$ pins | $V_{I O}=2.5 \mathrm{~V} ; f=100 \mathrm{kHz}$ |  | 0.05 |  | pF |
| $\mathrm{C}_{\text {IO-GND }}$ | Capacitance IO to GND | $\begin{aligned} & \mathrm{D}+, \mathrm{D} \text { pins (DRT) } \\ & \mathrm{D} 1+, \mathrm{D} 1- \\ & \mathrm{D} 2+, \mathrm{D} 2-(\mathrm{DQA}) \end{aligned}$ | $\mathrm{V}_{\mathrm{IO}}=2.5 \mathrm{~V} ; f=100 \mathrm{kHz}$ |  | 0.7 |  | pF |
|  |  |  |  |  | 0.8 |  |  |
| $V_{B R}$ | Break-down voltage, TPD2EUSB30, TPD4EUSB30 | $I_{I O}=1 \mathrm{~mA}$ |  |  | 7 |  | V |
|  | Break-down voltage, TPD2EUSB30A | $I_{I O}=1 \mathrm{~mA}$ |  |  | 4.5 |  | V |# 6.6 Typical Characteristics 

![img-4.jpeg](img-4.jpeg)

Figure 6-1. IO Capacitance vs IO Voltage
![img-5.jpeg](img-5.jpeg)

Measured at one IO, the other IO open
Figure 6-3. Peak Pulse Waveforms
![img-6.jpeg](img-6.jpeg)

Figure 6-5. IEC Clamping Waveforms ( 8 kV Contact)
![img-7.jpeg](img-7.jpeg)

Figure 6-2. Leakage Current vs Temperature
![img-8.jpeg](img-8.jpeg)

Figure 6-4. D+,D- Transmission Line Pulser Plot for TPD2EUSB30 (100 ns Pulse, 10 ns Rise Time)
![img-9.jpeg](img-9.jpeg)

Figure 6-6. IEC Clamping Waveforms ( $\mathbf{- 8} \mathbf{~ k V}$ Contact)# 6.6 Typical Characteristics (continued) 

![img-10.jpeg](img-10.jpeg)# 7 Detailed Description 

### 7.1 Overview

The TPD2EUSB30, TPD2EUSB30A, and TPD4EUSB30 are 2 and 4 channel Transient Voltage Suppressor (TVS) based Electrostatic Discharge (ESD) protection diode arrays. The TPDxEUSB30/A devices are rated to dissipate ESD strikes at the maximum contact level specified in the IEC 61000-4-2 international standard (Contact). These devices also offer 5 A ( $8 / 20 \mu \mathrm{~s}$ ) peak pulse current ratings per IEC 61000-4-5 (surge) specification.

### 7.2 Functional Block Diagrams

![img-11.jpeg](img-11.jpeg)

Figure 7-1. TPD4EUSB30 Circuit
![img-12.jpeg](img-12.jpeg)

Figure 7-2. TPD2EUSB30/A Circuit

### 7.3 Feature Description

TPDxEUSB30/A is a family of uni-directional Electrostatic Discharge (ESD) protection devices with low capacitance. Each IO line is rated to dissipate ESD strikes at or above the maximum level specified in the IEC 61000-4-2 (Level 4 Contact) international standard. The TPDxEUSB30/A's low loading capacitance makes it ideal for protection super speed high-speed signals.

### 7.4 Device Functional Modes

The TPDxEUSB30/A family of devices are passive integrated circuits that activate whenever voltages above $\mathrm{V}_{\mathrm{BR}}$ or below the lower diodes $\mathrm{V}_{\text {forward }}(-0.6 \mathrm{~V})$ are present upon the circuit being protected. During ESD events, voltages as high as $\pm 8 \mathrm{kV}$ (contact) can be directed to ground via the internal diode network. Once the voltages on the protected lines fall below the trigger voltage of the device (usually within 10's of nano-seconds) the device reverts to passive.# 8 Application and Implementation 

## Note

Information in the following applications sections is not part of the TI component specification, and TI does not warrant its accuracy or completeness. TI's customers are responsible for determining suitability of components for their purposes, as well as validating and testing their design implementation to confirm system functionality.

### 8.1 Application Information

The TPDxEUSB30/A family is a family of diode array type transient voltage suppressors (TVS) which are typically used to provide a path to ground for dissipating ESD events on hi-speed signal lines between a human interface connector and a system. As the current from ESD passes through the TVS, only a small voltage drop is present across the diode. This is the voltage presented to the protected IC. The low $R_{D Y N}$ of the triggered TVS holds this voltage, $\mathrm{V}_{\text {CLAMP }}$, to a tolerable level to the protected IC.

### 8.2 Typical Application

This application describes a TPDxEUSB30/A eye pattern test. Figure 10-2 shows the lab board that was designed to demonstrate the degradation of the eye pattern quality with and without the TPD2EUSB30/A in the USB 3.0 signal path. The measurements show that there is only $\sim 2 \mathrm{ps}$ jitter penalty to the differential signal when the TPD2EUSB30/A device is added in the signal path. A similar setup was employed to measure the eye diagram for the TPD4EUSB30.
![img-13.jpeg](img-13.jpeg)

Figure 8-1. Measurement Setup to collect the Eye Pattern on a Reference Board with TPD2EUSB30/A
![img-14.jpeg](img-14.jpeg)

Figure 8-2. Measurement Setup to collect the Eye Pattern on a Reference Board with TPD2EUSB30/A

### 8.2.1 Design Requirements

For this design example, a single TPD2EUSB30/A is used to protect a differential data pair lines, similar to a USB 3.0 application. Given the USB application, the following parameters are known.

Table 8-1. Design Parameters

| DESIGN PARAMETER | VALUE |
| :--: | :--: |
| Signal range on D+, and D- | 0 V to 3.3 V |
| Operating Frequency | 2.5 GHz |# 8.2.2 Detailed Design Procedure 

To begin the design process, some parameters must be decided upon; the designer needs to know the following:

- Signal range on all the protected lines
- Operating frequency


### 8.2.2.1 Signal Range on D+, D- Pins

The TPD2EUSB30 has 2 pins which support 0 to 5.5 V and the TPD2EUSB30A has 2 pins which support 0 to 3.6 V .

### 8.2.2.2 Operating Frequency

The 0.7 pF (TPD2EUSB30/A typ) line capacitance supports data rates in excess of 5 Gbps .

### 8.2.3 Application Curves

![img-15.jpeg](img-15.jpeg)

Figure 8-3. Output Eye Diagram Without TPD2EUSB30/A (Figure 8-2 Setup, 5 Gbps Data Rate)
![img-16.jpeg](img-16.jpeg)

Figure 8-4. Output Eye Diagram With the TPD2EUSB30/A (Figure 8-2 Setup, 5 Gbps Data Rate)
![img-17.jpeg](img-17.jpeg)

Figure 8-5. Output Eye Diagram Without the TPD4EUSB30 (5 Gbps Data Rate)
![img-18.jpeg](img-18.jpeg)

Figure 8-6. Output Eye Diagram with the TPD4EUSB30 (5 Gbps Data Rate)# 9 Power Supply Recommendations 

This family of devices are passive ESD protection devices and there is no need to power them. Care should be taken to not violate the maximum voltage specification to ensure that the device functions properly. The D+ and D - lines share a TVS diode which can tolerate up to 6 V .

## 10 Layout

### 10.1 Layout Guidelines

- The optimum placement is as close to the connector as possible.
- EMI during an ESD event can couple from the trace being struck to other nearby unprotected traces, resulting in early system failures.
- The PCB designer needs to minimize the possibility of EMI coupling by keeping any unprotected traces away from the protected traces which are between the TVS and the connector.
- Route the protected traces as straight as possible.
- Eliminate any sharp corners on the protected traces between the TVS and the connector by using rounded corners with the largest radii possible.
- Electric fields tend to build up on corners, increasing EMI coupling.

Refer to Figure 10-1, the TPD2EUSB30/A are offered in space saving DRT package. The DRT is a 1-mm $\times$ 1-mm package with flow-through pin-mapping for the high-speed differential lines. The TPD4EUSB30 is offered in space saving DQA package. The DQA is a $1-\mathrm{mm} \times 2.5-\mathrm{mm}$ package with flow-through pin-mapping for the high-speed differential lines. It is recommended to place the package right next to the USB 3.0 connector. The GND pin should connected to GND plane of the board through a large VIA. If a dedicated GND plane is not present right underneath, it is recommended to route to the GND plane through a wide trace. The current associated with IEC ESD stress can be in the range of 30Amps or higher momentarily. A good, low impedance GND path ensures the system robustness against IEC ESD stress.
The TPDxEUSB30/A can provide system level ESD protection to the high-speed differential ports (> 5 Gbps data rate). The flow-through package offers flexibility for board routing with traces up to 15 mills wide. It allows the differential signal pairs couple together right after they touch the ESD ports of the TPDxEUSB30/A.# 10.2 Layout Examples 

![img-19.jpeg](img-19.jpeg)

Three TPD2EUSB30 to Protect USB3.0 Class A connector (One Layer Routing)
![img-20.jpeg](img-20.jpeg)

One TPD4EUSB30 \& One TPD2EUSB30 to Protect USB3.0 Class A connec tor (Two Layer Routing)
Figure 10-1. TPDxEUSB30/A at the USB3.0 Class A Connector![img-21.jpeg](img-21.jpeg)

Figure 10-2. TPDxEUSB30/A EVM - TPD4EUSB30 Side
![img-22.jpeg](img-22.jpeg)

Figure 10-3. TPDxEUSB30/A EVM TPD2EUSB30/A Side# 11 Device and Documentation Support 

### 11.1 Receiving Notification of Documentation Updates

To receive notification of documentation updates, navigate to the device product folder on ti.com. Click on Subscribe to updates to register and receive a weekly digest of any product information that has changed. For change details, review the revision history included in any revised document.

### 11.2 Support Resources

TI E2E ${ }^{\text {TM }}$ support forums are an engineer's go-to source for fast, verified answers and design help - straight from the experts. Search existing answers or ask your own question to get the quick design help you need.
Linked content is provided "AS IS" by the respective contributors. They do not constitute TI specifications and do not necessarily reflect TI's views; see TI's Terms of Use.

### 11.3 Trademarks

TI E2E ${ }^{\text {TM }}$ is a trademark of Texas Instruments.
All trademarks are the property of their respective owners.

### 11.4 Electrostatic Discharge Caution

This integrated circuit can be damaged by ESD. Texas Instruments recommends that all integrated circuits be handled with appropriate precautions. Failure to observe proper handling and installation procedures can cause damage.
ESD damage can range from subtle performance degradation to complete device failure. Precision integrated circuits may be more susceptible to damage because very small parametric changes could cause the device not to meet its published specifications.

### 11.5 Glossary

TI Glossary This glossary lists and explains terms, acronyms, and definitions.

## 12 Mechanical, Packaging, and Orderable Information

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
|  TPD2EUSB30ADRTR | Active | Production | SOT-9X3 (DRT) | 3 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 5S  |
|  TPD2EUSB30ADRTR.B | Active | Production | SOT-9X3 (DRT) | 3 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 5S  |
|  TPD2EUSB30ADRTRG4 | Active | Production | SOT-9X3 (DRT) | 3 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 5S  |
|  TPD2EUSB30ADRTRG4.B | Active | Production | SOT-9X3 (DRT) | 3 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 5S  |
|  TPD2EUSB30DRTR | Active | Production | SOT-9X3 (DRT) | 3 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 5P  |
|  TPD2EUSB30DRTR.B | Active | Production | SOT-9X3 (DRT) | 3 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 5P  |
|  TPD2EUSB30DRTRG4 | Active | Production | SOT-9X3 (DRT) | 3 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 5P  |
|  TPD2EUSB30DRTRG4.B | Active | Production | SOT-9X3 (DRT) | 3 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | 5P  |
|  TPD4EUSB30DQAR | Active | Production | USON (DQA) | 10 | 3000 | LARGE T\&R | Yes | NIPDAUAG | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85  |
|  TPD4EUSB30DQAR.B | Active | Production | USON (DQA) | 10 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (667, 660, 66R, 66
V, BMR, CE5)  |
|  TPD4EUSB30DQARG4 | Active | Production | USON (DQA) | 10 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | (667, 660, 66R, 66
V, BMR, CE5)  |
|  TPD4EUSB30DQARG4.B | Active | Production | USON (DQA) | 10 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | $-40$ to 85 | CE5  |

${ }^{(1)}$ Status: For more details on status, see our product life cycle. ${ }^{(2)}$ Material type: When designated, preproduction parts are prototypes/experimental devices, and are not yet approved or released for full production. Testing and final process, including without limitation quality assurance, reliability performance testing, and/or process qualification, may not yet be complete, and this item is subject to further changes or possible discontinuation. If available for ordering, purchases will be subject to an additional waiver at checkout, and are intended for early internal evaluation purposes only. These items are sold without warranties of any kind. ${ }^{(3)}$ RoHS values: Yes, No, RoHS Exempt. See the TI RoHS Statement for additional information and value definition. ${ }^{(4)}$ Lead finish/Ball material: Parts may have multiple material finish options. Finish options are separated by a vertical ruled line. Lead finish/Ball material values may wrap to two lines if the finish value exceeds the maximum column width. ${ }^{(5)}$ MSL rating/Peak reflow: The moisture sensitivity level ratings and peak solder (reflow) temperatures. In the event that a part has multiple moisture sensitivity ratings, only the lowest level per JEDEC standards is shown. Refer to the shipping label for the actual reflow temperature that will be used to mount the part to the printed circuit board. ${ }^{(6)}$ Part marking: There may be an additional marking, which relates to the logo, the lot trace code information, or the environmental category of the part.Multiple part markings will be inside parentheses. Only one part marking contained in parentheses and separated by a "-" will appear on a part. If a line is indented then it is a continuation of the previous line and the two combined represent the entire part marking for that device.

Important Information and Disclaimer:The information provided on this page represents TI's knowledge and belief as of the date that it is provided. TI bases its knowledge and belief on information provided by third parties, and makes no representation or warranty as to the accuracy of such information. Efforts are underway to better integrate information from third parties. TI has taken and continues to take reasonable steps to provide representative and accurate information but may not have conducted destructive testing or chemical analysis on incoming materials and chemicals. TI and TI suppliers consider certain information to be proprietary, and thus CAS numbers and other limited information may not be available for release.

In no event shall TI's liability arising out of such information exceed the total purchase price of the TI part(s) at issue in this document sold by TI to Customer on an annual basis.# TAPE AND REEL INFORMATION 

![img-23.jpeg](img-23.jpeg)

TAPE DIMENSIONS
![img-24.jpeg](img-24.jpeg)

| A0 | Dimension designed to accommodate the component width |
| :-- | :-- |
| B0 | Dimension designed to accommodate the component length |
| K0 | Dimension designed to accommodate the component thickness |
| W | Overall width of the carrier tape |
| P1 | Pitch between successive cavity centers |

QUADRANT ASSIGNMENTS FOR PIN 1 ORIENTATION IN TAPE
![img-25.jpeg](img-25.jpeg)
*All dimensions are nominal

| Device | Package <br> Type | Package <br> Drawing | Pins | SPQ | Reel <br> Diameter <br> (mm) | Reel <br> Width <br> W1 (mm) | A0 <br> (mm) | B0 <br> (mm) | K0 <br> (mm) | P1 <br> (mm) | W <br> (mm) | Pin1 <br> Quadrant |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TPD2EUSB30ADRTR | SOT-9X3 | DRT | 3 | 3000 | 180.0 | 8.4 | 1.16 | 1.16 | 0.63 | 4.0 | 8.0 | Q3 |
| TPD2EUSB30ADRTRG4 | SOT-9X3 | DRT | 3 | 3000 | 180.0 | 8.4 | 1.16 | 1.16 | 0.63 | 4.0 | 8.0 | Q3 |
| TPD2EUSB30DRTR | SOT-9X3 | DRT | 3 | 3000 | 180.0 | 8.4 | 1.16 | 1.16 | 0.63 | 4.0 | 8.0 | Q3 |
| TPD2EUSB30DRTRG4 | SOT-9X3 | DRT | 3 | 3000 | 180.0 | 8.4 | 1.16 | 1.16 | 0.63 | 4.0 | 8.0 | Q3 |
| TPD4EUSB30DQAR | USON | DQA | 10 | 3000 | 180.0 | 8.4 | 1.2 | 2.7 | 0.63 | 4.0 | 8.0 | Q1 |
| TPD4EUSB30DQARG4 | USON | DQA | 10 | 3000 | 180.0 | 8.4 | 1.2 | 2.7 | 0.63 | 4.0 | 8.0 | Q1 |# PACKAGE MATERIALS INFORMATION

www.ti.com 18-Jun-2025

![img-26.jpeg](img-26.jpeg)

*All dimensions are nominal

|  Device | Package Type | Package Drawing | Pins | SPQ | Length (mm) | Width (mm) | Height (mm)  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  TPD2EUSB30ADRTR | SOT-9X3 | DRT | 3 | 3000 | 183.0 | 183.0 | 20.0  |
|  TPD2EUSB30ADRTRG4 | SOT-9X3 | DRT | 3 | 3000 | 183.0 | 183.0 | 20.0  |
|  TPD2EUSB30DRTR | SOT-9X3 | DRT | 3 | 3000 | 183.0 | 183.0 | 20.0  |
|  TPD2EUSB30DRTRG4 | SOT-9X3 | DRT | 3 | 3000 | 183.0 | 183.0 | 20.0  |
|  TPD4EUSB30DQAR | USON | DQA | 10 | 3000 | 210.0 | 185.0 | 35.0  |
|  TPD4EUSB30DQARG4 | USON | DQA | 10 | 3000 | 210.0 | 185.0 | 35.0  |

Pack Materials-Page 2# GENERIC PACKAGE VIEW 

## DQA 10

## USON - 0.55 mm max height

$1 \times 2.5,0.5 \mathrm{~mm}$ pitch

PLASTIC SMALL OUTLINE - NO LEAD

This image is a representation of the package family, actual package may vary.
Refer to the product data sheet for package details.
![img-27.jpeg](img-27.jpeg)![img-28.jpeg](img-28.jpeg)
![img-29.jpeg](img-29.jpeg)
![img-30.jpeg](img-30.jpeg)
![img-31.jpeg](img-31.jpeg)
![img-32.jpeg](img-32.jpeg)
![img-33.jpeg](img-33.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.![img-34.jpeg](img-34.jpeg)

NOTES: (continued)
3. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/lit/slua271).![img-35.jpeg](img-35.jpeg)

NOTES: (continued)
4. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.![img-36.jpeg](img-36.jpeg)

# DQA0010B 

## PACKAGE OUTLINE

## USON - 0.55 mm max height

PLASTIC SMALL OUTLINE - NO LEAD
![img-37.jpeg](img-37.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.![img-38.jpeg](img-38.jpeg)

NOTES: (continued)
3. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/lit/slua271).![img-39.jpeg](img-39.jpeg)

NOTES: (continued)
4. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.![img-40.jpeg](img-40.jpeg)

NOTES: A. All linear dimensions are in millimeters. Dimensioning and tolerancing per ASME Y14.5M-1994.
B. This drawing is subject to change without notice.

Body dimensions do not include mold flash, interlead flash, protrusions, or gate burrs. Mold flash, interlead flash, protrusions, or gate burrs shall not exceed 0,10 per end or side.
D. JEDEC package registration is pending.![img-41.jpeg](img-41.jpeg)

NOTES: A. All linear dimensions are in millimeters.
B. This drawing is subject to change without notice.
C. Publication IPC-7351 is recommended for alternate designs.
D. Customers should contact their board fabrication site for minimum solder mask web tolerances between signal pads.
E. Maximum stencil thickness $0,1016 \mathrm{~mm}$ ( 4 mils). All linear dimensions are in millimeters.
F. Laser cutting apertures with trapezoidal walls and also rounding corners will offer better paste release. Customers should contact their board assembly site for stencil design recommendations. Refer to IPC 7525 for stencil design considerations.
G. Side aperture dimensions over-print land for acceptable area ratio $>0.66$. Customer may reduce side aperture dimensions if stencil manufacturing process allows for sufficient release at smaller opening.# IMPORTANT NOTICE AND DISCLAIMER 

TI PROVIDES TECHNICAL AND RELIABILITY DATA (INCLUDING DATA SHEETS), DESIGN RESOURCES (INCLUDING REFERENCE DESIGNS), APPLICATION OR OTHER DESIGN ADVICE, WEB TOOLS, SAFETY INFORMATION, AND OTHER RESOURCES "AS IS" AND WITH ALL FAULTS, AND DISCLAIMS ALL WARRANTIES, EXPRESS AND IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE OR NON-INFRINGEMENT OF THIRD PARTY INTELLECTUAL PROPERTY RIGHTS.
These resources are intended for skilled developers designing with TI products. You are solely responsible for (1) selecting the appropriate TI products for your application, (2) designing, validating and testing your application, and (3) ensuring your application meets applicable standards, and any other safety, security, regulatory or other requirements.
These resources are subject to change without notice. TI grants you permission to use these resources only for development of an application that uses the TI products described in the resource. Other reproduction and display of these resources is prohibited. No license is granted to any other TI intellectual property right or to any third party intellectual property right. TI disclaims responsibility for, and you will fully indemnify TI and its representatives against, any claims, damages, costs, losses, and liabilities arising out of your use of these resources.
TI's products are provided subject to TI's Terms of Sale or other applicable terms available either on ti.com or provided in conjunction with such TI products. TI's provision of these resources does not expand or otherwise alter TI's applicable warranties or warranty disclaimers for TI products.
TI objects to and rejects any additional or different terms you may have proposed.
Mailing Address: Texas Instruments, Post Office Box 655303, Dallas, Texas 75265
Copyright © 2025, Texas Instruments Incorporated