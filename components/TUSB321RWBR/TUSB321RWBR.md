# TUSB321 USB Type-C ${ }^{\text {TM }}$ Configuration Channel Logic and Port Control with VCONN 

## 1 Features

- USB Type-C ${ }^{\text {TM }}$ Specification 1.1
- Backward Compatible with USB Type-C Specification 1.0
- Supports Up to 3 A of Current Advertisement through dedicated Current Mode pin
- Mode Configuration
- Host Only - DFP (Source)
- Device Only - UFP (Sink)
- Dual Role Port - DRP
- Channel Configuration (CC)
- Attach of USB Port Detection
- Cable Orientation Detection
- Role Detection
- Type-C Current Mode advertisement and detection (Default, Medium, High)
- $\mathrm{V}_{\text {BUS }}$ Detection
- VCONN Support for Active Cables
- Cable Detection and Direction Control for External Switches
- Supply Voltage: 4.5 V to 5.5 V
- Low Current Consumption


## 2 Applications

- Host, Device, Dual Role Port Applications
- Mobile Phones
- Tablets and Notebooks
- USB Peripherals


## Simplified Schematic

![img-0.jpeg](img-0.jpeg)

## 3 Description

The TUSB321 device enables USB Type-C ports with the configuration channel (CC) logic needed for TypeC ecosystems. The TUSB321 device uses the CC pins to determine port attach and detach, cable orientation, role detection, and port control for Type-C current mode. The TUSB321 device can be configured as a downstream facing port (DFP), upstream facing port (UFP) or a dual role port (DRP) making it ideal for any application.
The TUSB321 device when configured as a DRP alternates configuration as a DFP or UFP according to the Type-C Specifications. The CC logic block monitors the CC1 and CC2 pins for pullup or pulldown resistances to determine when a USB port has been attached, the orientation of the cable, and the role detected. The CC logic detects the Type-C current mode as default, medium, or high depending on the role detected. $\mathrm{V}_{\text {BUS }}$ detection is implemented to determine a successful attach in UFP and DRP modes.
The device operates over a wide supply range and has low-power consumption.

Device Information ${ }^{(1)}$

| PART NUMBER | PACKAGE | BODY SIZE (NOM) |
| :-- | :-- | :-- |
| TUSB321 | X2QFN (12) | $1.60 \mathrm{~mm} \times 1.60 \mathrm{~mm}$ |

(1) For all available packages, see the orderable addendum at the end of the data sheet.

## Sample Applications

![img-1.jpeg](img-1.jpeg)

An IMPORTANT NOTICE at the end of this data sheet addresses availability, warranty, changes, use in safety-critical applications, intellectual property matters and other important disclaimers. PRODUCTION DATA.# Table of Contents 

1 Features ..... 1
2 Applications ..... 1
3 Description ..... 1
4 Revision History ..... 2
5 Pin Configuration and Functions ..... 3
6 Specifications ..... 4
6.1 Absolute Maximum Ratings ..... 4
6.2 ESD Ratings ..... 4
6.3 Recommended Operating Conditions ..... 4
6.4 Thermal Information ..... 4
6.5 Electrical Characteristics ..... 5
6.6 Switching Characteristics ..... 6
7 Detailed Description ..... 8
7.1 Overview ..... 8
7.2 Functional Block Diagram ..... 9
7.3 Feature Description ..... 9
7.4 Device Functional Modes ..... 11
8 Application and Implementation ..... 13
8.1 Application Information ..... 13
8.2 Typical Application ..... 13
8.3 Initialization Set Up ..... 17
9 Power Supply Recommendations ..... 17
10 Layout ..... 17
10.1 Layout Guidelines ..... 17
10.2 Layout Example ..... 17
11 Device and Documentation Support ..... 18
11.1 Receiving Notification of Documentation Updates ..... 18
11.2 Community Resources ..... 18
11.3 Trademarks ..... 18
11.4 Electrostatic Discharge Caution ..... 18
11.5 Glossary ..... 18
12 Mechanical, Packaging, and Orderable Information ..... 18

## 4 Revision History

Changes from Revision B (September 2016) to Revision C Page

- Deleted Feature "Industrial Temperature Range of -40 to $85^{\circ} \mathrm{C}$ " ..... 1
- Deleted text from the Description: "The TUSB321 device is available in industrial and commercial temperature ranges." ..... 1
- Changed pin VBUS_DET description From: $900-\mathrm{k} \Omega$ To: $R_{\text {VBUS }}$ in Pin Functions table. ..... 3
- Changed $R_{\text {VBUS }}$ values From: MIN $=891$, TYP $=900$, MAX $=909 \mathrm{~K} \Omega$ To: MIN $=855$, TYP $=887$, MAX $=920 \mathrm{~K} \Omega$ ..... 6
- Changed resister value From: $900 \mathrm{k} \Omega$ To: To: $R_{\text {VBUS }}$ in Figure 3 ..... 8
- Changed resister value From: $900 \mathrm{k} \Omega$ To: To: $R_{\text {VBUS }}$ in Functional Block Diagram ..... 9
- Changed From: The system $\mathrm{V}_{\text {BUS }}$ voltage must be routed through a $900-\mathrm{k} \Omega$ resistor to the VBUS_DET pin .. To: The system $\mathrm{V}_{\text {BUS }}$ voltage must be routed through a $\mathrm{R}_{\text {VBUS }}$ resistor to the VBUS_DET pin .. in the $V_{\text {BUS }}$ Detection ..... 11
- Added resister $R_{\text {VBUS }}$ in Figure 4 ..... 14
- Added row for $R_{\text {VBUS }}$ to Table 4 ..... 15
- Changed From: must be connected through a $900-\mathrm{k} \Omega$ resistor to $\mathrm{V}_{\text {BUS }}$ on the Type-C... To: must be connected through a $R_{\text {VBUS }}$ resistor to $V_{\text {BUS }}$ on the Type-C .. in the Detailed Design Procedure ..... 15
Changes from Revision A (June 2015) to Revision B Page
- Changed pins CC1 and CC2 values From: MIN $=-0.3 \mathrm{MAX}=\mathrm{V}_{\mathrm{DD}}+0.3$ To: MIN -0.3 MAX $=6$ in the Absolute Maximum Ratings ..... 4
Changes from Original (June 2015) to Revision A Page
- Changed device status from Product Preview to Production Data ..... 1# 5 Pin Configuration and Functions 

![img-2.jpeg](img-2.jpeg)

Pin Functions

| PIN |  | TYPE | DESCRIPTION |
| :--: | :--: | :--: | :--: |
| NAME | NO. |  |  |
| CC1 | 1 | I/O | Type-C configuration channel signal 1 |
| CC2 | 2 | I/O | Type-C configuration channel signal 2 |
| CURRENT_MODE | 3 | I | Advertise VBUS current. This 3-level input is used to control current advertisement in DFP mode or DRP mode connected as source. (See Table 2.) <br> L - Default Current. Pull-down to GND or leave unconnected. <br> M - Medium (1.5A) current. Pull-up to $\mathrm{V}_{\mathrm{DD}}$ with $500-\mathrm{k} \Omega$ resistor. <br> H - High (3.0A) current. Pull-up to $\mathrm{V}_{\mathrm{DD}}$ with $10-\mathrm{k} \Omega$ resistor. |
| PORT | 4 | I | Tri-level input pin to indicate port mode. The state of this pin is sampled when VDD is active. H - DFP (Pull-up to $\mathrm{V}_{\mathrm{DD}}$ if DFP mode is desired) NC - DRP (Leave unconnected if DRP mode is desired) L - UFP (Pull-down or tie to GND if UFP mode is desired) |
| VBUS_DET | 5 | I | 5- to $28-\mathrm{V} \mathrm{V}_{\text {BUS }}$ input voltage. $\mathrm{V}_{\text {BUS }}$ detection determines UFP attachment. One $\mathrm{R}_{\text {VBUS }}$ external resistor required between system $\mathrm{V}_{\text {BUS }}$ and VBUS_DET pin. |
| VCONN_FAULT | 6 | 0 | Open-drain output and is asserted low for $\mathrm{t}_{\text {FAULT }}$ when VCONN over-current fault is detected. (See Figure 2.) |
| OUT1 | 7 | I/O | This pin is an open drain output for communicating Type-C current mode detect when the device is in UFP mode. Default current mode detected (H); medium or high current mode detected (L). (See Table 2.) |
| OUT2 | 8 | I/O | This pin is an open drain output for communicating Type-C current mode detect when the device is in UFP mode: default or medium current mode detected (H); high current mode detected (L). (See Table 2.) |
| ID | 9 | 0 | Open drain output; asserted low when the CC pins detect device attachment when port is a source (DFP), or dual-role (DRP) acting as source (DFP). |
| GND | 10 | G | Ground |
| DIR | 11 | 0 | DIR of plug. This open drain output indicates the detected plug orientation: Type-C plug position $2(\mathrm{H})$; Type-C plug position $1(\mathrm{~L})$. |
| VDD | 12 | $P$ | Positive supply voltage |# 6 Specifications 

### 6.1 Absolute Maximum Ratings

over operating free-air temperature range (unless otherwise noted) ${ }^{(1)}$

|  |  | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: |
| Supply voltage | $\mathrm{V}_{\mathrm{DD}}$ | $-0.3$ | 6 | V |
| Control pins | PORT, CURRENT_MODE, ID, DIR, VCONN_FAULT | $-0.3$ | $\mathrm{V}_{\mathrm{DD}}+0.3$ | V |
|  | CC1, CC2 | $-0.3$ | 6 |  |
|  | OUT1, OUT2 | $-0.3$ | $\mathrm{V}_{\mathrm{DD}}+0.3$ |  |
|  | VBUS_DET | $-0.3$ | 4 |  |
| Storage temperature, $\mathrm{T}_{\text {stg }}$ |  | $-65$ | 150 | ${ }^{\circ} \mathrm{C}$ |

(1) Stresses beyond those listed under Absolute Maximum Ratings may cause permanent damage to the device. These are stress ratings only, which do not imply functional operation of the device at these or any other conditions beyond those indicated under Recommended Operating Conditions. Exposure to absolute-maximum-rated conditions for extended periods may affect device reliability.

### 6.2 ESD Ratings

|  |  |  | VALUE | UNIT |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {(ESD) }}$ | Electrostatic discharge | Human-body model (HBM), per ANSI/ESDA/JEDEC JS-001 ${ }^{(1)}$ | $\pm 7000$ | V |
|  |  | Charged-device model (CDM), per JEDEC specification JESD22- <br> C101 ${ }^{(2)}$ | $\pm 1500$ |  |

(1) JEDEC document JEP155 states that 500-V HBM allows safe manufacturing with a standard ESD control process.
(2) JEDEC document JEP157 states that 250-V CDM allows safe manufacturing with a standard ESD control process.

### 6.3 Recommended Operating Conditions

over operating free-air temperature range (unless otherwise noted)

|  |  | MIN | NOM | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{DD}}$ | Supply voltage range | 4.5 |  | 5.5 | V |
| $\mathrm{V}_{\text {BUS }}$ | System $\mathrm{V}_{\text {BUS }}$ voltage | 4 | 5 | 28 | V |
| VBUS_DET | VBUS_DET threshold voltage on the pin |  |  | 4 | V |
| VCONN | Supply for active cable (With $\mathrm{V}_{\mathrm{DD}}$ at 5 V ) | 4.75 |  | 5.5 | V |
| $T_{A}$ | Operating free air temperature range | 0 | 25 | 70 | ${ }^{\circ} \mathrm{C}$ |

### 6.4 Thermal Information

| THERMAL METRIC ${ }^{(1)}$ |  | RWB (X2QFN) |  |
| :--: | :--: | :--: | :--: |
|  |  | 12 PINS | UNIT |
| $R_{5 \text { iJA }}$ | Junction-to-ambient thermal resistance | 169.3 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{5 \text { (c)top) }}$ | Junction-to-case (top) thermal resistance | 68.1 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{5 \text { iJB }}$ | Junction-to-board thermal resistance | 83.4 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\forall_{\text {JT }}$ | Junction-to-top characterization parameter | 2.2 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\forall_{\text {JB }}$ | Junction-to-board characterization parameter | 83.4 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{5 \text { (c) }}$ (bot) | Junction-to-case (bottom) thermal resistance | N/A | — |

(1) For more information about traditional and new thermal metrics, see the Semiconductor and C Package Thermal Metrics application report.# 6.5 Electrical Characteristics 

over operating free-air temperature range (unless otherwise noted)

| PARAMETER |  | TEST <br> CONDITIONS | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Power Consumption |  |  |  |  |  |  |
| IUNATTACHED_UFP | Current consumption in unattached mode when port is unconnected and waiting for connection. ( $\mathrm{V}_{\mathrm{DD}}=5 \mathrm{~V}$, PORT $=\mathrm{L}$ ) |  |  | 100 |  | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {ACTIVE_UFP }}$ | Current consumption in active mode. ( $\mathrm{V}_{\mathrm{DD}}=5 \mathrm{~V}$, PORT $=$ L) |  |  | 100 |  | $\mu \mathrm{A}$ |
| CC1 and CC2 Pins |  |  |  |  |  |  |
| $R_{C C, D}$ | Pulldown resistor when in UFP or DRP mode. |  | 4.6 | 5.1 | 5.6 | k $\Omega$ |
| $\mathrm{V}_{\text {TH_UFP_CC_USB }}$ | Voltage threshold for detecting a DFP attach when configured as a UFP and DFP is advertising default current source capability. |  | 0.15 | 0.2 | 0.25 | V |
| $\mathrm{V}_{\text {TH_UFP_CC_MED }}$ | Voltage threshold for detecting a DFP attach when configured as a UFP and DFP is advertising medium (1.5 A) current source capability. |  | 0.61 | 0.66 | 0.7 | V |
| $\mathrm{V}_{\text {TH_UFP_CC_HIGH }}$ | Voltage threshold for detecting a DFP attach when configured as a UFP and DFP is advertising high (3 A) current source capability. |  | 1.169 | 1.23 | 1.29 | V |
| $\mathrm{V}_{\text {TH_DFP_CC_USB }}$ | Voltage threshold for detecting a UFP attach when configured as a DFP and advertising default current source capability. |  | 1.51 | 1.6 | 1.64 | V |
| $\mathrm{V}_{\text {TH_DFP_CC_MED }}$ | Voltage threshold for detecting a UFP attach when configured as a DFP and advertising medium current (1.5 A) source capability. |  | 1.51 | 1.6 | 1.64 | V |
| $\mathrm{V}_{\text {TH_DFP_CC_HIGH }}$ | Voltage threshold for detecting a active cable attach when configured as a DFP and advertising high current (3.0 A) source capability. |  | 2.46 | 2.6 | 2.74 | V |
| $\mathrm{V}_{\text {TH_AC_CC_USB }}$ | Voltage threshold for detecting a active cable attach when configured as a DFP and advertising default current source. |  | 0.15 | 0.20 | 0.25 | V |
| $\mathrm{V}_{\text {TH_AC_CC_MED }}$ | Voltage threshold for detecting a active cable attach when configured as a DFP and advertising medium current (1.5 A) source. |  | 0.35 | 0.40 | 0.45 | V |
| $\mathrm{V}_{\text {TH_AC_CC_HIGH }}$ | Voltage threshold for detecting a active cable attach when configured as a DFP and advertising high current (3.0 A) source. |  | 0.76 | 0.80 | 0.84 | V |
| $\mathrm{I}_{\text {CC_DEFAULT }}{ }^{\text {P }}$ | Default mode pullup current source when operating in DFP or DRP mode. |  | 64 | 80 | 96 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {CC_MED }}{ }^{\text {P }}$ | Medium (1.5 A) mode pullup current source when operating in DFP or DRP mode. |  | 166 | 180 | 194 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {CC_HIGH }}{ }^{\text {P }}$ | High (3 A) mode pullup current source when operating in DFP or DRP mode. ${ }^{(1)}$ |  | 304 | 330 | 356 | $\mu \mathrm{A}$ |
| Control Pins: PORT, CURRENT_MODE, VCONN_FAULT, DIR, ID, OUT1, OUT2 |  |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{IL}}$ | Low-level control signal input voltage, (PORT, CURRENT_MODE) |  |  |  | 0.4 | V |
| $\mathrm{V}_{\text {IM }}$ | Mid-level control signal input voltage (PORT, CURRENT_MODE) |  | $0.28 \times \mathrm{V}_{\mathrm{DD}}$ |  | $0.56 \times \mathrm{V}_{\mathrm{DD}}$ | V |
| $\mathrm{V}_{\text {IH }}$ | High-level control signal input voltage (PORT, CURRENT_MODE) |  | $\mathrm{V}_{\mathrm{DD}}-0.3$ |  |  | V |
| $\mathrm{I}_{\text {IH }}$ | High-level input current |  | $-20$ |  | 20 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {IL }}$ | Low-level input current |  | $-10$ |  | 10 | $\mu \mathrm{A}$ |
| $R_{I H}$ | Internal pullup resistance (PORT) |  |  | 588 |  | k $\Omega$ |
| $R_{I H I}$ | Internal pulldown resistance (PORT) |  |  | 1.1 |  | M $\Omega$ |
| $R_{P D, C U R}$ | Internal pulldown resistance for CURRENT_MODE pin |  |  | 275 |  | k $\Omega$ |
| $\mathrm{V}_{\text {OL }}$ | Low-level signal output voltage (open-drain) (VCONN_FAULT, ID, OUT1, OUT2) | $\mathrm{I}_{\mathrm{OL}}=-1.6 \mathrm{~mA}$ |  |  | 0.4 | V |
| $R_{P, O D e x t}$ | External pullup resistor on open drain IOs (VCONN_FAULT, ID, OUT1, OUT2) |  |  | 200 |  | k $\Omega$ |
| $R_{P, T L e x t}$ | Tri-level input external pull-up resistor (PORT) |  |  | 4.7 |  | k $\Omega$ |

(1) $\mathrm{V}_{\mathrm{DD}}$ must be 3.5 V or greater to advertise 3 A current.# Electrical Characteristics (continued) 

over operating free-air temperature range (unless otherwise noted)

| PARAMETER |  | TEST <br> CONDITIONS | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $R_{p \text { _on_med }}$ | External pull-up resistor on CURRENT_MODE pin to advertise 1.5-A current |  |  | 500 |  | $\mathrm{k} \Omega$ |
| $R_{p \text { _on_high }}$ | External pull-up resistor on CURRENT_MODE pin to advertise 3.0-A current |  |  | 10 |  | $\mathrm{k} \Omega$ |
| VBUS_DET IO Pins (Connected to System $\mathrm{V}_{\text {BUS }}$ signal through external resistor) |  |  |  |  |  |  |
| $\mathrm{V}_{\text {BUS_THR }}$ | $\mathrm{V}_{\text {BUS }}$ threshold range |  | 2.95 | 3.30 | 3.80 | V |
| $R_{V B U S}$ | External resistor between $\mathrm{V}_{\text {BUS }}$ and VBUS_DET pin |  | 855 | 887 | 920 | $\mathrm{K} \Omega$ |
| $R_{\text {VBUS_PO }}$ | Internal pulldown resistance for VBUS_DET |  |  | 95 |  | $\mathrm{K} \Omega$ |
| DIR pin (Open Drain IO) |  |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{OL}}$ | Low-level signal output voltage | $\mathrm{I}_{\mathrm{OL}}=-1.6 \mathrm{~mA}$ |  |  | 0.4 | V |
| VCONN |  |  |  |  |  |  |
| $R_{O N}$ | On resistance of the VCONN power FET |  |  |  | 1.25 | $\Omega$ |
| $V_{\text {TOL }}$ | Voltage tolerance on VCONN power FET |  |  |  | 5.5 | V |
| $V_{\text {PASS }}$ | Voltage to pass through VCONN power FET |  |  |  | 5.5 | V |
| $\mathrm{I}_{\text {VCONN }}$ | VCONN current limit; VCONN is disconnected above this value |  | 200 |  |  | mA |
| $C_{\text {BULK }}$ | Bulk capacitance on VCONN; placed on $\mathrm{V}_{\mathrm{DD}}$ supply |  | 10 |  | 200 | $\mu \mathrm{F}$ |

### 6.6 Switching Characteristics

over operating free-air temperature range (unless otherwise noted)

| PARAMETER |  | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{t}_{\text {CCCB_DEFAULT }}$ | Power on default of CC1 and CC2 voltage debounce time |  | 133 |  | ms |
| $\mathrm{t}_{\text {VBUS_DB }}$ | Debounce of VBUS_DET pin after valid $\mathrm{V}_{\text {BUS_THR }}$ (See Figure 1.) |  | 2 |  | ms |
| $\mathrm{t}_{\text {DRP DUTY_CYCLE }}$ | Power-on default of percentage of time DRP advertises DFP during a $\mathrm{T}_{\text {DRP }}$ |  | $30 \%$ |  |  |
| $t_{\text {DRP }}$ | The period TUSB321 in DFP mode completes a DFP to UFP and back advertisement. | 50 | 75 | 100 | ms |
| $t_{\text {FAULT }}$ | VCONN_FAULT asserted low time after VCONN over-current condition is detected. (See Figure 2.) | 7 | 10 | 13 | $\mu \mathrm{s}$ |![img-3.jpeg](img-3.jpeg)

Figure 1. VBUS Detect and Debounce
![img-4.jpeg](img-4.jpeg)

Figure 2. VCONN_FAULT Assertion Pulse Timing# 7 Detailed Description 

### 7.1 Overview

The USB Type-C ecosystem operates around a small form factor connector and cable that is flippable and reversible. Because of the nature of the connector, a scheme is needed to determine the connector orientation. Additional schemes are needed to determine when a USB port is attached and the acting role of the USB port (DFP, UFP, DRP), as well as to communicate Type-C current capabilities. These schemes are implemented over the CC pins according to the USB Type-C specifications. The TUSB321 device provides Configuration Channel (CC) logic for determining USB port attach and detach, role detection, cable orientation, and Type-C current mode. The TUSB321 device also contains several features such as VCONN sourcing, USB3.1 MUX direction control, mode configuration and low standby current which make this device ideal for source or sinks in USB2.0 or USB3.1 applications.

### 7.1.1 Cables, Adapters, and Direct Connect Devices

Type-C Specification 1.1 defines several cables, plugs and receptacles to be used to attach ports. The TUSB321 device supports all cables, receptacles, and plugs. The TUSB321 device does not support e-marking.

### 7.1.1.1 USB Type-C Receptacles and Plugs

Below is list of Type-C receptacles and plugs supported by the TUSB321 device:

- USB Type-C receptacle for USB2.0 and USB3.1 and full-featured platforms and devices
- USB full-featured Type-C plug
- USB2.0 Type-C plug


### 7.1.1.2 USB Type-C Cables

Below is a list of Type-C cables types supported by the TUSB321 device:

- USB full-featured Type-C cable with USB3.1 full-featured plug
- USB2.0 Type-C cable with USB2.0 plug
- Captive cable with either a USB full-featured plug or USB2.0 plug


### 7.1.1.3 Legacy Cables and Adapters

The TUSB321 device supports legacy cable adapters as defined by the Type-C Specification. The cable adapter must correspond to the mode configuration of the TUSB321 device.
![img-5.jpeg](img-5.jpeg)

Figure 3. Legacy Adapter Implementation Circuit

### 7.1.1.4 Direct Connect Devices

The TUSB321 device supports the attaching and detaching of a direct-connect device.# 7.2 Functional Block Diagram 

![img-6.jpeg](img-6.jpeg)

### 7.3 Feature Description

### 7.3.1 Port Role Configuration

The TUSB321 device can be configured as a downstream facing port (DFP), upstream facing port (UFP), or dualrole port (DRP) using the tri-level PORT pin. The PORT pin should be pulled high to $\mathrm{V}_{\mathrm{DD}}$ using a pullup resistance, low to GND or left as floated on the PCB to achieve the desired mode. This flexibility allows the TUSB321 device to be used in a variety of applications. The TUSB321 device samples the PORT pin after reset and maintains the desired mode until the TUSB321 device is reset again. Table 1 lists the supported features in each mode:# Feature Description (continued) 

Table 1. Supported Features for the v Device by Mode

| PORT PIN | HIGH <br> (DFP ONLY) | LOW <br> (UFP ONLY) | NC <br> (DRP) |
| :--: | :--: | :--: | :--: |
| SUPPORTED <br> FEATURES | Yes | Yes | Yes |
| Port attach and <br> detach | Yes | Yes | Yes |
| Cable orientation | Yes | - | Yes (DFP) |
| Current advertisement | Yes | Yes | Yes (UFP) |
| Current detection | - | Yes | Yes (DFP) |
| Active cable detection | Yes | - | Yes (DFP) |
| VCONN | Yes | - | Yes (DFP) |
| Legacy cables | Yes | Yes | Yes |
| $\mathrm{V}_{\text {BUS }}$ detection | - | Yes | Yes (UFP) |

### 7.3.1.1 Downstream Facing Port (DFP) - Source

The TUSB321 device can be configured as a DFP only by pulling the PORT pin high through a resistance to $\mathrm{V}_{\mathrm{DD}}$. In DFP mode, the TUSB321 device constantly presents Rps on both CC. In DFP mode, the TUSB321 device advertises USB Type-C current based on the state of the CURRENT_MODE pin.
When configured as a DFP, the TUSB321 can operate with older USB Type-C 1.0 devices except for a USB Type-C 1.0 DRP device. The TUSB321 can not operate with a USB Type-C 1.0 DRP device. This limitation is a result of backwards compatibility problem between USB Type-C 1.1 DFP and a USB Type-C 1.0 DRP.

### 7.3.1.2 Upstream Facing Port (UFP) - Sink

The TUSB321 device can be configured as a UFP only by pulling the PORT pin low to GND. In UFP mode, the TUSB321 device constantly presents pulldown resistors (Rd) on both CC pins. The TUSB321 device monitors the CC pins for the voltage level corresponding to the Type-C mode current advertisement by the connected DFP. The TUSB321 device debounces the CC pins and wait for $\mathrm{V}_{\text {BUS }}$ detection before successfully attaching. As a UFP, the TUSB321 device detects and communicates the advertised current level of the DFP to the system through the OUT1 and OUT2 pins.

### 7.3.1.3 Dual Role Port (DRP)

The TUSB321 device can be configured to operate as a DRP when the PORT pin is left floated on the PCB. In DRP mode, the TUSB321 device toggles between operating as a DFP and a UFP. When functioning as a DFP in DRP mode, the TUSB321 device complies with all operations as defined for a DFP according to the Type-C Specification. When presenting as a UFP in DRP mode, the TUSB321 device operates as defined for a UFP according to the Type-C Specification.

### 7.3.2 Type-C Current Mode

The TUSB321 device supports both advertising and detection of Type-C current. When TUSB321 is a UFP or a DRP connected as a sink, the OUT1 and OUT2 pins are used to inform the system the detected USB Type-C current being broadcasted by the attached DFP. When TUSB321 device is a DFP or a DRP connected as a source, the CURRENT_MODE pin is used to advertise the USB Type-C current. The current advertisement for the TUSB321 device is 500 mA (for USB2.0) or 900 mA (for USB3.1) if CURRENT_MODE pin is left unconnected or pulled to GND. If a higher level of current is required, the CURRENT_MODE can be pulled up to VDD through a $500-\mathrm{k} \Omega$ resistor to advertise medium current at 1.5 A or pulled up to VDD through a $10-\mathrm{k} \Omega$ resistor to advertise high current at 3 A . Table 2 lists the Type-C current advertisements and detection.Table 2. Type-C Current Advertisement and Detection

| TYPE-C CURRENT |  | UFP or DRP acting as UFP Current Detection | DFP or DRP acting as DFP Current Advertisement |
| :--: | :--: | :--: | :--: |
| Default | $\begin{aligned} & 500 \mathrm{~mA}(\text { USB2.0) } \\ & 900 \mathrm{~mA}(\text { USB3.1) } \end{aligned}$ | OUT1 = High <br> OUT2 = High (unattached) or Low (attached) | CURRENT_MODE $=\mathrm{L}$ |
| Medium - 1.5 A |  | OUT1 = Low OUT2 = High | CURRENT_MODE $=M$ |
| High - 3 A |  | OUT1 = Low OUT2 = Low | CURRENT_MODE $=\mathrm{H}$ |

# 7.3.3 $\mathrm{V}_{\text {BUS }}$ Detection 

The TUSB321 device supports $\mathrm{V}_{\text {BUS }}$ detection according to the Type-C Specification. $\mathrm{V}_{\text {BUS }}$ detection is used to determine the attachment and detachment of a UFP. $\mathrm{V}_{\text {BUS }}$ detection is also used to successfully resolve the role in DRP mode.
The system $\mathrm{V}_{\text {BUS }}$ voltage must be routed through a $\mathrm{R}_{\text {VBUS }}$ resistor to the VBUS_DET pin on the TUSB321 device if the PORT pin is configured as a DRP or a UFP. If the TUSB321 device is configured as a DFP and only ever used in DFP mode, the VBUS_DET pin can be left unconnected.

### 7.3.4 Cable Orientation and External MUX Control

The TUSB321 device has the ability to control an external/discrete MUX using the DIR pin. The TUSB321 detects the cable orientation by monitoring the voltage on the CC pins. When a voltage level within the proper threshold is detected on CC1, the DIR pin is pulled low. When a voltage level within the proper threshold is detected on CC2, the DIR is pulled high. If the direction polarity of the external MUX is opposite of the TUSB321, the TUSB321 CC1/CC2 connection to USB Type-C receptacle can be reversed. The DIR pin is an open drain output.

### 7.3.5 VCONN Support for Active Cables

The TUSB321 device supplies VCONN to active cables when configured in DFP mode or in DRP acting as a DFP mode. VCONN is provided only when the unconnected CC pin is terminated to a resistance, Ra, and after a UFP is detected and the Attached.SRC state is entered. When in DFP mode or in DRP acting as a DFP mode, a 5-V source must be connected to the VDD pin of the TUSB321 device after Attached.SRC. VCONN is supplied from VDD through a low resistance power FET out to the unconnected CC pin. VCONN is removed when a detach event is detected and the active cable is removed.

### 7.4 Device Functional Modes

The TUSB321 device has two functional modes. Table 3 lists these modes:
Table 3. USB Type-C States According to TUSB321 Functional Modes

| MODES | GENERAL BEHAVIOR | PORT PIN | STATES $^{(1)}$ |
| :--: | :--: | :--: | :--: |
| Unattached | USB port unattached. ID, PORT operational. CC pins configure according to PORT pin. | UFP | Unattached.SNK |
|  |  |  | AttachWait.SNK |
|  |  | DRP | Toggle Unattached.SNK $\rightarrow$ Unattached.SRC |
|  |  |  | AttachedWait.SRC or AttachedWait.SNK |
|  |  | DFP | Unattached.SRC |
|  |  |  | AttachWait.SRC |
| Active | USB port attached. All GPIOs operational. | UFP | Attached.SNK |
|  |  | DRP | Attached.SNK |
|  |  |  | Attached.SRC |
|  |  | DFP | Attached.SRC |

(1) Required; not in sequential order.# 7.4.1 Unattached Mode 

Unattached mode is the primary mode of operation for the TUSB321 device, because a USB port can be unattached for a lengthy period of time. In unattached mode, $\mathrm{V}_{\mathrm{DD}}$ is available, and all IOs are operational. After the TUSB321 device is powered up, the part enters unattached mode until a successful attach has been determined. Initially, right after power up, the TUSB321 device comes up as an Unattached.SNK. The TUSB321 device checks the PORT pin and operates according to the mode configuration. The TUSB321 device toggles between the UFP and the DFP if configured as a DRP. The PORT pin is only sampled at reset or power up.

### 7.4.2 Active Mode

Active mode is defined as the port being attached. In active mode, all GPIOs are operational. When in active mode, the TUSB321 device communicates to the AP that the USB port is attached. This happens through the ID pin if TUSB321 is configured as a DFP or DRP connect as source. If TUSB321 is configured as a UFP or a DRP connected as a sink, the OUT1 and OUT2 pins are used. The TUSB321 device exits active mode under the following conditions:

- Cable unplug
- $\mathrm{V}_{\text {BUS }}$ removal if attached as a UFP# 8 Application and Implementation 

NOTE
Information in the following applications sections is not part of the TI component specification, and TI does not warrant its accuracy or completeness. TI's customers are responsible for determining suitability of components for their purposes. Customers should validate and test their design implementation to confirm system functionality.

### 8.1 Application Information

The TUSB321 device is a Type-C configuration channel logic and port controller. The TUSB321 device can detect when a Type-C device is attached, what type of device is attached, the orientation of the cable, and power capabilities (both detection and broadcast). The TUSB321 device can be used in a source application (DFP) or in a sink application (UFP).

### 8.2 Typical Application

### 8.2.1 DFP Mode

Figure 4 shows the TUSB321 device configured as a DFP.Typical Application (continued)
![img-7.jpeg](img-7.jpeg)

Figure 4. DFP Mode Schematic# Typical Application (continued) 

### 8.2.1.1 Design Requirements

For this design example, use the parameters listed in Table 4:
Table 4. Design Requirements for DFP Mode

| DESIGN PARAMETER | VALUE |
| :--: | :--: |
| $\mathrm{V}_{\mathrm{DD}}(4.5 \mathrm{~V}$ to 5.5 V$)$ | 5 V |
| Type-C port type (UFP, DFP, or DRP) | DFP <br> PORT pin is pulled up |
| Advertised Type-C Current (Default, 1.5 A, 3.0 A) | 3.0 A |
| $\mathrm{R}_{\text {VBUS }}(855-\mathrm{k} \Omega$ to $920-\mathrm{k} \Omega)$ | $900-\mathrm{k} \Omega$ |
| VCONN Support | Yes |

### 8.2.1.2 Detailed Design Procedure

The TUSB321 device supports a $\mathrm{V}_{\mathrm{DD}}$ in the range of 4.5 to 5.5 V . In this particular case, $\mathrm{V}_{\mathrm{DD}}$ is set to 5 V . A 100nF capacitor is placed near $\mathrm{V}_{\mathrm{DD}}$. Also, a $100 \mu \mathrm{~F}$ is used to meet the USB Type-C bulk capacitance requirement of $10 \mu \mathrm{~F}$ to $220 \mu \mathrm{~F}$.
The TUSB321 current advertisement is determined by the state of the CURRENT_MODE pin. In this particular example, 3.0 A advertisement is desired so the CURRENT_MODE pin is pulled high to $\mathrm{V}_{\mathrm{DD}}$ through $10-\mathrm{k} \Omega$ resistor.
The DIR pin is used to control the MUX for connecting the USB3 SS signals to the appropriate pins on the USB Type-C receptacle. In this particular case, a HD3SS3212 is used as the MUX. In order to minimize crossing in routing the USB3 SS signals to the USB Type C connector, the connection of CC1 and CC2 to the TUSB321 is swapped.
The Type-C port mode is determined by the state of the PORT pin. When the PORT pin is pulled high, the TUSB321 device is in DFP mode.
The VBUS_DET pin must be connected through a $\mathrm{R}_{\text {VBUS }}$ resistor to $\mathrm{V}_{\text {BUS }}$ on the Type-C that is connected. This large resistor is required to protect the TUSB321 device from large $\mathrm{V}_{\text {BUS }}$ voltage that is possible in present day systems. This resistor along with internal pulldown keeps the voltage observed by the TUSB321 device in the recommended range.
The USB2 specification requires the bulk capacitance on $\mathrm{V}_{\text {BUS }}$ based on UFP or DFP. When operating the TUSB321 device in a DFP mode, a bulk capacitance of at least $120 \mu \mathrm{~F}$ is required. In this particular case, a 150$\mu \mathrm{F}$ capacitor was chosen.# 8.2.1.3 Application Curve 

![img-8.jpeg](img-8.jpeg)

Figure 5. Application Curve for DFP Mode# 8.3 Initialization Set Up 

The general power-up sequence for the TUSB321 device is as follows:

1. System is powered off (device has no $\mathrm{V}_{\mathrm{DD}}$ ). The TUSB321 device is configured internally in UFP mode with Rds on CC pins.
2. $\mathrm{V}_{\mathrm{DD}}$ ramps - POR circuit.
3. The TUSB321 device enters unattached mode and determines the voltage level from the PORT pin. This determines the mode in which the TUSB321 device operates (DFP, UFP, DRP).
4. The TUSB321 device monitors the CC pins as a DFP and $\mathrm{V}_{\text {BUS }}$ for attach as a UFP.
5. The TUSB321 device enters active mode when attach has been successfully detected.

## 9 Power Supply Recommendations

The TUSB321 device has a wide power supply range from 4.5 to 5.5 V . The TUSB321 device can be run off of a system power such as a battery.

## 10 Layout

### 10.1 Layout Guidelines

1. An extra trace (or stub) is created when connecting between more than two points. A trace connecting pin A6 to pin B6 will create a stub because the trace also has to go to the USB Host. Ensure that:

- A stub created by short on pin A6 (DP) and pin B6 (DP) at Type-C receptacle does not exceed 3.5 mm .
- A stub created by short on pin A7 (DM) and pin B7 (DM) at Type-C receptacle does not exceed 3.5 mm .

2. A 100-nF capacitor should be placed as close as possible to the TUSB321 $\mathrm{V}_{\mathrm{DD}}$ pin.

### 10.2 Layout Example

![img-9.jpeg](img-9.jpeg)

Figure 6. TUSB321 Layout# 11 Device and Documentation Support 

### 11.1 Receiving Notification of Documentation Updates

To receive notification of documentation updates, navigate to the device product folder on ti.com. In the upper right corner, click on Alert me to register and receive a weekly digest of any product information that has changed. For change details, review the revision history included in any revised document.

### 11.2 Community Resources

The following links connect to TI community resources. Linked contents are provided "AS IS" by the respective contributors. They do not constitute TI specifications and do not necessarily reflect TI's views; see TI's Terms of Use.
TI E2E ${ }^{\text {TM }}$ Online Community TI's Engineer-to-Engineer (E2E) Community. Created to foster collaboration among engineers. At e2e.ti.com, you can ask questions, share knowledge, explore ideas and help solve problems with fellow engineers.
Design Support TI's Design Support Quickly find helpful E2E forums along with design support tools and contact information for technical support.

### 11.3 Trademarks

E2E is a trademark of Texas Instruments.
USB Type-C is a trademark of USB Implementers Forum.
All other trademarks are the property of their respective owners.

### 11.4 Electrostatic Discharge Caution

These devices have limited built-in ESD protection. The leads should be shorted together or the device placed in conductive foam during storage or handling to prevent electrostatic damage to the MOS gates.

### 11.5 Glossary

SLYZ022 - TI Glossary.
This glossary lists and explains terms, acronyms, and definitions.

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
|  TUSB321RWBR | Active | Production | X2QFN (RWB) | 12 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-2-260C-1 YEAR | 0 to 70 | 21  |
|  TUSB321RWBR.A | Active | Production | X2QFN (RWB) | 12 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-2-260C-1 YEAR | 0 to 70 | 21  |
|  TUSB321RWBRG4 | Active | Production | X2QFN (RWB) | 12 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | 0 to 70 | 21  |
|  TUSB321RWBRG4.A | Active | Production | X2QFN (RWB) | 12 | 3000 | LARGE T\&R | Yes | NIPDAU | Level-1-260C-UNLIM | 0 to 70 | 21  |

${ }^{(1)}$ Status: For more details on status, see our product life cycle. ${ }^{(2)}$ Material type: When designated, preproduction parts are prototypes/experimental devices, and are not yet approved or released for full production. Testing and final process, including without limitation quality assurance, reliability performance testing, and/or process qualification, may not yet be complete, and this item is subject to further changes or possible discontinuation. If available for ordering, purchases will be subject to an additional waiver at checkout, and are intended for early internal evaluation purposes only. These items are sold without warranties of any kind. ${ }^{(3)}$ RoHS values: Yes, No, RoHS Exempt. See the TI RoHS Statement for additional information and value definition. ${ }^{(4)}$ Lead finish/Ball material: Parts may have multiple material finish options. Finish options are separated by a vertical ruled line. Lead finish/Ball material values may wrap to two lines if the finish value exceeds the maximum column width. ${ }^{(5)}$ MSL rating/Peak reflow: The moisture sensitivity level ratings and peak solder (reflow) temperatures. In the event that a part has multiple moisture sensitivity ratings, only the lowest level per JEDEC standards is shown. Refer to the shipping label for the actual reflow temperature that will be used to mount the part to the printed circuit board. ${ }^{(6)}$ Part marking: There may be an additional marking, which relates to the logo, the lot trace code information, or the environmental category of the part.

Multiple part markings will be inside parentheses. Only one part marking contained in parentheses and separated by a "-" will appear on a part. If a line is indented then it is a continuation of the previous line and the two combined represent the entire part marking for that device.

Important Information and Disclaimer:The information provided on this page represents TI's knowledge and belief as of the date that it is provided. TI bases its knowledge and belief on information provided by third parties, and makes no representation or warranty as to the accuracy of such information. Efforts are underway to better integrate information from third parties. TI has taken and continues to take reasonable steps to provide representative and accurate information but may not have conducted destructive testing or chemical analysis on incoming materials and chemicals. TI and TI suppliers consider certain information to be proprietary, and thus CAS numbers and other limited information may not be available for release.

In no event shall TI's liability arising out of such information exceed the total purchase price of the TI part(s) at issue in this document sold by TI to Customer on an annual basis.# TAPE AND REEL INFORMATION 

![img-10.jpeg](img-10.jpeg)

TAPE DIMENSIONS
![img-11.jpeg](img-11.jpeg)

| A0 | Dimension designed to accommodate the component width |
| :-- | :-- |
| B0 | Dimension designed to accommodate the component length |
| K0 | Dimension designed to accommodate the component thickness |
| W | Overall width of the carrier tape |
| P1 | Pitch between successive cavity centers |

QUADRANT ASSIGNMENTS FOR PIN 1 ORIENTATION IN TAPE
![img-12.jpeg](img-12.jpeg)

Pocket Quadrants
*All dimensions are nominal

| Device | Package <br> Type | Package <br> Drawing | Pins | SPQ | Reel <br> Diameter <br> (mm) | Reel <br> Width <br> W1 (mm) | A0 <br> (mm) | B0 <br> (mm) | K0 <br> (mm) | P1 <br> (mm) | W <br> (mm) | Pin1 <br> Quadrant |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TUSB321RWBR | X2QFN | RWB | 12 | 3000 | 180.0 | 8.4 | 1.8 | 1.8 | 0.48 | 4.0 | 8.0 | Q2 |
| TUSB321RWBRG4 | X2QFN | RWB | 12 | 3000 | 180.0 | 8.4 | 1.8 | 1.8 | 0.48 | 4.0 | 8.0 | Q2 |# PACKAGE MATERIALS INFORMATION

## TAPE AND REEL BOX DIMENSIONS

![img-13.jpeg](img-13.jpeg)

*All dimensions are nominal

|  Device | Package Type | Package Drawing | Pins | SPQ | Length (mm) | Width (mm) | Height (mm)  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  TUSB321RWBR | X2QFN | RWB | 12 | 3000 | 210.0 | 185.0 | 35.0  |
|  TUSB321RWBRG4 | X2QFN | RWB | 12 | 3000 | 210.0 | 185.0 | 35.0  |![img-14.jpeg](img-14.jpeg)
![img-15.jpeg](img-15.jpeg)
![img-16.jpeg](img-16.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.![img-17.jpeg](img-17.jpeg)

NOTES: (continued)
3. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/lit/slua271).![img-18.jpeg](img-18.jpeg)

NOTES: (continued)
4. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.# IMPORTANT NOTICE AND DISCLAIMER 

TI PROVIDES TECHNICAL AND RELIABILITY DATA (INCLUDING DATA SHEETS), DESIGN RESOURCES (INCLUDING REFERENCE DESIGNS), APPLICATION OR OTHER DESIGN ADVICE, WEB TOOLS, SAFETY INFORMATION, AND OTHER RESOURCES "AS IS" AND WITH ALL FAULTS, AND DISCLAIMS ALL WARRANTIES, EXPRESS AND IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE OR NON-INFRINGEMENT OF THIRD PARTY INTELLECTUAL PROPERTY RIGHTS.
These resources are intended for skilled developers designing with TI products. You are solely responsible for (1) selecting the appropriate TI products for your application, (2) designing, validating and testing your application, and (3) ensuring your application meets applicable standards, and any other safety, security, regulatory or other requirements.
These resources are subject to change without notice. TI grants you permission to use these resources only for development of an application that uses the TI products described in the resource. Other reproduction and display of these resources is prohibited. No license is granted to any other TI intellectual property right or to any third party intellectual property right. TI disclaims responsibility for, and you will fully indemnify TI and its representatives against, any claims, damages, costs, losses, and liabilities arising out of your use of these resources.
TI's products are provided subject to TI's Terms of Sale or other applicable terms available either on ti.com or provided in conjunction with such TI products. TI's provision of these resources does not expand or otherwise alter TI's applicable warranties or warranty disclaimers for TI products.
TI objects to and rejects any additional or different terms you may have proposed.
Mailing Address: Texas Instruments, Post Office Box 655303, Dallas, Texas 75265
Copyright © 2025, Texas Instruments Incorporated