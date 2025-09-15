# TLV758P 500-mA, High-Accuracy, Adjustable LDO in a Small Size Package 

## 1 Features

- Input voltage range: 1.5 V to 6.0 V
- Adjustable output voltage:
- 0.55 V to 5.5 V
- Low dropout:
- 130 mV (max) at 500 mA (3.3 V ${ }_{\text {OUT }}$ )
- High output accuracy: $0.7 \%$ (typical) and $1 \%$ (maximum over temperature)
- $\mathrm{I}_{\mathrm{Q}}: 25 \mu \mathrm{~A}$ (typical)
- Built-in soft-start with monotonic $V_{\text {OUT }}$ rise
- Packages:
- 2-mm $\times$ 2-mm WSON-6 (DRV)
- SOT23-5 (DBV)
- Active output discharge


## 2 Applications

- Gaming consoles
- Home theaters and entertainment
- PC and notebooks
- Connected peripherals and printers
- Rack and server power
- Thermostats
- Retail automation and payment


## 3 Description

The TLV758P is an adjustable 500-mA low-dropout (LDO) regulator. This device is available in a small, 6 -pin, $2-\mathrm{mm} \times 2-\mathrm{mm}$ WSON package and a 5 -pin SOT23 package and consumes very low quiescent current and provides fast line and load transient performance. The TLV758P features an ultra-low dropout of 130 mV at 500 mA that can help improve the power efficiency of the system.
The TLV758P is optimized for a wide variety of applications by supporting an input voltage range from 1.5 V to 6.0 V and an externally adjustable output range of 0.55 V to 5.5 V . The low output voltage enables this LDO to power the modern microcontrollers with lower core voltages.
The TLV758P is stable with small ceramic output capacitors, allowing for a small overall solution size. A precision band-gap and error amplifier provides high accuracy of $0.7 \%$ (max) at $25^{\circ} \mathrm{C}$ and $1 \%$ (max) over temperature $\left(85^{\circ} \mathrm{C}\right)$. This device includes integrated thermal shutdown, current limit, and undervoltage lockout (UVLO) features. The TLV758P has an internal foldback current limit that helps reduce the thermal dissipation during short-circuit events.

## Package Information

| PART NUMBER | PACKAGE $^{(1)}$ | PACKAGE SIZE $^{(2)}$ |
| :-- | :-- | :-- |
| TLV758P | DRV (WSON, 6) | $2 \mathrm{~mm} \times 2 \mathrm{~mm}$ |
|  | DBV (SOT-23, 5) | $2.9 \mathrm{~mm} \times 2.8 \mathrm{~mm}$ |

(1) For all available packages, see the orderable addendum at the end of the data sheet.
(2) The package size (length $\times$ width) is a nominal value and includes pins, where applicable.
![img-0.jpeg](img-0.jpeg)

Typical Application# Table of Contents 

1 Features ..... 1
2 Applications ..... 1
3 Description ..... 1
4 Pin Configuration and Functions ..... 3
5 Specifications ..... 4
5.1 Absolute Maximum Ratings ..... 4
5.2 ESD Ratings ..... 4
5.3 Recommended Operating Conditions ..... 5
5.4 Thermal Information ..... 5
5.5 Electrical Characteristics ..... 6
5.6 Typical Characteristics ..... 7
6 Detailed Description ..... 13
6.1 Overview ..... 13
6.2 Functional Block Diagram ..... 13
6.3 Feature Description ..... 13
6.4 Device Functional Modes ..... 15
7 Application and Implementation ..... 16
7.1 Application Information ..... 16
7.2 Typical Application ..... 21
7.3 Power Supply Recommendations ..... 22
7.4 Layout ..... 22
8 Device and Documentation Support ..... 24
8.1 Documentation Support ..... 24
8.2 Receiving Notification of Documentation Updates ..... 24
8.3 Support Resources ..... 24
8.4 Trademarks ..... 24
8.5 Electrostatic Discharge Caution ..... 24
8.6 Glossary ..... 24
9 Revision History ..... 25
10 Mechanical, Packaging, and Orderable Information ..... 25# 4 Pin Configuration and Functions 

![img-1.jpeg](img-1.jpeg)

Figure 4-1. DRV Package, 6-Pin Adjustable WSON (Top View)
![img-2.jpeg](img-2.jpeg)

Figure 4-2. DBV Package, 5-Pin Adjustable SOT-23
(Top View)

Table 4-1. Pin Functions

| PIN |  | I/O | DESCRIPTION |
| :--: | :--: | :--: | :--: |
| NAME | NO. |  |  |
| DNC | 5 | - | Do not connect |
| EN | 4 | Input | Enable pin. Drive EN greater than $\mathrm{V}_{\text {EN(Hi) }}$ to turn on the regulator. Drive EN less than $\mathrm{V}_{\text {EN(LO) }}$ to put the LDO into shutdown mode. |
| FB | 2 | - | This pin is used as an input to the control loop error amplifier and is used to set the output voltage of the LDO. |
| GND | 3 | - | Ground pin |
| IN | 6 | Input | Input pin. For best transient response and to minimize input impedance, use the recommended value or larger ceramic capacitor from IN to ground as listed in the Recommended Operating Conditions table and the Input and Output Capacitor Selection section. Place the input capacitor as close to the output of the device as possible. |
| OUT | 1 | Output | Regulated output voltage pin. A capacitor is required from OUT to ground for stability. For best transient response, use the nominal recommended value or larger ceramic capacitor from OUT to ground; see the Recommended Operating Conditions table and the Input and Output Capacitor Selection section. Place the output capacitor as close to output of the device as possible. |
| Thermal pad | Pad | - | Connect the thermal pad to a large area GND plane for improved thermal performance. |# 5 Specifications 

### 5.1 Absolute Maximum Ratings

over operating free-air temperature range (unless otherwise noted) ${ }^{(1)}$

|  |  | MIN | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: |
| Voltage | Supply, $\mathrm{V}_{\text {IN }}$ | $-0.3$ | 6.5 | V |
|  | Enable, $\mathrm{V}_{\mathrm{EN}}$ | $-0.3$ | 6.5 |  |
|  | Feedback, $\mathrm{V}_{\mathrm{FB}}$ | $-0.3$ | 2 |  |
|  | Output, $\mathrm{V}_{\text {OUT }}$ | $-0.3$ | $\mathrm{V}_{\text {IN }}+0.3^{(2)}$ |  |
| Temperature | Operating junction, $T_{J}$ | $-40$ | 150 | ${ }^{\circ} \mathrm{C}$ |
|  | Storage, $\mathrm{T}_{\text {stg }}$ | $-65$ | 150 |  |

(1) Stresses beyond those listed under Absolute Maximum Ratings may cause permanent damage to the device. These are stress ratings only, which do not imply functional operation of the device at these or any other conditions beyond those indicated under Recommended Operating Conditions. Exposure to absolute-maximum-rated conditions for extended periods may affect device reliability.
(2) The absolute maximum rating is $\mathrm{V}_{\mathrm{IN}}+0.3 \mathrm{~V}$ or 6.5 V , whichever is smaller.

### 5.2 ESD Ratings

|  |  |  | VALUE | UNIT |
| :-- | :-- | :-- | :--: | :--: |
| $\mathrm{V}_{\text {(ESD) }}$ | Electrostatic discharge | Human-body model (HBM), per ANSI/ESDA/JEDEC JS-001 ${ }^{(1)}$ | $\pm 2000$ | V |
|  |  | Charged-device model (CDM), per JEDEC specification JESD22-C101 ${ }^{(2)}$ | $\pm 500$ |  |

(1) JEDEC document JEP155 states that 500-V HBM allows safe manufacturing with a standard ESD control process. Manufacturing with less than 500-V HBM is possible with the necessary precautions.
(2) JEDEC document JEP157 states that 250-V CDM allows safe manufacturing with a standard ESD control process. Manufacturing with less than 250-V CDM is possible with the necessary precautions.# 5.3 Recommended Operating Conditions 

over operating free-air temperature range (unless otherwise noted)

|  |  | MIN | NOM | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {IN }}$ | Input voltage | 1.5 |  | 6.0 | V |
| $\mathrm{V}_{\text {OUT }}$ | Output voltage | 0.55 |  | 5.5 | V |
| $\mathrm{I}_{\text {OUT }}$ | Output current | 0 |  | 500 | mA |
| $\mathrm{C}_{\text {IN }}$ | Input capacitor | 1 |  |  | $\mu \mathrm{F}$ |
| $\mathrm{C}_{\text {OUT }}$ | Output capacitor ${ }^{(1)}$ | 1 |  | 220 | $\mu \mathrm{F}$ |
| $\mathrm{V}_{\text {EN }}$ | Enable voltage ${ }^{(2)}$ | 0 |  | 6.0 | V |
| $f_{E N}$ | Enable toggle frequency |  |  | 10 | kHz |
| $T_{J}$ | Junction temperature | $-40$ |  | 125 | ${ }^{\circ} \mathrm{C}$ |

(1) Minimum derated capacitance of $0.47 \mu \mathrm{~F}$ is required for stability.
(2) If $\mathrm{V}_{\mathrm{EN}}>\mathrm{V}_{\mathrm{IN}}$, when $\mathrm{V}_{\mathrm{EN}}>\mathrm{V}_{\mathrm{UVLO}}$ rising (min), the input pin (IN) must sink 1 mA of current to avoid the device being turn on with floating input pin.

### 5.4 Thermal Information

| THERMAL METRIC ${ }^{(1)}$ |  | TLV758P |  | UNIT |
| :--: | :--: | :--: | :--: | :--: |
|  |  | DBV (SOT-23) | DRV (WSON) |  |
|  |  | 5 PINS | 6 PINS |  |
| $R_{B, i A}$ | Junction-to-ambient thermal resistance | 176.9 | 80.3 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{B, i C(b i p)}$ | Junction-to-case (top) thermal resistance | 95.3 | 98.7 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{B, i B}$ | Junction-to-board thermal resistance | 45.0 | 44.8 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\psi_{J T}$ | Junction-to-top characterization parameter | 21.0 | 6.1 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\psi_{J B}$ | Junction-to-board characterization parameter | 44.8 | 45.0 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $R_{B, i C(b o t)}$ | Junction-to-case (bottom) thermal resistance | N/A | 20.8 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |

(1) For more information about traditional and new thermal metrics, see the Semiconductor and IC Package Thermal Metrics application note.# 5.5 Electrical Characteristics 

at operating temperature range $\left(T_{J}=-40^{\circ} \mathrm{C}\right.$ to $\left.+125^{\circ} \mathrm{C}\right), \mathrm{V}_{\mathrm{IN}}=\mathrm{V}_{\text {OUT( }}(\mathrm{NOM})+0.5 \mathrm{~V}$ or 1.5 V (whichever is greater), $\mathrm{I}_{\text {OUT }}=1 \mathrm{~mA}$, $\mathrm{V}_{\mathrm{EN}}=\mathrm{V}_{\mathrm{IN}}$, and $\mathrm{C}_{\mathrm{IN}}=\mathrm{C}_{\text {OUT }}=1 \mu \mathrm{~F}$ (unless otherwise noted); all typical values are at $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$

| PARAMETER |  | TEST CONDITIONS |  | MIN | TYP | MAX | UNIT |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{FB}}$ | Feedback voltage | $T_{J}=25^{\circ} \mathrm{C}$ |  |  | 0.55 |  | V |
|  | Output accuracy ${ }^{(1)}$ | $\begin{aligned} & \mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C} \\ & -40^{\circ} \mathrm{C} \leq \mathrm{T}_{\mathrm{J}} \leq+85^{\circ} \mathrm{C} \\ & -40^{\circ} \mathrm{C} \leq \mathrm{T}_{\mathrm{J}} \leq+125^{\circ} \mathrm{C} \end{aligned}$ |  | $\begin{gathered} -0.7 \% \\ -1 \% \\ -1.5 \% \end{gathered}$ |  | $\begin{gathered} 0.7 \% \\ 1 \% \\ 1.5 \% \end{gathered}$ |  |
|  |  | Line regulation | $\mathrm{V}_{\text {OUT( }}(\mathrm{NOM})+0.5 \mathrm{~V}^{(2)} \leq \mathrm{V}_{\mathrm{I}} \leq 6.0 \mathrm{~V}$ |  | 2 | 7.5 | mV |
|  | Load regulation | $0.1 \mathrm{~mA} \leq \mathrm{I}_{\text {OUT }} \leq 500 \mathrm{~mA}, \mathrm{~V}_{\text {IN }} \geq 2.0 \mathrm{~V}$ |  |  | 0.030 |  | V/A |
| $\mathrm{I}_{\text {GND }}$ | Ground current | $\mathrm{I}_{\text {OUT }}=0 \mathrm{~mA}$ | $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$ | 10 | 25 | 31 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {GND }}$ | Ground current |  | $-40^{\circ} \mathrm{C} \leq \mathrm{T}_{\mathrm{J}} \leq+125^{\circ} \mathrm{C}$ |  |  | 35 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {SHEN }}$ | Shutdown current | $\mathrm{V}_{\mathrm{EN}} \leq 0.3 \mathrm{~V}, 1.5 \mathrm{~V} \leq \mathrm{V}_{\mathrm{IN}} \leq 6.0 \mathrm{~V}$ |  |  | 0.1 | 1 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\mathrm{FB}}$ | Feedback pin current |  |  |  | 0.01 | 0.1 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\mathrm{CL}}$ | Output current limit | $\mathrm{V}_{\mathrm{IN}}=\mathrm{V}_{\text {OUT( }}(\mathrm{NOM})+1.0 \mathrm{~V}$ | $\begin{aligned} & \mathrm{V}_{\text {OUT }}=\mathrm{V}_{\text {OUT( }}(\mathrm{NOM})-0.2 \mathrm{~V}, \\ & \mathrm{~V}_{\text {OUT }}<1.5 \mathrm{~V} \end{aligned}$ | 530 | 720 | 865 | mA |
|  |  |  | $\begin{aligned} & \mathrm{V}_{\text {OUT }}=0.9 \mathrm{~V} \times \mathrm{V}_{\text {OUT( }}(\mathrm{NOM}) \text { - } \\ & \mathrm{V}_{\text {OUT }} \geq 1.5 \mathrm{~V} \end{aligned}$ | 530 | 720 | 865 |  |
| $\mathrm{I}_{\mathrm{SC}}$ | Short-circuit current limit | $\mathrm{V}_{\mathrm{IN}}=\mathrm{V}_{\text {OUT( }}(\mathrm{NOM})+1.0 \mathrm{~V}$ | $\mathrm{V}_{\text {OUT }}=0 \mathrm{~V}$ |  | 350 |  | mA |
|  |  |  | $0.65 \mathrm{~V} \leq \mathrm{V}_{\text {OUT }}<0.8 \mathrm{~V}$ |  | 720 | 880 | mV |
|  |  |  | $0.8 \mathrm{~V} \leq \mathrm{V}_{\text {OUT }}<1.0 \mathrm{~V}$ |  | 585 | 750 |  |
|  |  |  | $1.0 \mathrm{~V} \leq \mathrm{V}_{\text {OUT }}<1.2 \mathrm{~V}$ |  | 420 | 570 |  |
| $\mathrm{V}_{\mathrm{DO}}$ | Dropout voltage | $\mathrm{I}_{\text {OUT }}=500 \mathrm{~mA}$, $-40^{\circ} \mathrm{C} \leq \mathrm{T}_{\mathrm{J}} \leq+125^{\circ} \mathrm{C}$, $\mathrm{V}_{\text {OUT }}=0.95 \times \mathrm{V}_{\text {OUT( }}(\mathrm{NOM})$ | $1.2 \mathrm{~V} \leq \mathrm{V}_{\text {OUT }}<1.5 \mathrm{~V}$ |  | 285 | 400 |  |
|  |  |  | $1.5 \mathrm{~V} \leq \mathrm{V}_{\text {OUT }}<1.8 \mathrm{~V}$ |  | 180 | 235 |  |
|  |  |  | $1.8 \mathrm{~V} \leq \mathrm{V}_{\text {OUT }}<2.5 \mathrm{~V}$ |  | 140 | 185 |  |
|  |  |  | $2.5 \mathrm{~V} \leq \mathrm{V}_{\text {OUT }}<3.3 \mathrm{~V}$ |  | 102 | 140 |  |
|  |  |  | $3.3 \mathrm{~V} \leq \mathrm{V}_{\text {OUT }} \leq 5.5 \mathrm{~V}$ |  | 95 | 130 |  |
| PSRR | Power-supply rejection ratio | $\begin{aligned} & \mathrm{V}_{\mathrm{IN}}=\mathrm{V}_{\text {OUT( }}(\mathrm{NOM})+1.0 \mathrm{~V}, \\ & \mathrm{I}_{\text {OUT }}=50 \mathrm{~mA} \end{aligned}$ | $\mathrm{f}=1 \mathrm{kHz}$ |  | 50 |  | dB |
|  |  |  | $\mathrm{f}=100 \mathrm{kHz}$ |  | 45 |  |
|  |  |  | $\mathrm{f}=1 \mathrm{MHz}$ |  | 30 |  |
| $\mathrm{V}_{\mathrm{n}}$ | Output noise voltage | $\mathrm{BW}=10 \mathrm{~Hz}$ to $100 \mathrm{kHz}, \mathrm{V}_{\text {OUT }}=0.9 \mathrm{~V}$ |  |  | 53 |  | $\mu \mathrm{V}_{\text {RMS }}$ |
| $\mathrm{V}_{\text {UVLO }}$ | Undervoltage lockout | $\mathrm{V}_{\mathrm{IN}}$ rising |  | 1.21 | 1.33 | 1.47 | V |
|  |  | $\mathrm{V}_{\mathrm{IN}}$ falling |  | 1.17 | 1.29 | 1.42 | V |
| $\begin{aligned} & \mathrm{V}_{\text {UVLO, }} \\ & \text { rYYT } \end{aligned}$ | Undervoltage lockout hysteresis | $\mathrm{V}_{\mathrm{IN}}$ Hysteresis |  |  | 40 |  | mV |
| $t_{\text {STR }}$ | Start-up time | From EN low-to-high transition to $\mathrm{V}_{\text {OUT }}=\mathrm{V}_{\text {OUT( }}(\mathrm{NOM}) \times 95 \%$ |  |  | 500 |  | $\mu \mathrm{s}$ |
| $\mathrm{V}_{\text {EN(Hi) }}$ | EN pin high voltage |  |  | 1.0 |  |  | V |
| $\mathrm{V}_{\text {EN(LO) }}$ | EN pin low voltage |  |  |  |  | 0.3 | V |
| $\mathrm{I}_{\text {EN }}$ | Enable pin current | $\mathrm{V}_{\mathrm{IN}}=\mathrm{EN}=6.0 \mathrm{~V}$ |  |  | 10 |  | nA |
| $\begin{aligned} & \mathrm{R}_{\text {PULL }} \\ & \text { DOWN } \end{aligned}$ | Pulldown resistance | $\mathrm{V}_{\mathrm{IN}}=6.0 \mathrm{~V}$ |  |  | 95 |  | $\Omega$ |
| $\mathrm{T}_{\mathrm{SD}}$ | Thermal shutdown | Shutdown, temperature increasing |  |  | 170 |  | ${ }^{\circ} \mathrm{C}$ |
|  |  | Reset, temperature decreasing |  |  | 155 |  |

(1) When the device is connected to external feedback resistors at the FB pin, external resistor tolerances are not included
(2) $\mathrm{V}_{\mathrm{IN}}=1.5 \mathrm{~V}$ for $\mathrm{V}_{\text {OUT }}<1.0 \mathrm{~V}$.# 5.6 Typical Characteristics 

at operating temperature range $T_{J}=25^{\circ} \mathrm{C}, \mathrm{V}_{\mathrm{IN}}=\mathrm{V}_{\text {OUT(NOM) }}+0.5 \mathrm{~V}$ or 1.5 V (whichever is greater), $\mathrm{I}_{\text {OUT }}=1 \mathrm{~mA}, \mathrm{~V}_{\mathrm{EN}}=\mathrm{V}_{\mathrm{IN}}$, and $C_{\text {IN }}=C_{\text {OUT }}=1 \mu \mathrm{~F}$ (unless otherwise noted)
![img-3.jpeg](img-3.jpeg)

Figure 5-1. 3.3-V Line Regulation vs $\mathrm{V}_{\text {IN }}$
![img-4.jpeg](img-4.jpeg)

Figure 5-3. 5.5-V Line Regulation vs $\mathrm{V}_{\text {IN }}$
![img-5.jpeg](img-5.jpeg)

Figure 5-5. 0.55-V Dropout Voltage vs $\mathrm{I}_{\text {OUT }}$
![img-6.jpeg](img-6.jpeg)

Figure 5-2. 0.55-V Line Regulation vs $\mathrm{V}_{\text {IN }}$
![img-7.jpeg](img-7.jpeg)

Figure 5-4. 3.3-V Dropout Voltage vs $\mathrm{I}_{\text {OUT }}$
![img-8.jpeg](img-8.jpeg)

Figure 5-6. 5.5-V Dropout Voltage vs $\mathrm{I}_{\text {OUT }}$# 5.6 Typical Characteristics (continued) 

at operating temperature range $T_{J}=25^{\circ} \mathrm{C}, \mathrm{V}_{\mathrm{IN}}=\mathrm{V}_{\text {OUT(NOM) }}+0.5 \mathrm{~V}$ or 1.5 V (whichever is greater), $\mathrm{I}_{\text {OUT }}=1 \mathrm{~mA}, \mathrm{~V}_{\mathrm{EN}}=\mathrm{V}_{\mathrm{IN}}$, and $C_{\text {IN }}=C_{\text {OUT }}=1 \mu \mathrm{~F}$ (unless otherwise noted)
![img-9.jpeg](img-9.jpeg)

Figure 5-7. $\mathrm{V}_{\mathrm{DO}}$ vs $\mathrm{V}_{\text {OUT }}$
![img-10.jpeg](img-10.jpeg)

Figure 5-9. $\mathrm{I}_{\text {SHDN }}$ vs $\mathrm{V}_{\text {IN }}$
![img-11.jpeg](img-11.jpeg)

Figure 5-11. 3.3-V Load Regulation vs $\mathrm{I}_{\text {OUT }}$
![img-12.jpeg](img-12.jpeg)

Figure 5-8. $\mathrm{I}_{\text {GND }}$ vs $\mathrm{I}_{\text {OUT }}$
![img-13.jpeg](img-13.jpeg)

Figure 5-10. $\mathrm{I}_{\mathrm{Q}}$ vs $\mathrm{V}_{\text {IN }}$
![img-14.jpeg](img-14.jpeg)

Figure 5-12. 0.55-V Load Regulation vs $\mathrm{I}_{\text {OUT }}$# 5.6 Typical Characteristics (continued) 

at operating temperature range $T_{J}=25^{\circ} \mathrm{C}, V_{\text {IN }}=V_{\text {OUT(NOM) }}+0.5 \mathrm{~V}$ or 1.5 V (whichever is greater), $I_{\text {OUT }}=1 \mathrm{~mA}, V_{\text {EN }}=V_{\text {IN }}$, and $C_{\text {IN }}=C_{\text {OUT }}=1 \mu \mathrm{~F}$ (unless otherwise noted)
![img-15.jpeg](img-15.jpeg)

Figure 5-13. 5.5-V Load Regulation vs $\mathrm{I}_{\text {OUT }}$
![img-16.jpeg](img-16.jpeg)

Figure 5-15. $V_{\text {EN(HI) }}$ and $V_{\text {EN(LO) }}$ vs Temperature
![img-17.jpeg](img-17.jpeg)

Figure 5-17. 3.3-V Foldback Current Limit vs $\mathrm{I}_{\text {OUT }}$
![img-18.jpeg](img-18.jpeg)

Figure 5-14. $V_{\text {OUT }}$ vs $I_{\text {OUT }}$ Pulldown Resistor
![img-19.jpeg](img-19.jpeg)

Figure 5-16. $I_{\text {EN }}$ vs $V_{\text {IN }}$
![img-20.jpeg](img-20.jpeg)# 5.6 Typical Characteristics (continued) 

at operating temperature range $T_{J}=25^{\circ} \mathrm{C}, V_{\text {IN }}=V_{\text {OUT(NOM) }}+0.5 \mathrm{~V}$ or 1.5 V (whichever is greater), $I_{\text {OUT }}=1 \mathrm{~mA}, V_{\text {EN }}=V_{\text {IN }}$, and $C_{\text {IN }}=C_{\text {OUT }}=1 \mu \mathrm{~F}$ (unless otherwise noted)
![img-21.jpeg](img-21.jpeg)# 5.6 Typical Characteristics (continued) 

at operating temperature range $T_{J}=25^{\circ} \mathrm{C}, V_{\mathrm{IN}}=V_{\text {OUT(NOM) }}+0.5 \mathrm{~V}$ or 1.5 V (whichever is greater), $\mathrm{I}_{\text {OUT }}=1 \mathrm{~mA}, \mathrm{~V}_{\mathrm{EN}}=\mathrm{V}_{\mathrm{IN}}$, and $C_{\text {IN }}=C_{\text {OUT }}=1 \mu \mathrm{~F}$ (unless otherwise noted)
![img-22.jpeg](img-22.jpeg)# 5.6 Typical Characteristics (continued) 

at operating temperature range $T_{J}=25^{\circ} \mathrm{C}, \mathrm{V}_{\mathrm{IN}}=\mathrm{V}_{\text {OUT(NOM) }}+0.5 \mathrm{~V}$ or 1.5 V (whichever is greater), $\mathrm{I}_{\text {OUT }}=1 \mathrm{~mA}, \mathrm{~V}_{\mathrm{EN}}=\mathrm{V}_{\mathrm{IN}}$, and $C_{\text {IN }}=C_{\text {OUT }}=1 \mu \mathrm{~F}$ (unless otherwise noted)
![img-23.jpeg](img-23.jpeg)

Figure 5-31. PSRR vs Frequency and $\mathrm{C}_{\mathrm{FF}}$
![img-24.jpeg](img-24.jpeg)
$\mathrm{V}_{\text {IN }}=3.8 \mathrm{~V}, \mathrm{~V}_{\text {OUT }}=3.3 \mathrm{~V}, \mathrm{C}_{\text {OUT }}=500 \mathrm{~mA}$
Figure 5-31. PSRR vs Frequency and $\mathrm{C}_{\mathrm{FF}}$
![img-25.jpeg](img-25.jpeg)

Figure 5-35. Output Spectral Noise Density vs Frequency and $\mathrm{C}_{\text {OUT }}$
![img-26.jpeg](img-26.jpeg)

Figure 5-32. PSRR vs Frequency and $\mathrm{I}_{\mathrm{LOAD}}$
![img-27.jpeg](img-27.jpeg)

Figure 5-34. Output Spectral Noise Density vs Frequency and $\mathrm{C}_{\mathrm{FF}}$
![img-28.jpeg](img-28.jpeg)
$\mathrm{I}_{\text {OUT }}=500 \mathrm{~mA}, \mathrm{C}_{\text {OUT }}=2.2 \mu \mathrm{~F}, \mathrm{~V}_{\text {RMS }} \mathrm{BW}=10 \mathrm{~Hz}$ to 100 kHz

Figure 5-36. Output Spectral Noise Density vs Frequency# 6 Detailed Description 

### 6.1 Overview

The TLV758P low-dropout regulators (LDO) consumes low quiescent current and delivers excellent line and load transient performance. These characteristics, combined with low noise and good PSRR with low dropout voltage, make this device designed for portable consumer applications.

This regulator offers foldback current limit, shutdown, and thermal protection. The operating junction temperature for this device is $-40^{\circ} \mathrm{C}$ to $+125^{\circ} \mathrm{C}$.

### 6.2 Functional Block Diagram

![img-29.jpeg](img-29.jpeg)

### 6.3 Feature Description

### 6.3.1 Undervoltage Lockout (UVLO)

The TLV758P uses an undervoltage lockout (UVLO) circuit that disables the output until the input voltage is greater than the rising UVLO voltage ( $\mathrm{V}_{\text {UVLO }}$ ). This circuit ensures that the device does not exhibit any unpredictable behavior when the supply voltage is lower than the operational range of the internal circuitry. When $V_{I N}$ is less than $V_{U V L O}$, the output is connected to ground with a pulldown resistor ( $R_{\text {PULLDOWN }}$ ).

### 6.3.2 Shutdown

The enable pin (EN) is active high. Enable the device by forcing the EN pin to exceed $\mathrm{V}_{\mathrm{EN}(\mathrm{H})}$. Turn off the device by forcing the EN pin to drop below $\mathrm{V}_{\mathrm{EN}(\mathrm{LO})}$. If shutdown capability is not required, connect EN to IN.

The TLV758P has an internal pulldown MOSFET that connects an $R_{\text {PULLDOWN }}$ resistor to ground when the device is disabled. The discharge time after disabling depends on the output capacitance ( $\mathrm{C}_{\text {OUT }}$ ) and the load resistance $\left(R_{L}\right)$ in parallel with the pulldown resistor ( $R_{\text {PULLDOWN }}$ ). Equation 1 calculates the time constant:

$$
\tau=\left(R_{\text {PULLDOWN }} \times R_{L}\right) /\left(R_{\text {PULLDOWN }}+R_{L}\right)
$$

### 6.3.3 Foldback Current Limit

The device has an internal current limit circuit that protects the regulator during transient high-load current faults or shorting events. The current limit is a hybrid brick-wall-foldback scheme. The current limit transitions from a brick-wall scheme to a foldback scheme at the foldback voltage ( $\mathrm{V}_{\text {FOLDBACK }}$ ). In a high-load current fault with the output voltage above $\mathrm{V}_{\text {FOLDBACK }}$, the brick-wall scheme limits the output current to the current limit ( $\mathrm{I}_{\mathrm{CL}}$ ). When the voltage drops below $\mathrm{V}_{\text {FOLDBACK }}$, a foldback current limit activates that scales back the current as theoutput voltage approaches GND. When the output is shorted, the device supplies a typical current called the short-circuit current limit ( $\mathrm{I}_{\mathrm{SC}}$ ). $\mathrm{I}_{\mathrm{CL}}$ and $\mathrm{I}_{\mathrm{SC}}$ are listed in the Electrical Characteristics table.
For this device, $\mathrm{V}_{\text {FOLDBACK }}=0.4 \mathrm{~V} \times \mathrm{V}_{\text {OUT(NOM) }}$.
The output voltage is not regulated when the device is in current limit. When a current limit event occurs, the device begins to heat up because of the increase in power dissipation. When the device is in brick-wall current limit, the pass transistor dissipates power $\left[\left(\mathrm{V}_{\mathrm{IN}}-\mathrm{V}_{\text {OUT }}\right) \times \mathrm{I}_{\mathrm{CL}}\right]$. When the device output is shorted and the output is below $\mathrm{V}_{\text {FOLDBACK }}$, the pass transistor dissipates power $\left[\left(\mathrm{V}_{\mathrm{IN}}-\mathrm{V}_{\text {OUT }}\right) \times \mathrm{I}_{\mathrm{SC}}\right]$. If thermal shutdown is triggered, the device turns off. After the device cools down, the internal thermal shutdown circuit turns the device back on. If the output current fault condition continues, the device cycles between current limit and thermal shutdown. For more information on current limits, see the Know Your Limits application note.

Figure 6-1 shows a diagram of the foldback current limit.
![img-30.jpeg](img-30.jpeg)

Figure 6-1. Foldback Current Limit

# 6.3.4 Thermal Shutdown 

Thermal shutdown protection disables the output when the junction temperature rises to approximately $170^{\circ} \mathrm{C}$. Disabling the device eliminates the power dissipated by the device, allowing the device to cool. When the junction temperature cools to approximately $155^{\circ} \mathrm{C}$, the output circuitry is again enabled. Depending on power dissipation, thermal resistance, and ambient temperature, the thermal protection circuit can cycle on and off. This cycling limits regulator dissipation, protecting the LDO from damage as a result of overheating.
Activating the thermal shutdown feature usually indicates excessive power dissipation as a result of the product of the $\left(V_{I N}-V_{\text {OUT }}\right)$ voltage and the load current. For reliable operation, limit junction temperature to $125^{\circ} \mathrm{C}$ maximum. To estimate the margin of safety in a complete design, increase the ambient temperature until the thermal protection is triggered; use worst-case loads and signal conditions.
The TLV758P internal protection circuitry protects against overload conditions but is not intended to be activated in normal operation. Continuously running the TLV758P into thermal shutdown degrades device reliability.# 6.4 Device Functional Modes 

### 6.4.1 Device Functional Mode Comparison

Table 6-1 shows the conditions that lead to the different modes of operation. See the Electrical Characteristics table for parameter values.

Table 6-1. Device Functional Mode Comparison

| OPERATING MODE | PARAMETER |  |  |  |
| :--: | :--: | :--: | :--: | :--: |
|  | $V_{I N}$ | $V_{E N}$ | $\mathrm{I}_{\text {OUT }}$ | $T_{J}$ |
| Normal operation | $V_{\text {IN }}>V_{\text {OUT }(\text { nom })}+V_{\text {DO }}$ and $V_{\text {IN }}>V_{I N(\text { min })}$ | $V_{\text {EN }}>V_{\text {EN(Hi) }}$ | $\mathrm{I}_{\text {OUT }}<\mathrm{I}_{\text {OUT(max) }}$ | $T_{J}<T_{S D \text { (shutdown) }}$ |
| Dropout operation | $V_{I N(\text { min })}<V_{I N}<V_{\text {OUT }(\text { nom })}+V_{D O}$ | $V_{E N}>V_{E N(I H)}$ | $\mathrm{I}_{\text {OUT }}<\mathrm{I}_{\text {OUT(max) }}$ | $T_{J}<T_{S D \text { (shutdown) }}$ |
| Disabled <br> (any true condition <br> disables the device) | $V_{I N}<V_{U V L O}$ | $V_{E N}<V_{E N(I, O W)}$ | Not applicable | $T_{J}>T_{S D \text { (shutdown) }}$ |

### 6.4.2 Normal Operation

The device regulates to the nominal output voltage when the following conditions are met:

- The input voltage is greater than the nominal output voltage plus the dropout voltage $\left(V_{\text {OUT }(\text { nom })}+V_{\text {DO }}\right)$
- The output current is less than the current limit $\left(I_{\text {OUT }}<I_{\text {CL }}\right)$
- The device junction temperature is less than the thermal shutdown temperature ( $\mathrm{T}_{\mathrm{J}}<\mathrm{T}_{\mathrm{SD}}$ )
- The enable voltage has previously exceeded the enable rising threshold voltage and has not yet decreased to less than the enable falling threshold


### 6.4.3 Dropout Operation

If the input voltage is lower than the nominal output voltage plus the specified dropout voltage, but all other conditions are met for normal operation, the device operates in dropout mode. In this mode, the output voltage tracks the input voltage. During this mode, the transient performance of the device becomes significantly degraded because the pass transistor is in the ohmic or triode region, and acts as a switch. Line or load transients in dropout can result in large output-voltage deviations.
When the device is in a steady dropout state (defined as when the device is in dropout, $V_{I N}<V_{\text {OUT }(\text { NOM })}+V_{\text {DO }}$, directly after being in a normal regulation state, but not during start-up), the pass transistor is driven into the ohmic or triode region. When the input voltage returns to a value greater than or equal to the nominal output voltage plus the dropout voltage $\left(V_{\text {OUT }(\text { NOM })}+V_{\text {DO }}\right)$, the output voltage can overshoot for a short period of time while the device pulls the pass transistor back into the linear region.

### 6.4.4 Disabled

The output of the device can be shutdown by forcing the voltage of the enable pin to less than the maximum EN pin low-level input voltage (see the Electrical Characteristics table). When disabled, the pass transistor is turned off, internal circuits are shutdown, and the output voltage is actively discharged to ground by an internal discharge circuit from the output to ground.# 7 Application and Implementation 

## Note

Information in the following applications sections is not part of the TI component specification, and TI does not warrant its accuracy or completeness. TI's customers are responsible for determining suitability of components for their purposes, as well as validating and testing their design implementation to confirm system functionality.

### 7.1 Application Information

### 7.1.1 Adjustable Device Feedback Resistors

Figure 7-1 shows that the output voltage of the TLV758P can be adjusted from 0.55 V to 5.5 V by using a resistor divider network.
![img-31.jpeg](img-31.jpeg)

Figure 7-1. Adjustable Operation
The adjustable-version device requires external feedback divider resistors to set the output voltage. $\mathrm{V}_{\text {OUT }}$ is set using the feedback divider resistors, $R_{1}$ and $R_{2}$, according to the following equation:

$$
\mathrm{V}_{\text {OUT }}=\mathrm{V}_{\mathrm{FB}} \times\left(1+\mathrm{R}_{1} / \mathrm{R}_{2}\right)
$$

For this device, $\mathrm{V}_{\mathrm{FB}}=0.55 \mathrm{~V}$.
To ignore the FB pin current error term in the $\mathrm{V}_{\text {OUT }}$ equation, set the feedback divider current to 100 times the FB pin current listed in the Electrical Characteristics table. This setting provides the maximum feedback divider series resistance, as shown in the following equation:

$$
R_{1}+R_{2} \leq V_{\text {OUT }} /\left(I_{\mathrm{FB}} \times 100\right)
$$

For this device, $\mathrm{I}_{\mathrm{FB}}=10 \mathrm{nA}$.

### 7.1.2 Input and Output Capacitor Selection

The TLV758P requires an output capacitance of $0.47 \mu \mathrm{~F}$ or larger for stability. Use X5R- and X7R-type ceramic capacitors because these capacitors have minimal variation in value and equivalent series resistance (ESR) over temperature. When choosing a capacitor for a specific application, pay attention to the dc bias characteristics for the capacitor. Higher output voltages cause a significant derating of the capacitor. For best performance, the maximum recommended output capacitance is $220 \mu \mathrm{~F}$.
Although an input capacitor is not required for stability, good analog design practice is to connect a capacitor from IN to GND. Some input supplies have a high impedance, thus placing the input capacitor on the input supply helps reduce the input impedance. This capacitor counteracts reactive input sources and improves transient response, input ripple, and PSRR. If the input supply has a high impedance over a large range of frequencies, several input capacitors can be used in parallel to lower the impedance over frequency. Use a higher-value capacitor if large, fast, rise-time load transients are anticipated, or if the device is located several inches from the input power source.# 7.1.3 Dropout Voltage 

The TLV758P uses a PMOS pass transistor to achieve low dropout. When ( $\mathrm{V}_{\text {IN }}-\mathrm{V}_{\text {OUT }}$ ) is less than the dropout voltage $\left(\mathrm{V}_{\mathrm{DO}}\right)$, the PMOS pass transistor is in the linear region of operation and the input-to-output resistance is the $\mathrm{R}_{\mathrm{DS}(\mathrm{ON})}$ of the PMOS pass transistor. $\mathrm{V}_{\mathrm{DO}}$ scales approximately with output current because the PMOS pass transistor behaves like a resistor in dropout mode. As with any linear regulator, PSRR and transient response degrade as $\left(\mathrm{V}_{\mathrm{IN}}-\mathrm{V}_{\text {OUT }}\right)$ approaches dropout operation.

### 7.1.4 Exiting Dropout

Some applications have transients that place the LDO into dropout, such as slower ramps on $\mathrm{V}_{\text {IN }}$ during start-up. As with other LDOs, the output may overshoot on recovery from these conditions. A ramping input supply causes an LDO to overshoot on start-up, as shown in Figure 7-2, when the slew rate and voltage levels are in the correct range. Use an enable signal to avoid this condition.
![img-32.jpeg](img-32.jpeg)

Figure 7-2. Start-Up Into Dropout
Line transients out of dropout can also cause overshoot on the output of the regulator. These overshoots are caused by the error amplifier having to drive the gate capacitance of the pass transistor and bring the gate back to the correct voltage for proper regulation. Figure 7-3 illustrates what is happening internally with the gate voltage and how overshoot can be caused during operation. When the LDO is placed in dropout, the gate voltage (VGS) is pulled all the way down to ground to give the pass transistor the lowest on-resistance as possible. However, if a line transient occurs when the device is in dropout, the loop is not in regulation and can cause the output to overshoot until the loop responds and the output current pulls the output voltage back down into regulation. If these transients are not acceptable, then continue to add input capacitance in the system until the transient is slow enough to reduce the overshoot.![img-33.jpeg](img-33.jpeg)

Figure 7-3. Line Transients From Dropout# 7.1.5 Reverse Current 

As with most LDOs, excessive reverse current can damage this device.
Reverse current flows through the body diode on the pass transistor instead of the normal conducting channel. At high magnitudes, this current flow degrades the long-term reliability of the device, as a result of one of the following conditions:

- Degradation caused by electromigration
- Excessive heat dissipation
- Potential for a latch-up condition

Conditions where reverse current can occur are outlined in this section, all of which can exceed the absolute maximum rating of $\mathrm{V}_{\text {OUT }}>\mathrm{V}_{\mathrm{IN}}+0.3 \mathrm{~V}$ :

- If the device has a large $\mathrm{C}_{\text {OUT }}$ and the input supply collapses with little or no load current
- The output is biased when the input supply is not established
- The output is biased above the input supply

If reverse current flow is expected in the application, external protection must be used to protect the device. Figure 7-4 shows one approach of protecting the device.
![img-34.jpeg](img-34.jpeg)

Figure 7-4. Example Circuit for Reverse Current Protection Using a Schottky Diode

### 7.1.6 Power Dissipation $\left(\mathrm{P}_{\mathrm{D}}\right)$

Circuit reliability requires consideration of the device power dissipation, location of the circuit on the printed circuit board ( PCB ), and correct sizing of the thermal plane. The PCB area around the regulator must have few or no other heat-generating devices that cause added thermal stress.

To first-order approximation, power dissipation in the regulator depends on the input-to-output voltage difference and load conditions. Equation 4 calculates power dissipation $\left(\mathrm{P}_{\mathrm{D}}\right)$.

$$
P_{D}=\left(V_{\text {IN }}-V_{\text {OUT }}\right) \times I_{\text {OUT }}
$$

## Note

Power dissipation can be minimized, and therefore greater efficiency can be achieved, by correct selection of the system voltage rails. For the lowest power dissipation use the minimum input voltage required for correct output regulation.

For devices with a thermal pad, the primary heat conduction path for the device package is through the thermal pad to the PCB. Solder the thermal pad to a copper pad area under the device. This pad area must contain an array of plated vias that conduct heat to additional copper planes for increased heat dissipation.

The maximum power dissipation determines the maximum allowable ambient temperature $\left(T_{A}\right)$ for the device. According to Equation 5, power dissipation and junction temperature are most often related by the junction-toambient thermal resistance ( $\mathrm{R}_{\mathrm{BJA}}$ ) of the combined PCB and device package and the temperature of the ambient air $\left(T_{A}\right)$.$$
\mathrm{T}_{\mathrm{J}}=\mathrm{T}_{\mathrm{A}}+\left(\mathrm{R}_{\mathrm{BJA}} \times \mathrm{P}_{\mathrm{D}}\right)
$$

Thermal resistance $\left(R_{B J A}\right)$ is highly dependent on the heat-spreading capability built into the particular PCB design, and therefore varies according to the total copper area, copper weight, and location of the planes. The junction-to-ambient thermal resistance listed in the Thermal Information table is determined by the JEDEC standard PCB and copper-spreading area, and is used as a relative measure of package thermal performance.

# 7.1.7 Feed-Forward Capacitor ( $\mathrm{C}_{\mathrm{FF}}$ ) 

For the adjustable-voltage version device, a feed-forward capacitor ( $\mathrm{C}_{\mathrm{FF}}$ ) can be connected from the OUT pin to the FB pin. $\mathrm{C}_{\mathrm{FF}}$ improves transient, noise, and PSRR performance, but is not required for regulator stability. Recommended $\mathrm{C}_{\mathrm{FF}}$ values are listed in the Recommended Operating Conditions table. A higher capacitance $\mathrm{C}_{\mathrm{FF}}$ can be used; however, the start-up time increases. For a detailed description of $\mathrm{C}_{\mathrm{FF}}$ tradeoffs, see the Pros and Cons of Using a Feedforward Capacitor with a Low-Dropout Regulator application note.

### 7.1.8 Start-Up Sequencing

If $\mathrm{V}_{\mathrm{EN}}$ is greater than $\mathrm{V}_{\mathrm{UVLO}}$ rising (min), then the input pin (IN) must sink 1 mA of current to avoid the device being turn on with a floating input pin.# 7.2 Typical Application 

Figure 7-5 shows the typical application circuit for the TLV758P. Input and output capacitances must be at least $1 \mu \mathrm{~F}$.
![img-35.jpeg](img-35.jpeg)

Figure 7-5. TLV758P Typical Application

### 7.2.1 Design Requirements

Use the parameters listed in Table 7-1 for typical linear regulator applications.
Table 7-1. Design Parameters

| PARAMETER | DESIGN REQUIREMENT |
| :--: | :--: |
| Input voltage | 3.8 V |
| Output voltage | $3.3 \mathrm{~V}, \pm 1 \%$ |
| Input current | 500 mA (maximum) |
| Output load | $500-\mathrm{mA}$ DC |
| Maximum ambient temperature | $70^{\circ} \mathrm{C}$ |

### 7.2.2 Detailed Design Procedure

Input and output capacitors are required to achieve the output voltage transient requirements. Capacitance values of $2.2 \mu \mathrm{~F}$ are selected to give the maximum output capacitance in a small, low-cost package; see the Input and Output Capacitor Selection section for details.

Figure 7-1 illustrates the output voltage of the TLV758P; set the output voltage using the resistor divider.

### 7.2.2.1 Input Current

During normal operation, the input current to the LDO is approximately equal to the output current of the LDO. During start-up, the input current is higher as a result of the inrush current charging the output capacitor. Use Equation 6 to calculate the current through the input.

$$
\mathrm{I}_{\text {OUT(t) }}=\left\{\frac{\mathrm{C}_{\text {OUT }} \times \mathrm{dV}_{\text {OUT }}(\mathrm{t})}{\mathrm{dt}}\right\}+\left[\frac{\mathrm{V}_{\text {OUT }}(\mathrm{t})}{\mathrm{R}_{\text {LOAD }}}\right]
$$

where:

- $\mathrm{V}_{\text {OUT }}(\mathrm{t})$ is the instantaneous output voltage of the turn-on ramp
- $\mathrm{dV}_{\text {OUT }}(\mathrm{t}) / \mathrm{dt}$ is the slope of the $\mathrm{V}_{\text {OUT }}$ ramp
- $R_{\text {LOAD }}$ is the resistive load impedance# 7.2.2.2 Thermal Dissipation 

The junction temperature can be determined using the junction-to-ambient thermal resistance ( $\mathrm{R}_{\mathrm{BJA}}$ ) and the total power dissipation $\left(\mathrm{P}_{\mathrm{D}}\right)$. Use Equation 7 to calculate the power dissipation. Multiply $\mathrm{P}_{\mathrm{D}}$ by $\mathrm{R}_{\mathrm{BJA}}$ as Equation 8 shows and add the ambient temperature $\left(T_{A}\right)$ to calculate the junction temperature $\left(T_{J}\right)$.

$$
\begin{aligned}
& P_{D}=\left(I_{G N D}+I_{O U T}\right) \times\left(V_{I N}-V_{O U T}\right) \\
& T_{J}=R_{B J A} \times P_{D}+T_{A}
\end{aligned}
$$

Calculate the maximum ambient temperature as Equation 9 shows if the $\left(T_{J(\text { MAX })}\right)$ value does not exceed $125^{\circ} \mathrm{C}$. Equation 10 calculates the maximum ambient temperature with a value of $104.93^{\circ} \mathrm{C}$.

$$
\begin{aligned}
& T_{A(\text { MAX })}=T_{J(\text { MAX })}-R_{B J A} \times P_{D} \\
& T_{A(\text { MAX })}=125^{\circ} \mathrm{C}-80.3^{\circ} \mathrm{C} / \mathrm{W} \times(3.8 \mathrm{~V}-3.3 \mathrm{~V}) \times(0.5 \mathrm{~A})=104.93^{\circ} \mathrm{C}
\end{aligned}
$$

### 7.2.3 Application Curve

![img-36.jpeg](img-36.jpeg)

Figure 7-6. PSRR vs Frequency and $\mathrm{I}_{\text {LOAD }}$

### 7.3 Power Supply Recommendations

Connect a low output impedance power supply directly to the IN pin of the TLV758P.

### 7.4 Layout

### 7.4.1 Layout Guidelines

- Place input and output capacitors as close to the device as possible.
- Use copper planes for device connections, in order to optimize thermal performance.
- Place thermal vias around the device to distribute the heat.
- Do not place a thermal via directly beneath the thermal pad of the DRV package. A via can wick solder or solder paste away from the thermal pad joint during the soldering process, leading to a compromised solder joint on the thermal pad.# 7.4.2 Layout Examples 

![img-37.jpeg](img-37.jpeg)

Figure 7-7. DBV Package Layout Example
![img-38.jpeg](img-38.jpeg)

Figure 7-8. DRV Package Layout Example# 8 Device and Documentation Support 

### 8.1 Documentation Support

### 8.1.1 Device Nomenclature

Table 8-1. Device Nomenclature ${ }^{(1)}$

| PRODUCT | $\mathbf{V}_{\text {OUT }}$ |
| :--: | :-- |
| TLV758 xx(x)Pyyyz | $\mathbf{x x}(\mathbf{x})$ is the nominal output voltage. For output voltages with a resolution of 100 mV , two digits are used <br> in the ordering number; otherwise, three digits are used (for example, $28=2.8 \mathrm{~V} ; 125=1.25 \mathrm{~V}$ ). 01 is for <br> adjustable version. <br> $\mathbf{P}$ indicates an active output discharge feature. All members of the TLV758P family actively discharge <br> the output when the device is disabled. <br> yyy is the package designator. <br> $\mathbf{z}$ is the package quantity. R is for reel (3000 pieces), T is for tape (250 pieces). |

(1) For the most current package and ordering information see the Package Option Addendum at the end of this document, or visit the device product folder on www.ti.com.

### 8.1.2 Related Documentation

For related documentation see the following:
Texas Instruments, Pros and cons of using a feedforward capacitor with a low-dropout regulator application note

### 8.2 Receiving Notification of Documentation Updates

To receive notification of documentation updates, navigate to the device product folder on ti.com. Click on Subscribe to updates to register and receive a weekly digest of any product information that has changed. For change details, review the revision history included in any revised document.

### 8.3 Support Resources

TI E2E ${ }^{\text {TM }}$ support forums are an engineer's go-to source for fast, verified answers and design help - straight from the experts. Search existing answers or ask your own question to get the quick design help you need.
Linked content is provided "AS IS" by the respective contributors. They do not constitute TI specifications and do not necessarily reflect TI's views; see TI's Terms of Use.

### 8.4 Trademarks

TI E2E ${ }^{\text {TM }}$ is a trademark of Texas Instruments.
All trademarks are the property of their respective owners.

### 8.5 Electrostatic Discharge Caution

This integrated circuit can be damaged by ESD. Texas Instruments recommends that all integrated circuits be handled with appropriate precautions. Failure to observe proper handling and installation procedures can cause damage.
ESD damage can range from subtle performance degradation to complete device failure. Precision integrated circuits may be more susceptible to damage because very small parametric changes could cause the device not to meet its published specifications.

### 8.6 Glossary

TI Glossary This glossary lists and explains terms, acronyms, and definitions.# 9 Revision History 

NOTE: Page numbers for previous revisions may differ from page numbers in the current version.
Changes from Revision C (March 2019) to Revision D (October 2023) Page

- Changed DBV package from Advance Information to Production Data ..... 1
- Added links to Applications section. ..... 1
- Changed 5-V to 5.5-V in title of 5.5-V Load Regulation vs Iout figure ..... 7
- Added Startup Sequencing section. ..... 20
- Added Device Nomenclature section ..... 24
Changes from Revision B (March 2019) to Revision C (March 2019) Page
- Deleted thermal pad from DBV pin out drawing ..... 3


## 10 Mechanical, Packaging, and Orderable Information

The following pages include mechanical, packaging, and orderable information. This information is the most current data available for the designated devices. This data is subject to change without notice and revision of this document. For browser-based versions of this data sheet, refer to the left-hand navigation.# PACKAGE OPTION ADDENDUM

|  PACKAGE OPTION ADDENDUM |  |  |  |  |  |  |  |  |   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
|  |   |   |   |   |   |   |   |   |   |
Important Information and Disclaimer:The information provided on this page represents TI's knowledge and belief as of the date that it is provided. TI bases its knowledge and belief on information provided by third parties, and makes no representation or warranty as to the accuracy of such information. Efforts are underway to better integrate information from third parties. TI has taken and continues to take reasonable steps to provide representative and accurate information but may not have conducted destructive testing or chemical analysis on incoming materials and chemicals. TI and TI suppliers consider certain information to be proprietary, and thus CAS numbers and other limited information may not be available for release.

In no event shall TI's liability arising out of such information exceed the total purchase price of the TI part(s) at issue in this document sold by TI to Customer on an annual basis.# TAPE AND REEL INFORMATION 

![img-39.jpeg](img-39.jpeg)

TAPE DIMENSIONS
![img-40.jpeg](img-40.jpeg)

| A0 | Dimension designed to accommodate the component width |
| :-- | :-- |
| B0 | Dimension designed to accommodate the component length |
| K0 | Dimension designed to accommodate the component thickness |
| W | Overall width of the carrier tape |
| P1 | Pitch between successive cavity centers |

QUADRANT ASSIGNMENTS FOR PIN 1 ORIENTATION IN TAPE
![img-41.jpeg](img-41.jpeg)

Pocket Quadrants
*All dimensions are nominal

| Device | Package <br> Type | Package <br> Drawing | Pins | SPQ | Reel <br> Diameter <br> (mm) | Reel <br> Width <br> W1 (mm) | A0 <br> (mm) | B0 <br> (mm) | K0 <br> (mm) | P1 <br> (mm) | W <br> (mm) | Pin1 <br> Quadrant |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TLV75801PDBVJ | SOT-23 | DBV | 5 | 8000 | 330.0 | 8.4 | 3.2 | 3.2 | 1.4 | 4.0 | 8.0 | Q3 |
| TLV75801PDBVR | SOT-23 | DBV | 5 | 3000 | 178.0 | 8.4 | 3.2 | 3.2 | 1.4 | 4.0 | 8.0 | Q3 |
| TLV75801PDBVT | SOT-23 | DBV | 5 | 250 | 180.0 | 8.4 | 3.2 | 3.2 | 1.4 | 4.0 | 8.0 | Q3 |
| TLV75801PDRVR | WSON | DRV | 6 | 3000 | 180.0 | 8.4 | 2.3 | 2.3 | 1.15 | 4.0 | 8.0 | Q2 |
| TLV75801PDRVRG4 | WSON | DRV | 6 | 3000 | 180.0 | 8.4 | 2.3 | 2.3 | 1.15 | 4.0 | 8.0 | Q2 |
| TLV75801PDRVT | WSON | DRV | 6 | 250 | 180.0 | 8.4 | 2.3 | 2.3 | 1.15 | 4.0 | 8.0 | Q2 |# PACKAGE MATERIALS INFORMATION

## TAPE AND REEL BOX DIMENSIONS

![img-42.jpeg](img-42.jpeg)

*All dimensions are nominal

|  Device | Package Type | Package Drawing | Pins | SPQ | Length (mm) | Width (mm) | Height (mm)  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  TLV75801PDBVJ | SOT-23 | DBV | 5 | 8000 | 360.0 | 360.0 | 36.0  |
|  TLV75801PDBVR | SOT-23 | DBV | 5 | 3000 | 208.0 | 191.0 | 35.0  |
|  TLV75801PDBVT | SOT-23 | DBV | 5 | 250 | 210.0 | 185.0 | 35.0  |
|  TLV75801PDRVR | WSON | DRV | 6 | 3000 | 210.0 | 185.0 | 35.0  |
|  TLV75801PDRVRG4 | WSON | DRV | 6 | 3000 | 210.0 | 185.0 | 35.0  |
|  TLV75801PDRVT | WSON | DRV | 6 | 250 | 210.0 | 185.0 | 35.0  |![img-43.jpeg](img-43.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.
3. Reference JEDEC MO-178.
4. Body dimensions do not include mold flash, protrusions, or gate burrs. Mold flash, protrusions, or gate burrs shall not exceed 0.25 mm per side.
5. Support pin may differ or may not be present.![img-44.jpeg](img-44.jpeg)

NOTES: (continued)
6. Publication IPC-7351 may have alternate designs.
7. Solder mask tolerances between and around signal pads can vary based on board fabrication site.![img-45.jpeg](img-45.jpeg)

NOTES: (continued)
8. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.
9. Board assembly site may have different recommendations for stencil design.![img-46.jpeg](img-46.jpeg)

Images above are just a representation of the package family, actual package may vary. Refer to the product data sheet for package details.![img-47.jpeg](img-47.jpeg)
![img-48.jpeg](img-48.jpeg)
![img-49.jpeg](img-49.jpeg)

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M.
2. This drawing is subject to change without notice.
3. The package thermal pad must be soldered to the printed circuit board for thermal and mechanical performance.![img-50.jpeg](img-50.jpeg)

NOTES: (continued)
4. This package is designed to be soldered to a thermal pad on the board. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/iti/slua271).
5. Vias are optional depending on application, refer to device data sheet. If some or all are implemented, recommended via locations are shown.![img-51.jpeg](img-51.jpeg)

NOTES: (continued)
6. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.# IMPORTANT NOTICE AND DISCLAIMER 

TI PROVIDES TECHNICAL AND RELIABILITY DATA (INCLUDING DATA SHEETS), DESIGN RESOURCES (INCLUDING REFERENCE DESIGNS), APPLICATION OR OTHER DESIGN ADVICE, WEB TOOLS, SAFETY INFORMATION, AND OTHER RESOURCES "AS IS" AND WITH ALL FAULTS, AND DISCLAIMS ALL WARRANTIES, EXPRESS AND IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE OR NON-INFRINGEMENT OF THIRD PARTY INTELLECTUAL PROPERTY RIGHTS.
These resources are intended for skilled developers designing with TI products. You are solely responsible for (1) selecting the appropriate TI products for your application, (2) designing, validating and testing your application, and (3) ensuring your application meets applicable standards, and any other safety, security, regulatory or other requirements.
These resources are subject to change without notice. TI grants you permission to use these resources only for development of an application that uses the TI products described in the resource. Other reproduction and display of these resources is prohibited. No license is granted to any other TI intellectual property right or to any third party intellectual property right. TI disclaims responsibility for, and you will fully indemnify TI and its representatives against, any claims, damages, costs, losses, and liabilities arising out of your use of these resources.
TI's products are provided subject to TI's Terms of Sale or other applicable terms available either on ti.com or provided in conjunction with such TI products. TI's provision of these resources does not expand or otherwise alter TI's applicable warranties or warranty disclaimers for TI products.
TI objects to and rejects any additional or different terms you may have proposed.
Mailing Address: Texas Instruments, Post Office Box 655303, Dallas, Texas 75265
Copyright © 2025, Texas Instruments Incorporated