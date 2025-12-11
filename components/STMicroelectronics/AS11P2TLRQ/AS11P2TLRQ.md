# Low voltage $1 \Omega$ single-pole double-throw analog switch with break- 

before-make feature

Datasheet - production data
![img-0.jpeg](img-0.jpeg)

## Features

- High speed:
- $t_{P D}=130 \mathrm{ps}$ (typ.) at $\mathrm{V}_{\mathrm{CC}}=3.0 \mathrm{~V}$
- $t_{P D}=140 \mathrm{ps}$ (typ.) at $\mathrm{V}_{\mathrm{CC}}=2.3 \mathrm{~V}$
- Ultra low power dissipation:
- $\quad \mathrm{I}_{\mathrm{CC}}=0.2 \mu \mathrm{~A}$ (max.) at $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}$
- Low ON resistance:
- $\quad R_{O N}=1.0 \Omega$ (typ.) at $\mathrm{V}_{\mathrm{CC}}=4.5 \mathrm{~V}$
- $\quad R_{O N}=1.2 \Omega$ (typ.) at $\mathrm{V}_{\mathrm{CC}}=3.0 \mathrm{~V}$
- $\quad R_{O N}=2.0 \Omega$ (typ.) at $\mathrm{V}_{\mathrm{CC}}=1.8 \mathrm{~V}$
- Wide operating voltage range:
- $\mathrm{V}_{\mathrm{CC}}$ (opr.) $=1.65$ to 4.5 V single supply
- 5 V tolerant and 1.8 V compatible threshold ON digital control input at $\mathrm{V}_{\mathrm{CC}}=1.65$ to 4.5 V
- Latch-up performance exceeds 200 mA per JESD 78, Class II
- ESD performance tested per JESD 22
- 2000 V human-body model (A114-B, Class II)
- 200 V machine model (A115-A)
- 1000 V charged-device model (C101)


## Description

The AS11P2TLR is a high speed CMOS low voltage single analog SPDT (single-pole doublethrow) switch or 2:1 multiplexer/demultiplexer switch manufactured using silicon gate $\mathrm{C}^{3}$ MOS technology. Designed to operate from a 1.65 to 4.5 V supply, this device is ideal for portable applications.

The device offers very low ON-resistance ( $1 \Omega$ ) at $\mathrm{V}_{\mathrm{CC}}=4.5 \mathrm{~V}$. Switch S 1 is ON (connected to common ports Dn ) when the SEL input is held high, and OFF (state of high impedance exists between the two ports) when SEL is held low.

Switch S2 is ON (connected to common port D) when the SEL input is held low, and OFF (state of high impedance exists between the two ports) when SEL is held high.

Additional key features are fast switching speed, break-before-make delay time and ultralow power consumption. All inputs and outputs are equipped with protection circuits to protect against static discharge, giving them immunity from ESD and transient excess voltage.

Table 1. Device summary

| Order code | Package | Packaging |
| :--: | :--: | :--: |
| AS11P2TLR | DFN6L <br> $(1.2 \times 1 \mathrm{~mm})$ | Tape and reel |# Contents 

1 Pin connections and functions ..... 3
2 Electrical ratings ..... 4
3 Electrical characteristics ..... 5
3.1 DC electrical characteristics ..... 5
3.2 AC electrical characteristics ..... 6
3.3 Analog switch characteristics ..... 7
4 Test circuits ..... 8
5 Package information ..... 12
6 Revision history ..... 17# 1 Pin connections and functions 

Figure 1. Pin connections (top through view)
![img-1.jpeg](img-1.jpeg)

Table 2. Pin descriptions

| Pin number | Symbol | Name and function |
| :--: | :--: | :-- |
| 4 | S1 | Independent channel |
| 6 | S2 | Independent channel |
| 1 | D | Common channels |
| 3 | SEL | Control |
| 2 | $\mathrm{V}_{\mathrm{CC}}$ | Positive supply voltage |
| 5 | GND | Ground $(0 \mathrm{~V})$ |

Figure 2. Input equivalent circuit
![img-2.jpeg](img-2.jpeg)

Table 3. Truth table

| Sel | Switch S1 | Switch S2 |
| :--: | :--: | :--: |
| H | ON | OFF $^{(1)}$ |
| L | OFF $^{(1)}$ | ON |

1. High impedance.# 2 Electrical ratings 

Stressing the device above the rating listed in Table 4: Absolute maximum ratings may cause permanent damage to the device. These are stress ratings only and operation of the device at these or any other conditions above those indicated in Table 5: Recommended operating conditions of this specification is not implied. Exposure to absolute maximum rating conditions for extended periods may affect device reliability. Refer also to the STMicroelectronics ${ }^{\text {TM }}$ SURE program and other relevant quality documents.

Table 4. Absolute maximum ratings

| Symbol | Parameter | Value | Unit |
| :--: | :-- | :--: | :--: |
| $\mathrm{V}_{\mathrm{CC}}$ | Supply voltage | -0.5 to 5.5 | V |
| $\mathrm{V}_{\mathrm{I}}$ | DC input voltage | -0.5 to $\mathrm{V}_{\mathrm{CC}}+0.5$ | V |
| $\mathrm{V}_{\mathrm{IC}}$ | DC control input voltage | -0.5 to 5.5 | V |
| $\mathrm{V}_{\mathrm{O}}$ | DC output voltage | -0.5 to $\mathrm{V}_{\mathrm{CC}}+0.5$ | V |
| $\mathrm{I}_{\text {IKC }}$ | DC input diode current on control pin $\left(\mathrm{V}_{\text {SEL }}<0 \mathrm{~V}\right)$ | -50 | mA |
| $\mathrm{I}_{\text {IK }}$ | DC input diode current $\left(\mathrm{V}_{\mathrm{IN}}<0 \mathrm{~V}\right)$ | $\pm 50$ | mA |
| $\mathrm{I}_{\mathrm{OK}}$ | DC output diode current | $\pm 20$ | mA |
| $\mathrm{I}_{\mathrm{O}}$ | DC output current | $\pm 200$ | mA |
| $\mathrm{I}_{\mathrm{OP}}$ | DC output current peak <br> (pulse at $1 \mathrm{~ms}, 10 \%$ duty cycle) | $\pm 400$ | mA |
| $\mathrm{I}_{\mathrm{CC}}$ or $\mathrm{I}_{\mathrm{GND}}$ | DC $\mathrm{V}_{\mathrm{CC}}$ or ground current | $\pm 100$ | mA |
| $\mathrm{P}_{\mathrm{D}}$ | Power dissipation at $\mathrm{T}_{\mathrm{A}}=70^{\circ} \mathrm{C}^{(1)}$ | 1120 | mW |
| $\mathrm{T}_{\text {STG }}$ | Storage temperature | -65 to 150 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\mathrm{L}}$ | Lead temperature (10 s) | 300 | ${ }^{\circ} \mathrm{C}$ |

1. Derate above $70^{\circ} \mathrm{C}$ by $18.5 \mathrm{~mW} /{ }^{\circ} \mathrm{C}$.

Table 5. Recommended operating conditions

| Symbol | Parameter | Value | Unit |
| :--: | :-- | :--: | :--: |
| $\mathrm{V}_{\mathrm{CC}}$ | Supply voltage | 1.65 to 4.5 | V |
| $\mathrm{V}_{\mathrm{I}}$ | Input voltage | 0 to $\mathrm{V}_{\mathrm{CC}}$ | V |
| $\mathrm{V}_{\mathrm{IC}}$ | Control input voltage | 0 to 4.5 | V |
| $\mathrm{V}_{\mathrm{O}}$ | Output voltage | 0 to $\mathrm{V}_{\mathrm{CC}}$ | V |
| $\mathrm{T}_{\mathrm{op}}$ | Operating temperature | -40 to 85 | ${ }^{\circ} \mathrm{C}$ |
| dt/dv | Input rise and fall time <br> control input | $\mathrm{V}_{\mathrm{CC}}=1.65$ to 2.7 V | 0 to 20 | ns/V |
|  |  | $\mathrm{V}_{\mathrm{CC}}=3.0$ to 4.5 V | 0 to 10 |  |# 3 Electrical characteristics 

### 3.1 DC electrical characteristics

Table 6. DC specifications

| Symbol | Parameter | $\mathrm{V}_{\mathrm{CC}}(\mathrm{V})$ | Test condition | Value |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | $T_{A}=25^{\circ} \mathrm{C}$ |  |  | -40 to $85^{\circ} \mathrm{C}$ |  |
|  |  |  |  | Min. | Typ. | Max. | Min. | Max. |
| $V_{i H}$ | High level input voltage | $1.65-1.95$ |  | $0.65 \mathrm{~V}_{\mathrm{CC}}$ |  |  | $0.65 \mathrm{~V}_{\mathrm{CC}}$ |  | V |
|  |  | $2.3-2.5$ |  | 1.2 |  |  | 1.2 |  |
|  |  | $2.7-3.0$ |  | 1.3 |  |  | 1.3 |  |
|  |  | $3.3-3.6$ |  | 1.4 |  |  | 1.4 |  |
|  |  | 4.5 |  | 1.6 |  |  | 1.6 |  |
| $V_{i L}$ | Low level input voltage | $1.65-1.95$ |  |  |  | 0.40 |  | 0.40 | V |
|  |  | $2.3-2.5$ |  |  |  | 0.60 |  | 0.60 |
|  |  | $2.7-3.0$ |  |  |  | 0.60 |  | 0.60 |
|  |  | $3.3-3.6$ |  |  |  | 0.60 |  | 0.60 |
|  |  | 4.5 |  |  |  | 0.80 |  | 0.80 |
| $R_{O N}$ | Switch ONresistance | 1.8 | $\begin{aligned} & V_{S}=0 \mathrm{~V} \text { to } V_{C C} \\ & I_{S}=100 \mathrm{~mA} \end{aligned}$ |  | 2.0 | 3.0 |  | 3.5 | $\Omega$ |
|  |  | 2.7 |  |  | 1.3 | 1.6 |  | 1.8 |
|  |  | 3.0 |  |  | 1.2 | 1.5 |  | 1.7 |
|  |  | 4.5 |  |  | 1.0 | 1.2 |  | 1.4 |
| $\Delta R_{\text {ON }}$ | ON-resistance match between channels ${ }^{(1)}$ | 1.8 | $\begin{aligned} & V_{S} \text { at } R_{O N} \max \\ & I_{S}=100 \mathrm{~mA} \end{aligned}$ |  | 0.06 |  |  |  | $\Omega$ |
|  |  | 2.7 |  |  | 0.05 |  |  |  |
|  |  | 3.0 |  |  | 0.05 |  |  |  |
|  |  | 4.5 |  |  | 0.05 |  |  |  |
| $R_{\text {FLAT }}$ | ON-resistance flatness ${ }^{(2)}$ | 1.8 | $\begin{aligned} & V_{S}=0 \mathrm{~V} \text { to } V_{C C} \\ & I_{S}=100 \mathrm{~mA} \end{aligned}$ |  | 1.0 | 1.5 |  | 1.5 | $\Omega$ |
|  |  | 2.7 |  |  | 0.45 | 0.60 |  | 0.70 |
|  |  | 3.0 |  |  | 0.43 | 0.50 |  | 0.60 |
|  |  | 4.5 |  |  | 0.39 | 0.50 |  | 0.60 |
| IOFF | OFF state leakage current (Sn), (D) | 4.3 | $V_{S}=0.3$ or 4 V |  |  | $\pm 20$ |  | $\pm 100$ | nA |
| $I_{I N}$ | Input leakage current | $0-4.5$ | $\mathrm{V}_{\text {SEL }}=0$ to 4.5 V |  |  | $\pm 0.1$ |  | $\pm 1.0$ | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\mathrm{CC}}$ | Quiescent supply current | $1.65-4.5$ | $\begin{aligned} & \mathrm{V}_{\text {SEL }}=\mathrm{V}_{\mathrm{CC}} \text { or } \\ & \text { GND } \end{aligned}$ |  |  | $\pm 0.1$ |  | $\pm 1.0$ | $\mu \mathrm{A}$ |Table 6. DC specifications (continued)

| Symbol | Parameter | $\mathrm{V}_{\mathrm{CC}}(\mathrm{V})$ | Test condition | Value |  |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | $T_{A}=25^{\circ} \mathrm{C}$ |  |  | -40 to $85^{\circ} \mathrm{C}$ |  |  |
|  |  |  |  | Min. | Typ. | Max. | Min. | Max. |  |
| $\mathrm{I}_{\text {CCLV }}$ | Quiescent supply current low voltage driving | 4.3 | $\mathrm{V}_{\text {SEL }}=1.65 \mathrm{~V}$ |  | $\pm 17$ | $\pm 35$ |  | $\pm 70$ | $\mu \mathrm{A}$ |
|  |  | 4.3 | $\mathrm{V}_{\text {SEL }}=1.80 \mathrm{~V}$ |  | $\pm 15$ | $\pm 30$ |  | $\pm 60$ |  |
|  |  | 4.3 | $\mathrm{V}_{\text {SEL }}=2.60 \mathrm{~V}$ |  | $\pm 5$ | $\pm 10$ |  | $\pm 20$ |  |

1. $\Delta R_{O N}=R_{O N(\text { Max })} \cdot R_{O N(\text { Min })}$
2. Flatness is defined as the difference between the maximum and minimum value of ON-resistance as measured over the specified analog signal ranges.

# 3.2 AC electrical characteristics 

Table 7. AC electrical characteristics $\left(C_{L}=35 \mathrm{pF}, R_{L}=50 \Omega, t_{f}=t_{f} \leq 5 \mathrm{~ns}\right)$

| Symbol | Parameter | $\begin{gathered} \mathrm{V}_{\mathrm{CC}} \\ (\mathrm{~V}) \end{gathered}$ | Test conditions | Value |  |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  |  | $T_{A}=25^{\circ} \mathrm{C}$ |  |  | -40 to $85^{\circ} \mathrm{C}$ |  |
|  |  |  |  |  | Min. | Typ. | Max. | Min. | Max. |
| $t_{\text {PLH }}, t_{\text {PHL }}$ | Propagation delay | 1.65 - 1.95 |  |  | 0.15 |  |  |  | ns |
|  |  | 2.3 - 2.7 |  |  | 0.14 |  |  |  |  |
|  |  | $3.0-3.3$ |  |  | 0.13 |  |  |  |  |
|  |  | $3.6-5.0$ |  |  | 0.13 |  |  |  |  |
| $t_{\text {ON }}$ | Turn-ON time | 1.65 - 1.95 | $\mathrm{V}_{\mathrm{S}}=0.8 \mathrm{~V}$ |  | 36 |  |  |  | ns |
|  |  | 2.3 - 2.7 | $\mathrm{V}_{\mathrm{S}}=1.5 \mathrm{~V}$ |  | 31 | 40 |  | 45 |  |
|  |  | $3.0-3.3$ |  |  | 24 | 31 |  | 40 |  |
|  |  | $3.6-5.0$ |  |  | 21 | 28 |  | 32 |  |
| $t_{\text {OFF }}$ | Turn-OFF time | 1.65 - 1.95 | $\mathrm{V}_{\mathrm{S}}=0.8 \mathrm{~V}$ |  | 29 |  |  |  | ns |
|  |  | 2.3 - 2.7 | $\mathrm{V}_{\mathrm{S}}=1.5 \mathrm{~V}$ |  | 17 | 27 |  | 37 |  |
|  |  | $3.0-3.3$ |  |  | 12 | 23 |  | 33 |  |
|  |  | $3.6-5.0$ |  |  | 11 | 21 |  | 31 |  |
| $t_{D}$ | Break-before- <br> make time delay | 1.65 - 1.95 | $\begin{aligned} & \mathrm{C}_{\mathrm{L}}=35 \mathrm{pF} \\ & \mathrm{R}_{\mathrm{L}}=50 \Omega \\ & \mathrm{~V}_{\mathrm{S}}=1.5 \mathrm{~V} \end{aligned}$ |  | 15 |  |  |  | ns |
|  |  | 2.3 - 2.7 |  |  | 10 |  |  |  |  |
|  |  | $3.0-3.3$ |  |  | 8 |  |  |  |  |
|  |  | $3.6-5.0$ |  |  | 6 |  |  |  |  |
| Q | Charge injection | 1.65 | $\begin{aligned} & \mathrm{C}_{\mathrm{L}}=100 \mathrm{pF} \\ & \mathrm{~V}_{\mathrm{GEN}}=0 \mathrm{~V} \\ & \mathrm{R}_{\mathrm{GEN}}=0 \Omega \end{aligned}$ |  | 16 |  |  |  | pC |
|  |  | 2.3 |  |  | 22 |  |  |  |  |
|  |  | 3 |  |  | 26 |  |  |  |  |
|  |  | 5.0 |  |  | 33 |  |  |  |  |# 3.3 Analog switch characteristics 

Table 8. Analog switch characteristics $\left(C_{L}=5 \mathrm{pF}, R_{L}=50 \Omega, T_{A}=25^{\circ} \mathrm{C}\right)$

| Symbol | Parameter | $\mathrm{V}_{\mathrm{CC}}(\mathrm{V})$ | Test conditions | Value |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | $\mathrm{T}_{\mathrm{A}}=25^{\circ} \mathrm{C}$ |  | -40 to $85^{\circ} \mathrm{C}$ |  |  |
|  |  |  |  | Min. | Typ. | Max. | Min. | Max. |
| OIRR | OFF isolation ${ }^{(1)}$ | $1.65-5.0$ | $\begin{aligned} & \mathrm{V}_{\mathrm{S}}=1 \mathrm{~V}_{\text {RMS }} \\ & \mathrm{f}=100 \mathrm{kHz} \end{aligned}$ |  | $-75$ |  |  |  | dB |
| Xtalk | Crosstalk | $1.6-5.0$ | $\begin{aligned} & \mathrm{V}_{\mathrm{S}}=1 \mathrm{~V}_{\text {RMS }} \\ & \mathrm{f}=100 \mathrm{kHz} \end{aligned}$ |  | $-80$ |  |  |  | dB |
| THD | Total harmonic distortion | $2.3-5.0$ | $\begin{aligned} & R_{L}=600 \Omega \\ & V_{S}=2 \mathrm{~V}_{\mathrm{PP}} \\ & f=20 \mathrm{~Hz} \text { to } \\ & 20 \mathrm{kHz} \end{aligned}$ |  | 0.03 |  |  |  | \% |
| BW | -3 dB bandwidth | $1.65-5.0$ | $R_{L}=50 \Omega$ |  | 150 |  |  |  | MHz |
| $\mathrm{C}_{\text {IN }}$ | Control pin input capacitance |  |  |  | 6 |  |  |  | pF |
| $\mathrm{C}_{\mathrm{ON}}$ | Sn port capacitance when switch is enabled | 3.3 | $f=1 \mathrm{MHz}$ |  | 52 |  |  |  |  |
| $\mathrm{C}_{\text {OFF }}$ | Sn port capacitance when switch is disabled | 3.3 | $f=1 \mathrm{MHz}$ |  | 25 |  |  |  |  |
| $C_{D}$ | D port capacitance when switch is enabled | 3.3 | $f=1 \mathrm{MHz}$ |  | 50 |  |  |  |  |

1. OFF isolation $=20 \log _{10}\left(V_{D} / V_{S}\right), V_{D}=$ output. $V_{S}=$ input to OFF switch.# 4 Test circuits 

Figure 3. ON resistance
![img-3.jpeg](img-3.jpeg)

Figure 4. Bandwidth
![img-4.jpeg](img-4.jpeg)Figure 5. OFF leakage
![img-5.jpeg](img-5.jpeg)

Figure 6. Channel-to-channel crosstalk
![img-6.jpeg](img-6.jpeg)Figure 7. OFF isolation
![img-7.jpeg](img-7.jpeg)

Figure 8. Test circuit
![img-8.jpeg](img-8.jpeg)

1. $C_{L}=5 / 35 \mathrm{pF}$ or equivalent: (includes jig capacitance).
2. $R_{L}=50 \Omega$ or equivalent.
3. $R_{T}=Z_{\text {OUT }}$ of pulse generator (typically $50 \Omega$ ).Figure 9. Break-before-make time delay
![img-9.jpeg](img-9.jpeg)

Figure 10. Switching time and charge injection $\left(\mathrm{V}_{\mathrm{GEN}}=0 \mathrm{~V}, \mathrm{R}_{\mathrm{GEN}}=0 \Omega, \mathrm{R}_{\mathrm{L}}=1 \mathrm{M} \Omega, \mathrm{C}_{\mathrm{L}}=100 \mathrm{pF}\right)$
![img-10.jpeg](img-10.jpeg)

Figure 11. Turn-on, turn-off delay time
![img-11.jpeg](img-11.jpeg)# 5 Package information 

In order to meet environmental requirements, ST offers these devices in different grades of ECOPACK ${ }^{\circledR}$ packages, depending on their level of environmental compliance. ECOPACK specifications, grade definitions and product status are available at: www.st.com. ECOPACK is an ST trademark.

Figure 12. DFN6L ( $1.2 \times 1 \mathrm{~mm}$ ) package outline
![img-12.jpeg](img-12.jpeg)

1. Drawing is not to scale.Table 9. DFN6L ( $1.2 \times 1 \mathrm{~mm}$ ) mechanical data

| Symbol | Dimensions (millimeters) |  |  |
| :--: | :--: | :--: | :--: |
|  | Typ. | Min. | Max. |
| A | 0.50 | 0.45 | 0.55 |
| A1 | 0.02 | 0 | 0.05 |
| A3 | 0.127 |  |  |
| b | 0.20 | 0.15 | 0.25 |
| D | 1.20 | 1.15 | 1.25 |
| E | 1 | 0.95 | 1.05 |
| e | 0.40 |  |  |
| L | 0.35 | 0.30 | 0.40 |
| L1 | 0.45 | 0.40 | 0.50 |

Figure 13. DFN6L ( $1.2 \times 1 \mathrm{~mm}$ ) footprint recommendation
![img-13.jpeg](img-13.jpeg)Figure 14. DFN6L carrier tape information
![img-14.jpeg](img-14.jpeg)

1. Measured from centreline of sprocket hole to centreline of pocket.
2. Cumulative tolerance of 10 sprocket holes is $\pm 0.20$.
3. Measured from centreline of sprocket hole to centreline of pocket.
4. Other material available.
5. Drawing is not to scale.
6. All dimensions are in millimeters unless otherwise stated.Figure 15. DFN6L reel information drawing (back view)
![img-15.jpeg](img-15.jpeg)

1. Drawing is not to scale.
2. Dimensions are in millimeters.Figure 16. DFN6L reel information drawing (front view)
![img-16.jpeg](img-16.jpeg)

1. Drawing not to scale.

Dimensions are in millimeters.# 6 Revision history 

Table 10. Document revision history

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 07-Mar-2014 | 1 | Initial release. |Please Read Carefully:

Information in this document is provided solely in connection with ST products. STMicroelectronics NV and its subsidiaries ("ST") reserve the right to make changes, corrections, modifications or improvements, to this document, and the products and services described herein at any time, without notice.

All ST products are sold pursuant to ST's terms and conditions of sale.
Purchasers are solely responsible for the choice, selection and use of the ST products and services described herein, and ST assumes no liability whatsoever relating to the choice, selection or use of the ST products and services described herein.

No license, express or implied, by estoppel or otherwise, to any intellectual property rights is granted under this document. If any part of this document refers to any third party products or services it shall not be deemed a license grant by ST for the use of such third party products or services, or any intellectual property contained therein or considered as a warranty covering the use in any manner whatsoever of such third party products or services or any intellectual property contained therein.

UNLESS OTHERWISE SET FORTH IN ST'S TERMS AND CONDITIONS OF SALE ST DISCLAIMS ANY EXPRESS OR IMPLIED WARRANTY WITH RESPECT TO THE USE AND/OR SALE OF ST PRODUCTS INCLUDING WITHOUT LIMITATION IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE (AND THEIR EQUIVALENTS UNDER THE LAWS OF ANY JURISDICTION), OR INFRINGEMENT OF ANY PATENT, COPYRIGHT OR OTHER INTELLECTUAL PROPERTY RIGHT.
ST PRODUCTS ARE NOT DESIGNED OR AUTHORIZED FOR USE IN: (A) SAFETY CRITICAL APPLICATIONS SUCH AS LIFE SUPPORTING, ACTIVE IMPLANTED DEVICES OR SYSTEMS WITH PRODUCT FUNCTIONAL SAFETY REQUIREMENTS; (B) AERONAUTIC APPLICATIONS; (C) AUTOMOTIVE APPLICATIONS OR ENVIRONMENTS, AND/OR (D) AEROSPACE APPLICATIONS OR ENVIRONMENTS. WHERE ST PRODUCTS ARE NOT DESIGNED FOR SUCH USE, THE PURCHASER SHALL USE PRODUCTS AT PURCHASER'S SOLE RISK, EVEN IF ST HAS BEEN INFORMED IN WRITING OF SUCH USAGE, UNLESS A PRODUCT IS EXPRESSLY DESIGNATED BY ST AS BEING INTENDED FOR "AUTOMOTIVE, AUTOMOTIVE SAFETY OR MEDICAL" INDUSTRY DOMAINS ACCORDING TO ST PRODUCT DESIGN SPECIFICATIONS. PRODUCTS FORMALLY ESCC, QML OR JAN QUALIFIED ARE DEEMED SUITABLE FOR USE IN AEROSPACE BY THE CORRESPONDING GOVERNMENTAL AGENCY.

Resale of ST products with provisions different from the statements and/or technical features set forth in this document shall immediately void any warranty granted by ST for the ST product or service described herein and shall not create or extend in any manner whatsoever, any liability of ST.

ST and the ST logo are trademarks or registered trademarks of ST in various countries.
Information in this document supersedes and replaces all information previously supplied.
The ST logo is a registered trademark of STMicroelectronics. All other names are the property of their respective owners.
(c) 2014 STMicroelectronics - All rights reserved

STMicroelectronics group of companies
Australia - Belgium - Brazil - Canada - China - Czech Republic - Finland - France - Germany - Hong Kong - India - Israel - Italy - Japan Malaysia - Malta - Morocco - Philippines - Singapore - Spain - Sweden - Switzerland - United Kingdom - United States of America
www.st.com