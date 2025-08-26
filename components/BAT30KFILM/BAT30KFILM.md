# Small signal Schottky diodes 

Datasheet - production data
![img-0.jpeg](img-0.jpeg)

## Description

The BAT30 series uses 30 V Schottky barrier diodes encapsulated in SOD-523 or SOT-323 packages.
This device is specially suited for switching mode applications needing low forward voltage drop diodes.

## Features

- Very low conduction losses
- Negligible switching losses
- Low forward and reverse recovery times
- Extremely fast switching
- Surface mount device
- Low capacitance diode
- ECOPACK ${ }^{\circledR} 2$ and RoHS compliant component

Table 1. Device summary

| Symbol | Value |
| :--: | :--: |
| $\mathrm{I}_{\mathrm{F}}$ | 300 mA |
| $\mathrm{V}_{\text {RRM }}$ | 30 V |
| C (typ.) | 14 pF |
| $\mathrm{T}_{\mathrm{j}}$ (max.) | $150^{\circ} \mathrm{C}$ |# 1 Characteristics 

Table 2. Absolute ratings (limiting values at $\mathrm{T}_{\text {amb }}=25^{\circ} \mathrm{C}$, unless otherwise specified)

| Symbol | Parameter | Value | Unit |
| :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {RRM }}$ | Repetitive peak reverse voltage | 30 | V |
| $\mathrm{I}_{\mathrm{F}}$ | Continuous forward current | 300 | mA |
| $\mathrm{I}_{\text {FSM }}$ | Surge non repetitive forward current | $t_{p}=10 \mathrm{~ms}$ Sinusoidal | 1 A |
| $\mathrm{I}_{\text {FRM }}$ | Repetitive peak forward current, square wave | $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}, \delta=0.1$ | 0.9 A |
| $\mathrm{P}_{\mathrm{D}}{ }^{(1)}$ | Power dissipation | SOT-323 | 225 mW |
|  |  | SOD-523 | 200 |
| $\mathrm{T}_{\text {stg }}$ | Storage temperature range |  | -65 to +150 ${ }^{\circ} \mathrm{C}$ |
| $T_{j}$ | Maximum operating junction temperature | 150 | ${ }^{\circ} \mathrm{C}$ |
| $T_{L}$ | Maximum soldering temperature | 260 | ${ }^{\circ} \mathrm{C}$ |

1. On epoxy printed circuit board with recommended pad layout

Table 3. Thermal parameters

| Symbol | Parameter |  | Value | Unit |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{R}_{\text {(hij-a) }}$ | Junction to ambient ${ }^{(1)}$ | SOT-323 | 550 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
|  |  | SOD-523 | 600 |  |

1. On epoxy printed circuit board with recommended pad layoutTable 4. Static electrical characteristics

| Symbol | Parameter | Test conditions |  | Min. | Typ. | Max. | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $I_{R}{ }^{(1)}$ | Reverse leakage current | $T_{j}=25^{\circ} \mathrm{C}$ | $\begin{aligned} & V_{R}=5 \mathrm{~V} \\ & V_{R}=10 \mathrm{~V} \\ & V_{R}=25 \mathrm{~V} \\ & V_{R}=30 \mathrm{~V} \end{aligned}$ | - |  | 0.5 | $\mu \mathrm{A}$ |
|  |  |  |  | - |  | 1 |
|  |  |  |  | - | 0.65 | 3 |
|  |  |  |  | $V_{R}=30 \mathrm{~V}$ | - |  | 5 |
|  |  | $\begin{aligned} & T_{j}=70^{\circ} \mathrm{C} \\ & T_{j}=85^{\circ} \mathrm{C} \end{aligned}$ | $\begin{aligned} & V_{R}=10 \mathrm{~V} \\ & \hline \end{aligned}$ | - | 7 | 20 |  |
|  |  |  |  | - | 18 | 50 |  |
| $V_{F}{ }^{(2)}$ | Forward voltage drop | $T_{j}=25^{\circ} \mathrm{C}$ | $\begin{aligned} & I_{F}=0.1 \mathrm{~mA} \\ & I_{F}=1 \mathrm{~mA} \\ & I_{F}=10 \mathrm{~mA} \\ & I_{F}=30 \mathrm{~mA} \end{aligned}$ | - |  | 240 | mV |
|  |  |  |  | - |  | 300 |  |
|  |  |  |  | - |  | 375 |  |
|  |  |  |  | $\mathrm{I}_{\mathrm{F}}=30 \mathrm{~mA}$ | - |  | 430 |  |
|  |  |  |  | $\mathrm{I}_{\mathrm{F}}=100 \mathrm{~mA}$ | - |  | 500 |  |
|  |  |  |  | $\mathrm{I}_{\mathrm{F}}=200 \mathrm{~mA}$ | - |  | 580 |  |
|  |  |  |  | $\mathrm{I}_{\mathrm{F}}=300 \mathrm{~mA}$ | - | 530 |  |

1. Pulse test: $t_{p}=5 \mathrm{~ms}, \delta<2 \%$
2. Pulse test: $t_{p}=380 \mu \mathrm{~s}, \delta<2 \%$

Table 5. Dynamic characteristics

| Symbol | Parameter | Test conditions | Min. | Typ. | Max. | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| C | Diode capacitance | $\mathrm{V}_{\mathrm{R}}=0 \mathrm{~V}, \mathrm{~F}=1 \mathrm{MHz}$ | - | 22 | - | pF |
|  |  | $\mathrm{V}_{\mathrm{R}}=1 \mathrm{~V}, \mathrm{~F}=1 \mathrm{MHz}$ | - | 14 | - |  |
|  |  | $\mathrm{V}_{\mathrm{R}}=10 \mathrm{~V}, \mathrm{~F}=1 \mathrm{MHz}$ | - | 6 | - |  |![img-1.jpeg](img-1.jpeg)

Figure 3. Relative variation of thermal impedance junction to ambient versus pulse duration
![img-2.jpeg](img-2.jpeg)

Figure 5. Leakage current versus reverse applied voltage (typical values)
![img-3.jpeg](img-3.jpeg)

Figure 2. Continuous forward current versus ambiant temperature
![img-4.jpeg](img-4.jpeg)

Figure 4. Relative variation of thermal impedance junction to ambient versus pulse duration
![img-5.jpeg](img-5.jpeg)

Figure 6. Relative variation of reverse leakage current versus junction temperature (typical values)
![img-6.jpeg](img-6.jpeg)Figure 7. Junction capacitance versus reverse applied voltage (typical values)
![img-7.jpeg](img-7.jpeg)

Figure 8. Forward voltage drop versus forward current (typical values)
![img-8.jpeg](img-8.jpeg)

Figure 9. Forward voltage drop versus forward current (typical values)
![img-9.jpeg](img-9.jpeg)# 2 Package information 

- Epoxy meets UL94, V0
- Lead-free packages

In order to meet environmental requirements, ST offers these devices in different grades of ECOPACK ${ }^{\circledR}$ packages, depending on their level of environmental compliance. ECOPACK ${ }^{\circledR}$ specifications, grade definitions and product status are available at: www.st.com.
ECOPACK ${ }^{\circledR}$ is an ST trademark.

### 2.1 SOD-523 package information

Figure 10. SOD-523 package outline
![img-10.jpeg](img-10.jpeg)Table 6. SOD-523 package mechanical data

|  | Dimensions |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Ref. | Millimeters |  |  | Inches ${ }^{(1)}$ |  |  |
|  | Typ. | Min. | Max. | Typ. | Min. | Max. |
| A | 0.60 | 0.50 | 0.70 | 0.024 | 0.020 | 0.028 |
| E | 1.60 | 1.50 | 1.70 | 0.063 | 0.059 | 0.067 |
| E1 | 1.20 | 1.10 | 1.30 | 0.047 | 0.043 | 0.051 |
| D | 0.80 | 0.70 | 0.90 | 0.031 | 0.028 | 0.035 |
| b | - | 0.25 | 0.35 | - | 0.010 | 0.014 |
| c | - | 0.07 | 0.20 | - | 0.003 | 0.008 |
| L | 0.20 | 0.15 | 0.25 | 0.008 | 0.006 | 0.010 |
| L1 | - | 0.05 | 0.20 | - | 0.002 | 0.008 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 11. SOD-523 footprint (dimensions in mm)
![img-11.jpeg](img-11.jpeg)# 2.2 SOT-323 package information 

Figure 12. SOT-323 package outline
![img-12.jpeg](img-12.jpeg)Table 7. SOT-323 package mechanical data

|  | Dimensions |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Ref. | Millimeters |  |  | Inches ${ }^{(1)}$ |  |  |
|  | Typ. | Min. | Max. | Typ. | Min. | Max. |
| A | - | 0.8 | 1.1 | - | 0.031 | 0.043 |
| A1 | - | 0.0 | 0.1 | - | 0.0 | 0.004 |
| b | - | 0.25 | 0.4 | - | 0.010 | 0.016 |
| c | - | 0.1 | 0.26 | - | 0.004 | 0.010 |
| D | 2.0 | 1.8 | 2.2 | 0.079 | 0.071 | 0.086 |
| E | 1.25 | 1.15 | 1.35 | 0.049 | 0.045 | 0.053 |
| e | 0.65 | - | - | 0.026 | - | - |
| H | 2.1 | 1.8 | 2.4 | 0.083 | 0.071 | 0.094 |
| L | 0.2 | 0.1 | 0.3 | 0.008 | 0.004 | 0.012 |
| q | - | 0 | $30^{\circ}$ | - | 0 | $30^{\circ}$ |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 13. SOT-323 footprint (dimensions in mm)
![img-13.jpeg](img-13.jpeg)# 3 Ordering information 

Table 8. Ordering information

| Order code | Marking | Package | Weight | Base qty. | Packing mode |
| :-- | :--: | :--: | :--: | :--: | :--: |
| BAT30CWFILM | C30 | SOT-323 <br> Common cathode | 6 mg | 3000 | Tape and reel |
| BAT30KFILM | 30 | SOD-523 Single | 1.45 mg | 3000 | Tape and reel |
| BAT30SWFILM | S30 | SOT-323 Serial | 6 mg | 3000 | Tape and reel |

Figure 14. Ordering information scheme
![img-14.jpeg](img-14.jpeg)# 4 Revision history 

Table 9. Document revision history

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 24-Jul-2006 | 1 | First issue |
| 08-Jul-2009 | 2 | Added SOD-923 package. Table 12 sorted on alphabetic sequence <br> of order code. Updated ECOPACK statement. |
| 13-Oct-2009 | 3 | Updated Table 6 quote "L1" from 0.10 to 0.05 . |
| 01-Apr-2014 | 4 | Added Pin 1 anode marker to SOT-666 package graphics. Updated <br> Table 2: Absolute ratings (limiting values at $T_{\text {amb }}=25^{\circ} \mathrm{C}$, unless <br> otherwise specified). |
| 01-Apr-2015 | 5 | Package information updated and removed: SOD-323, SOD-923, <br> SOT-23 and SOT666. Updated cover page. <br> Updated Table 2 and Table 3. <br> Updated Figure 14 and Figure 3. <br> Format updated to current standard. |# IMPORTANT NOTICE - PLEASE READ CAREFULLY 

STMicroelectronics NV and its subsidiaries ("ST") reserve the right to make changes, corrections, enhancements, modifications, and improvements to ST products and/or to this document at any time without notice. Purchasers should obtain the latest relevant information on ST products before placing orders. ST products are sold pursuant to ST's terms and conditions of sale in place at the time of order acknowledgement.

Purchasers are solely responsible for the choice, selection, and use of ST products and ST assumes no liability for application assistance or the design of Purchasers' products.

No license, express or implied, to any intellectual property right is granted by ST herein.

Resale of ST products with provisions different from the information set forth herein shall void any warranty granted by ST for such product.

ST and the ST logo are trademarks of ST. All other product or service names are the property of their respective owners.

Information in this document supersedes and replaces information previously supplied in any prior versions of this document.
(c) 2015 STMicroelectronics - All rights reserved