# Low drop power Schottky rectifier in flat package 

## Features

- Very low profile package: 0.85 mm
- Backward compatible with standard STmite footprint
- Very small conduction losses
- Negligible switching losses
- Extremely fast switching
- Low forward voltage drop for higher efficiency and extended battery life
- Low thermal resistance
- Avalanche capability specified


## Description

Single Schottky rectifier suited for switch mode power supplies and high frequency dc to dc converters.

Packaged in STmite flat, this device is intended for use in low voltage, high frequency inverters, free wheeling and polarity protection applications. Due to the very small size of the package this device fits battery powered equipment (cellular, notebook, PDA's, printers) as well as chargers and PCMCIA cards.
![img-0.jpeg](img-0.jpeg)

Table 1. Device summary

| Symbol | Value |
| :--: | :--: |
| $\mathrm{I}_{\mathrm{F}(\mathrm{AV})}$ | 1 A |
| $\mathrm{V}_{\text {RRM }}$ | 20 V |
| $\mathrm{T}_{\mathrm{j}}(\max )$ | $150^{\circ} \mathrm{C}$ |
| $\mathrm{V}_{\mathrm{F}}(\max )$ | 0.37 V |# 1 Characteristics 

Table 2. Absolute ratings (limiting values)

| Symbol | Parameter | Value | Unit |
| :--: | :-- | :-- | :--: |
| $\mathrm{V}_{\text {RRM }}$ | Repetitive peak reverse voltage | 20 | V |
| $\mathrm{I}_{\mathrm{F}(\text { RMS) }}$ | Forward current rms | 2 | A |
| $\mathrm{I}_{\mathrm{F}(\mathrm{AV})}$ | Average forward current | $\mathrm{T}_{\mathrm{C}}=140^{\circ} \mathrm{C} \quad \delta=0.5$ | 1 | A |
| $\mathrm{I}_{\text {FSM }}$ | Surge non repetitive forward current | $t_{p}=10 \mathrm{~ms}$ sinusoidal | 50 | A |
| $\mathrm{P}_{\text {ARM }}$ | Repetitive peak avalanche power | $t_{p}=1 \mu \mathrm{~s} \quad \mathrm{~T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ | 1400 | W |
| $\mathrm{T}_{\text {stg }}$ | Storage temperature range | -65 to +150 | ${ }^{\circ} \mathrm{C}$ |
| $T_{j}$ | Maximum operating junction temperature ${ }^{(1)}$ | 150 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{dV} / \mathrm{dt}$ | Critical rate of rise of reverse voltage (rated $\mathrm{V}_{\mathrm{R}}, \mathrm{T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ ) | 10000 | $\mathrm{V} / \mu \mathrm{s}$ |

1. $\frac{\mathrm{dP}(\mathrm{ol}}{\mathrm{dT})}<\frac{1}{\mathrm{R}(\mathrm{th}(j-a)}$ condition to avoid thermal runaway for a diode on its own heatsink

Table 3. Thermal resistance

| Symbol | Parameter | Value | Unit |
| :--: | :-- | :-- | :--: |
| $\mathrm{R}_{\text {(hij-c) }}$ | Junction to case | 20 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{R}_{\text {(hij-a) }}{ }^{(1)}$ | Junction to ambient | 250 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |

1. Mounted with minimum recommended pad size, PC board FR4

Table 4. Static electrical characteristics

| Symbol | Parameter | Tests conditions |  | Min. | Typ. | Max. | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $I_{R}{ }^{(1)}$ | Reverse leakage current | $\mathrm{T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ | $V_{R}=V_{R R M}$ |  | 0.015 | 0.075 | mA |
|  |  | $T_{j}=85^{\circ} \mathrm{C}$ |  |  | 0.90 | 4.50 |  |
|  |  | $\mathrm{T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ | $V_{R}=10 \mathrm{~V}$ |  | 0.005 | 0.035 |  |
|  |  | $T_{j}=85^{\circ} \mathrm{C}$ |  |  | 0.45 | 2.50 |  |
|  |  | $\mathrm{T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ | $V_{R}=5 \mathrm{~V}$ |  | 0.003 | 0.025 |  |
|  |  | $T_{j}=85^{\circ} \mathrm{C}$ |  |  | 0.30 | 1.60 |  |
| $V_{F}{ }^{(1)}$ | Forward voltage drop | $\mathrm{T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ | $I_{F}=1 \mathrm{~A}$ |  | 0.38 | 0.43 | V |
|  |  | $T_{j}=85^{\circ} \mathrm{C}$ |  |  | 0.32 | 0.37 |  |
|  |  | $\mathrm{T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ | $I_{F}=2 \mathrm{~A}$ |  | 0.42 | 0.47 |  |
|  |  | $T_{j}=85^{\circ} \mathrm{C}$ |  |  | 0.37 | 0.42 |  |
|  |  | $\mathrm{T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ | $I_{F}=3 \mathrm{~A}$ |  | 0.46 | 0.53 |  |
|  |  | $T_{j}=85^{\circ} \mathrm{C}$ |  |  | 0.42 | 0.49 |  |
|  |  | $\mathrm{T}_{\mathrm{j}}=25^{\circ} \mathrm{C}$ | $I_{F}=4 \mathrm{~A}$ |  | 0.50 | 0.60 |  |
|  |  | $T_{j}=85^{\circ} \mathrm{C}$ |  |  | 0.46 | 0.56 |  |

1. Pulse test: $=380 \mu \mathrm{~s}, \delta<2 \%$

To evaluate the conduction losses use the following equation:
$P=0.32 \times I_{F(A V)}+0.05 I_{F}{ }^{2}{ }_{(R M S)}$Figure 1. Conduction losses versus average current
![img-1.jpeg](img-1.jpeg)

Figure 3. Normalized avalanche power derating versus pulse duration
![img-2.jpeg](img-2.jpeg)

Figure 5. Non repetitive surge peak forward current versus overload duration (maximum values)
![img-3.jpeg](img-3.jpeg)

Figure 2. Average forward current versus ambient temperature ( $\delta=0.5$ )
![img-4.jpeg](img-4.jpeg)

Figure 4. Normalized avalanche power derating versus junction temperature
![img-5.jpeg](img-5.jpeg)

Figure 6. Relative variation of thermal impedance junction to case versus pulse duration
![img-6.jpeg](img-6.jpeg)Figure 7. Reverse leakage currrent versus reverse voltage applied (typical values)
![img-7.jpeg](img-7.jpeg)

Figure 9. Junction capacitance versus reverse voltage applied (typical values)
![img-8.jpeg](img-8.jpeg)

Figure 10. Forward voltage drop versus forward current
![img-9.jpeg](img-9.jpeg)

Figure 11. Thermal resistance junction to ambient versus copper surface under tab (epoxy printed board FR4, copper thickness $=35 \mu \mathrm{~m}$, typical values)
![img-10.jpeg](img-10.jpeg)# 2 Package information 

- Epoxy meets UL94, V0
- Lead-free packages

In order to meet environmental requirements, ST offers these devices in different grades of ECOPACK ${ }^{\circledR}$ packages, depending on their level of environmental compliance. ECOPACK ${ }^{\circledR}$ specifications, grade definitions and product status are available at: www.st.com.
ECOPACK ${ }^{\circledR}$ is an ST trademark.
Table 5. STmite flat dimensions
![img-11.jpeg](img-11.jpeg)

Figure 12. STmite flat recommended footprint (all dimensions in mm)
![img-12.jpeg](img-12.jpeg)# 3 Ordering information 

Table 6. Ordering information

| Order code | Marking | Package | Weight | Base qty | Delivery mode |
| :--: | :--: | :--: | :--: | :--: | :--: |
| STPS1L20MF | F1L2 | STmite flat | 16 mg | 12000 | Tape and reel |

## 4 Revision history

Table 7. Document revision history

| Date | Revision | Changes |
| :--: | :--: | :-- |
| 21-Aug-2006 | 1 | First issue. |
| 07-Jul-2011 | 2 | Reformatted to current standards. Updated caption for Figure 6. |Please Read Carefully:

Information in this document is provided solely in connection with ST products. STMicroelectronics NV and its subsidiaries ("ST") reserve the right to make changes, corrections, modifications or improvements, to this document, and the products and services described herein at any time, without notice.

All ST products are sold pursuant to ST's terms and conditions of sale.
Purchasers are solely responsible for the choice, selection and use of the ST products and services described herein, and ST assumes no liability whatsoever relating to the choice, selection or use of the ST products and services described herein.

No license, express or implied, by estoppel or otherwise, to any intellectual property rights is granted under this document. If any part of this document refers to any third party products or services it shall not be deemed a license grant by ST for the use of such third party products or services, or any intellectual property contained therein or considered as a warranty covering the use in any manner whatsoever of such third party products or services or any intellectual property contained therein.

UNLESS OTHERWISE SET FORTH IN ST'S TERMS AND CONDITIONS OF SALE ST DISCLAIMS ANY EXPRESS OR IMPLIED WARRANTY WITH RESPECT TO THE USE AND/OR SALE OF ST PRODUCTS INCLUDING WITHOUT LIMITATION IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE (AND THEIR EQUIVALENTS UNDER THE LAWS OF ANY JURISDICTION), OR INFRINGEMENT OF ANY PATENT, COPYRIGHT OR OTHER INTELLECTUAL PROPERTY RIGHT.

UNLESS EXPRESSLY APPROVED IN WRITING BY AN AUTHORIZED ST REPRESENTATIVE, ST PRODUCTS ARE NOT RECOMMENDED, AUTHORIZED OR WARRANTED FOR USE IN MILITARY, AIR CRAFT, SPACE, LIFE SAVING, OR LIFE SUSTAINING APPLICATIONS, NOR IN PRODUCTS OR SYSTEMS WHERE FAILURE OR MALFUNCTION MAY RESULT IN PERSONAL INJURY, DEATH, OR SEVERE PROPERTY OR ENVIRONMENTAL DAMAGE. ST PRODUCTS WHICH ARE NOT SPECIFIED AS "AUTOMOTIVE GRADE" MAY ONLY BE USED IN AUTOMOTIVE APPLICATIONS AT USER'S OWN RISK.

Resale of ST products with provisions different from the statements and/or technical features set forth in this document shall immediately void any warranty granted by ST for the ST product or service described herein and shall not create or extend in any manner whatsoever, any liability of ST.

ST and the ST logo are trademarks or registered trademarks of ST in various countries.
Information in this document supersedes and replaces all information previously supplied.
The ST logo is a registered trademark of STMicroelectronics. All other names are the property of their respective owners.
© 2011 STMicroelectronics - All rights reserved

STMicroelectronics group of companies
Australia - Belgium - Brazil - Canada - China - Czech Republic - Finland - France - Germany - Hong Kong - India - Israel - Italy - Japan Malaysia - Malta - Morocco - Philippines - Singapore - Spain - Sweden - Switzerland - United Kingdom - United States of America
www.st.com