# LIS3DH 

## MEMS digital output motion sensor: ultra-low-power high-performance 3-axis "nano" accelerometer

Datasheet - production data
![img-0.jpeg](img-0.jpeg)

## Features

- Wide supply voltage, 1.71 V to 3.6 V
- Independent IO supply (1.8 V) and supply voltage compatible
- Ultra-low-power mode consumption down to $2 \mu \mathrm{~A}$
- $\pm 2 g / \pm 4 g / \pm 8 g / \pm 16 g$ dynamically selectable full scale
- $I^{2} \mathrm{C} / \mathrm{SPI}$ digital output interface
- 16-bit data output
- 2 independent programmable interrupt generators for free-fall and motion detection
- 6D/4D orientation detection
- Free-fall detection
- Motion detection
- Embedded temperature sensor
- Embedded self-test
- Embedded 32 levels of 16-bit data output FIFO
- 10000 g high shock survivability
- ECOPACK ${ }^{\circledR}$, RoHS and "Green" compliant


## Applications

- Motion activated functions
- Free-fall detection
- Click/double-click recognition
- Intelligent power saving for handheld devices
- Pedometers
- Display orientation
- Gaming and virtual reality input devices
- Impact recognition and logging
- Vibration monitoring and compensation


## Description

The LIS3DH is an ultra-low-power highperformance three-axis linear accelerometer belonging to the "nano" family, with digital $\mathrm{I}^{2} \mathrm{C} / \mathrm{SPI}$ serial interface standard output. The device features ultra-low-power operational modes that allow advanced power saving and smart embedded functions.

The LIS3DH has dynamically user-selectable full scales of $\pm 2 \mathrm{~g} / \pm 4 \mathrm{~g} / \pm 8 \mathrm{~g} / \pm 16 \mathrm{~g}$ and is capable of measuring accelerations with output data rates from 1 Hz to 5.3 kHz . The self-test capability allows the user to check the functioning of the sensor in the final application. The device may be configured to generate interrupt signals using two independent inertial wake-up/free-fall events as well as by the position of the device itself. Thresholds and timing of interrupt generators are programmable by the end user on the fly. The LIS3DH has an integrated 32-level first-in, firstout (FIFO) buffer allowing the user to store data in order to limit intervention by the host processor. The LIS3DH is available in small thin plastic land grid array package (LGA) and is guaranteed to operate over an extended temperature range from $-40^{\circ} \mathrm{C}$ to $+85^{\circ} \mathrm{C}$.

Table 1. Device summary

| Order codes | Temp. <br> range $\left[{ }^{\circ} \mathrm{C}\right]$ | Package | Packaging |
| :-- | :--: | :--: | :--: |
| LIS3DHTR | -40 to +85 | LGA-16 | Tape and reel |# Contents 

1 Block diagram and pin description ..... 8
1.1 Block diagram ..... 8
1.2 Pin description ..... 8
2 Mechanical and electrical specifications ..... 10
2.1 Mechanical characteristics ..... 10
2.2 Temperature sensor characteristics ..... 12
2.3 Electrical characteristics ..... 12
2.4 Communication interface characteristics ..... 13
2.4.1 SPI - serial peripheral interface ..... 13
2.4.2 $\mathrm{I}^{2} \mathrm{C}$ - Inter IC control interface ..... 14
2.5 Absolute maximum ratings ..... 15
3 Terminology and functionality ..... 16
3.1 Terminology ..... 16
3.1.1 Sensitivity ..... 16
3.1.2 Zero-g level ..... 16
3.2 Functionality ..... 16
3.2.1 High-resolution, normal mode, low-power mode ..... 16
3.2.2 Self-test ..... 17
3.2.3 6D / 4D orientation detection ..... 18
3.2.4 "Sleep-to-wake" and "Return-to-sleep" ..... 18
3.3 Sensing element ..... 18
3.4 IC interface ..... 18
3.5 Factory calibration ..... 19
3.6 FIFO ..... 19
3.7 Auxiliary ADC and temperature sensor ..... 19
4 Application hints ..... 20
4.1 Soldering information ..... 21
5 Digital main blocks ..... 22
5.1 FIFO ..... 225.1.1 Bypass mode ..... 22
5.1.2 FIFO mode ..... 22
5.1.3 Stream mode ..... 23
5.1.4 Stream-to-FIFO mode ..... 23
5.1.5 Retrieving data from FIFO ..... 23
6 Digital interfaces ..... 24
$6.1 \quad \mathrm{I}^{2} \mathrm{C}$ serial interface ..... 24
6.1.1 $\quad \mathrm{I}^{2} \mathrm{C}$ operation ..... 25
6.2 SPI bus interface ..... 27
6.2.1 SPI read ..... 28
6.2.2 SPI write ..... 29
6.2.3 SPI read in 3-wire mode ..... 30
7 Register mapping ..... 31
8 Registers description ..... 33
8.1 STATUS_REG_AUX (07h) ..... 33
8.2 OUT_ADC1_L (08h), OUT_ADC1_H (09h) ..... 33
8.3 OUT_ADC2_L (0Ah), OUT_ADC2_H (0Bh) ..... 33
8.4 OUT_ADC3_L (0Ch), OUT_ADC3_H (0Dh) ..... 33
8.5 WHO_AM_I (0Fh) ..... 34
8.6 CTRL_REG0 (1Eh) ..... 34
8.7 TEMP_CFG_REG (1Fh) ..... 34
8.8 CTRL_REG1 (20h) ..... 35
8.9 CTRL_REG2 (21h) ..... 36
8.10 CTRL_REG3 (22h) ..... 36
8.11 CTRL_REG4 (23h) ..... 37
8.12 CTRL_REG5 (24h) ..... 38
8.13 CTRL_REG6 (25h) ..... 38
8.14 REFERENCE (26h) ..... 39
8.15 STATUS_REG (27h) ..... 39
8.16 OUT_X_L (28h), OUT_X_H (29h) ..... 40
8.17 OUT_Y_L (2Ah), OUT_Y_H (2Bh) ..... 40
8.18 OUT_Z_L (2Ch), OUT_Z_H (2Dh) ..... 408.19 FIFO_CTRL_REG (2Eh) ..... 40
8.20 FIFO_SRC_REG (2Fh) ..... 41
8.21 INT1_CFG (30h) ..... 41
8.22 INT1_SRC (31h) ..... 42
8.23 INT1_THS (32h) ..... 43
8.24 INT1_DURATION (33h) ..... 43
8.25 INT2_CFG (34h) ..... 44
8.26 INT2_SRC (35h) ..... 45
8.27 INT2_THS (36h) ..... 45
8.28 INT2_DURATION (37h) ..... 46
8.29 CLICK_CFG (38h) ..... 46
8.30 CLICK_SRC (39h) ..... 47
8.31 CLICK_THS (3Ah) ..... 47
8.32 TIME_LIMIT (3Bh) ..... 47
8.33 TIME_LATENCY (3Ch) ..... 48
8.34 TIME WINDOW (3Dh) ..... 48
8.35 ACT_THS (3Eh) ..... 48
8.36 ACT_DUR (3Fh) ..... 48
9 Package information ..... 49
9.1 LGA-16 package information ..... 50
9.2 LGA-16 packing information ..... 51
10 Revision history ..... 53# List of tables 

Table 1. Device summary ..... 1
Table 2. Pin description ..... 9
Table 3. Internal pull-up values (typ.) for SDO/SA0 pin ..... 9
Table 4. Mechanical characteristics ..... 10
Table 5. Temperature sensor characteristics ..... 12
Table 6. Electrical characteristics ..... 12
Table 7. SPI slave timing values ..... 13
Table 8. $\mathrm{I}^{2} \mathrm{C}$ slave timing values ..... 14
Table 9. Absolute maximum ratings ..... 15
Table 10. Operating mode selection ..... 16
Table 11. Turn-on time for operating mode transition ..... 17
Table 12. Current consumption of operating modes ..... 17
Table 13. Internal pin status ..... 21
Table 14. Serial interface pin description ..... 24
Table 15. $\mathrm{I}^{2} \mathrm{C}$ terminology ..... 24
Table 16. SAD+Read/Write patterns ..... 25
Table 17. Transfer when master is writing one byte to slave ..... 25
Table 18. Transfer when master is writing multiple bytes to slave ..... 25
Table 19. Transfer when master is receiving (reading) one byte of data from slave ..... 26
Table 20. Transfer when master is receiving (reading) multiple bytes of data from slave ..... 26
Table 21. Register address map ..... 31
Table 22. STATUS_REG_AUX register ..... 33
Table 23. STATUS_REG_AUX description ..... 33
Table 24. WHO_AM_I register ..... 34
Table 25. CTRL_REG0 register ..... 34
Table 26. CTRL_REG0 description ..... 34
Table 27. TEMP_CFG_REG register ..... 34
Table 28. TEMP_CFG_REG description ..... 34
Table 29. CTRL_REG1 register ..... 35
Table 30. CTRL_REG1 description ..... 35
Table 31. Data rate configuration ..... 35
Table 32. CTRL_REG2 register ..... 36
Table 33. CTRL_REG2 description ..... 36
Table 34. High-pass filter mode configuration ..... 36
Table 35. CTRL_REG3 register ..... 36
Table 36. CTRL_REG3 description ..... 36
Table 37. CTRL_REG4 register ..... 37
Table 38. CTRL_REG4 description ..... 37
Table 39. Self-test mode configuration ..... 37
Table 40. CTRL_REG5 register ..... 38
Table 41. CTRL_REG5 description ..... 38
Table 42. CTRL_REG6 register ..... 38
Table 43. CTRL_REG6 description ..... 38
Table 44. REFERENCE register ..... 39
Table 45. REFERENCE register description ..... 39
Table 46. STATUS register ..... 39
Table 47. STATUS register description ..... 39
Table 48. REFERENCE register ..... 40Table 49. REFERENCE register description ..... 40
Table 50. FIFO mode configuration ..... 40
Table 51. FIFO_SRC_REG register ..... 41
Table 52. FIFO_SRC_REG description ..... 41
Table 53. INT1_CFG register ..... 41
Table 54. INT1_CFG description ..... 41
Table 55. Interrupt mode ..... 42
Table 56. INT1_SRC register ..... 42
Table 57. INT1_SRC description ..... 42
Table 58. INT1_THS register ..... 43
Table 59. INT1_THS description ..... 43
Table 60. INT1_DURATION register ..... 43
Table 61. INT1_DURATION description ..... 43
Table 62. INT2_CFG register ..... 44
Table 63. INT2_CFG description ..... 44
Table 64. Interrupt mode ..... 44
Table 65. INT2_SRC register ..... 45
Table 66. INT2_SRC description ..... 45
Table 67. INT2_THS register ..... 45
Table 68. INT2_THS description ..... 45
Table 69. INT2_DURATION register ..... 46
Table 70. INT2_DURATION description ..... 46
Table 71. CLICK_CFG register ..... 46
Table 72. CLICK_CFG description ..... 46
Table 73. CLICK_SRC register ..... 47
Table 74. CLICK_SRC description ..... 47
Table 75. CLICK_THS register ..... 47
Table 76. CLICK_SRC description ..... 47
Table 77. TIME_LIMIT register ..... 47
Table 78. TIME_LIMIT description ..... 47
Table 79. TIME_LATENCY register ..... 48
Table 80. TIME_LATENCY description ..... 48
Table 81. TIME_WINDOW register. ..... 48
Table 82. TIME_WINDOW description ..... 48
Table 83. ACT_THS register ..... 48
Table 84. ACT_THS description ..... 48
Table 85. ACT_DUR register ..... 48
Table 86. ACT_DUR description ..... 48
Table 87. Reel dimensions for carrier tape of LGA-16 package ..... 52
Table 88. Document revision history. ..... 53# List of figures 

Figure 1. Block diagram ..... 8
Figure 2. Pin connections ..... 8
Figure 3. SPI slave timing diagram ..... 13
Figure 4. $\quad \mathrm{I}^{2} \mathrm{C}$ slave timing diagram ..... 14
Figure 5. LIS3DH electrical connections ..... 20
Figure 6. Read and write protocol ..... 27
Figure 7. SPI read protocol ..... 28
Figure 8. Multiple byte SPI read protocol (2-byte example) ..... 28
Figure 9. SPI write protocol ..... 29
Figure 10. Multiple byte SPI write protocol (2-byte example). ..... 29
Figure 11. SPI read protocol in 3-wire mode ..... 30
Figure 12. LGA-16 package outline and mechanical dimensions ..... 50
Figure 13. Carrier tape information for LGA-16 package ..... 51
Figure 14. LGA-16 package orientation in carrier tape ..... 51
Figure 15. Reel information for carrier tape of LGA-16 package ..... 52# 1 Block diagram and pin description 

### 1.1 Block diagram

Figure 1. Block diagram
![img-1.jpeg](img-1.jpeg)

### 1.2 Pin description

Figure 2. Pin connections
![img-2.jpeg](img-2.jpeg)Table 2. Pin description

| Pin\# | Name | Function |
| :--: | :--: | :--: |
| 1 | Vdd_IO | Power supply for I/O pins |
| 2 | NC | Not connected |
| 3 | NC | Not connected |
| 4 | SCL <br> SPC | $\mathrm{I}^{2} \mathrm{C}$ serial clock (SCL) SPI serial port clock (SPC) |
| 5 | GND | 0 V supply |
| 6 | SDA <br> SDI <br> SDO | $\mathrm{I}^{2} \mathrm{C}$ serial data (SDA) <br> SPI serial data input (SDI) <br> 3-wire interface serial data output (SDO) |
| $7^{(1)}$ | SDO <br> SA0 | SPI serial data output (SDO) <br> $I^{2} \mathrm{C}$ less significant bit of the device address (SA0) |
| 8 | CS | SPI enable <br> $I^{2} \mathrm{C} /$ SPI mode selection: <br> 1: SPI idle mode $/ I^{2} \mathrm{C}$ communication enabled <br> 0: SPI communication mode / $\mathrm{I}^{2} \mathrm{C}$ disabled |
| 9 | INT2 | Inertial interrupt 2 |
| 10 | RES | Connect to GND |
| 11 | INT1 | Inertial interrupt 1 |
| 12 | GND | 0 V supply |
| 13 | ADC3 | Analog-to-digital converter input 3 |
| 14 | Vdd | Power supply |
| 15 | ADC2 | Analog-to-digital converter input 2 |
| 16 | ADC1 | Analog-to-digital converter input 1 |

1. SDO/SA0 pin is internally pulled up. Refer to Table 3 for the internal pull-up values (typ.).

Table 3. Internal pull-up values (typ.) for SDO/SA0 pin

| Vdd_IO | Resistor value for SDO/SA0 pin |
| --- | --- |
|  | Typ. $(\mathbf{k} \Omega)$ |
| 1.7 V | 54.4 |
| 1.8 V | 49.2 |
| 2.5 V | 30.4 |
| 3.6 V | 20.4 |# 2 Mechanical and electrical specifications 

### 2.1 Mechanical characteristics

Vdd $=2.5 \mathrm{~V}, \mathrm{~T}=25^{\circ} \mathrm{C}$ unless otherwise noted ${ }^{(a)}$

Table 4. Mechanical characteristics

| Symbol | Parameter | Test conditions | Min. | Typ. ${ }^{(1)}$ | Max. | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| FS | Measurement range ${ }^{(2)}$ | FS bit set to 00 |  | $\pm 2.0$ |  |  |
|  |  | FS bit set to 01 |  | $\pm 4.0$ |  |  |
|  |  | FS bit set to 10 |  | $\pm 8.0$ |  |  |
|  |  | FS bit set to 11 |  | $\pm 16.0$ |  | $g$ |
| So | Sensitivity | FS bit set to 00; <br> High-resolution mode |  | 1 |  | $\mathrm{mg} /$ digit |
|  |  | FS bit set to 00; <br> Normal mode |  | 4 |  |  |
|  |  | FS bit set to 00; <br> Low-power mode |  | 16 |  |  |
|  |  | FS bit set to 01; <br> High-resolution mode |  | 2 |  | $\mathrm{mg} /$ digit |
|  |  | FS bit set to 01; <br> Normal mode |  | 8 |  |  |
|  |  | FS bit set to 01; <br> Low-power mode |  | 32 |  |  |
|  |  | FS bit set to 10; <br> High-resolution mode |  | 4 |  | $\mathrm{mg} /$ digit |
|  |  | FS bit set to 10; <br> Normal mode |  | 16 |  |  |
|  |  | FS bit set to 10; <br> Low-power mode |  | 64 |  |  |
|  |  | FS bit set to 11; <br> High-resolution mode |  | 12 |  | $\mathrm{mg} /$ digit |
|  |  | FS bit set to 11; <br> Normal mode |  | 48 |  |  |
|  |  | FS bit set to 11; <br> Low-power mode |  | 192 |  |  |
| TCSo | Sensitivity change vs temperature | FS bit set to 00 |  | 0.01 |  | $\%^{\circ} \mathrm{C}$ |
| TyOff | Typical zero-g level offset accuracy ${ }^{(3),(4)}$ | FS bit set to 00 |  | $\pm 40$ |  | mg |

a. The product is factory calibrated at 2.5 V . The operational power supply range is from 1.71 V to 3.6 V .Table 4. Mechanical characteristics

| Symbol | Parameter | Test conditions | Min. | Typ. ${ }^{(1)}$ | Max. | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TCOff | Zero-g level change vs temperature | Max delta from $25^{\circ} \mathrm{C}$ |  | $\pm 0.5$ |  | $\mathrm{mg} /{ }^{\circ} \mathrm{C}$ |
| An | Acceleration noise density | FS bit set to 00, High-Resolution mode (Table 10), ODR $>1300 \mathrm{~Hz}$ |  | 220 |  | $\mu \mathrm{g} / \sqrt{ } \mathrm{Hz}$ |
| Vst | Self-test output change ${ }^{(5)(6)(7)}$ | FS bit set to 00 <br> X-axis; Normal mode | 17 |  | 360 | LSb |
|  |  | FS bit set to 00 <br> Y-axis; Normal mode | 17 |  | 360 | LSb |
|  |  | FS bit set to 00 <br> Z-axis; Normal mode | 17 |  | 360 | LSb |
| Top | Operating temperature range |  | $-40$ |  | $+85$ | ${ }^{\circ} \mathrm{C}$ |

1. Typical specifications are not guaranteed.
2. Verified by wafer level test and measurement of initial offset and sensitivity.
3. Typical zero-g level offset value after MSL3 preconditioning.
4. Offset can be eliminated by enabling the built-in high-pass filter.
5. The sign of "Self-test output change" is defined by the ST bits in CTRL_REG4 (23h), for all axes.
6. "Self-test output change" is defined as the absolute value of:

OUTPUT[LSb] $[$ Self test enabled $]^{-}$OUTPUT[LSb] $[$ Self test disabled $]^{-} 1 \mathrm{~L} \mathrm{~S} \mathrm{~b}=4 \mathrm{mg}$ at 10 -bit representation, $\pm 2 \mathrm{~g}$ full scale.
7. After enabling the self-test, correct data is obtained after two samples (low-power mode / normal mode) or after eight samples (high-resolution mode).# 2.2 Temperature sensor characteristics 

Vdd $=2.5 \mathrm{~V}, \mathrm{~T}=25^{\circ} \mathrm{C}$ unless otherwise noted ${ }^{(b)}$
Table 5. Temperature sensor characteristics

| Symbol | Parameter | Test condition | Min. | Typ. $\left.{ }^{(1)}\right\|$ | Max. | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| TSDr | Temperature sensor output change vs <br> temperature |  |  | 1 |  | digit/ ${ }^{\circ} \mathrm{C}^{(2)}$ |
| TODR | Temperature refresh rate |  |  | ODR |  | Hz |
| Top | Operating temperature range |  | -40 |  | +85 | ${ }^{\circ} \mathrm{C}$ |

1. Typical specifications are not guaranteed.
2. 8 -bit resolution.

### 2.3 Electrical characteristics

Vdd $=2.5 \mathrm{~V}, \mathrm{~T}=25^{\circ} \mathrm{C}$ unless otherwise noted ${ }^{(6)}$
Table 6. Electrical characteristics

| Symbol | Parameter | Test conditions | Min. | Typ. ${ }^{(1)}$ | Max. | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| Vdd | Supply voltage |  | 1.71 | 2.5 | 3.6 | V |
| Vdd_IO | I/O pins supply voltage ${ }^{(2)}$ |  | 1.71 |  | Vdd+0.1 | V |
| Idd | Current consumption in normal mode | 50 Hz ODR |  | 11 |  | $\mu \mathrm{A}$ |
| Idd | Current consumption in normal mode | 1 Hz ODR |  | 2 |  | $\mu \mathrm{A}$ |
| IddLP | Current consumption in low-power mode | 50 Hz ODR |  | 6 |  | $\mu \mathrm{A}$ |
| IddPdn | Current consumption in power-down <br> mode |  |  | 0.5 |  | $\mu \mathrm{A}$ |
| VIH | Digital high-level input voltage |  | $0.8^{*}$ Vdd_IO |  |  | V |
| VIL | Digital low-level input voltage |  |  |  | $0.2^{*}$ Vdd_IO | V |
| VOH | High-level output voltage |  | $0.9^{*}$ Vdd_IO |  |  | V |
| VOL | Low-level output voltage |  |  |  | $0.1^{*}$ Vdd_IO | V |
| BW | System bandwidth ${ }^{(3)}$ |  |  | ODR/2 |  | Hz |
| Top | Operating temperature range |  | -40 |  | +85 | ${ }^{\circ} \mathrm{C}$ |

1. Typical specification are not guaranteed.
2. It is possible to remove Vdd maintaining Vdd_IO without blocking the communication busses, in this condition the measurement chain is powered off.
3. Refer to Table 25 for the ODR value and configuration.
b. The product is factory calibrated at 2.5 V . Temperature sensor operation is guaranteed in the range $2 \mathrm{~V}-3.6 \mathrm{~V}$.
c. The product is factory calibrated at 2.5 V . The operational power supply range is from 1.71 V to 3.6 V .# 2.4 Communication interface characteristics 

### 2.4.1 SPI - serial peripheral interface

Subject to general operating conditions for Vdd and Top.
Table 7. SPI slave timing values

| Symbol | Parameter | Value ${ }^{(1)}$ |  | Unit |
| :--: | :--: | :--: | :--: | :--: |
|  |  | Min | Max |  |
| $t_{c(\text { SPC })}$ | SPI clock cycle | 100 |  | ns |
| $f_{c(\text { SPC })}$ | SPI clock frequency |  | 10 | MHz |
| $t^{\text {su(CS) }}$ | CS setup time | 5 |  | ns |
| $t_{h(C S)}$ | CS hold time | 20 |  |  |
| $t_{\text {su(SI) }}$ | SDI input setup time | 5 |  |  |
| $t_{h(S I)}$ | SDI input hold time | 15 |  |  |
| $t_{v(S O)}$ | SDO valid output time |  | 50 |  |
| $t_{h(S O)}$ | SDO output hold time | 5 |  |  |
| $t_{\text {dis(SO) }}$ | SDO output disable time |  | 50 |  |

1. Values are guaranteed at 10 MHz clock frequency for SPI with both 4 and 3 wires, based on characterization results, not tested in production.

Figure 3. SPI slave timing diagram
![img-3.jpeg](img-3.jpeg)

1. When no communication is ongoing, data on SDO is driven by internal pull-up resistors.

Note: Measurement points are done at 0.2$\cdot$Vdd_IO and 0.8$\cdot$Vdd_IO, for both input and output ports.# 2.4.2 ${ }^{2} \mathrm{C}$ - Inter IC control interface 

Subject to general operating conditions for Vdd and top.
Table 8. $\mathrm{I}^{2} \mathrm{C}$ slave timing values

| Symbol | Parameter | $\mathrm{I}^{2} \mathrm{C}$ standard |  | $\mathrm{I}^{2} \mathrm{C}$ fast mode ${ }^{(1)}$ |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | Min | Max | Min | Max |  |
| $f_{(S C L)}$ | SCL clock frequency | 0 | 100 | 0 | 400 | kHz |
| $t_{w(S C L L)}$ | SCL clock low time | 4.7 |  | 1.3 |  | $\mu \mathrm{s}$ |
| $t_{w(S C L H)}$ | SCL clock high time | 4.0 |  | 0.6 |  |  |
| $t_{\text {su(SDA) }}$ | SDA setup time | 250 |  | 100 |  | ns |
| $t_{h(S D A)}$ | SDA data hold time | 0 | 3.45 | 0 | 0.9 | $\mu \mathrm{s}$ |
| $t_{h(S T)}$ | START condition hold time | 4 |  | 0.6 |  | $\mu \mathrm{s}$ |
| $t_{\text {su(SR) }}$ | Repeated START condition setup time | 4.7 |  | 0.6 |  |  |
| $t_{\text {su(SP) }}$ | STOP condition setup time | 4 |  | 0.6 |  |  |
| $t_{w(S P: S R)}$ | Bus free time between STOP and START condition | 4.7 |  | 1.3 |  |  |

1. Data based on standard $\mathrm{I}^{2} \mathrm{C}$ protocol requirement, not tested in production.

Figure 4. $\mathrm{I}^{2} \mathrm{C}$ slave timing diagram
![img-4.jpeg](img-4.jpeg)

Note:
Measurement points are done at 0.2$\cdot$Vdd_IO and 0.8$\cdot$Vdd_IO, for both ports.# 2.5 Absolute maximum ratings 

Stresses above those listed as "absolute maximum ratings" may cause permanent damage to the device. This is a stress rating only and functional operation of the device under these conditions is not implied. Exposure to maximum rating conditions for extended periods may affect device reliability.

Table 9. Absolute maximum ratings

| Symbol | Ratings | Maximum value | Unit |
| :--: | :--: | :--: | :--: |
| Vdd | Supply voltage | $-0.3$ to 4.8 | V |
| Vdd_IO | I/O pins Supply voltage | $-0.3$ to 4.8 | V |
| Vin | Input voltage on any control pin <br> (CS, SCL/SPC, SDA/SDI/SDO, SDO/SA0) | $-0.3$ to Vdd_IO $+0.3$ | V |
| $\mathrm{A}_{\text {POW }}$ | Acceleration (any axis, powered, Vdd $=2.5 \mathrm{~V}$ ) | 3000 g for 0.5 ms |  |
|  |  | 10000 g for 0.2 ms |  |
| $\mathrm{A}_{\text {UNP }}$ | Acceleration (any axis, unpowered) | 3000 g for 0.5 ms |  |
|  |  | 10000 g for 0.2 ms |  |
| $\mathrm{T}_{\mathrm{OP}}$ | Operating temperature range | -40 to +85 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\text {STG }}$ | Storage temperature range | -40 to +125 | ${ }^{\circ} \mathrm{C}$ |
| ESD | Electrostatic discharge protection | 2 (HBM) | kV |

Note: Supply voltage on any pin should never exceed 4.8 V
This device is sensitive to mechanical shock, improper handling can cause permanent damage to the part.

This device is sensitive to electrostatic discharge (ESD), improper handling can cause permanent damage to the part.# 3 Terminology and functionality 

### 3.1 Terminology

### 3.1.1 Sensitivity

Sensitivity describes the gain of the sensor and can be determined, for example, by applying 1 g acceleration to it. As the sensor can measure DC accelerations this can be done easily by pointing the axis of interest towards the center of the Earth, noting the output value, rotating the sensor by 180 degrees (pointing to the sky) and noting the output value again. By doing so, $\pm 1 \mathrm{~g}$ acceleration is applied to the sensor. Subtracting the larger output value from the smaller one, and dividing the result by 2 , leads to the actual sensitivity of the sensor. This value changes very little over temperature and also time. The sensitivity tolerance describes the range of sensitivities of a large population of sensors.

### 3.1.2 Zero-g level

The zero-g level offset (TyOff) describes the deviation of an actual output signal from the ideal output signal if no acceleration is present. A sensor in a steady state on a horizontal surface measures 0 g for the X -axis and 0 g for the Y -axis whereas the Z -axis measures 1 g . The output is ideally in the middle of the dynamic range of the sensor (content of OUT registers 00 h , data expressed as 2's complement number). A deviation from ideal value in this case is called Zero-g offset. Offset is to some extent a result of stress to MEMS sensor and therefore the offset can slightly change after mounting the sensor onto a printed circuit board or exposing it to extensive mechanical stress. Offset changes little over temperature, see Table 4 "Zero-g level change vs. temperature" (TCOff). The zero-g level tolerance (TyOff) describes the standard deviation of the range of zero-g levels of a population of sensors.

### 3.2 Functionality

### 3.2.1 High-resolution, normal mode, low-power mode

LIS3DH provides three different operating modes: high-resolution mode, normal mode and low-power mode.

The table below summarizes how to select the operating mode.
Table 10. Operating mode selection

| Operating mode | CTRL_REG1[3] <br> (LPen bit) | CTRL_REG4[3] <br> (HR bit) | BW [Hz] | Turn-on <br> time [ms] | So @ $\pm 2 g$ <br> $[\mathrm{mg} /$ digit $]$ |
| :-- | :--: | :--: | :--: | :--: | :--: |
| Low-power mode <br> (8-bit data output) | 1 | 0 | ODR/2 | 1 | 16 |
| Normal mode <br> (10-bit data output) | 0 | 0 | ODR/2 | 1.6 | 4 |
| High-resolution mode <br> (12-bit data output) | 0 | 1 | ODR/9 | 7/ODR | 1 |
| Not allowed | 1 | 1 | -- | -- | -- |The turn-on time to transition to another operating mode is given in Table 11.

Table 11. Turn-on time for operating mode transition

| Operating mode change | Turn-on time <br> $[\mathrm{ms}]$ |
| :-- | :--: |
| 12-bit mode to 8-bit mode | $1 / O D R$ |
| 12-bit mode to 10-bit mode | $1 / O D R$ |
| 10-bit mode to 8-bit mode | $1 / O D R$ |
| 10-bit mode to 12-bit mode | $7 / O D R$ |
| 8-bit mode to 10-bit mode | $1 / O D R$ |
| 8-bit mode to 12-bit mode | $7 / O D R$ |

Table 12. Current consumption of operating modes

| Operating mode $[\mathrm{Hz}]$ | Low-power mode <br> (8-bit data output) <br> $[\mu \mathrm{A}]$ | Normal mode <br> (10-bit data output) <br> $[\mu \mathrm{A}]$ | High resolution <br> (12-bit data output) <br> $[\mu \mathrm{A}]$ |
| :-- | :--: | :--: | :--: |
| 1 | 2 | 2 | 2 |
| 10 | 3 | 4 | 4 |
| 25 | 4 | 6 | 6 |
| 50 | 6 | 11 | 11 |
| 100 | 10 | 20 | 20 |
| 200 | 18 | 38 | 38 |
| 400 | 36 | 73 | 73 |
| 1344 | -- | 185 | 185 |
| 1620 | 100 | -- | -- |
| 5376 | 185 | -- | -- |

# 3.2.2 Self-test 

The self-test allows the user to check the sensor functionality without moving it. The self-test function is off when the self-test bit (ST) is programmed to ' 0 '. When the self-test bit is programmed to ' 1 ', an actuation force is applied to the sensor, simulating a definite input acceleration. In this case the sensor outputs exhibit a change in their DC levels which are related to the selected full scale through the device sensitivity. When the self-test is activated, the device output level is given by the algebraic sum of the signals produced by the acceleration acting on the sensor and by the electrostatic test-force. If the output signals change within the amplitude specified inside Table 4, then the sensor is working properly and the parameters of the interface chip are within the defined specifications.# 3.2.3 6D / 4D orientation detection 

The LIS3DH provides the capability to detect the orientation of the device in space, enabling easy implementation of energy-saving procedures and automatic image rotation for mobile devices.

The 4D detection is a subset of the 6D function especially defined to be implemented in mobile devices for portrait and landscape computation. In 4D configuration, the Z-axis position detection is disabled.

### 3.2.4 "Sleep-to-wake" and "Return-to-sleep"

The LIS3DH can be programmed to automatically switch to low-power mode upon recognition of a determined event.
Once the event condition is over, the device returns back to the preset normal or highresolution mode.

To enable this function the desired threshold value must be stored inside the ACT_THS (3Eh) register while the duration value is written inside the ACT_DUR (3Fh) register.

When the acceleration falls below the threshold value, the device automatically switches to low-power mode ( 10 Hz ODR).

During this condition, the ODR[3:0] bits and the LPen bit inside CTRL_REG1 (20h) and the HR bit in CTRL_REG4 (23h) are not considered.

As soon as the acceleration rises above threshold, the module restores the operating mode and ODRs as determined by the CTRL_REG1 (20h) and CTRL_REG4 (23h) settings.

### 3.3 Sensing element

A proprietary process is used to create a surface micro-machined accelerometer. The technology allows carrying out suspended silicon structures which are attached to the substrate in a few points called anchors and are free to move in the direction of the sensed acceleration. To be compatible with the traditional packaging techniques a cap is placed on top of the sensing element to avoid blocking the moving parts during the moulding phase of the plastic encapsulation.

When an acceleration is applied to the sensor the proof mass displaces from its nominal position, causing an imbalance in the capacitive half-bridge. This imbalance is measured using charge integration in response to a voltage pulse applied to the capacitor.

At steady state the nominal value of the capacitors are few pF and when an acceleration is applied the maximum variation of the capacitive load is in the fF range.

### 3.4 IC interface

The complete measurement chain is composed by a low-noise capacitive amplifier which converts the capacitive unbalancing of the MEMS sensor into an analog voltage that is finally available to the user by an analog-to-digital converter.

The acceleration data may be accessed through an $\mathrm{I}^{2} \mathrm{C} / \mathrm{SPI}$ interface thus making the device particularly suitable for direct interfacing with a microcontroller.The LIS3DH features a Data-Ready signal (DRDY) which indicates when a new set of measured acceleration data is available, thus simplifying data synchronization in the digital system that uses the device.

The LIS3DH may also be configured to generate an inertial wake-up and free-fall interrupt signal accordingly to a programmed acceleration event along the enabled axes. Both freefall and wake-up can be available simultaneously on two different pins.

# 3.5 Factory calibration 

The IC interface is factory calibrated for sensitivity (So) and Zero-g level (TyOff).
The trim values are stored inside the device in non-volatile memory. Any time the device is turned on, these values are downloaded into the registers to be used during active operation. This allows using the device without further calibration.

### 3.6 FIFO

The LIS3DH contains a 10-bit, 32-level FIFO. Buffered output allows 4 operation modes: FIFO, Stream, Stream-to-FIFO and FIFO bypass. When FIFO bypass mode is activated, FIFO is not operating and remains empty. In FIFO mode, measurement data from acceleration detection on the $x, y$, and $z$ axes are stored in the FIFO buffer.

### 3.7 Auxiliary ADC and temperature sensor

The LIS3DH contains an auxiliary ADC with 3 separate dedicated inputs: pins ADC1, ADC2, ADC3.

The user can retrieve the converted data from registers OUT_ADC1_L (08h), OUT_ADC1_H (09h), OUT_ADC2_L (0Ah), OUT_ADC2_H (0Bh) and OUT_ADC3_L (0Ch), OUT_ADC3_H (0Dh).

In order to use the auxiliary ADC , the user must set the BDU bit (bit 7) to 1 in CTRL_REG4 (23h) and the ADC_EN bit (bit 7) to 1 in TEMP_CFG_REG (1Fh). The ADC sampling frequency is the same as that of the ODR in CTRL_REG1 (20h).

The input range is $1200 \mathrm{mv} \pm 400 \mathrm{mV}$ and the data output is expressed in 2's complement left-aligned.

The ADC resolution is 10 bits if the LPen (bit 3) in CTRL_REG1 (20h) is cleared (highresolution / normal mode), otherwise, in low-power mode, the ADC resolution is 8-bit.

Channel 3 of the ADC can be connected to the temperature sensor by setting the TEMP_EN bit (bit 6) to 1 in TEMP_CFG_REG (1Fh). Refer to Table 5: Temperature sensor characteristics for the conversion factor.# 4 Application hints 

Figure 5. LIS3DH electrical connections
![img-5.jpeg](img-5.jpeg)

The device core is supplied through the Vdd line while the I/O pads are supplied through the Vdd_IO line. Power supply decoupling capacitors ( 100 nF ceramic, $10 \mu \mathrm{~F}$ aluminum) should be placed as near as possible to pin 14 of the device (common design practice).

All the voltage and ground supplies must be present at the same time to have proper behavior of the IC (refer to Figure 5). It is possible to remove Vdd maintaining Vdd_IO without blocking the communication bus, in this condition the measurement chain is powered off.

The functionality of the device and the measured acceleration data is selectable and accessible through the $\mathrm{I}^{2} \mathrm{C}$ or SPI interfaces. When using the $\mathrm{I}^{2} \mathrm{C}$, CS must be tied high.
ADC1, ADC2 \& ADC3 if not used can be left floating or connected to Vdd or GND.
The functions, the threshold and the timing of the two interrupt pins (INT1 and INT2) can be completely programmed by the user through the $\mathrm{I}^{2} \mathrm{C} / \mathrm{SPI}$ interface.Table 13. Internal pin status

| Pin\# | Name | Function | Pin status |
| :--: | :--: | :--: | :--: |
| 1 | Vdd_IO | Power supply for I/O pins |  |
| 2 | NC | Not connected |  |
| 3 | NC | Not connected |  |
| 4 | $\begin{aligned} & \text { SCL } \\ & \text { SPC } \end{aligned}$ | $\mathrm{I}^{2} \mathrm{C}$ serial clock (SCL) SPI serial port clock (SPC) | Default: input high impedance |
| 5 | GND | 0 V supply |  |
| 6 | $\begin{aligned} & \text { SDA } \\ & \text { SDI } \\ & \text { SDO } \end{aligned}$ | $\mathrm{I}^{2} \mathrm{C}$ serial data (SDA) <br> SPI serial data input (SDI) <br> 3-wire interface serial data output (SDO) | Default: (SDA) input high impedance |
| 7 | $\begin{aligned} & \text { SDO } \\ & \text { SA0 } \end{aligned}$ | SPI serial data output (SDO) <br> $I^{2} \mathrm{C}$ less significant bit of the device address (SA0) | Default: input with internal pull-up ${ }^{(1)}$ |
| 8 | CS | SPI enable <br> $I^{2} \mathrm{C} /$ SPI mode selection: <br> 1: SPI idle mode / $I^{2} \mathrm{C}$ communication enabled <br> 0: SPI communication mode / $I^{2} \mathrm{C}$ disabled | Default: input high impedance |
| 9 | INT2 | Inertial interrupt 2 | Default: push-pull output forced to GND |
| 10 | RES | Connect to GND |  |
| 11 | INT1 | Inertial interrupt 1 | Default: push-pull output forced to GND |
| 12 | GND | 0 V supply |  |
| 13 | ADC3 | Analog-to-digital converter input 3 | Default: input high impedance |
| 14 | Vdd | Power supply |  |
| 15 | ADC2 | Analog-to-digital converter input 2 | Default: input high impedance |
| 16 | ADC1 | Analog-to-digital converter input 1 | Default: input high impedance |

1. In order to disable the internal pull-up on the SDO/SA0 pin, write 90 h in CTRL_REG0 (1Eh).

# 4.1 Soldering information 

The LGA package is compliant with the ECOPACK ${ }^{\circledR}$, RoHS and "Green" standard. It is qualified for soldering heat resistance according to JEDEC J-STD-020.
Leave "Pin 1 Indicator" unconnected during soldering.
Land pattern and soldering recommendations are available at www.st.com.# 5 Digital main blocks 

### 5.1 FIFO

The LIS3DH embeds a 32-level FIFO for each of the three output channels, $X, Y$ and $Z$.
This allows consistent power saving for the system, since the host processor does not need to continuously poll data from the sensor, but it can wake up only when needed and burst the significant data out from the FIFO.

In order to enable the FIFO buffer, the FIFO_EN bit in CTRL_REG5 (24h) must be set to '1'.
This buffer can work according to the following different modes: Bypass mode, FIFO mode, Stream mode and Stream-to-FIFO mode. Each mode is selected by the FM [1:0] bits in FIFO_CTRL_REG (2Eh). Programmable FIFO watermark level, FIFO empty or FIFO overrun events can be enabled to generate dedicated interrupts on the INT1 pin (configuration through CTRL_REG3 (22h)).
In the FIFO_SRC_REG (2Fh) register the EMPTY bit is equal to ' 1 ' when all FIFO samples are ready and FIFO is empty.
In the FIFO_SRC_REG (2Fh) register the WTM bit goes to ' 1 ' if new data is written in the buffer and FIFO_SRC_REG (2Fh) (FSS [4:0]) is greater than or equal to FIFO_CTRL REG (2Eh) (FTH [4:0]). FIFO_SRC_REG (2Fh) (WTM) goes to ' 0 ' if reading an $X, Y, Z$ data slot from FIFO and FIFO_SRC_REG (2Fh) (FSS [4:0]) is less than or equal to FIFO_CTRL REG (2Eh) (FTH [4:0]).
In the FIFO_SRC_REG (2Fh) register the OVRN_FIFO bit is equal to ' 1 ' if the FIFO slot is overwritten.

### 5.1.1 Bypass mode

In Bypass mode the FIFO is not operational and for this reason it remains empty. For each channel only the first address is used. The remaining FIFO levels are empty.
Bypass mode must be used in order to reset the FIFO buffer when a different mode is operating (i.e. FIFO mode).

### 5.1.2 FIFO mode

In FIFO mode, the buffer continues filling data from the $X, Y$ and $Z$ accelerometer channels until it is full (a set of 32 samples stored). When the FIFO is full, it stops collecting data from the input channels and the FIFO content remains unchanged.
An overrun interrupt can be enabled, I1_OVERRUN = ' 1 ' in the CTRL_REG3 (22h) register, in order to be raised when the FIFO stops collecting data. When the overrun interrupt occurs, the first data has been overwritten and the FIFO stops collecting data from the input channels.

After the last read it is necessary to exit Bypass mode in order to reset the FIFO content. After this reset command, it is possible to restart FIFO mode just by selecting the FIFO mode configuration (FM[1:0] bits) in register FIFO_CTRL REG (2Eh).# 5.1.3 Stream mode 

In Stream mode the FIFO continues filling data from the $X, Y$, and $Z$ accelerometer channels until the buffer is full (a set of 32 samples stored) at which point the FIFO buffer index restarts from the beginning and older data is replaced by the current data. The oldest values continue to be overwritten until a read operation frees the FIFO slots.

An overrun interrupt can be enabled, I1_OVERRUN = '1' in the CTRL_REG3 (22h) register, in order to read the entire contents of the FIFO at once. If, in the application, it is mandatory not to lose data and it is not possible to read at least one sample for each axis within one ODR period, a watermark interrupt can be enabled in order to read partially the FIFO and leave memory slots free for incoming data.

Setting the FTH [4:0] bit in the FIFO_CTRL_REG (2Eh) register to an N value, the number of $X, Y$ and $Z$ data samples that should be read at the rise of the watermark interrupt is up to $(\mathrm{N}+1)$.

### 5.1.4 Stream-to-FIFO mode

In Stream-to-FIFO mode, data from the $X, Y$ and $Z$ accelerometer channels are collected in a combination of Stream mode and FIFO mode. The FIFO buffer starts operating in Stream mode and switches to FIFO mode when the selected interrupt occurs.

The FIFO operating mode changes according to the INT1 pin value if the TR bit is set to ' 0 ' in the FIFO_CTRL_REG (2Eh) register or the INT2 pin value if the TR bit is set to'1' in the FIFO_CTRL_REG (2Eh) register.

When the interrupt pin is selected and the interrupt event is configured on the corresponding pin, the FIFO operates in Stream mode if the pin value is equal to ' 0 ' and it operates in FIFO mode if the pin value is equal to ' 1 '. Switching modes is dynamically performed according to the pin value.

Stream-to-FIFO can be used in order to analyze the sampling history that generates an interrupt. The standard operation is to read the contents of FIFO when the FIFO mode is triggered and the FIFO buffer is full and stopped.

### 5.1.5 Retrieving data from FIFO

FIFO data is read from OUT_X_L (28h), OUT_X_H (29h), OUT_Y_L (2Ah), OUT_Y_H (2Bh) and OUT_Z_L (2Ch), OUT_Z_H (2Dh). When the FIFO is in Stream, Stream-to-FIFO or FIFO mode, a read operation from the OUT_X_L (28h), OUT_X_H (29h), OUT_Y_L (2Ah), OUT_Y_H (2Bh) or OUT_Z_L (2Ch), OUT_Z_H (2Dh) registers provides the data stored in the FIFO. Each time data is read from the FIFO, the oldest $X, Y$ and $Z$ data are placed in the OUT_X_L (28h), OUT_X_H (29h), OUT_Y_L (2Ah), OUT_Y_H (2Bh) and OUT_Z_L (2Ch), OUT_Z_H (2Dh) registers and both single read and read burst operations can be used.

The address to be read is automatically updated by the device and it rolls back to $0 \times 28$ when register $0 \times 2 \mathrm{D}$ is reached. In order to read all FIFO levels in a multiple byte read, 192 bytes ( 6 output registers of 32 levels) have to be read.# 6 Digital interfaces 

The registers embedded inside the LIS3DH may be accessed through both the $\mathrm{I}^{2} \mathrm{C}$ and SPI serial interfaces. The latter may be SW configured to operate either in 3-wire or 4-wire interface mode.

The serial interfaces are mapped onto the same pads. To select/exploit the $\mathrm{I}^{2} \mathrm{C}$ interface, the CS line must be tied high (i.e. connected to Vdd_IO).

Table 14. Serial interface pin description

| Pin name | Pin description |
| :--: | :-- |
| CS | SPI enable <br> $I^{2} \mathrm{C} / \mathrm{SPI}$ mode selection: <br> 1: SPI idle mode / $I^{2} \mathrm{C}$ communication enabled <br> 0: SPI communication mode / $I^{2} \mathrm{C}$ disabled |
| SCL <br> SPC | $I^{2} \mathrm{C}$ serial clock (SCL) <br> SPI serial port clock (SPC) |
| SDA <br> SDI <br> SDO | $I^{2} \mathrm{C}$ serial data (SDA) <br> SPI serial data input (SDI) <br> 3-wire interface serial data output (SDO) |
| SA0 <br> SDO | $I^{2} \mathrm{C}$ less significant bit of the device address (SA0) <br> SPI serial data output (SDO) |

### 6.1 $\mathrm{I}^{2} \mathrm{C}$ serial interface

The LIS3DH $I^{2} \mathrm{C}$ is a bus slave. The $I^{2} \mathrm{C}$ is employed to write data into registers whose content can also be read back.

The relevant $I^{2} \mathrm{C}$ terminology is given in the table below.
Table 15. $\mathrm{I}^{2} \mathrm{C}$ terminology

| Term | Description |
| :--: | :-- |
| Transmitter | The device which sends data to the bus |
| Receiver | The device which receives data from the bus |
| Master | The device which initiates a transfer, generates clock signals and terminates a <br> transfer |
| Slave | The device addressed by the master |

There are two signals associated with the $I^{2} \mathrm{C}$ bus: the serial clock line (SCL) and the Serial DAta line (SDA). The latter is a bidirectional line used for sending and receiving the data to/from the interface. Both the lines must be connected to Vdd_IO through external pull-up resistor. When the bus is free both the lines are high.
The $I^{2} \mathrm{C}$ interface is compliant with fast mode $(400 \mathrm{kHz}) \mathrm{I}^{2} \mathrm{C}$ standards as well as with normal mode.# 6.1.1 $\mathrm{I}^{2} \mathrm{C}$ operation 

The transaction on the bus is started through a START (ST) signal. A START condition is defined as a HIGH-to-LOW transition on the data line while the SCL line is held HIGH. After this has been transmitted by the Master, the bus is considered busy. The next byte of data transmitted after the start condition contains the address of the slave in the first 7 bits and the eighth bit tells whether the Master is receiving data from the slave or transmitting data to the slave. When an address is sent, each device in the system compares the first seven bits after a start condition with its address. If they match, the device considers itself addressed by the Master.

The Slave ADdress (SAD) associated to the LIS3DH is 001100xb. The SDO/SA0 pad can be used to modify the less significant bit of the device address. If the SA0 pad is connected to the voltage supply, LSb is ' 1 ' (address 0011001 b ) else if SA0 pad is connected to ground, the LSb value is ' 0 ' (address 0011000b). This solution permits to connect and address two different accelerometers to the same $\mathrm{I}^{2} \mathrm{C}$ lines.

Data transfer with acknowledge is mandatory. The transmitter must release the SDA line during the acknowledge pulse. The receiver must then pull the data line LOW so that it remains stable low during the HIGH period of the acknowledge clock pulse. A receiver which has been addressed is obliged to generate an acknowledge after each byte of data received.

The $\mathrm{I}^{2} \mathrm{C}$ embedded inside the LIS3DH behaves like a slave device and the following protocol must be adhered to. After the start condition (ST) a slave address is sent, once a slave acknowledge (SAK) has been returned, an 8-bit sub-address (SUB) is transmitted: the 7 LSb represent the actual register address while the MSB enables address auto increment. If the MSb of the SUB field is ' 1 ', the SUB (register address) is automatically increased to allow multiple data read/writes.

The slave address is completed with a Read/Write bit. If the bit was ' 1 ' (Read), a repeated START (SR) condition must be issued after the two sub-address bytes; if the bit is ' 0 ' (Write) the Master transmit to the slave with direction unchanged. Table 16 explains how the SAD+Read/Write bit pattern is composed, listing all the possible configurations.

Table 16. SAD+Read/Write patterns

| Command | SAD $[6: 1]$ | SAD[0] = SA0 | R/W | SAD+R/W |
| :--: | :--: | :--: | :--: | :--: |
| Read | 001100 | 0 | 1 | 00110001 (31h) |
| Write | 001100 | 0 | 0 | 00110000 (30h) |
| Read | 001100 | 1 | 1 | 00110011 (33h) |
| Write | 001100 | 1 | 0 | 00110010 (32h) |

Table 17. Transfer when master is writing one byte to slave

| Master | ST | SAD + W |  | SUB |  | DATA |  | SP |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Slave |  |  | SAK |  | SAK |  | SAK |  |

Table 18. Transfer when master is writing multiple bytes to slave

| Master | ST | SAD + W |  | SUB |  | DATA |  | DATA |  | SP |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Slave |  |  | SAK |  | SAK |  | SAK |  | SAK |  |Table 19. Transfer when master is receiving (reading) one byte of data from slave

| Master | ST | SAD + W |  | SUB |  | SR | SAD + R |  |  | NMAK | SP |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |
| Slave |  |  | SAK |  | SAK |  |  | SAK | DATA |  |  |

Table 20. Transfer when master is receiving (reading) multiple bytes of data from slave

| Master | ST | SAD+W |  | SUB |  | SR | SAD+R |  |  | MAK |  | MAK |  | NMAK | SP |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |
| Slave |  |  | SAK |  | SAK |  |  | SAK | DATA |  | DATA |  | DATA |  |  |

Data are transmitted in byte format (DATA). Each data transfer contains 8 bits. The number of bytes transferred per transfer is unlimited. Data is transferred with the Most Significant bit (MSb) first. If a receiver can't receive another complete byte of data until it has performed some other function, it can hold the clock line, SCL LOW to force the transmitter into a wait state. Data transfer only continues when the receiver is ready for another byte and releases the data line. If a slave receiver doesn't acknowledge the slave address (i.e. it is not able to receive because it is performing some real-time function) the data line must be left HIGH by the slave. The Master can then abort the transfer. A LOW-to-HIGH transition on the SDA line while the SCL line is HIGH is defined as a STOP condition. Each data transfer must be terminated by the generation of a STOP (SP) condition.

In order to read multiple bytes, it is necessary to assert the most significant bit of the subaddress field. In other words, SUB(7) must be equal to 1 while SUB(6-0) represents the address of first register to be read.

In the presented communication format MAK is Master acknowledge and NMAK is No Master Acknowledge.# 6.2 SPI bus interface 

The LIS3DH SPI is a bus slave. The SPI allows writing to and reading from the registers of the device.

The serial interface interacts with the application using 4 wires: CS, SPC, SDI and SDO.
Figure 6. Read and write protocol
![img-6.jpeg](img-6.jpeg)

CS is the serial port enable and it is controlled by the SPI master. It goes low at the start of the transmission and goes back high at the end. SPC is the serial port clock and it is controlled by the SPI master. It is stopped high when CS is high (no transmission). SDI and SDO are respectively the serial port data input and output. Those lines are driven at the falling edge of SPC and should be captured at the rising edge of SPC.
Both the read register and write register commands are completed in 16 clock pulses or in multiples of 8 in case of multiple read/write bytes. Bit duration is the time between two falling edges of SPC. The first bit (bit 0 ) starts at the first falling edge of SPC after the falling edge of CS while the last bit (bit 15, bit 23, ...) starts at the last falling edge of SPC just before the rising edge of CS.
bit 0: $\mathrm{R} \overline{\mathrm{W}}$ bit. When 0 , the data $\mathrm{DI}(7: 0)$ is written into the device. When 1 , the data $\mathrm{DO}(7: 0)$ from the device is read. In latter case, the chip drives SDO at the start of bit 8.
bit 1: $\mathrm{M} \overline{\mathrm{S}}$ bit. When 0 , the address remains unchanged in multiple read/write commands. When 1, the address is auto incremented in multiple read/write commands.
bit 2-7: address $\mathrm{AD}(5: 0)$. This is the address field of the indexed register.
bit 8-15: data DI(7:0) (write mode). This is the data that is written into the device (MSb first).
bit 8-15: data $\mathrm{DO}(7: 0)$ (read mode). This is the data that is read from the device (MSb first).
In multiple read/write commands further blocks of 8 clock periods are added. When the $\mathrm{M} \overline{\mathrm{S}}$ bit is ' 0 ', the address used to read/write data remains the same for every block. When the $\mathrm{M} \overline{\mathrm{S}}$ bit is ' 1 ', the address used to read/write data is increased at every block.

The function and the behavior of SDI and SDO remain unchanged.# 6.2.1 SPI read 

Figure 7. SPI read protocol
![img-7.jpeg](img-7.jpeg)

The SPI read command is performed with 16 clock pulses. A multiple byte read command is performed by adding blocks of 8 clock pulses to the previous one.
bit 0: READ bit. The value is 1.
bit 1: $\mathrm{M} \overline{\mathrm{S}}$ bit. When 0 , does not increment the address; when 1 , increments the address in multiple reads.
bit 2-7: address AD(5:0). This is the address field of the indexed register.
bit 8-15: data DO(7:0) (read mode). This is the data that is read from the device (MSb first).
bit 16-... : data DO(...)8). Further data in multiple byte reads.
Figure 8. Multiple byte SPI read protocol (2-byte example)
![img-8.jpeg](img-8.jpeg)# 6.2.2 SPI write 

Figure 9. SPI write protocol
![img-9.jpeg](img-9.jpeg)

The SPI write command is performed with 16 clock pulses. A multiple byte write command is performed by adding blocks of 8 clock pulses to the previous one.
bit 0: WRITE bit. The value is 0 .
bit 1: $\overline{\mathrm{MS}}$ bit. When 0 , does not increment the address; when 1 , increments the address in multiple writes.
bit 2 -7: address AD(5:0). This is the address field of the indexed register.
bit 8-15: data DI(7:0) (write mode). This is the data that is written inside the device (MSb first).
bit 16-... : data DI(...) 8). Further data in multiple byte writes.
Figure 10. Multiple byte SPI write protocol (2-byte example)
![img-10.jpeg](img-10.jpeg)# 6.2.3 SPI read in 3-wire mode 

3-wire mode is entered by setting the bit SIM (SPI serial interface mode selection) to ' 1 ' in CTRL_REG4 (23h).

Figure 11. SPI read protocol in 3-wire mode
![img-11.jpeg](img-11.jpeg)

The SPI read command is performed with 16 clock pulses:
bit 0: READ bit. The value is 1.
bit 1: $\overline{\mathrm{M}} \overline{\mathrm{S}}$ bit. When 0 , does not increment the address; when 1 , increments the address in multiple reads.
bit 2-7: address AD(5:0). This is the address field of the indexed register.
bit 8-15: data DO(7:0) (read mode). This is the data that is read from the device (MSb first).
The multiple read command is also available in 3-wire mode.# 7 Register mapping 

The table given below provides a list of the 8 -bit registers embedded in the device and the corresponding addresses.

Table 21. Register address map

| Name | Type | Register address |  | Default | Comment |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | Hex | Binary |  |  |
| Reserved (do not modify) |  | $00-06$ |  |  | Reserved |
| STATUS_REG_AUX | r | 07 | 0000111 | Output |  |
| OUT_ADC1_L | r | 08 | 0001000 | Output |  |
| OUT_ADC1_H | r | 09 | 0001001 | Output |  |
| OUT_ADC2_L | r | 0A | 0001010 | Output |  |
| OUT_ADC2_H | r | 0B | 0001011 | Output |  |
| OUT_ADC3_L | r | 0C | 0001100 | Output |  |
| OUT_ADC3_H | r | 0D | 0001101 | Output |  |
| Reserved (do not modify) |  | 0E |  |  | Reserved |
| WHO_AM_I | r | 0F | 0001111 | 00110011 | Dummy register |
| Reserved (do not modify) |  | $10-1 D$ |  |  | Reserved |
| CTRL_REG0 | rw | 1E | 0011110 | 00010000 |  |
| TEMP_CFG_REG | rw | 1F | 0011111 | 00000000 |  |
| CTRL_REG1 | rw | 20 | 0100000 | 00000111 |  |
| CTRL_REG2 | rw | 21 | 0100001 | 00000000 |  |
| CTRL_REG3 | rw | 22 | 0100010 | 00000000 |  |
| CTRL_REG4 | rw | 23 | 0100011 | 00000000 |  |
| CTRL_REG5 | rw | 24 | 0100100 | 00000000 |  |
| CTRL_REG6 | rw | 25 | 0100101 | 00000000 |  |
| REFERENCE | rw | 26 | 0100110 | 00000000 |  |
| STATUS_REG | r | 27 | 0100111 | Output |  |
| OUT_X_L | r | 28 | 0101000 | Output |  |
| OUT_X_H | r | 29 | 0101001 | Output |  |
| OUT_Y_L | r | 2A | 0101010 | Output |  |
| OUT_Y_H | r | 2B | 0101011 | Output |  |
| OUT_Z_L | r | 2C | 0101100 | Output |  |
| OUT_Z_H | r | 2D | 0101101 | Output |  |
| FIFO_CTRL_REG | rw | 2E | 0101110 | 00000000 |  |
| FIFO_SRC_REG | r | 2F | 0101111 | Output |  |Table 21. Register address map

| Name | Type | Register address |  | Default | Comment |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | Hex | Binary |  |  |
| INT1_CFG | rw | 30 | 0110000 | 00000000 |  |
| INT1_SRC | $r$ | 31 | 0110001 | Output |  |
| INT1_THS | rw | 32 | 0110010 | 00000000 |  |
| INT1_DURATION | rw | 33 | 0110011 | 00000000 |  |
| INT2_CFG | rw | 34 | 0110100 | 00000000 |  |
| INT2_SRC | $r$ | 35 | 0110101 | Output |  |
| INT2_THS | rw | 36 | 0110110 | 00000000 |  |
| INT2_DURATION | rw | 37 | 0110111 | 00000000 |  |
| CLICK_CFG | rw | 38 | 0111000 | 00000000 |  |
| CLICK_SRC | $r$ | 39 | 0111001 | Output |  |
| CLICK_THS | rw | 3A | 0111010 | 00000000 |  |
| TIME_LIMIT | rw | 3B | 0111011 | 00000000 |  |
| TIME_LATENCY | rw | 3C | 0111100 | 00000000 |  |
| TIME_WINDOW | rw | 3D | 0111101 | 00000000 |  |
| ACT_THS | rw | 3E | 0111110 | 00000000 |  |
| ACT_DUR | rw | 3F | 0111111 | 00000000 |  |

Registers marked as Reserved or not listed in the table above must not be changed. Writing to those registers may cause permanent damage to the device.

The content of the registers that are loaded at boot should not be changed. They contain the factory calibration values. Their content is automatically restored when the device is powered up.

The boot procedure is complete about 5 milliseconds after device power-up.# 8 Registers description 

### 8.1 STATUS_REG_AUX (07h)

Table 22. STATUS_REG_AUX register

| 321OR | 3OR | 2OR | 1OR | 321DA | 3DA | 2DA | 1DA |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 23. STATUS_REG_AUX description

| 321OR | 1, 2 and 3-axis data overrun. Default value: 0 <br> (0: no overrun has occurred; 1: a new set of data has overwritten the previous set) |
| :-- | :-- |
| 3OR | 3-axis data overrun. Default value: 0 <br> (0: no overrun has occurred; <br> 1: new data for the 3-axis has overwritten the previous data) |
| 2OR | 2-axis data overrun. Default value: 0 <br> (0: no overrun has occurred; <br> 1: new data for the 2-axis has overwritten the previous data) |
| 1OR | 1-axis data overrun. Default value: 0 <br> (0: no overrun has occurred; <br> 1: new data for the 1-axis has overwritten the previous data) |
| 321DA | 1, 2 and 3-axis new data available. Default value: 0 <br> (0: a new set of data is not yet available; 1: a new set of data is available) |
| 3DA | 3-axis new data available. Default value: 0 <br> (0: new data for the 3-axis is not yet available; <br> 1: new data for the 3-axis is available) |
| 2DA | 2 -axis new data available. Default value: 0 <br> (0: new data for the 2-axis is not yet available; <br> 1: new data for the 2-axis is available) |
| 1DA | 1-axis new data available. Default value: 0 <br> (0: new data for the 1-axis is not yet available; <br> 1: new data for the 1-axis is available) |

### 8.2 OUT_ADC1_L (08h), OUT_ADC1_H (09h)

Auxiliary 10-bit ADC channel 1 conversion. For auxiliary ADC setting refer to Section 3.7: Auxiliary ADC and temperature sensor.

### 8.3 OUT_ADC2_L (0Ah), OUT_ADC2_H (0Bh)

Auxiliary 10-bit ADC channel 2 conversion. For auxiliary ADC setting refer to Section 3.7: Auxiliary ADC and temperature sensor.

### 8.4 OUT_ADC3_L (0Ch), OUT_ADC3_H (0Dh)

Auxiliary 10-bit ADC channel 3 conversion or temperature sensor data output. Refer to Section 3.7: Auxiliary ADC and temperature sensor.# 8.5 WHO_AM_I (0Fh) 

Table 24. WHO_AM_I register

| 0 | 0 | 1 | 1 | 0 | 0 | 1 | 1 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Device identification register.

### 8.6 CTRL_REG0 (1Eh)

Table 25. CTRL_REG0 register

| SDO_PU_DISC | $0^{(1)}$ | $0^{(1)}$ | $1^{(2)}$ | $0^{(1)}$ | $0^{(1)}$ | $0^{(1)}$ | $0^{(1)}$ |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

1. This bit must be set to 0 for correct operation of the device.
2. This bit must be set to 1 for correct operation of the device.

Table 26. CTRL_REG0 description

| SDO_PU_DISC | Disconnect SDO/SA0 pull-up. Default value: 00010000 <br> (0: pull-up connected to SDO/SA0 pin; <br> 1: pull-up disconnected to SDO/SA0 pin) |
| :-- | :-- |

Note: Leave bits 0 through 6 at the default value in order to ensure correct operation of the device.

### 8.7 TEMP_CFG_REG (1Fh)

Table 27. TEMP_CFG_REG register

| ADC_EN | TEMP_EN | 0 | 0 | 0 | 0 | 0 | 0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 28. TEMP_CFG_REG description

| TEMP_EN | Temperature sensor (T) enable. Default value: 0 <br> (0: T disabled; 1: T enabled) |
| :-- | :-- |
| ADC_EN | ADC enable. Default value: 0 <br> (0: ADC disabled; 1: ADC enabled) |# 8.8 CTRL_REG1 (20h) 

Table 29. CTRL_REG1 register

| ODR3 | ODR2 | ODR1 | ODR0 | LPen | Zen | Yen | Xen |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 30. CTRL_REG1 description

| ODR[3:0] | Data rate selection. Default value: 0000 <br> (0000: power-down mode; others: Refer to Table 31: Data rate configuration) |
| :--: | :-- |
| LPen | Low-power mode enable. Default value: 0 <br> (0: high-resolution mode / normal mode, 1: low-power mode) <br> (Refer to section Section 3.2.1: High-resolution, normal mode, low-power mode) |
| Zen | Z-axis enable. Default value: 1 <br> (0: Z-axis disabled; 1: Z-axis enabled) |
| Yen | Y-axis enable. Default value: 1 <br> (0: Y-axis disabled; 1: Y-axis enabled) |
| Xen | X-axis enable. Default value: 1 <br> (0: X-axis disabled; 1: X-axis enabled) |

ODR[3:0] is used to set the power mode and ODR selection. The following table indicates the frequency of each combination of ODR[3:0].

Table 31. Data rate configuration

| ODR3 | ODR2 | ODR1 | ODR0 | Power mode selection |
| :-- | :-- | :-- | :-- | :-- |
| 0 | 0 | 0 | 0 | Power-down mode |
| 0 | 0 | 0 | 1 | HR / Normal / Low-power mode $(1 \mathrm{~Hz})$ |
| 0 | 0 | 1 | 0 | HR / Normal / Low-power mode $(10 \mathrm{~Hz})$ |
| 0 | 0 | 1 | 1 | HR / Normal / Low-power mode $(25 \mathrm{~Hz})$ |
| 0 | 1 | 0 | 0 | HR / Normal / Low-power mode $(50 \mathrm{~Hz})$ |
| 0 | 1 | 0 | 1 | HR / Normal / Low-power mode $(100 \mathrm{~Hz})$ |
| 0 | 1 | 1 | 0 | HR / Normal / Low-power mode $(200 \mathrm{~Hz})$ |
| 0 | 1 | 1 | 1 | HR / Normal / Low-power mode $(400 \mathrm{~Hz})$ |
| 1 | 0 | 0 | 0 | Low power mode $(1.60 \mathrm{kHz})$ |
| 1 | 0 | 0 | 1 | HR / normal $(1.344 \mathrm{kHz})$ <br> Low-power mode $(5.376 \mathrm{kHz})$ |# 8.9 CTRL_REG2 (21h) 

Table 32. CTRL_REG2 register

| HPM1 | HPM0 | HPCF2 | HPCF1 | FDS | HPCLICK | HP_IA2 | HP_IA1 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 33. CTRL_REG2 description

| HPM[1:0] | High-pass filter mode selection. Default value: 00 <br> Refer to Table 34: High-pass filter mode configuration |
| :-- | :-- |
| HPCF[2:1] | High-pass filter cutoff frequency selection |
| FDS | Filtered data selection. Default value: 0 <br> (0: internal filter bypassed; <br> 1: data from internal filter sent to output register and FIFO) |
| HPCLICK | High-pass filter enabled for CLICK function. <br> (0: filter bypassed; 1: filter enabled) |
| HP_IA2 | High-pass filter enabled for AOI function on interrupt 2, <br> (0: filter bypassed; 1: filter enabled) |
| HP_IA1 | High-pass filter enabled for AOI function on interrupt 1, <br> (0: filter bypassed; 1: filter enabled) |

Table 34. High-pass filter mode configuration

| HPM1 | HPM0 | High-pass filter mode |
| :-- | :-- | :-- |
| 0 | 0 | Normal mode (reset by reading REFERENCE (26h)) |
| 0 | 1 | Reference signal for filtering |
| 1 | 0 | Normal mode |
| 1 | 1 | Autoreset on interrupt event |

### 8.10 CTRL_REG3 (22h)

Table 35. CTRL_REG3 register

| I1_CLICK | I1_IA1 | I1_IA2 | I1_ZYXDA | I1_321DA | I1_WTM | I1_OVERRUN | -- |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 36. CTRL_REG3 description

| I1_CLICK | Click interrupt on INT1. Default value: 0 <br> (0: disable; 1: enable) |
| :-- | :-- |
| I1_IA1 | IA1 interrupt on INT1. Default value: 0 <br> (0: disable; 1: enable) |
| I1_IA2 | IA2 interrupt on INT1. Default value: 0 <br> (0: disable; 1: enable) |
| I1_ZYXDA | ZYXDA interrupt on INT1. Default value: 0 <br> (0: disable; 1: enable) |
| I1_321DA | 321DA interrupt on INT1. Default value: 0 <br> (0: disable; 1: enable) |
| I1_WTM | FIFO watermark interrupt on INT1. Default value: 0 <br> (0: disable; 1: enable) |
| I1_OVERRUN | FIFO overrun interrupt on INT1. Default value: 0 <br> (0: disable; 1: enable) |# 8.11 CTRL_REG4 (23h) 

Table 37. CTRL_REG4 register

| BDU | BLE $^{(1)}$ | FS1 | FS0 | HR | ST1 | ST0 | SIM |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

1. The BLE function can be activated only in high-resolution mode.

Table 38. CTRL_REG4 description

| BDU | Block data update. Default value: 0 <br> (0: continuous update; <br> 1: output registers not updated until MSB and LSB reading) |
| :-- | :-- |
| BLE | Big/little endian data selection. Default value 0. <br> (0: Data LSB @ lower address; 1: Data MSB @ lower address) |
| FS[1:0] | Full-scale selection. default value: 00 <br> (00: $\pm 2 \mathrm{~g} ; 01: \pm 4 \mathrm{~g} ; 10: \pm 8 \mathrm{~g} ; 11: \pm 16 \mathrm{~g}$ ) |
| HR | High-resolution output mode: Default value: 0 <br> (0: high-resolution disabled; 1: high-resolution enabled) |
| ST[1:0] | Self-test enable. Default value: 00 <br> (00: self-test disabled; other: See Table 39) |
| SIM | SPI serial interface mode selection. Default value: 0 <br> (0: 4-wire interface; 1: 3-wire interface) |

Table 39. Self-test mode configuration

| ST1 | ST0 | Self test mode |
| :-- | :-- | :-- |
| 0 | 0 | Normal mode |
| 0 | 1 | Self-test 0 |
| 1 | 0 | Self-test 1 |
| 1 | 1 | - |# 8.12 CTRL_REG5 (24h) 

Table 40. CTRL_REG5 register

| BOOT | FIFO_EN | -- | -- | LIR_INT1 | D4D_INT1 | LIR_INT2 | D4D_INT2 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 41. CTRL_REG5 description

| BOOT | Reboot memory content. Default value: 0 <br> (0: normal mode; 1: reboot memory content) |
| :-- | :-- |
| FIFO_EN | FIFO enable. Default value: 0 <br> (0: FIFO disable; 1: FIFO enable) |
| LIR_INT1 | Latch interrupt request on INT1_SRC register, with INT1_SRC (31h) register <br> cleared by reading INT1_SRC (31h) itself. Default value: 0. <br> (0: interrupt request not latched; 1: interrupt request latched) |
| D4D_INT1 | 4D enable: 4D detection is enabled on INT1 when 6D bit on INT1_CFG is set <br> to 1. |
| LIR_INT2 | Latch interrupt request on INT2_SRC (35h) register, with INT2_SRC (35h) <br> register cleared by reading INT2_SRC (35h) itself. Default value: 0 <br> (0: interrupt request not latched; 1: interrupt request latched) |
| D4D_INT2 | 4D enable: 4D detection is enabled on INT2 pin when 6D bit on INT2_CFG <br> (34h) is set to 1. |

### 8.13 CTRL_REG6 (25h)

Table 42. CTRL_REG6 register

| I2_CLICK | I2_IA1 | I2_IA2 | I2_BOOT | I2_ACT | -- | INT_POLARITY | - |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 43. CTRL_REG6 description

| I2_CLICK | Click interrupt on INT2 pin. Default value: 0 <br> (0: disabled; 1: enabled) |
| :-- | :-- |
| I2_IA1 | Enable interrupt 1 function on INT2 pin. Default value: 0 <br> (0: function disabled; 1: function enabled) |
| I2_IA2 | Enable interrupt 2 function on INT2 pin. Default value: 0 <br> (0: function disabled; 1: function enabled) |
| I2_BOOT | Enable boot on INT2 pin. Default value: 0 <br> (0: disabled; 1:enabled) |
| I2_ACT | Enable activity interrupt on INT2 pin. Default value: 0 <br> (0: disabled; 1:enabled) |
| INT_POLARITY | INT1 and INT2 pin polarity. Default value: 0 <br> (0: active-high; 1: active-low) |# 8.14 REFERENCE (26h) 

Table 44. REFERENCE register

| Ref7 | Ref6 | Ref5 | Ref4 | Ref3 | Ref2 | Ref1 | Ref0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 45. REFERENCE register description

| Ref[7:0] | Reference value for Interrupt generation. Default value: 00000000 |
| :-- | :-- |

### 8.15 STATUS_REG (27h)

Table 46. STATUS register

| ZYXOR | ZOR | YOR | XOR | ZYXDA | ZDA | YDA | XDA |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 47. STATUS register description

| ZYXOR | X, Y and Z-axis data overrun. Default value: 0 <br> (0: no overrun has occurred; 1: a new set of data has overwritten the previous set) |
| :-- | :-- |
| ZOR | Z-axis data overrun. Default value: 0 <br> (0: no overrun has occurred; <br> 1: a new data for the Z-axis has overwritten the previous data) |
| YOR | Y-axis data overrun. Default value: 0 <br> (0: no overrun has occurred; <br> 1: new data for the Y-axis has overwritten the previous data) |
| XOR | X-axis data overrun. Default value: 0 <br> (0: no overrun has occurred; <br> 1: new data for the X-axis has overwritten the previous data) |
| ZYXDA | X, Y and Z-axis new data available. Default value: 0 <br> (0: a new set of data is not yet available; 1: a new set of data is available) |
| ZDA | Z-axis new data available. Default value: 0 <br> (0: new data for the Z-axis is not yet available; <br> 1: new data for the Z-axis is available) |
| YDA | Y-axis new data available. Default value: 0 <br> (0: new data for the Y-axis is not yet available; <br> 1: new data for the Y-axis is available) |# 8.16 OUT_X_L (28h), OUT_X_H (29h) 

X-axis acceleration data. The value is expressed as two's complement left-justified. Please refer to Section 3.2.1: High-resolution, normal mode, low-power mode.

### 8.17 OUT_Y_L (2Ah), OUT_Y_H (2Bh)

Y-axis acceleration data. The value is expressed as two's complement left-justified. Please refer to Section 3.2.1: High-resolution, normal mode, low-power mode.

### 8.18 OUT_Z_L (2Ch), OUT_Z_H (2Dh)

Z-axis acceleration data. The value is expressed as two's complement left-justified. Please refer to Section 3.2.1: High-resolution, normal mode, low-power mode.

### 8.19 FIFO_CTRL_REG (2Eh)

Table 48. REFERENCE register

| FM1 | FM0 | TR | FTH4 | FTH3 | FTH2 | FTH1 | FTH0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 49. REFERENCE register description

| FM[1:0] | FIFO mode selection. Default value: 00 (see Table 50) |
| :-- | :-- |
| TR | Trigger selection. Default value: 0 <br> (0: trigger event allows triggering signal on INT1 <br> 1: trigger event allows triggering signal on INT2) |
| FTH[4:0] | Default value: 00000 |

Table 50. FIFO mode configuration

| FM1 | FM0 | Self test mode |
| :-- | :-- | :-- |
| 0 | 0 | Bypass mode |
| 0 | 1 | FIFO mode |
| 1 | 0 | Stream mode |
| 1 | 1 | Stream-to-FIFO |# 8.20 FIFO_SRC_REG (2Fh) 

Table 51. FIFO_SRC_REG register

| WTM | OVRN_FIFO | EMPTY | FSS4 | FSS3 | FSS2 | FSS1 | FSS0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 52. FIFO_SRC_REG description

| WTM | WTM bit is set high when FIFO content exceeds watermark level |
| :-- | :-- |
| OVRN_FIFO | OVRN bit is set high when FIFO buffer is full; this means that the FIFO buffer <br> contains 32 unread samples. At the following ODR a new sample set replaces the <br> oldest FIFO value. The OVRN bit is set to 0 when the first sample set has been <br> read |
| EMPTY | EMPTY flag is set high when all FIFO samples have been read and FIFO is empty |
| FSS $[4: 0]$ | FSS $[4: 0]$ field always contains the current number of unread samples stored in the <br> FIFO buffer. When FIFO is enabled, this value increases at ODR frequency until <br> the buffer is full, whereas, it decreases every time one sample set is retrieved from <br> FIFO. |

### 8.21 INT1_CFG (30h)

Table 53. INT1_CFG register

| AOI | 6D | ZHIE | ZLIE | YHIE | YLIE | XHIE | XLIE |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 54. INT1_CFG description

| AOI | And/Or combination of Interrupt events. Default value: 0 Refer to Table 55: Interrupt mode |
| :-- | :-- |
| 6D | 6 direction detection function enabled. Default value: 0Refer to Table 55: Interrupt mode |
| ZHIE | Enable interrupt generation on Z high event or on Direction recognition. Default value: 0 <br> (0: disable interrupt request;1: enable interrupt request) |
| ZLIE | Enable interrupt generation on Z low event or on Direction recognition. Default value: 0 (0: <br> disable interrupt request;1: enable interrupt request) |
| YHIE | Enable interrupt generation on Y high event or on Direction recognition. Default value: 0 <br> (0: disable interrupt request; 1: enable interrupt request.) |
| YLIE | Enable interrupt generation on Y low event or on Direction recognition. Default value: 0 (0: <br> disable interrupt request; 1: enable interrupt request.) |
| XHIE | Enable interrupt generation on X high event or on Direction recognition. Default value: 0 <br> (0: disable interrupt request; 1: enable interrupt request.) |
| XLIE | Enable interrupt generation on X low event or on Direction recognition. Default value: 0 (0: <br> disable interrupt request; 1: enable interrupt request.) |

Content of this register is loaded at boot.Write operation at this address is possible only after system boot.
Table 55. Interrupt mode

| AOI | 6D | Interrupt mode |
| :--: | :--: | :-- |
| 0 | 0 | OR combination of interrupt events |
| 0 | 1 | 6-direction movement recognition |
| 1 | 0 | AND combination of interrupt events |
| 1 | 1 | 6-direction position recognition |

Difference between $\mathrm{AOI}-6 \mathrm{D}={ }^{\prime} 01^{\prime}$ and $\mathrm{AOI}-6 \mathrm{D}={ }^{\prime} 11^{\prime}$.
AOI-6D = '01' is movement recognition. An interrupt is generated when the orientation moves from an unknown zone to known zone. The interrupt signal remains for a duration ODR.

AOI-6D = '11' is direction recognition. An interrupt is generated when the orientation is inside a known zone. The interrupt signal remains until the orientation is inside the zone.

# 8.22 INT1_SRC (31h) 

Table 56. INT1_SRC register

| 0 | IA | ZH | ZL | YH | YL | XH | XL |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |

Table 57. INT1_SRC description

| IA | Interrupt active. Default value: 0 <br> (0: no interrupt has been generated; 1: one or more interrupts have been generated) |
| :--: | :-- |
| ZH | Z high. Default value: 0 <br> (0: no interrupt, 1: Z high event has occurred) |
| ZL | Z low. Default value: 0 <br> (0: no interrupt; 1: Z low event has occurred) |
| YH | Y high. Default value: 0 <br> (0: no interrupt, 1: Y high event has occurred) |
| YL | Y low. Default value: 0 <br> (0: no interrupt, 1: Y low event has occurred) |
| XH | X high. Default value: 0 <br> (0: no interrupt, 1: X high event has occurred) |
| XL | X low. Default value: 0 <br> (0: no interrupt, 1: X low event has occurred) |

Interrupt 1 source register. Read-only register.
Reading at this address clears the INT1_SRC (31h) IA bit (and the interrupt signal on the INT 1 pin) and allows the refresh of data in INT1_SRC (31h) if the latched option was chosen.# 8.23 INT1_THS (32h) 

Table 58. INT1_THS register

| 0 | THS6 | THS5 | THS4 | THS3 | THS2 | THS1 | THS0 |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Table 59. INT1_THS description |  |  |  |  |  |  |  |
|  | Interrupt 1 threshold. Default value: 0000000 |  |  |  |  |  |  |
|  | $1 \mathrm{LSb}=16 \mathrm{mg} @ \mathrm{FS}= \pm 2 \mathrm{~g}$ |  |  |  |  |  |  |
|  | $1 \mathrm{LSb}=32 \mathrm{mg} @ \mathrm{FS}= \pm 4 \mathrm{~g}$ |  |  |  |  |  |  |
|  | $1 \mathrm{LSb}=62 \mathrm{mg} @ \mathrm{FS}= \pm 8 \mathrm{~g}$ |  |  |  |  |  |  |
|  | $1 \mathrm{LSb}=186 \mathrm{mg} @ \mathrm{FS}= \pm 16 \mathrm{~g}$ |  |  |  |  |  |  |

### 8.24 INT1_DURATION (33h)

Table 60. INT1_DURATION register

| 0 | D6 | D5 | D4 | D3 | D2 | D1 | D0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 61. INT1_DURATION description

| $\mathrm{D}[6: 0]$ | Duration value. Default value: 0000000 |
| :-- | :-- |
|  | $1 \mathrm{LSb}=1 / \mathrm{ODR}$ |

The D[6:0] bits set the minimum duration of the Interrupt 2 event to be recognized. Duration steps and maximum values depend on the ODR chosen.

Duration time is measured in N/ODR, where N is the content of the duration register.# 8.25 INT2_CFG (34h) 

Table 62. INT2_CFG register

| AOI | 6D | ZHIE | ZLIE | YHIE | YLIE | XHIE | XLIE |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 63. INT2_CFG description

| AOI | AND/OR combination of interrupt events. Default value: 0 <br> (see Table 64) |
| :-- | :-- |
| 6D | 6-direction detection function enabled. Default value: 0. Refer to Table 64. |
| ZHIE | Enable interrupt generation on Z high event. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value higher than preset threshold) |
| ZLIE | Enable interrupt generation on Z low event. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value lower than preset threshold) |
| YHIE | Enable interrupt generation on Y high event. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value higher than preset threshold) |
| YLIE | Enable interrupt generation on Y low event. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value lower than preset threshold) |
| XHIE | Enable interrupt generation on X high event. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value higher than preset threshold) |
| XLIE | Enable interrupt generation on X low event. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value lower than preset threshold) |

The content of this register is loaded at boot.
A write operation to this address is possible only after system boot.
Table 64. Interrupt mode

| AOI | 6D | Interrupt mode |
| :--: | :--: | :--: |
| 0 | 0 | OR combination of interrupt events |
| 0 | 1 | 6-direction movement recognition |
| 1 | 0 | AND combination of interrupt events |
| 1 | 1 | 6-direction position recognition |

The difference between $\mathrm{AOI}-6 \mathrm{D}={ }^{\prime} 01^{\prime}$ and $\mathrm{AOI}-6 \mathrm{D}={ }^{\prime} 11^{\prime}$.
AOI-6D = '01' is movement recognition. An interrupt is generated when the orientation moves from an unknown zone to a known zone. The interrupt signal remains for a duration ODR.

AOI-6D = '11' is direction recognition. An interrupt is generated when the orientation is inside a known zone. The interrupt signal remains while the orientation is inside the zone.# 8.26 INT2_SRC (35h) 

Table 65. INT2_SRC register

| 0 | IA | ZH | ZL | YH | YL | XH | XL |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 66. INT2_SRC description

| IA | Interrupt active. Default value: 0 <br> (0: no interrupt has been generated; 1: one or more interrupts have been generated) |
| :-- | :-- |
| ZH | Z high. Default value: 0 <br> (0: no interrupt, 1: Z high event has occurred) |
| ZL | Z low. Default value: 0 <br> (0: no interrupt; 1: Z low event has occurred) |
| YH | Y high. Default value: 0 <br> (0: no interrupt, 1: Y high event has occurred) |
| YL | Y low. Default value: 0 <br> (0: no interrupt, 1: Y low event has occurred) |
| XH | X high. Default value: 0 <br> (0: no interrupt, 1: X high event has occurred) |
| XL | X low. Default value: 0 <br> (0: no interrupt, 1: X low event has occurred) |

Interrupt 2 source register. Read-only register.
Reading at this address clears the INT2_SRC (35h) IA bit (and the interrupt signal on the INT2 pin) and allows the refresh of data in the INT2_SRC (35h) register if the latched option was chosen.

### 8.27 INT2_THS (36h)

Table 67. INT2_THS register

| 0 | THS6 | THS5 | THS4 | THS3 | THS2 | THS1 | THS0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 68. INT2_THS description

|  | Interrupt 2 threshold. Default value: 0000000 |  |
| :-- | :-- | :-- |
|  | $1 \mathrm{LSb}=16 \mathrm{mg} @ \mathrm{FS}= \pm 2 \mathrm{~g}$ |  |
| THS[6:0] | $1 \mathrm{LSb}=32 \mathrm{mg} @ \mathrm{FS}= \pm 4 \mathrm{~g}$ |  |
|  | $1 \mathrm{LSb}=62 \mathrm{mg} @ \mathrm{FS}= \pm 8 \mathrm{~g}$ |  |
|  | $1 \mathrm{LSb}=186 \mathrm{mg} @ \mathrm{FS}= \pm 16 \mathrm{~g}$ |  |# 8.28 INT2_DURATION (37h) 

Table 69. INT2_DURATION register

| 0 | D6 | D5 | D4 | D3 | D2 | D1 | D0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 70. INT2_DURATION description

| D[6:0] | Duration value. Default value: 0000000 |
| :-- | :-- |
|  | $1 \mathrm{LSb}=1 / \mathrm{ODR}^{(1)}$ |

1. Duration time is measured in N/ODR, where N is the content of the duration register.

The D[6:0] bits set the minimum duration of the Interrupt 2 event to be recognized. Duration time steps and maximum values depend on the ODR chosen.

### 8.29 CLICK_CFG (38h)

Table 71. CLICK_CFG register

| - | - | zd | zs | yd | ys | xd | xs |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 72. CLICK_CFG description

| ZD | Enable interrupt double click on Z-axis. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value higher than preset threshold) |
| :--: | :--: |
| ZS | Enable interrupt single click on Z-axis. Default value: 0 <br> (0: disable interrupt request; 1: enable interrupt request on measured accel. value higher than preset threshold) |
| YD | Enable interrupt double click on Y-axis. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value higher than preset threshold) |
| YS | Enable interrupt single click on Y-axis. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value higher than preset threshold) |
| XD | Enable interrupt double click on X-axis. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value higher than preset threshold) |
| XS | Enable interrupt single click on X-axis. Default value: 0 <br> (0: disable interrupt request; <br> 1: enable interrupt request on measured accel. value higher than preset threshold) |# 8.30 CLICK_SRC (39h) 

Table 73. CLICK_SRC register

|  | IA | DCLICK | SCLICK | Sign | Z | Y | X |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 74. CLICK_SRC description

| IA | Interrupt active. Default value: 0 <br> (0: no interrupt has been generated; 1: one or more interrupts have been generated) |
| :-- | :-- |
| DCLICK | Double-click enable. Default value: 0 <br> (0:double-click detection disabled, 1: double-click detection enabled) |
| SCLICK | Single-click enable. Default value: 0 <br> (0: Single-click detection disabled, 1: single-click detection enabled) |
| Sign | Click sign. <br> (0: positive detection, 1: negative detection) |
| Z | Z click detection. Default value: 0 <br> (0: no interrupt, 1: Z high event has occurred) |
| Y | Y click detection. Default value: 0 <br> (0: no interrupt, 1: Y high event has occurred) |
| X | X click detection. Default value: 0 <br> (0: no interrupt, 1: X high event has occurred) |

### 8.31 CLICK_THS (3Ah)

Table 75. CLICK_THS register

| LIR_Click | Ths6 | Ths5 | Ths4 | Ths3 | Ths2 | Ths1 | Ths0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 76. CLICK_SRC description

| LIR_Click | If the LIR_Click bit is not set, the interrupt is kept high for the duration of the latency <br> window. <br> If the LIR_Click bit is set, the interrupt is kept high until the CLICK_SRC (39h) register is <br> read. |
| :-- | :-- |
| Ths[6:0] | Click threshold. Default value: 0000000 |

### 8.32 TIME_LIMIT (3Bh)

Table 77. TIME_LIMIT register

| - | TLI6 | TLI5 | TLI4 | TLI3 | TLI2 | TLI1 | TLI0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 78. TIME_LIMIT description

| TLI[6:0] | Click time limit. Default value: 0000000 |
| :-- | :-- |# 8.33 TIME_LATENCY (3Ch) 

Table 79. TIME_LATENCY register

| TLA7 | TLA6 | TLA5 | TLA4 | TLA3 | TLA2 | TLA1 | TLA0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 80. TIME_LATENCY description

| TLA[7:0] | Click time latency. Default value: 00000000 |  |  |  |  |  |  |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

### 8.34 TIME WINDOW (3Dh)

Table 81. TIME_WINDOW register

| TW7 | TW6 | TW5 | TW4 | TW3 | TW2 | TW1 | TW0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 82. TIME_WINDOW description

| TW[7:0] | Click time window |
| :-- | :-- |

### 8.35 ACT_THS (3Eh)

Table 83. ACT_THS register

| - | Acth6 | Acth5 | Acth4 | Acth3 | Acth2 | Acth1 | Acth0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 84. ACT_THS description

| Acth[6:0] | Sleep-to-wake, return-to-sleep activation threshold in low-power mode <br> $1 \mathrm{LSb}=16 \mathrm{mg} @ \mathrm{FS}= \pm 2 \mathrm{~g}$ <br> $1 \mathrm{LSb}=32 \mathrm{mg} @ \mathrm{FS}= \pm 4 \mathrm{~g}$ <br> $1 \mathrm{LSb}=62 \mathrm{mg} @ \mathrm{FS}= \pm 8 \mathrm{~g}$ <br> $1 \mathrm{LSb}=186 \mathrm{mg} @ \mathrm{FS}= \pm 16 \mathrm{~g}$ |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: |

### 8.36 ACT_DUR (3Fh)

Table 85. ACT_DUR register

| ActD7 | ActD6 | ActD5 | ActD4 | ActD3 | ActD2 | ActD1 | ActD0 |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |

Table 86. ACT_DUR description

| ActD[7:0] | Sleep-to-wake, return-to-sleep duration <br> $1 \mathrm{LSb}=\left(8^{*} 1[\mathrm{LSb}]+1\right) / \mathrm{ODR}$ |
| :-- | :-- |# 9 Package information 

In order to meet environmental requirements, ST offers these devices in different grades of ECOPACK ${ }^{\circledR}$ packages, depending on their level of environmental compliance. ECOPACK ${ }^{\circledR}$ specifications, grade definitions and product status are available at: www.st.com.
ECOPACK is an ST trademark.# 9.1 LGA-16 package information 

Figure 12. LGA-16 package outline and mechanical dimensions
![img-12.jpeg](img-12.jpeg)# 9.2 LGA-16 packing information 

Figure 13. Carrier tape information for LGA-16 package
![img-13.jpeg](img-13.jpeg)

Figure 14. LGA-16 package orientation in carrier tape
![img-14.jpeg](img-14.jpeg)Figure 15. Reel information for carrier tape of LGA-16 package
![img-15.jpeg](img-15.jpeg)

Table 87. Reel dimensions for carrier tape of LGA-16 package

| Reel dimensions $(\mathbf{m m})$ |  |
| :--: | :--: |
| A (max) | 330 |
| B (min) | 1.5 |
| C | $13 \pm 0.25$ |
| D (min) | 20.2 |
| N (min) | 60 |
| G | $12.4+2 /-0$ |
| T (max) | 18.4 |# 10 Revision history 

Table 88. Document revision history

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 21-May-2010 | 1 | Initial release |
| 12-Dec-2016 | 2 | Updated Table 1: Device summary <br> Updated Features and Figure 1: Block diagram <br> Updated Table 2: Pin description and Table 14: Serial interface pin description <br> Added Table 3: Internal pull-up values (typ.) for SDO/SAO pin <br> Updated Table 9: Absolute maximum ratings <br> Updated Section 3.7: Auxiliary ADC and temperature sensor <br> Updated Section 4: Application hints <br> Updated Section 5: Digital main blocks <br> Updated Section 7: Register mapping and Section 8: Registers description <br> Updated Section 9.1: LGA-16 package information <br> Added Section 9.2: LGA-16 packing information <br> Minor textual updates |# IMPORTANT NOTICE - PLEASE READ CAREFULLY 

STMicroelectronics NV and its subsidiaries ("ST") reserve the right to make changes, corrections, enhancements, modifications, and improvements to ST products and/or to this document at any time without notice. Purchasers should obtain the latest relevant information on ST products before placing orders. ST products are sold pursuant to ST's terms and conditions of sale in place at the time of order acknowledgement.

Purchasers are solely responsible for the choice, selection, and use of ST products and ST assumes no liability for application assistance or the design of Purchasers' products.

No license, express or implied, to any intellectual property right is granted by ST herein.

Resale of ST products with provisions different from the information set forth herein shall void any warranty granted by ST for such product.

ST and the ST logo are trademarks of ST. All other product or service names are the property of their respective owners.

Information in this document supersedes and replaces information previously supplied in any prior versions of this document.
(c) 2016 STMicroelectronics - All rights reserved