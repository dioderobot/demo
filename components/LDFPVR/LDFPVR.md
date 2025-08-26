# 1 A very low drop voltage regulator 

## Features

- Input voltage from 2.6 to 16 V
- Very low-dropout voltage ( 500 mV max. at 1 A load)
- Low quiescent current ( $200 \mu \mathrm{~A}$ typ. @ 1 A load)
- Available in 1\% precision in DFN6 package
- 1 A guaranteed output current
- Wide range of output voltages available on request: adjustable from 0.8 V
- Logic-controlled electronic shutdown
- Power Good DFN package
- Fast dynamic response to line and load changes
- Internal current and thermal protections
- Temperature range: $-40^{\circ} \mathrm{C}$ to $125^{\circ} \mathrm{C}$


## Applications

## Maturity status link

LDF

- Computer and laptop
- Battery-powered equipments
- Industrial and medical equipment
- Consumer and set-top box


## Description

The LDF is a fast, very low drop linear regulator which operates from an input supply voltage in the range of 2.6 V to 16 V .

It is available in adjustable output voltage versions, from 0.8 V to 12 V .
The LDF features are: high output precision, very low-dropout voltage, low noise, and low quiescent current, therefore suitable for low voltage microprocessors and memory applications.

Enable logic control pin and Power Good output are featured on DFN package.
Current and thermal protection are provided.1 Block diagram

Figure 1. Block diagram
![img-0.jpeg](img-0.jpeg)Figure 2. Pin connection (top view)
![img-1.jpeg](img-1.jpeg)

DFN6

Table 1. DFN6-2x2 and 3x3 pin description

| Pin $\mathbf{n}^{\circ}$ | Symbol | Function |
| :--: | :--: | :-- |
| 2 | ADJ | For adjustable versions: error amplifier input pin |
| 6 | $\mathrm{V}_{\text {IN }}$ | Input voltage |
| 1 | $\mathrm{V}_{\text {OUT }}$ | Output voltage |
| 5 | EN | Enable pin logic input: low = shutdown, high = active |
| 3 | PG | Power-good output |
| 4 | GND | Ground |
| Exposed pad | GND | Ground |# 3 Absolute maximum ratings 

Table 2. Absolute maximum ratings

| Symbol | Parameter | Value | Unit |
| :-- | :-- | :-- | :-- |
| $\mathrm{V}_{\text {IN }}$ | DC input voltage | -0.3 to 20 | V |
| $\mathrm{V}_{\text {OUT }}$ | DC output voltage | -0.3 to $\mathrm{V}_{\text {IN }}+0.3$ | V |
| $\mathrm{V}_{\text {EN }}$ | Enable input voltage | -0.3 to $\mathrm{V}_{\text {IN }}+0.3$ | V |
| $\mathrm{V}_{\text {ADJ }}$ | ADJ pin voltage | -0.3 to 2 | V |
| $\mathrm{V}_{\text {PG }}$ | PG pin voltage | -0.3 to $\mathrm{V}_{\text {IN }}+0.3$ | V |
| $\mathrm{I}_{\text {LOAD }}$ | Output current | Internally limited | mA |
| $\mathrm{P}_{\mathrm{D}}$ | Power dissipation | Internally limited | mW |
| $\mathrm{T}_{\text {STG }}$ | Storage temperature range | -65 to 150 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\text {OP }}$ | Operating junction temperature range | -40 to 125 | ${ }^{\circ} \mathrm{C}$ |

Note: Absolute maximum ratings are those values beyond which damage to the device may occur. Functional operation under these conditions is not implied. All values are referred to GND.

Table 3. Thermal data

| Symbol | Parameter | Value |  | Unit |
| :-- | :-- | :-- | :-- | :-- |
|  |  | DFN6-2x2 | DFN6-3x3 |  |
| $\mathrm{R}_{\text {INJA }}$ | Thermal resistance junction-ambient | 65 | 55 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{R}_{\text {INJC }}$ | Thermal resistance junction-case | 6.5 | 10 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |# 4 Electrical characteristics 

$T_{J}=25^{\circ} \mathrm{C}, \mathrm{V}_{\text {IN }}=\mathrm{V}_{\text {OUT(NOM) }}+1 \mathrm{~V}, \mathrm{C}_{\text {IN }}=1 \mu \mathrm{~F}, \mathrm{C}_{\text {OUT }}=2.2 \mu \mathrm{~F}, \mathrm{I}_{\text {LOAD }}=10 \mathrm{~mA}, \mathrm{~V}_{\mathrm{EN}}=2 \mathrm{~V}$, unless otherwise specified.

Table 4. LDF (adjustable version) electrical characteristics

| Symbol | Parameter | Test conditions | Min. | Typ. | Max. | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {IN }}$ | Operating input voltage |  | 2.6 |  | 16 | V |
| $\mathrm{V}_{\text {ADJ }}$ | Reference voltage | $\mathrm{V}_{\text {IN }}=\mathrm{V}_{\text {OUT }}+1 \mathrm{~V}^{(1)}$ |  | 0.8 |  | V |
|  |  | $\mathrm{V}_{\text {OUT }}+1 \mathrm{~V} \leq \mathrm{V}_{\text {IN }} \leq 16 \mathrm{~V} \mathrm{I}_{\text {LOAD }}=10 \mathrm{~mA}^{(1)}$ | $-1$ |  | 1 | \% |
|  |  | $10 \mathrm{~mA} \leq \mathrm{I}_{\text {LOAD }} \leq 1 \mathrm{~A} \mathrm{~T}_{\mathrm{J}}=-40$ to $125^{\circ} \mathrm{C}$ | $-1.5$ |  | 1.5 |  |
| $\Delta \mathrm{V}_{\text {OUT }}$ | Static line regulation | $\mathrm{V}_{\text {OUT }}+1 \mathrm{~V} \leq \mathrm{V}_{\text {IN }} \leq 16 \mathrm{~V}$ Section 4: Electrical characteristics |  | 0.01 |  | $\% \mathrm{~V}$ |
|  |  | $\mathrm{V}_{\text {OUT }}+1 \mathrm{~V} \leq \mathrm{V}_{\text {IN }} \leq 16 \mathrm{~V}, \mathrm{~T}_{\mathrm{J}}=-40$ to 125 <br> *CSection 4: Electrical characteristics |  |  | 0.04 |  |
| $\Delta \mathrm{V}_{\text {OUT }}$ | Static load regulation | $10 \mathrm{~mA} \leq \mathrm{I}_{\text {LOAD }} \leq 1 \mathrm{~A}$ |  | 0.2 |  | $\% / A$ |
|  |  | $10 \mathrm{~mA} \leq \mathrm{I}_{\text {LOAD }} \leq 1 \mathrm{~A}, \mathrm{~T}_{\mathrm{J}}=-40$ to $125^{\circ} \mathrm{C}$ |  | 0.2 | 0.6 |  |
| $\mathrm{V}_{\text {DROP }}$ | Dropout voltage ${ }^{(2)}$ | $\mathrm{V}_{\text {OUT }}$ fixed to $2.5 \mathrm{~V}, \mathrm{I}_{\text {LOAD }}=1 \mathrm{~A},-40^{\circ} \mathrm{C}<\mathrm{T}_{\mathrm{J}}<125^{\circ} \mathrm{C}$ |  | 200 | 500 | mV |
| $\mathrm{I}_{\text {g }}$ | Quiescent current | ON mode: $\mathrm{V}_{\mathrm{EN}}=2 \mathrm{~V} \mathrm{I}_{\text {LOAD }}=10 \mathrm{~mA}$ to $1 \mathrm{~A}, \mathrm{~T}_{\mathrm{J}}=-40$ to $125^{\circ} \mathrm{C}$ |  | 200 | 800 | $\mu \mathrm{A}$ |
|  |  | OFF mode: $\mathrm{V}_{\mathrm{EN}}=$ GND, PPAK and DFN versions |  | 30 |  |  |
|  |  | OFF mode: $\mathrm{V}_{\mathrm{EN}}=$ GND, PPAK and DFN versions, -40 ${ }^{\circ} \mathrm{C}<\mathrm{T}_{\mathrm{J}}<125^{\circ} \mathrm{C}$ |  |  | 120 |  |
| $\mathrm{I}_{\mathrm{SC}}$ | Short-circuit current | $\mathrm{V}_{\text {IN }}>3 \mathrm{~V}$ |  | 1.5 |  | A |
| $\mathrm{V}_{\mathrm{EN}}$ | Enable input logic low | $\mathrm{V}_{\text {IN }}=2.6 \mathrm{~V}$ to $16 \mathrm{~V},-40^{\circ} \mathrm{C}<\mathrm{T}_{\mathrm{J}}<125^{\circ} \mathrm{C}$ |  |  | 0.8 | V |
|  | Enable input logic high |  | 2 |  |  |  |
| $\mathrm{I}_{\mathrm{EN}}$ | Enable pin input current | $\mathrm{V}_{\mathrm{EN}}=\mathrm{V}_{\text {IN }}$ |  | 5 | 10 | $\mu \mathrm{A}$ |
| PG | Power-good output threshold | Rising edge |  | $0.92^{*} \mathrm{~V}_{\text {ADJ }}$ |  | V |
|  |  | Falling edge |  | $0.8^{*} \mathrm{~V}_{\text {ADJ }}$ |  |  |
|  | Power-good output voltage low | $\mathrm{I}_{\text {SINK }}=6 \mathrm{~mA}$, open drain output |  | 0.4 |  |  |
| SVR | Supply voltage rejection | $\begin{aligned} & \mathrm{V}_{\text {IN }}=3 \mathrm{~V}+/-0.5 \mathrm{~V}_{\text {RIPPLE }} \mathrm{f}=120 \mathrm{~Hz}, \mathrm{~V}_{\text {OUT }}=0.8 \mathrm{~V} \\ & \mathrm{~V}_{\text {IN }}=3 \mathrm{~V}+/-0.5 \mathrm{~V}_{\text {RIPPLE }} \mathrm{f}=120 \mathrm{~Hz} \text { to } 100 \mathrm{kHz} \mathrm{V}_{\text {OUT }}= \\ & 0.8 \mathrm{~V} \end{aligned}$ |  | 62 |  | dB |
|  |  | $\mathrm{V}_{\text {IN }}=3 \mathrm{~V}+/-0.5 \mathrm{~V}_{\text {RIPPLE }} \mathrm{f}=120 \mathrm{~Hz}$ to $100 \mathrm{kHz} \mathrm{V}_{\text {OUT }}=$ 0.8 V |  | 55 |  |  |
| $e_{N}$ | Output noise voltage | $\mathrm{B}_{\mathrm{w}}=10 \mathrm{~Hz}$ to $100 \mathrm{kHz}, \mathrm{I}_{\text {LOAD }}=100 \mathrm{~mA} \mathrm{C}_{\text {OUT }}=2.2 \mu \mathrm{~F}$ |  | 50 |  | $\begin{gathered} \mu \mathrm{V}_{\text {RMS }} / \\ \mathrm{V}_{\text {OUT }} \end{gathered}$ |
| $\mathrm{T}_{\text {SHDN }}$ | Thermal shutdown |  |  | 170 |  | ${ }^{\circ} \mathrm{C}$ |
|  | Hysteresis |  |  | 10 |  |  |

1. For $\mathrm{V}_{\text {OUT }}<1.6 \mathrm{~V} ; \mathrm{V}_{\text {IN }}=2.6 \mathrm{~V}$.
2. Dropout voltage is the input-to-output voltage difference at which the output voltage is 100 mV below its nominal value. This specification does not apply to output voltages below 1.6 V .# 5.1 External capacitors 

The LDF voltage regulator requires external ceramic capacitors to assure the control loop stability. These capacitors must be selected to meet the requirements of minimum capacitance and equivalent series resistance (see Section 6: Typical characteristics and Section 6: Typical characteristics. Input/output capacitors should be located as closer as possible to the relative pins.

### 5.1.1 Input capacitor

An input capacitor, whose minimum value is $1 \mu \mathrm{~F}$, must not be located farther than $0.5^{\circ}$ from the input pin of the device and returned to a clean analog ground.

### 5.1.2 Output capacitor

Ceramic capacitors could be used on the output, provided that they must meet the minimum amount of capacitance and E.S.R. (equivalent series resistance) value required. $2.2 \mu \mathrm{~F}$ is suggested as minimum capacitance to guarantee the stability of the regulator. Anyway, other COUT values can be used according to the Section 6: Typical characteristics and Section 6: Typical characteristics showing the allowable ESR range as a function of the output capacitance. The output capacitor must maintain its ESR in the stable region over the full operating temperature range to assure stability. Besides, capacitor tolerance and temperature variation must be taken into account to assure the minimum amount of capacitance.

### 5.2 Output voltage setting for ADJ version

In the adjustable version, the output voltage can be set from 0.8 V up to the input voltage minus the voltage drop across the pass transistor (dropout voltage), by connecting a resistor divider between the ADJ pin and the output, thus allowing remote voltage sensing.
The resistor divider could be selected by the following equation:

$$
V_{O U T}=V_{A D J}\left(1+R 1 / R 2\right), \text { with } V_{A D J}=0.8 \mathrm{~V}(\text { Typ. })
$$

It is recommended to use resistors with values in the range of $10 \mathrm{k} \Omega$ to $100 \mathrm{k} \Omega$. Lower values can also be suitable, but current consumption increases.

### 5.3 Enable pin operation

This pin can be used to turn OFF the regulator when it is pulled down, so to drastically reduce the current consumption. When the enable feature is not used, this pin must be tied to $\mathrm{V}_{\mathrm{IN}}$ to keep the regulator output in ON state every time. To assure the proper operation, the signal source, used to drive the EN pin, must be able to swing above and below the specified thresholds listed in the electrical characteristics $\left(\mathrm{V}_{\mathrm{EN}}\right)$. The EN pin must not be left floating because it is not internally pulled down/up.

### 5.4 Power Good

The LDF features an open drain PG pin to sequence either external supplies or loads and to provide fault detection. This pin requires an external resistor $\left(R_{P G}\right)$ to pull Power Good high when the output is within the power-good tolerance window. Typical values for this resistor range from $10 \mathrm{k} \Omega$ to $100 \mathrm{k} \Omega$.# 6 Typical characteristics 

$$
C_{I N}=C_{\text {OUT }}=1 \mu \mathrm{~F}, V_{\text {IN }}=V_{\text {OUT }}+1 \mathrm{~V}, V_{\text {EN }} \text { to } V_{\text {IN }}, I_{\text {OUT }}=10 \mathrm{~mA} \text {, unless otherwise specified. }
$$

Figure 3. Line regulation vs. temperature
![img-2.jpeg](img-2.jpeg)

Figure 4. Load regulation vs. temperature
![img-3.jpeg](img-3.jpeg)

Figure 5. Short-circuit current vs. dropout
![img-4.jpeg](img-4.jpeg)

Figure 6. Dropout voltage vs. temperature
![img-5.jpeg](img-5.jpeg)

Figure 7. Quiescent current vs. temperature, $\mathrm{I}_{\text {OUT }}=\mathbf{1 0} \mathrm{mA}$
![img-6.jpeg](img-6.jpeg)

Figure 8. Quiescent current vs. temperature, $\mathrm{I}_{\text {OUT }}=\mathbf{1} \mathrm{A}$
![img-7.jpeg](img-7.jpeg)Figure 9. Shutdown current vs. temperature
![img-8.jpeg](img-8.jpeg)

Figure 10. Enable pin current vs. temperature
![img-9.jpeg](img-9.jpeg)

Figure 11. Enable high threshold vs. temperature
![img-10.jpeg](img-10.jpeg)

Figure 12. Enable low threshold vs. temperature
![img-11.jpeg](img-11.jpeg)

Figure 13. Output voltage vs. temperature, adjustable version
![img-12.jpeg](img-12.jpeg)

Figure 14. Load transient $\left(\mathrm{V}_{\text {OUT }}=\mathrm{V}_{\text {ADJ }}\right)$
![img-13.jpeg](img-13.jpeg)Figure 15. SVR vs. frequency $\left(\mathrm{V}_{\text {OUT }}=\mathrm{V}_{\text {ADJ }}\right)$
![img-14.jpeg](img-14.jpeg)

Figure 16. Stability plane ADJ ( $\mathrm{C}_{\text {OUT }}$. ESR)
![img-15.jpeg](img-15.jpeg)# 7 Package information 

To meet environmental requirements, ST offers these devices in different grades of ECOPACK packages, depending on their level of environmental compliance. ECOPACK specifications, grade definitions, and product status are available at: www.st.com. ECOPACK is an ST trademark.

### 7.1 DFN6 (3x3) package information

Figure 17. DFN6 (3x3) package outline
![img-16.jpeg](img-16.jpeg)# Table 5. DFN6 (3x3) mechanical data

|  Dim. | mm |  |   |
| --- | --- | --- | --- |
|   | Min. | Typ. | Max.  |
|  A | 0.80 |  | 1  |
|  A1 | 0 | 0.02 | 0.05  |
|  A3 |  | 0.20 |   |
|  b | 0.23 |  | 0.45  |
|  D | 2.90 | 3 | 3.10  |
|  D2 | 2.23 |  | 2.50  |
|  E | 2.90 | 3 | 3.10  |
|  E2 | 1.50 |  | 1.75  |
|  e |  | 0.95 |   |
|  L | 0.30 | 0.40 | 0.50  |

# Figure 18. DFN6 (3x3) recommended footprint

## FOOTPRINT RECOMMENDED

![img-17.jpeg](img-17.jpeg)# 7.3 DFN6 (2x2x0,90) package information 

Figure 19. DFN6 (2x2) package outline
![img-18.jpeg](img-18.jpeg)Table 6. DFN6 (2x2) mechanical data

|  Dim. | mm |  |   |
| --- | --- | --- | --- |
|   | Min. | $3 \mathrm{~cm}$. | Max.  |
|  A | 0.80 | 0.90 | 1.00  |
|  A1 | 0.00 | 0.02 | 0.05  |
|  b | 0.25 | 0.30 | 0.35  |
|  D | 2.00 BSC |  |   |
|  E | 2.00 BSC |  |   |
|  e | 0.65 BSC |  |   |
|  D2 | 1.45 |  | 1.70  |
|  E2 | 0.85 |  | 1.10  |
|  L | 0.20 |  | 0.30  |
|  K |  | 0.15 |   |
|  aaa |  | 0.05 |   |
|  bbb |  | 0.10 |   |
|  ccc |  | 0.10 |   |
|  ddd |  | 0.05 |   |
|  eee |  | 0.08 |   |
|  N |  | 6 |   |Figure 20. DFN6 (2x2) recommended footprint
![img-19.jpeg](img-19.jpeg)

Notes:

1) This footprint is able to ensure insulation up to 60 Vrms (according to CEI IEC 664-1)
2) The device must be positioned within $\oplus 0.02 \mathrm{~A} \mathrm{~B}$# 8 Ordering information

Table 7. Order code

|  Package |  | Output voltage (V)  |
| --- | --- | --- |
|  DFN6-2x3 | DFN6-2x2 |   |
|  LDFPUR | LDFPVR | ADJ  |# Revision history

Table 8. Document revision history

|  Date | Revision | Changes  |
| --- | --- | --- |
|  05-Dec-2013 | 1 | Initial release.  |
|  12-Apr-2017 | 2 | Updated Figure 14: "Enable pin current vs. temperature" and Section 8:
"Package information".
Added Section 6.2: "Output voltage setting for ADJ version".
Minor text changes.  |
|  08-Oct-2024 | 3 | Updated Table 7. Order code.  |# - Disclaimer 

## IMPORTANT NOTICE - READ CAREFULLY

STMicroelectronics NV and its subsidiaries ("ST") reserve the right to make changes, corrections, enhancements, modifications, and improvements to ST products and/or to this document at any time without notice. Purchasers should obtain the latest relevant information on ST products before placing orders. ST products are sold pursuant to ST's terms and conditions of sale in place at the time of order acknowledgment.

Purchasers are solely responsible for the choice, selection, and use of ST products and ST assumes no liability for application assistance or the design of purchasers' products.

No license, express or implied, to any intellectual property right is granted by ST herein.
Resale of ST products with provisions different from the information set forth herein shall void any warranty granted by ST for such product.
ST and the ST logo are trademarks of ST. For additional information about ST trademarks, refer to www.st.com/trademarks. All other product or service names are the property of their respective owners.

Information in this document supersedes and replaces information previously supplied in any prior versions of this document.
(c) 2024 STMicroelectronics - All rights reserved