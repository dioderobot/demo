# 57 <br> STMPS2141, STMPS2151, STMPS2161, STMPS2171 

## Enhanced single channel power switches

## Features

■ $90 \mathrm{~m} \Omega$ high-side MOSFET switch
■ 500/1000 mA continuous current

- Thermal and short-circuit protection with overcurrent logic output
- Operating range from 2.7 to 5.5 V
- CMOS and TTL compatible enable input
- Undervoltage lockout (UVLO)
- $12 \mu \mathrm{~A}$ maximum standby supply current
- Ambient temperature range, -40 to $85^{\circ} \mathrm{C}$
- 8 kV ESD protection
- Reverse current protection
- Fault blanking
- UL recognized components (UL file number: E354278)


## Description

The STMPS2141, STMPS2151, STMPS2161, STMPS2171 power distribution switches are intended for applications where heavy capacitive loads and short-circuits are likely to be encountered. These devices incorporate $90 \mathrm{~m} \Omega$ N -channel MOSFET high-side power switches for power distribution. These switches are controlled by a logic enable input.
![img-0.jpeg](img-0.jpeg)

When the output load exceeds the current limit threshold or a short is present, the device limits the output current to a safe level by switching into a constant current mode. When continuous heavy overloads and short-circuits increase the power dissipation in the switch, causing the junction temperature to rise, a thermal protection circuit shuts the switch off to prevent damage. Recovery from a thermal shutdown is automatic once the device has cooled sufficiently. Internal circuitry ensures the switch remains off until a valid input voltage is present.

Table 1. Device summary

| Order codes |  |  | Rated continuous output current <br> $(\mathrm{mA})$ | Enable |
| :--: | :--: | :--: | :--: | :--: |
| SO-8 | SOT23-5L | MSOP8(1) |  |  |
| STMPS2141MTR | STMPS2141STR | STMPS2141TTR | 500 | Active low |
| STMPS2151MTR | STMPS2151STR | STMPS2151TTR | 500 | Active high |
| STMPS2161MTR | STMPS2161STR | STMPS2161TTR | 1000 | Active low |
| STMPS2171MTR | STMPS2171STR | STMPS2171TTR | 1000 | Active high |

1. MSOP8 package is also known as "TSSOP8".# Contents 

1 Block diagram ..... 7
2 Pin settings ..... 8
2.1 Pin connections ..... 8
2.2 Pin description ..... 8
3 Functional description ..... 9
3.1 Fault blanking ..... 9
3.2 Overcurrent/overtemperature protection ..... 9
3.3 Fault conditions ..... 9
3.4 Reversed current blocking ..... 10
3.5 UVLO ..... 10
4 Ambient temperature ..... 11
5 Maximum ratings ..... 12
5.1 Absolute maximum ratings ..... 12
5.2 Recommended operating conditions ..... 12
6 Electrical specifications ..... 13
7 Detail device characteristics ..... 16
7.1 STMPS2141, STMPS2151 additional electrical charts ..... 16
7.1.1 Turn-on/off characteristics at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ ..... 16
7.1.2 Turn-on/off characteristics at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ ..... 17
7.1.3 UVLO ..... 18
7.1.4 OC protection characteristics ..... 18
7.1.5 Other electrical characteristics ..... 19
7.2 STMPS2161, STMPS2171 electrical charts ..... 22
7.2.1 Turn-on/off characteristics at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ ..... 22
7.2.2 Turn-on/off characteristics at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ ..... 23
7.2.3 UVLO ..... 24
7.2.4 OC protection characteristics ..... 24
7.2.5 Other electrical characteristics ..... 258 Package mechanical data ..... 28
9 Ordering information ..... 35
10 Revision history ..... 36# List of tables 

Table 1. Device summary ..... 1
Table 2. Pin description ..... 8
Table 3. Fault conditions ..... 9
Table 4. $\quad$ SOT23-5L $\left(191^{\circ} \mathrm{C} / \mathrm{W}\right)$ ..... 11
Table 5. MSOP8 $\left(220^{\circ} \mathrm{C} / \mathrm{W}\right)$ ..... 11
Table 6. SO-8 $\left(160^{\circ} \mathrm{C} / \mathrm{W}\right)$ ..... 11
Table 7. Absolute maximum ratings ..... 12
Table 8. Recommended operating conditions ..... 12
Table 9. SOT-23-5L electrical characteristics ..... 13
Table 10. MSO8P/SO-8 electrical characteristics ..... 13
Table 11. Current limit characteristics $\left(V_{I N}=5.5 \mathrm{~V}, I_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified) ..... 14
Table 12. Supply current characteristics $\left(V_{\text {IN }}=5.5 \mathrm{~V}, \mathrm{I}_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified) ..... 14
Table 13. Thermal characteristics $\left(V_{\text {IN }}=5.5 \mathrm{~V}, \mathrm{I}_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified) ..... 15
Table 14. UVLO characteristics $\left(V_{\text {IN }}=5.5 \mathrm{~V}, \mathrm{I}_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified) ..... 15
Table 15. FAULT pin characteristics $\left(V_{\text {IN }}=5.5 \mathrm{~V}, I_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified) ..... 15
Table 16. EN pin characteristics $\left(V_{\text {IN }}=5.5 \mathrm{~V}, I_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified) ..... 15
Table 17. SOT23-5L package mechanical data ..... 28
Table 18. SOT23-5L footprint dimensions ..... 29
Table 19. SO-8 mechanical data ..... 30
Table 20. MSOP8 package mechanical data ..... 32
Table 21. Reel mechanical data ..... 34
Table 22. Order codes ..... 35
Table 23. Document revision history ..... 36# List of figures 

Figure 1. Block diagram ..... 7
Figure 2. SOT23-5L, SO-8 and MSOP8 pin connections ..... 8
Figure 3. Voltage output turn-on delay time (STMPS2141/2151, 5 V) ..... 16
Figure 4. Voltage output turn-off delay time (STMPS2141/2151, 5 V) ..... 16
Figure 5. Current output turn-on delay time (STMPS2141/2151, 5 V) ..... 16
Figure 6. Current output turn-off delay time (STMPS2141/2151, 5 V) ..... 16
Figure 7. Voltage output turn-on delay time (STMPS2141/2151, 3 V) ..... 17
Figure 8. Voltage output turn-off delay time (STMPS2141/2151, 3 V) ..... 17
Figure 9. Current output turn-on delay time (STMPS2141/2151, 3 V) ..... 17
Figure 10. Current output turn-off delay time (STMPS2141/2151, 3 V) ..... 17
Figure 11. UVLO rising (STMPS2141/2151) ..... 18
Figure 12. UVLO falling (STMPS2141/2151) ..... 18
Figure 13. OC protection at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ (STMPS2141/2151) ..... 18
Figure 14. OC protection at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ (STMPS2141/2151 - detail) ..... 18
Figure 17. $\mathrm{I}_{\mathrm{CC}}$ vs. $\mathrm{V}_{\mathrm{IN}}$ (enabled) (STMPS2141/2151) ..... 19
Figure 18. $\mathrm{I}_{\mathrm{CC}}$ vs. temperature (enabled) (STMPS2141/2151) ..... 19
Figure 15. OC protection at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ (STMPS2141/2151) ..... 19
Figure 16. OC protection at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ (STMPS2141/2151 - detail) ..... 19
Figure 19. $\mathrm{I}_{\mathrm{CC}}$ vs. $\mathrm{V}_{\mathrm{IN}}$ (disabled) (STMPS2141/2151) ..... 19
Figure 20. $\mathrm{I}_{\mathrm{CC}}$ vs. temperature (disabled) (STMPS2141/2151) ..... 19
Figure 21. $R_{\text {ON }}$ vs. $V_{\text {IN }}$ (STMPS2141/2151) ..... 20
Figure 22. $R_{\text {ON }}$ vs. temperature (STMPS2141/2151) ..... 20
Figure 23. $\mathrm{I}_{\text {OS }}$ vs. temperature (STMPS2141/2151) ..... 20
Figure 24. Switch leakage vs. temperature (STMPS2141/2151) ..... 20
Figure 25. Output rise time vs. $\mathrm{V}_{\text {IN }}$ (STMPS2141/2151) ..... 20
Figure 26. Output fall time vs. $\mathrm{V}_{\mathrm{IN}}$ (STMPS2141/2151) ..... 20
Figure 27. UVLO vs. temperature (STMPS2141/2151) ..... 21
Figure 28. Voltage output turn-on delay time (STMPS2161/2171, 5 V) ..... 22
Figure 29. Voltage output turn-off delay time (STMPS2161/2171, 5 V) ..... 22
Figure 30. Current output turn-on delay time (STMPS2161/2171, 5 V) ..... 22
Figure 31. Current output turn-off delay time (STMPS2161/2171, 5 V) ..... 22
Figure 32. Voltage output turn-on delay time (STMPS2161/2171, 3 V) ..... 23
Figure 33. Voltage output turn-off delay time (STMPS2161/2171, 3 V) ..... 23
Figure 34. Current output turn-on delay time (STMPS2161/2171, 3 V) ..... 23
Figure 35. Current output turn-off delay time (STMPS2161/2171, 3 V) ..... 23
Figure 36. UVLO rising (STMPS2161/2171) ..... 24
Figure 37. UVLO falling (STMPS2161/2171) ..... 24
Figure 38. OC protection at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ (STMPS2161/2171) ..... 24
Figure 39. OC protection at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ (STMPS2161/2171- detail) ..... 24
Figure 42. $\mathrm{I}_{\mathrm{CC}}$ vs. $\mathrm{V}_{\mathrm{IN}}$ (enabled) (STMPS2161/2171) ..... 25
Figure 43. $\mathrm{I}_{\mathrm{CC}}$ vs. temperature (enabled) (STMPS2161/2171) ..... 25
Figure 40. OC protection at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ (STMPS2161/2171) ..... 25
Figure 41. OC protection at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ (STMPS2161/2171- detail) ..... 25
Figure 44. $\mathrm{I}_{\mathrm{CC}}$ vs. $\mathrm{V}_{\mathrm{IN}}$ (disabled) (STMPS2161/2171) ..... 25
Figure 45. $\mathrm{I}_{\mathrm{CC}}$ vs. temperature (disabled) (STMPS2161/2171) ..... 25
Figure 46. $R_{\text {ON }}$ vs. $V_{\text {IN }}$ (STMPS2161/2171) ..... 26
Figure 47. $R_{\text {ON }}$ vs. temperature (STMPS2161/2171) ..... 26
Figure 48. $\mathrm{I}_{\mathrm{OS}}$ vs. temperature (STMPS2161/2171) ..... 26Figure 49. Switch leakage vs. temperature (STMPS2161/2171) ..... 26
Figure 50. Output rise time vs. $\mathrm{V}_{\mathrm{IN}}$ (STMPS2161/2171) ..... 26
Figure 51. Output fall time vs. $\mathrm{V}_{\mathrm{IN}}$ (STMPS2161/2171) ..... 26
Figure 52. UVLO vs. temperature (STMPS2161/2171) ..... 27
Figure 53. SOT23-5L package outline ..... 28
Figure 54. SOT23-5L footprint recommendations ..... 29
Figure 55. SOT23-5L carrier tape ..... 29
Figure 56. SO-8 package outline ..... 30
Figure 57. SO-8 carrier tape ..... 31
Figure 58. MSOP8 package outline ..... 32
Figure 59. MSOP8 carrier tape ..... 33
Figure 60. Reel information ..... 34# 1 Block diagram 

Figure 1. Block diagram
![img-1.jpeg](img-1.jpeg)# 2 Pin settings 

### 2.1 Pin connections

Figure 2. SOT23-5L, SO-8 and MSOP8 pin connections
![img-2.jpeg](img-2.jpeg)

### 2.2 Pin description

Table 2. Pin description

| Pin number |  |  | Name | Function |
| :--: | :--: | :--: | :--: | :--: |
| SO-8 | MSOP8 | SOT23-5L |  |  |
| 1 | 1 | 2 | GND | Ground |
| 2 | 2 | 5 | IN | $2.7-5.5 \mathrm{~V}$ input |
| 3 | 3 | - | IN | $2.7-5.5 \mathrm{~V}$ input |
| 4 | 4 | 4 | EN | Enable for power switch |
| 5 | 5 | 3 | FAULT | Open drain FAULT indicator, active low |
| 6 | 6 | 1 | OUT | Output of power switch |
| 7 | 7 | - | OUT | Output of power switch |
| 8 | 8 | - | OUT | Output of power switch |# 3 Functional description 

### 3.1 Fault blanking

The STMPS devices feature a 10 ms fault blanking. Fault blanking allows current limit faults, including momentary short-circuit faults that occur when hot-swapping a capacitive load, and also ensures that no fault is issued during power-up. When a load transient causes the device to enter current limit, an internal counter starts. If the load fault persists beyond the 10 ms fault blanking timeout, the FAULT output asserts "low". Load transient faults less than 10 ms (typ.) do not cause a FAULT output assertion. Only current limit faults are blanked. Die overtemperature faults and input voltage drops below the UVLO threshold cause an immediate fault output.

### 3.2 Overcurrent/overtemperature protection

In overcurrent or short-circuit condition, the switch limits the current at a value of about $120 \%$ of the rated current. If the temperature of the die goes above the limit value, the switch turns off.

### 3.3 Fault conditions

In power switch applications, 4 types of fault conditions are common. These fault conditions and the response of the STMPS21x1 power switches are described in Table 3.

Table 3. Fault conditions

| Fault | Condition | STMPS21x1 action |
| :-- | :-- | :-- |
| Short-circuit | Output shorted to GND via resistance <br> path of < 1 $\Omega$ causing a rapid current <br> surge. | Reduces output voltage to reduce the <br> current. Asserts FAULT pin after <br> a blanking period |
| Overcurrent | Output connected to a load that sinks <br> current above threshold. | Reduces output voltage to reduce the <br> current. Asserts FAULT pin after <br> a blanking period. |
| Overheating | Temperature of junction exceeds 135 ${ }^{\circ} \mathrm{C}$ <br> due to any reason. | Turn OFF output until temperature falls <br> below 125 ${ }^{\circ} \mathrm{C}$. Asserts FAULT pin <br> immediately. |
| Undervoltage | Input voltage drops below the UVLO <br> threshold. | Turn OFF output until input voltage rises <br> above the UVLO threshold plus <br> hysteresis. Asserts FAULT pin <br> immediately. |# 3.4 Reversed current blocking 

When the switch is OFF (disabled through the EN pin), or when the STMPS device is unpowered $\left(V_{I N}=0 \mathrm{~V}\right)$ the switch behaves as an Hi-Z at the output pin, ensuring that no reverse current will flow into the device when $V_{I N}<V_{\text {OUT }}$.
Note: In the case where the switch is ON, and a voltage higher than $V_{I N}$ is applied to the OUT pin, a reverse current occurs. This operating condition is not allowed.

### 3.5 UVLO

When the input voltage drops below critical values, the power switch turns off to prevent improper operation due to low voltage.# 4 Ambient temperature 

In "Enable" operating mode, an amount of power is dissipated as heat in the power switch due to the on-resistance. The power dissipation is: $P=I^{2} R$.

Table 4. $\quad$ SOT23-5L $\left(191^{\circ} \mathrm{C} / \mathrm{W}\right)$

| Part number | Max. current | Max. $\mathrm{R}_{\mathrm{ON}}$ <br> at 5 V | Power <br> dissipation | Temperature <br> difference <br> (junction - ambient) | Maximum ambient <br> temperature <br> (at junction <br> temperature $\left.125^{\circ} \mathrm{C}\right)$ |
| :-- | :--: | :--: | :--: | :--: | :--: |
| STMPS2141 | 0.50 A | $135 \mathrm{~m} \Omega$ | 33.8 mW | 6.5 | 118.5 |
| STMPS2151 |  |  |  |  |  |
| STMPS2161 | 1.00 A | $135 \mathrm{~m} \Omega$ | 135.0 mW | 25.8 | 99.2 |
| STMPS2171 |  |  |  |  |  |

Table 5. MSOP8 ( $\left.220^{\circ} \mathrm{C} / \mathrm{W}\right)$

| Part number | Max. current | Max. $\mathrm{R}_{\mathrm{ON}}$ <br> at 5 V | Power <br> dissipation | Temperature <br> difference <br> (junction - ambient) | Maximum ambient <br> temperature <br> (at junction <br> temperature $\left.125^{\circ} \mathrm{C}\right)$ |
| :-- | :--: | :--: | :--: | :--: | :--: |
| STMPS2141 | 0.50 A | $140 \mathrm{~m} \Omega$ | 35.0 mW | 7.7 | 117.3 |
| STMPS2151 |  |  |  |  |  |
| STMPS2161 | 1.00 A | $140 \mathrm{~m} \Omega$ | 140.0 mW | 30.8 | 94.2 |
| STMPS2171 |  |  |  |  |  |

Table 6. SO-8 ( $\left.160^{\circ} \mathrm{C} / \mathrm{W}\right)$

| Part number | Max. current | Max. $\mathrm{R}_{\mathrm{ON}}$ <br> at 5 V | Power <br> dissipation | Temperature <br> difference <br> (junction - ambient) | Maximum ambient <br> temperature <br> (at junction <br> temperature $\left.125^{\circ} \mathrm{C}\right)$ |
| :-- | :--: | :--: | :--: | :--: | :--: |
| STMPS2141 | 0.50 A | $140 \mathrm{~m} \Omega$ | 35.0 mW | 5.6 | 119.4 |
| STMPS2151 |  |  |  |  |  |
| STMPS2161 | 1.00 A | $140 \mathrm{~m} \Omega$ | 140.0 mW | 22.4 | 102.6 |
| STMPS2171 |  |  |  |  |  |# 5 Maximum ratings 

Stressing the device above the rating listed in Table 7: Absolute maximum ratings may cause permanent damage to the device. These are stress ratings only and operation of the device at these or any other conditions above those indicated in Section 5.2: Recommended operating conditions of this specification is not implied. Exposure to absolute maximum rating conditions for extended periods may affect device reliability.

### 5.1 Absolute maximum ratings

Table 7. Absolute maximum ratings

| Symbol | Parameter | Value | Unit |
| :--: | :-- | :--: | :--: |
| $\mathrm{V}_{\text {IN }}$ | Input voltage range | $-0.3-6.0$ | V |
| $\mathrm{V}_{\text {OUT }}$ | Output voltage range | $-0.3-\left(\mathrm{V}_{\mathrm{IN}}+0.3\right)$ | V |
| $\mathrm{V}_{\text {IENX }}$ | EN Input voltage range | $-0.3-6.0$ | V |
| $\mathrm{I}_{\text {OUT }}$ | Continuous output current | Internally limited | - |
| ESD | ESD protection level | 8 | kV |
| $\mathrm{T}_{\mathrm{J}}$ | Junction operating temperature | -40 to 125 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\text {STG }}$ | Storage temperature | -55 to 150 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\mathrm{R}}$ | Thermal resistance (MSOP8) | 220 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{T}_{\mathrm{R}}$ | Thermal resistance (SOT23-5L) | 191 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
| $\mathrm{T}_{\mathrm{R}}$ | Thermal resistance (SO-8) | 160 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |

### 5.2 Recommended operating conditions

Table 8. Recommended operating conditions

| Symbol | Parameter | Value |  |  | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: |
|  |  | Min. | Typ. | Max. |  |
| $\mathrm{V}_{\text {IN }}$ | Input voltage | 2.7 | 5.0 | 5.5 | V |
| $\mathrm{V}_{\text {OUT }}$ | Output voltage | 0 | 5.0 | 5.5 | V |
| $\begin{aligned} & \mathrm{I}_{\text {OUT }} \\ & \text { (STMPS2141 } \\ & \text { (STMPS2151) } \end{aligned}$ | Continuous output current | 0 | - | 500 | mA |
| $\begin{aligned} & \mathrm{I}_{\text {OUT }} \\ & \text { (STMPS2161 } \\ & \text { (STMPS2171) } \end{aligned}$ | Continuous output current | 0 | - | 1000 | mA |# 6 Electrical specifications 

Table 9. SOT-23-5L electrical characteristics

| Symbol | Parameter | Test condition | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min. | Typ. | Max. |  |
| $R_{\text {ON }}$ | Static drain source ON state resistance SOT23-5L package load $=500 \mathrm{~mA}$ (STMPS2141/ <br> STMPS2151) load $=1000 \mathrm{~mA}$ (STMPS2161/ STMPS2171) | $\mathrm{V}_{\mathrm{IN}}=2.7 \mathrm{~V} ; \mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$; | $-$ | 120 | 160 | $\mathrm{m} \Omega$ |
|  |  | $\mathrm{V}_{\mathrm{IN}}=5.0 \mathrm{~V} ; \mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$; | $-$ | 90 | 110 | $\mathrm{m} \Omega$ |
| $R_{\text {ON }}$ | Static drain source ON state resistance | $\begin{aligned} & \mathrm{V}_{\mathrm{IN}}=2.7 \mathrm{~V} ; \\ & -40<\mathrm{T}_{\mathrm{J}}<125^{\circ} \mathrm{C} \end{aligned}$ | $-$ | $-$ | 200 | $\mathrm{m} \Omega$ |
|  |  | $\begin{aligned} & \mathrm{V}_{\mathrm{IN}}=5.0 \mathrm{~V} ; \\ & -40<\mathrm{T}_{\mathrm{J}}<125^{\circ} \mathrm{C} \end{aligned}$ | $-$ | $-$ | 135 |  |
| $t_{r}$ | Output rise time | $\begin{aligned} & \mathrm{V}_{\mathrm{IN}}=5.0 \mathrm{~V} \mathrm{R}_{\mathrm{L}}=10 \Omega \\ & \mathrm{C}_{\mathrm{L}}=1 \mu \mathrm{~F} \end{aligned}$ | 0.05 | $-$ | 2 | ms |

Table 10. MSO8P/SO-8 electrical characteristics

| Symbol | Parameter | Test condition | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min. | Typ. | Max. |  |
| $R_{\text {ON }}$ | Static drain source ON state resistance SO-8 and MSO8 package load $=500 \mathrm{~mA}$ (STMPS2141/ <br> STMPS2151) load $=1000 \mathrm{~mA}$ (STMPS2161/ STMPS2171) | $\mathrm{V}_{\mathrm{IN}}=2.7 \mathrm{~V} ; \mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$ | $-$ | 130 | 170 | $\mathrm{m} \Omega$ |
|  |  | $\mathrm{V}_{\mathrm{IN}}=5.0 \mathrm{~V} ; \mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$ | $-$ | 110 | 125 | $\mathrm{m} \Omega$ |
| $R_{\text {ON }}$ | Static drain source ON state resistance | $\begin{aligned} & \mathrm{V}_{\mathrm{IN}}=2.7 \mathrm{~V} \\ & -40<\mathrm{T}_{\mathrm{J}}<125^{\circ} \mathrm{C} \end{aligned}$ | $-$ | $-$ | 200 | $\mathrm{m} \Omega$ |
|  |  | $\begin{aligned} & \mathrm{V}_{\mathrm{IN}}=5.0 \mathrm{~V} \\ & -40<\mathrm{T}_{\mathrm{J}}<125^{\circ} \mathrm{C} \end{aligned}$ | $-$ | $-$ | 140 |  |
| $t_{r}$ | Output rise time | $\begin{aligned} & \mathrm{V}_{\mathrm{IN}}=5.0 \mathrm{~V} \mathrm{R}_{\mathrm{L}}=10 \Omega \\ & \mathrm{C}_{\mathrm{L}}=1 \mu \mathrm{~F} \end{aligned}$ | 0.05 | $-$ | 2 | ms |Table 11. Current limit characteristics $\left(V_{I N}=5.5 \mathrm{~V}, I_{\text {OUT }}=\right.$ rated current, $T_{J}=25^{\circ} \mathrm{C}$, unless otherwise specified $)$

| Symbol | Parameter | Test condition | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min. | Typ. | Max. |  |
| $\begin{aligned} & \mathrm{I}_{\text {OS }} \\ & \text { (STMPS2141 } \\ & \text { STMPS2151) } \end{aligned}$ | Overcurrent limiting threshold | $\begin{aligned} & V_{\text {IN }}=5.5 \mathrm{~V} \\ & V_{\text {OUT }}=5.0 \mathrm{~V} \end{aligned}$ | 0.60 | 0.80 | 1.00 | A |
| $\begin{aligned} & \mathrm{I}_{\text {OS }} \\ & \text { (STMPS2161 } \\ & \text { STMPS2171) } \end{aligned}$ | Overcurrent limiting threshold |  | 1.10 | 1.50 | 1.90 | A |
| $\begin{gathered} \mathrm{I}_{\mathrm{OS}} \\ (2141,2151) \end{gathered}$ | Short-circuit output current | $\mathrm{V}_{\text {IN }}=5.5 \mathrm{~V}$, OUT connected to GND, device enabled into short-circuit | - | - | 0.9 | A |
| $\begin{gathered} \mathrm{I}_{\mathrm{OS}} \\ (2161,2171) \end{gathered}$ | Short-circuit output current |  | - | - | 1.8 | A |

Table 12. Supply current characteristics $\left(V_{I N}=5.5 \mathrm{~V}, \mathrm{I}_{\text {OUT }}=\right.$ rated current, $T_{J}=25^{\circ} \mathrm{C}$, unless otherwise specified)

| Symbol | Parameter | Test condition | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min. | Typ. | Max. |  |
| $\mathrm{I}_{\text {OFF }}$ | Switch turned off | No load | - | 6.0 | 12 | $\mu \mathrm{A}$ |
|  |  | No load; $-40<T_{J}<125^{\circ} \mathrm{C}$ | - | - | 15 |  |
| $\mathrm{I}_{\text {ON }}$ | Switch turned on | No load | - | 40 | 60 | $\mu \mathrm{A}$ |
|  |  | No load; $-40<T_{J}<125^{\circ} \mathrm{C}$ | - | - | 70 |  |
| $\mathrm{I}_{\text {leakage }}$ | Output leakage current ${ }^{(1)}$ | Output grounded, switch is OFF | - | - | 2 | $\mu \mathrm{A}$ |
|  |  | Output grounded, switch is OFF; $-40<T_{J}<125^{\circ} \mathrm{C}$ | - | - | 5 |  |
| $\mathrm{I}_{\text {reverse }}$ | Reversed leakage current | Switch is off, $\mathrm{V}_{\text {IN }}<\mathrm{V}_{\text {OUT }}$, output connected to $5.5 \mathrm{~V}, 25^{\circ} \mathrm{C}$ | - | 0.5 | 2 | $\mu \mathrm{A}$ |
|  |  | Switch is off, $\mathrm{V}_{\text {IN }}<\mathrm{V}_{\text {OUT }}$, output connected to $5.5 \mathrm{~V}, 125^{\circ} \mathrm{C}$ | - | 0.5 | 3 |  |

1. $\mathrm{I}_{\text {leakage }}=\mathrm{I}_{\text {OFF-ground }}-\mathrm{I}_{\text {OFF }}$, where $\mathrm{I}_{\text {OFF-ground }}=$ current into $\mathrm{V}_{\text {IN }}$ when switch is off and output is grounded.Table 13. Thermal characteristics $\left(V_{I N}=5.5 \mathrm{~V}, \mathrm{I}_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified $)$

| Symbol | Parameter | Test condition | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min. | Typ. | Max. |  |
| T1 | Thermal shutdown threshold |  | - | - | 145 | ${ }^{\circ} \mathrm{C}$ |
| T2 | Recovery from thermal shutdown |  | 120 | - | - | ${ }^{\circ} \mathrm{C}$ |
| Hysteresis | - |  | - | 14 | - | ${ }^{\circ} \mathrm{C}$ |

Table 14. UVLO characteristics
$\left(V_{I N}=5.5 \mathrm{~V}, \mathrm{I}_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified $)$

| Symbol | Parameter | Test condition | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min. | Typ. | Max. |  |
| $\mathrm{V}_{\text {UVLO }}$ | Undervoltage lockout threshold |  | 2.0 | - | 2.5 | V |
| Hysteresis | - |  | 40 | 75 | 110 | mV |

Table 15. FAULT pin characteristics
$\left(V_{I N}=5.5 \mathrm{~V}, \mathrm{I}_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified $)$

| Symbol | Parameter | Test condition | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min. | Typ. | Max. |  |
| OC blanking | FAULT assertion and deassertion |  | 4 | 8 | 15 | ms |
| $\mathrm{V}_{\text {OUT }}$ | Output low voltage | $\mathrm{I}_{\text {OUT }}=5 \mathrm{~mA}$ | - | - | 0.4 | V |
| $\mathrm{I}_{\text {OFF }}$ | Off current | $\begin{aligned} & \mathrm{V}_{\text {FAULT }}=2.7 \mathrm{~V}, 5.5 \mathrm{~V} \\ & \text { (no OC condition) } \end{aligned}$ | - | - | 1.0 | $\mu \mathrm{A}$ |

Table 16. EN pin characteristics
$\left(V_{I N}=5.5 \mathrm{~V}, \mathrm{I}_{\text {OUT }}=\right.$ rated current, $\mathrm{T}_{\mathrm{J}}=25^{\circ} \mathrm{C}$, unless otherwise specified $)$

| Symbol | Parameter | Test condition | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min. | Typ. | Max. |  |
| $\mathrm{V}_{\mathrm{IN}}$ | High level input voltage | $\mathrm{V}_{\mathrm{IN}}=2.7$ to 5.5 V | 2.0 | - | - | V |
| $\mathrm{V}_{\mathrm{IL}}$ | Low level input voltage | $\mathrm{V}_{\mathrm{IN}}=4.5$ to 5.5 V | - | - | 0.8 | V |
|  |  | $\mathrm{V}_{\mathrm{IN}}=2.7$ to 4.5 V | - | - | 0.4 | V |
| $\mathrm{I}_{\text {IN }}$ | Input current | $\mathrm{V}_{\text {IEN }}=0 \mathrm{~V}$ or $\mathrm{V}_{\text {IN }}$ | $-0.5$ | - | 0.5 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {ON }}$ | Turn-ON time ${ }^{(1)}$ | $\begin{aligned} & R_{L}=10 \Omega \\ & C_{L}=100 \mu \mathrm{~F} \end{aligned}$ | - | - | 5 | ms |
| $\mathrm{I}_{\text {OFF }}$ | Turn-OFF time ${ }^{(1)}$ | $\begin{aligned} & R_{L}=10 \Omega \\ & C_{L}=100 \mu \mathrm{~F} \end{aligned}$ | - | - | 10 | ms |

1. Not tested in production, specified by design.# 7 Detail device characteristics 

### 7.1 STMPS2141, STMPS2151 additional electrical charts

The waveforms displayed in Section 7.1 are captured with the STMPS2141 device. The STMPS2151 device is expected to have the same characteristics with EN in the opposite polarity.

### 7.1.1 Turn-on/off characteristics at $V_{\text {OUT }}=5.0 \mathrm{~V}$

Figure 3. Voltage output turn-on delay time (STMPS2141/2151, 5 V)
![img-3.jpeg](img-3.jpeg)

Figure 5. Current output turn-on delay time (STMPS2141/2151, 5 V)
![img-4.jpeg](img-4.jpeg)

Figure 4. Voltage output turn-off delay time (STMPS2141/2151, 5 V)
![img-5.jpeg](img-5.jpeg)

Figure 6. Current output turn-off delay time (STMPS2141/2151, 5 V)
![img-6.jpeg](img-6.jpeg)# 7.1.2 Turn-on/off characteristics at $V_{\text {OUT }}=3.0 \mathrm{~V}$ 

Figure 7. Voltage output turn-on delay time (STMPS2141/2151, 3 V)
![img-7.jpeg](img-7.jpeg)

Figure 9. Current output turn-on delay time (STMPS2141/2151, 3 V)
![img-8.jpeg](img-8.jpeg)

Figure 8. Voltage output turn-off delay time (STMPS2141/2151, 3 V)
![img-9.jpeg](img-9.jpeg)

Figure 10. Current output turn-off delay time (STMPS2141/2151, 3 V)
![img-10.jpeg](img-10.jpeg)# 7.1.3 UVLO 

Figure 11. UVLO rising (STMPS2141/2151)
![img-11.jpeg](img-11.jpeg)

Figure 12. UVLO falling (STMPS2141/2151)
![img-12.jpeg](img-12.jpeg)

### 7.1.4 OC protection characteristics

Figure 13. OC protection at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ (STMPS2141/2151)
![img-13.jpeg](img-13.jpeg)

Figure 14. OC protection at $\mathrm{V}_{\text {OUT }} \approx 3.0 \mathrm{~V}$ (STMPS2141/2151 - detail)
![img-14.jpeg](img-14.jpeg)Figure 15. OC protection at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ (STMPS2141/2151)
![img-15.jpeg](img-15.jpeg)

Figure 16. OC protection at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ (STMPS2141/2151 - detail)
![img-16.jpeg](img-16.jpeg)

# 7.1.5 Other electrical characteristics 

Figure 17. $\mathrm{I}_{\mathrm{CC}}$ vs. $\mathrm{V}_{\text {IN }}$ (enabled) (STMPS2141/2151)
![img-17.jpeg](img-17.jpeg)

Figure 19. $\mathrm{I}_{\mathrm{CC}}$ vs. $\mathrm{V}_{\text {IN }}$ (disabled) (STMPS2141/2151)
![img-18.jpeg](img-18.jpeg)

Figure 18. $\mathrm{I}_{\mathrm{CC}}$ vs. temperature (enabled) (STMPS2141/2151)
![img-19.jpeg](img-19.jpeg)

Figure 20. $\mathrm{I}_{\mathrm{CC}}$ vs. temperature (disabled) (STMPS2141/2151)
![img-20.jpeg](img-20.jpeg)Figure 21. $R_{O N}$ vs. $V_{I N}$ (STMPS2141/2151)
![img-21.jpeg](img-21.jpeg)

Figure 23. $I_{O S}$ vs. temperature (STMPS2141/2151)
![img-22.jpeg](img-22.jpeg)

Figure 25. Output rise time vs. $V_{I N}$ (STMPS2141/2151)
![img-23.jpeg](img-23.jpeg)

Figure 22. $R_{O N}$ vs. temperature (STMPS2141/2151)
![img-24.jpeg](img-24.jpeg)

Figure 24. Switch leakage vs. temperature (STMPS2141/2151)
![img-25.jpeg](img-25.jpeg)

Figure 26. Output fall time vs. $V_{I N}$ (STMPS2141/2151)
![img-26.jpeg](img-26.jpeg)Figure 27. UVLO vs. temperature (STMPS2141/2151)
![img-27.jpeg](img-27.jpeg)# 7.2 STMPS2161, STMPS2171 electrical charts 

The waveforms displayed in Section 7.2 are captured with the STMPS2161 device. The STMPS2171 device is expected to have the same characteristics with EN in the opposite polarity.

### 7.2.1 Turn-on/off characteristics at $V_{\text {OUT }}=5.0 \mathrm{~V}$

Figure 28. Voltage output turn-on delay time (STMPS2161/2171, 5 V)
![img-28.jpeg](img-28.jpeg)

Figure 30. Current output turn-on delay time (STMPS2161/2171, 5 V)
![img-29.jpeg](img-29.jpeg)

Figure 31. Current output turn-off delay time (STMPS2161/2171, 5 V)
![img-30.jpeg](img-30.jpeg)# 7.2.2 Turn-on/off characteristics at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ 

Figure 32. Voltage output turn-on delay time (STMPS2161/2171, 3 V)
![img-31.jpeg](img-31.jpeg)

Figure 34. Current output turn-on delay time (STMPS2161/2171, 3 V)
![img-32.jpeg](img-32.jpeg)

Figure 33. Voltage output turn-off delay time (STMPS2161/2171, 3 V)
![img-33.jpeg](img-33.jpeg)

Figure 35. Current output turn-off delay time (STMPS2161/2171, 3 V)
![img-34.jpeg](img-34.jpeg)# 7.2.3 UVLO 

Figure 36. UVLO rising (STMPS2161/2171)
![img-35.jpeg](img-35.jpeg)

Figure 37. UVLO falling (STMPS2161/2171)
![img-36.jpeg](img-36.jpeg)

### 7.2.4 OC protection characteristics

Figure 38. OC protection at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ (STMPS2161/2171)
![img-37.jpeg](img-37.jpeg)

Figure 39. OC protection at $\mathrm{V}_{\text {OUT }}=3.0 \mathrm{~V}$ (STMPS2161/2171- detail)
![img-38.jpeg](img-38.jpeg)Figure 40. OC protection at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ (STMPS2161/2171)
![img-39.jpeg](img-39.jpeg)

Figure 41. OC protection at $\mathrm{V}_{\text {OUT }}=5.0 \mathrm{~V}$ (STMPS2161/2171- detail)
![img-40.jpeg](img-40.jpeg)

# 7.2.5 Other electrical characteristics 

Figure 42. $\mathrm{I}_{\mathrm{CC}}$ vs. $\mathrm{V}_{\mathrm{IN}}$ (enabled) (STMPS2161/2171)
![img-41.jpeg](img-41.jpeg)

Figure 44. $\mathrm{I}_{\mathrm{CC}}$ vs. $\mathrm{V}_{\mathrm{IN}}$ (disabled) (STMPS2161/2171)
![img-42.jpeg](img-42.jpeg)

Figure 45. $\mathrm{I}_{\mathrm{CC}}$ vs. temperature (disabled) (STMPS2161/2171)
![img-43.jpeg](img-43.jpeg)Figure 46. $R_{\text {ON }}$ vs. $V_{\text {IN }}$ (STMPS2161/2171)
![img-44.jpeg](img-44.jpeg)

Figure 48. $I_{\text {OS }}$ vs. temperature (STMPS2161/2171)
![img-45.jpeg](img-45.jpeg)

Figure 50. Output rise time vs. $V_{\text {IN }}$ (STMPS2161/2171)
![img-46.jpeg](img-46.jpeg)

Figure 47. $R_{\text {ON }}$ vs. temperature (STMPS2161/2171)
![img-47.jpeg](img-47.jpeg)

Figure 49. Switch leakage vs. temperature (STMPS2161/2171)
![img-48.jpeg](img-48.jpeg)

Figure 51. Output fall time vs. $V_{\text {IN }}$ (STMPS2161/2171)
![img-49.jpeg](img-49.jpeg)Figure 52. UVLO vs. temperature (STMPS2161/2171)
![img-50.jpeg](img-50.jpeg)# 8 Package mechanical data 

In order to meet environmental requirements, ST offers these devices in different grades of ECOPACK ${ }^{\circledR}$ packages, depending on their level of environmental compliance. ECOPACK specifications, grade definitions and product status are available at: www.st.com. ECOPACK is an ST trademark.

Figure 53. SOT23-5L package outline
![img-51.jpeg](img-51.jpeg)

Table 17. SOT23-5L package mechanical data

| Symbol | Dimensions |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Millimeters |  |  | Inches |  |  |
|  | Min. | Typ. | Max. | Min. | Typ. | Max. |
| A | 0.90 | - | 1.45 | 35.4 | - | 57.1 |
| A1 | 0.00 | - | 0.10 | 0.0 | - | 3.9 |
| A2 | 0.90 | - | 1.30 | 35.4 | - | 51.2 |
| b | 0.35 | - | 0.50 | 13.7 | - | 19.7 |
| C | 0.09 | - | 0.20 | 3.5 | - | 7.8 |
| D | 2.80 | - | 3.00 | 110.2 | - | 118.1 |
| E | 1.50 | - | 1.75 | 59.0 | - | 68.8 |
| e | - | 0.95 | - | - | 37.4 | - |
| H | 2.60 | - | 3.00 | 102.3 | - | 118.1 |
| L | 0.10 | - | 0.60 | 3.9 | - | 23.6 |Figure 54. SOT23-5L footprint recommendations
![img-52.jpeg](img-52.jpeg)

Table 18. SOT23-5L footprint dimensions

| Footprint data |  |  |
| :--: | :--: | :--: |
| Symbol | Dimensions |  |
|  | Millimeters | Inches |
| A | 3.50 | 0.138 |
| B | 1.10 | 0.043 |
| C | 0.60 | 0.024 |
| D | 0.95 | 0.037 |
| E | 1.20 | 0.047 |
| F | 2.30 | 0.090 |

Figure 55. SOT23-5L carrier tape
![img-53.jpeg](img-53.jpeg)Figure 56. SO-8 package outline
![img-54.jpeg](img-54.jpeg)

1. Drawing is not to scale.

Table 19. SO-8 mechanical data

| Symbol | Dimensions |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Millimeters |  |  | Inches |  |  |
|  | Min. | Typ. | Max. | Min. | Typ. | Max. |
| A | 1.35 | - | 1.75 | 0.053 | - | 0.069 |
| A1 | 0.10 | - | 0.25 | 0.004 | - | 0.010 |
| A2 | 1.10 | - | 1.65 | 0.043 | - | 0.065 |
| B | 0.33 | - | 0.51 | 0.013 | - | 0.020 |
| C | 0.19 | - | 0.25 | 0.007 | - | 0.010 |
| $D^{(1)}$ | 4.80 | - | 5.00 | 0.189 | - | 0.197 |
| E | 3.80 | - | 4.00 | 0.15 | - | 0.157 |
| e | - | 1.27 | - | - | 0.050 | - |
| H | 5.80 | - | 6.20 | 0.228 | - | 0.244 |
| h | 0.25 | - | 0.50 | 0.010 | - | 0.020 |
| L | 0.40 | - | 1.27 | 0.016 | - | 0.050 |
| k | $0^{\circ}$ (min.), $8^{\circ}$ (max.) |  |  |  |  |  |
| ddd | - | - | 0.10 | - | - | 0.004 |

1. Dimension D does not include mold flash, protrusions or gate burrs. Mold flash, potrusions or gate burrs shall not exceed 0.15 mm ( 0.006 inch) in total (both sides).Figure 57. SO-8 carrier tape
![img-55.jpeg](img-55.jpeg)![img-56.jpeg](img-56.jpeg)

1. Drawing not to scale.

Table 20. MSOP8 package mechanical data

| Symbol | Dimensions |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Millimeters |  |  | Inches |  |  |
|  | Min. | Typ. | Max. | Min. | Typ. | Max. |
| A | - | - | 1.10 | - | - | 0.043 |
| A1 | 0.05 | - | 0.15 | 0.002 | 0.004 | 0.006 |
| A2 | 0.75 | 0.85 | 0.95 | 0.031 | 0.034 | 0.037 |
| b | 0.25 | - | 0.40 | 0.010 | 0.013 | 0.016 |
| c | 0.13 | - | 0.23 | 0.005 | 0.007 | 0.009 |
| D | 2.90 | 3.00 | 3.10 | 0.114 | 0.118 | 0.122 |
| E | 4.65 | 4.90 | 5.15 | 0.187 | 0.193 | 0.199 |
| E1 | 2.90 | 3.00 | 3.10 | 0.114 | 0.118 | 0.122 |
| e | - | 0.65 | - | - | 0.026 | - |
| L | 0.40 | 0.55 | 0.70 | 0.016 | 0.022 | 0.028 |
| L1 | - | 0.95 | - | - | 0.037 |  |
| K | $0^{\circ}$ | - | $6^{\circ}$ | $0^{\circ}$ | - | $6^{\circ}$ |
| ccc |  |  | 0.10 |  |  | 0.004 |Figure 59. MSOP8 carrier tape
![img-57.jpeg](img-57.jpeg)Figure 60. Reel information
![img-58.jpeg](img-58.jpeg)

Table 21. Reel mechanical data

| Symbol | Dimensions (mm) |  |  |
| :--: | :--: | :--: | :--: |
|  | Min. | Typ. | Max. |
| A |  |  |  |
| SOT23-5L | - | - | 180 |
| S0-8, MSOP8 |  |  | 330 |
| C | 12.8 | 13.0 | 13.2 |
| D | 20.2 | - | - |
| N | 60 | - | - |
| T | - | - | 22.4 |# 9 Ordering information 

Table 22. Order codes

| Part number | Package | Marking |
| :--: | :--: | :--: |
| STMPS2141MTR | SO-8 | 2141E |
| STMPS2151MTR |  | 2151E |
| STMPS2161MTR |  | 2161E |
| STMPS2171MTR |  | 2171E |
| STMPS2141STR | SOT23-5L | 2141 |
| STMPS2151STR |  | 2151 |
| STMPS2161STR |  | 2161 |
| STMPS2171STR |  | 2171 |
| STMPS2141TTR | MSOP8 | 2141 |
| STMPS2151TTR |  | 2151 |
| STMPS2161TTR |  | 2161 |
| STMPS2171TTR |  | 2171 |# 10 Revision history 

Table 23. Document revision history

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 01-Aug-2007 | 1 | Initial release. |
| 18-Dec-2007 | 2 | Minor text changes, updated Figure 53 on page 28, added Section 7: Detail device characteristics on page 16. |
| 24-Jan-2008 | 3 | Footnote added in Table 1 on page 1, replaced Figure 58 on page 32 and Table 20 on page 32, TSSOP8 package name replaced with MSOP8. |
| 17-Jul-2009 | 4 | Updated Chapter 3, test conditions modified for $I_{\text {reverse }}$ in Table 12 on page 14 and Chapter 7. <br> Added: Figure 55, Figure 56, Figure 59, Figure 59 and Figure 60. |
| 21-Nov-2012 | 5 | Updated Table 1 (replaced "Current limit" by "Rated continuous output current"). <br> Updated values and units in Table 4 to Table 6. <br> Corrected Figure 1 and Figure 2 (replaced EN_N by EN). <br> Replaced $I_{I}$ by $I_{I N}, I_{L I M I T}$ by $I_{O S}, I_{O}$ by $I_{O U T}, m S$ by ms, OC by FAULT, $R_{d s(o n)}$ by $R_{O N}, V_{I}$ and $V_{C C}$ by $V_{I N}, V_{O}$ by $V_{O U T}$, in the whole document. <br> Updated Section 5 (added cross-references). <br> Updated Table 9 and Table 10 (replaced Tr by $t_{r}$ ), Table 11 (updated test conditions). <br> Updated Table 15 (replaced Fault, OCx, and VOC by FAULT). <br> Updated Table 16 (replaced $\mathrm{V}_{\text {IENX }}$ by $\mathrm{V}_{\text {IEN }}, \mathrm{mF}$ by $\mu \mathrm{F}$ ). <br> Updated titles of Figure 3 to Figure 52 (added conditions). <br> Updated Figure 3 to Figure 16, Figure 25, Figure 26, Figure 28 to <br> Figure 41, Figure 50 and Figure 51 (replaced $R_{L}$ by $I_{L}, m S$ by ms, and uF by $\mu \mathrm{F}$ ). <br> Updated Figure 55, Figure 59, and Figure 59 (removed superfluous references to notes). <br> Reformatted Section 8 (moved Figure 57 on page 31). <br> Removed Figure 56. <br> Updated Table 21 (added SOT23-5L, SO-8, and MSOP package and max. value for SOT23-5L package). <br> Added Section 9. <br> Minor corrections throughout document. |
| 25-Jan-2013 | 6 | Updated Features (added UL recognized components). |Please Read Carefully:

Information in this document is provided solely in connection with ST products. STMicroelectronics NV and its subsidiaries ("ST") reserve the right to make changes, corrections, modifications or improvements, to this document, and the products and services described herein at any time, without notice.

All ST products are sold pursuant to ST's terms and conditions of sale.
Purchasers are solely responsible for the choice, selection and use of the ST products and services described herein, and ST assumes no liability whatsoever relating to the choice, selection or use of the ST products and services described herein.

No license, express or implied, by estoppel or otherwise, to any intellectual property rights is granted under this document. If any part of this document refers to any third party products or services it shall not be deemed a license grant by ST for the use of such third party products or services, or any intellectual property contained therein or considered as a warranty covering the use in any manner whatsoever of such third party products or services or any intellectual property contained therein.

UNLESS OTHERWISE SET FORTH IN ST'S TERMS AND CONDITIONS OF SALE ST DISCLAIMS ANY EXPRESS OR IMPLIED WARRANTY WITH RESPECT TO THE USE AND/OR SALE OF ST PRODUCTS INCLUDING WITHOUT LIMITATION IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE (AND THEIR EQUIVALENTS UNDER THE LAWS OF ANY JURISDICTION), OR INFRINGEMENT OF ANY PATENT, COPYRIGHT OR OTHER INTELLECTUAL PROPERTY RIGHT.
UNLESS EXPRESSLY APPROVED IN WRITING BY TWO AUTHORIZED ST REPRESENTATIVES, ST PRODUCTS ARE NOT RECOMMENDED, AUTHORIZED OR WARRANTED FOR USE IN MILITARY, AIR CRAFT, SPACE, LIFE SAVING, OR LIFE SUSTAINING APPLICATIONS, NOR IN PRODUCTS OR SYSTEMS WHERE FAILURE OR MALFUNCTION MAY RESULT IN PERSONAL INJURY, DEATH, OR SEVERE PROPERTY OR ENVIRONMENTAL DAMAGE. ST PRODUCTS WHICH ARE NOT SPECIFIED AS "AUTOMOTIVE GRADE" MAY ONLY BE USED IN AUTOMOTIVE APPLICATIONS AT USER'S OWN RISK.

Resale of ST products with provisions different from the statements and/or technical features set forth in this document shall immediately void any warranty granted by ST for the ST product or service described herein and shall not create or extend in any manner whatsoever, any liability of ST.

ST and the ST logo are trademarks or registered trademarks of ST in various countries.
Information in this document supersedes and replaces all information previously supplied.
The ST logo is a registered trademark of STMicroelectronics. All other names are the property of their respective owners.
(c) 2013 STMicroelectronics - All rights reserved

STMicroelectronics group of companies
Australia - Belgium - Brazil - Canada - China - Czech Republic - Finland - France - Germany - Hong Kong - India - Israel - Italy - Japan Malaysia - Malta - Morocco - Philippines - Singapore - Spain - Sweden - Switzerland - United Kingdom - United States of America
www.st.com