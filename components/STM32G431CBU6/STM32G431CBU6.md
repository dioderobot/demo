# STM32G431x6 STM32G431x8 STM32G431xB 

Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M4 32-bit MCU+FPU, 170 MHz /213 DMIPS, up to 128 KB Flash, 32 KB SRAM, rich analog, math accelerator

Datasheet - production data

## Features

## Includes ST state-of-the-art patented technology

- Core: Arm ${ }^{\circledR} 32$-bit Cortex ${ }^{\circledR}$-M4 CPU with FPU, Adaptive real-time accelerator (ART Accelerator) allowing 0 -wait-state execution from Flash memory, frequency up to 170 MHz with 213 DMIPS, MPU, DSP instructions
- Operating conditions:
$-\mathrm{V}_{\mathrm{DD}}, \mathrm{V}_{\mathrm{DDA}}$ voltage range: 1.71 V to 3.6 V
- Mathematical hardware accelerators
- CORDIC for trigonometric functions acceleration
- FMAC: filter mathematical accelerator
- Memories
- 128 Kbytes of Flash memory with ECC support, proprietary code readout protection (PCROP), securable memory area, 1 Kbyte OTP
- 22 Kbytes of SRAM, with hardware parity check implemented on the first 16 Kbytes
- Routine booster: 10 Kbytes of SRAM on instruction and data bus, with hardware parity check (CCM SRAM)
- Reset and supply management
- Power-on/power-down reset (POR/PDR/BOR)
- Programmable voltage detector (PVD)
- Low-power modes: sleep, stop, standby and shutdown
- $\mathrm{V}_{\text {BAT }}$ supply for RTC and backup registers
![img-0.jpeg](img-0.jpeg)
- Clock management
- 4 to 48 MHz crystal oscillator
- 32 kHz oscillator with calibration
- Internal 16 MHz RC with PLL option ( $\pm 1 \%$ )
- Internal 32 kHz RC oscillator ( $\pm 5 \%$ )
- Up to 86 fast I/Os
- All mappable on external interrupt vectors
- Several I/Os with 5 V tolerant capability
- Interconnect matrix
- 12-channel DMA controller
- $2 \times$ ADCs $0.25 \mu \mathrm{~s}$ (up to 23 channels). Resolution up to 16-bit with hardware oversampling, 0 to 3.6 V conversion range
- $4 \times 12$-bit DAC channels
- 2 x buffered external channels 1 MSPS
- 2 x unbuffered internal channels 15 MSPS
- 4 x ultra-fast rail-to-rail analog comparators
- 3 x operational amplifiers that can be used in PGA mode, all terminals accessible
- Internal voltage reference buffer (VREFBUF) supporting three output voltages ( 2.048 V , $2.5 \mathrm{~V}, 2.9 \mathrm{~V}$ )
- 14 timers:
- $1 \times 32$-bit timer and $2 \times 16$-bit timers with up to four IC/OC/PWM or pulse counter and quadrature (incremental) encoder input
- $2 \times 16$-bit 8-channel advanced motor control timers, with up to $8 \times$ PWM channels, dead time generation and emergency stop- $1 \times 16$-bit timer with $2 \times$ IC/OCs, one OCN/PWM, dead time generation and emergency stop
- $2 \times 16$-bit timers with IC/OC/OCN/PWM, dead time generation and emergency stop
- $2 \times$ watchdog timers (independent, window)
- 1 x SysTick timer: 24-bit downcounter
- $2 \times 16$-bit basic timers
- 1 x low-power timer
- Calendar RTC with alarm, periodic wakeup from stop/standby
- Communication interfaces
- $1 \times$ FDCAN controller supporting flexible data rate
- $3 \times I^{2} \mathrm{C}$ Fast mode plus ( $1 \mathrm{Mbit} / \mathrm{s}$ ) with 20 mA current sink, SMBus/PMBus, wakeup from stop
- $4 \times$ USART/UARTs (ISO 7816 interface, LIN, IrDA, modem control)
- $1 \times$ LPUART
- $3 \times$ SPIs, 4 to 16 programmable bit frames, $2 \times$ with multiplexed half duplex $\mathrm{I}^{2} \mathrm{~S}$ interface
- $1 \times$ SAI (serial audio interface)
- USB 2.0 full-speed interface with LPM and BCD support
- IRTIM (infrared interface)
- USB Type-C ${ }^{\text {TM }}$ /USB power delivery controller (UCPD)
- True random number generator (RNG)
- CRC calculation unit, 96-bit unique ID
- Development support: serial wire debug (SWD), JTAG, Embedded Trace Macrocell ${ }^{\text {TM }}$

Table 1. Device summary

| Reference | Part number |
| :-- | :-- |
| STM32G431x6 | STM32G431C6, STM32G431K6, STM32G431R6, STM32G431V6, STM32G431M6 |
| STM32G431x8 | STM32G431C8, STM32G431K8, STM32G431R8, STM32G431V8, STM32G431M8 |
| STM32G431xB | STM32G431CB, STM32G431KB, STM32G431RB, STM32G431VB, STM32G431MB |# Contents 

1 Introduction ..... 12
2 Description ..... 13
3 Functional overview ..... 17
3.1 Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M4 core with FPU ..... 17
3.2 Adaptive real-time memory accelerator (ART accelerator) ..... 17
3.3 Memory protection unit ..... 17
3.4 Embedded Flash memory ..... 17
3.5 Embedded SRAM ..... 18
3.6 Multi-AHB bus matrix ..... 19
3.7 Boot modes ..... 19
3.8 CORDIC ..... 20
3.9 Filter mathematical accelerator (FMAC) ..... 20
3.10 Cyclic redundancy check calculation unit (CRC) ..... 21
3.11 Power supply management ..... 21
3.11.1 Power supply schemes ..... 21
3.11.2 Power supply supervisor ..... 22
3.11.3 Voltage regulator ..... 22
3.11.4 Low-power modes ..... 23
3.11.5 Reset mode ..... 23
3.11.6 VBAT operation ..... 24
3.12 Interconnect matrix ..... 25
3.13 Clocks and startup ..... 26
3.14 General-purpose inputs/outputs (GPIOs) ..... 27
3.15 Direct memory access controller (DMA) ..... 27
3.16 DMA request router (DMAMUX) ..... 28
3.17 Interrupts and events ..... 28
3.17.1 Nested vectored interrupt controller (NVIC) ..... 28
3.17.2 Extended interrupt/event controller (EXTI) ..... 28
3.18 Analog-to-digital converter (ADC) ..... 29
3.18.1 Temperature sensor ..... 293.18.2 Internal voltage reference (VREFINT) ..... 30
3.18.3 VBAT battery voltage monitoring ..... 30
3.19 Digital to analog converter (DAC) ..... 30
3.20 Voltage reference buffer (VREFBUF) ..... 31
3.21 Comparators (COMP) ..... 32
3.22 Operational amplifier (OPAMP) ..... 32
3.23 Random number generator (RNG) ..... 32
3.24 Timers and watchdogs ..... 32
3.24.1 Advanced motor control timer (TIM1, TIM8) ..... 33
3.24.2 General-purpose timers (TIM2, TIM3, TIM4, TIM15, TIM16, TIM17) ..... 34
3.24.3 Basic timers (TIM6 and TIM7) ..... 34
3.24.4 Low-power timer (LPTIM1) ..... 35
3.24.5 Independent watchdog (IWDG) ..... 35
3.24.6 System window watchdog (WWDG) ..... 35
3.24.7 SysTick timer ..... 35
3.25 Real-time clock (RTC) and backup registers ..... 36
3.26 Tamper and backup registers (TAMP) ..... 36
3.27 Infrared transmitter ..... 37
3.28 Inter-integrated circuit interface ( $\left.\mathrm{I}^{2} \mathrm{C}\right)$ ..... 38
3.29 Universal synchronous/asynchronous receiver transmitter (USART) ..... 39
3.30 Low-power universal asynchronous receiver transmitter (LPUART) ..... 40
3.31 Serial peripheral interface (SPI) ..... 40
3.32 Serial audio interfaces (SAI) ..... 41
3.32.1 SAI peripheral supports ..... 41
3.33 Controller area network (FDCAN1) ..... 42
3.34 Universal serial bus (USB) ..... 42
3.35 USB Type-C ${ }^{\text {TM }}$ / USB Power Delivery controller (UCPD) ..... 42
3.36 Clock recovery system (CRS) ..... 43
3.37 Development support ..... 43
3.37.1 Serial wire JTAG debug port (SWJ-DP) ..... 43
3.37.2 Embedded trace macrocell ${ }^{\text {TM }}$ ..... 43
4 Pinouts and pin description ..... 44
4.1 UFQFPN32 pinout description ..... 444.2 LQFP32 pinout description ..... 44
4.3 UFQFPN48 pinout description ..... 45
4.4 LQFP48 pinout description ..... 45
4.5 WLCSP49 ballout description ..... 46
4.6 LQFP64 pinout description ..... 46
4.7 UFBGA64 ballout description ..... 47
4.8 LQFP80 pinout description ..... 48
4.9 LQFP100 pinout description ..... 49
4.10 Pin definition ..... 50
4.11 Alternate functions ..... 61
5 Electrical characteristics ..... 67
5.1 Parameter conditions ..... 67
5.1.1 Minimum and maximum values ..... 67
5.1.2 Typical values ..... 67
5.1.3 Typical curves ..... 67
5.1.4 Loading capacitor ..... 67
5.1.5 Pin input voltage ..... 67
5.1.6 Power supply scheme ..... 68
5.1.7 Current consumption measurement ..... 69
5.2 Absolute maximum ratings ..... 69
5.3 Operating conditions ..... 71
5.3.1 General operating conditions ..... 71
5.3.2 Operating conditions at power-up / power-down ..... 72
5.3.3 Embedded reset and power control block characteristics ..... 72
5.3.4 Embedded voltage reference ..... 74
5.3.5 Supply current characteristics ..... 75
5.3.6 Wakeup time from low-power modes and voltage scaling transition times ..... 97
5.3.7 External clock source characteristics ..... 98
5.3.8 Internal clock source characteristics ..... 103
5.3.9 PLL characteristics ..... 106
5.3.10 Flash memory characteristics ..... 107
5.3.11 EMC characteristics ..... 108
5.3.12 Electrical sensitivity characteristics ..... 109
5.3.13 I/O current injection characteristics ..... 1105.3.14 I/O port characteristics ..... 111
5.3.15 NRST pin characteristics ..... 116
5.3.16 Extended interrupt and event controller input (EXTI) characteristics ..... 117
5.3.17 Analog switches booster ..... 117
5.3.18 Analog-to-digital converter characteristics ..... 118
5.3.19 Digital-to-Analog converter characteristics ..... 133
5.3.20 Voltage reference buffer characteristics ..... 140
5.3.21 Comparator characteristics ..... 143
5.3.22 Operational amplifiers characteristics ..... 144
5.3.23 Temperature sensor characteristics ..... 148
5.3.24 $\mathrm{V}_{\mathrm{BAT}}$ monitoring characteristics ..... 148
5.3.25 Timer characteristics ..... 149
5.3.26 Communication interfaces characteristics ..... 150
5.3.27 UCPD characteristics ..... 160
6 Package information ..... 161
6.1 UFQFPN32 package information ..... 161
6.2 LQFP32 package information ..... 164
6.3 UFQFPN48 package information ..... 167
6.4 LQFP48 package information ..... 170
6.5 WLCSP49 package information ..... 174
6.6 LQFP64 package information ..... 178
6.7 UFBGA64 package information ..... 181
6.8 LQFP80 package information ..... 184
6.9 LQFP100 package information ..... 187
6.10 Thermal characteristics ..... 190
6.10.1 Reference document ..... 191
6.10.2 Selecting the product temperature range ..... 192
7 Ordering information ..... 194
8 Revision history ..... 195# List of tables 

Table 1. Device summary ..... 2
Table 2. STM32G431x6/x8/xB features and peripheral counts ..... 14
Table 3. STM32G431x6/x8/xB peripherals interconnect matrix ..... 25
Table 4. DMA implementation ..... 27
Table 5. Temperature sensor calibration values ..... 30
Table 6. Internal voltage reference calibration values ..... 30
Table 7. Timer feature comparison ..... 32
Table 8. I2C implementation ..... 38
Table 9. USART/UART/LPUART features ..... 39
Table 10. SAI features implementation ..... 41
Table 11. Legend/abbreviations used in the pinout table ..... 50
Table 12. STM32G431x6/x8/xB pin definition ..... 51
Table 13. Alternate function ..... 61
Table 14. Voltage characteristics ..... 69
Table 15. Current characteristics ..... 70
Table 16. Thermal characteristics ..... 70
Table 17. General operating conditions ..... 71
Table 18. Operating conditions at power-up / power-down ..... 72
Table 19. Embedded reset and power control block characteristics. ..... 72
Table 20. Embedded internal voltage reference ..... 74
Table 21. Current consumption in Run and Low-power run modes, code with data processing running from Flash in single Bank, ART enable (Cache ON Prefetch OFF) ..... 76
Table 22. Current consumption in Run and Low-power run modes, code with data processing running from SRAM1 ..... 78
Table 23. Typical current consumption in Run and Low-power run modes, with different codes running from Flash, ART enable (Cache ON Prefetch OFF) ..... 80
Table 24. Typical current consumption in Run and Low-power run modes, with different codes running from SRAM1 ..... 81
Table 25. Typical current consumption in Run and Low-power run modes, with different codes running from SRAM2 ..... 82
Table 26. Typical current consumption in Run and Low-power run modes, with different codes running from CCM ..... 83
Table 27. Current consumption in Sleep and Low-power sleep mode Flash ON ..... 84
Table 28. Current consumption in low-power sleep modes, Flash in power-down ..... 85
Table 29. Current consumption in Stop 1 mode ..... 86
Table 30. Current consumption in Stop 0 mode ..... 87
Table 31. Current consumption in Standby mode ..... 87
Table 32. Current consumption in Shutdown mode ..... 89
Table 33. Current consumption in VBAT mode ..... 91
Table 34. Peripheral current consumption ..... 93
Table 35. Low-power mode wakeup timings ..... 97
Table 36. Regulator modes transition times ..... 98
Table 37. Wakeup time using USART/LPUART ..... 98
Table 38. High-speed external user clock characteristics ..... 98
Table 39. Low-speed external user clock characteristics ..... 99
Table 40. HSE oscillator characteristics ..... 100
Table 41. LSE oscillator characteristics ( $\mathrm{f}_{\mathrm{LSE}}=32.768 \mathrm{kHz}$ ) ..... 102
Table 42. HSI16 oscillator characteristics. ..... 103Table 43. HSI48 oscillator characteristics ..... 104
Table 44. LSI oscillator characteristics ..... 105
Table 45. PLL characteristics ..... 106
Table 46. Flash memory characteristics ..... 107
Table 47. Flash memory endurance and data retention ..... 107
Table 48. EMS characteristics ..... 108
Table 49. EMI characteristics ..... 109
Table 50. ESD absolute maximum ratings ..... 109
Table 51. Electrical sensitivities ..... 110
Table 52. I/O current injection susceptibility ..... 110
Table 53. I/O static characteristics ..... 111
Table 54. Output voltage characteristics ..... 113
Table 55. I/O (except FT_c) AC characteristics ..... 114
Table 56. I/O FT_c AC characteristics ..... 115
Table 57. NRST pin characteristics ..... 116
Table 58. EXTI input characteristics ..... 117
Table 59. Analog switches booster characteristics ..... 117
Table 60. ADC characteristics ..... 118
Table 61. Maximum ADC RAIN ..... 121
Table 62. ADC accuracy - limited test conditions 1 ..... 123
Table 63. ADC accuracy - limited test conditions 2 ..... 125
Table 64. ADC accuracy - limited test conditions 3 ..... 127
Table 65. ADC accuracy (Multiple ADCs operation) - limited test conditions 1 ..... 129
Table 66. ADC accuracy (Multiple ADCs operation) - limited test conditions 2 ..... 130
Table 67. ADC accuracy (Multiple ADCs operation) - limited test conditions 3 ..... 131
Table 68. DAC 1MSPS characteristics ..... 133
Table 69. DAC 1MSPS accuracy ..... 136
Table 70. DAC 15MSPS characteristics ..... 137
Table 71. DAC 15MSPS accuracy ..... 139
Table 72. VREFBUF characteristics ..... 140
Table 73. COMP characteristics ..... 143
Table 74. OPAMP characteristics ..... 144
Table 75. TS characteristics ..... 148
Table 76. $\mathrm{V}_{\mathrm{BAT}}$ monitoring characteristics ..... 148
Table 77. $\mathrm{V}_{\mathrm{BAT}}$ charging characteristics ..... 148
Table 78. TIMx characteristics ..... 149
Table 79. IWDG min/max timeout period at 32 kHz (LSI) ..... 150
Table 80. WWDG min/max timeout value at 170 MHz (PCLK) ..... 150
Table 81. Minimum I2CCLK frequency in all I2C modes ..... 151
Table 82. I2C analog filter characteristics ..... 151
Table 83. SPI characteristics ..... 152
Table 84. I2S characteristics ..... 155
Table 85. SAI characteristics ..... 157
Table 86. USB electrical characteristics ..... 159
Table 87. USART electrical characteristics ..... 159
Table 88. UCPD characteristics ..... 160
Table 89. UFQFPN32 - Mechanical data ..... 162
Table 90. LQFP32 - Mechanical data ..... 165
Table 91. UFQFPN48 - Mechanical data ..... 168
Table 92. LQFP48 - Mechanical data ..... 171
Table 93. WLCSP49 - Mechanical data ..... 175
Table 94. WLCSP49 - Recommended PCB design rules ..... 176Table 95. LQFP64 - Mechanical data ..... 178
Table 96. UFBGA64 - Mechanical data ..... 181
Table 97. UFBGA64 - Recommended PCB design rules ( 0.5 mm pitch BGA). ..... 182
Table 98. LQFP80 - Mechanical data ..... 184
Table 99. LQPF100 - Mechanical data ..... 187
Table 100. Package thermal characteristics ..... 190
Table 101. Ordering information scheme ..... 194
Table 102. Document revision history ..... 195# List of figures 

Figure 1. STM32G431x6/x8/xB block diagram ..... 16
Figure 2. Multi-AHB bus matrix ..... 19
Figure 3. Voltage reference buffer ..... 31
Figure 4. Infrared transmitter ..... 37
Figure 5. STM32G431x6/x8/xB UFQFPN32 pinout ..... 44
Figure 6. STM32G431x6/x8/xB LQFP32 pinout ..... 44
Figure 7. STM32G431x6/x8/xB UFQFPN48 pinout ..... 45
Figure 8. STM32G431x6/x8/xB LQFP48 pinout ..... 45
Figure 9. STM32G431x6/x8/xB WLCSP49 ballout ..... 46
Figure 10. STM32G431x6/x8/xB LQFP64 pinout ..... 46
Figure 11. STM32G431x6/x8/xB UFBGA64 ballout ..... 47
Figure 12. STM32G431x6/x8/xB LQFP80 pinout ..... 48
Figure 13. STM32G431x6/x8/xB LQFP100 pinout ..... 49
Figure 14. Pin loading conditions ..... 67
Figure 15. Pin input voltage ..... 67
Figure 16. Power supply scheme ..... 68
Figure 17. Current consumption measurement ..... 69
Figure 18. VREFINT versus temperature ..... 75
Figure 19. High-speed external clock source AC timing diagram ..... 99
Figure 20. Low-speed external clock source AC timing diagram ..... 99
Figure 21. Typical application with an 8 MHz crystal ..... 101
Figure 22. Typical application with a 32.768 kHz crystal ..... 102
Figure 23. HSI16 frequency versus temperature ..... 104
Figure 24. HSI48 frequency versus temperature ..... 105
Figure 25. I/O input characteristics ..... 112
Figure 26. I/O AC characteristics definition ${ }^{(1)}$ ..... 116
Figure 27. Recommended NRST pin protection ..... 117
Figure 28. ADC accuracy characteristics ..... 132
Figure 29. Typical connection diagram when using the ADC with FT/TT pins featuring analog switch function ..... 132
Figure 30. 12-bit buffered / non-buffered DAC ..... 135
Figure 31. VREFOUT_TEMP in case VRS $=00$ ..... 141
Figure 32. VREFOUT_TEMP in case VRS $=01$ ..... 142
Figure 33. VREFOUT_TEMP in case VRS $=10$ ..... 142
Figure 34. OPAMP noise density @ $25^{\circ} \mathrm{C}$ ..... 147
Figure 35. SPI timing diagram - slave mode and CPHA $=0$ ..... 153
Figure 36. SPI timing diagram - slave mode and CPHA $=1$ ..... 154
Figure 37. SPI timing diagram - master mode ..... 154
Figure 38. SAI master timing waveforms ..... 158
Figure 39. SAI slave timing waveforms ..... 158
Figure 40. UFQFPN32 - Outline ..... 161
Figure 41. UFQFPN32 - Recommended footprint ..... 162
Figure 42. UFQFPN32 top view example ..... 163
Figure 43. LQFP32 - Outline ..... 164
Figure 44. LQFP32 - Recommended footprint ..... 165
Figure 45. LQFP32 top view example ..... 166
Figure 46. UFQFPN48 - Outline ..... 167
Figure 47. UFQFPN48 - Recommended footprint ..... 168Figure 48. UFQFPN48 top view example ..... 169
Figure 49. LQFP48 - Outline ..... 170
Figure 50. LQFP48 - Recommended footprint ..... 172
Figure 51. LQFP48 top view example ..... 173
Figure 52. WLCSP49 - Outline ..... 174
Figure 53. WLCSP49 - Recommended footprint ..... 176
Figure 54. WLCSP49 top view example ..... 177
Figure 55. LQFP64 - Outline ..... 178
Figure 56. LQFP64 - Recommended footprint ..... 179
Figure 57. LQFP64 top view example ..... 180
Figure 58. UFBGA64 - Outline ..... 181
Figure 59. UFBGA64 - Recommended footprint ..... 182
Figure 60. UFBGA64 top view example ..... 183
Figure 61. LQFP80 - Outline ..... 184
Figure 62. LQFP80 - Recommended footprint ..... 185
Figure 63. LQFP80 top view example ..... 186
Figure 64. LQFP100 - Outline ..... 187
Figure 65. LQFP100 - Recommended footprint ..... 188
Figure 66. LQFP100 top view example ..... 189# 1 Introduction 

This datasheet provides the ordering information and mechanical device characteristics of the STM32G431x6/x8/xB microcontrollers.

This document should be read in conjunction with the reference manual RM0440 "STM32G4 Series advanced Arm ${ }^{\circledR} 32$-bit MCUs". The reference manual is available from the STMicroelectronics website www.st.com.

For information on the Arm ${ }^{\circledR(\mathrm{a})}$ Cortex ${ }^{\circledR}$-M4 core, refer to the Cortex ${ }^{\circledR}$-M4 technical reference manual, available from the www.arm.com website.

## arm

[^0]
[^0]:    a. Arm is a registered trademark of Arm Limited (or its subsidiaries) in the US and/or elsewhere.# 2 Description 

The STM32G431x6/x8/xB devices are based on the high-performance Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M4 32-bit RISC core. They operate at a frequency of up to 170 MHz .

The Cortex-M4 core features a single-precision floating-point unit (FPU), which supports all the Arm single-precision data-processing instructions and all the data types. It also implements a full set of DSP (digital signal processing) instructions and a memory protection unit (MPU) which enhances the application's security.

These devices embed high-speed memories (up to 128 Kbytes of Flash memory, and 32 Kbytes of SRAM), an extensive range of enhanced I/Os and peripherals connected to two APB buses, two AHB buses and a 32-bit multi-AHB bus matrix.

The devices also embed several protection mechanisms for embedded Flash memory and SRAM: readout protection, write protection, securable memory area and proprietary code readout protection.

The devices embed peripherals allowing mathematical/arithmetic function acceleration (CORDIC for trigonometric functions and FMAC unit for filter functions).

They offer two fast 12-bit ADCs (4 Msps), four comparators, three operational amplifiers, four DAC channels ( 2 external and 2 internal), an internal voltage reference buffer, a lowpower RTC, one general-purpose 32-bit timers, two 16-bit PWM timers dedicated to motor control, seven general-purpose 16-bit timers, and one 16-bit low-power timer.

They also feature standard and advanced communication interfaces such as:

- Three I2Cs
- Three SPIs multiplexed with two half duplex I2Ss
- Three USARTs, one UART and one low-power UART.
- One FDCAN
- One SAI
- USB device
- UCPD

The devices operate in the -40 to $+85^{\circ} \mathrm{C}\left(+105^{\circ} \mathrm{C}\right.$ junction) and -40 to $+125^{\circ} \mathrm{C}\left(+130^{\circ} \mathrm{C}\right.$ junction) temperature ranges from a 1.71 to 3.6 V power supply. A comprehensive set of power-saving modes allows the design of low-power applications.

Some independent power supplies are supported including an analog independent supply input for ADC, DAC, OPAMPs and comparators. A V ${ }_{\text {BAT }}$ input allows backup of the RTC and the registers.

The STM32G431x6/x8/xB family offers 9 packages from 32-pin to 100-pin.Table 2. STM32G431x6/x8/xB features and peripheral counts

| Peripheral |  | STM32G431Kx |  |  | STM32G431Cx |  |  | STM32G431Rx |  |  | STM32G431Mx |  |  | STM32G431Vx |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Flash memory |  | $\begin{aligned} & 32 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 64 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 128 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 32 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 64 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 128 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 32 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 64 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 128 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 32 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 64 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 128 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 32 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 64 \\ & \mathrm{~KB} \end{aligned}$ | $\begin{aligned} & 128 \\ & \mathrm{~KB} \end{aligned}$ |
| SRAM |  | $32(16+6+10) \mathrm{KB}$ |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Timers | Advanced motor control | 2 (16-bit) |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | General purpose | 5 (16-bit) <br> 1 (32-bit) |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Basic | 2 (16-bit) |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Low power | 1 (16-bit) |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | SysTick timer | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Watchdog timers (independent, window) | 2 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | PWM channels (all) | 23 |  | 32 |  | 36 |  |  | 36 |  |  | 36 |  |  |  |
|  | PWM channels (except complementary) | 23 |  | 25 |  | 25 |  |  | 25 |  |  | 25 |  |  |  |
| Comm. interfaces | $\operatorname{SPI(I2S)}^{(1)}$ | 3 (2) |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | $\mathrm{I}^{2} \mathrm{C}$ | 3 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | USART | 2 |  | 3 |  |  |  |  |  |  |  |  |  |  |  |
|  | UART | 0 |  | 0 in LQFP48 <br> 1 in UFQFPN48 |  |  |  |  | 1 |  |  |  |  |  |  |
|  | LPUART | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | FDCANs | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | USB device | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | UCPD | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | SAI | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |
| RTC | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Tamper pins | 1 |  | 2 |  |  |  |  |  | 2 |  |  | 3 |  |  |  |
| Random number generator | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| AES | No |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CORDIC | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| FMAC | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| GPIOs | 26 |  | 38 in LQFP48 <br> 42 in UFQFPN48 <br> 41 in WLCSP49 |  | 52 |  |  | 66 |  |  | 86 |  |  |  |  |
| Wakeup pins | 2 |  | 3 |  | 4 |  |  | 4 |  |  | 5 |  |  |  |  |Table 2. STM32G431x6/x8/xB features and peripheral counts (continued)

| Peripheral | STM32G431Kx | STM32G431Cx | STM32G431Rx | STM32G431Mx | STM32G431Vx |
| :--: | :--: | :--: | :--: | :--: | :--: |
| 12-bit ADCs <br> Number of channels | 2 |  |  |  |  |
|  | 11 | 17 in LQFP48 <br> 18 in UFQFPN48 <br> 18 in WLCSP49 | 23 | 23 | 23 |
| 12-bit DAC <br> Number of channels | 2 <br> 4 (2 external +2 internal) |  |  |  |  |
| Internal voltage reference buffer | Yes |  |  |  |  |
| Analog comparator | 4 |  |  |  |  |
| Operational amplifiers | 3 |  |  |  |  |
| Max. CPU frequency | 170 MHz |  |  |  |  |
| Operating voltage | 1.71 V to 3.6 V |  |  |  |  |
| Operating temperature | Ambient operating temperature: -40 to $85^{\circ} \mathrm{C} /-40$ to $125^{\circ} \mathrm{C}$ |  |  |  |  |
| Packages | LQFP32/ UFQFPN32 | LQFP48/ UFQFPN48/ WLCSP49 | LQFP64/ UFBGA64 | LQFP80 | LQFP100 |

1. The SPI2/3 interfaces can work in an exclusive way in either the SPI mode or the I2S audio mode.Figure 1. STM32G431x6/x8/xB block diagram
![img-1.jpeg](img-1.jpeg)

1. AF: alternate function on I/O pins.# 3 Functional overview 

### 3.1 Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M4 core with FPU

The Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M4 with FPU processor is the latest generation of Arm processors for embedded systems. It was developed to provide a low-cost platform that meets the needs of the MCU implementation, with a reduced pin count and with low-power consumption, while delivering outstanding computational performance and an advanced response to interrupts.
The Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M4 with FPU 32-bit RISC processor features an exceptional codeefficiency, delivering the expected high-performance from an Arm core in a memory size usually associated with 8 -bit and 16 -bit devices.

The processor supports a set of DSP instructions which allows an efficient signal processing and a complex algorithm execution. Its single precision FPU speeds up the software development by using metalanguage development tools to avoid saturation.

With its embedded Arm core, the STM32G431x6/x8/xB family is compatible with all Arm tools and software.

Figure 1 shows the general block diagram of the STM32G431x6/x8/xB devices.

### 3.2 Adaptive real-time memory accelerator (ART accelerator)

The ART accelerator is a memory accelerator that is optimized for the STM32 industrystandard Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M4 processors. It balances the inherent performance advantage of the Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M4 over Flash memory technologies, which normally requires the processor to wait for the Flash memory at higher frequencies.

### 3.3 Memory protection unit

The memory protection unit (MPU) is used to manage the CPU accesses to the memory and to prevent one task to accidentally corrupt the memory or the resources used by any other active task. This memory area is organized into up to 8 protected areas, which can be divided in up into 8 subareas each. The protection area sizes range between 32 bytes and the whole 4 gigabytes of addressable memory.

The MPU is especially helpful for applications where some critical or certified code has to be protected against the misbehavior of other tasks. It is usually managed by an RTOS (realtime operating system). If a program accesses a memory location that is prohibited by the MPU, the RTOS can detect it and take action. In an RTOS environment, the kernel can dynamically update the MPU area setting based on the process to be executed.

The MPU is optional and can be bypassed for applications that do not need it.

### 3.4 Embedded Flash memory

The STM32G431x6/x8/xB devices feature up to 128 Kbytes of embedded Flash memory which is available for storing programs and data.

Flexible protections can be configured thanks to the option bytes:- Readout protection (RDP) to protect the whole memory. Three levels of protection are available:
- Level 0: no readout protection
- Level 1: memory readout protection; the Flash memory cannot be read from or written to if either the debug features are connected or the boot in RAM or bootloader are selected
- Level 2: chip readout protection; the debug features (Cortex-M4 JTAG and serial wire), the boot in RAM and the bootloader selection are disabled (JTAG fuse). This selection is irreversible.
- Write protection (WRP): the protected area is protected against erasing and programming.
- Proprietary code readout protection (PCROP): a part of the Flash memory can be protected against read and write from third parties. The protected area is execute-only and it can only be reached by the STM32 CPU as an instruction code, while all other accesses (DMA, debug and CPU data read, write and erase) are strictly prohibited. An additional option bit (PCROP_RDP) allows to select if the PCROP area is erased or not when the RDP protection is changed from Level 1 to Level 0.
- $\quad$ Securable memory area: a part of Flash memory can be configured by option bytes to be securable. After reset this securable memory area is not secured and it behaves like the remainder of main Flash memory (execute, read, write access). When secured, any access to this securable memory area generates corresponding read/write error. Purpose of the Securable memory area is to protect sensitive code and data (secure keys storage) which can be executed only once at boot, and never again unless a new reset occurs.

The Flash memory embeds the error correction code (ECC) feature supporting:

- Single error detection and correction
- Double error detection
- The address of the ECC fail can be read in the ECC register
- 1 Kbyte (128 double word) OTP (one-time programmable) for user data. The OTP area is available in Bank 1 only. The OTP data cannot be erased and can be written only once.


# 3.5 Embedded SRAM 

STM32G431x6/x8/xB devices feature 32 Kbytes of embedded SRAM. This SRAM is split into three blocks:

- 16 Kbytes mapped at address $0 \times 20000000$ (SRAM1). The CM4 can access the SRAM1 through the System Bus (or through the I-Code/D-Code buses when boot from SRAM1 is selected or when physical remap is selected by SYSCFG_MEMRMP register). Whole SRAM1 supports hardware parity check.
- 6 Kbytes mapped at address $0 \times 20004000$ (SRAM2). The CM4 can access the SRAM2 through the System bus. SRAM2 can be kept in stop and standby modes.
- 10 Kbytes mapped at address $0 \times 10000000$ (CCM SRAM). It is accessed by the CPU through I-Code/D-Code bus for maximum performance.
It is also aliased at $0 \times 20005800$ address to be accessed by all masters (CPU, DMA1, DMA2) through SBUS contiguously to SRAM1 and SRAM2. The CCM SRAM supports hardware parity check and can be write-protected with 1-Kbyte granularity.
- The memory can be accessed in read/write at max CPU clock speed with 0 wait states.# 3.6 Multi-AHB bus matrix 

The 32-bit multi-AHB bus matrix interconnects all the masters (CPU, DMAs) and the slaves (Flash memory, RAM,AHB and APB peripherals). It also ensures a seamless and efficient operation even when several high-speed peripherals work simultaneously.

Figure 2. Multi-AHB bus matrix
![img-2.jpeg](img-2.jpeg)

### 3.7 Boot modes

At startup, a BOOT0 pin (or nBOOT0 option bit)and an nBOOT1 option bit are used to select one of three boot options:

- Boot from user Flash
- Boot from system memory
- Boot from embedded SRAM

The BOOT0 value may come from the PB8-BOOT0 pin or from an nBOOT0 option bit depending on the value of a user nBOOT_SEL option bit to free the GPIO pad if needed.
The boot loader is located in the system memory. It is used to reprogram the Flash memory by using USART, I2C, SPI, and USB through the DFU (device firmware upgrade).# 3.8 CORDIC 

The CORDIC provides hardware acceleration of certain mathematical functions, notably trigonometric, commonly used in motor control, metering, signal processing and many other applications.

It speeds up the calculation of these functions compared to a software implementation, allowing a lower operating frequency, or freeing up processor cycles in order to perform other tasks.

## Cordic features

- 24-bit CORDIC rotation engine
- Circular and Hyperbolic modes
- Rotation and Vectoring modes
- Functions: Sine, Cosine, Sinh, Cosh, Atan, Atan2, Atanh, Modulus, Square root, Natural logarithm
- Programmable precision up to 20-bit
- Fast convergence: 4 bits per clock cycle
- Supports 16-bit and 32-bit fixed point input and output formats
- Low latency AHB slave interface
- Results can be read as soon as ready without polling or interrupt
- DMA read and write channels


### 3.9 Filter mathematical accelerator (FMAC)

The filter mathematical accelerator unit performs arithmetic operations on vectors. It comprises a multiplier/accumulator (MAC) unit, together with address generation logic, which allows it to index vector elements held in local memory.

The unit includes support for circular buffers on input and output, which allows digital filters to be implemented. Both finite and infinite impulse response filters can be realized.

The unit allows frequent or lengthy filtering operations to be offloaded from the CPU, freeing up the processor for other tasks. In many cases it can accelerate such calculations compared to a software implementation, resulting in a speed-up of time critical tasks.# FMAC features 

- $16 \times 16$-bit multiplier
- 24+2-bit accumulator with addition and subtraction
- 16-bit input and output data
- $256 \times 16$-bit local memory
- Up to three areas can be defined in memory for data buffers (two input, one output), defined by programmable base address pointers and associated size registers
- Input and output sample buffers can be circular
- Buffer "watermark" feature reduces overhead in interrupt mode
- Filter functions: FIR, IIR (direct form 1)
- AHB slave interface
- DMA read and write data channels


### 3.10 Cyclic redundancy check calculation unit (CRC)

The CRC (cyclic redundancy check) calculation unit is used to get a CRC code using a configurable generator with polynomial value and size.
Among other applications, the CRC-based techniques are used to verify data transmission or storage integrity. In the scope of the EN/IEC 60335-1 standard, they offer a mean to verify the Flash memory integrity.
The CRC calculation unit helps to compute a signature of the software during runtime, which can be ulteriorly compared with a reference signature generated at link-time and which can be stored at a given memory location.

### 3.11 Power supply management

### 3.11.1 Power supply schemes

The STM32G431x6/x8/xB devices require a 1.71 V to $3.6 \mathrm{~V} \mathrm{~V}_{\mathrm{DD}}$ operating voltage supply. Several independent supplies, can be provided for specific peripherals:

- $\mathrm{V}_{\mathrm{DD}}=1.71 \mathrm{~V}$ to 3.6 V
$\mathrm{V}_{\mathrm{DD}}$ is the external power supply for the I/Os, the internal regulator and the system analog such as reset, power management and internal clocks. It is provided externally through the VDD pins.
- $\quad \mathrm{V}_{\mathrm{DDA}}=1.62 \mathrm{~V}$ to 3.6 V (see Section 5: Electrical characteristics for the minimum $\mathrm{V}_{\mathrm{DDA}}$ voltage required for ADC, DAC, COMP, OPAMP, VREFBUF operation).
$\mathrm{V}_{\mathrm{DDA}}$ is the external analog power supply for A/D converters, D/A converters, voltage reference buffer, operational amplifiers and comparators. The $\mathrm{V}_{\mathrm{DDA}}$ voltage level is independent from the $\mathrm{V}_{\mathrm{DD}}$ voltage and should preferably be connected to $\mathrm{V}_{\mathrm{DD}}$ when these peripherals are not used.
- $\quad \mathrm{V}_{\mathrm{BAT}}=1.55 \mathrm{~V}$ to 3.6 V
$\mathrm{V}_{\mathrm{BAT}}$ is the power supply for RTC, external clock 32 kHz oscillator and backup registers (through power switch) when $\mathrm{V}_{\mathrm{DD}}$ is not present.- VREF-, VREF+
$\mathrm{V}_{\mathrm{REF}+}$ is the input reference voltage for ADCs and DACs. It is also the output of the internal voltage reference buffer when enabled.
When $\mathrm{V}_{\mathrm{DDA}}<2 \mathrm{~V} \mathrm{~V}_{\mathrm{REF}+}$ must be equal to $\mathrm{V}_{\mathrm{DDA}}$.
When $\mathrm{V}_{\mathrm{DDA}} \geq 2 \mathrm{~V} \mathrm{~V}_{\mathrm{REF}+}$ must be between 2 V and $\mathrm{V}_{\mathrm{DDA}}$.
The internal voltage reference buffer supports three output voltages, which are configured with VRS bits in the VREFBUF_CSR register:
$-\quad \mathrm{V}_{\mathrm{REF}+}=2.048 \mathrm{~V}$
$-\quad \mathrm{V}_{\mathrm{REF}+}=2.5 \mathrm{~V}$
$-\quad \mathrm{V}_{\mathrm{REF}+}=2.9 \mathrm{~V}$
$\mathrm{V}_{\text {REF }}$, is double bonded with $\mathrm{V}_{\mathrm{SSA}}$.


# 3.11.2 Power supply supervisor 

The device has an integrated ultra-low-power brown-out reset (BOR) active in all modes (except for Shutdown mode). The BOR ensures proper operation of the device after poweron and during power down. The device remains in reset mode when the monitored supply voltage $\mathrm{V}_{\mathrm{DD}}$ is below a specified threshold, without the need for an external reset circuit.
The lowest BOR level is 1.71 V at power on, and other higher thresholds can be selected through option bytes. The device features an embedded programmable voltage detector (PVD) that monitors the $\mathrm{V}_{\mathrm{DD}}$ power supply and compares it to the VPVD threshold. An interrupt can be generated when $\mathrm{V}_{\mathrm{DD}}$ drops below the VPVD threshold and/or when $\mathrm{V}_{\mathrm{DD}}$ is higher than the VPVD threshold. The interrupt service routine can then generate a warning message and/or put the MCU into a safe state. The PVD is enabled by software.
In addition, the device embeds a peripheral voltage monitor which compares the independent supply voltages $\mathrm{V}_{\mathrm{DDA}}$, with a fixed threshold in order to ensure that the peripheral is in its functional supply range.

### 3.11.3 Voltage regulator

Two embedded linear voltage regulators, main regulator (MR) and low-power regulator (LPR), supply most of digital circuitry in the device. The MR is used in Run and Sleep modes. The LPR is used in Low-power run, Low-power sleep and Stop modes. In Standby and Shutdown modes, both regulators are powered down and their outputs set in highimpedance state, such as to bring their current consumption close to zero.
The device supports dynamic voltage scaling to optimize its power consumption in Run mode. the voltage from the main regulator that supplies the logic (VCORE) can be adjusted according to the system's maximum operating frequency.
The main regulator (MR) operates in the following ranges:

- Range 1 boost mode with the CPU running at up to 170 MHz .
- Range 1 normal mode with CPU running at up to 150 MHz .
- Range 2 with a maximum CPU frequency of 26 MHz .# 3.11.4 Low-power modes 

By default, the microcontroller is in Run mode after system or power Reset. It is up to the user to select one of the low-power modes described below:

- Sleep mode: In Sleep mode, only the CPU is stopped. All peripherals continue to operate and can wake up the CPU when an interrupt/event occurs.
- Low-power run mode: This mode is achieved with VCORE supplied by the low-power regulator to minimize the regulator's operating current. The code can be executed from SRAM or from Flash, and the CPU frequency is limited to 2 MHz . The peripherals with independent clock can be clocked by HSI16.
- Low-power sleep mode: This mode is entered from the low-power run mode. Only the CPU clock is stopped. When wakeup is triggered by an event or an interrupt, the system reverts to the Low power run mode.
- Stop mode: In Stop mode, the device achieves the lowest power consumption while retaining the SRAM and register contents. All clocks in the VCORE domain are stopped. The PLL, as well as the HSI16 RC oscillator and the HSE crystal oscillator are disabled. The LSE or LSI keep running. The RTC can remain active (Stop mode with RTC, Stop mode without RTC). Some peripherals with wakeup capability can enable the HSI16 RC during Stop mode, so as to get clock for processing the wakeup event.
- Standby mode: The Standby mode is used to achieve the lowest power consumption with brown-out reset, BOR. The internal regulator is switched off to power down the VCORE domain. The PLL, as well as the HSI16 RC oscillator and the HSE crystal oscillator are also powered down. The RTC can remain active (Standby mode with RTC, Standby mode without RTC). The BOR always remains active in Standby mode. For each I/O, the software can determine whether a pull-up, a pull-down or no resistor shall be applied to that I/O during Standby mode. Upon entering Standby mode, SRAM and register contents are lost except for registers in the RTC domain and standby circuitry. The device exits Standby mode upon external reset event (NRST pin), IWDG reset event, wakeup event (WKUP pin, configurable rising or falling edge) or RTC event (alarm, periodic wakeup, timestamp, tamper), or when a failure is detected on LSE (CSS on LSE).
- Shutdown mode: The Shutdown mode allows to achieve the lowest power consumption. The internal regulator is switched off to power down the VCORE domain. The PLL, as well as the HSI16 and LSI RC-oscillators and HSE crystal oscillator are also powered down. The RTC can remain active (Shutdown mode with RTC, Shutdown mode without RTC). The BOR is not available in Shutdown mode. No power voltage monitoring is possible in this mode. Therefore, switching to RTC domain is not supported. SRAM and register contents are lost except for registers in the RTC domain. The device exits Shutdown mode upon external reset event (NRST pin), IWDG reset event, wakeup event (WKUP pin, configurable rising or falling edge) or RTC event (alarm, periodic wakeup, timestamp, tamper).


### 3.11.5 Reset mode

In order to improve the consumption under reset, the I/Os state under and after reset is "analog state" (the I/O schmitt trigger is disabled). In addition, the internal reset pull-up is deactivated when the reset source is internal.# 3.11.6 $\quad \mathrm{V}_{\text {BAT }}$ operation 

The $\mathrm{V}_{\mathrm{BAT}}$ pin allows to power the device $\mathrm{V}_{\mathrm{BAT}}$ domain from an external battery, an external supercapacitor, or from $\mathrm{V}_{\mathrm{DD}}$ when there is no external battery and when an external supercapacitor is present. The $\mathrm{V}_{\mathrm{BAT}}$ pin supplies the RTC with LSE and the backup registers. Three anti-tamper detection pins are available in $\mathrm{V}_{\mathrm{BAT}}$ mode.
The $\mathrm{V}_{\mathrm{BAT}}$ operation is automatically activated when $\mathrm{V}_{\mathrm{DD}}$ is not present. An internal $\mathrm{V}_{\mathrm{BAT}}$ battery charging circuit is embedded and can be activated when $\mathrm{V}_{\mathrm{DD}}$ is present.
Note: When the microcontroller is supplied from $\mathrm{V}_{\mathrm{BAT}}$, neither external interrupts nor RTC alarm/events exit the microcontroller from the $\mathrm{V}_{\mathrm{BAT}}$ operation.# 3.12 Interconnect matrix 

Several peripherals have direct connections between them. This allows autonomous communication between peripherals, saving CPU resources thus power supply consumption. In addition, these hardware connections allow fast and predictable latency.
Depending on peripherals, these interconnections can operate in Run, Sleep and Stop modes.

Table 3. STM32G431x6/x8/xB peripherals interconnect matrix

| Interconnect source | Interconnect destination | Interconnect action |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TIMx | TIMx | Timers synchronization or chaining | Y | Y | Y | - |
|  | ADCx <br> DACx | Conversion triggers | Y | Y | Y | - |
|  | DMA | Memory to memory transfer trigger | Y | Y | Y | - |
|  | COMPx | Comparator output blanking | Y | Y | Y | - |
| TIM16/TIM17 | IRTIM | Infrared interface output generation | Y | Y | Y | - |
| COMPx | TIM1, 8 <br> TIM2, 3, 4 | Timer input channel, trigger, break from analog signals comparison | Y | Y | Y | - |
|  | LPTIMER1 | Low-power timer triggered by analog signals comparison | Y | Y | Y | Y |
| ADCx | TIM1, 8 | Timer triggered by analog watchdog | Y | Y | Y | - |
| RTC | TIM16 | Timer input channel from RTC events | Y | Y | Y | - |
|  | LPTIMER1 | Low-power timer triggered by RTC alarms or tampers | Y | Y | Y | Y |
| All clocks sources (internal and external) | TIM15, 16, 17 | Clock source used as input channel for RC measurement and trimming | Y | Y | Y | - |
| USB | TIM2 | Timer triggered by USB SOF | Y | Y | - | - |
| CSS <br> CPU (hard fault) <br> RAM (parity error) <br> Flash memory (ECC error) <br> COMPx <br> PVD | TIM1, 8 <br> TIM15, 16, 17 | Timer break | Y | Y | Y | - |
| GPIO | TIMx | External trigger | Y | Y | Y | - |
|  | LPTIMER1 | External trigger | Y | Y | Y | - |
|  | ADCx <br> DACx | Conversion external trigger | Y | Y | Y | - |# 3.13 Clocks and startup 

The clock controller distributes the clocks coming from different oscillators to the core and the peripherals. It also manages clock gating for low-power modes and ensures clock robustness. It features:

- Clock prescaler: to get the best trade-off between speed and current consumption, the clock frequency to the CPU and peripherals can be adjusted by a programmable prescaler
- Safe clock switching: clock sources can be changed safely on the fly in run mode through a configuration register.
- Clock management: to reduce power consumption, the clock controller can stop the clock to the core, individual peripherals or memory.
- System clock source: three different sources can deliver SYSCLK system clock:
- 4 - 48 MHz high-speed oscillator with external crystal or ceramic resonator (HSE). It can supply clock to system PLL. The HSE can also be configured in bypass mode for an external clock.
- 16 MHz high-speed internal RC oscillator (HSI16), trimmable by software. It can supply clock to system PLL.
- System PLL with maximum output frequency of 170 MHz . It can be fed with HSE or HSI16 clocks.
- RC48 with clock recovery system (HSI48): internal HSIRC48 MHz clock source can be used to drive the USB or the RNG peripherals.
- Auxiliary clock source: two ultra-low-power clock sources for the real-time clock (RTC):
- 32.768 kHz low-speed oscillator with external crystal (LSE), supporting four drive capability modes. The LSE can also be configured in bypass mode for using an external clock.
- 32 kHz low-speed internal RC oscillator (LSI) with $\pm 5 \%$ accuracy, also used to clock an independent watchdog.
- Peripheral clock sources: several peripherals (I2S, USART, I2C, LPTimer, ADC, SAI, RNG) have their own clock independent of the system clock.
- Clock security system (CSS): in the event of HSE clock failure, the system clock is automatically switched to HSI16 and, if enabled, a software interrupt is generated. LSE clock failure can also be detected and generate an interrupt.
- Clock-out capability:
- MCO: microcontroller clock output: it outputs one of the internal clocks for external use by the application
- LSCO: low speed clock output: it outputs LSI or LSE in all low-power modes.

Several prescalers allow to configure the AHB frequency, the High-speed APB (APB2) and the low speed APB (APB1) domains. The maximum frequency of the AHB and the APB domains is 170 MHz .# 3.14 General-purpose inputs/outputs (GPIOs) 

Each of the GPIO pins can be configured by software as output (push-pull or open-drain), as input (with or without pull-up or pull-down) or as peripheral alternate function. Most of the GPIO pins are shared with digital or analog alternate functions. Fast I/O toggling can be achieved thanks to their mapping on the AHB2 bus.

The I/Os alternate function configuration can be locked if needed following a specific sequence in order to avoid spurious writing to the I/Os registers.

### 3.15 Direct memory access controller (DMA)

The device embeds 2 DMAs. Refer to Table 4: DMA implementation for the features implementation.

Direct memory access (DMA) is used in order to provide a high-speed data transfer between peripherals and memory as well as from memory to memory. Data can be quickly moved by DMA without any CPU actions. This keeps the CPU resources free for other operations.

The two DMA controllers have 12 channels in total, each one dedicated to manage memory access requests from one or more peripherals. Each controller has an arbiter for handling the priority between DMA requests.

The DMA supports:

- 12 independently configurable channels (requests)
- Each channel is connected to a dedicated hardware DMA request, a software trigger is also supported on each channel. This configuration is done by software.
- Priorities between requests from channels of one DMA are both software programmable (4 levels: very high, high, medium, low) or hardware programmable in case of equality (request 1 has priority over request 2 , etc.)
- Independent source and destination transfer size (byte, half word, word), emulating packing and unpacking. Source/destination addresses must be aligned on the data size.
- Support for circular buffer management
- 3 event flags (DMA half transfer, DMA transfer complete and DMA transfer error) logically ORed together in a single interrupt request for each channel
- Memory-to-memory transfer
- Peripheral-to-memory, memory-to-peripheral, and peripheral-to-peripheral transfers
- Access to Flash, SRAM, APB and AHB peripherals as source and destination
- Programmable number of data to be transferred: up to 65536.

Table 4. DMA implementation

| DMA features | DMA1 | DMA2 |
| :--: | :--: | :--: |
| Number of regular channels | 6 | 6 |# 3.16 DMA request router (DMAMUX) 

When a peripheral indicates a request for DMA transfer by setting its DMA request line, the DMA request is pending until it is served and the corresponding DMA request line is reset. The DMA request router allows to route the DMA control lines between the peripherals and the DMA controllers of the product.

An embedded multi-channel DMA request generator can be considered as one of such peripherals. The routing function is ensured by a multi-channel DMA request line multiplexer. Each channel selects a unique set of DMA control lines, unconditionally or synchronously with events on synchronization inputs.

For simplicity, the functional description is limited to DMA request lines. The other DMA control lines are not shown in figures or described in the text. The DMA request generator produces DMA requests following events on DMA request trigger inputs.

### 3.17 Interrupts and events

### 3.17.1 Nested vectored interrupt controller (NVIC)

The STM32G431x6/x8/xB devices embed a nested vectored interrupt controller which is able to manage 16 priority levels, and to handle up to 71 maskable interrupt channels plus the 16 interrupt lines of the Cortex ${ }^{\circledR}$-M4.

The NVIC benefits are the following:

- Closely coupled NVIC gives low latency interrupt processing
- Interrupt entry vector table address passed directly to the core
- Allows early processing of interrupts
- Processing of late arriving higher priority interrupts
- Support for tail chaining
- Processor state automatically saved
- Interrupt entry restored on interrupt exit with no instruction overhead

The NVIC hardware block provides flexible interrupt management features with minimal interrupt latency.

### 3.17.2 Extended interrupt/event controller (EXTI)

The extended interrupt/event controller consists of 39 edge detector lines used to generate interrupt/event requests and to wake-up the system from the Stop mode. Each external line can be independently configured to select the trigger event (rising edge, falling edge, both) and can be masked independently.

A pending register maintains the status of the interrupt requests. The internal lines are connected to peripherals with wakeup from Stop mode capability. The EXTI can detect an external line with a pulse width shorter than the internal clock period. Up to 86 GPIOs can be connected to the 16 external interrupt lines.# 3.18 Analog-to-digital converter (ADC) 

The device embeds two successive approximation analog-to-digital converters with the following features:

- 12-bit native resolution, with built-in calibration
- 4 Msps maximum conversion rate with full resolution
- Down to 41.67 ns sampling time
- Increased conversion rate for lower resolution (up to 6.66 Msps for 6-bit resolution)
- One external reference pin is available on all packages, allowing the input voltage range to be independent from the power supply
- Single-ended and differential mode inputs
- Low-power design
- Capable of low-current operation at low conversion rate (consumption decreases linearly with speed)
- Dual clock domain architecture: ADC speed independent from CPU frequency
- Highly versatile digital interface
- Single-shot or continuous/discontinuous sequencer-based scan mode: 2 groups of analog signals conversions can be programmed to differentiate background and high-priority real-time conversions
- Each ADC support multiple trigger inputs for synchronization with on-chip timers and external signals
- Results stored into a data register or in RAM with DMA controller support
- Data pre-processing: left/right alignment and per channel offset compensation
- Built-in oversampling unit for enhanced SNR
- Channel-wise programmable sampling time
- Analog watchdog for automatic voltage monitoring, generating interrupts and trigger for selected timers
- Hardware assistant to prepare the context of the injected channels to allow fast context switching
- Flexible sample time control
- Hardware gain and offset compensation


### 3.18.1 Temperature sensor

The temperature sensor (TS) generates a voltage $\mathrm{V}_{\mathrm{TS}}$ that varies linearly with temperature. The temperature sensor is internally connected to the ADC1_IN16 input channel which is used to convert the sensor output voltage into a digital value.

The sensor provides good linearity but it has to be calibrated to obtain good overall accuracy of the temperature measurement. As the offset of the temperature sensor varies from chip to chip due to process variation, the uncalibrated internal temperature sensor is suitable for applications that detect temperature changes only.
To improve the accuracy of the temperature sensor measurement, each device is individually factory-calibrated by ST. The temperature sensor factory calibration data are stored by ST in the system memory area, accessible in read-only mode.Table 5. Temperature sensor calibration values

| Calibration value name | Description | Memory address |
| :--: | :-- | :-- |
| TS_CAL1 | TS ADC raw data acquired at a <br> temperature of $30^{\circ} \mathrm{C}\left( \pm 5^{\circ} \mathrm{C}\right)$, <br> $\mathrm{V}_{\mathrm{DDA}}=\mathrm{V}_{\mathrm{REF}+}=3.0 \mathrm{~V}( \pm 10 \mathrm{mV})$ | $0 \times 1$ FFF 75A8 - $0 \times 1$ FFF 75A9 |
| TS_CAL2 | TS ADC raw data acquired at a <br> temperature of $130^{\circ} \mathrm{C}\left( \pm 5^{\circ} \mathrm{C}\right)$, <br> $\mathrm{V}_{\mathrm{DDA}}=\mathrm{V}_{\mathrm{REF}+}=3.0 \mathrm{~V}( \pm 10 \mathrm{mV})$ | $0 \times 1$ FFF 75CA - $0 \times 1$ FFF 75CB |

# 3.18.2 Internal voltage reference ( $\mathrm{V}_{\text {REFINT }}$ ) 

The internal voltage reference (VREFINT) provides a stable (bandgap) voltage output for the ADC and the comparators. The VREFINT is internally connected to the ADC1_IN18 input channel. The precise voltage of VREFINT is individually measured for each part by ST during production test and stored in the system memory area. It is accessible in read-only mode.

Table 6. Internal voltage reference calibration values

| Calibration value name | Description | Memory address |
| :--: | :-- | :-- |
| VREFINT | Raw data acquired at a <br> temperature of $30^{\circ} \mathrm{C}\left( \pm 5^{\circ} \mathrm{C}\right)$, <br> $\mathrm{V}_{\mathrm{DDA}}=\mathrm{V}_{\mathrm{REF}+}=3.0 \mathrm{~V}( \pm 10 \mathrm{mV})$ | $0 \times 1$ FFF 75AA - $0 \times 1$ FFF 75AB |

### 3.18.3 $\mathrm{V}_{\mathrm{BAT}}$ battery voltage monitoring

This embedded hardware enables the application to measure the $\mathrm{V}_{\mathrm{BAT}}$ battery voltage using the internal ADC1_IN17 channel. As the $\mathrm{V}_{\mathrm{BAT}}$ voltage may be higher than the $\mathrm{V}_{\mathrm{DDA}}$, and thus outside the ADC input range, the VBAT pin is internally connected to a bridge divider by 3. As a consequence, the converted digital value is one third of the $\mathrm{V}_{\mathrm{BAT}}$ voltage.

### 3.19 Digital to analog converter (DAC)

Four 12 bit DAC channels ( 2 external buffered and 2 internal unbuffered) can be used to convert digital signals into analog voltage signal outputs. The chosen design structure is composed of integrated resistor strings and an amplifier in inverting configuration.This digital interface supports the following features:

- Up to two DAC output channels
- 8-bit or 12-bit output mode
- Buffer offset calibration (factory and user trimming)
- Left or right data alignment in 12-bit mode
- Synchronized update capability
- Noise-wave generation
- Triangular-wave generation
- Saw tooth wave generation
- Dual DAC channel independent or simultaneous conversions
- DMA capability for each channel
- External triggers for conversion
- Sample and hold low-power mode, with internal or external capacitor
- Up to 1 Msps for external output and 15 Msps for internal output

The DAC channels are triggered through the timer update outputs that are also connected to different DMA channels.

# 3.20 Voltage reference buffer ( $\mathrm{V}_{\text {REFBUF }}$ ) 

The STM32G431x6/x8/xB devices embed a voltage reference buffer which can be used as voltage reference for ADC, DACs and also as voltage reference for external components through the VREF + pin.
The internal voltage reference buffer supports three voltages:

- 2.048 V
- 2.5 V
- 2.9 V

An external voltage reference can be provided through the VREF + pin when the internal voltage reference buffer is off.
The VREF + pin is double-bonded with $\mathrm{V}_{\mathrm{DDA}}$ on some packages. In these packages the internal voltage reference buffer is not available.

Figure 3. Voltage reference buffer
![img-3.jpeg](img-3.jpeg)# 3.21 Comparators (COMP) 

The STM32G431x6/x8/xB devices embed four rail-to-rail comparators with programmable reference voltage (internal or external), hysteresis.

The reference voltage can be one of the following:

- External I/O
- DAC output channels
- Internal reference voltage or submultiple (1/4, 1/2, 3/4).

All comparators can wake up from Stop mode, generate interrupts and breaks for the timers.

### 3.22 Operational amplifier (OPAMP)

The STM32G431x6/x8/xB devices embed three operational amplifiers with external or internal follower routing and PGA capability.

The operational amplifier features:

- 13 MHz bandwidth
- Rail-to-rail input/output
- PGA with a non-inverting gain ranging of $2,4,8,16,32$ or 64 or inverting gain ranging of $-1,-3,-7,-15,-31$ or -63


### 3.23 Random number generator (RNG)

All devices embed an RNG that delivers 32-bit random numbers generated by an integrated analog circuit.

### 3.24 Timers and watchdogs

The STM32G431x6/x8/xB devices include two advanced motor control timers, up to six general-purpose timers, two basic timers, one low-power timer, two watchdog timers and a SysTick timer. The table below compares the features of the advanced motor control, general purpose and basic timers.

Table 7. Timer feature comparison

| Timer type | Timer | Counter <br> resolution | Counter <br> type | Prescaler <br> factor | DMA <br> request <br> generation | Capture/ <br> compare <br> channels | Complementary <br> outputs |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Advanced <br> motor <br> control | TIM1, TIM8 | 16-bit | Up, <br> down, <br> Up/down | Any integer <br> between 1 and <br> 65536 | Yes | 4 | 4 |
| General- <br> purpose | TIM2 | 32-bit | Up, <br> down, <br> Up/down | Any integer <br> between 1 and <br> 65536 | Yes | 4 | No |
| General- <br> purpose | TIM3, TIM4 | 16-bit | Up, <br> down, <br> Up/down | Any integer <br> between 1 and <br> 65536 | Yes | 4 | No |Table 7. Timer feature comparison (continued)

| Timer type | Timer | Counter <br> resolution | Counter <br> type | Prescaler <br> factor | DMA <br> request <br> generation | Capture/ <br> compare <br> channels | Complementary <br> outputs |
| :-- | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| General- <br> purpose | TIM15 | 16-bit | Up | Any integer <br> between 1 and <br> 65536 | Yes | 2 | 1 |
| General- <br> purpose | TIM16, TIM17 | 16-bit | Up | Any integer <br> between 1 and <br> 65536 | Yes | 1 | 1 |
| Basic | TIM6, TIM7 | 16-bit | Up | Any integer <br> between 1 and <br> 65536 | Yes | 0 | No |

# 3.24.1 Advanced motor control timer (TIM1, TIM8) 

The advanced motor control timers can each be seen as a four-phase PWM multiplexed on 8 channels. They have complementary PWM outputs with programmable inserted dead-times. They can also be seen as complete general-purpose timers.

The 4 independent channels can be used for:

- Input capture
- Output compare
- PWM generation (edge or center-aligned modes) with full modulation capability $(0-100 \%)$
- One-pulse mode output

In debug mode, the advanced motor control timer counter can be frozen and the PWM outputs disabled in order to turn off any power switches driven by these outputs.

Many features are shared with the general-purpose TIMx timers (described in Section 3.24.2) using the same architecture, so the advanced motor control timers can work together with the TIMx timers via the Timer Link feature for synchronization or event chaining.# 3.24.2 General-purpose timers (TIM2, TIM3, TIM4, TIM15, TIM16, TIM17) 

There are up to six synchronizable general-purpose timers embedded in the STM32G431x6/x8/xB devices (see Table 7 for differences). Each general-purpose timer can be used to generate PWM outputs, or act as a simple time base.

- TIM2, TIM3, and TIM4

They are full-featured general-purpose timers:

- TIM2 has a 32-bit auto-reload up/downcounter and 32-bit prescaler
- TIM3 and TIM4 have 16-bit auto-reload up/downcounter and 16-bit prescaler.

These timers feature 4 independent channels for input capture/output compare, PWM or one-pulse mode output. They can work together, or with the other general-purpose timers via the Timer Link feature for synchronization or event chaining.
The counters can be frozen in debug mode.
All have independent DMA request generation and support quadrature encoders.

- TIM15, 16 and 17

They are general-purpose timers with mid-range features:
They have 16-bit auto-reload upcounters and 16-bit prescalers.

- TIM15 has 2 channels and 1 complementary channel
- TIM16 and TIM17 have 1 channel and 1 complementary channel

All channels can be used for input capture/output compare, PWM or one-pulse mode output.
The timers can work together via the Timer Link feature for synchronization or event chaining. The timers have independent DMA request generation.
The counters can be frozen in debug mode.

### 3.24.3 Basic timers (TIM6 and TIM7)

The basic timers are mainly used for DAC trigger generation. They can also be used as generic 16-bit timebases.# 3.24.4 Low-power timer (LPTIM1) 

The devices embed a low-power timer. This timer has an independent clock and are running in Stop mode if it is clocked by LSE, LSI or an external clock. It is able to wakeup the system from Stop mode.

LPTIM1 is active in Stop mode.
This low-power timer supports the following features:

- 16-bit up counter with 16-bit autoreload register
- 16-bit compare register
- Configurable output: pulse, PWM
- Continuous/ one shot mode
- Selectable software/hardware input trigger
- Selectable clock source
- Internal clock sources: LSE, LSI, HSI16 or APB clock
- External clock source over LPTIM input (working even with no internal clock source running, used by pulse counter application).
- Programmable digital glitch filter
- Encoder mode


### 3.24.5 Independent watchdog (IWDG)

The independent watchdog is based on a 12-bit downcounter and an 8-bit prescaler. It is clocked from an independent 32 kHz internal RC (LSI) and as it operates independently from the main clock, it can operate in Stop and Standby modes. It can be used either as a watchdog to reset the device when a problem occurs, or as a free running timer for application timeout management. It is hardware or software configurable through the option bytes. The counter can be frozen in debug mode.

### 3.24.6 System window watchdog (WWDG)

The window watchdog is based on a 7-bit downcounter that can be set as free running. It can be used as a watchdog to reset the device when a problem occurs. It is clocked from the main clock. It has an early warning interrupt capability and the counter can be frozen in debug mode.

### 3.24.7 SysTick timer

This timer is dedicated to real-time operating systems, but could also be used as a standard down counter. It features:

- A 24-bit down counter
- Autoreload capability
- Maskable system interrupt generation when the counter reaches 0 .
- Programmable clock source# 3.25 Real-time clock (RTC) and backup registers 

The RTC supports the following features:

- Calendar with subsecond, seconds, minutes, hours (12 or 24 format), week day, date, month, year, in BCD (binary-coded decimal) format.
- Automatic correction for 28, 29 (leap year), 30, and 31 days of the month.
- Two programmable alarms.
- On-the-fly correction from 1 to 32767 RTC clock pulses. This can be used to synchronize it with a master clock.
- Reference clock detection: a more precise second source clock ( 50 or 60 Hz ) can be used to enhance the calendar precision.
- Digital calibration circuit with 0.95 ppm resolution, to compensate for quartz crystal inaccuracy.
- Timestamp feature which can be used to save the calendar content. This function can be triggered by an event on the timestamp pin, or by a tamper event, or by a switch to $\mathrm{V}_{\text {BAT }}$ mode.
- 17-bit auto-reload wakeup timer (WUT) for periodic events with programmable resolution and period.

The RTC is supplied through a switch that takes power either from the $\mathrm{V}_{\mathrm{DD}}$ supply when present or from the VBAT pin.
The RTC clock sources can be:

- A 32.768 kHz external crystal (LSE)
- An external resonator or oscillator (LSE)
- The internal low power RC oscillator (LSI, with typical frequency of 32 kHz )
- The high-speed external clock (HSE) divided by 32.

The RTC is functional in $\mathrm{V}_{\mathrm{BAT}}$ mode and in all low-power modes when it is clocked by the LSE. When clocked by the LSI, the RTC is not functional in $\mathrm{V}_{\mathrm{BAT}}$ mode, but is functional in all low-power modes except Shutdown mode.

All RTC events (Alarm, WakeUp Timer, Timestamp) can generate an interrupt and wakeup the device from the low-power modes.

### 3.26 Tamper and backup registers (TAMP)

- 16 32-bit backup registers, retained in all low-power modes and also in $\mathrm{V}_{\text {BAT }}$ mode. They can be used to store sensitive data as their content is protected by an tamper detection circuit. They are not reset by a system or power reset, or when the device wakes up from Standby or Shutdown mode.
- Up to three tamper pins for external tamper detection events. The external tamper pins can be configured for edge detection, edge and level, level detection with filtering.
- Five internal tampers events.
- Any tamper detection can generate a RTC timestamp event.
- Any tamper detection erases the backup registers.
- Any tamper detection can generate an interrupt and wake-up the device from all lowpower modes.# 3.27 Infrared transmitter 

The STM32G431x6/x8/xB devices provide an infrared transmitter solution. The solution is based on internal connections between TIM16 and TIM17 as shown in the figure below.

TIM17 is used to provide the carrier frequency and TIM16 provides the main signal to be sent. The infrared output signal is available on PB9 or PA13.

To generate the infrared remote control signals, TIM16 channel 1 and TIM17 channel 1 must be properly configured to generate correct waveforms. All standard IR pulse modulation modes can be obtained by programming the two timers output compare channels.

Figure 4. Infrared transmitter
![img-4.jpeg](img-4.jpeg)# 3.28 Inter-integrated circuit interface ( $\mathrm{I}^{2} \mathrm{C}$ ) 

The device embeds three I2Cs. Refer to Table 8: I2C implementation for the features implementation.
The $I^{2} \mathrm{C}$ bus interface handles communications between the microcontroller and the serial $I^{2} \mathrm{C}$ bus. It controls all $I^{2} \mathrm{C}$ bus-specific sequencing, protocol, arbitration and timing.
The I2C peripheral supports:

- I²C-bus specification and user manual rev. 5 compatibility:
- Slave and master modes, multimaster capability
- Standard-mode (Sm), with a bitrate up to $100 \mathrm{kbit} / \mathrm{s}$
- Fast-mode (Fm), with a bitrate up to $400 \mathrm{kbit} / \mathrm{s}$
- Fast-mode Plus (Fm+), with a bitrate up to $1 \mathrm{Mbit} / \mathrm{s}$ and 20 mA output drive I/Os
- 7-bit and 10-bit addressing mode, multiple 7-bit slave addresses
- Programmable setup and hold times
- Optional clock stretching
- System management bus (SMBus) specification rev 2.0 compatibility:
- Hardware PEC (packet error checking) generation and verification with ACK control
- Address resolution protocol (ARP) support
- SMBus alert
- Power system management protocol (PMBus ${ }^{\mathrm{TM}}$ ) specification rev 1.1 compatibility
- Independent clock: a choice of independent clock sources allowing the I2C communication speed to be independent from the PCLK reprogramming.
- Wakeup from Stop mode on address match
- Programmable analog and digital noise filters
- 1-byte buffer with DMA capability

Table 8. I2C implementation

| I2C features $^{(1)}$ | I2C1 | I2C2 | I2C3 |
| :-- | :--: | :--: | :--: |
| Standard-mode (up to $100 \mathrm{kbit} / \mathrm{s}$ ) | X | X | X |
| Fast-mode (up to $400 \mathrm{kbit} / \mathrm{s}$ ) | X | X | X |
| Fast-mode Plus with 20mA output drive I/Os (up to $1 \mathrm{Mbit} / \mathrm{s}$ ) | X | X | X |
| Programmable analog and digital noise filters | X | X | X |
| SMBus/PMBus hardware support | X | X | X |
| Independent clock | X | X | X |
| Wakeup from Stop mode on address match | X | X | X |

1. X: supported# 3.29 Universal synchronous/asynchronous receiver transmitter (USART) 

The STM32G431x6/x8/xB devices have three embedded universal synchronous receiver transmitters (USART1, USART2 and USART3) and one universal asynchronous receiver transmitters (UART4).

These interfaces provide asynchronous communication, IrDA SIR ENDEC support, multiprocessor communication mode, single-wire half-duplex communication mode and have LIN master/slave capability. They provide hardware management of the CTS and RTS signals, and RS485 driver enable.

The USART1, USART2 and USART3 also provide a Smartcard mode (ISO 7816 compliant) and an SPI-like communication capability.

The USART comes with a Transmit FIFO (TXFIFO) and a Receive FIFO (RXFIFO). FIFO mode is enabled by software and is disabled by default.

All USART have a clock domain independent from the CPU clock, allowing the USARTx $(x=1,2,3,4)$ to wake up the MCU from Stop mode. The wakeup from Stop mode can be done on:

- Start bit detection
- Any received data frame
- A specific programmed data frame
- Some specific TXFIFO/RXFIFO status interrupts when FIFO mode is enabled

All USART interfaces can be served by the DMA controller.
Table 9. USART/UART/LPUART features

| USART modes/features ${ }^{(1)}$ | USART1 | USART2 | USART3 | UART4 | LPUART1 |
| :--: | :--: | :--: | :--: | :--: | :--: |
| Hardware flow control for modem | $X$ | $X$ | $X$ | $X$ | $X$ |
| Continuous communication using DMA | $X$ | $X$ | $X$ | $X$ | $X$ |
| Multiprocessor communication | $X$ | $X$ | $X$ | $X$ | $X$ |
| Synchronous mode | $X$ | $X$ | $X$ | - | - |
| Smartcard mode | $X$ | $X$ | $X$ | - | - |
| Single-wire half-duplex communication | $X$ | $X$ | $X$ | $X$ | $X$ |
| IrDA SIR ENDEC block | $X$ | $X$ | $X$ | $X$ | - |
| LIN mode | $X$ | $X$ | $X$ | $X$ | - |
| Dual clock domain | $X$ | $X$ | $X$ | $X$ | $X$ |
| Wakeup from Stop mode | $X$ | $X$ | $X$ | $X$ | $X$ |
| Receiver timeout interrupt | $X$ | $X$ | $X$ | $X$ | - |
| Modbus communication | $X$ | $X$ | $X$ | $X$ | - |
| Auto baud rate detection | $X$ (4 modes) |  |  |  | - |
| Driver Enable | $X$ | $X$ | $X$ | $X$ | $X$ |
| LPUART/USART data length | 7,8 and 9 bits |  |  |  |  |Table 9. USART/UART/LPUART features (continued)

| USART modes/features ${ }^{(1)}$ | USART1 | USART2 | USART3 | UART4 | LPUART1 |
| :-- | :--: | :--: | :--: | :--: | :--: |
| Tx/Rx FIFO | X |  |  |  |  |
| Tx/Rx FIFO size | 8 |  |  |  |  |

1. $X=$ supported.

# 3.30 Low-power universal asynchronous receiver transmitter (LPUART) 

The STM32G431x6/x8/xB devices embed one Low-Power UART. The LPUART supports asynchronous serial communication with minimum power consumption. It supports halfduplex single-wire communication and modem operations (CTS/RTS). It allows multiprocessor communication.
The LPUART comes with a Transmit FIFO (TXFIFO) and a Receive FIFO (RXFIFO). FIFO mode is enabled by software and is disabled by default. It has a clock domain independent from the CPU clock, and can wakeup the system from Stop mode. The wake up from Stop mode can be done on:

- Start bit detection
- Any received data frame
- A specific programmed data frame
- Some specific TXFIFO/RXFIFO status interrupts when FIFO mode is enabled

Only a 32.768 kHz clock (LSE) is needed to allow LPUART communication up to 9600 baud. Therefore, even in Stop mode, the LPUART can wait for an incoming frame while having an extremely low energy consumption. Higher speed clock can be used to reach higher baudrates.

The LPUART interface can be served by the DMA controller.

### 3.31 Serial peripheral interface (SPI)

Three SPI interfaces allow communication up to 75 Mbits/s in master and up to 41 Mbits/s in slave, half-duplex, full-duplex and simplex modes. The 3-bit prescaler gives 8 master mode frequencies and the frame size is configurable from 4 bits to 16 bits. The SPI interfaces support NSS pulse mode, TI mode and hardware CRC calculation.
Two standard I²S interfaces (multiplexed with SPI2 and SPI3) supporting four different audio standards can operate as master or slave at half-duplex communication modes. They can be configured to transfer 16 and 24 or 32 bits with 16-bit or 32-bit data resolution and synchronized by a specific signal. Audio sampling frequency from 8 kHz up to 192 kHz can be set by 8-bit programmable linear prescaler. When operating in master mode it can output a clock for an external audio component at 256 times the sampling frequency.

All SPI interfaces can be served by the DMA controller.# 3.32 Serial audio interfaces (SAI) 

The device embeds 1 SAI. The SAI bus interface handles communications between the microcontroller and the serial audio protocol.

### 3.32.1 SAI peripheral supports

- Two independent audio sub-blocks which can be transmitters or receivers with their respective FIFO.
- 8-word integrated FIFOs for each audio sub-block.
- Synchronous or asynchronous mode between the audio sub-blocks.
- Master or slave configuration independent for both audio sub-blocks.
- Clock generator for each audio block to target independent audio frequency sampling when both audio sub-blocks are configured in master mode.
- Data size configurable: 8-, 10-, 16-, 20-, 24-, 32-bit.
- Peripheral with large configurability and flexibility allowing to target as example the following audio protocol: I2S, LSB or MSB-justified, PCM/DSP, TDM, AC'97 and SPDIF out.
- Up to 16 slots available with configurable size and with the possibility to select which ones are active in the audio frame.
- Number of bits by frame may be configurable.
- Frame synchronization active level configurable (offset, bit length, level).
- First active bit position in the slot is configurable.
- LSB first or MSB first for data transfer.
- Mute mode.
- Stereo/Mono audio frame capability.
- Communication clock strobing edge configurable (SCK).
- Error flags with associated interrupts if enabled respectively.
- Overrun and underrun detection.
- Anticipated frame synchronization signal detection in slave mode.
- Late frame synchronization signal detection in slave mode.
- Codec not ready for the AC'97 mode in reception.
- Interruption sources when enabled:
- Errors.
- FIFO requests.
- DMA interface with 2 dedicated channels to handle access to the dedicated integrated FIFO of each SAI audio sub-block.

Table 10. SAI features implementation

| SAI features | Support ${ }^{(1)}$ |
| :-- | :--: |
| I2S, LSB or MSB-justified, PCM/DSP, TDM, AC'97 | X |
| Mute mode | X |
| Stereo/Mono audio frame capability | X |
| 16 slots | X |Table 10. SAI features implementation (continued)

| SAI features | Support $^{(1)}$ |
| :-- | :--: |
| Data size configurable: $8-, 10-, 16-, 20-, 24-, 32$-bit | X |
| FIFO size | $X(8$ word $)$ |
| SPDIF | $X$ |

1. $X$ : supported.

# 3.33 Controller area network (FDCAN1) 

The controller area network (CAN) subsystem consists of one CAN module and message RAM memory.

The CAN module (FDCAN) is compliant with ISO 11898-1 (CAN protocol specification version 2.0 part A, B) and CAN FD protocol specification version 1.0.

A 1-Kbyte message RAM memory implements filters, receive FIFOs, receive buffers, transmit event FIFOs, transmit buffers.

### 3.34 Universal serial bus (USB)

The STM32G431x6/x8/xB devices embed a full-speed USB device peripheral compliant with the USB specification version 2.0. The internal USB PHY supports USB FS signaling, embedded DP pull-up and also battery charging detection according to Battery Charging Specification Revision 1.2. The USB interface implements a full-speed ( $12 \mathrm{Mbit} / \mathrm{s}$ ) function interface with added support for USB 2.0 Link Power Management. It has softwareconfigurable endpoint setting with packet memory up-to 1 Kbyte and suspend/resume support. It requires a precise 48 MHz clock which can be generated from the internal main PLL (the clock source must use a HSE crystal oscillator) or by the internal 48 MHz oscillator in automatic trimming mode. The synchronization for this oscillator can be taken from the USB data stream itself (SOF signalization) which allows crystal less operation.

### 3.35 USB Type-C ${ }^{\text {TM }}$ / USB Power Delivery controller (UCPD)

The device embeds one controller (UCPD) compliant with USB Type-C Rev. 1.2 and USB Power Delivery Rev. 3.0 specifications.

The controller uses specific I/Os supporting the USB Type-C and USB Power Delivery requirements, featuring:

- USB Type-C pull-up (Rp, all values) and pull-down (Rd) resistors
- "Dead battery" support
- USB Power Delivery message transmission and reception
- FRS (fast role swap) supportThe digital controller handles notably:

- USB Type-C level detection with de-bounce, generating interrupts
- FRS detection, generating an interrupt
- Byte-level interface for USB Power Delivery payload, generating interrupts (DMA compatible)
- USB Power Delivery timing dividers (including a clock pre-scaler)
- CRC generation/checking
- 4b5b encode/decode
- Ordered sets (with a programmable ordered set mask at receive)
- Frequency recovery in receiver during preamble

The interface offers low-power operation compatible with Stop mode, maintaining the capacity to detect incoming USB Power Delivery messages and FRS signaling.

# 3.36 Clock recovery system (CRS) 

The devices embed a special block which allows automatic trimming of the internal 48 MHz oscillator to guarantee its optimal accuracy over the whole device operational range. This automatic trimming is based on the external synchronization signal, which could be either derived from USB SOF signalization, from LSE oscillator, from an external signal on CRS_SYNC pin or generated by user software. For faster lock-in during startup it is also possible to combine automatic trimming with manual trimming action.

### 3.37 Development support

### 3.37.1 Serial wire JTAG debug port (SWJ-DP)

The Arm SWJ-DP interface is embedded, and is a combined JTAG and serial wire debug port that enables either a serial wire debug or a JTAG probe to be connected to the target.

Debug is performed using 2 pins only instead of 5 required by the JTAG (JTAG pins could be re-use as GPIO with alternate function): the JTAG TMS and TCK pins are shared with SWDIO and SWCLK, respectively, and a specific sequence on the TMS pin is used to switch between JTAG-DP and SW-DP.

### 3.37.2 Embedded trace macrocell ${ }^{\text {TM }}$

The Arm embedded trace macrocell provides a greater visibility of the instruction and data flow inside the CPU core by streaming compressed data at a very high rate from the STM32G431x6/x8/xB devices through a small number of ETM pins to an external hardware trace port analyzer (TPA) device. Real-time instruction and data flow activity be recorded and then formatted for display on the host computer that runs the debugger software. TPA hardware is commercially available from common development tool vendors.

The Embedded trace macrocell operates with third party debugger software tools.# 4 Pinouts and pin description 

### 4.1 UFQFPN32 pinout description

Figure 5. STM32G431x6/x8/xB UFQFPN32 pinout
![img-5.jpeg](img-5.jpeg)

1. The above figure shows the package top view.

### 4.2 LQFP32 pinout description

Figure 6. STM32G431x6/x8/xB LQFP32 pinout
![img-6.jpeg](img-6.jpeg)

1. The above figure shows the package top view.# 4.3 UFQFPN48 pinout description 

Figure 7. STM32G431x6/x8/xB UFQFPN48 pinout
![img-7.jpeg](img-7.jpeg)

1. The above figure shows the package top view.
2. VSS pads are connected to the exposed pad.

### 4.4 LQFP48 pinout description

Figure 8. STM32G431x6/x8/xB LQFP48 pinout
![img-8.jpeg](img-8.jpeg)

1. The above figure shows the package top view.# 4.5 WLCSP49 ballout description 

Figure 9. STM32G431x6/x8/xB WLCSP49 ballout
![img-9.jpeg](img-9.jpeg)

1. The above figure shows the package top view.

### 4.6 LQFP64 pinout description

Figure 10. STM32G431x6/x8/xB LQFP64 pinout
![img-10.jpeg](img-10.jpeg)

1. The above figure shows the package top view.# 4.7 UFBGA64 ballout description 

Figure 11. STM32G431x6/x8/xB UFBGA64 ballout
![img-11.jpeg](img-11.jpeg)
![img-12.jpeg](img-12.jpeg)

1. The above figure shows the package top view.# 4.8 LQFP80 pinout description 

Figure 12. STM32G431x6/x8/xB LQFP80 pinout
![img-13.jpeg](img-13.jpeg)

1. The above figure shows the package top view.# 4.9 LQFP100 pinout description 

Figure 13. STM32G431x6/x8/xB LQFP100 pinout
![img-14.jpeg](img-14.jpeg)

1. The above figure shows the package top view.# 4.10 Pin definition 

Table 11. Legend/abbreviations used in the pinout table

| Name | Abbreviation | Definition |
| :--: | :--: | :--: |
| Pin name | Unless otherwise specified in brackets below the pin name, the pin function during and after reset is the same as the actual pin name |  |
| Pin type | S | Supply pin |
|  | I | Input only pin |
|  | I/O | Input / output pin |
| I/O structure | FT | 5 V tolerant I/O |
|  | TT | 3.6 V tolerant I/O |
|  | B | Dedicated BOOT0 pin |
|  | NRST | Bidirectional reset pin with embedded weak pull-up resistor |
|  | Option for TT or FT I/Os |  |
|  | _a ${ }^{(1)}$ | I/O, with Analog switch function supplied by $\mathrm{V}_{\text {DDA }}$ |
|  | _c | I/O, USB Type-C PD capable |
|  | _d | I/O, USB Type-C PD Dead Battery function |
|  | _f(2) | I/O, Fm+ capable |
|  | _u ${ }^{(3)}$ | I/O, with USB function |
| Notes | Unless otherwise specified by a note, all I/Os are set as floating inputs during and after reset |  |
| Pin functions | Alternate functions | Functions selected through GPIOx_AFR registers |
|  | Additional functions | Functions directly selected/enabled through peripheral registers |

1. The related I/O structures in Table 12 are: FT_a, FT_fa, TT_a.
2. The related I/O structures in Table 12 are: FT_f, FT_fa.
3. The related I/O structures in Table 12 are FT_u.Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| UFQFPN32 | LQFP32 | UFQFPN48 | LQFP48 | WLCSP49 | LQFP64 | UFBGA64 | LQFP80 |  |  |  |  |  |  |
| - | - | - | - | - | - | - | 1 | PE2 | I/O | FT | - | TRACECK, TIM3_CH1, SAI1_CK1, SAI1_MCLK_A, EVENTOUT | - |
| - | - | - | - | - | - | - | 2 | PE3 | I/O | FT | - | TRACED0, TIM3_CH2, SAI1_SD_B, EVENTOUT | - |
| - | - | - | - | - | - | - | 3 | PE4 | I/O | FT | - | TRACED1, TIM3_CH3, SAI1_D2, SAI1_FS_A, EVENTOUT | - |
| - | - | - | - | - | - | - | 4 | PE5 | I/O | FT | - | TRACED2, TIM3_CH4, SAI1_CK2, SAI1_SCK_A, EVENTOUT | - |
| - | - | - | - | - | - | - | 5 | PE6 | I/O | FT | - | TRACED3, SAI1_D1, SAI1_SD_A, EVENTOUT | WKUP3, <br> RTC_TAMP3 |
| - | - | 1 | 1 | B7 | 1 | C2 | 1 | 6 | VBAT | S | - | - | - |
| - | - | 2 | 2 | C5 | 2 | B1 | 2 | 7 | PC13 | I/O | FT | (2) <br> (3) | TIM1_BKIN, TIM1_CH1N, TIM8_CH4N, EVENTOUT | WKUP2, <br> RTC_TAMP1, <br> RTC_TS, <br> RTC_OUT1 |
| - | - | 3 | 3 | C7 | 3 | C1 | 3 | 8 | PC14- <br> OSC32_IN | I/O | FT | (2) <br> (3) | EVENTOUT | OSC32_IN |
| - | - | 4 | 4 | D7 | 4 | D1 | 4 | 9 | PC15- <br> OSC32_OUT | I/O | FT | (2) <br> (3) | EVENTOUT | OSC32_OUT |
| - | - | - | - | - | - | - | - | 10 | PF9 | I/O | FT | - | TIM15_CH1, SPI2_SCK, SAI1_FS_B, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 11 | PF10 | I/O | FT | - | TIM15_CH2, SPI2_SCK, SAI1_D3, EVENTOUT | - |
| 2 | 2 | 5 | 5 | E7 | 5 | E1 | 5 | 12 | PF0-OSC_IN | I/O | FT_fa | - | I2C2_SDA, <br> SPI2_NSS/ <br> I2S2_WS, TIM1_CH3N, EVENTOUT | ADC1_IN10, OSC_IN |
| 3 | 3 | 6 | 6 | E6 | 6 | F1 | 6 | 13 | PF1- <br> OSC_OUT | I/O | FT_a | - | SPI2_SCK/ <br> I2S2_CK, EVENTOUT | ADC2_IN10, COMP3_INM, OSC_OUT |
| 4 | 4 | 7 | 7 | C6 | 7 | D2 | 7 | 14 | PG10-NRST | I/O | $\begin{aligned} & \text { NRST } \\ & \text { (4) } \end{aligned}$ | - | MCO, EVENTOUT | NRST |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| LQFP32 | LQFP48 | LQFP48 | WLCSP49 | LQFP64 | LCFP80 | LQFP100 |  |  |  |  |  |  |  |
| - | - | - | - | - | 8 | E2 | 8 | 15 | PC0 | I/O | FT_a | - | LPTIM1_IN1, <br> TIM1_CH1, <br> LPUART1_RX, <br> EVENTOUT | ADC12_IN6, COMP3_INM |
| - | - | - | - | - | 9 | C3 | 9 | 16 | PC1 | I/O | TT_a | - | LPTIM1_OUT, <br> TIM1_CH2, <br> LPUART1_TX, <br> SAI1_SD_A, <br> EVENTOUT | ADC12_IN7, COMP3_INP |
| - | - | - | - | - | 10 | D3 | 10 | 17 | PC2 | I/O | FT_a | - | LPTIM1_IN2, <br> TIM1_CH3, <br> COMP3_OUT, <br> EVENTOUT | ADC12_IN8 |
| - | - | - | - | - | 11 | G1 | 11 | 18 | PC3 | I/O | FT_a | - | LPTIM1_ETR, <br> TIM1_CH4, <br> SAI1_D1, <br> TIM1_BKIN2, <br> SAI1_SD_A, <br> EVENTOUT | ADC12_IN9 |
| - | - | - | - | - | - | - | - | 19 | PF2 | I/O | FT | - | I2C2_SMBA, EVENTOUT | - |
| 5 | 5 | 8 | 8 | F7 | 12 | F2 | 12 | 20 | PA0 | I/O | TT_a | - | TIM2_CH1, <br> USART2_CTS, <br> COMP1_OUT, <br> TIM8_BKIN, TIM8_ETR, <br> TIM2_ETR, EVENTOUT | ADC12_IN1, COMP1_INM, COMP3_INP, RTC_TAMP2, WKUP1 |
| 6 | 6 | 9 | 9 | D6 | 13 | E3 | 13 | 21 | PA1 | I/O | TT_a | - | RTC_REFIN, <br> TIM2_CH2, <br> USART2_RTS_DE, <br> TIM15_CH1N, <br> EVENTOUT | ADC12_IN2, COMP1_INP, OPAMP1_VINP, OPAMP3_VINP |
| 7 | 7 | 10 | 10 | F6 | 14 | F3 | 14 | 22 | PA2 | I/O | TT_a | - | TIM2_CH3, USART2_TX, COMP2_OUT, TIM15_CH1, LPUART1_TX, UCPD1_FRSTX, EVENTOUT | ADC1_IN3, COMP2_INM, OPAMP1_VOUT, WKUP4/ LSCO |
| - | - | - | - | - | 15 | G2 | 15 | 23 | VSS | S | - | - | - | - |
| - | - | - | - | - | 16 | H1 | 16 | 24 | VDD | S | - | - | - | - |
| 8 | 8 | 11 | 11 | G7 | 17 | H2 | 17 | 25 | PA3 | I/O | TT_a | - | TIM2_CH4, SAI1_CK1, USART2_RX, TIM15_CH2, LPUART1_RX, SAI1_MCLK_A, EVENTOUT | ADC1_IN4, COMP2_INP, OPAMP1_VINM/ OPAMP1_VINP |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 9 | 9 | 12 | 12 | E5 | 18 | D4 | 18 | 26 | PA4 | I/O | TT_a | - | TIM3_CH2, SPI1_NSS, SPI3_NSS/ <br> I2S3_WS, USART2_CK, SAI1_FS_B, EVENTOUT | ADC2_IN17, DAC1_OUT1, COMP1_INM |
| 10 | 10 | 13 | 13 | F5 | 19 | E4 | 19 | 27 | PA5 | I/O | TT_a | - | TIM2_CH1, TIM2_ETR, SPI1_SCK, <br> UCPD1_FRSTX, EVENTOUT | ADC2_IN13, DAC1_OUT2, COMP2_INM, OPAMP2_VINM |
| 11 | 11 | 14 | 14 | G6 | 20 | G3 | 20 | 28 | PA6 | I/O | TT_a | - | TIM16_CH1, TIM3_CH1, TIM8_BKIN, SPI1_MISO, TIM1_BKIN, COMP1_OUT, LPUART1_CTS, EVENTOUT | ADC2_IN3, OPAMP2_VOUT |
| 12 | 12 | 15 | 15 | D5 | 21 | H3 | 21 | 29 | PA7 | I/O | TT_a | - | TIM17_CH1, TIM3_CH2, TIM8_CH1N, SPI1_MOSI, TIM1_CH1N, COMP2_OUT, UCPD1_FRSTX, EVENTOUT | ADC2_IN4, COMP2_INP, OPAMP1_VINP, OPAMP2_VINP |
| - | - | 16 | - | D4 | 22 | D5 | 22 | 30 | PC4 | I/O | FT_fa | - | TIM1_ETR, I2C2_SCL, USART1_TX, EVENTOUT | ADC2_IN5 |
| - | - | - | - | - | 23 | F4 | 23 | 31 | PC5 | I/O | TT_a | - | TIM15_BKIN, SAI1_D3, TIM1_CH4N, USART1_RX, EVENTOUT | ADC2_IN11, OPAMP1_VINM, OPAMP2_VINM, WKUP5 |
| 13 | 13 | 17 | 16 | G5 | 24 | E5 | 24 | 32 | PB0 | I/O | TT_a | - | TIM3_CH3, TIM8_CH2N, TIM1_CH2N, UCPD1_FRSTX, EVENTOUT | ADC1_IN15, COMP4_INP, OPAMP2_VINP, OPAMP3_VINP |
| - | - | 18 | 17 | E4 | 25 | F5 | 25 | 33 | PB1 | I/O | TT_a | - | TIM3_CH4, TIM8_CH3N, TIM1_CH3N, COMP4_OUT, LPUART1_RTS_DE, EVENTOUT | ADC1_IN12, COMP1_INP, OPAMP3_VOUT |
| - | - | 19 | 18 | F4 | 26 | H4 | 26 | 34 | PB2 | I/O | TT_a | - | RTC_OUT2, LPTIM1_OUT, I2C3_SMBA, EVENTOUT | ADC2_IN12, COMP4_INM, OPAMP3_VINM |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| LIFOFPN32 | LQFP32 | LQFPPN48 | LQFP48 | WLCSP48 | LQFP64 | LIFRGA64 | LQFP80 | LQFP100 |  |  |  |  |  |
| 14 | 14 | - | 19 | G4 | 27 | G4 | 27 | 35 | VSSA | S | - | - | - |
| - | - | 20 | 20 | F3 | 28 | G5 | 28 | 36 | VREF+ | S | - | - | - VREFBUF_OUT |
| - | - | 21 | 21 | G3 | 29 | H5 | 29 | 37 | VDDA | S | - | - | - |
| 15 | 15 | - | - | - | - | - | - | - | VDDA/VREF+ | S | - | - | - |
| - | - | - | - | - | - | - | 30 | 38 | PE7 | I/O | TT_a | - | TIM1_ETR, SAI1_SD_B, EVENTOUT | COMP4_INP |
| - | - | - | - | - | - | - | 31 | 39 | PE8 | I/O | FT_a | - | TIM1_CH1N, SAI1_SCK_B, EVENTOUT | COMP4_INM |
| - | - | - | - | - | - | - | 32 | 40 | PE9 | I/O | FT | - | TIM1_CH1_SAI1_FS_B, EVENTOUT | - |
| - | - | - | - | - | - | - | 33 | 41 | PE10 | I/O | FT | - | TIM1_CH2N, SAI1_MCLK_B, EVENTOUT | - |
| - | - | - | - | - | - | - | 34 | 42 | PE11 | I/O | FT | - | TIM1_CH2, EVENTOUT | - |
| - | - | - | - | - | - | - | 35 | 43 | PE12 | I/O | FT | - | TIM1_CH3N, EVENTOUT | - |
| - | - | - | - | - | - | - | 36 | 44 | PE13 | I/O | FT | - | TIM1_CH3, EVENTOUT | - |
| - | - | - | - | - | - | - | 37 | 45 | PE14 | I/O | FT | - | TIM1_CH4, TIM1_BKIN2, EVENTOUT | - |
| - | - | - | - | - | - | - | 38 | 46 | PE15 | I/O | FT | - | TIM1_BKIN, TIM1_CH4N, USART3_RX, EVENTOUT | - |
| - | - | 22 | 22 | F2 | 30 | H6 | 39 | 47 | PB10 | I/O | TT_a | - | TIM2_CH3, USART3_TX, LPUART1_RX, TIM1_BKIN, SAI1_SCK_A, EVENTOUT | OPAMP3_VINM |
| 16 | 16 | - | 23 | G2 | 31 | G7 | 40 | 48 | VSS | S | - | - | - | - |
| 17 | 17 | 23 | 24 | G1 | 32 | H8 | 41 | 49 | VDD | S | - | - | - | - |
| - | - | 24 | 25 | E3 | 33 | H7 | 42 | 50 | PB11 | I/O | FT_a | - | TIM2_CH4, USART3_RX, LPUART1_TX, EVENTOUT | ADC12_IN14 |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| LIFOFPN32 | LQFP32 | LQFPPN48 | LQFP48 | WLCSP49 | LQFP64 | UFBGA64 | LQFP80 |  |  |  |  |  |  |
| - | - | 25 | 26 | F1 | 34 | G8 | 43 | 51 | PB12 | I/O | FT_a | - | I2C2_SMBA, SPI2_NSS/ I2S2_WS, TIM1_BKIN, USART3_CK, LPUART1_RTS_DE, EVENTOUT | ADC1_IN11 |
| - | - | 26 | 27 | E2 | 35 | G6 | 44 | 52 | PB13 | I/O | TT_a | - | SPI2_SCK/I2S2_CK, TIM1_CH1N, USART3_CTS, LPUART1_CTS, EVENTOUT | OPAMP3_VINP |
| - | - | 27 | 28 | D3 | 36 | F8 | 45 | 53 | PB14 | I/O | TT_a | - | TIM15_CH1, SPI2_MISO, TIM1_CH2N, USART3_RTS_DE, COMP4_OUT, EVENTOUT | ADC1_IN5, OPAMP2_VINP |
| - | - | 28 | 29 | E1 | 37 | F7 | 46 | 54 | PB15 | I/O | FT_a | - | RTC_REFIN, TIM15_CH2, TIM15_CH1N, COMP3_OUT, TIM1_CH3N, SPI2_MOSI/ I2S2_SD, EVENTOUT | ADC2_IN15 |
| - | - | - | - | - | - | - | 47 | 55 | PD8 | I/O | FT_a | - | USART3_TX, EVENTOUT | - |
| - | - | - | - | - | - | - | 48 | 56 | PD9 | I/O | FT | - | USART3_RX, EVENTOUT | - |
| - | - | - | - | - | - | - | 49 | 57 | PD10 | I/O | FT | - | USART3_CK, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 58 | PD11 | I/O | FT_a | - | USART3_CTS, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 59 | PD12 | I/O | FT | - | TIM4_CH1, USART3_RTS_DE, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 60 | PD13 | I/O | FT | - | TIM4_CH2, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 61 | PD14 | I/O | TT_a | - | TIM4_CH3, EVENTOUT | OPAMP2_VINP |
| - | - | - | - | - | - | - | - | 62 | PD15 | I/O | FT | - | TIM4_CH4, SPI2_NSS, EVENTOUT | - |
| - | - | - | - | - | - | - | 50 | 63 | VSS | S | - | - | - | - |
| - | - | - | - | - | - | - | 51 | 64 | VDD | S | - | - | - | - |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| LIFOFPN32 | LQFP32 | LQFPPN48 | LQFP48 | WLCSP48 | LQFP64 | UFBGA64 | LQFP80 | LQFP100 |  |  |  |  |  |
| - | - | 29 | - | D1 | 38 | E8 | 52 | 65 | PC6 | I/O | FT | - | TIM3_CH1, TIM8_CH1, I2S2_MCK, EVENTOUT | - |
| - | - | - | - | - | 39 | E7 | 53 | 66 | PC7 | I/O | FT | - | TIM3_CH2, TIM8_CH2, I2S3_MCK, EVENTOUT | - |
| - | - | - | - | - | 40 | F6 | 54 | 67 | PC8 | I/O | FT_f | - | TIM3_CH3, TIM8_CH3, I2C3_SCL, EVENTOUT | - |
| - | - | - | - | - | 41 | D8 | 55 | 68 | PC9 | I/O | FT_f | - | TIM3_CH4, TIM8_CH4, I2SCKIN, TIM8_BKIN2, I2C3_SDA, EVENTOUT | - |
| 18 | 18 | 30 | 30 | D2 | 42 | E6 | 56 | 69 | PA8 | I/O | FT_f | - | MCO, I2C3_SCL, <br> I2C2_SDA, I2S2_MCK, <br> TIM1_CH1, <br> USART1_CK, <br> TIM4_ETR, SAI1_CK2, <br> SAI1_SCK_A, <br> EVENTOUT | - |
| 19 | 19 | 31 | 31 | C3 | 43 | D7 | 57 | 70 | PA9 | I/O | FT_fd | - | I2C3_SMBA, I2C2_SCL, I2S3_MCK, TIM1_CH2, USART1_TX, TIM15_BKIN, TIM2_CH3, SAI1_FS_A, EVENTOUT | UCPD1_DBCC1 |
| 20 | 20 | 32 | 32 | C2 | 44 | D6 | 58 | 71 | PA10 | I/O | $\begin{aligned} & \text { FT_d } \\ & \text { a } \end{aligned}$ | - | TIM17_BKIN, USB_CRS_SYNC, I2C2_SMBA, SPI2_MISO, TIM1_CH3, USART1_RX, TIM2_CH4, TIM8_BKIN, SAI1_D1, SAI1_SD_A, EVENTOUT | UCPD1_DBCC2 |
| 21 | 21 | 33 | 33 | C1 | 45 | C8 | 59 | 72 | PA11 | I/O | FT_u | - | SPI2_MOSI/ <br> I2S2_SD, <br> TIM1_CH1N, <br> USART1_CTS, <br> COMP1_OUT, <br> FDCAN1_RX, <br> TIM4_CH1, <br> TIM1_CH4, <br> TIM1_BKIN2, <br> EVENTOUT | USB_DM |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| 22 | 22 | 34 | 34 | B1 | 46 | B8 | 60 | 73 | PA12 | I/O | FT_u | - | TIM16_CH1, I2SCKIN, TIM1_CH2N, USART1_RTS_DE, COMP2_OUT, FDCAN1 TX, TIM4_CH2, TIM1_ETR, EVENTOUT | USB_DP |
| - | - | - | 35 | - | 47 | B7 | 61 | 74 | VSS | S | - | - | - | - |
| - | - | 35 | 36 | - | 48 | A8 | 62 | 75 | VDD | S | - | - | - | - |
| 23 | 23 | 36 | 37 | B2 | 49 | C7 | 63 | 76 | PA13 | I/O | FT_f | (5) | SWDIO-JTMS, TIM16_CH1N, I2C1_SCL, IR_OUT, USART3_CTS, TIM4_CH3, SAI1_SD_B, EVENTOUT | - |
| 24 | 24 | 37 | 38 | B3 | 50 | C6 | 64 | 77 | PA14 | I/O | FT_f | (5) | SWCLK-JTCK, LPTIM1_OUT, I2C1_SDA, TIM8_CH2, TIM1_BKIN, USART2_TX, SAI1_FS_B, EVENTOUT | - |
| 25 | 25 | 38 | 39 | A1 | 51 | A7 | 65 | 78 | PA15 | I/O | FT_f | (5) | JTDI, <br> TIM2_CH1, TIM8_CH1, I2C1_SCL, SPI1_NSS, SPI3_NSS/ <br> I2S3_WS, USART2_RX, UART4_RTS_DE, TIM1_BKIN, TIM2_ETR, EVENTOUT | - |
| - | - | 39 | - | - | 52 | C5 | 66 | 79 | PC10 | I/O | FT | - | TIM8_CH1N, UART4_TX, SPI3_SCK/ I2S3_CK, USART3_TX, EVENTOUT | - |
| - | - | 40 | - | A2 | 53 | B6 | 67 | 80 | PC11 | I/O | FT_f | - | TIM8_CH2N, UART4_RX, SPI3_MISO, USART3_RX, I2C3_SDA, EVENTOUT | - |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| LGFP32 | LQFP32 | LQFPN48 | LQFP48 | WLCSP49 | LQFP64 | UFBGA64 | LQFP80 |  |  |  |  |  |  |
| - | - | - | - | - | 54 | A6 | 68 | 81 | PC12 | I/O | FT | - | TIM8_CH3N, SPI3_MOSI/ I2S3_SD, USART3_CK, UCPD1_FRSTX, EVENTOUT | - |
| - | - | - | - | - | - | - | 69 | 82 | PD0 | I/O | FT | - | TIM8_CH4N, <br> FDCAN1_RX, EVENTOUT | - |
| - | - | - | - | - | - | - | 70 | 83 | PD1 | I/O | FT | - | TIM8_CH4, <br> TIM8_BKIN2, <br> FDCAN1_TX, EVENTOUT | - |
| - | - | - | - | - | 55 | B5 | 71 | 84 | PD2 | I/O | FT | - | TIM3_ETR, TIM8_BKIN, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 85 | PD3 | I/O | FT | - | TIM2_CH1/ <br> TIM2_ETR, <br> USART2_CTS, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 86 | PD4 | I/O | FT | - | TIM2_CH2, USART2_RTS_DE, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 87 | PD5 | I/O | FT | - | USART2_TX, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 88 | PD6 | I/O | FT | - | TIM2_CH4, SAI1_D1, USART2_RX, SAI1_SD_A, EVENTOUT | - |
| - | - | - | - | - | - | - | - | 89 | PD7 | I/O | FT | - | TIM2_CH3, USART2_CK, EVENTOUT | - |
| 26 | 26 | 41 | 40 | A3 | 56 | A5 | 72 | 90 | PB3 | I/O | FT | (5) | JTDO-TRACESWO, TIM2_CH2, TIM4_ETR, USB_CRS_SYNC, TIM8_CH1N, SPI1_SCK, SPI3_SCK/ I2S3_CK, USART2_TX, TIM3_ETR, SAI1_SCK_B, EVENTOUT | - |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| 27 | 27 | 42 | 41 | B4 | 57 | C4 | 73 | 91 | PB4 | I/O | $\mathrm{FT} \_c$ | (5) | JTRST, <br> TIM16_CH1, <br> TIM3_CH1, <br> TIM8_CH2N, <br> SPI1_MISO, <br> SPI3_MISO, <br> USART2_RX, <br> TIM17_BKIN, <br> SAI1_MCLK_B, <br> EVENTOUT | UCPD1_CC2 |
| 28 | 28 | 43 | 42 | A43 | 58 | B4 | 74 | 92 | PB5 | I/O | FT_f | - | TIM16_BKIN, TIM3_CH2, TIM8_CH3N, I2C1_SMBA, SPI1_MOSI, SPI3_MOSI/ I2S3_SD, USART2_CK, I2C3_SDA, TIM17_CH1, LPTIM1_IN1, SAI1_SD_B, EVENTOUT | - |
| 29 | 29 | 44 | 43 | C4 | 59 | A4 | 75 | 93 | PB6 | I/O | FT_c | (6) | TIM16_CH1N, TIM4_CH1, TIM8_CH1, TIM8_ETR, USART1_TX, COMP4_OUT, TIM8_BKIN2, LPTIM1_ETR, SAI1_FS_B, EVENTOUT | UCPD1_CC1 |
| 30 | 30 | 45 | 44 | A5 | 60 | A3 | 76 | 94 | PB7 | I/O | FT_f | - | TIM17_CH1N, TIM4_CH2, I2C1_SDA, TIM8_BKIN, USART1_RX, COMP3_OUT, TIM3_CH4, LPTIM1_IN2, UART4_CTS, EVENTOUT | PVD_IN |
| 31 | 31 | 46 | 45 | B5 | 61 | B3 | 77 | 95 | PB8-BOOT0 | I/O | FT_f | (7) | TIM16_CH1, TIM4_CH3, SAI1_CK1, I2C1_SCL, USART3_RX, COMP1_OUT, FDCAN1_RX, TIM8_CH2, TIM1_BKIN, SAI1_MCLK_A, EVENTOUT | - |Table 12. STM32G431x6/x8/xB pin definition ${ }^{(1)}$ (continued)

| Pin Number |  |  |  |  |  |  |  | Pin name (function after reset) | Pin type | I/O structure | Notes | Alternate function | Additional functions |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| LQFP32 | LQFP48 | LQFP48 | WLCSP49 | LQFP64 | UFBGA64 | LQFP80 | LQFP100 |  |  |  |  |  |  |
| - | - | 47 | 46 | B6 | 62 | A2 | 78 | 96 | PB9 | I/O | FT_f | - | TIM17_CH1, <br> TIM4_CH4, <br> SAI1_D2, I2C1_SDA, <br> IR_OUT, USART3_TX, <br> COMP2_OUT, <br> FDCAN1_TX, <br> TIM8_CH3, <br> TIM1_CH3N, <br> SAI1_FS_A, <br> EVENTOUT | - |
| - | - | - | - | - | - | - | - | 97 | PE0 | I/O | FT | - | TIM4_ETR, <br> TIM16_CH1, <br> USART1_TX, <br> EVENTOUT | - |
| - | - | - | - | - | - | - | - | 98 | PE1 | I/O | FT | - | TIM17_CH1, <br> USART1_RX, <br> EVENTOUT | - |
| 32 | 32 | - | 47 | A6 | 63 | B2 | 79 | 99 | VSS | S | - | - | - | - |
| 1 | 1 | 48 | 48 | A7 | 64 | A1 | 80 | 100 | VDD | S | - | - | - | - |

1. Function availability depends on the chosen device.
2. PC13, PC14 and PC15 are supplied through the power switch. Since the switch only sinks a limited amount of current (3 mA ), the use of GPIOs PC13 to PC15 in output mode is limited:

- The speed should not exceed 2 MHz with a maximum load of 30 pF
- These GPIOs must not be used as current sources (for instance to drive an LED).

3. After a Backup domain power-up, PC13, PC14 and PC15 operate as GPIOs. Their function then depends on the content of the RTC registers which are not reset by the system reset. For details on how to manage these GPIOs, refer to the Backup domain and RTC register descriptions in the reference manual RM0440 "STM32G4 Series advanced Arm ${ }^{\text {® }}$-based 32-bit $\mathrm{MCUs}^{\circ}$.
4. PG10-NRST pin is FT tolerant if it is configured as PG10 GPIO by option bytes except for the startup time until option bytes are loaded.
5. After reset, these pins are configured as JTAG/SW debug alternate functions, and the internal pull-up on PA15, PA13, PB4 pins and the internal pull-down on PA14 pin are activated.
6. After reset, a pull-down resistor ( $R d=5.1 \mathrm{k} \Omega$ from UCPD peripheral) can be activated on PB6, PB4 (UCPD1_CC1, UCPD1_CC2). The pull-down on PB6 (UCPD1_CC1) is activated by high level on PA9 (UCPD1_DBCC1). The pull-down on PB4 (UCPD1_CC2) is activated by high level on PA10 (UCPD1_DBCC2). This pull-down control (dead battery support on UCPD peripheral) can be disabled by setting bit UCPD1_DBDIS=1 in the PWR_CR3 register. PB4, PB6 have UCPD_CC functionality which implements an internal pull-down resistor ( $5.1 \mathrm{k} \Omega$ ) which is controlled by the voltage on the UCPD_DBCC pin (PA10, PA9). A high level on the UCPD_DBCC pin activates the pull-down on the UCPD_CC pin. The pull-down effect on the CC lines can be removed by using the bit UCPD1_DBDIS $=1$ (USB Type-C and power delivery dead battery disable) in the PWR_CR3 register.
7. It is recommended to set PB8 in another mode than analog mode after startup to limit consumption if the pin is left unconnected.# 4.11 Alternate functions

Table 13. Alternate function

|   |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   | Port | SYS_AF | LPTIM1/7/ M2/5/15/1 6/17 | $\begin{aligned} & \text { QC3/TIM1/ } \ & \text { 2/2/4/6/15/ } \ & \text { GPCOMP1 } \end{aligned}$ | $\begin{aligned} & \text { QC3/SAI1/ } \ & \text { USB/TIM8/ } \ & \text { 15/ } \ & \text { GPCOMP3 } \end{aligned}$ | $\begin{aligned} & \text { QC1/2/3/ } \ & \text { TIM1/6/16/ } \ & 17 \end{aligned}$ | SPH1/2/3/ $\begin{aligned} & \text { I2S2/3/ } \ & \text { UART4/ } \ & \text { /TIM8/infra } \ & \text { red } \end{aligned}$ | $\begin{aligned} & \text { SPQ/3/ } \ & \text { I2S2/3/ } \ & \text { TIM1/8/ } \ & \text { Infrared } \end{aligned}$ | USART1/2/ 3 | $\begin{aligned} & \text { QC3/4/ } \ & \text { /UART4/ } \ & \text { LPUART1/ } \ & \text { GPCOMP1/ } \ & \text { 3/3 } \end{aligned}$ | $\begin{aligned} & \text { TIM1/8/15/ } \ & \text { FDCAN1 } \end{aligned}$ | TIM2/2/4/8/ 17 | LPTIM1/7/ $\begin{aligned} & \text { M1/8/FDCA } \ & \text { N1 } \end{aligned}$ | LPUART1/ SAI1/TIM1 | SAI1/ OPAMP2 | UART4/SAI 1/TIM2/15/ UCPD1 | EVENT  |
|   | PA0 | - | TIM2, $\mathrm{CH} 1$ | - | - | - | - | - | USART2, CTS | COMP1, OUT | TIM8_BKIN | TIM8_ETR | - | - | - | TIM2_ETR | EVENT OUT  |
|   | PA1 | $\begin{aligned} & \text { RTC } \ & \text { REFIN } \end{aligned}$ | TIM2, $\mathrm{CH} 2$ | - | - | - | - | - | USART2, RTS_DE | - | TIM15, CH1N | - | - | - | - | - | EVENT OUT  |
|   | PA2 | - | TIM2, $\mathrm{CH} 3$ | - | - | - | - | - | USART2, TX | COMP2, OUT | TIM15, $\mathrm{CH} 1$ | - | - | LPUART1, TX | - | UCPD1, FRSTX | EVENT OUT  |
|   | PA3 | - | TIM2, $\mathrm{CH} 4$ | - | SAI1_CK1 | - | - | - | USART2, RX | - | TIM15, $\mathrm{CH} 2$ | - | - | LPUART1, RX | SAI1, MCLK_A | - | EVENT OUT  |
|   | PA4 | - | - | TIM3_CH2 | - | - | SP11_NSS | SP13_NSS/ I2S3_WS | USART2, CK | - | - | - | - | - | SAI1_FS_B | - | EVENT OUT  |
|   | PA5 | - | TIM2, $\mathrm{CH} 1$ | TIM2_ETR | - | - | SP11_SCK | - | - | - | - | - | - | - | - | UCPD1, FRSTX | EVENT OUT  |
|   | PA6 | - | TIM16, $\mathrm{CH} 1$ | TIM3_CH1 | - | TIM8_BKIN | SP11_MISO | TIM1_BKIN | - | COMP1, OUT | - | - | - | LPUART1, CTS | - | - | EVENT OUT  |
|   | PA7 | - | TIM17, $\mathrm{CH} 1$ | TIM3_CH2 | - | TIM8, CH1N | SP11_MOSI | TIM1, CH1N | - | COMP2, OUT | - | - | - | - | - | UCPD1, FRSTX | EVENT OUT  |
|   | PA8 | MCO | - | I2C3_SCL | - | I2C2_SDA | I2S2_MCK | TIM1_CH1 | USART1, CK | - | - | TIM4_ETR | - | SAI1_CK2 | - | SAI1, SCK_A | EVENT OUT  |
|   | PA9 | - | - | I2C3, SMBA | - | I2C2_SCL | I2S3_MCK | TIM1_CH2 | USART1, TX | - | TIM15, BKIN | TIM2_CH3 | - | - | - | SAI1_FS_A | EVENT OUT  |
|   | PA10 | - | TIM17, BKIN | - | USB, CRS_SYNC | I2C2, SMBA | SP12_MISO | TIM1_CH3 | USART1, RX | - | - | TIM2_CH4 | TIM8_BKIN | SAI1_D1 | - | SAI1_SD, A | EVENT OUT  |
|   | PA11 | - | - | - | - | - | SP12_MOSI /I2S2_SD | TIM1, CH1N | USART1, CTS | COMP1, OUT | FDCAN1_R X | TIM4_CH1 | TIM1_CH4 | TIM1, BKIN2 | - | - | EVENT OUT  |
|   | PA12 | - | TIM16, $\mathrm{CH} 1$ | - | - | - | I2SCKIN | TIM1, CH2N | USART1, RTS_DE | COMP2, OUT | FDCAN1_T X | TIM4_CH2 | TIM1_ETR | - | - | - | EVENT OUT  |
|   | PA13 | SWDC/ JTMS | TIM16, CH1N | - | - | I2C1_SCL | IR_OUT | - | USART3, CTS | - | - | TIM4_CH3 | - | - | SAI1_SD, B | - | EVENT OUT  |
|   | PA14 | SWCLKJTCK | LPTIM1, OUT | - | - | I2C1_SDA | TIM8_CH2 | TIM1_BKIN | USART2, TX | - | - | - | - | - | SAI1_FS_B | - | EVENT OUT  |
|   | PA15 | JTDI | TIM2, $\mathrm{CH} 1$ | TIM8_CH1 | - | I2C1_SCL | SP11_NSS | SP13_NSS/ I2S3_WS | USART2, RX | UART4, RTS_DE | TIM1_BKIN | - | - | - | - | TIM2_ETR | EVENT OUT  |Table 13. Alternate function (continued)

|  Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  | SYS_AF | LPTIM1/TI M2/5/15/1 6/17 | I2C3/TIM1/ 2/3/4/8/15/ GPCOMP1 | I2C3/SAI1/ 1/200/TIM0 15/ GPCOMP3 | I2C1/2/3/ TIM1/8/16/ 17 | SPI1/2/3/ I2S2/3/ UART4 /TIM8/infra red | SPI2/3/ I2S2/3/ TIM1/8/ infrared | USART1/2/ 3 | I2C3/4 /UART4/ LPUART1/ GPCOMP1/ 2/3 | TIM1/8/15/ FDCAN1 | TIM2/3/4/8/ 17 | LPTIM1/TI M1/8/FDCAN1 | LPUART1/ SAI1/TIM1 | SAI1/ OPAMP2 | UART4/SAI 1/TIM2/15/ UCPD1 | EVENT  |
|   | PB0 | - | - | TIM3_CH3 | - | TIM8_ CH2N | - | TIM1_ CH2N | - | - | - | - | - | - | - | UCPD1_FR STX | EVENT OUT  |
|   | PB1 | - | - | TIM3_CH4 | - | TIM8_ CH3N | - | TIM1_ CH3N | - | COMP4_ OUT | - | - | - | LPUART1_ RTS_DE | - | - | EVENT OUT  |
|   | PB2 | RTC_OUT2 | LPTIM1/ OUT | - | - | I2C3_ SMB4 | - | - | - | - | - | - | - | - | - | - | EVENT OUT  |
|   | PB3 | JTOO- TRACESWO | TIM2_ CH2 | TIM4_ETR | USB_CRS_ SYNC | TIM8_ CH1N | SPI1_SCK | SPI3_SCK/ I2S3_CK | USART2_ TX | - | - | TIM3_ETR | - | - | - | SAI1_SCK_B | EVENT OUT  |
|   | PB4 | JTRST | TIM10_ CH1 | TIM3_CH1 | - | TIM8_ CH2N | SPI1_MISO | SPI3_MISO | USART2_ RX | - | - | TIM17_ BKIN | - | - | - | SAI1_ MCLK_B | EVENT OUT  |
|   | PB5 | - | TIM16_ BKIN | TIM3_CH2 | TIM8_ CH3N | I2C1_ SMB4 | SPI1_MOSI | SPI3_MOS/ /I2S3_SD | USART2_ CK | I2C3_SDA | - | TIM17_ CH1 | LPTIM1_ IN1 | SAI1_SD_ B | - | - | EVENT OUT  |
|   | PB6 | - | TIM16_ CH1N | TIM4_CH1 | - | - | TIM8_CH1 | TIM8_ETR | USART1_ TX | COMP4_ OUT | - | TIM8_ BKIN2 | LPTIM1_ ETR | - | - | SAI1_FS_B | EVENT OUT  |
|   | PB7 | - | TIM17_ CH1N | TIM4_CH2 | - | I2C1_SDA | TIM8_BKIN | - | USART1_ RX | COMP3_ OUT | - | TIM3_CH4 | LPTIM1_ IN2 | - | - | UART4_ CTS | EVENT OUT  |
|   | PB8 | - | TIM16_ CH1 | TIM4_CH3 | SAI1_CK1 | I2C1_SCL | - | - | USART3_ RX | COMP1_ OUT | FDCAN1_R X | TIM8_CH2 | - | TIM1_BKIN | - | SAI1_ MCLK_A | EVENT OUT  |
|   | PB9 | - | TIM17_ CH1 | TIM4_CH4 | SAI1_D2 | I2C1_SDA | - | IR_OUT | USART3_ TX | COMP2_ OUT | FDCAN1_T X | TIM8_CH3 | - | TIM1_ CH3N | - | SAI1_FS_A | EVENT OUT  |
|   | PB10 | - | TIM2_ CH2 | - | - | - | - | - | USART3_ TX | LPUART1_ RX | - | - | - | TIM1_BKIN | - | SAI1_ SCK_A | EVENT OUT  |
|   | PB11 | - | TIM2_ CH4 | - | - | - | - | - | USART3_ RX | LPUART1_ TX | - | - | - | - | - | - | EVENT OUT  |
|   | PB12 | - | - | - | - | I2C2_ SMB4 | SPI2_NSS/ I2S2_WS | TIM1_BKIN | USART3_ CK | LPUART1_ RTS_DE | - | - | - | - | - | - | EVENT OUT  |
|   | PB13 | - | - | - | - | - | SPI2_SCK/ I2S2_CK | TIM1_ CH1N | USART3_ CTS | LPUART1_ CTS | - | - | - | - | - | - | EVENT OUT  |
|   | PB14 | - | TIM15_ CH1 | - | - | - | SPI2_MISO | TIM1_ CH2N | USART3_ RTS_DE | COMP4_ OUT | - | - | - | - | - | - | EVENT OUT  |
|   | PB15 | RTC_REFIN | TIM15_ CH2 | TIM15_ CH1N | COMP3_ OUT | TIM1_ CH3N | SPI2_MOS/ /I2S2_SD | - | - | - | - | - | - | - | - | - | EVENT OUT  |Table 13. Alternate function (continued)

|  Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  | SYS_AF | LPTIM1/TI
M2/S/15/1
6/17 | I2C3/TIM1/
2/3/4/8/15/
GPCOMP1 | I2C3/SAI1/
I/2I5/TIMI/
15/
GPCOMP3 | I2C1/2/3/
TIM1/8/16/
17 | SPI1/2/3/
I2S2/3/
UART4
/TIM8/infra red | SPQ/3/
I2S2/3/
TIM1/8/
Infrared | USART1/2/
3 | I2C3/4
/UART4/
LPUART1/
GPCOMP1/
2/3 | TIM1/8/15/
FDCAN1 | TIM2/3/4/8/
17 | LPTIM1/TI
M1/8/FDCA
N1 | LPUART1/
SAI1/TIM1 | SAI1/
OPAMP2 | UART4/SAI
1/TIM2/15/
UCPD1 | EVENT  |
|   | PC0 | - | LPTIM1_
IN1 | TIM1_CH1 | - | - | - | - | - | LPUART1_
RX | - | - | - | - | - | - | EVENT
OUT  |
|   | PC1 | - | LPTIM1_
OUT | TIM1_CH2 | - | - | - | - | - | LPUART1_
TX | - | - | - | - | SAI1_SD_ | - | EVENT
OUT  |
|   | PC2 | - | LPTIM1_
IN2 | TIM1_CH3 | COMP3_
OUT | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|   | PC3 | - | LPTIM1_
ETR | TIM1_CH4 | SAI1_D1 | - | - | TIM1_
BKIN2 | - | - | - | - | - | - | SAI1_SD_ | - | EVENT
OUT  |
|   | PC4 | - | - | TIM1_ETR | - | I2C2_SCL | - | - | USART1_
TX | - | - | - | - | - | - | - | EVENT
OUT  |
|   | PC5 | - | - | TIM15_
BKIN | SAI1_D3 | - | - | TIM1_
CH4N | USART1_
RX | - | - | - | - | - | - | - | EVENT
OUT  |
|   | PC6 | - | - | TIM3_CH1 | - | TIM8_
CH1 | - | I2S2_MCK | - | - | - | - | - | - | - | - | EVENT
OUT  |
|   | PC7 | - | - | TIM3_CH2 | - | TIM8_
CH2 | - | I2S3_MCK | - | - | - | - | - | - | - | - | EVENT
OUT  |
|   | PC8 | - | - | TIM3_CH3 | - | TIM8_
CH3 | - | - | - | I2C3_SCL | - | - | - | - | - | - | EVENT
OUT  |
|   | PC9 | - | - | TIM3_CH4 | - | TIM8_
CH4 | I2SCKIN | TIM8_
BKIN2 | - | I2C3_SDA | - | - | - | - | - | - | EVENT
OUT  |
|   | PC10 | - | - | - | - | TIM8_
CH1N | UART4_TX | SPI3_SCK/
I2S3_CK | USART3_
TX | - | - | - | - | - | - | - | EVENT
OUT  |
|   | PC11 | - | - | - | - | TIM8_
CH2N | UART4_RX | SPI3_MISO | USART3_
RX | I2C3_SDA | - | - | - | - | - | - | EVENT
OUT  |
|   | PC12 | - | - | - | - | TIM8_
CH3N | - | SPI3_MOS/
/I2S3_SD | USART3_
CK | - | - | - | - | - | - | UCPD1_
FRSTX | EVENT
OUT  |
|   | PC13 | - | - | TIM1_BKIN | - | TIM1_
CH1N | - | TIM8_
CH4N | - | - | - | - | - | - | - | - | EVENT
OUT  |
|   | PC14 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|   | PC15 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |Table 13. Alternate function (continued)

|  Port | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   | SYS_AF | LPTIM1/TI
M2/S/15/1
6/17 | QC3/TIM1/
2/3/4/8/15/
GPCOMP1 | QC3/SAI1/
OSB/TIM0/
15/
GPCOMP3 | QC1/2/3/
TIM1/8/16/
17 | SPI1/2/3/
QS2/3/
UART4
/TIM8/infra
red | SPQ/3/
QS2/3/
TIM1/8/
Infrared | USART1/2/
3 | QC3/4
/UART4/
LPUART1/
GPCOMP1/
2/3 | TIM1/8/15/
FDCAN1 | TIM2/3/4/8/
17 | LPTIM1/TI
M1/8/FDCA
N1 | LPUART1/
SAI1/TIM1 | SAI1/
OPAMP2 | UART4/SAI
1/TIM2/15/
UCPD1 | EVENT  |
|  PD0 | - | - | - | - | - | - | TIM8_
CH4N | - | - | FDCAN1_R
X | - | - | - | - | - | EVENT
OUT  |
|  PD1 | - | - | - | - | TIM8_CH4 | - | TIM8_
BKIN2 | - | - | FDCAN1_T
X | - | - | - | - | - | EVENT
OUT  |
|  PD2 | - | - | TIM3_ETR | - | TIM8_BKIN | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD3 | - | - | TIM2_CH1/
TIM2_ETR | - | - | - | - | USART2_
CTS | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD4 | - | - | TIM2_CH2 | - | - | - | - | USART2_
RTS_DE | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD5 | - | - | - | - | - | - | - | USART2_
TX | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD6 | - | - | TIM2_CH4 | SAI1_D1 | - | - | - | USART2_
RX | - | - | - | - | - | SAI1_SD_
A | - | EVENT
OUT  |
|  PD7 | - | - | TIM2_CH3 | - | - | - | - | USART2_
CK | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD8 | - | - | - | - | - | - | - | USART3_
TX | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD9 | - | - | - | - | - | - | - | USART3_
RX | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD10 | - | - | - | - | - | - | - | USART3_
CK | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD11 | - | - | - | - | - | - | - | USART3_
CTS | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD12 | - | - | TIM4_CH1 | - | - | - | - | USART3_
RTS_DE | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD13 | - | - | TIM4_CH2 | - | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD14 | - | - | TIM4_CH3 | - | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PD15 | - | - | TIM4_CH4 | - | - | - | SPI2_NSS | - | - | - | - | - | - | - | - | EVENT
OUT  |Table 13. Alternate function (continued)

|  Port | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   | SYS_AF | LPTIM1/TI
M2/S/15/1
6/17 | QC3/TIM1/
2/3/4/8/15/
GPCOMP1 | QC3/SAI1/
OSB/TIM0
15/
GPCOMP3 | QC1/2/3/
TIM1/8/16/
17 | SPI1/2/3/
QS2/3/
UART4
/TIM8/infra
red | SPQ/3/
QS2/3/
TIM1/8/
Infrared | USART1/2/
3 | QC3/4
/UART4/
LPUART1/
GPCOMP1/
2/3 | TIM1/8/15/
FDCAN1 | TIM2/3/4/8/
17 | LPTIM1/TI
M1/8/FDCA
N1 | LPUART1/
SAI1/TIM1 | SAI1/
OPAMP2 | UART4/SAI
1/TIM2/15/
UCPD1 | EVENT  |
|  PE0 | - | - | TIM4_ETR | - | TIM16_
CH1 | - | - | USART1_
T4 | - | - | - | - | - | - | - | EVENT
OUT  |
|  PE1 | - | - | - | - | TIM17_
CH1 | - | - | USART1_
RX | - | - | - | - | - | - | - | EVENT
OUT  |
|  PE2 | TRACECK | - | TIM3_
CH1 | SAI1_CK1 | - | - | - | - | - | - | - | - | - | SAI1_
MCLK_A | - | EVENT
OUT  |
|  PE3 | TRACED0 | - | TIM3_
CH2 | - | - | - | - | - | - | - | - | - | - | SAI1_
SD_B | - | EVENT
OUT  |
|  PE4 | TRACED1 | - | TIM3_
CH3 | SAI1_D2 | - | - | - | - | - | - | - | - | - | SAI1_
FS_A | - | EVENT
OUT  |
|  PE5 | TRACED2 | - | TIM3_
CH4 | SAI1_CK2 | - | - | - | - | - | - | - | - | - | SAI1_
SCK_A | - | EVENT
OUT  |
|  PE6 | TRACED3 | - | - | SAI1_D1 | - | - | - | - | - | - | - | - | - | SAI1_
SD_A | - | EVENT
OUT  |
|  PE7 | - | - | TIM1_
ETR | - | - | - | - | - | - | - | - | - | - | SAI1_
SD_B | - | EVENT
OUT  |
|  PE8 | - | - | TIM1_
CH1N | - | - | - | - | - | - | - | - | - | - | SAI1_
SCK_B | - | EVENT
OUT  |
|  PE9 | - | - | TIM1_
CH1 | - | - | - | - | - | - | - | - | - | - | SAI1_
FS_B | - | EVENT
OUT  |
|  PE10 | - | - | TIM1_
CH2N | - | - | - | - | - | - | - | - | - | - | SAI1_
MCLK_B | - | EVENT
OUT  |
|  PE11 | - | - | TIM1_
CH2 | - | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PE12 | - | - | TIM1_
CH3N | - | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PE13 | - | - | TIM1_
CH5 | - | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PE14 | - | - | TIM1_
CH4 | - | - | - | TIM1_
BKIN2 | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PE15 | - | - | TIM1_
BKIN | - | - | - | TIM1_
CH4N | USART3_
RX | - | - | - | - | - | - | - | EVENT
OUT  |Table 13. Alternate function (continued)

|  Port | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   | SYS_AF | LPTIM1/TI
M2/S/15/1
6/17 | QC3/TIM1/
2/3/4/8/15/
GPCOMP1 | QC3/SAI1/
QS8/TIM6/
15/
GPCOMP3 | QC1/2/3/
TIM1/8/16/
17 | SPI1/2/3/
QS2/3/
UART4
/TIM8/infra
red | SPQ/3/
QS2/3/
TIM1/8/
Infrared | USART1/2/
3 | QC3/4
/UART4/
LPUART1/
GPCOMP1/
2/3 | TIM1/8/15/
FDCAN1 | TIM2/3/4/8/
17 | LPTIM1/TI
M1/8/FDCAN
17 | LPUART1/
SAI1/TIM1 | SAI1/
OPAMP2 | UART4/SAI
1/TIM2/15/
UCPD1 | EVENT  |
|  PF0 | - | - | - | - | QC2
SDA | SPQ NSS/
QS2 WS | TIM1
CH3N | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PF1 | - | - | - | - | - | SPQ SCK/
QS2 CK | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PF2 | - | - | - | - | QC2
SMBA | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |
|  PF9 | - | - | - | TIM15_CH1 | - | SPQ SCK | - | - | - | - | - | - | - | SAI1_FS_B | - | EVENT
OUT  |
|  PF10 | - | - | - | TIM15_CH2 | - | SPQ SCK | - | - | - | - | - | - | - | SAI1_D3 | - | EVENT
OUT  |
|  PG10 | MCO | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVENT
OUT  |# 5 Electrical characteristics 

### 5.1 Parameter conditions

Unless otherwise specified, all voltages are referenced to $\mathrm{V}_{\mathrm{SS}}$.

### 5.1.1 Minimum and maximum values

Unless otherwise specified, the minimum and maximum values are guaranteed in the worst conditions of ambient temperature, supply voltage and frequencies by tests in production on $100 \%$ of the devices with an ambient temperature at $\mathrm{T}_{\mathrm{A}}=25^{\circ} \mathrm{C}$ and $\mathrm{T}_{\mathrm{A}}=\mathrm{T}_{\mathrm{A}} \max$ (given by the selected temperature range).

Data based on characterization results, design simulation and/or technology characteristics are indicated in the table footnotes and are not tested in production. Based on characterization, the minimum and maximum values refer to sample tests and represent the mean value plus or minus three times the standard deviation (mean $\pm 3 \sigma$ ).

### 5.1.2 Typical values

Unless otherwise specified, typical data are based on $\mathrm{T}_{\mathrm{A}}=25^{\circ} \mathrm{C}, \mathrm{V}_{\mathrm{DD}}=\mathrm{V}_{\mathrm{DDA}}=3 \mathrm{~V}$. They are given only as design guidelines and are not tested.

Typical ADC accuracy values are determined by characterization of a batch of samples from a standard diffusion lot over the full temperature range, where $95 \%$ of the devices have an error less than or equal to the value indicated (mean $\pm 2 \sigma$ ).

### 5.1.3 Typical curves

Unless otherwise specified, all typical curves are given only as design guidelines and are not tested.

### 5.1.4 Loading capacitor

The loading conditions used for pin parameter measurement are shown in Figure 14.

### 5.1.5 Pin input voltage

The input voltage measurement on a pin of the device is described in Figure 15.
![img-15.jpeg](img-15.jpeg)# 5.1.6 Power supply scheme 

Figure 16. Power supply scheme
![img-16.jpeg](img-16.jpeg)

Caution: Each power supply pair $\left(\mathrm{V}_{\mathrm{DD}} / \mathrm{V}_{\mathrm{SS}}, \mathrm{V}_{\mathrm{DDA}} / \mathrm{V}_{\mathrm{SSA}}\right.$ etc.) must be decoupled with filtering ceramic capacitors as shown above. These capacitors must be placed as close as possible to, or below, the appropriate pins on the underside of the PCB to ensure the good functionality of the device.# 5.1.7 Current consumption measurement 

Figure 17. Current consumption measurement
![img-17.jpeg](img-17.jpeg)

The $\mathrm{I}_{\mathrm{DD} \_ \text {ALL }}$ parameters given in Table 21 to Table 24 represent the total MCU consumption including the current supplying $\mathrm{V}_{\mathrm{DD}}, \mathrm{V}_{\mathrm{DDA}}$ and $\mathrm{V}_{\mathrm{BAT}}$.

### 5.2 Absolute maximum ratings

Stresses above the absolute maximum ratings listed in Table 14: Voltage characteristics, Table 15: Current characteristics and Table 16: Thermal characteristics may cause permanent damage to the device. These are stress ratings only and functional operation of the device at these conditions is not implied. Exposure to maximum rating conditions for extended periods may affect device reliability. Exposure to maximum rating conditions for extended periods may affect device reliability. Device mission profile (application conditions) is compliant with JEDEC JESD47 qualification standard, extended mission profiles are available on demand.

Table 14. Voltage characteristics ${ }^{(1)}$

| Symbol | Ratings | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{DD}}-\mathrm{V}_{\mathrm{SS}}$ | External main supply voltage (including $\mathrm{V}_{\mathrm{DD}}$, $\mathrm{V}_{\text {DDA }}, \mathrm{V}_{\text {BAT }}$ and $\mathrm{V}_{\text {REF }+}$ ) | $-0.3$ | 4.0 | V |
| $\mathrm{V}_{\mathrm{IN}}{ }^{(2)}$ | Input voltage on FT_xxx pins except FT_c pins | $\mathrm{V}_{\mathrm{SS}}-0.3$ | $\begin{gathered} \min \left(\mathrm{V}_{\mathrm{DD}}, \mathrm{V}_{\mathrm{DDA}}\right) \\ +4.0^{(3)(4)} \end{gathered}$ |  |
|  | Input voltage on FT_c pins | $\mathrm{V}_{\mathrm{SS}}-0.3$ | 5.5 |  |
|  | Input voltage on TT_xx pins | $\mathrm{V}_{\mathrm{SS}}-0.3$ | 4.0 |  |
|  | Input voltage on any other pins | $\mathrm{V}_{\mathrm{SS}}-0.3$ | 4.0 |  |
| $\left\|\Delta \mathrm{V}_{\mathrm{DDx}}\right\|$ | Variations between different $\mathrm{V}_{\mathrm{DDX}}$ power pins of the same domain | - | 50 | mV |
| $\left\|\mathrm{V}_{\mathrm{SSx}}-\mathrm{V}_{\mathrm{SS}}\right\|$ | Variations between all the different ground pins ${ }^{(5)}$ | - | 50 |  |
| $\mathrm{V}_{\text {REF+ }}-\mathrm{V}_{\text {DDA }}$ | Allowed voltage difference for $\mathrm{V}_{\text {REF+ }}>\mathrm{V}_{\text {DDA }}$ | - | 0.4 | V |

1. All main power $\left(\mathrm{V}_{\mathrm{DD}}, \mathrm{V}_{\mathrm{DDA}}, \mathrm{V}_{\mathrm{BAT}}\right)$ and ground $\left(\mathrm{V}_{\mathrm{SS}}, \mathrm{V}_{\mathrm{SSA}}\right)$ pins must always be connected to the external power supply, in the permitted range.2. $\mathrm{V}_{\mathrm{IN}}$ maximum must always be respected. Refer to Table 15: Current characteristics for the maximum allowed injected current values.
3. This formula has to be applied only on the power supplies related to the IO structure described in the pin definition table.
4. To sustain a voltage higher than 4 V the internal pull-up/pull-down resistors must be disabled.
5. Include VREF- pin.

Table 15. Current characteristics

| Symbol | Ratings | Max | Unit |
| :--: | :--: | :--: | :--: |
| $\sum \mathrm{IV}_{\mathrm{DD}}$ | Total current into sum of all $\mathrm{V}_{\mathrm{DD}}$ power lines (source) ${ }^{(1)}$ | 150 | mA |
| $\sum \mathrm{IV}_{\mathrm{SS}}$ | Total current out of sum of all $\mathrm{V}_{\mathrm{SS}}$ ground lines (sink) ${ }^{(1)}$ | 150 |  |
| $\mathrm{IV}_{\mathrm{DD}(\mathrm{P}(\mathrm{N})}$ | Maximum current into each $\mathrm{V}_{\mathrm{DD}}$ power pin (source) ${ }^{(1)}$ | 100 |  |
| $\mathrm{IV}_{\mathrm{SS}(\mathrm{P}(\mathrm{N})}$ | Maximum current out of each $\mathrm{V}_{\mathrm{SS}}$ ground pin (sink) ${ }^{(1)}$ | 100 |  |
| $\mathrm{I}_{\mathrm{IO}(\mathrm{P}(\mathrm{N})}$ | Output current sunk by any I/O and control pin except FT_f | 20 |  |
|  | Output current sunk by any FT_f pin | 20 |  |
|  | Output current sourced by any I/O and control pin | 20 |  |
| $\sum \mathrm{I}_{\mathrm{IO}(\mathrm{P}(\mathrm{N})}$ | Total output current sunk by sum of all I/Os and control pins ${ }^{(2)}$ | 100 |  |
|  | Total output current sourced by sum of all I/Os and control pins ${ }^{(2)}$ | 100 |  |
| $\mathrm{I}_{\text {INJ(PIN) }}{ }^{(3)}$ | Injected current on FT_xxx, TT_xx, NRST pins | $-5 / 0^{(4)}$ |  |
| $\sum\left|\mathrm{I}_{\mathrm{INJ}(\mathrm{PIN})}\right|$ | Total injected current (sum of all I/Os and control pins) ${ }^{(5)}$ | $\pm 25$ |  |

1. All main power $\left(\mathrm{V}_{\mathrm{DD}}, \mathrm{V}_{\mathrm{DDA}}, \mathrm{V}_{\mathrm{BAT}}\right)$ and ground $\left(\mathrm{V}_{\mathrm{SS}}, \mathrm{V}_{\mathrm{SSA}}\right)$ pins must always be connected to the external power supplies, in the permitted range.
2. This current consumption must be correctly distributed over all I/Os and control pins. The total output current must not be sunk/sourced between two consecutive power supply pins referring to high pin count LQFP packages.
3. Positive injection (when $\left.\mathrm{V}_{\mathrm{IN}}>\mathrm{V}_{\mathrm{DD}}\right)$ is not possible on these I/Os and does not occur for input voltages lower than the specified maximum value.
4. A negative injection is induced by VIN < VSS. IINJ(PIN) must never be exceeded. Refer also to Table 14: Voltage characteristics for the minimum allowed input voltage values.
5. When several inputs are submitted to a current injection, the maximum $\sum\left|\mathrm{I}_{\mathrm{INJ}(\mathrm{PIN})}\right|$ is the absolute sum of the negative injected currents (instantaneous values).

Table 16. Thermal characteristics

| Symbol | Ratings | Value | Unit |
| :--: | :-- | :--: | :--: |
| $\mathrm{T}_{\text {STG }}$ | Storage temperature range | -65 to +150 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\mathrm{J}}$ | Maximum junction temperature | 150 | ${ }^{\circ} \mathrm{C}$ |# 5.3 Operating conditions 

### 5.3.1 General operating conditions

Table 17. General operating conditions

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {HCLK }}$ | Internal AHB clock frequency | - | 0 | 170 | MHz |
| $f_{\text {PCLK1 }}$ | Internal APB1 clock frequency | - | 0 | 170 |  |
| $f_{\text {PCLK2 }}$ | Internal APB2 clock frequency | - | 0 | 170 |  |
| $V_{D D}$ | Standard operating voltage | - | $1.71^{(1)}$ | 3.6 | V |
| $V_{\text {DDA }}$ | Analog supply voltage | ADC or COMP used | 1.62 | 3.6 | V |
|  |  | DAC 1 MSPS or DAC 15 MSPS | 1.71 |  |  |
|  |  | OPAMP used | 2.0 | 3.6 |  |
|  |  | VREFBUF used | 2.4 | 3.6 |  |
|  |  | ADC, DAC, OPAMP, COMP, VREFBUF not used | 0 |  |  |
| $V_{\text {BAT }}$ | Backup operating voltage | - | 1.55 | 3.6 | V |
| $V_{\text {IN }}$ | I/O input voltage | TT_xx I/O | $-0.3$ | $\mathrm{V}_{\mathrm{DD}}+0.3$ | V |
|  |  | FT_c I/O | $-0.3$ | 5 |  |
|  |  | All I/O except TT_xx and FT_c | $-0.3$ | $\begin{gathered} \text { MIN }\left(\text { MIN }\left(\mathrm{V}_{\mathrm{DD}}\right.\right. \\ \left.\left.\mathrm{V}_{\mathrm{DDA}}\right)+3.6 \mathrm{~V}\right) \\ 5.5 \mathrm{~V}\right)^{(2)(3)} \end{gathered}$ |  |
| $P_{D}$ | Power dissipation | See Section 6.10: Thermal characteristics for application appropriate thermal resistance and package. <br> Power dissipation is then calculated according ambient temperature $\left(T_{A}\right)$ and maximum junction temperature $\left(T_{J}\right)$ and selected thermal resistance. |  |  | mW |
| $T_{A}$ | Ambient temperature for the suffix 6 version | Maximum power dissipation | $-40$ | 85 | ${ }^{\circ} \mathrm{C}$ |
|  |  | Low-power dissipation ${ }^{(4)}$ | $-40$ | 105 |  |
|  | Ambient temperature for the suffix 3 version | Maximum power dissipation | $-40$ | 125 |  |
|  |  | Low-power dissipation ${ }^{(4)}$ | $-40$ | 130 |  |
| $T_{J}$ | Junction temperature range | Suffix 6 version | $-40$ | 105 | ${ }^{\circ} \mathrm{C}$ |
|  |  | Suffix 3 version | $-40$ | 130 |  |

1. When RESET is released functionality is guaranteed down to $\mathrm{V}_{\text {BOR0 }}$ Min.
2. This formula has to be applied only on the power supplies related to the IO structure described by the pin definition table. Maximum I/O input voltage is the smallest value between $\operatorname{MIN}\left(\mathrm{V}_{\mathrm{DD}}, \mathrm{V}_{\mathrm{DDA}}\right)+3.6 \mathrm{~V}$ and 5.5 V .
3. For operation with voltage higher than $\operatorname{Min}\left(V_{D D}, V_{D D A}\right)+0.3 \mathrm{~V}$, the internal Pull-up and Pull-Down resistors must be disabled.
4. In low-power dissipation state, $\mathrm{T}_{\mathrm{A}}$ can be extended to this range as long as $\mathrm{T}_{\mathrm{J}}$ does not exceed $\mathrm{T}_{\mathrm{J} \text { max }}$ (see Section 6.10: Thermal characteristics).# 5.3.2 Operating conditions at power-up / power-down 

The parameters given in Table 18 are derived from tests performed under the ambient temperature condition summarized in Table 17.

Table 18. Operating conditions at power-up / power-down

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{t}_{\text {VDD }}$ | $\mathrm{V}_{\mathrm{DD}}$ rise time rate | - | 0 | $\infty$ | $\mu \mathrm{s} / \mathrm{V}$ |
|  | $\mathrm{V}_{\mathrm{DD}}$ fall time rate |  | 10 | $\infty$ |  |
| $\mathrm{t}_{\text {VDDA }}$ | $\mathrm{V}_{\mathrm{DDA}}$ rise time rate | - | 0 | $\infty$ | $\mu \mathrm{s} / \mathrm{V}$ |
|  | $\mathrm{V}_{\mathrm{DDA}}$ fall time rate |  | 10 | $\infty$ |  |

### 5.3.3 Embedded reset and power control block characteristics

The parameters given in Table 19 are derived from tests performed under the ambient temperature conditions summarized in Table 17: General operating conditions.

Table 19. Embedded reset and power control block characteristics

| Symbol | Parameter | Conditions ${ }^{(1)}$ | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{t}_{\text {RSTTEMPO }}{ }^{(2)}$ | Reset temporization after BOR0 is detected | $\mathrm{V}_{\mathrm{DD}}$ rising | - | 250 | 400 | $\mu \mathrm{s}$ |
| $\mathrm{V}_{\text {BOR0 }}{ }^{(2)}$ | Brown-out reset threshold 0 | Rising edge | 1.62 | 1.66 | 1.7 | V |
|  |  | Falling edge | 1.6 | 1.64 | 1.69 |  |
| $\mathrm{V}_{\text {BOR1 }}$ | Brown-out reset threshold 1 | Rising edge | 2.06 | 2.1 | 2.14 | V |
|  |  | Falling edge | 1.96 | 2 | 2.04 |  |
| $\mathrm{V}_{\text {BOR2 }}$ | Brown-out reset threshold 2 | Rising edge | 2.26 | 2.31 | 2.35 | V |
|  |  | Falling edge | 2.16 | 2.20 | 2.24 |  |
| $\mathrm{V}_{\text {BOR3 }}$ | Brown-out reset threshold 3 | Rising edge | 2.56 | 2.61 | 2.66 | V |
|  |  | Falling edge | 2.47 | 2.52 | 2.57 |  |
| $\mathrm{V}_{\text {BOR4 }}$ | Brown-out reset threshold 4 | Rising edge | 2.85 | 2.90 | 2.95 | V |
|  |  | Falling edge | 2.76 | 2.81 | 2.86 |  |
| $\mathrm{V}_{\text {PVD0 }}$ | Programmable voltage detector threshold 0 | Rising edge | 2.1 | 2.15 | 2.19 | V |
|  |  | Falling edge | 2 | 2.05 | 2.1 |  |
| $\mathrm{V}_{\text {PVD1 }}$ | PVD threshold 1 | Rising edge | 2.26 | 2.31 | 2.36 | V |
|  |  | Falling edge | 2.15 | 2.20 | 2.25 |  |
| $\mathrm{V}_{\text {PVD2 }}$ | PVD threshold 2 | Rising edge | 2.41 | 2.46 | 2.51 | V |
|  |  | Falling edge | 2.31 | 2.36 | 2.41 |  |
| $\mathrm{V}_{\text {PVD3 }}$ | PVD threshold 3 | Rising edge | 2.56 | 2.61 | 2.66 | V |
|  |  | Falling edge | 2.47 | 2.52 | 2.57 |  |Table 19. Embedded reset and power control block characteristics (continued)

| Symbol | Parameter | Conditions ${ }^{(1)}$ | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{\text {PVD4 }}$ | PVD threshold 4 | Rising edge | 2.69 | 2.74 | 2.79 | V |
|  |  | Falling edge | 2.59 | 2.64 | 2.69 |  |
| $V_{\text {PVD5 }}$ | PVD threshold 5 | Rising edge | 2.85 | 2.91 | 2.96 | V |
|  |  | Falling edge | 2.75 | 2.81 | 2.86 |  |
| $V_{\text {PVD6 }}$ | PVD threshold 6 | Rising edge | 2.92 | 2.98 | 3.04 | V |
|  |  | Falling edge | 2.84 | 2.90 | 2.96 |  |
| $V_{\text {hyst_BORH0 }}$ | Hysteresis voltage of BORHO | Hysteresis in continuous mode | - | 20 | - | mV |
|  |  | Hysteresis in other mode | - | 30 | - |  |
| $V_{\text {hyst_BOR_PVD }}$ | Hysteresis voltage of BORH (except BORHO) and PVD | - | - | 100 | - | mV |
| $\begin{gathered} \mathrm{I}_{\mathrm{DD}} \\ \text { (BOR_PVD) }^{(2)} \end{gathered}$ | $\mathrm{BOR}^{(3)}$ (except BOR0) and PVD consumption from $\mathrm{V}_{\mathrm{DD}}$ | - | - | 1.1 | 1.6 | $\mu \mathrm{A}$ |
| $V_{\text {PVM1 }}$ | $\mathrm{V}_{\text {DDA }}$ peripheral voltage monitoring (COMP/ADC) | Rising edge | 1.61 | 1.65 | 1.69 | V |
|  |  | Falling edge | 1.6 | 1.64 | 1.68 |  |
| $V_{\text {PVM2 }}$ | $\mathrm{V}_{\text {DDA }}$ peripheral voltage monitoring (OPAMP/DAC) | Rising edge | 1.78 | 1.82 | 1.86 | V |
|  |  | Falling edge | 1.77 | 1.81 | 1.85 |  |
| $V_{\text {hyst_PVM1 }}$ | PVM1 hysteresis | - | - | 10 | - | mV |
| $V_{\text {hyst_PVM2 }}$ | PVM2 hysteresis | - | - | 10 | - | mV |
| $\begin{gathered} \mathrm{I}_{\mathrm{DD}} \\ \text { (PVM1/PVM2) } \\ \text { (2) } \end{gathered}$ | PVM1 and PVM2 consumption from $\mathrm{V}_{\mathrm{DD}}$ | - | - | 2 | - | $\mu \mathrm{A}$ |

1. Continuous mode means Run/Sleep modes, or temperature sensor enable in Low-power run/Low-power sleep modes.
2. Guaranteed by design.
3. BOR0 is enabled in all modes (except shutdown) and its consumption is therefore included in the supply current characteristics tables.# 5.3.4 Embedded voltage reference 

The parameters given in Table 20 are derived from tests performed under the ambient temperature and supply voltage conditions summarized in Table 17: General operating conditions.

Table 20. Embedded internal voltage reference

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{\text {REFINT }}$ | Internal reference voltage | $-40^{\circ} \mathrm{C}<\mathrm{T}_{\mathrm{A}}<+130^{\circ} \mathrm{C}$ | 1.182 | 1.212 | 1.232 | V |
| $t_{S \_ \text {vrefint }}{ }^{(1)}$ | ADC sampling time when reading the internal reference voltage | - | $4^{(2)}$ | - | - | $\mu \mathrm{s}$ |
| $t_{\text {start_vrefint }}$ | Start time of reference voltage buffer when ADC is enable | - | - | 8 | $12^{(2)}$ | $\mu \mathrm{s}$ |
| $I_{D D}\left(V_{\text {REFINTBUF }}\right)$ | $V_{\text {REFINT }}$ buffer consumption from $V_{D D}$ when converted by ADC | - | - | 12.5 | $20^{(2)}$ | $\mu \mathrm{A}$ |
| $\Delta V_{\text {REFINT }}$ | Internal reference voltage spread over the temperature range | $V_{D D}=3 \mathrm{~V}$ | - | 5 | $7.5^{(2)}$ | mV |
| $T_{\text {Coeff }}$ | Average temperature coefficient | $-40^{\circ} \mathrm{C}<\mathrm{T}_{\mathrm{A}}<+130^{\circ} \mathrm{C}$ | - | 30 | $50^{(2)}$ | $\mathrm{ppm} /{ }^{\circ} \mathrm{C}$ |
| $A_{\text {Coeff }}$ | Long term stability | 1000 hours, $T=25^{\circ} \mathrm{C}$ | - | 300 | $1000^{(2)}$ | ppm |
| $V_{\text {DDCoeff }}$ | Average voltage coefficient | $3.0 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | - | 250 | $1200^{(2)}$ | $\mathrm{ppm} / \mathrm{V}$ |
| $V_{\text {REFINT DIV1 }}$ | $1 / 4$ reference voltage | - | 24 | 25 | 26 | $\%$ <br> $V_{\text {REFINT }}$ |
| $V_{\text {REFINT DIV2 }}$ | $1 / 2$ reference voltage |  | 49 | 50 | 51 |  |
| $V_{\text {REFINT DIV3 }}$ | $3 / 4$ reference voltage |  | 74 | 75 | 76 |  |

1. The shortest sampling time is determined in the application by multiple iterations.
2. Guaranteed by design.Figure 18. $\mathrm{V}_{\text {REFINT }}$ versus temperature
![img-18.jpeg](img-18.jpeg)

# 5.3.5 Supply current characteristics 

The current consumption is a function of several parameters and factors such as the operating voltage, ambient temperature, I/O pin loading, device software configuration, operating frequencies, I/O pin switching rate, program location in memory and executed binary code

The current consumption is measured as described in Figure 17: Current consumption measurement.

## Typical and maximum current consumption

The MCU is placed under the following conditions:

- All I/O pins are in analog input mode
- All peripherals are disabled except when explicitly mentioned
- The Flash memory access time is adjusted with the minimum wait states number, depending on the $f_{\text {HCLK }}$ frequency (refer to the table "number of wait states according to CPU clock (HCLK) frequency" available in the reference manual RM0440 "STM32G4 Series advanced Arm ${ }^{\circledR}$-based 32-bit MCUs").
- When the peripherals are enabled $f_{\text {PCLK }}=f_{\text {HCLK }}$
- The voltage scaling Range 1 is adjusted to $f_{\text {HCLK }}$ frequency as follows:
- Voltage Range 1 Boost mode for $150 \mathrm{MHz}<f_{\text {HCLK }} \leq 170 \mathrm{MHz}$
- Voltage Range 1 Normal mode for $26 \mathrm{MHz}<f_{\text {HCLK }} \leq 150 \mathrm{MHz}$

The parameters given in Table 21 to Table 24 are derived from tests performed under ambient temperature and supply voltage conditions summarized in Table 17: General operating conditions.Table 21. Current consumption in Run and Low-power run modes, code with data processing running from Flash in single Bank, ART enable (Cache ON Prefetch OFF)

| Symbol | Parameter | Condition |  | $f_{\text {HCLK }}$ | Typ |  |  |  | Max |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | Voltage scaling |  | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |
| IDD (Run) | Supply current in Run mode | $f_{\text {HCLK }} \times f_{\text {HSE }}$ up to 48 MHz included, bypass mode PLL ON above 48 MHz all peripherals disable | Range 2 | 26 MHz | 3.20 | 3.35 | 3.60 | 4.15 | 5.05 | 3.90 | 4.90 | 7.40 | 11.0 | 15.0 | mA |
|  |  |  |  | 16 MHz | 2.05 | 2.15 | 2.50 | 3.05 | 3.95 | 2.70 | 3.70 | 6.10 | 9.30 | 14.0 |
|  |  |  |  | 8 MHz | 1.10 | 1.25 | 1.60 | 2.10 | 3.05 | 1.70 | 2.60 | 5.10 | 8.30 | 13.0 |
|  |  |  |  | 4 MHz | 0.635 | 0.755 | 1.15 | 1.65 | 2.60 | 1.10 | 2.10 | 3.60 | 7.60 | 13.0 |
|  |  |  |  | 2 MHz | 0.400 | 0.525 | 0.910 | 1.45 | 2.35 | 0.840 | 1.90 | 3.40 | 7.40 | 12.0 |
|  |  |  |  | 1 MHz | 0.280 | 0.415 | 0.800 | 1.35 | 2.25 | 0.710 | 1.70 | 3.20 | 7.30 | 12.0 |
|  |  |  |  | 100 KHz | 0.170 | 0.305 | 0.690 | 1.20 | 2.15 | 0.590 | 1.60 | 3.10 | 1.00 | 12.0 |
|  |  |  |  | Range 1 Boost mode | 170 MHz | 25.5 | 26.0 | 27.0 | 27.5 | 29.0 | 28.0 | 29.0 | 33.0 | 38.0 | 44.0 |
|  |  |  |  | Range 1 | 150 MHz | 21.0 | 21.5 | 22.0 | 23.0 | 24.0 | 23.0 | 24.0 | 28.0 | 32.0 | 38.0 |
|  |  |  |  |  | 120 MHz | 17.0 | 17.5 | 18.0 | 18.5 | 20.0 | 19.0 | 20.0 | 24.0 | 28.0 | 33.0 |
|  |  |  |  |  | 80 MHz | 11.5 | 11.5 | 12.5 | 13.0 | 14.0 | 13.0 | 14.0 | 18.0 | 22.0 | 27.0 |
|  |  |  |  |  | 72 MHz | 10.5 | 10.5 | 11.0 | 12.0 | 13.0 | 12.0 | 13.0 | 17.0 | 21.0 | 26.0 |
|  |  |  |  |  | 64 MHz | 9.30 | 9.50 | 10.0 | 11.0 | 12.0 | 11.0 | 12.0 | 15.0 | 20.0 | 25.0 |
|  |  |  |  |  | 48 MHz | 6.95 | 7.15 | 7.45 | 8.15 | 9.30 | 8.10 | 9.40 | 13.0 | 17.0 | 23.0 |
|  |  |  |  |  | 32 MHz | 4.70 | 4.90 | 5.25 | 5.95 | 7.10 | 5.80 | 7.10 | 11.0 | 15.0 | 20.0 |
|  |  |  |  |  | 24 MHz | 3.60 | 3.80 | 4.20 | 4.85 | 6.00 | 4.60 | 5.90 | 9.20 | 14.0 | 19.0 |
|  |  |  |  |  | 16 MHz | 2.45 | 2.65 | 3.10 | 3.75 | 4.90 | 3.40 | 4.70 | 8.00 | 13.0 | 18.0 |Table 21. Current consumption in Run and Low-power run modes, code with data processing running from Flash in single Bank, ART enable (Cache ON Prefetch OFF) (continued)

| Symbol | Parameter | Condition |  | $f_{\text {HCLK }}$ | Typ |  |  |  |  | Max |  |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - |  |  | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |  |
| IDD (LPRun) | Supply current in Low-power run mode | $f_{\text {HCLK }}=f_{\text {HSE }}$ <br> all peripherals disable |  |  | 2 MHz | 350 | 525 | 990 | 1600 | 2650 | 970 | 2200 | 3900 | 6700 | 11000 | $\mu \mathrm{A}$ |
|  |  |  |  |  | 1 MHz | 255 | 410 | 860 | 1500 | 2550 | 830 | 2100 | 3800 | 6600 | 11000 |
|  |  |  |  |  | 250 KHz | 145 | 300 | 750 | 1400 | 2450 | 690 | 1900 | 3700 | 6500 | 11000 |
|  |  |  |  |  | 62.5 KHz | 99.5 | 270 | 725 | 1350 | 2400 | 670 | 1800 | 3700 | 6500 | 11000 |
|  |  | $f_{\text {HCLK }}=f_{\text {HSI }} /$ HPRE <br> all peripherals disable |  |  | 2 MHz | 865 | 1050 | 1500 | 2150 | 3200 | 1600 | 2800 | 4500 | 7600 | 12000 |
|  |  |  |  |  | 1 MHz | 820 | 965 | 1400 | 2050 | 3100 | 1500 | 2700 | 4400 | 7400 | 12000 |
|  |  |  |  |  | 250 KHz | 725 | 875 | 1300 | 1950 | 3000 | 1400 | 2600 | 4400 | 7400 | 12000 |
|  |  |  |  |  | 62.5 KHz | 685 | 860 | 1300 | 1900 | 2950 | 1300 | 2600 | 4400 | 7400 | 12000 |Table 22. Current consumption in Run and Low-power run modes, code with data processing running from SRAM1

| Symbol | Parameter | Condition |  | $\mathrm{f}_{\text {HCLK }}$ | Typ |  |  |  | Max |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | Voltage scaling |  | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |
| IDD (Run) | Supply current in Run mode | $\mathrm{f}_{\text {HCLK }}=\mathrm{f}_{\text {HSE }}$ up to 48 MHz included, bypass mode PLL ON above 48 MHz all peripherals disable | Range 2 | 26 MHz | 2.85 | 3.00 | 3.30 | 3.85 | 4.75 | 3.50 | 4.40 | 6.90 | 11.0 | 15.0 |
|  |  |  |  | 16 MHz | 1.80 | 1.95 | 2.30 | 2.85 | 3.75 | 2.50 | 3.40 | 5.90 | 9.00 | 14.0 |
|  |  |  |  | 8 MHz | 0.995 | 1.15 | 1.50 | 2.05 | 2.95 | 1.50 | 2.50 | 4.00 | 7.80 | 13.0 |
|  |  |  |  | 4 MHz | 0.580 | 0.725 | 1.10 | 1.65 | 2.55 | 1.10 | 2.10 | 3.50 | 7.40 | 13.0 |
|  |  |  |  | 2 MHz | 0.370 | 0.510 | 0.900 | 1.45 | 2.35 | 0.800 | 1.80 | 3.30 | 7.00 | 12.0 |
|  |  |  |  | 1 MHz | 0.270 | 0.405 | 0.790 | 1.35 | 2.25 | 0.690 | 1.70 | 3.20 | 6.90 | 12.0 |
|  |  |  |  | 100 KHz | 0.170 | 0.310 | 0.695 | 1.25 | 2.15 | 0.590 | 1.60 | 3.10 | 6.70 | 12.0 |
|  |  |  | Range 1 Boost mode |  | 170 MHz | 23.0 | 23.5 | 24.0 | 25.0 | 26.5 | 24.0 | 26.0 | 30.0 | 35.0 | 41.0 |
|  |  |  |  | 150 MHz | 19.0 | 19.5 | 20.0 | 20.5 | 22.0 | 20.0 | 21.0 | 25.0 | 29.0 | 35.0 |
|  |  |  |  | 120 MHz | 15.5 | 15.5 | 16.0 | 17.0 | 18.0 | 16.0 | 18.0 | 21.0 | 25.0 | 31.0 |
|  |  |  |  | 80 MHz | 10.5 | 10.5 | 11.0 | 12.0 | 13.0 | 11.0 | 13.0 | 16.0 | 20.0 | 26.0 |
|  |  |  |  | 72 MHz | 9.35 | 9.55 | 10.0 | 11.0 | 12.0 | 11.0 | 12.0 | 15.0 | 19.0 | 25.0 |
|  |  |  |  | 64 MHz | 8.35 | 8.55 | 9.15 | 9.85 | 11.0 | 9.10 | 11.0 | 14.0 | 18.0 | 24.0 |
|  |  |  |  | 48 MHz | 6.25 | 6.45 | 6.75 | 7.45 | 8.65 | 7.10 | 8.40 | 12.0 | 16.0 | 22.0 |
|  |  |  |  | 32 MHz | 4.25 | 4.45 | 4.80 | 5.50 | 6.65 | 5.20 | 6.50 | 9.80 | 14.0 | 20.0 |
|  |  |  |  | 24 MHz | 3.25 | 3.45 | 3.85 | 4.55 | 5.65 | 4.20 | 5.40 | 8.70 | 13.0 | 19.0 |
|  |  |  |  | 16 MHz | 2.25 | 2.40 | 2.85 | 3.55 | 4.70 | 3.00 | 4.40 | 7.40 | 12.0 | 18.0 |Table 22. Current consumption in Run and Low-power run modes, code with data processing running from SRAM1 (continued)

| Symbol | Parameter | Condition |  | $f_{\text {HCLK }}$ | Typ |  |  |  | Max |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | Voltage scaling |  | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |
| IDD (LPRun) | Supply current in Low-power run mode | $f_{\text {HCLK }}=f_{\text {HSE }}$ <br> all peripherals disable |  | 2 MHz | 350 | 495 | 965 | 1600 | 2650 | 900 | 2100 | 3900 | 6700 | 12000 | $\mu \mathrm{A}$ |
|  |  |  |  | 1 MHz | 210 | 370 | 845 | 1500 | 2550 | 780 | 2000 | 3800 | 6600 | 11000 |
|  |  |  |  | 250 KHz | 115 | 275 | 755 | 1400 | 2450 | 680 | 1900 | 3700 | 6800 | 12000 |
|  |  |  |  | 62.5 KHz | 98.0 | 255 | 725 | 1350 | 2400 | 630 | 1800 | 3600 | 6400 | 11000 |
|  |  | $f_{\text {HCLK }}=f_{\text {HSI }} /$ HPRE <br> all peripherals disable |  | 2 MHz | 850 | 1000 | 1500 | 2100 | 3150 | 1500 | 2700 | 4500 | 7500 | 12000 |
|  |  |  |  | 1 MHz | 770 | 900 | 1400 | 2000 | 3050 | 1500 | 2700 | 4500 | 7600 | 13000 |
|  |  |  |  | 250 KHz | 720 | 840 | 1300 | 1950 | 3000 | 1400 | 2600 | 4400 | 7300 | 12000 |
|  |  |  |  | 62.5 KHz | 665 | 830 | 1300 | 1900 | 2950 | 1400 | 2600 | 4400 | 7300 | 12000 |Table 23. Typical current consumption in Run and Low-power run modes, with different codes running from Flash, ART enable (Cache ON Prefetch OFF)

| Symbol | Parameter | Conditions |  | Code | TYP <br> Single Bank <br> Mode | Unit | TYP <br> Single Bank Mode | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | Voltage scaling |  | $25^{\circ} \mathrm{C}$ |  | $25^{\circ} \mathrm{C}$ |  |
| $\begin{aligned} & \text { IDD } \\ & \text { (Run) } \end{aligned}$ | Supply current in Run mode | $f_{\text {HCLK }}=f_{\text {HSE }}$ up to 48 MHZ included, bypass mode PLL ON above 48 MHz all peripherals disable | Range2 $f_{\text {HCLK }}=26 \mathrm{MHz}$ | Pseudo-dhrystone | 3.20 | mA | 123 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  | Coremark | 3.15 | mA | 121 |  |
|  |  |  |  | Dhrystone2.1 | 3.20 | mA | 123 |  |
|  |  |  |  | Fibonacci | 3.60 | mA | 138 |  |
|  |  |  |  | While(1) | 3.00 | mA | 115 |  |
|  |  |  | Range 1 $f_{\text {HCLK }}=150 \mathrm{MHz}$ | Pseudo-dhrystone | 21.0 | mA | 140 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  | Coremark | 21.0 | mA | 140 |  |
|  |  |  |  | Dhrystone2.1 | 21.0 | mA | 140 |  |
|  |  |  |  | Fibonacci | 23.5 | mA | 157 |  |
|  |  |  |  | While(1) | 20.0 | mA | 133 |  |
|  |  |  | Range 1 Boost mode $f_{\text {HCLK }}=170 \mathrm{MHz}$ | Pseudo-dhrystone | 25.5 | mA | 150 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  | Coremark | 25.0 | mA | 147 |  |
|  |  |  |  | Dhrystone2.1 | 26.0 | mA | 153 |  |
|  |  |  |  | Fibonacci | 28.5 | mA | 168 |  |
|  |  |  |  | While(1) | 24.5 | mA | 144 |  |
| $\begin{aligned} & \text { IDD } \\ & \text { (LPRun) } \end{aligned}$ | Supply current in Low-power run | SYSCLK source is HSI $f_{\text {HCLK }}=2 \mathrm{MHz}$ all peripherals disable |  | Pseudo-dhrystone | 865 | $\mu \mathrm{A}$ | 433 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  | Coremark | 855 | $\mu \mathrm{A}$ | 428 |  |
|  |  |  |  | Dhrystone2.1 | 875 | $\mu \mathrm{A}$ | 438 |  |
|  |  |  |  | Fibonacci | 905 | $\mu \mathrm{A}$ | 453 |  |
|  |  |  |  | While(1) | 870 | $\mu \mathrm{A}$ | 435 |  |Table 24. Typical current consumption in Run and Low-power run modes, with different codes running from SRAM1

| Symbol | Parameter | Conditions |  | Code | TYP $25^{\circ} \mathrm{C}$ | Unit | TYP $25^{\circ} \mathrm{C}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | Voltage scaling |  | Single bank mode |  | Single bank mode |  |
| IDD (Run) | Supply current in Run mode | $f_{\text {HCLK }}=f_{\text {HSE }}$ up to 48 MHZ included, bypass mode PLL ON above 48 MHz all peripherals disable | Range2 $f_{\text {HCLK }}=26 \mathrm{MHz}$ | Pseudo-dhrystone | 2.85 | mA | 110 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  | Coremark | 2.95 | mA | 113 |  |
|  |  |  |  | Dhrystone2.1 | 2.85 | mA | 110 |  |
|  |  |  |  | Fibonacci | 2.85 | mA | 110 |  |
|  |  |  |  | While(1) | 3.05 | mA | 117 |  |
|  |  |  | Range 1 $f_{\text {HCLK }}=150 \mathrm{MHz}$ | Pseudo-dhrystone | 19.0 | mA | 127 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  | Coremark | 19.5 | mA | 130 |  |
|  |  |  |  | Dhrystone2.1 | 19.0 | mA | 127 |  |
|  |  |  |  | Fibonacci | 20.5 | mA | 137 |  |
|  |  |  |  | While(1) | 18.5 | mA | 123 |  |
|  |  |  | Range 1 Boost mode $f_{\text {HCLK }}=170 \mathrm{MHz}$ | Pseudo-dhrystone | 23.0 | mA | 135 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  | Coremark | 24.0 | mA | 141 |  |
|  |  |  |  | Dhrystone2.1 | 23.0 | mA | 135 |  |
|  |  |  |  | Fibonacci | 24.5 | mA | 144 |  |
|  |  |  |  | While(1) | 22.0 | mA | 129 |  |
| $\begin{aligned} & \text { IDD } \\ & \text { (LPRun) } \end{aligned}$ | Supply current in Low-power run | SYSCLK source is HSI $f_{\text {HCLK }}=2 \mathrm{MHz}$ all peripherals disable |  | Pseudo-dhrystone | 850 | $\mu \mathrm{A}$ | 425 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  | Coremark | 870 | $\mu \mathrm{A}$ | 435 |  |
|  |  |  |  | Dhrystone2.1 | 840 | $\mu \mathrm{A}$ | 420 |  |
|  |  |  |  | Fibonacci | 855 | $\mu \mathrm{A}$ | 428 |  |
|  |  |  |  | While(1) | 820 | $\mu \mathrm{A}$ | 410 |  |Table 25. Typical current consumption in Run and Low-power run modes, with different codes running from SRAM2

| Symbol | Parameter | Conditions |  | Code | TYP 25 ${ }^{\circ} \mathrm{C}$ | Unit | TYP 25 ${ }^{\circ} \mathrm{C}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | Voltage scaling |  | Single bank mode |  |  |  |
|  |  |  |  | Range2 $t_{\text {HCLK }}=26 \mathrm{MHz}$ |  | Pseudo-dhrystone | 2.40 | mA | 92 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  |  | Coremark | 2.50 | mA | 96 |  |
|  |  |  |  |  | Dhrystone2.1 | 2.40 | mA | 92 |  |
|  |  |  |  |  | Fibonacci | 2.35 | mA | 90 |  |
|  |  |  |  |  | While(1) | 2.25 | mA | 87 |  |
|  |  |  |  | Range 1 <br> $\mathrm{t}_{\text {HCLK }}=150 \mathrm{MHz}$ | Pseudo-dhrystone | 15.5 | mA | 103 |  |
|  |  |  |  |  | Coremark | 16.5 | mA | 110 |  |
|  |  |  |  |  | Dhrystone2.1 | 15.5 | mA | 103 |  |
|  |  |  |  |  | Fibonacci | 15.5 | mA | 103 |  |
|  |  |  |  |  | While(1) | 14.5 | mA | 97 |  |
|  |  |  |  | Range 1 <br> Boost mode <br> $\mathrm{t}_{\text {HCLK }}=170 \mathrm{MHz}$ | Pseudo-dhrystone | 19.0 | mA | 112 |  |
|  |  |  |  |  | Coremark | 20.0 | mA | 118 |  |
|  |  |  |  |  | Dhrystone2.1 | 19.0 | mA | 112 |  |
|  |  |  |  |  | Fibonacci | 19.0 | mA | 112 |  |
|  |  |  |  |  | While(1) | 18.0 | mA | 106 |  |
|  |  | SYSCLK source is HSI <br> $\mathrm{t}_{\text {HCLK }}=2 \mathrm{MHz}$ <br> all peripherals disable |  |  | Pseudo-dhrystone | 835 | $\mu \mathrm{A}$ | 418 |  |
|  |  |  |  |  | Coremark | 825 | $\mu \mathrm{A}$ | 413 |  |
|  |  |  |  |  | Dhrystone2.1 | 830 | $\mu \mathrm{A}$ | 415 |  |
|  |  |  |  |  | Fibonacci | 830 | $\mu \mathrm{A}$ | 415 |  |
|  |  |  |  |  | While(1) | 815 | $\mu \mathrm{A}$ | 408 |  |Table 26. Typical current consumption in Run and Low-power run modes, with different codes running from CCM

| Symbol | Parameter | Conditions |  | Code | TYP 25 ${ }^{\circ} \mathrm{C}$ | Unit | TYP 25 ${ }^{\circ} \mathrm{C}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | Voltage scaling |  | Single bank mode |  |  |  |
|  |  |  |  | Range2 $f_{\text {HCLK }}=26 \mathrm{MHz}$ | Pseudo-dhrystone | 2.65 | mA | 102 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  |  | Coremark | 2.80 | mA | 108 |
|  |  |  |  |  | Dhrystone2.1 | 2.65 | mA | 102 |
|  |  |  |  |  | Fibonacci | 3.25 | mA | 125 |
|  |  |  |  |  | While(1) | 3.25 | mA | 125 |
|  |  |  |  | Range 1 <br> fHCLK $=150 \mathrm{MHz}$ | Pseudo-dhrystone | 17.5 | mA | 117 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  |  | Coremark | 19.0 | mA | 127 |
|  |  |  |  |  | Dhrystone2.1 | 17.5 | mA | 117 |
|  |  |  |  |  | Fibonacci | 21.5 | mA | 143 |
|  |  |  |  |  | While(1) | 21.5 | mA | 143 |
|  |  |  |  | Range 1 <br> Boost mode <br> fHCLK $=170 \mathrm{MHz}$ | Pseudo-dhrystone | 21.5 | mA | 126 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  |  | Coremark | 23.0 | mA | 135 |
|  |  |  |  |  | Dhrystone2.1 | 21.5 | mA | 126 |
|  |  |  |  |  | Fibonacci | 26.0 | mA | 153 |
|  |  |  |  |  | While(1) | 26.0 | mA | 153 |
| $\begin{aligned} & \text { IDD } \\ & \text { (LPRun) } \end{aligned}$ | Supply current in Low-power run | SYSCLK source is HSI fHCLK $=2 \mathrm{MHz}$ all peripherals disable |  | Pseudo-dhrystone | 845 | $\mu \mathrm{A}$ | 423 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  |  |  |  |  | Coremark | 825 | $\mu \mathrm{A}$ | 413 |
|  |  |  |  |  | Dhrystone2.1 | 820 | $\mu \mathrm{A}$ | 410 |
|  |  |  |  |  | Fibonacci | 885 | $\mu \mathrm{A}$ | 443 |
|  |  |  |  |  | While(1) | 890 | $\mu \mathrm{A}$ | 445 |Table 27. Current consumption in Sleep and Low-power sleep mode Flash ON

| Symbol | Parameter | Condition | | $t_{\text {HCLK }}$ | Typ | | | | | Max | | | | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| | | - | Voltage scaling |  | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |
| IDD (Sleep) | Supply current in Sleep mode | $t_{\text {HCLK }}=\mathrm{t}_{\text {HSE }}$ up to 48 MHz included, bypass mode PLL ON above 48 MHz all peripherals disable | Range 2 | 26 MHz | 1.05 | 1.15 | 1.45 | 2.00 | 2.90 | 1.70 | 2.70 | 5.20 | 8.30 | 13.0 | mA |
|  |  |  |  | 16 MHz | 0.690 | 0.810 | 1.15 | 1.70 | 2.60 | 1.30 | 2.30 | 3.80 | 8.00 | 13.0 |
|  |  |  |  | 8 MHz | 0.425 | 0.545 | 0.920 | 1.45 | 2.35 | 0.930 | 2.00 | 3.50 | 7.70 | 13.0 |
|  |  |  |  | 4 MHz | 0.300 | 0.400 | 0.815 | 1.35 | 2.25 | 0.760 | 1.80 | 3.30 | 7.60 | 12.0 |
|  |  |  |  | 2 MHz | 0.230 | 0.355 | 0.755 | 1.30 | 2.20 | 0.670 | 1.70 | 3.20 | 7.50 | 12.0 |
|  |  |  |  | 1 MHz | 0.200 | 0.320 | 0.725 | 1.25 | 2.15 | 0.620 | 1.60 | 3.10 | 7.50 | 12.0 |
|  |  |  |  | 100 KHz | 0.165 | 0.285 | 0.690 | 1.25 | 2.15 | 0.580 | 1.60 | 3.10 | 7.40 | 12.0 |
|  |  |  |  | Range 1 <br> Boost <br> mode | 170 MHz | 7.40 | 7.65 | 8.30 | 9.10 | 10.5 | 8.60 | 11.0 | 14.0 | 19.0 | 25.0 |
|  |  |  |  |  | 150 MHz | 6.10 | 6.30 | 6.90 | 7.60 | 8.80 | 7.10 | 8.40 | 12.0 | 16.0 | 22.0 |
|  |  |  |  | Range 1 | 120 MHz | 4.95 | 5.15 | 5.70 | 6.40 | 7.60 | 5.90 | 7.20 | 11.0 | 15.0 | 21.0 |
|  |  |  |  |  | 80 MHz | 3.45 | 3.65 | 4.15 | 4.85 | 6.00 | 4.40 | 5.70 | 9.00 | 13.0 | 19.0 |
|  |  |  |  |  | 72 MHz | 3.15 | 3.35 | 3.85 | 4.55 | 5.70 | 4.10 | 5.30 | 8.60 | 13.0 | 19.0 |
|  |  |  |  |  | 64 MHz | 2.85 | 3.00 | 3.55 | 4.25 | 5.40 | 3.70 | 5.00 | 8.30 | 13.0 | 18.0 |
|  |  |  |  |  | 48 MHz | 2.10 | 2.30 | 2.55 | 3.25 | 4.40 | 3.10 | 4.40 | 7.60 | 12.0 | 18.0 |
|  |  |  |  |  | 32 MHz | 1.50 | 1.65 | 2.00 | 2.70 | 3.80 | 2.50 | 3.80 | 7.10 | 12.0 | 17.0 |
|  |  |  |  |  | 24 MHz | 1.15 | 1.35 | 1.75 | 2.40 | 3.55 | 2.20 | 3.40 | 6.70 | 11.0 | 17.0 |
|  |  |  |  |  | 16 MHz | 0.850 | 1.05 | 1.45 | 2.15 | 3.25 | 1.80 | 3.10 | 6.30 | 11.0 | 16.0 |Table 27. Current consumption in Sleep and Low-power sleep mode Flash ON (continued)

| Symbol | Parameter | Condition | | $\mathbf{f}_{\text {HCLK }}$ | Typ | | | | | Max | | | | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| | | - | Voltage scaling |  |  | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |
| IDD <br> (LPSleep) | Supply current in Low-power sleep mode | $f_{\text {HCLK }}=f_{\text {HSE }}$ <br> all peripherals disable |  | 2 MHz | 180 | 335 | 810 | 1450 | 2500 | 1600 | 2900 | 4600 | 7700 | 13000 | $\mu \mathrm{A}$ |
|  |  |  |  | 1 MHz | 135 | 300 | 770 | 1400 | 2450 | 1200 | 2400 | 4100 | 7100 | 12000 |
|  |  |  |  | 250 KHz | 115 | 265 | 740 | 1350 | 2400 | 670 | 2000 | 3600 | 6300 | 11000 |
|  |  |  |  | 62.5 KHz | 89.5 | 255 | 730 | 1350 | 2400 | 660 | 1900 | 3600 | 6300 | 11000 |
|  |  | $f_{\text {HCLK }}=f_{\text {HSI }} /$ HPRE all peripherals disable |  | 2 MHz | 730 | 875 | 1350 | 1950 | 3000 | 1400 | 2600 | 4300 | 7200 | 12000 |
|  |  |  |  | 1 MHz | 675 | 830 | 1300 | 1950 | 3000 | 1400 | 2600 | 4300 | 7200 | 12000 |
|  |  |  |  | 250 KHz | 655 | 820 | 1300 | 1950 | 3000 | 1400 | 2600 | 4300 | 7200 | 12000 |
|  |  |  |  | 62.5 KHz | 680 | 850 | 1300 | 1950 | 3000 | 1400 | 2600 | 4300 | 7200 | 12000 |

Table 28. Current consumption in low-power sleep modes, Flash in power-down

| Symbol | Parameter | Condition |  | $\mathbf{f}_{\text {HCLK }}$ | Typ |  |  |  | Max |  |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | Voltage scaling |  | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |
| IDD <br> (LPSleep) | Supply current in low-power sleep mode | $f_{\text {HCLK }}=f_{\text {HSE }}$ all peripherals disable | - | 2 MHz | 175 | 290 | 805 | 1450 | 2500 | 750 | 2000 | 3700 | 6400 | 11000 | $\mu \mathrm{A}$ |
|  |  |  |  | 1 MHz | 125 | 280 | 765 | 1400 | 2450 | 700 | 2000 | 3700 | 6400 | 11000 |
|  |  |  |  | 250 KHz | 105 | 240 | 735 | 1350 | 2400 | 670 | 1900 | 3700 | 6400 | 11000 |
|  |  |  |  | 62.5 KHz | 105 | 245 | 725 | 1350 | 2400 | 660 | 1900 | 3700 | 6400 | 11000 |
|  |  | $f_{\text {HCLK }}=f_{\text {HSI }}$ all peripherals disable | - | 2 MHz | 670 | 830 | 1350 | 1950 | 3000 | 1400 | 2600 | 4300 | 7200 | 12000 |
|  |  |  |  | 1 MHz | 655 | 825 | 1300 | 1950 | 3000 | 1400 | 2600 | 4300 | 7200 | 12000 |
|  |  |  |  | 250 KHz | 635 | 825 | 1300 | 1900 | 2950 | 1400 | 2600 | 4300 | 7100 | 12000 |
|  |  |  |  | 62.5 KHz | 640 | 840 | 1300 | 1900 | 2950 | 1200 | 2200 | 3700 | 6200 | 11000 |Table 29. Current consumption in Stop 1 mode

|  Symbol | Parameter | Conditions |  |  | TYP |  |  |  | $\operatorname{MAX}^{(1)}$ |  |  |  | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  | - | VDD | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$  |
|  IDD
(Stop 1) | Supply current in Stop 1 mode, RTC disabled | RTC disabled | 1.8 V | 58.5 | 175 | 550 | 1050 | 1900 | 430 | 1400 | 3600 | 6500 | 11000  |
|   |  |  | 2.4 V | 58.5 | 175 | 550 | 1050 | 1950 | 430 | 1400 | 3600 | 6600 | 11000  |
|   |  |  | 3.0 V | 59.0 | 175 | 555 | 1050 | 1950 | 430 | 1400 | 3700 | 6600 | 11000  |
|   |  |  | 3.6 V | 59.5 | 180 | 560 | 1100 | 1950 | 430 | 1400 | 3700 | 6700 | 11000  |
|  IDD
(Stop 1
with RTC) | Supply current in Stop 1 mode, RTC enabled | RTC clocked by LSI | 1.8 V | 59.0 | 175 | 550 | 1050 | 1950 | 430 | 1400 | 3600 | 6500 | 11000  |
|   |  |  | 2.4 V | 59.5 | 175 | 555 | 1050 | 1950 | 430 | 1400 | 3600 | 6600 | 11000  |
|   |  |  | 3.0 V | 59.5 | 175 | 555 | 1050 | 1950 | 440 | 1400 | 3700 | 6600 | 11000  |
|   |  |  | 3.6 V | 60.5 | 180 | 560 | 1100 | 1950 | 440 | 1400 | 3700 | 6700 | 11000  |
|   |  | RTC clocked by LSE bypassed at 32768 Hz | 1.8 V | 58.5 | 175 | 550 | 1050 | 1900 | - | - | - | - | -  |
|   |  |  | 2.4 V | 59.0 | 175 | 555 | 1050 | 1950 | - | - | - | - | -  |
|   |  |  | 3.0 V | 60.0 | 180 | 555 | 1050 | 1950 | - | - | - | - | -  |
|   |  |  | 3.6 V | 62.0 | 180 | 565 | 1100 | 1950 | - | - | - | - | -  |
|   |  | RTC clocked by LSE quartz in low drive mode at 32768 Hz | 1.8 V | 58.5 | 150 | 445 | 890 | - | - | - | - | - | -  |
|   |  |  | 2.4 V | 59.0 | 150 | 445 | 890 | - | - | - | - | - | -  |
|   |  |  | 3.0 V | 59.5 | 150 | 445 | 890 | - | - | - | - | - | -  |
|   |  |  | 3.6 V | 61.0 | 150 | 450 | 895 | - | - | - | - | - | -  |
|  IDD
(Stop 1
with RTC) | Supply current during wakeup from Stop 1 mode | Wakeup clock is HSI $=16 \mathrm{MHz}$, | 3.0 V | 1.39 | - | - | - | - | - | - | - | - | mA  |
|   |  | Wakeup clock is $\mathrm{HSI}=4 \mathrm{MHz}$, (HPRE divider=4), voltage Range 2 | 3.0 V | 0.93 | - | - | - | - | - | - | - | - |   |

1. Guaranteed by characterization results, unless otherwise specified.Table 30. Current consumption in Stop 0 mode

| Symbol | Parameter | Conditions |  | TYP |  |  |  | $\operatorname{MAX}^{(1)}$ |  |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | VDD | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |
| IDD(Stop 0) | Supply current in Stop 0 mode, RTC disabled |  | 1.8 V | 150 | 280 | 680 | 1200 | 2100 | 560 | 1600 | 4000 | 7100 | 12000 | $\mu \mathrm{A}$ |
|  |  |  | 2.4 V | 150 | 280 | 680 | 1200 | 2100 | 560 | 1600 | 4000 | 7100 | 12000 |
|  |  |  | 3 V | 155 | 280 | 685 | 1200 | 2150 | 560 | 1600 | 4000 | 7100 | 12000 |
|  |  |  | 3.6 V | 155 | 285 | 685 | 1200 | 2150 | 560 | 1600 | 4000 | 7200 | 12000 |

1. Guaranteed by characterization results, unless otherwise specified.

Table 31. Current consumption in Standby mode

| Symbol | Parameter | Conditions |  | TYP |  |  |  | $\operatorname{MAX}^{(1)}$ |  |  |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | - | VDD | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |
| IDD <br> (Standby) | Supply current in Standby mode (backup registers retained), RTC disabled | No independent watchdog | 1.8 V | 92.0 | 205 | 870 | 2250 | 5600 | 220 | 1000 | 3200 | 8000 | 20000 | nA |
|  |  |  | 2.4 V | 100 | 240 | 1000 | 2600 | 6450 | 250 | 1100 | 3600 | 9000 | 23000 |
|  |  |  | 3 V | 120 | 280 | 1200 | 3050 | 7400 | 280 | 1300 | 4200 | 11000 | 26000 |
|  |  |  | 3.6 V | 175 | 385 | 1550 | 3800 | 9200 | 370 | 1500 | 4900 | 12000 | 30000 |
|  |  | With independent watchdog | 1.8 V | 275 | - | - | - | - | - | - | - | - | - |
|  |  |  | 2.4 V | 335 | - | - | - | - | - | - | - | - | - |
|  |  |  | 3 V | 400 | - | - | - | - | - | - | - | - | - |
|  |  |  | 3.6 V | 510 | - | - | - | - | - | - | - | - | - |Table 31. Current consumption in Standby mode (continued)

|  Symbol | Parameter | Conditions |  | TYP |  |  |  |  | MAX ${ }^{(1)}$ |  |  |  |  | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  | - | VDD | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |   |
|  $\begin{aligned} & \text { IDD } \ & \text { (Standby with } \ & \text { RTC) } \end{aligned}$ | Supply current in Standby mode (backup registers retained), RTC enabled | RTC clocked by LSI, no independent watchdog | 1.8 V | 490 | 605 | 1300 | 2650 | 5950 | 670 | 1400 | 3600 | 8200 | 20000 | nA  |
|   |  |  | 2.4 V | 630 | 765 | 1550 | 3100 | 6950 | 850 | 1700 | 4200 | 9400 | 23000 |   |
|   |  |  | 3 V | 785 | 955 | 1850 | 3700 | 8050 | 1100 | 1900 | 4900 | 11000 | 26000 |   |
|   |  |  | 3.6 V | 1000 | 1200 | 2350 | 4600 | 9950 | 1400 | 2400 | 5800 | 13000 | 30000 |   |
|   |  | RTC clocked by LSI, with independent watchdog | 1.8 V | 530 | - | - | - | - | - | - | - | - | - |   |
|   |  |  | 2.4 V | 685 | - | - | - | - | - | - | - | - | - |   |
|   |  |  | 3 V | 860 | - | - | - | - | - | - | - | - | - |   |
|   |  |  | 3.6 V | 1100 | - | - | - | - | - | - | - | - | - |   |
|   |  | RTC clocked by LSE bypassed at 32768 Hz | 1.8 V | 360 | 470 | 1100 | 2450 | 5750 | - | - | - | - | - | nA  |
|   |  |  | 2.4 V | 480 | 625 | 1400 | 3000 | 6800 | - | - | - | - | - |   |
|   |  |  | 3 V | 825 | 1100 | 2200 | 4200 | 8700 | - | - | - | - | - |   |
|   |  |  | 3.6 V | 2550 | 3400 | 5250 | 8000 | 13500 | - | - | - | - | - |   |
|   |  | RTC clocked by LSE quartz ${ }^{(2)}$ in low drive mode | 1.8 V | 355 | 490 | 990 | 2150 | 4800 | - | - | - | - | - |   |
|   |  |  | 2.4 V | 455 | 605 | 1200 | 2550 | 5550 | - | - | - | - | - |   |
|   |  |  | 3 V | 595 | 775 | 1450 | 3100 | 6400 | - | - | - | - | - |   |
|   |  |  | 3.6 V | 810 | 1200 | 2050 | 3900 | 7750 | - | - | - | - | - |   |
|  $\begin{aligned} & \text { IDD } \ & \text { (SRAM2) }^{(3)} \end{aligned}$ | Supply current to be added in Standby mode when SRAM2 is retained | - | 1.8 V | 218 | 530 | 1680 | 3500 | 6900 | - | - | - | - | - | nA  |
|   |  |  | 2.4 V | 220 | 525 | 1700 | 3500 | 7050 | - | - | - | - | - |   |
|   |  |  | 3 V | 215 | 530 | 1650 | 3500 | 7100 | - | - | - | - | - |   |
|   |  |  | 3.6 V | 220 | 545 | 1700 | 3600 | 6800 | - | - | - | - | - |   |Table 31. Current consumption in Standby mode (continued)

|  Symbol | Parameter | Conditions |  | TYP |  |  |  |  | MAX ${ }^{(1)}$ |  |  |  |  | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  | - | VDD | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |   |
|  IDD (wakeup from Standby) | Supply current during wakeup from Standby mode | Wakeup clock is $\mathrm{HSI} 16=16 \mathrm{MHz}^{(4)}$ | 3 V | 2.0 | - | - | - | - | - | - | - | - | - | mA  |

1. Guaranteed by characterization results, unless otherwise specified.
2. Based on characterization done with a 32.768 kHz crystal (MC306-G-06Q-32.768, manufacturer JFVNY) with two 6.8 pF loading capacitors.
3. The supply current in Standby with SRAM2 mode is: IDD_ALL(Standby) + IDD_ALL(SRAM2). The supply current in Standby with RTC with SRAM2 mode is: IDD_ALL(Standby + RTC) + IDD_ALL(SRAM2).
4. Wakeup with code execution from Flash. Average value given for a typical wakeup time as specified in Table 35: Low-power mode wakeup timings.

Table 32. Current consumption in Shutdown mode

|  Symbol | Parameter | Conditions |  | TYP |  |  |  |  | $\begin{gathered} \text { MAX }^{(1)} \ 25^{\circ} \mathrm{C} \end{gathered}$ |  |  |  |  | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  | - | VDD | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |   |
|  IDD
(Shutdown) | Supply current in Shutdown mode (backup registers retained) RTC disabled | - | 1.8 V | 14.0 | 94.0 | 570 | 1600 | 4350 | 130 | 420 | 2100 | 6100 | 17000 | nA  |
|   |  |  | 2.4 V | 22.0 | 120 | 670 | 1900 | 4950 | 150 | 490 | 2400 | 6900 | 19000 |   |
|   |  |  | 3 V | 35.0 | 150 | 805 | 2200 | 5750 | 180 | 560 | 2800 | 7800 | 21000 |   |
|   |  |  | 3.6 V | 74.0 | 245 | 1100 | 2900 | 7350 | 220 | 710 | 3300 | 9100 | 24000 |   |Table 32. Current consumption in Shutdown mode (continued)

|  Symbol | Parameter | Conditions |  | TYP |  |  |  |  | $\begin{gathered} \text { MAX }^{(1)} \ 25^{\circ} \mathrm{C} \end{gathered}$ |  |  |  | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  | - | VDD | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$  |
|  IDD
(Shutdown with
RTC) | Supply current in Shutdown mode (backup registers retained) RTC enabled | RTC
clocked by
LSE
bypassed
at 32768
Hz | 1.8 V | 280 | 355 | 800 | 1800 | 4500 | - | - | - | - | -  |
|   |  |  | 2.4 V | 400 | 500 | 1050 | 2250 | 5350 | - | - | - | - | -  |
|   |  |  | 3 V | 745 | 985 | 1850 | 3400 | 7100 | - | - | - | - | -  |
|   |  |  | 3.6 V | 2450 | 3250 | 4850 | 7100 | 11500 | - | - | - | - | -  |
|   |  | RTC
clocked by
LSE
quartz ${ }^{(2)}$ in
low drive
mode | 1.8 V | 275 | 375 | 775 | 1650 | - | - | - | - | - | -  |
|   |  |  | 2.4 V | 375 | 495 | 950 | 2050 | - | - | - | - | - | -  |
|   |  |  | 3 V | 515 | 640 | 1200 | 2550 | - | - | - | - | - | -  |
|   |  |  | 3.6 V | 710 | 925 | 1750 | 3300 | - | - | - | - | - | -  |
|  IDD(wakeup from Shutdown) | Supply current during wakeup from Shutdown mode | Wakeup clock is $\mathrm{HSI} 16=$ $16 \mathrm{MHz}^{(3)}$ | 3 V | 0.24 | - | - | - | - | - | - | - | - | mA  |

1. Guaranteed by characterization results, unless otherwise specified.
2. Based on characterization done with a 32.768 kHz crystal (MC306-G-06Q-32.768, manufacturer JFVNY) with two 6.8 pF loading capacitors.
3. Wakeup with code execution from Flash. Average value given for a typical wakeup time as specified in Table 35: Low-power mode wakeup timings.Table 33. Current consumption in VBAT mode

|  Symbol | Parameter | Conditions |  | TYP |  |  |  |  | $\operatorname{MAX}^{(1)}$ |  |  |  |  | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   |  | - | VBAT | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ | $25^{\circ} \mathrm{C}$ | $55^{\circ} \mathrm{C}$ | $85^{\circ} \mathrm{C}$ | $105^{\circ} \mathrm{C}$ | $125^{\circ} \mathrm{C}$ |   |
|  IDD(VBAT) | Backup domain supply current | RTC
disabled | 1.8 V | 4.00 | 21.0 | 105 | 280 | 685 | - | - | - | - | - | nA  |
|   |  |  | 2.4 V | 5.00 | 24.0 | 120 | 310 | 765 | - | - | - | - | - |   |
|   |  |  | 3 V | 6.00 | 28.0 | 140 | 360 | 865 | - | - | - | - | - |   |
|   |  |  | 3.6 V | 15.0 | 54.0 | 240 | 615 | 1500 | - | - | - | - | - |   |
|   |  | RTC
enabled and clocked by LSE bypassed at 32768 Hz | 1.8 V | 270 | 275 | 330 | 475 | - | - | - | - | - | - |   |
|   |  |  | 2.4 V | 385 | 400 | 490 | 690 | - | - | - | - | - | - |   |
|   |  |  | 3 V | 725 | 865 | 1150 | 1550 | - | - | - | - | - | - |   |
|   |  |  | 3.6 V | 2500 | 3050 | 3900 | 4700 | - | - | - | - | - | - |   |
|   |  | RTC
enabled and clocked by LSE quartz ${ }^{(2)}$ | 1.8 V | 265 | 315 | 415 | 670 | 1000 | - | - | - | - | - |   |
|   |  |  | 2.4 V | 355 | 415 | 530 | 865 | 1150 | - | - | - | - | - |   |
|   |  |  | 3 V | 480 | 545 | 710 | 1050 | 1250 | - | - | - | - | - |   |
|   |  |  | 3.6 V | 675 | 870 | 1100 | 1500 | 1700 | - | - | - | - | - |   |

1. Guaranteed by characterization results, unless otherwise specified.
2. Based on characterization done with a 32.768 kHz crystal (MC306-G-06Q-32.768, manufacturer JFVNY) with two 6.8 pF loading capacitors.# I/O system current consumption 

The current consumption of the I/O system has two components: static and dynamic.

## I/O static current consumption

All the I/Os used as inputs with pull-up generate current consumption when the pin is externally held low. The value of this current consumption can be simply computed by using the pull-up/pull-down resistors values given in Table 53: I/O static characteristics.

For the output pins, any external pull-down or external load must also be considered to estimate the current consumption.

Additional I/O current consumption is due to I/Os configured as inputs if an intermediate voltage level is externally applied. This current consumption is caused by the input Schmitt trigger circuits used to discriminate the input value. Unless this specific configuration is required by the application, this supply current consumption can be avoided by configuring these I/Os in analog mode. This is notably the case of ADC, OPAMP, COMP input pins which should be configured as analog inputs.

Caution: Any floating input pin can also settle to an intermediate voltage level or switch inadvertently, as a result of external electromagnetic noise. To avoid current consumption related to floating pins, they must either be configured in analog mode, or forced internally to a definite digital value. This is done either by using pull-up/down resistors or by configuring the pins in output mode.

I/O dynamic current consumption
In addition to the internal peripheral current consumption measured previously (see Table 35: Low-power mode wakeup timings), the I/Os used by an application also contribute to the current consumption. When an I/O pin switches, it uses the current from the I/O supply voltage to supply the I/O pin circuitry and to charge/discharge the capacitive load (internal or external) connected to the pin:

$$
\mathrm{I}_{\mathrm{SW}}=\mathrm{V}_{\mathrm{DDIOx}} \times \mathrm{f}_{\mathrm{SW}} \times \mathrm{C}
$$

where
$\mathrm{I}_{\mathrm{SW}}$ is the current sunk by a switching I/O to charge/discharge the capacitive load
$\mathrm{V}_{\mathrm{DD}}$ is the $\mathrm{I} / \mathrm{O}$ supply voltage
$\mathrm{f}_{\mathrm{SW}}$ is the $\mathrm{I} / \mathrm{O}$ switching frequency
C is the total capacitance seen by the I/O pin: $\mathrm{C}=\mathrm{C}_{\mathrm{INT}}+\mathrm{C}_{\mathrm{EXT}}+\mathrm{C}_{\mathrm{S}}$
$\mathrm{C}_{\mathrm{S}}$ is the PCB board capacitance including the pad pin.
The test pin is configured in push-pull output mode and is toggled by software at a fixed frequency.# On-chip peripheral current consumption 

The current consumption of the on-chip peripherals is given in Table 34. The MCU is placed under the following conditions:

- All I/O pins are in Analog mode
- The given value is calculated by measuring the difference of the current consumptions:
- when the peripheral is clocked on
- when the peripheral is clocked off
- Ambient operating temperature and supply voltage conditions summarized in Table 14: Voltage characteristics
- The power consumption of the digital part of the on-chip peripherals is given in Table 34. The power consumption of the analog part of the peripherals (where applicable) is indicated in each related section of the datasheet.

Table 34. Peripheral current consumption

| BUS | Peripheral | Range 1 <br> Boost Mode | Range 1 | Range 2 | Low-power <br> run and sleep | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| AHB | Bus Matrix | 5.31 | 5.00 | 4.07 | 4.97 | $\mu \mathrm{A} / \mathrm{MHz}$ |
| AHB1 | DMA1 | 3.21 | 2.95 | 2.45 | 2.68 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  | DMA2 | 3.10 | 2.86 | 2.37 | 2.59 |  |
|  | DMAMUX | 7.48 | 6.97 | 5.74 | 6.43 |  |
|  | CORDIC | 1.61 | 1.50 | 1.24 | 1.34 |  |
|  | FMAC | 3.70 | 3.47 | 2.86 | 3.27 |  |
|  | FLASH | 6.10 | 5.66 | 4.64 | 5.33 |  |
|  | SRAM1 | 0.31 | 0.32 | 0.26 | 0.38 |  |Table 34. Peripheral current consumption (continued)

| BUS | Peripheral | Range 1 Boost Mode | Range 1 | Range 2 | Low-power run and sleep | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| AHB2 | CRC | 1.11 | 1.05 | 0.86 | 0.90 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  | GPIOA | 1.00 | 0.91 | 0.73 | 0.93 |  |
|  | GPIOB | 0.55 | 0.50 | 0.41 | 0.54 |  |
|  | GPIOC | 0.56 | 0.51 | 0.42 | 0.43 |  |
|  | GPIOD | 0.35 | 0.33 | 0.26 | 0.26 |  |
|  | GPIOE | 0.59 | 0.55 | 0.45 | 0.41 |  |
|  | GPIOF | 0.46 | 0.43 | 0.36 | 0.31 |  |
|  | GPIOG | 0.38 | 0.36 | 0.29 | 0.26 |  |
|  | CCMSRAM | 0.32 | 0.31 | 0.26 | 0.25 |  |
|  | SRAM2 | 0.70 | 0.66 | 0.55 | 0.55 |  |
|  | ADC12 AHB clock domain | 6.72 | 6.27 | 5.17 | 5.95 |  |
|  | ADC12 independent clock domain | 0.61 | 0.59 | 0.46 | 0.56 |  |
|  | DAC1 | 5.57 | 5.17 | 4.40 | 4.99 |  |
|  | DAC3 | 5.67 | 5.30 | NA | NA |  |
|  | RNG clock domain | 3.63 | 3.37 | NA | Na |  |
|  | RNG independent clock domain | 1.06 | 1.00 | NA | NA |  |
| AHB | ALL AHB peripherals | 79.97 | 74.54 | 57.83 | 66.98 | $\mu \mathrm{A} / \mathrm{MHz}$ |
| APB1 | AHB to APB1 bridge | 0.47 | 0.37 | 0.32 | 0.08 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  | TIM2 | 10.84 | 10.04 | 8.21 | 9.31 |  |
|  | TIM3 | 9.32 | 8.65 | 7.10 | 8.02 |  |
|  | TIM4 | 8.60 | 8.00 | 6.61 | 7.53 |  |
|  | TIM6 | 2.88 | 2.69 | 2.22 | 2.66 |  |
|  | TIM7 | 2.72 | 2.53 | 2.09 | 2.41 |  |
|  | CRS | 0.65 | 0.62 | 0.50 | 0.57 |  |
|  | RTC | 3.72 | 3.49 | 2.92 | 3.73 |  |
|  | WWDG | 0.77 | 0.74 | 0.60 | 0.71 |  |
|  | SPI2 | 4.96 | 4.63 | 3.82 | 4.33 |  |
|  | SPI3 | 5.33 | 4.98 | 4.09 | 4.67 |  |
|  | I2S2 clock domain | 3.45 | 3.23 | 2.65 | 2.95 |  |
|  | I2S2 independent clock domain | 1.51 | 1.40 | 1.17 | 1.38 |  |
|  | I2S3 clock domain | 3.86 | 3.62 | 2.97 | 3.49 |  |
|  | I2S3 independent clock domain | 1.47 | 1.36 | 1.12 | 1.18 |  |Table 34. Peripheral current consumption (continued)

| BUS | Peripheral | Range 1 <br> Boost Mode | Range 1 | Range 2 | Low-power <br> run and sleep | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| APB1 | USART2 clock domain | 3.57 | 3.36 | 2.76 | 3.22 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  | USART2 independent clock domain | 7.93 | 7.36 | 6.10 | 6.84 |  |
|  | USART3 clock domain | 3.50 | 3.29 | 2.68 | 3.12 |  |
|  | USART3 independent clock domain | 7.69 | 7.14 | 5.94 | 6.71 |  |
|  | UART4 clock domain | 3.30 | 3.10 | 2.54 | 2.91 |  |
|  | UART4 independent clock domain | 6.53 | 6.06 | 5.02 | 5.61 |  |
|  | I2C1 clock domain | 1.69 | 1.60 | 1.31 | 1.53 |  |
|  | I2C1 independent clock domain | 3.95 | 3.68 | 3.05 | 3.47 |  |
|  | I2C2 clock domain | 1.69 | 1.60 | 1.31 | 1.53 |  |
|  | I2C2 independent clock domain | 4.04 | 3.76 | 3.11 | 3.58 |  |
|  | USB clock domain | 0.57 | 0.55 | 0.44 | 0.51 |  |
|  | USB independent clock domain | 1.19 | 1.10 | 5.28 | NA |  |
|  | FDCAN clock domain | 9.52 | 8.90 | 7.32 | 8.29 |  |
|  | FDCAN independent clock domain | 4.82 | 4.48 | 3.70 | 4.37 |  |
|  | PWR | 1.26 | 1.19 | 0.96 | 1.04 |  |
|  | I2C3 clock domain | 1.68 | 1.59 | 1.30 | 1.53 |  |
|  | I2C3 independent clock domain | 2.48 | 2.30 | 1.92 | 2.19 |  |
|  | LPTIM1 clock domain | 1.52 | 1.45 | 1.17 | 1.43 |  |
|  | LPTIM1 independent clock domain | 4.38 | 4.05 | 3.38 | 3.68 |  |
|  | LPUART1 clock domain | 2.42 | 2.29 | 1.87 | 2.15 |  |
|  | LPUART1 independent clock <br> domain | 4.65 | 4.30 | 3.59 | 4.14 |  |
|  | ALL APB1 on | 138.92 | 129.50 | 105.42 | 120.34 |  |
|  | AHB to APB2 bridge | 0.43 | 0.36 | 0.30 | 0.19 |  |
|  | UCPD clock domain | 3.67 | 3.42 | 2.82 | 3.24 |  |
|  | UCPD independent clock domain | 1.28 | 1.20 | 5.73 | NA |  |Table 34. Peripheral current consumption (continued)

| BUS | Peripheral | Range 1 <br> Boost Mode | Range 1 | Range 2 | Low-power <br> run and sleep | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| APB2 | SYSCFG/VREFBUF/COMP | 1.94 | 1.81 | 1.49 | 1.82 | $\mu \mathrm{A} / \mathrm{MHz}$ |
|  | TIM1 | 12.00 | 11.16 | 9.20 | 10.41 |  |
|  | SPI1 | 2.47 | 2.32 | 1.92 | 2.18 |  |
|  | TIM8 | 11.65 | 10.83 | 8.93 | 10.17 |  |
|  | USART1 clock domain | 2.84 | 2.65 | 2.18 | 2.48 |  |
|  | USART1 independent clock domain | 7.01 | 6.53 | 5.38 | 6.17 |  |
|  | SPI4 | 2.47 | 2.32 | 1.92 | 2.18 |  |
|  | TIM15 | 6.00 | 5.57 | 4.61 | 5.26 |  |
|  | TIM16 | 4.18 | 3.89 | 3.20 | 3.57 |  |
|  | TIM17 | 4.37 | 4.06 | 3.33 | 3.76 |  |
|  | SAI1 clock domain | 3.08 | 2.88 | 2.36 | 2.79 |  |
|  | SAI1 independent clock domain | 3.07 | 2.84 | 2.35 | 2.63 |  |
|  | ALL APB2 on | 62.79 | 58.41 | 52.90 | 53.64 |  |
|  | ALL peripherals | 250.00 | 210.44 | 179.05 | 225.00 |  |# 5.3.6 Wakeup time from low-power modes and voltage scaling transition times 

The wakeup times given in Table 35 are the latency between the event and the execution of the first user instruction.

The device goes in low-power mode after the WFE (Wait For Event) instruction.
Table 35. Low-power mode wakeup timings ${ }^{(1)}$

| Symbol | Parameter | Conditions | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {WUSLEEP }}$ | Wakeup time from Sleep mode to Run mode | - | 11 | 12 | Nb of CPU cycles |
| $t_{\text {WULPSLEEP }}$ | Wakeup time from Lowpower sleep mode to Lowpower run mode | - | 10 | 11 |  |
| $t_{\text {WUSTOP0 }}$ | Wake up time from Stop 0 mode to Run mode in Flash | Range 1 | Wakeup clock HSI16 = 16 MHz | 5.8 | 6 | $\mu \mathrm{s}$ |
|  |  | Range 2 | Wakeup clock HSI16 = 16 MHz | 18.4 | 19.1 |
|  | Wake up time from Stop 0 mode to Run mode in SRAM1 | Range 1 | Wakeup clock HSI16 = 16 MHz | 2.8 | 3 |
|  |  | Range 2 | Wakeup clock HSI16 = 16 MHz | 2.9 | 3 |
| $t_{\text {WUSTOP1 }}$ | Wake up time from Stop 1 mode to Run in Flash | Range 1 | Wakeup clock HSI16 = 16 MHz | 9.5 | 9.8 |
|  |  | Range 2 | Wakeup clock HSI16 = 16 MHz | 21.9 | 22.7 |
|  | Wake up time from Stop 1 mode to Run mode in SRAM1 | Range 1 | Wakeup clock HSI16 = 16 MHz | 6.6 | 6.9 |
|  |  | Range 2 | Wakeup clock HSI16 = 16 MHz | 6.4 | 6.6 |
|  | Wake up time from Stop 1 mode to Low-power run mode in Flash | Regulator in low-power mode (LPR=1 in PWR_CR1) | Wakeup clock HSI16 = 16 MHz, with HPRE $=8$ | 26.1 | $27.1^{(2)}$ |
|  | Wake up time from Stop 1 mode to Low-power run mode in SRAM1 |  |  | 14.4 | $15^{(2)}$ |
| $t_{\text {WUSTBY }}$ | Wakeup time from Standby mode to Run mode | Range 1 | Wakeup clock HSI16 = 16 MHz | 29.7 | 33.8 |
| $t_{\text {WUSTBY }}$ <br> SRAM2 | Wakeup time from Standby with SRAM2 to Run mode | Range 1 | Wakeup clock HSI16 = 16 MHz | 29.7 | 33.5 |
| $t_{\text {WUSHDN }}$ | Wakeup time from Shutdown mode to Run mode | Range 1 | Wakeup clock HSI16 = 16 MHz | 267.9 | $274.6^{(2)}$ |
| $t_{\text {WULPRUN }}$ | Wakeup time from Lowpower run mode to Run mode ${ }^{(3)}$ | Wakeup clock HSI16 = 16 MHz HPRE $=8$ | 5 | 7 |  |

1. Guaranteed by characterization results.
2. Characterization results for temperature range from $0^{\circ} \mathrm{C}$ to $125^{\circ} \mathrm{C}$.
3. Time until REGLPF flag is cleared in PWR_SR2.Table 36. Regulator modes transition times ${ }^{(1)}$

| Symbol | Parameter | Conditions | Typ | Max | Unit |
| :-- | :-- | :-- | :--: | :--: | :--: |
| $\mathrm{t}_{\text {VOST }}$ | Regulator transition time from Range <br> 2 to Range 1 or <br> Range 1 to Range $2^{(2)}$ | Wakeup clock HSI16 = 16 MHz <br> HPRE $=8$ | 20 | 40 | $\mu \mathrm{s}$ |

1. Guaranteed by characterization results.
2. Time until VOSF flag is cleared in PWR_SR2.

Table 37. Wakeup time using USART/LPUART ${ }^{(1)}$

| Symbol | Parameter | Conditions | Typ | Max | Unit |
| :-- | :-- | :-- | :--: | :--: | :--: |
|  | Wakeup time needed to calculate the <br> maximum USART/LPUART baudrate <br> allowing to wakeup up from stop mode <br> when USART/LPUART clock source is <br> HSI16 | Stop 0 mode | - | 1.7 |  |
|  | Stop 1 mode | - | 8.5 |  |  |

1. Guaranteed by design.

# 5.3.7 External clock source characteristics 

## High-speed external user clock generated from an external source

In bypass mode the HSE oscillator is switched off and the input pin is a standard GPIO.
The external clock signal has to respect the I/O characteristics in Section 5.3.14. However, the recommended clock input waveform is shown in Figure 19: High-speed external clock source AC timing diagram.

Table 38. High-speed external user clock characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | User external clock <br> source frequency | Voltage scaling <br> Range 1 | - | 8 | 48 | MHz |
|  |  | Voltage scaling <br> Range 2 | - | 8 | 26 |  |
|  | OSC_IN input pin high <br> level voltage | - | $0.7 \mathrm{~V}_{\mathrm{DD}}$ | - | $V_{D D}$ | V |
|  | OSC_IN input pin low <br> level voltage | - | $V_{S S}$ | - | $0.3 \mathrm{~V}_{\mathrm{DD}}$ |  |
|  | OSC_IN high or low time | Voltage scaling <br> Range 1 | 7 | - | - | ns |
|  |  | Voltage scaling <br> Range 2 | 18 | - | - |  |

1. Guaranteed by design.Figure 19. High-speed external clock source AC timing diagram
![img-19.jpeg](img-19.jpeg)

# Low-speed external user clock generated from an external source 

In bypass mode the LSE oscillator is switched off and the input pin is a standard GPIO.
The external clock signal has to respect the I/O characteristics in Section 5.3.14. However, the recommended clock input waveform is shown in Figure 20.

Table 39. Low-speed external user clock characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{f}_{\text {LSE_ext }}$ | User external clock source <br> frequency | - | - | 32.768 | 1000 | kHz |
| $\mathrm{V}_{\text {LSEH }}$ | OSC32_IN input pin high <br> level voltage | - | $0.7 \mathrm{~V}_{\mathrm{DD}}$ | - | $\mathrm{V}_{\mathrm{DD}}$ | V |
| $\mathrm{V}_{\text {LSEL }}$ | OSC32_IN input pin low level <br> voltage | - | $\mathrm{V}_{\mathrm{SS}}$ | - | $0.3 \mathrm{~V}_{\mathrm{DD}}$ |  |
| $\mathrm{t}_{\mathrm{w}(\mathrm{LSEH})}$ | OSC32_IN high or low time | - | 250 | - | - | ns |

1. Guaranteed by design.

Figure 20. Low-speed external clock source AC timing diagram
![img-20.jpeg](img-20.jpeg)# High-speed external clock generated from a crystal/ceramic resonator 

The high-speed external (HSE) clock can be supplied with a 4 to 48 MHz crystal/ceramic resonator oscillator. All the information given in this paragraph are based on design simulation results obtained with typical external components specified in Table 40. In the application, the resonator and the load capacitors have to be placed as close as possible to the oscillator pins in order to minimize output distortion and startup stabilization time. Refer to the crystal resonator manufacturer for more details on the resonator characteristics (frequency, package, accuracy).

Table 40. HSE oscillator characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions ${ }^{(2)}$ | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {OSC_IN }}$ | Oscillator frequency | - | 4 | 8 | 48 | MHz |
| $R_{F}$ | Feedback resistor | - | - | 200 | - | $\mathrm{k} \Omega$ |
| $\mathrm{I}_{\text {DD(HSE) }}$ | HSE current consumption | During startup ${ }^{(3)}$ | - | - | 5.5 | mA |
|  |  | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V}, \\ & \mathrm{Rm}=30 \Omega, \\ & \mathrm{CL}=10 \mathrm{pF} @ 8 \mathrm{MHz} \end{aligned}$ | - | 0.44 | - |  |
|  |  | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V}, \\ & \mathrm{Rm}=45 \Omega, \\ & \mathrm{CL}=10 \mathrm{pF} @ 8 \mathrm{MHz} \end{aligned}$ | - | 0.45 | - |  |
|  |  | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V}, \\ & \mathrm{Rm}=30 \Omega, \\ & \mathrm{CL}=5 \mathrm{pF} @ 48 \mathrm{MHz} \end{aligned}$ | - | 0.68 | - |  |
|  |  | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V}, \\ & \mathrm{Rm}=30 \Omega, \\ & \mathrm{CL}=10 \mathrm{pF} @ 48 \mathrm{MHz} \end{aligned}$ | - | 0.94 | - |  |
|  |  | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V}, \\ & \mathrm{Rm}=30 \Omega, \\ & \mathrm{CL}=20 \mathrm{pF} @ 48 \mathrm{MHz} \end{aligned}$ | - | 1.77 | - |  |
| $G_{m}$ | Maximum critical crystal transconductance | Startup | - | - | 1.5 | $\mathrm{mA} / \mathrm{V}$ |
| $\mathrm{t}_{\text {SU(HSE) }}{ }^{(4)}$ | Startup time | $V_{D D}$ is stabilized | - | 2 | - | ms |

1. Guaranteed by design.
2. Resonator characteristics given by the crystal/ceramic resonator manufacturer.
3. This consumption level occurs during the first $2 / 3$ of the $\mathrm{t}_{\mathrm{SU}(\mathrm{HSE})}$ startup time
4. $\mathrm{t}_{\mathrm{SU}(\mathrm{HSE})}$ is the startup time measured from the moment it is enabled (by software) to a stabilized 8 MHz oscillation is reached. This value is measured for a standard crystal resonator and it can vary significantly with the crystal manufacturer

For $C_{L 1}$ and $C_{L 2}$, it is recommended to use high-quality external ceramic capacitors in the 5 pF to 20 pF range (typ.), designed for high-frequency applications, and selected to match the requirements of the crystal or resonator (see Figure 21). $\mathrm{C}_{\mathrm{L} 1}$ and $\mathrm{C}_{\mathrm{L} 2}$ are usually the same size. The crystal manufacturer typically specifies a load capacitance which is the series combination of $C_{L 1}$ and $C_{L 2}$. PCB and MCU pin capacitance must be included ( 10 pF can be used as a rough estimate of the combined pin and board capacitance) when sizing $\mathrm{C}_{\mathrm{L} 1}$ and $\mathrm{C}_{\mathrm{L} 2}$.Note: $\quad$ For information on selecting the crystal, refer to the application note AN2867 "Oscillator design guide for ST microcontrollers" available from the ST website www.st.com.

Figure 21. Typical application with an 8 MHz crystal
![img-21.jpeg](img-21.jpeg)

1. $\mathrm{R}_{\text {EXT }}$ value depends on the crystal characteristics.

# Low-speed external clock generated from a crystal resonator 

The low-speed external (LSE) clock can be supplied with a 32.768 kHz crystal resonator oscillator. All the information given in this paragraph are based on design simulation results obtained with typical external components specified in Table 41. In the application, the resonator and the load capacitors have to be placed as close as possible to the oscillator pins in order to minimize output distortion and startup stabilization time. Refer to the crystal resonator manufacturer for more details on the resonator characteristics (frequency, package, accuracy).Table 41. LSE oscillator characteristics $\left(f_{L S E}=32.768 \mathrm{kHz}\right)^{(1)}$

| Symbol | Parameter | Conditions ${ }^{(2)}$ | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{I}_{\text {DD(LSE) }}$ | LSE current consumption | LSEDRV[1:0] = 00 <br> Low drive capability | - | 250 | - | nA |
|  |  | LSEDRV[1:0] = 01 <br> Medium low drive capability | - | 315 | - |  |
|  |  | LSEDRV[1:0] = 10 <br> Medium high drive capability | - | 500 | - |  |
|  |  | LSEDRV[1:0] = 11 <br> High drive capability | - | 630 | - |  |
| $\mathrm{Gm}_{\text {critmax }}$ | Maximum critical crystal gm | LSEDRV[1:0] = 00 <br> Low drive capability | - | - | 0.5 | $\mu \mathrm{AV}$ |
|  |  | LSEDRV[1:0] = 01 <br> Medium low drive capability | - | - | 0.75 |  |
|  |  | LSEDRV[1:0] = 10 <br> Medium high drive capability | - | - | 1.7 |  |
|  |  | LSEDRV[1:0] = 11 <br> High drive capability | - | - | 2.7 |  |
| $\mathrm{f}_{\text {SU(LSE }}{ }^{(3)}$ | Startup time | $\mathrm{V}_{\mathrm{DD}}$ is stabilized | - | 2 | - | s |

1. Guaranteed by design.
2. Refer to the note and caution paragraphs below the table, and to the application note AN2867 "Oscillator design guide for ST microcontrollers".
3. $\mathrm{t}_{\text {SU(LSE) }}$ is the startup time measured from the moment it is enabled (by software) to a stabilized 32.768 kHz oscillation is reached. This value is measured for a standard crystal and it can vary significantly with the crystal manufacturer

Note: For information on selecting the crystal, refer to the application note AN2867 "Oscillator design guide for ST microcontrollers" available from the ST website www.st.com.

Figure 22. Typical application with a 32.768 kHz crystal
![img-22.jpeg](img-22.jpeg)

Note: An external resistor is not required between OSC32_IN and OSC32_OUT and it is forbidden to add one.# 5.3.8 Internal clock source characteristics 

The parameters given in Table 42 are derived from tests performed under ambient temperature and supply voltage conditions summarized in Table 17: General operating conditions. The provided curves are characterization results, not tested in production.

High-speed internal (HSI16) RC oscillator
Table 42. HSI16 oscillator characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {HSI16 }}$ | HSI16 Frequency | $\mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=30^{\circ} \mathrm{C}$ | 15.88 | - | 16.08 | MHz |
| TRIM | HSI16 user trimming step | Trimming code is not a multiple of 64 | 0.2 | 0.3 | 0.4 | \% |
|  |  | Trimming code is a multiple of 64 | $-4$ | $-6$ | $-8$ |  |
| $\operatorname{DuCy(HSI16)}^{(2)}$ | Duty Cycle | - | 45 | - | 55 | \% |
| $\Delta_{\text {Temp }}(\mathrm{HSI16)}$ | HSI16 oscillator frequency drift over temperature | $\mathrm{T}_{\mathrm{A}}=0$ to $85^{\circ} \mathrm{C}$ | $-1$ | - | 1 | \% |
|  |  | $\mathrm{T}_{\mathrm{A}}=-40$ to $125^{\circ} \mathrm{C}$ | $-2$ | - | 1.5 | \% |
| $\Delta_{\text {VDD }}(\mathrm{HSI16)}$ | HSI16 oscillator frequency drift over $\mathrm{V}_{\mathrm{DD}}$ | $\mathrm{V}_{\mathrm{DD}}=1.62 \mathrm{~V}$ to 3.6 V | $-0.1$ | - | 0.05 | \% |
| $t_{\text {su }}(\mathrm{HSI16)}^{(2)}$ | HSI16 oscillator start-up time | - | - | 0.8 | 1.2 | $\mu \mathrm{s}$ |
| $t_{\text {stab }}(\mathrm{HSI16)}^{(2)}$ | HSI16 oscillator stabilization time | - | - | 3 | 5 | $\mu \mathrm{s}$ |
| $I_{D D}(H S I 16)^{(2)}$ | HSI16 oscillator power consumption | - | - | 155 | 190 | $\mu \mathrm{A}$ |

1. Guaranteed by characterization results.
2. Guaranteed by design.Figure 23. HSI16 frequency versus temperature
![img-23.jpeg](img-23.jpeg)

High-speed internal 48 MHz (HSI48) RC oscillator
Table 43. HSI48 oscillator characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {HSI48 }}$ | HSI48 Frequency | $\mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=30^{\circ} \mathrm{C}$ | - | 48 | - | MHz |
| TRIM | HSI48 user trimming step | - | - | $0.11^{(2)}$ | $0.18^{(2)}$ | \% |
| USER TRIM <br> COVERAGE | HSI48 user trimming coverage | $\pm 32$ steps | $\pm 3^{(3)}$ | $\pm 3.5^{(3)}$ | - | \% |
| DuCy(HSI48) | Duty Cycle | - | $45^{(2)}$ | - | $55^{(2)}$ | \% |
| $\mathrm{ACC}_{\text {HSI48_REL }}$ | Accuracy of the HSI48 oscillator over temperature (factory calibrated) | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V} \text { to } 3.6 \mathrm{~V}, \\ & \mathrm{~T}_{\mathrm{A}}=-15 \text { to } 85^{\circ} \mathrm{C} \end{aligned}$ | - | - | $\pm 3^{(3)}$ | \% |
|  |  | $\mathrm{V}_{\mathrm{DD}}=1.65 \mathrm{~V}$ to 3.6 V , <br> $\mathrm{T}_{\mathrm{A}}=-40$ to $125^{\circ} \mathrm{C}$ | - | - | $\pm 4.5^{(3)}$ |  |
| $\mathrm{D}_{\text {VDD }}(\mathrm{HSI} 48)$ | HSI48 oscillator frequency drift with $\mathrm{V}_{\mathrm{DD}}$ | $\mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V}$ to 3.6 V | - | $0.025^{(3)}$ | $0.05^{(3)}$ | \% |
|  |  | $\mathrm{V}_{\mathrm{DD}}=1.65 \mathrm{~V}$ to 3.6 V | - | $0.05^{(3)}$ | $0.1^{(3)}$ |  |
| $t_{\text {su }}(\mathrm{HSI} 48)$ | HSI48 oscillator start-up time | - | - | $2.5^{(2)}$ | $6^{(2)}$ | $\mu \mathrm{s}$ |
| $I_{D D}(H S I 48)$ | HSI48 oscillator power consumption | - | - | $340^{(2)}$ | $380^{(2)}$ | $\mu \mathrm{A}$ |Table 43. HSI48 oscillator characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{N}_{\mathrm{T}}$ jitter | Next transition jitter <br> Accumulated jitter on 28 <br> cycles $^{(4)}$ | - | - | $+/-0.15^{(2)}$ | - | ns |
| $\mathrm{P}_{\mathrm{T}}$ jitter | Paired transition jitter <br> Accumulated jitter on 56 <br> cycles $^{(4)}$ | - | - | $+/-0.25^{(2)}$ | - | ns |

1. $\mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=-40$ to $125^{\circ} \mathrm{C}$ unless otherwise specified.
2. Guaranteed by design.
3. Guaranteed by characterization results.
4. Jitter measurement are performed without clock source activated in parallel.

Figure 24. HSI48 frequency versus temperature
![img-24.jpeg](img-24.jpeg)

# Low-speed internal (LSI) RC oscillator 

Table 44. LSI oscillator characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{L S I}$ | LSI Frequency | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V}, \\ & \mathrm{~T}_{\mathrm{A}}=30^{\circ} \mathrm{C} \end{aligned}$ | 31.04 | - | 32.96 | kHz |
|  |  | $\mathrm{V}_{\mathrm{DD}}=1.62$ to $3.6 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=-40$ to $125^{\circ} \mathrm{C}$ | 29.5 | - | 34 |  |
| $\mathrm{t}_{\mathrm{SU}}(\mathrm{LSI})^{(2)}$ | LSI oscillator start-up time | - | - | 80 | 130 | $\mu \mathrm{s}$ |Table 44. LSI oscillator characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{t}_{\text {STAB }}(\mathrm{LSI})^{(\mathrm{2})}$ | LSI oscillator stabilization time | $5 \%$ of final frequency | - | 125 | 180 | $\mu \mathrm{s}$ |
| $\mathrm{I}_{\mathrm{DD}}(\mathrm{LSI})^{(\mathrm{2})}$ | LSI oscillator power consumption | - | - | 110 | 180 | nA |

1. Guaranteed by characterization results.
2. Guaranteed by design.

# 5.3.9 PLL characteristics 

The parameters given in Table 45 are derived from tests performed under temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 17: General operating conditions.

Table 45. PLL characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {PLL_IN }}$ | PLL input clock ${ }^{(2)}$ | - | 2.66 | - | 16 | MHz |
|  | PLL input clock duty cycle | - | 45 | - | 55 | $\%$ |
| $f_{\text {PLL_P_OUT }}$ | PLL multiplier output clock $P$ | Voltage scaling Range 1 Boost mode | 2.0645 | - | 170 | MHz |
|  |  | Voltage scaling Range 1 | 2.0645 | - | 150 |  |
|  |  | Voltage scaling Range 2 | 2.0645 | - | 26 |  |
| $f_{\text {PLL_Q_OUT }}$ | PLL multiplier output clock $Q$ | Voltage scaling Range 1 Boost mode | 8 | - | 170 |  |
|  |  | Voltage scaling Range 1 | 8 | - | 150 |  |
|  |  | Voltage scaling Range 2 | 8 | - | 26 |  |
| $f_{\text {PLL_R_OUT }}$ | PLL multiplier output clock $R$ | Voltage scaling Range 1 Boost mode | 8 | - | 170 |  |
|  |  | Voltage scaling Range 1 | 8 | - | 150 |  |
|  |  | Voltage scaling Range 2 | 8 | - | 26 |  |
| $f_{\text {VCO_OUT }}$ | PLL VCO output | Voltage scaling Range 1 | 96 | - | 344 |  |
|  |  | Voltage scaling Range 2 | 96 | - | 128 |  |
| $t_{\text {LOCK }}$ | PLL lock time | - | - | 15 | 40 | $\mu \mathrm{s}$ |
| Jitter | RMS cycle-to-cycle jitter | System clock 150 MHz | - | 28.6 | - | $\pm \mathrm{ps}$ |
|  | RMS period jitter |  | - | 21.4 | - |  |
| $\mathrm{I}_{\mathrm{DD}}(\mathrm{PLL})$ | PLL power consumption on $\mathrm{V}_{\mathrm{DD}}{ }^{(1)}$ | VCO freq $=96 \mathrm{MHz}$ | - | 200 | 260 | $\mu \mathrm{A}$ |
|  |  | VCO freq $=192 \mathrm{MHz}$ | - | 300 | 380 |  |
|  |  | VCO freq $=344 \mathrm{MHz}$ | - | 520 | 650 |  |

1. Guaranteed by design.
2. Take care of using the appropriate division factor $M$ to obtain the specified PLL input clock values.# 5.3.10 Flash memory characteristics 

Table 46. Flash memory characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {prog }}$ | 64-bit programming time | - | 81.7 | 83.35 | $\mu \mathrm{s}$ |
| $\mathrm{t}_{\text {prog_row }}$ | One row (32 double word) programming time | Normal programming | 2.61 | 2.7 | ms |
|  |  | Fast programming | 1.91 | 1.95 |  |
| $\mathrm{t}_{\text {prog_page }}$ | One page (2 Kbytes) programming time | Normal programming | 20.91 | 21.34 |  |
|  |  | Fast programming | 15.29 | 15.6 |  |
| ${ }_{\text {ERASE }}$ | Page (2 Kbytes) erase time | - | 22.02 | 24.47 |  |
| ${ }_{\text {prog_bank }}$ | One bank (128 Kbyte) programming time | Normal programming | 1.34 | 1.49 | s |
|  |  | Fast programming | 0.98 | 1.09 |  |
| $t_{M E}$ | Mass erase time | - | 22.13 | 24.6 | ms |
| $t_{D D}$ | Average consumption from $\mathrm{V}_{\mathrm{DD}}$ | Write mode | 3.5 | - | mA |
|  |  | Erase mode | 3.5 | - |  |
|  | Maximum current (peak) | Write mode | 7 (for $6 \mu \mathrm{~s}$ ) | - |  |
|  |  | Erase mode | 7 (for $67 \mu \mathrm{~s}$ ) | - |  |

1. Guaranteed by design.

Table 47. Flash memory endurance and data retention

| Symbol | Parameter | Conditions | Min $^{(1)}$ | Unit |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{N}_{\text {END }}$ | Endurance | $\mathrm{T}_{\mathrm{A}}=-40$ to $+105^{\circ} \mathrm{C}$ | 10 | kcycles |
| $t_{\text {RET }}$ | Data retention | 1 kcycle $^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}$ | 30 | Years |
|  |  | 1 kcycle $^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=105^{\circ} \mathrm{C}$ | 15 |  |
|  |  | 1 kcycle $^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=125^{\circ} \mathrm{C}$ | 7 |  |
|  |  | 10 kcycles $^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=55^{\circ} \mathrm{C}$ | 30 |  |
|  |  | $10 \mathrm{kcycles}^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}$ | 15 |  |
|  |  | $10 \mathrm{kcycles}^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=105^{\circ} \mathrm{C}$ | 10 |  |

1. Guaranteed by characterization results.
2. Cycling performed over the whole temperature range.# 5.3.11 EMC characteristics 

Susceptibility tests are performed on a sample basis during device characterization.

## Functional EMS (electromagnetic susceptibility)

While a simple application is executed on the device (toggling 2 LEDs through I/O ports). the device is stressed by two electromagnetic events until a failure occurs. The failure is indicated by the LEDs:

- Electrostatic discharge (ESD) (positive and negative) is applied to all device pins until a functional disturbance occurs. This test is compliant with the IEC 61000-4-2 standard.
- FTB: A Burst of Fast Transient voltage (positive and negative) is applied to $\mathrm{V}_{\mathrm{DD}}$ and $\mathrm{V}_{\mathrm{SS}}$ through a 100 pF capacitor, until a functional disturbance occurs. This test is compliant with the IEC 61000-4-4 standard.

A device reset allows normal operations to be resumed.
The test results are given in Table 48. They are based on the EMS levels and classes defined in application note AN1709.

Table 48. EMS characteristics

| Symbol | Parameter | Conditions | Level/ <br> Class |
| :--: | :-- | :-- | :--: |
|  | Voltage limits to be applied on any I/O pin <br> to induce a functional disturbance | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}=3.3 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=+25^{\circ} \mathrm{C}, \\ & \mathrm{f}_{\mathrm{HCLK}}=170 \mathrm{MHz} \\ & \text { conforming to IEC 61000-4-2 } \end{aligned}$ | 2B |
|  | Fast transient voltage burst limits to be <br> applied through 100 pF on $\mathrm{V}_{\mathrm{DD}}$ and $\mathrm{V}_{\mathrm{SS}}$ <br> pins to induce a functional disturbance | $\mathrm{V}_{\mathrm{DD}}=3.3 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=+25^{\circ} \mathrm{C}$, <br> $\mathrm{f}_{\mathrm{HCLK}}=170 \mathrm{MHz}$, <br> conforming to IEC 61000-4-4 | 5A |

## Designing hardened software to avoid noise problems

EMC characterization and optimization are performed at component level with a typical application environment and simplified MCU software. It should be noted that good EMC performance is highly dependent on the user application and the software in particular.

Therefore it is recommended that the user applies EMC software optimization and prequalification tests in relation with the EMC level requested for his application.

Software recommendations
The software flowchart must include the management of runaway conditions such as:

- Corrupted program counter
- Unexpected reset
- Critical Data corruption (control registers...)

Prequalification trials
Most of the common failures (unexpected reset and program counter corruption) can be reproduced by manually forcing a low state on the NRST pin or the Oscillator pins for 1 second.To complete these trials, ESD stress can be applied directly on the device, over the range of specification values. When unexpected behavior is detected, the software can be hardened to prevent unrecoverable errors occurring (see application note AN1015).

# Electromagnetic Interference (EMI) 

The electromagnetic field emitted by the device are monitored while a simple application is executed (toggling 2 LEDs through the I/O ports). This emission test is compliant with IEC 61967-2 standard which specifies the test board and the pin loading.

Table 49. EMI characteristics

| Symbol | Parameter | Conditions | Monitored frequency band | Max vs. $\left[f_{\text {HSE }} / f_{\text {HCLK }}\right]$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | 8 MHz / 170 MHz |  |
| $S_{\text {EMI }}$ | Peak level | $\begin{aligned} & V_{D D}=3.6 \mathrm{~V}, T_{A}=25^{\circ} \mathrm{C} \\ & \text { LQFP100 package } \\ & \text { compliant with IEC 61967-2 } \end{aligned}$ | 0.1 MHz to 30 MHz | 3 | $\mathrm{dB} \mu \mathrm{V}$ |
|  |  |  | 30 MHz to 130 MHz | $-2$ |  |
|  |  |  | 130 MHz to 1 GHz | 25 |  |
|  |  |  | 1 GHz to 2 GHz | 18 |  |
|  |  |  | EMI Level | 4 | - |

### 5.3.12 Electrical sensitivity characteristics

Based on three different tests (ESD, LU) using specific measurement methods, the device is stressed in order to determine its performance in terms of electrical sensitivity.

## Electrostatic discharge (ESD)

Electrostatic discharges (a positive then a negative pulse separated by 1 second) are applied to the pins of each sample according to each pin combination. The sample size depends on the number of supply pins in the device ( 3 parts $\times(n+1)$ supply pins). This test conforms to the ANSI/JEDEC standard.

Table 50. ESD absolute maximum ratings

| Symbol | Ratings | Conditions | Class | Maximum <br> value $^{(1)}$ | Unit |
| :-- | :-- | :-- | :--: | :--: | :--: |
| $\mathrm{V}_{\text {ESD(HBM) }}$ | Electrostatic discharge <br> voltage (human body model) | $\mathrm{T}_{\mathrm{A}}=+25^{\circ} \mathrm{C}$, conforming to <br> ANSI/ESDA/JEDEC JS-001 | 2 | 2000 | V |
| $\mathrm{V}_{\text {ESD(CDM) }}$ | Electrostatic discharge <br> voltage (charge device model) | $\mathrm{T}_{\mathrm{A}}=+25^{\circ} \mathrm{C}$, conforming to <br> ANSI/ESDA/JEDEC JS- 002 | C 1 | 250 | V |

1. Guaranteed by characterization results.# Static latch-up 

Two complementary static tests are required on three parts to assess the latch-up performance:

- A supply overvoltage is applied to each power supply pin.
- A current injection is applied to each input, output and configurable I/O pin.

These tests are compliant with EIA/JESD 78E IC latch-up standard.
Table 51. Electrical sensitivities

| Symbol | Parameter | Conditions | Class |
| :--: | :--: | :--: | :--: |
| LU | Static latch-up class | TA $=+125^{\circ} \mathrm{C}$ conforming to JESD78E | Class II level A |

### 5.3.13 I/O current injection characteristics

As a general rule, current injection to the I/O pins, due to external voltage below $\mathrm{V}_{\mathrm{SS}}$ or above $\mathrm{V}_{\mathrm{DD}}$ (for standard, 3.3 V -capable I/O pins) should be avoided during normal product operation. However, in order to give an indication of the robustness of the microcontroller in cases when abnormal injection accidentally happens, susceptibility tests are performed on a sample basis during device characterization.

## Functional susceptibility to I/O current injection

While a simple application is executed on the device, the device is stressed by injecting current into the I/O pins programmed in floating input mode. While current is injected into the I/O pin, one at a time, the device is checked for functional failures.

The failure is indicated by an out of range parameter: ADC error above a certain limit (higher than 5 LSB TUE), out of conventional limits of induced leakage current on adjacent pins (out of the $-5 \mu \mathrm{~A} /+0 \mu \mathrm{~A}$ range) or other functional failure (for example reset occurrence or oscillator frequency deviation).
The characterization results are given in Table 52.
Negative induced leakage current is caused by negative injection and positive induced leakage current is caused by positive injection.

Table 52. I/O current injection susceptibility

| Symbol | Description |  | Functional susceptibility |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Negative injection | Positive injection |  |
| $I_{\text {IN }} \text { (1) }$ | Injected current on pin | All except TT_a, PF2 | $-5$ | NA | mA |
|  |  | PF2 | $-0$ | NA |  |
|  |  | TT_a pins | $-5$ | 0 |  |

1. Guaranteed by characterization.# 5.3.14 I/O port characteristics 

## General input/output characteristics

Unless otherwise specified, the parameters given in Table 53 are derived from tests performed under the conditions summarized in Table 17: General operating conditions. All I/Os are designed as CMOS- and TTL-compliant.

Table 53. I/O static characteristics

| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{I L}{ }^{(1)(2)}$ | I/O input low level voltage | All except FT_c | $1.62 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | - | - | $0.3 \times V_{D D}$ | V |
|  |  | FT_c | $1.62 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | - | - | $0.3 \times V_{D D}$ |  |
|  |  |  |  |  |  | $0.25 \times V_{D D}$ |  |
| $V_{I H}{ }^{(1)(2)}$ | I/O input high level voltage | All except FT_c | $1.62 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | $0.7 \times V_{D D}$ | - | - | V |
|  |  |  |  | $0.49 \times V_{D D}+0.26^{(3)}$ | - | - |  |
|  |  | FT_c | $1.62 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | $0.7 \times V_{D D}$ | - | - |  |
| $V_{\text {HYS }}{ }^{(3)}$ | Input hysteresis | TT_xx, <br> FT_xxx, <br> NRST | $1.62 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | - | 200 | - | mV |
| Ileak | Input leakage current ${ }^{(3)}$ | FT_xx <br> except <br> FT_c | $0<V_{\text {IN }} \leq V_{D D}$ | - | - | $\pm 100$ | nA |
|  |  |  | $V_{D D} \leq V_{\text {IN }} \leq V_{D D}+1 \mathrm{~V}$ | - | - | $650^{(4)}$ |  |
|  |  |  | $V_{D D}+1 \mathrm{~V}<\mathrm{V}_{\text {IN }} \leq 5.5 \mathrm{~V}$ | - | - | $200^{(4)}$ |  |
|  |  | FT_c | $0 \leq V_{\text {IN }} \leq V_{\text {DDMAX }}$ | - | - | 2000 |  |
|  |  |  | $V_{D D} \leq V_{\text {IN }}<0.5 \mathrm{~V}$ | - | - | 3000 |  |
|  |  | FT_u, PC3 | $0 \leq V_{\text {IN }} \leq V_{D D}$ | - | - | $\pm 150$ |  |
|  |  |  | $V_{D D} \leq V_{\text {IN }} \leq V_{D D}+1 \mathrm{~V}$ | - | - | $\pm 2500$ |  |
|  |  |  | $V_{D D} \leq V_{\text {IN }} \leq 5.5 \mathrm{~V}$ | - | - | $\pm 250$ |  |
|  |  | FT_d | $0 \leq V_{\text {IN }} \leq V_{D D}$ | - | - | $\pm 4500$ |  |
|  |  |  | $V_{D D}+1 \mathrm{~V} \leq V_{\text {IN }} \leq 5.5 \mathrm{~V}$ | - | - | $\pm 9000$ |  |
|  |  | TT_xx | $0 \leq V_{\text {IN }} \leq V_{D D}$ | - | - | $\pm 150$ |  |
|  |  |  | $V_{D D} \leq V_{\text {IN }} \leq 3.6 \mathrm{~V}$ | - | - | 2000 |  |
| $R_{P U}$ | Weak pullup equivalent resistor ${ }^{(5)}$ | $V_{\text {IN }}=V_{\text {SS }}$ |  | 25 | 40 | 55 | k 3 |
| $R_{P D}$ | Weak pulldown equivalent resistor ${ }^{(5)}$ | $V_{\text {IN }}=V_{D D}$ |  | 25 | 40 | 55 |  |
| $C_{\text {IO }}$ | I/O pin capacitance | I/O pin capacitance | - | - | 5 | - | pF |

1. Refer to Figure 25: I/O input characteristics2. Data based on characterization results, not tested in production
3. Guaranteed by design.
4. This value represents the pad leakage of the I/O itself. The total product pad leakage is provided by this formula: $\mathrm{I}_{\text {Total_ Ileak_max }}=10 \mu \mathrm{~A}+[$ number of I/Os where VIN is applied on the pad $]_{+} \mathrm{I}_{\text {Ikg }}($ Max $)$.
5. Pull-up and pull-down resistors are designed with a true resistance in series with a switchable PMOS/NMOS. This PMOS/NMOS contribution to the series resistance is minimal ( $\sim 10 \%$ order).

Note: $\quad$ For more information about GPIO properties, refer to the application note AN4899 "STM32 GPIO configuration for hardware settings and low-power consumption" available from the ST website www.st.com.

All I/Os are CMOS- and TTL-compliant (no software configuration required). Their characteristics cover more than the strict CMOS-technology or TTL parameters. The coverage of these requirements is shown in Figure 25 for standard I/Os, and 5 V tolerant I/Os (except FT_c).

Figure 25. I/O input characteristics
![img-25.jpeg](img-25.jpeg)

# Output driving current 

The GPIOs (general purpose input/outputs) can sink or source up to $\pm 8 \mathrm{~mA}$, and sink or source up to $\pm 20 \mathrm{~mA}$ (with a relaxed $\mathrm{V}_{\mathrm{OL}} / \mathrm{V}_{\mathrm{OH}}$ ).In the user application, the number of I/O pins which can drive current must be limited to respect the absolute maximum rating specified in Section 5.2:

- The sum of the currents sourced by all the I/Os on $\mathrm{V}_{\mathrm{DD}}$ plus the maximum consumption of the MCU sourced on $\mathrm{V}_{\mathrm{DD}}$, cannot exceed the absolute maximum rating $\Sigma \mathrm{I}_{\mathrm{VDD}}$ (see Table 14: Voltage characteristics).
- The sum of the currents sunk by all the I/Os on $\mathrm{V}_{\mathrm{SS}}$, plus the maximum consumption of the MCU sunk on $\mathrm{V}_{\mathrm{SS}}$, cannot exceed the absolute maximum rating $\Sigma \mathrm{I}_{\mathrm{VSS}}$ (see Table 14: Voltage characteristics).


# Output voltage levels 

Unless otherwise specified, the parameters given in the table below are derived from tests performed under the ambient temperature and supply voltage conditions summarized in Table 17: General operating conditions. All I/Os are CMOS- and TTL-compliant (FT OR TT unless otherwise specified).

Table 54. Output voltage characteristics ${ }^{(1)(2)}$

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{OL}}{ }^{(3)}$ | Output low level voltage for an I/O pin | CMOS port | - | 0.4 | V |
| $\mathrm{V}_{\mathrm{OH}}{ }^{(3)}$ | Output high level voltage for an I/O pin | $\left\|\mathrm{I}_{\mathrm{IO}}\right\|=2 \mathrm{~mA}$ for $\mathrm{FT} \_$c <br> $\mathrm{I} / \mathrm{Os}=8 \mathrm{~mA}$ for other I/Os $\mathrm{V}_{\mathrm{DD}}$ $\geq 2.7 \mathrm{~V}$ | $\mathrm{V}_{\mathrm{DD}}-0.4$ | - |  |
| $\mathrm{V}_{\mathrm{OL}}{ }^{(3)}$ | Output low level voltage for an I/O pin | TTL port | - | 0.4 |  |
| $\mathrm{V}_{\mathrm{OH}}{ }^{(3)}$ | Output high level voltage for an I/O pin | $\left\|\mathrm{I}_{\mathrm{IO}}\right\|=2 \mathrm{~mA}$ for $\mathrm{FT} \_$c <br> $\mathrm{I} / \mathrm{Os}=8 \mathrm{~mA}$ for other I/Os $\mathrm{V}_{\mathrm{DD}} \geq 2.7 \mathrm{~V}$ | 2.4 | - |  |
| $\mathrm{V}_{\mathrm{OL}}{ }^{(3)}$ | Output low level voltage for an I/O pin | All I/Os except FT_c | - | 1.3 |  |
| $\mathrm{V}_{\mathrm{OH}}{ }^{(3)}$ | Output high level voltage for an I/O pin | $\left\|\mathrm{I}_{\mathrm{IO}}\right\|=20 \mathrm{~mA}$ <br> $\mathrm{V}_{\mathrm{DD}} \geq 2.7 \mathrm{~V}$ | $\mathrm{V}_{\mathrm{DD}}-1.3$ | - |  |
| $\mathrm{V}_{\mathrm{OL}}{ }^{(3)}$ | Output low level voltage for an I/O pin | $\left\|\mathrm{I}_{\mathrm{IO}}\right\|=1 \mathrm{~mA}$ for $\mathrm{FT} \_$c <br> $\mathrm{I} / \mathrm{Os}=4 \mathrm{~mA}$ for other I/Os $\mathrm{V}_{\mathrm{DD}} \geq 1.62 \mathrm{~V}$ | - | 0.4 |  |
| $\mathrm{V}_{\mathrm{OH}}{ }^{(3)}$ | Output high level voltage for an I/O pin | $\mathrm{V}_{\mathrm{DD}}-0.45$ | - |  |  |
| $\mathrm{V}_{\mathrm{OL}}\left(\mathrm{M}^{+}\right.$ <br> (3) | Output low level voltage for an FT I/O pin in FM+ mode (FT I/O with "f" option) | $\left\|\mathrm{I}_{\mathrm{IO}}\right\|=20 \mathrm{~mA}$ <br> $\mathrm{V}_{\mathrm{DD}} \geq 2.7 \mathrm{~V}$ | - | 0.4 |  |
|  |  | $\left\|\mathrm{I}_{\mathrm{IO}}\right\|=10 \mathrm{~mA}$ <br> $\mathrm{V}_{\mathrm{DD}} \geq 1.62 \mathrm{~V}$ | - | 0.4 |  |

1. The $I_{I O}$ current sourced or sunk by the device must always respect the absolute maximum rating specified in Table 14: Voltage characteristics, and the sum of the currents sourced or sunk by all the I/Os (I/O ports and control pins) must always respect the absolute maximum ratings $\Sigma \mathrm{I}_{\mathrm{IO}}$.
2. TTL and CMOS outputs are compatible with JEDEC standards JESD36 and JESD52.
3. Guaranteed by design.

## Input/output AC characteristics

The definition and values of input/output AC characteristics are given in Figure 26 and Table 55, respectively.

Unless otherwise specified, the parameters given are derived from tests performed under the ambient temperature and supply voltage conditions summarized in Table 17: General operating conditions.Table 55. I/O (except FT_c) AC characteristics ${ }^{(1)(2)}$

| Speed | Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| 00 | Fmax | Maximum frequency | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 5 | MHz |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 1 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 10 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 1.5 |
|  | $\operatorname{Tr} / \operatorname{Tf}$ | Output rise and fall time | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 25 | ns |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 52 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 17 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 37 |
| 01 | Fmax | Maximum frequency | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 25 | MHz |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 10 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 50 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 15 |
|  | $\operatorname{Tr} / \operatorname{Tf}$ | Output rise and fall time | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 9 | ns |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 16 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 4.5 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 9 |
| 10 | Fmax | Maximum frequency | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 50 | MHz |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 25 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | $100^{(3)}$ |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 37.5 |
|  | $\operatorname{Tr} / \operatorname{Tf}$ | Output rise and fall time | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 5.8 | ns |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 11 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 2.5 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 5 |
| 11 | Fmax | Maximum frequency | $\mathrm{C}=30 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | $120^{(3)}$ | MHz |
|  |  |  | $\mathrm{C}=30 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 50 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | $180^{(3)}$ |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 75 |
|  | $\operatorname{Tr} / \operatorname{Tf}$ | Output rise and fall time ${ }^{(4)}$ | $\mathrm{C}=30 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 3.3 | ns |
|  |  |  | $\mathrm{C}=30 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 6 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 2.7 \mathrm{VsV}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 1.7 |
|  |  |  | $\mathrm{C}=10 \mathrm{pF}, 1.62 \mathrm{VsV}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 3.3 |Table 55. I/O (except FT_c) AC characteristics ${ }^{(1)(2)}$ (continued)

| Speed | Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| FM+ | Fmax $^{(5)}$ | Maximum <br> frequency | $C=50 \mathrm{pF}, 1.6 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 1 | MHz |
|  | $\operatorname{Tr} / \mathrm{TF}^{(4)}$ | Output high to <br> low level fall <br> time |  | - | 5 | ns |

1. The I/O speed is configured using the OSPEEDRy[1:0] bits. The Fm+ mode is configured in the SYSCFG_CFGR1 register. Refer to the reference manual RM0440 "STM32G4 Series advanced Arm ${ }^{\circledR}$ based 32-bit MCUs" for a description of GPIO Port configuration register.
2. Guaranteed by design.
3. This value represented the I/O capability but maximum system frequency is 170 MHz .
4. The fall time is defined between $70 \%$ and $30 \%$ of the output waveform accordingly to I2C specification.
5. The maximum frequency is defined with the following conditions:

- $(\mathrm{Tr}+\mathrm{Tf}) \leq 2 / 3 \mathrm{~T}$.
$-45 \%<$ Duty cycle $<55 \%$

Table 56. I/O FT_c AC characteristics ${ }^{(1)(2)}$

| Speed | Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| 0 | Fmax | Maximum frequency | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 2 | MHz |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.6 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 1 |  |
|  | Tr/Tf | Output H/L to <br> L/H level fall <br> time | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 170 | ns |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.6 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 330 |  |
| 1 | Fmax | Maximum frequency | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 10 | MHz |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.6 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 5 |  |
|  | Tr/Tf | Output H/L to <br> L/H level fall <br> time | $\mathrm{C}=50 \mathrm{pF}, 2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 35 | ns |
|  |  |  | $\mathrm{C}=50 \mathrm{pF}, 1.6 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | 65 |  |

1. The I/O speed is configured using the OSPEEDRy[1:0] bits. The Fm+ mode is configured in the SYSCFG_CFGR1 register. Refer to the reference manual RM0440 "STM32G4 Series advanced Arm ${ }^{\circledR}$ based 32-bit MCUs" for a description of GPIO Port configuration register.
2. Guaranteed by design.Figure 26. I/O AC characteristics definition ${ }^{(1)}$
![img-26.jpeg](img-26.jpeg)

1. Refer to Table 55: I/O (except FT_c) AC characteristics.

# 5.3.15 NRST pin characteristics 

The NRST pin input driver uses the CMOS technology. It is connected to a permanent pullup resistor, $\mathrm{R}_{\mathrm{PU}}$.

Unless otherwise specified, the parameters given in the table below are derived from tests performed under the ambient temperature and supply voltage conditions summarized in Table 17: General operating conditions.

Table 57. NRST pin characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{\text {IL(NRST) }}$ | NRST input low level voltage | - | - | - | $0.3_{v} V_{\text {DD }}$ | V |
| $V_{\text {IH(NRST) }}$ | NRST input high level voltage | - | $0.7_{v} V_{D D}$ | - | - |  |
| $V_{\text {hys(NRST) }}$ | NRST Schmitt trigger voltage hysteresis | - | - | 200 | - | mV |
| $R_{P U}$ | Weak pull-up equivalent resistor ${ }^{(2)}$ | $V_{\text {IN }}=V_{S S}$ | 25 | 40 | 55 | $\mathrm{k} \Omega$ |
| $V_{F(\text { NRST) }}$ | NRST input filtered pulse | - | - | - | 70 | ns |
| $V_{\text {NF(NRST) }}$ | NRST input not filtered pulse | $\begin{gathered} 1.71 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \\ \leq 3.6 \mathrm{~V} \end{gathered}$ | 350 | - | - | ns |

1. Guaranteed by design.
2. The pull-up is designed with a true resistance in series with a switchable PMOS. This PMOS contribution to the series resistance is minimal ( $\sim 10 \%$ order).Figure 27. Recommended NRST pin protection
![img-27.jpeg](img-27.jpeg)

1. The reset network protects the device against parasitic resets.
2. The user must ensure that the level on the NRST pin can go below the $\mathrm{V}_{\mathrm{IL} \text { (NRST) }}$ max level specified in Table 57: NRST pin characteristics. Otherwise the reset is not taken into account by the device.
3. The external capacitor on NRST must be placed as close as possible to the device.

# 5.3.16 Extended interrupt and event controller input (EXTI) characteristics 

The pulse on the interrupt input must have a minimal length in order to guarantee that it is detected by the event controller.

Table 58. EXTI input characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| PLEC | Pulse length to event <br> controller | - | 20 | - | - | ns |

1. Guaranteed by design.

### 5.3.17 Analog switches booster

Table 59. Analog switches booster characteristics ${ }^{(1)}$

| Symbol | Parameter | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{DD}}$ | Supply voltage | 1.62 | - | 3.6 | V |
| $\mathrm{t}_{\mathrm{SU}(\text { BOOST) }}$ | Booster startup time | - | - | 240 | $\mu \mathrm{s}$ |
| $\mathrm{I}_{\mathrm{DD}(\text { BOOST) }}$ | Booster consumption for <br> $1.62 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 2.0 \mathrm{~V}$ | - | - | 250 | $\mu \mathrm{A}$ |
|  | Booster consumption for <br> $2.0 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 2.7 \mathrm{~V}$ | - | - | 500 |  |
|  | Booster consumption for <br> $2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | - | 900 |  |

1. Guaranteed by design.# 5.3.18 Analog-to-digital converter characteristics 

Unless otherwise specified, the parameters given in Table 60 are preliminary values derived from tests performed under ambient temperature, $\mathrm{f}_{\mathrm{PCLK}}$ frequency and $\mathrm{V}_{\mathrm{DDA}}$ supply voltage conditions summarized in Table 17: General operating conditions.

Note: It is recommended to perform a calibration after each power-up.
Table 60. ADC characteristics ${ }^{(1)(2)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{\text {DDA }}$ | Analog supply voltage | - | 1.62 | - | 3.6 | V |
| $V_{\text {REF }+}$ | Positive reference voltage | $\mathrm{V}_{\mathrm{DDA}} \geq 2 \mathrm{~V}$ | 2 | - | $V_{\text {DDA }}$ | V |
|  |  | $V_{D D A}<2 \mathrm{~V}$ | $V_{\text {DDA }}$ |  |  | V |
| $V_{\text {REF }}$. | Negative reference voltage | - | $V_{\text {SSA }}$ |  |  | V |
| $V_{\text {CMIN }}$ | Input common mode | Differential | $\begin{aligned} & \left(V_{\text {REF }+}+V_{\text {REF. }}\right) / 2-0.18 \\ & ) / 2-0.18 \end{aligned}$ | $\begin{aligned} & \left(V_{\text {REF }+}+ \\ & V_{\text {REF. }}\right) / 2 \end{aligned}$ | $\begin{aligned} & \left(V_{\text {REF }+}+V_{\text {REF. }}\right) / 2+0.18 \\ & ) / 2+0.18 \end{aligned}$ | V |
| $f_{\text {ADC }}$ | ADC clock frequency | Range 1, single ADC operation | 0.14 | - | 60 | MHz |
|  |  | Range 2 | - | - | 26 |  |
|  |  | Range 1, all ADCs operation, single ended mode $V_{D D A} \geq 2.7 \mathrm{~V}$ | 0.14 | - | 52 |  |
|  |  | Range 1, all ADCs operation, single ended mode $V_{D D A} \geq 1.62 \mathrm{~V}$ | 0.14 | - | 42 |  |
|  |  | Range 1, all ADCs operation, differential mode $V_{D D A} \geq 1.62 \mathrm{~V}$ | 0.14 | - | 56 |  |
| $f_{s}$ | Sampling rate, continuous mode | For given resolution and sampling time cycles $\left(t_{s}\right)$ | 0.001 | $\begin{gathered} \mathrm{f}_{\text {ADC }} / \text { (sampling time } \\ \text { [cycles] }+ \text { resolution [bits] }+ \\ 0.5 \text { ) } \end{gathered}$ | Msps |
| $T_{\text {TRIG }}$ | External trigger period | Considering trigger conversion latency time ( $\mathrm{t}_{\mathrm{LATR}}$ or $\mathrm{t}_{\mathrm{LATRINJ}}$ ) | - | - | 1 ms | - |
|  |  | Resolution $=$ 12 bits, $\mathrm{f}_{\mathrm{ADC}=60 \mathrm{MHz}}$ | $\begin{aligned} & \text { tconv }+[\mathrm{t}_{\mathrm{LATR}} \\ & \text { or } \mathrm{t}_{\mathrm{LATRINJ}}] \end{aligned}$ | - |  |  |
| $V_{\text {AIN }}{ }^{(3)}$ | Conversion voltage range | - | 0 | - | $V_{\text {REF }+}$ | V |Table 60. ADC characteristics ${ }^{(1)(2)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{R}_{\text {AIN }}{ }^{(4)}$ | External input impedance | - | - | - | 50 | k $\Omega$ |
| $\mathrm{C}_{\text {ADC }}$ | Internal sample and hold capacitor | - | - | 5 | - | pF |
| $t_{\text {STAB }}$ | Power-up time | - | 1 |  |  | conversion cycle |
| $t_{\text {CAL }}$ | Calibration time | $\mathrm{f}_{\text {ADC }}=60 \mathrm{MHz}$ | 1.93 |  |  | $\mu \mathrm{s}$ |
|  |  | - | 116 |  |  | $1 / f_{\text {ADC }}$ |
| $t_{\text {LATR }}$ | Trigger conversion latency Regular and injected channels without conversion abort | CKMODE $=00$ | 1.5 | 2 | 2.5 | $1 / f_{\text {ADC }}$ |
|  |  | CKMODE $=01$ | - | - | 2.0 |  |
|  |  | CKMODE $=10$ | - | - | 2.25 |  |
|  |  | CKMODE $=11$ | - | - | 2.125 |  |
| $t_{\text {LATRINJ }}$ | Trigger conversion latency Injected channels aborting a regular conversion | CKMODE $=00$ | 2.5 | 3 | 3.5 | $1 / f_{\text {ADC }}$ |
|  |  | CKMODE $=01$ | - | - | 3.0 |  |
|  |  | CKMODE $=10$ | - | - | 3.25 |  |
|  |  | CKMODE $=11$ | - | - | 3.125 |  |
| $t_{s}$ | Sampling time | $\mathrm{f}_{\text {ADC }}=60 \mathrm{MHz}$ | 0.0416 | - | 10.675 | $\mu \mathrm{s}$ |
|  |  | - | 2.5 | - | 640.5 | $1 / f_{\text {ADC }}$ |
| $t_{\text {ADCYREG_STUP }}$ | ADC voltage regulator start-up time | - | - | - | 20 | $\mu \mathrm{s}$ |
| $t_{\text {CONV }}$ | Total conversion time (including sampling time) | $\begin{aligned} & \mathrm{f}_{\text {ADC }}=60 \mathrm{MHz} \\ & \text { Resolution }= \\ & 12 \text { bits } \end{aligned}$ | 0.25 | - | 10.883 | $\mu \mathrm{s}$ |
|  |  | - | $t_{s}[\text { cycles }]+$ resolution [bits] $+0.5=15$ to 653 |  |  | $1 / f_{\text {ADC }}$ |
| $I_{\text {DDA }}(\text { ADC }) \end{aligned}$ | ADC consumption from the $\mathrm{V}_{\text {DDA }}$ supply | fs $=4$ Msps | - | 590 | 730 | $\mu \mathrm{A}$ |
|  |  | fs $=1$ Msps | - | 160 | 220 |  |
|  |  | fs $=10$ ksps | - | 16 | 50 |  |
| $I_{\text {DDV_S }}(\text { ADC }) \end{aligned}$ | ADC consumption from the $\mathrm{V}_{\text {REF }+}$ single ended mode | fs $=4$ Msps | - | 110 | 140 | $\mu \mathrm{A}$ |
|  |  | fs $=1$ Msps | - | 30 | 40 |  |
|  |  | fs $=10$ ksps | - | 0.6 | 2 |  |
| $I_{\text {DDV_D }}(\text { ADC }) \end{aligned}$ | ADC consumption from the $\mathrm{V}_{\text {REF }+}$ differential mode | fs $=4$ Msps | - | 220 | 270 | $\mu \mathrm{A}$ |
|  |  | fs $=1$ Msps | - | 60 | 70 |  |
|  |  | fs $=10$ ksps | - | 1.3 | 3 |  |

1. Guaranteed by design.2. The I/O analog switch voltage booster is enabled when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ (BOOSTEN $=1$ in the SYSCFG_CFGR1 when $\left.V_{D D A}<2.4 \mathrm{~V}\right)$. It is disabled when $V_{D D A} \geq 2.4 \mathrm{~V}$.
3. $\mathrm{V}_{\mathrm{REF}+}$ can be internally connected to $\mathrm{V}_{\mathrm{DDA}}$, depending on the package. Refer to Section 4: Pinouts and pin description for further details.
4. The maximum value of RAIN can be found in Table 61: Maximum ADC RAIN.The maximum value of $R_{\text {AIN }}$ can be found in Table 61: Maximum ADC RAIN.

Table 61. Maximum ADC $\mathrm{R}_{\text {AIN }}{ }^{(1)(2)}$

| Resolution | Sampling cycle @60 MHz | Sampling time [ns] | $\mathrm{R}_{\text {AIN }}$ max ( $\Omega$ ) |  |
| :--: | :--: | :--: | :--: | :--: |
|  |  |  | Fast channels ${ }^{(3)}$ | Slow channels ${ }^{(4)}$ |
| 12 bits | 2.5 | 41.67 | 100 | N/A |
|  | 6.5 | 108.33 | 330 | 100 |
|  | 12.5 | 208.33 | 680 | 470 |
|  | 24.5 | 408.33 | 1500 | 1200 |
|  | 47.5 | 791.67 | 2200 | 1800 |
|  | 92.5 | 1541.67 | 4700 | 3900 |
|  | 247.5 | 4125 | 12000 | 10000 |
|  | 640.5 | 10675 | 39000 | 33000 |
| 10 bits | 2.5 | 41.67 | 120 | N/A |
|  | 6.5 | 108.33 | 390 | 180 |
|  | 12.5 | 208.33 | 820 | 560 |
|  | 24.5 | 408.33 | 1500 | 1200 |
|  | 47.5 | 791.67 | 2200 | 1800 |
|  | 92.5 | 1541.67 | 5600 | 4700 |
|  | 247.5 | 4125 | 12000 | 10000 |
|  | 640.5 | 10675 | 47000 | 39000 |
| 8 bits | 2.5 | 41.67 | 180 | N/A |
|  | 6.5 | 108.33 | 470 | 270 |
|  | 12.5 | 208.33 | 1000 | 680 |
|  | 24.5 | 408.33 | 1800 | 1500 |
|  | 47.5 | 791.67 | 2700 | 2200 |
|  | 92.5 | 1541.67 | 6800 | 5600 |
|  | 247.5 | 4125 | 15000 | 12000 |
|  | 640.5 | 10675 | 50000 | 50000 |
| 6 bits | 2.5 | 41.67 | 220 | N/A |
|  | 6.5 | 108.33 | 560 | 330 |
|  | 12.5 | 208.33 | 1200 | 1000 |
|  | 24.5 | 408.33 | 2700 | 2200 |
|  | 47.5 | 791.67 | 3900 | 3300 |
|  | 92.5 | 1541.67 | 8200 | 6800 |
|  | 247.5 | 4125 | 18000 | 15000 |
|  | 640.5 | 10675 | 50000 | 50000 |1. Guaranteed by design.
2. The I/O analog switch voltage booster is enabled when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ (BOOSTEN $=1$ in the SYSCFG_CFGR1 when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ ). It is disabled when $\mathrm{V}_{\mathrm{DDA}} \geq 2.4 \mathrm{~V}$.
3. Fast channels are: ADCx_IN1 to ADCx_IN5.
4. Slow channels are: all ADC inputs except the fast channels.Table 62. ADC accuracy - limited test conditions $1^{(1)(2)(3)}$

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| ET | Total unadjusted error | Single ADC operation ADC clock frequency $\leq 60 \mathrm{MHz}$, $\mathrm{V}_{\mathrm{DDA}}=\mathrm{VREF}+=3 \mathrm{~V}, \mathrm{TA}=$ $25^{\circ} \mathrm{C}$ <br> Continuous mode, sampling rate: <br> Fast channels@4Msps <br> Slow channels@2Msps | Single <br> ended | Fast channel (max speed) | - | 5.9 | 6.9 | LSB |
|  |  |  |  | Slow channel (max speed) | - | 5.5 | 6.9 |
|  |  |  | Differential | Fast channel (max speed) | - | 4.6 | 5.6 |
|  |  |  |  | Slow channel (max speed) | - | 4 | 5.6 |
| EO | Offset error |  | Single <br> ended | Fast channel (max speed) | - | 2.5 | 4 |
|  |  |  |  | Slow channel (max speed) | - | 1.9 | 4 |
|  |  |  | Differential | Fast channel (max speed) | - | 1.8 | 2.8 |
|  |  |  |  | Slow channel (max speed) | - | 1.1 | 2.8 |
| EG | Gain error |  | Single <br> ended | Fast channel (max speed) | - | 4.6 | 6.6 |
|  |  |  |  | Slow channel (max speed) | - | 4.5 | 6.6 |
|  |  |  | Differential | Fast channel (max speed) | - | 3.6 | 4.6 |
|  |  |  |  | Slow channel (max speed) | - | 3.3 | 4.6 |
| ED | Differential linearity error |  | Single <br> ended | Fast channel (max speed) | - | 1.1 | 1.9 |
|  |  |  |  | Slow channel (max speed) | - | 1.3 | 1.9 |
|  |  |  | Differential | Fast channel (max speed) | - | 1.3 | 1.6 |
|  |  |  |  | Slow channel (max speed) | - | 1.4 | 1.6 |
| EL | Integral <br> linearity <br> error |  | Single <br> ended | Fast channel (max speed) | - | 2.3 | 3.4 |
|  |  |  |  | Slow channel (max speed) | - | 2.4 | 3.4 |
|  |  |  | Differential | Fast channel (max speed) | - | 2.1 | 3.2 |
|  |  |  |  | Slow channel (max speed) | - | 2.2 | 3.2 |
| ENOB | Effective number of bits |  | Single <br> ended | Fast channel (max speed) | 10.4 | 10.6 | - | bits |
|  |  |  |  | Slow channel (max speed) | 10.4 | 10.6 | - |
|  |  |  | Differential | Fast channel (max speed) | 10.8 | 10.9 | - |
|  |  |  |  | Slow channel (max speed) | 10.8 | 10.9 | - |
| SINAD | Signal-tonoise and distortion ratio |  | Single <br> ended | Fast channel (max speed) | 64.4 | 65.6 | - | dB |
|  |  |  |  | Slow channel (max speed) | 64.4 | 65.6 | - |
|  |  |  | Differential | Fast channel (max speed) | 66.8 | 67.5 | - |
|  |  |  |  | Slow channel (max speed) | 66.8 | 67.5 | - |
| SNR | Signal-tonoise ratio |  | Single <br> ended | Fast channel (max speed) | 65 | 66.9 | - |
|  |  |  |  | Slow channel (max speed) | 65 | 66.9 | - |
|  |  |  | Differential | Fast channel (max speed) | 67 | 69 | - |
|  |  |  |  | Slow channel (max speed) | 67 | 69 | - |Table 62. ADC accuracy - limited test conditions $1^{(1)(2)(3)}$ (continued)

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| THD | Total harmonic distortion | Single ADC operation ADC clock frequency $\leq 60 \mathrm{MHz}$. $\begin{aligned} & \mathrm{V}_{\text {DDA }}=\text { VREF }+=3 \mathrm{~V}, \mathrm{TA}= \\ & 25^{\circ} \mathrm{C} \end{aligned}$ <br> Continuous mode, sampling rate: <br> Fast channels@4Msps <br> Slow channels@2Msps | Single <br> ended | Fast channel (max speed) | - | $-73$ | $-72$ | dB |
|  |  |  |  | Slow channel (max speed) | - | $-73$ | $-72$ |  |
|  |  |  | Differential | Fast channel (max speed) | - | $-73$ | $-72$ |  |
|  |  |  |  | Slow channel (max speed) | - | $-73$ | $-72$ |  |

1. Evaluated by characterization - Not tested in production.
2. ADC DC accuracy values are measured after internal calibration.
3. ADC accuracy vs. negative Injection Current: Injecting negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to analog pins which may potentially inject negative current.
4. The I/O analog switch voltage booster is enabled when $\mathrm{V}_{\text {DDA }}<2.4 \mathrm{~V}$ (BOOSTEN $=1$ in the SYSCFG_CFGR1 when $\left.V_{\text {DDA }} \approx 2.4 \mathrm{~V}\right)$. It is disabled when $\mathrm{V}_{\text {DDA }} \geq 2.4 \mathrm{~V}$. No oversampling.Table 63. ADC accuracy - limited test conditions $2^{(1)(2)(3)}$

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| ET | Total unadjusted error | Single ADC operation ADC clock frequency $\leq 60 \mathrm{MHz}, 2 \mathrm{~V} \leq \mathrm{V}_{\text {DDA }}$ Continuous mode, sampling rate: Fast channels@4Msps Slow channels@2Msps | Single <br> ended | Fast channel (max speed) | - | 5.9 | 8.4 | LSB |
|  |  |  | Slow channel (max speed) | - | 5.5 | 8 |  |
|  |  |  | Differential | Fast channel (max speed) | - | 4.6 | 6.6 |
|  |  |  |  | Slow channel (max speed) | - | 4 | 6 |
| EO | Offset error |  | Single <br> ended | Fast channel (max speed) | - | 2.5 | 6 |
|  |  |  |  | Slow channel (max speed) | - | 1.9 | 6.9 |
|  |  |  | Differential | Fast channel (max speed) | - | 1.8 | 3.3 |
|  |  |  |  | Slow channel (max speed) | - | 1.1 | 3.3 |
| EG | Gain error |  | Single <br> ended | Fast channel (max speed) | - | 4.6 | 8.1 |
|  |  |  |  | Slow channel (max speed) | - | 4.5 | 8.1 |
|  |  |  | Differential | Fast channel (max speed) | - | 3.6 | 4.6 |
|  |  |  |  | Slow channel (max speed) | - | 3.3 | 4.6 |
| ED | Differential linearity error |  | Single ADC operation ADC clock frequency $\leq 60 \mathrm{MHz}, 2 \mathrm{~V} \leq \mathrm{V}_{\text {DDA }}$ Continuous mode, sampling rate: Fast channels@4Msps Slow channels@2Msps | Single <br> ended | Fast channel (max speed) | - | 1.1 | 1.8 |
|  |  |  |  | Slow channel (max speed) | - | 1.3 | 1.8 |
|  |  |  | Differential | Fast channel (max speed) | - | 1.3 | 1.6 |
|  |  |  |  | Slow channel (max speed) | - | 1.4 | 1.6 |
| EL | Integral linearity error |  | Single <br> ended | Fast channel (max speed) | - | 2.3 | 4.4 |
|  |  |  |  | Slow channel (max speed) | - | 2.4 | 4.4 |
|  |  |  | Differential | Fast channel (max speed) | - | 2.1 | 4.1 |
|  |  |  |  | Slow channel (max speed) | - | 2.2 | 3.7 |
| ENOB | Effective number of bits |  | Single <br> ended | Fast channel (max speed) | 10 | 10.6 | - | bits |
|  |  |  |  | Slow channel (max speed) | 10 | 10.6 | - |
|  |  |  | Differential | Fast channel (max speed) | 10.7 | 10.9 | - |
|  |  |  |  | Slow channel (max speed) | 10.7 | 10.9 | - |
| SINAD | Signal-tonoise and distortion ratio |  | Single <br> ended | Fast channel (max speed) | 62 | 65.6 | - | dB |
|  |  |  |  | Slow channel (max speed) | 62 | 65.6 | - |
|  |  |  | Differential | Fast channel (max speed) | 65 | 67.5 | - |
|  |  |  |  | Slow channel (max speed) | 65 | 67.5 | - |
| SNR | Signal-tonoise ratio |  | Single <br> ended | Fast channel (max speed) | 64 | 66.9 | - |
|  |  |  |  | Slow channel (max speed) | 64 | 66.9 | - |
|  |  |  | Differential | Fast channel (max speed) | 66.5 | 69 | - |
|  |  |  |  | Slow channel (max speed) | 66.5 | 69 | - |Table 63. ADC accuracy - limited test conditions $2^{(1)(2)(3)}$ (continued)

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| THD | Total harmonic distortion | Single ADC operation ADC clock frequency $\leq 60 \mathrm{MHz}, 2 \mathrm{~V} \leq \mathrm{V}_{\text {DDA }}$ <br> Continuous mode, sampling rate: <br> Fast channels@4Msps <br> Slow channels@2Msps | Single <br> ended | Fast channel (max speed) | - | $-73$ | $-65$ | dB |
|  |  |  |  | Slow channel (max speed) | - | $-73$ | $-67$ |  |
|  |  |  | Differential | Fast channel (max speed) | - | $-73$ | $-70$ |  |
|  |  |  |  | Slow channel (max speed) | - | $-73$ | $-71$ |  |

1. Evaluated by characterization - Not tested in production.
2. ADC DC accuracy values are measured after internal calibration.
3. ADC accuracy vs. negative Injection Current: Injecting negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to analog pins which may potentially inject negative current.
4. The I/O analog switch voltage booster is enabled when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ (BOOSTEN $=1$ in the SYSCFG_CFGR1 when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ ). It is disabled when $\mathrm{V}_{\mathrm{DDA}} \geq 2.4 \mathrm{~V}$. No oversampling.Table 64. ADC accuracy - limited test conditions $3^{(1)(2)(3)}$

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| ET | Total unadjusted error | Single ADC operation ADC clock frequency $\leq$ 60 MHz , $1.62 \mathrm{~V} \leq \mathrm{V}_{\text {DDA }}=\mathrm{V}_{\text {REF }},$ $\leq 3.6 \mathrm{~V}$, <br> Continuous mode, sampling rate. Fast channels@4Msps Slow channels@2Msps | Single <br> ended | Fast channel (max speed) | - | 5.9 | 7.9 | LSB |
|  |  |  |  | Slow channel (max speed) | - | 5.5 | 7.5 |  |
|  |  |  | Differential | Fast channel (max speed) | - | 4.6 | 7.6 |  |
|  |  |  |  | Slow channel (max speed) | - | 4 | 5.5 |  |
| EO | Offset error |  | Single <br> ended | Fast channel (max speed) | - | 2.5 | 5.5 |  |
|  |  |  |  | Slow channel (max speed) | - | 1.9 | 5.5 |  |
|  |  |  | Differential | Fast channel (max speed) | - | 1.8 | 3.5 |  |
|  |  |  |  | Slow channel (max speed) | - | 1.1 | 3 |  |
| EG | Gain error |  | Single <br> ended | Fast channel (max speed) | - | 4.6 | 7.1 |  |
|  |  |  |  | Slow channel (max speed) | - | 4.5 | 7 |  |
|  |  |  | Differential | Fast channel (max speed) | - | 3.6 | 4.1 |  |
|  |  |  |  | Slow channel (max speed) | - | 3.3 | 4.8 |  |
| ED | Differential linearity error | Single ADC operation ADC clock frequency $\leq$ 60 MHz , $1.62 \mathrm{~V} \leq \mathrm{V}_{\text {DDA }}=\mathrm{V}_{\text {REF }},$ $\leq 3.6 \mathrm{~V}$, <br> Continuous mode, sampling rate. Fast channels@4Msps Slow channels@2Msps | Single <br> ended | Fast channel (max speed) | - | 1.1 | 1.9 |  |
|  |  |  |  | Slow channel (max speed) | - | 1.3 | 1.9 |  |
|  |  |  | Differential | Fast channel (max speed) | - | 1.3 | 1.6 |  |
|  |  |  |  | Slow channel (max speed) | - | 1.4 | 1.6 |  |
| EL | Integral linearity error |  | Single <br> ended | Fast channel (max speed) | - | 2.3 | 4.4 |  |
|  |  |  |  | Slow channel (max speed) | - | 2.4 | 4.4 |  |
|  |  |  | Differential | Fast channel (max speed) | - | 2.1 | 3.7 |  |
|  |  |  |  | Slow channel (max speed) | - | 2.2 | 3.7 |  |
| ENOB | Effective number of bits |  | Single <br> ended | Fast channel (max speed) | 10 | 10.6 | - | bits |
|  |  |  |  | Slow channel (max speed) | 10 | 10.6 | - |  |
|  |  |  | Differential | Fast channel (max speed) | 10.6 | 10.9 | - |  |
|  |  |  |  | Slow channel (max speed) | 10.6 | 10.9 | - |  |
| SINAD | Signal-tonoise and distortion ratio |  | Single <br> ended | Fast channel (max speed) | 62 | 65.6 | - | dB |
|  |  |  |  | Slow channel (max speed) | 62 | 65.6 | - |  |
|  |  |  | Differential | Fast channel (max speed) | 65 | 67.5 | - |  |
|  |  |  |  | Slow channel (max speed) | 65 | 67.5 | - |  |
| SNR | Signal-tonoise ratio |  | Single <br> ended | Fast channel (max speed) | 63 | 66.9 | - |  |
|  |  |  |  | Slow channel (max speed) | 63 | 66.9 | - |  |
|  |  |  | Differential | Fast channel (max speed) | 66 | 69 | - |  |
|  |  |  |  | Slow channel (max speed) | 66 | 69 | - |  |Table 64. ADC accuracy - limited test conditions $3^{(1)(2)(3)}$ (continued)

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| THD | Total harmonic distortion | Single ADC operation ADC clock frequency $\leq$ 60 MHz , $1.62 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DDA}}=\mathrm{V}_{\mathrm{REF}}$, $\leq 3.6 \mathrm{~V}$, <br> Continuous mode, sampling rate; <br> Fast channels@4Msps <br> Slow channels@2Msps | Single <br> ended | Fast channel (max speed) | - | $-73$ | $-67$ | dB |
|  |  |  |  | Slow channel (max speed) | - | $-73$ | $-67$ |  |
|  |  |  | Differential | Fast channel (max speed) | - | $-73$ | $-71$ |  |
|  |  |  |  | Slow channel (max speed) | - | $-73$ | $-71$ |  |

1. Evaluated by characterization - Not tested in production.
2. ADC DC accuracy values are measured after internal calibration.
3. ADC accuracy vs. negative Injection Current: Injecting negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to analog pins which may potentially inject negative current.
4. The I/O analog switch voltage booster is enabled when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ (BOOSTEN $=1$ in the SYSCFG_CFGR1 when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ ). It is disabled when $\mathrm{V}_{\mathrm{DDA}} \geq 2.4 \mathrm{~V}$. No oversampling.Table 65. ADC accuracy (Multiple ADCs operation) - limited test conditions 1(1)(2)(3)

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| ET | Total unadjusted error | Multiple ADC operation ADC clock frequency: single ended $\leq 52 \mathrm{MHz}$, differential $\leq 56 \mathrm{MHz}$, $\mathrm{V}_{\text {DDA }}=\mathrm{V}_{\text {REF }} \equiv 3.3 \mathrm{~V}$, $25^{\circ} \mathrm{C}$, <br> Continuous mode, sampling time: <br> Fast channels: 2.5 cycles Slow channels: 6.5 cycles LQFP100 package | Single ended | - | 4.5 | - | LSB |
|  |  |  | Differential | - | 4.1 | - |  |
|  |  |  | Single ended | - | 1.3 | - |  |
|  |  |  | Differential | - | 0.4 | - |  |
| EG | Gain error |  | Single ended | - | 3.9 | - |  |
|  |  |  | Differential | - | 3.4 | - |  |
| ED | Differential linearity error |  | Single ended | - | 1.5 | - |  |
|  |  |  | Differential | - | 1.2 | - |  |
| EL | Integral linearity error | $\begin{aligned} & \mathrm{V}_{\text {DDA }}=\mathrm{V}_{\text {REF }} \equiv 3.3 \mathrm{~V} \\ & 25^{\circ} \mathrm{C} \end{aligned}$ | Single ended | - | 1.7 | - |  |
|  |  |  | Differential | - | 2.1 | - |  |
| ENOB | Effective number of bits |  | Single ended | - | 10.7 | - | bits |
|  |  |  | Differential | - | 10.9 | - |  |
| SINAD | Signal-to-noise and distortion ratio |  | Single ended | - | 66.3 | - | dB |
|  |  |  | Differential | - | 67.2 | - |  |
| SNR | Signal-to-noise ratio |  | Single ended | - | 67.3 | - |  |
|  |  |  | Differential | - | 68.6 | - |  |
| THD | Total harmonic distortion |  | Single ended | - | $-73.5$ | - | dB |
|  |  |  | Differential | - | $-73$ | - |  |

1. Data based on characterization result, not tested in production.
2. ADC DC accuracy values are measured after internal calibration.
3. ADC accuracy vs. negative Injection Current: Injecting negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to analog pins which may potentially inject negative current.
4. The I/O analog switch voltage booster is enabled when $\mathrm{V}_{\text {DDA }}<2.4 \mathrm{~V}$ (BOOSTEN $=1$ in the SYSCFG_CFGR1 when $\mathrm{V}_{\text {DDA }}<2.4 \mathrm{~V}$ ). It is disabled when $\mathrm{V}_{\text {DDA }} \geq 2.4 \mathrm{~V}$. No oversampling.Table 66. ADC accuracy (Multiple ADCs operation) - limited test conditions $2^{(1)(2)(3)}$

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| ET | Total unadjusted error | Multiple ADC operation ADC clock frequency: single ended $\leq 52 \mathrm{MHz}$, differential $\leq 56 \mathrm{MHz}$, $\mathrm{V}_{\text {DDA }} \geq 2.7 \mathrm{~V}, \mathrm{~V}_{\text {REF }} \geq 1.62 \mathrm{~V}$, -40 to $125^{\circ} \mathrm{C}$, Continuous mode, sampling time: <br> Fast channels: 2.5 cycles Slow channels: 6.5 cycles LQFP100 package | Single ended | - | 7.1 | - | LSB |
|  |  |  | Differential | - | 4.6 | - |  |
| EO | Offset error |  | Single ended | - | 4.2 | - |  |
|  |  |  | Differential | - | 2.8 | - |  |
| EG | Gain error |  | Single ended | - | 6.8 | - |  |
|  |  |  | Differential | - | 4.3 | - |  |
| ED | Differential linearity error |  | Single ended | - | 1.5 | - |  |
|  |  |  | Differential | - | 1.7 | - |  |
| EL | Integral linearity error | $\begin{aligned} & \mathrm{V}_{\text {DDA }} \geq 2.7 \mathrm{~V}, \mathrm{~V}_{\text {REF }} \geq 1.62 \mathrm{~V} \\ & \text {-40 to } 125^{\circ} \mathrm{C} \end{aligned}$ | Single ended | - | 3.1 | - |  |
|  |  |  | Differential | - | 2.4 | - |  |
| ENOB | Effective number of bits |  | Single ended | - | 10.2 | - | bits |
|  |  |  | Differential | - | 10.6 | - |  |
| SINAD | Signal-to-noise and distortion ratio |  | Single ended | - | 62.9 | - | dB |
|  |  |  | Differential | - | 65.3 | - |  |
| SNR | Signal-to-noise ratio |  | Single ended | - | 63.6 | - |  |
|  |  |  | Differential | - | 66.3 | - |  |
| THD | Total harmonic distortion |  | Single ended | - | $-70.9$ | - | dB |
|  |  |  | Differential | - | $-71.8$ | - |  |

1. Data based on characterization result, not tested in production.
2. ADC DC accuracy values are measured after internal calibration.
3. ADC accuracy vs. negative Injection Current: Injecting negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to analog pins which may potentially inject negative current.
4. The I/O analog switch voltage booster is enabled when $\mathrm{V}_{\text {DDA }}<2.4 \mathrm{~V}$ (BOOSTEN $=1$ in the SYSCFG_CFGR1 when $\mathrm{V}_{\text {DDA }}<2.4 \mathrm{~V}$ ). It is disabled when $\mathrm{V}_{\text {DDA }} \geq 2.4 \mathrm{~V}$. No oversampling.Table 67. ADC accuracy (Multiple ADCs operation) - limited test conditions $3^{(1)(2)(3)}$

| Symbol | Parameter | Conditions ${ }^{(4)}$ |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| ET | Total unadjusted error | Multiple ADC operation ADC clock frequency: single ended $\leq 42 \mathrm{MHz}$, differential $\leq 56 \mathrm{MHz}$, $\mathrm{V}_{\mathrm{DDA}} \approx \mathrm{V}_{\text {REF }} \geq 1.62 \mathrm{~V}$, -40 to $125^{\circ} \mathrm{C}$, Continuous mode, sampling time: <br> Fast channels: 2.5 cycles Slow channels: 6.5 cycles LQFP100 package | Single ended | - | 7.4 | - | LSB |
|  |  |  | Differential | - | 4.6 | - |  |
|  |  |  | Single ended | - | 4 | - |  |
|  |  |  | Differential | - | 2.8 | - |  |
| EG | Gain error |  | Single ended | - | 7.2 | - |  |
|  |  |  | Differential | - | 4.3 | - |  |
| ED | Differential linearity error |  | Single ended | - | 1.8 | - |  |
|  |  |  | Differential | - | 1.7 | - |  |
| EL | Integral linearity error | $\begin{aligned} & \mathrm{V}_{\text {DDA }} \approx \mathrm{V}_{\text {REF }} \geq 1.62 \mathrm{~V} \\ & \text {-40 to } 125^{\circ} \mathrm{C} \end{aligned}$ | Single ended | - | 3.1 | - |  |
|  |  |  | Differential | - | 2.4 | - |  |
| ENOB | Effective number of bits |  | Single ended | - | 10.1 | - | bits |
|  |  |  | Differential | - | 10.6 | - |  |
| SINAD | Signal-to-noise and distortion ratio |  | Single ended | - | 62.6 | - | dB |
|  |  |  | Differential | - | 65.3 | - |  |
| SNR | Signal-to-noise ratio |  | Single ended | - | 63.2 | - |  |
|  |  |  | Differential | - | 66.3 | - |  |
| THD | Total harmonic distortion |  | Single ended | - | $-70.6$ | - | dB |
|  |  |  | Differential | - | $-71.8$ | - |  |

1. Data based on characterization result, not tested in production.
2. ADC DC accuracy values are measured after internal calibration.
3. ADC accuracy vs. negative Injection Current: Injecting negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to analog pins which may potentially inject negative current.
4. The I/O analog switch voltage booster is enabled when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ (BOOSTEN $=1$ in the SYSCFG_CFGR1 when $\mathrm{V}_{\mathrm{DDA}}<2.4 \mathrm{~V}$ ). It is disabled when $\mathrm{V}_{\mathrm{DDA}} \geq 2.4 \mathrm{~V}$. No oversampling.Figure 28. ADC accuracy characteristics
![img-28.jpeg](img-28.jpeg)

Figure 29. Typical connection diagram when using the ADC with FT/TT pins featuring analog switch function
![img-29.jpeg](img-29.jpeg)

1. Refer to Table 60: ADC characteristics for the values of $R_{A i N}$ and $C_{A D C}$.
2. $\mathrm{C}_{\text {parasitic }}$ represents the capacitance of the PCB (dependent on soldering and PCB layout quality) plus the pad capacitance (refer to Table 53: I/O static characteristics for the value of the pad capacitance). A high $\mathrm{C}_{\text {parasitic }}$ value downgrades conversion accuracy. To remedy this, $\mathrm{I}_{\mathrm{ADC}}$ should be reduced.
3. Refer to Table 53: I/O static characteristics for the values of $\mathrm{I}_{\mathrm{kg}}$.
4. Refer to Figure 16: Power supply scheme.

# General PCB design guidelines 

Power supply decoupling must be performed as shown in Figure 16: Power supply scheme. The decoupling capacitor on $\mathrm{V}_{\mathrm{DDA}}$ must be ceramic (good quality) and it must be placed as close as possible to the chip.# 5.3.19 Digital-to-Analog converter characteristics 

Table 68. DAC 1MSPS characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{D D A}$ | Analog supply voltage for DAC ON | DAC output buffer OFF, DAC_OUT pin not connected (internal connection only) |  | 1.71 | - | 3.6 | V |
|  |  | Other modes |  | 1.80 | - |  |  |
| $V_{\text {REF }+}$ | Positive reference voltage | DAC output buffer OFF, DAC_OUT pin not connected (internal connection only) |  | 1.71 | - | $V_{D D A}$ |  |
|  |  | Other modes |  | 1.80 | - |  |  |
| $V_{\text {REF- }}$ | Negative reference voltage | - |  |  | $V_{\text {SSA }}$ |  |  |
| $R_{L}$ | Resistive load | DAC output buffer ON | connected to $\mathrm{V}_{\text {SSA }}$ | 5 | - | - | k $\Omega$ |
|  |  |  | connected to $\mathrm{V}_{\text {DDA }}$ | 25 | - | - |  |
| $R_{O}$ | Output Impedance | DAC output buffer OFF |  | 9.6 | 11.7 | 13.8 | $\mathrm{k} \Omega$ |
| $R_{\text {BON }}$ | Output impedance sample and hold mode, output buffer ON | $\begin{aligned} & V_{D D}=2.7 \mathrm{~V} \\ & V_{D D}=2.0 \mathrm{~V} \end{aligned}$ |  | - | - | 2 | k $\Omega$ |
|  |  |  |  | - | - | 3.5 |  |
| $R_{\text {BOFF }}$ | Output impedance sample and hold mode, output buffer OFF | $\begin{aligned} & V_{D D}=2.7 \mathrm{~V} \\ & V_{D D}=2.0 \mathrm{~V} \end{aligned}$ |  | - | - | 16.5 | k $\Omega$ |
|  |  |  |  | - | - | 18.0 |  |
| $\mathrm{C}_{\mathrm{L}}$ | Capacitive load | DAC output buffer ON |  | - | - | 50 | pF |
| $\mathrm{C}_{\mathrm{SH}}$ |  | Sample and hold mode |  | - | 0.1 | 1 | $\mu \mathrm{F}$ |
| $V_{\text {DAC_OUT }}$ | Voltage on DAC_OUT output | DAC output buffer ON |  | 0.2 | - | $\begin{gathered} \mathrm{V}_{\text {REF+ }} \\ -0.2 \end{gathered}$ | V |
|  |  | DAC output buffer OFF |  | 0 | - | $V_{\text {REF+ }}$ |  |
| $t_{\text {SETTLING }}$ | Settling time (full scale: for a 12-bit code transition between the lowest and the highest input codes when DAC_OUT reaches final value) | Normal mode DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}$, $\mathrm{RL} \geq 5 \mathrm{k} \Omega$ | $\pm 0.5$ LSB | - | 1.7 | 3 | $\mu \mathrm{s}$ |
|  |  |  | $\pm 1$ LSB | - | 1.6 | 2.9 |  |
|  |  |  | $\pm 2$ LSB | - | 1.55 | 2.85 |  |
|  |  |  | $\pm 4$ LSB | - | 1.48 | 2.8 |  |
|  |  |  | $\pm 8$ LSB | - | 1.4 | 2.75 |  |
|  |  | Normal mode DAC output buffer OFF, $\pm 1$ LSB, CL $=10 \mathrm{pF}$ | - | 2 | 2.5 |  |  |
| $t_{\text {WAKEUP }}{ }^{(2)}$ | Wakeup time from off state (setting the ENx bit in the DAC Control register) until final value $\pm 1$ LSB | Normal mode DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega$ |  | - | 4.2 | 7.5 | $\mu \mathrm{s}$ |
|  |  | Normal mode DAC output buffer OFF, CL $\leq 10 \mathrm{pF}$ |  | - | 2 | 5 |  |
| PSRR | $V_{\text {DDA }}$ supply rejection ratio | Normal mode DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL}=5 \mathrm{k} \Omega, \mathrm{DC}$ |  | - | $-80$ | $-28$ | dB |Table 68. DAC 1MSPS characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $T_{W \_t o \_W}$ | Minimal time between two consecutive writes into the DAC_DORx register to guarantee a correct DAC_OUT for a small variation of the input code (1 LSB) <br> DAC_MCR:MODEx[2:0] = 000 or 001 <br> DAC_MCR:MODEx[2:0] = 010 or 011 | $\begin{aligned} & \mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega \\ & \mathrm{CL} \leq 10 \mathrm{pF} \end{aligned}$ | $\begin{aligned} & 1 \\ & 1.4 \end{aligned}$ | - | - | $\mu \mathrm{s}$ |
| $t_{\text {SAMP }}$ | Sampling time in sample and hold mode (code transition between the lowest input code and the highest input code when DACOUT reaches final value $\pm 1 \mathrm{LSB}$ ) | DAC_OUT pin connected | DAC output buffer $\mathrm{ON}, \mathrm{C}_{\mathrm{SH}}=100 \mathrm{nF}$ | - | 0.7 | 3.5 | ms |
|  |  |  | DAC output buffer OFF, $\mathrm{C}_{\mathrm{SH}}=100 \mathrm{nF}$ | - | 10.5 | 18 |
|  |  | DAC_OUT pin not connected (internal connection only) | DAC output buffer OFF | - | 2 | 3.5 | $\mu \mathrm{s}$ |
| $I_{\text {leak }}$ | Output leakage current | Sample and hold mode, DAC_OUT pin connected |  | - | - | ${ }^{(3)}$ | nA |
| $\mathrm{Cl}_{\text {int }}$ | Internal sample and hold capacitor | - |  | 5.2 | 7 | 8.8 | pF |
| $t_{\text {TRIM }}$ | Middle code offset trim time | DAC output buffer ON |  | 50 | - | - | $\mu \mathrm{s}$ |
| $V_{\text {offset }}$ | Middle code offset for 1 trim code step | $\mathrm{V}_{\text {REF }+}=3.6 \mathrm{~V}$ |  | - | 1500 | - | $\mu \mathrm{V}$ |
|  |  | $\mathrm{V}_{\text {REF }+}=1.8 \mathrm{~V}$ |  | - | 750 | - |  |
| $I_{D D A}(D A C)$ | DAC consumption from $\mathrm{V}_{\text {DDA }}$ | DAC output buffer ON | No load, middle code (0x800) | - | 315 | 500 | $\mu \mathrm{A}$ |
|  |  |  | No load, worst code (0xF1C) | - | 450 | 670 |  |
|  |  | DAC output buffer OFF | No load, middle code (0x800) | - | - | 0.2 |  |
|  |  | Sample and hold mode, $\mathrm{C}_{\mathrm{SH}}=$ 100 nF |  | - | $\begin{gathered} 315+ \\ \text { Ton/(Ton } \\ \text { +Toff) } \\ \text { (4) } \end{gathered}$ | $\begin{gathered} 670+ \\ \text { Ton/(Ton } \\ \text { +Toff) } \\ \text { (4) } \end{gathered}$ |  |Table 68. DAC 1MSPS characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{I}_{\text {DDV }}(\text { DAC }) \begin{aligned} & \mathrm{I}_{\text {DDV }}(\text { DAC }) \\ & \mathrm{V}_{\text {REF }+} \end{aligned}$ | DAC consumption from $\mathrm{V}_{\text {REF }+}$ | DAC output buffer ON | No load, middle code (0x800) | - | 185 | 240 | $\mu \mathrm{A}$ |
|  |  |  | No load, worst code (0xF1C) | - | 340 | 400 |  |
|  |  | DAC output buffer OFF | No load, middle code (0x800) | - | 155 | 205 |  |
|  |  | Sample and hold mode, buffer ON, $\mathrm{C}_{\mathrm{SH}}=100 \mathrm{nF}$, worst case |  | - | $\begin{gathered} 185 \times \\ \text { Ton/(Ton } \\ + \text { Toff) } \\ (4) \end{gathered}$ | $\begin{gathered} 400 \times \\ \text { Ton/(Ton } \\ + \text { Toff }) \\ (4) \end{gathered}$ |  |
|  |  | Sample and hold mode, buffer OFF, $\mathrm{C}_{\mathrm{SH}}=100 \mathrm{nF}$, worst case |  | - | $\begin{gathered} 155 \times \\ \text { Ton/(Ton } \\ + \text { Toff) } \\ (4) \end{gathered}$ | $\begin{gathered} 205 \times \\ \text { Ton/(Ton } \\ + \text { Toff }) \\ (4) \end{gathered}$ |  |

1. Guaranteed by design.
2. In buffered mode, the output can overshoot above the final value for low input code (starting from min value).
3. Refer to Table 53: I/O static characteristics.
4. Ton is the Refresh phase duration. Toff is the Hold phase duration. Refer to the reference manual RM0440 "STM32G4 Series advanced Arm ${ }^{\circledR}$-based 32-bit MCUs" for more details.

Figure 30. 12-bit buffered / non-buffered DAC
![img-30.jpeg](img-30.jpeg)

1. The DAC integrates an output buffer to reduce the output impedance and to drive external loads directly without the use of an external operational amplifier. The buffer can be bypassed by configuring the BOFFx bit in the DAC_CR register.Table 69. DAC 1MSPS accuracy ${ }^{(1)}$

| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| DNL | Differential non linearity ${ }^{(2)}$ | DAC output buffer ON |  | - | - | $\pm 2$ | LSB |
|  |  | DAC output buffer OFF |  | - | - | $\pm 2$ |  |
| - | monotonicity | 10 bits |  | Guaranteed |  |  |  |
| INL | Integral non linearity ${ }^{(3)}$ | DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega$ |  | - | - | $\pm 4$ |  |
|  |  | DAC output buffer OFF $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL |  | - | - | $\pm 4$ |  |
| Offset | Offset error at code $0 \times 800^{(3)}$ | DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega$ | $\mathrm{V}_{\mathrm{REF}+}=3.6 \mathrm{~V}$ | - | - | $\pm 12$ |  |
|  |  |  | $\mathrm{V}_{\text {REF }+}=1.8 \mathrm{~V}$ | - | - | $\pm 25$ |  |
|  |  | DAC output buffer OFF $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL |  | - | - | $\pm 8$ |  |
| Offset1 | Offset error at code $0 \times 001^{(4)}$ | DAC output buffer OFF $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL |  | - | - | $\pm 5$ |  |
| OffsetCal | Offset Error at code $0 \times 800$ after calibration | DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega$ | $\mathrm{V}_{\text {REF }+}=3.6 \mathrm{~V}$ | - | - | $\pm 5$ |  |
|  |  |  | $\mathrm{V}_{\text {REF }+}=1.8 \mathrm{~V}$ | - | - | $\pm 7$ |  |
| Gain | Gain error ${ }^{(5)}$ | DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega$ |  | - | - | $\pm 0.5$ | \% |
|  |  | DAC output buffer OFF $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL |  | - | - | $\pm 0.5$ |  |
| TUE | Total unadjusted error | DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega$ |  | - | - | $\pm 30$ | LSB |
|  |  | DAC output buffer OFF $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL |  | - | - | $\pm 12$ |  |
| TUECal | Total unadjusted error after calibration | DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega$ |  | - | - | $\pm 23$ | LSB |
| SNR | Signal-to-noise ratio | DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega$ <br> 1 kHz , BW 500 kHz |  | - | 71.2 | - | dB |
|  |  | DAC output buffer OFF $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL, 1 kHz BW 500 kHz |  | - | 71.6 | - |  |
| THD | Total harmonic distortion | DAC output buffer ON $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega, 1 \mathrm{kHz}$ |  | - | $-78$ | - | dB |
|  |  | DAC output buffer OFF $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL, 1 kHz |  | - | $-79$ | - |  |Table 69. DAC 1MSPS accuracy ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| SINAD | Signal-to-noise <br> and distortion <br> ratio | DAC output buffer ON <br> $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega, 1 \mathrm{kHz}$ | - | 70.4 | - | dB |
|  |  | DAC output buffer OFF <br> $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL, 1 kHz | - | 71 | - |  |
| ENOB | Effective <br> number of bits | DAC output buffer ON <br> $\mathrm{CL} \leq 50 \mathrm{pF}, \mathrm{RL} \geq 5 \mathrm{k} \Omega, 1 \mathrm{kHz}$ | - | 11.4 | - | bits |
|  |  | DAC output buffer OFF <br> $\mathrm{CL} \leq 50 \mathrm{pF}$, no RL, 1 kHz | - | 11.5 | - |  |

1. Guaranteed by design.
2. Difference between two consecutive codes - 1 LSB.
3. Difference between measured value at Code i and the value at Code i on a line drawn between Code 0 and last Code 4095.
4. Difference between the value measured at Code $(0 \times 001)$ and the ideal value.
5. Difference between ideal slope of the transfer function and measured slope computed from code $0 \times 000$ and $0 x F F F$ when buffer is OFF, and from code giving 0.2 V and $\left(\mathrm{V}_{\mathrm{REF}+}-0.2\right) \mathrm{V}$ when buffer is ON.

Table 70. DAC 15MSPS characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {DDA }}$ | Analog supply voltage for DAC ON | - | 1.71 | - | 3.6 | V |
| $\mathrm{V}_{\text {REF }+}$ | Positive reference voltage | - | 1.71 | - | $\mathrm{V}_{\text {DDA }}$ |  |
| $\mathrm{V}_{\text {REF }}$. | Negative reference voltage | - |  | $V_{\text {SSA }}$ |  |  |
| $\mathrm{V}_{\text {DAC_OUT }}$ | Voltage on DAC_OUT output | - | 0 | - | $\mathrm{V}_{\text {REF }+}$ | V |
| $t_{\text {SETTLING }}$ | Settling time (full scale: for a 12-bit code transition between the lowest and the highest input codes when DAC_OUT reaches final value) | $\mathrm{V}_{\text {DDA }}>2,7 \mathrm{~V}$ <br> With One comparator <br> on DAC output | $10 \%-90 \%$ | - | 16 | 22 | ns |
|  |  |  | $5 \%-95 \%$ | - | 21 | 29 |
|  |  |  | $1 \%-99 \%$ | - | 33 | 46 |
|  |  |  | 32lsb | - | 40 | 53 |
|  |  |  | 1lsb | - | 64 | 87 |
|  |  | $\mathrm{V}_{\text {DDA }}>2,7 \mathrm{~V}$ <br> With One comparator and OPAMP on DAC output | $10 \%-90 \%$ | - | 24 | 32 |  |
|  |  |  | $5 \%-95 \%$ | - | 32 | 43 |
|  |  |  | $1 \%-99 \%$ | - | 49 | 67 |
|  |  |  | 32lsb | - | 57 | 75 |
|  |  |  | 1lsb | - | 93 | 125 |Table 70. DAC 15MSPS characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {SETTLING }}$ | Settling time (full scale: for a 12-bit code transition between the lowest and the highest input codes when DAC_OUT reaches final value) | $\mathrm{V}_{\text {DDA }}<2,7 \mathrm{~V}$ <br> With One comparator on DAC output | $10 \%-90 \%$ | - | 16 | 88 | ns |
|  |  |  | $5 \%-95 \%$ | - | 21 | 116 |  |
|  |  |  | $1 \%-99 \%$ | - | 33 | 181 |  |
|  |  |  | 32lsb | - | 40 | 196 |  |
|  |  |  | 1 lsb | - | 64 | 332 |  |
|  |  | $\mathrm{V}_{\text {DDA }}<2,7 \mathrm{~V}$ <br> With One comparator and OPAMP on DAC output | $10 \%-90 \%$ | - | 24 | 128 |  |
|  |  |  | $5 \%-95 \%$ | - | 32 | 170 |  |
|  |  |  | $1 \%-99 \%$ | - | 49 | 265 |  |
|  |  |  | 32lsb | - | 57 | 284 |  |
|  |  |  | 1 lsb | - | 93 | 483 |  |
| $t_{\text {WAKEUP }}{ }^{(2)}$ | Wakeup time from off state (setting the ENx bit in the DAC Control register) until final value $\pm 1$ LSB | Normal mode CL $\leq 10 \mathrm{pF}$ |  | - | 1.4 | 3.5 | $\mu \mathrm{s}$ |
| PSRR | $\mathrm{V}_{\text {DDA }}$ supply rejection ratio | $\mathrm{V}_{\mathrm{DD}}>2.7 \mathrm{~V}$ |  | 65 | 85 | - | dB |
|  |  | $\mathrm{V}_{\mathrm{DD}}<2.7 \mathrm{~V}$ |  | 40 | 85 | - |  |
| $t_{\text {SAMP }}$ | Sampling time in sample and hold mode (code transition between the lowest input code and the highest input code when DACOUT reaches final value $\pm 1$ LSB) | - |  | - | 0.7 | - | $\mu \mathrm{s}$ |
| $\mathrm{Cl}_{\text {int }}$ | Internal sample and hold capacitor | - |  | - | 4 | 5 | pF |
| $\begin{gathered} \mathrm{dV} / \mathrm{dt} \text { (hold } \\ \text { phase) } \end{gathered}$ | Voltage decay rate in Sample and hold mode, during hold phase | $\begin{gathered} \mathrm{CSH}=4 \mathrm{pF} \\ \mathrm{~T}=55^{\circ} \mathrm{C} \end{gathered}$ |  | - | 50 | - | $\mathrm{mV} / \mathrm{ms}$ |
| $I_{\text {DDA }}(\text { DAC) }$ | DAC consumption from $\mathrm{V}_{\text {DDA }}$ | No load, middle code (0x800) |  | - | - | 0.2 | $\mu \mathrm{A}$ |
| $I_{\text {DDV }}(\text { DAC) }$ | DAC consumption from $\mathrm{V}_{\text {REF }+}$ | No load, middle code (0x800) ${ }^{(3)}$ |  | - | 720 | 955 |  |

1. Guaranteed by design.
2. In buffered mode, the output can overshoot above the final value for low input code (starting from min value).
3. Worst case consumption is at code $0 \times 800$.Table 71. DAC 15MSPS accuracy ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| DNL | Differential non linearity ${ }^{(2)}$ | - | -2 | - | 2 | LSB |
| INL | Integral non linearity ${ }^{(3)}$ | CL $\leq 50 \mathrm{pF}$, no RL | -5 | - | 5 |  |
| TUE | Total unadjusted error | CL $\leq 50 \mathrm{pF}$, no RL | -5 | - | 5 |  |
| DCS | Dynamic code spike | Spike amplitude on DAC voltage when <br> DAC output value is decreasing | - | 0 | 4 |  |

1. Guaranteed by design.
2. Difference between two consecutive codes - 1 LSB .
3. Difference between measured value at code $i$ and the value at code $i$ on a line drawn between code 0 and last code 4095. Offset error is included.# 5.3.20 Voltage reference buffer characteristics 

Table 72. VREFBUF characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{\text {DDA }}$ | Analog supply voltage | Normal mode | VRS $=00$ | 2.4 | - | 3.6 | V |
|  |  |  | VRS $=01$ | 2.8 | - | 3.6 |  |
|  |  |  | VRS $=10$ | 3.135 | - | 3.6 |  |
|  |  | Degraded mode ${ }^{(2)}$ | VRS $=00$ | 1.65 | - | 2.4 |  |
|  |  |  | VRS $=01$ | 1.65 | - | 2.8 |  |
|  |  |  | VRS $=10$ | 1.65 | - | 3.135 |  |
| $\begin{aligned} & \mathrm{V}_{\text {REFBUF_ }} \\ & \text { OUT } \end{aligned}$ | Voltage reference output | Normal mode | VRS $=00$ | 2.044 | 2.048 | 2.052 |  |
|  |  |  | VRS $=01$ | 2.496 | 2.5 | 2.504 |  |
|  |  |  | VRS $=10$ | 2.896 | 2.9 | 2.904 |  |
|  |  |  | VRS $=00$ | $\mathrm{V}_{\text {DDA }}-250 \mathrm{mV}$ | - | $\mathrm{V}_{\text {DDA }}$ |  |
|  |  |  | VRS $=01$ | $\mathrm{V}_{\text {DDA }}-250 \mathrm{mV}$ | - | $\mathrm{V}_{\text {DDA }}$ |  |
|  |  |  | VRS $=10$ | $\mathrm{V}_{\text {DDA }}-250 \mathrm{mV}$ | - | $\mathrm{V}_{\text {DDA }}$ |  |
| $\begin{aligned} & \mathrm{V}_{\text {REFOUT_ }} \\ & \text { TEMP } \end{aligned}$ | Voltage reference output spread over the temperature range | $\mathrm{V}_{\text {DDA }}=3 \mathrm{~V}$ |  | - | - | See <br> Figure 31, <br> Figure 32, <br> Figure 33 | mV |
| TRIM | Trim step resolution | - |  | - | $\pm 0.05$ | $\pm 0.1$ | \% |
| CL | Load capacitor | - |  | 0.5 | 1 | 1.5 | $\mu \mathrm{F}$ |
| esr | Equivalent Serial Resistor of Cload | - |  | - | - | 2 | $\Omega$ |
| $\mathrm{I}_{\text {load }}$ | Static load current | - |  | - | - | 6.5 | mA |
| $\mathrm{I}_{\text {line_reg }}{ }^{(3)}$ | Line regulation | - |  | - | 1000 | 2000 | $\mathrm{ppm} / \mathrm{V}$ |
| $\mathrm{I}_{\text {load_reg }}$ | Load regulation | $\begin{aligned} & 500 \mu \mathrm{~A} \leq \\ & \mathrm{I}_{\text {load }} \leq 4 \mathrm{~mA} \end{aligned}$ | Normal mode | - | 50 | 500 | $\begin{gathered} \text { ppm/m } \\ \text { A } \end{gathered}$ |
| $T_{\text {Coeff }}$ | Temperature coefficient | $-40^{\circ} \mathrm{C}<\mathrm{TJ}<+125^{\circ} \mathrm{C}$ |  | - | - | Tcoeff_vr <br> efint + <br> $50^{(4)}$ | $\mathrm{ppm} /{ }^{\circ} \mathrm{C}$ |
|  |  | $0^{\circ} \mathrm{C}<\mathrm{TJ}<+50^{\circ} \mathrm{C}$ |  | - | - |  |  |
| PSRR | Power supply rejection | DC |  | 40 | 55 | - | dB |
|  |  | 100 kHz |  | 25 | 40 | - |  |
| $\mathrm{t}_{\text {START }}$ | Start-up time | $\mathrm{CL}=0.5 \mu \mathrm{~F}^{(5)}$ |  | - | 300 | 350 | $\mu \mathrm{s}$ |
|  |  | $\mathrm{CL}=1.1 \mu \mathrm{~F}^{(5)}$ |  | - | 500 | 650 |  |
|  |  | $\mathrm{CL}=1.5 \mu \mathrm{~F}^{(5)}$ |  | - | 650 | 800 |  |Table 72. VREFBUF characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{I}_{\text {INRUSH }}$ | Control of maximum DC current drive on VREFBUF OUT during startup phase ${ }^{(6)}$ | - | - | 8 | - | mA |
| $\mathrm{I}_{\text {DDA } \text { (VREF }}$ <br> BUF) | VREFBUF consumption from $\mathrm{V}_{\text {DDA }}$ | $\mathrm{I}_{\text {load }}=0 \mu \mathrm{~A}$ | - | 16 | 25 | $\mu \mathrm{A}$ |
|  |  | $\mathrm{I}_{\text {load }}=500 \mu \mathrm{~A}$ | - | 18 | 30 |  |
|  |  | $\mathrm{I}_{\text {load }}=4 \mathrm{~mA}$ | - | 35 | 50 |  |
|  |  | $\mathrm{I}_{\text {load }}=6.5 \mathrm{~mA}$ | - | 45 | 80 |  |

1. Guaranteed by design, unless otherwise specified.
2. In degraded mode, the voltage reference buffer can not maintain accurately the output voltage which follows ( $\mathrm{V}_{\text {DDA }}$ - drop voltage).
3. Line regulation is given for overall supply variation, in normal mode.
4. Tcoeff_vrefint refer to Tcoeff parameter in the embedded voltage reference section.
5. The capacitive load must include a 100 nF low ESR capacitor in order to cut-off the high frequency noise.
6. To correctly control the VREFBUF inrush current during start-up phase and scaling change, the $\mathrm{V}_{\text {DDA }}$ voltage should be in the range $[2.4 \mathrm{~V}$ to 3.6 V$],[2.8 \mathrm{~V}$ to 3.6 V$]$ and $[3.135 \mathrm{~V}$ to 3.6 V$]$ respectively for VRS $=0,1$ and 2 .

Figure 31. $\mathbf{V}_{\text {REFOUT }}$ TEMP in case VRS $=00$
![img-31.jpeg](img-31.jpeg)Figure 32. $\mathrm{V}_{\text {REFOUT TEMP }}$ in case VRS $=01$
![img-32.jpeg](img-32.jpeg)

Figure 33. $\mathrm{V}_{\text {REFOUT TEMP }}$ in case VRS $=10$
![img-33.jpeg](img-33.jpeg)# 5.3.21 Comparator characteristics 

Table 73. COMP characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{D D A}$ | Analog supply voltage | - | 1.62 | - | 3.6 | V |
| $V_{\text {IN }}$ | Comparator input voltage range | - | 0 | - | $V_{\text {DDA }}$ | V |
| $V_{B C}{ }^{(2)}$ | Scaler input voltage | - | VREFINT |  |  |  |
| $V_{S C}{ }^{(3)}$ | Scaler offset voltage | - | - | $\pm 5$ | $\pm 10$ | mV |
| $\mathrm{I}_{\text {DDA }}(\text { SCALER }) \text { }$ | Scaler static consumption from $V_{\text {DDA }}$ | BRG_EN=0 (bridge disable) | - | 200 | 300 | nA |
|  |  | BRG_EN=1 (bridge enable) | - | 0.8 | 1 | $\mu \mathrm{A}$ |
| $\mathrm{t}_{\text {START }}$ SCALER | Scaler startup time | - | - | 100 | 200 | $\mu \mathrm{s}$ |
| $t_{\text {START }}$ | Comparator startup time to reach propagation delay specification | - | - | - | 5 | $\mu \mathrm{s}$ |
| $t_{D}{ }^{(4)}$ | Propagation delay for 200 mV step with 100 mV overdrive | 50pF load on output | $\begin{aligned} & V_{D D A}<2.7 \mathrm{~V} \\ & V_{D D A} \geq 2.7 \mathrm{~V} \end{aligned}$ | - | - | 35 | ns |
|  |  |  |  | - | 16.7 | 31 | ns |
| $V_{\text {offset }}{ }^{(3)}$ | Comparator offset error | Full $V_{\text {DDA }}$ voltage range, full temperature range |  | $-9$ | $-6 /+2$ | 3 | mV |
| $V_{\text {hys }}$ | Comparator hysteresis | HYST[2:0] = 0 |  | - | 0 | - | mV |
|  |  | HYST[2:0] =1 |  | 4 | 9 | 16 |
|  |  | HYST[2:0] = 2 |  | 7 | 18 | 32 |
|  |  | HYST[2:0] = 3 |  | 11 | 27 | 47 |
|  |  | HYST[2:0] = 4 |  | 15 | 36 | 63 |
|  |  | HYST[2:0] = 5 |  | 19 | 45 | 79 |
|  |  | HYST[2:0] = 6 |  | 23 | 54 | 95 |
|  |  | HYST[2:0] = 7 |  | 26 | 63 | 110 |
| $\mathrm{I}_{\text {DDA }}$ (COMP) | Comparator consumption from $V_{\text {DDA }}$ | Static |  | - | 450 | 720 | $\mu \mathrm{A}$ |
|  |  | With $50 \mathrm{kHz} \pm 100 \mathrm{mV}$ overdrive square signal |  | - | 450 | - |

1. Guaranteed by design, unless otherwise specified.
2. Refer to Table 20: Embedded internal voltage reference.
3. Guaranteed by characterization results.
4. Typical value (3V) is an average for all comparators propagation delay.# 5.3.22 Operational amplifiers characteristics 

Table 74. OPAMP characteristics ${ }^{(1)(2)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {DDA }}$ | Analog supply voltage | - | 2 | 3.3 | 3.6 | V |
| CMIR | Common mode input range | - | 0 | - | $V_{D D A}$ | V |
| VI $_{\text {OFFSET }}$ | Input offset voltage | $25^{\circ} \mathrm{C}$, No Load on output. | - | - | $\pm 1.5$ | mV |
|  |  | All voltage/temperature. | - | - | $\pm 3$ |  |
| $\Delta \mathrm{VI}_{\text {OFFSET }}$ | Input offset voltage drift | - | - | $\pm 10$ | - | $\mu \mathrm{V} /{ }^{\circ} \mathrm{C}$ |
| TRIMOFFSE <br> TP | Offset trim step at low common input voltage $\left(0.1 \times V_{D D A}\right)$ | - | - | 1.1 | 1.2 | mV |
| TRIMOFFSE <br> TN | Offset trim step at high common input voltage $\left(0.9 \times V_{D D A}\right)$ | - | - | 1.3 | 1.65 |  |
| $\mathrm{I}_{\text {LOAD }}$ | Drive current | - | - | - | 500 | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {LOAD_PGA }}$ | Drive current in PGA mode | - | - | - | 270 |  |
| $\mathrm{C}_{\text {LOAD }}$ | Capacitive load | - | - | - | 50 | pF |
| CMRR | Common mode rejection ratio | - | - | 60 | - | dB |
| PSRR | Power supply rejection ratio | $\begin{aligned} & \mathrm{C}_{\mathrm{LOAD}} \leq 50 \mathrm{pf} \\ & \mathrm{R}_{\mathrm{LOAD}} \geq 4 \mathrm{k} \Omega \mathrm{DC} \mathrm{Vcom=} \mathrm{V}_{\mathrm{DDA}} / 2 \end{aligned}$ | - | 80 | - | dB |
| GBW | Gain Bandwidth Product | $\begin{aligned} & 100 \mathrm{mV} \leq \text { Output dynamic range } \leq \mathrm{V}_{\text {DDA }} \\ & 100 \mathrm{mV} \end{aligned}$ | 7 | 13 | - | MHz |
| $\mathrm{SR}^{(3)}$ | Slew rate (from 10 and $90 \%$ of output voltage) | Normal mode | 2.5 | 6.5 | - | $\mathrm{V} / \mu \mathrm{s}$ |
|  |  | High-speed mode | 18 | 45 | - |  |
| AO | Open loop gain | $100 \mathrm{mV} \leq$ Output dynamic range $\leq \mathrm{V}_{\text {DDA }}$ 100 mV | 65 | 95 | - | dB |
|  |  | $200 \mathrm{mV} \leq$ Output dynamic range $\leq \mathrm{V}_{\text {DDA }}$ 200 mV | 75 | 95 | - |  |
| $\mathrm{V}_{\text {OHSAT }}{ }^{(3)}$ | High saturation voltage | $\mathrm{I}_{\text {load }}=\max$ or $\mathrm{R}_{\text {load }}=\min$ Input at $\mathrm{V}_{\text {DDA }}$ Follower mode | $\begin{aligned} & \mathrm{V}_{\text {DDA }} \\ & -100 \end{aligned}$ | - | - | mV |
| $\mathrm{V}_{\text {OLSAT }}{ }^{(3)}$ | Low saturation voltage | $\mathrm{I}_{\text {load }}=\max$ or $\mathrm{R}_{\text {load }}=\min$ Input at 0 . <br> Follower mode | - | - | 100 |  |
| $\varphi_{\mathrm{m}}$ | Phase margin | Follower mode, Vcom=V DDA $/ 2$ | - | 65 | - | ${ }^{\circ}$ |
| GM | Gain margin | Follower mode, Vcom=V DDA $/ 2$ | - | 10 | - | dB |Table 74. OPAMP characteristics ${ }^{(1)(2)}$ (continued)

| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| IWAKEUP | Wake up time from OFF state. | Normal mode | $\begin{aligned} & \mathrm{C}_{\mathrm{LOAD}} \leq 50 \mathrm{pf}, \\ & \mathrm{R}_{\mathrm{LOAD}} \geq 4 \mathrm{k} \Omega \\ & \text { follower } \\ & \text { configuration } \end{aligned}$ | - | 3 | 6 | $\mu \mathrm{s}$ |
|  |  | High-speed mode | $\begin{aligned} & \mathrm{C}_{\mathrm{LOAD}} \leq 50 \mathrm{pf}, \\ & \mathrm{R}_{\mathrm{LOAD}} \geq \\ & 20 \mathrm{k} \Omega \\ & \text { follower } \\ & \text { configuration } \end{aligned}$ | - | 3 | 6 |  |
| $I_{\text {bias }}$ | OPAMP input bias current | See $\mathrm{I}_{\text {leak }}$ parameter in Table 53: I/O static characteristics for given pin. |  |  |  |  |  |
| PGA gain | Non inverting gain value ${ }^{(4)}$ | PGA Gain $=20.1 \leq$ Out dynamic range $\leq V_{D D A}-$ 0.1 | $\mathrm{V}_{\mathrm{DDA}}<2.2$ | $-2$ | - | 2 | \% |
|  |  |  | $V_{D D A} \geq 2.2$ | $-1$ | - | 1 |  |
|  |  | PGA Gain=4, $100 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-100 \mathrm{mV}$ |  | $-1$ | - | 1 |  |
|  |  | PGA Gain=8 $100 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-100 \mathrm{mV}$ |  | $-1$ | - | 1 |  |
|  |  | PGA Gain=16, $100 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-100 \mathrm{mV}$ |  | $-1$ | - | 1 |  |
|  |  | PGA Gain=32 $200 \mathrm{mV} \leq$ Output $\leq V_{D D A}-$ 200mV |  | $-2$ | - | 2 |  |
|  |  | PGA Gain=64 $200 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-200 \mathrm{mV}$ |  | $-2$ | - | 2 |  |
|  | Inverting gain value | PGA Gain $=-1$ | $V_{D D A}<2.2$ | $-2$ | - | 2 | \% |
|  |  | $100 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-100 \mathrm{mV}$ | $V_{D D A} \geq 2.2$ | $-1$ | - | 1 |  |
|  |  | PGA Gain=-3, $100 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-100 \mathrm{mV}$ |  | $-1$ | - | 1 |  |
|  |  | PGA Gain=-7 $100 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-100 \mathrm{mV}$ |  | $-1$ | - | 1 |  |
|  |  | PGA Gain=-15, $100 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-100 \mathrm{mV}$ |  | $-1$ | - | 1 |  |
|  |  | PGA Gain=-31 $200 \mathrm{mV} \leq$ Output $\leq V_{D D A}-$ 200mV |  | $-2$ | - | 2 |  |
|  |  | PGA Gain=-63 $200 \mathrm{mV} \leq$ Output dynamic range $\leq V_{D D A}-200 \mathrm{mV}$ |  | $-5$ | - | 2 |  |Table 74. OPAMP characteristics ${ }^{(1)(2)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $R_{\text {network }}$ | R2/R1 internal resistance values in non-inverting PGA mode ${ }^{(5)}$ | PGA Gain $=2$ | - | $10 / 10$ | - | $\mathrm{k} \Omega / \mathrm{k}$ |
|  |  | PGA Gain $=4$ | - | $30 / 10$ | - |  |
|  |  | PGA Gain $=8$ | - | $70 / 10$ | - |  |
|  |  | PGA Gain $=16$ | - | $150 / 10$ | - |  |
|  |  | PGA Gain $=32$ | - | $310 / 10$ | - |  |
|  |  | PGA Gain $=64$ | - | $630 / 10$ | - |  |
|  | R2/R1 internal resistance values in inverting PGA mode ${ }^{(5)}$ | PGA Gain $=-1$ | - | $10 / 10$ | - |  |
|  |  | PGA Gain $=-3$ | - | $30 / 10$ | - |  |
|  |  | PGA Gain $=-7$ | - | $70 / 10$ | - |  |
|  |  | PGA Gain $=-15$ | - | $150 / 10$ | - |  |
|  |  | PGA Gain $=-31$ | - | $310 / 10$ | - |  |
|  |  | PGA Gain $=-63$ | - | $630 / 10$ | - |  |
| Delta R | Resistance variation (R1 or R2) | - | $-15$ | - | $+15$ | \% |
| PGA BW | PGA bandwidth for different non inverting gain | Gain $=2$ | - | GBW/2 | - | MHz |
|  |  | Gain $=4$ | - | GBW/4 | - |  |
|  |  | Gain $=8$ | - | GBW/8 | - |  |
|  |  | Gain $=16$ | - | GBW/16 | - |  |
|  |  | Gain $=32$ | - | GBW/32 | - |  |
|  |  | Gain $=64$ | - | GBW/64 | - |  |
|  | PGA bandwidth for different inverting gain | Gain $=-1$ | - | GBW/2 | - | MHz |
|  |  | Gain $=-3$ | - | GBW/4 | - |  |
|  |  | Gain $=-7$ | - | GBW/8 | - |  |
|  |  | Gain $=-15$ | - | GBW/16 | - |  |
|  |  | Gain $=-31$ | - | GBW/32 | - |  |
|  |  | Gain $=-63$ | - | GBW/64 | - |  |
| eN | Voltage noise density | at 1 kHz , Output loaded with $4 \mathrm{k} \Omega$ |  | - | 250 | - | $\begin{gathered} \mathrm{nV} / \mathrm{V} \\ \mathrm{~Hz} \end{gathered}$ |
|  |  | at 10 kHz , Output loaded with $4 \mathrm{k} \Omega$ |  | - | 90 | - |  |
| $\mathrm{I}_{\text {DDA }}$ (OPAMP) | OPAMP consumption from $\mathrm{V}_{\text {DDA }}$ | Normal mode | No load, follower mode | - | 1.3 | 2.2 | mA |
|  |  | High-speed mode |  | - | 1.4 | 2.6 |  |
| $\begin{aligned} & \mathrm{T}_{\mathrm{S} \text { _OPAMP_VO }} \\ & \text { UT } \end{aligned}$ | ADC sampling time when reading the OPAMP output. OPAINTOEN $=1$ | $\mathrm{V}_{\text {DDA }}<2 \mathrm{~V}$ |  | 300 | - | - | ns |
|  |  | $\mathrm{V}_{\text {DDA }} \geq 2 \mathrm{~V}$ |  | 200 | - | - |  |
| $\mathrm{I}_{\text {DDA }}$ (OPAMPI NT) | OPAMP consumption from $\mathrm{V}_{\text {DDA- }}$ OPAINTOEN $=1$ | Normal mode | no load, follower mode | - | 0.45 | 0.7 | mA |
|  |  | High-speed mode |  | - | 0.5 | 0.8 |  |1. Guaranteed by design, unless otherwise specified.
2. Data guaranteed on normal and high speed mode unless otherwise specified.
3. Guaranteed by characterization results.
4. Valid also for inverting gain configuration with external bias.
5. R2 is the internal resistance between OPAMP output and OPAMP inverting input. R1 is the internal resistance between OPAMP inverting input and ground. The PGA gain $=1+R 2 / R 1$

Figure 34. OPAMP noise density @ $25^{\circ} \mathrm{C}$
![img-34.jpeg](img-34.jpeg)# 5.3.23 Temperature sensor characteristics 

Table 75. TS characteristics

| Symbol | Parameter | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: |
| $\mathrm{T}_{\mathrm{L}}{ }^{(1)}$ | $\mathrm{V}_{\text {TS }}$ linearity with temperature | - | $\pm 1$ | $\pm 2$ | ${ }^{\circ} \mathrm{C}$ |
| Avg_Slope ${ }^{(1)}$ | Average slope | 2.3 | 2.5 | 2.7 | $\mathrm{mV} /{ }^{\circ} \mathrm{C}$ |
| $\mathrm{V}_{30}$ | Voltage at $30^{\circ} \mathrm{C}\left( \pm 5^{\circ} \mathrm{C}\right)^{(2)}$ | 0.742 | 0.76 | 0.785 | V |
| $\mathrm{t}_{\text {START-RUN }}{ }^{(1)}$ | Start-up time in Run mode (start-up of buffer) | - | 8 | 15 | $\mu \mathrm{s}$ |
| $\mathrm{t}_{\text {START_CONT }}{ }^{(3)}$ | Start-up time when entering in continuous <br> mode | - | 70 | 120 | $\mu \mathrm{s}$ |
| $\mathrm{t}_{\mathrm{S} \text { temp }^{(1)}}$ | ADC sampling time when reading the <br> temperature | 5 | - | - | $\mu \mathrm{s}$ |
| $\mathrm{t}_{\mathrm{DD}}(\mathrm{TS})^{(1)}$ | Temperature sensor consumption from VDD, <br> when selected by ADC | - | 4.7 | 7 | $\mu \mathrm{A}$ |

1. Guaranteed by design.
2. Measured at $\mathrm{V}_{\mathrm{DDA}}=3.0 \mathrm{~V} \pm 10 \mathrm{mV}$. The $\mathrm{V}_{30}$ ADC conversion result is stored in the TS_CAL1 byte. Refer to Table 5: Temperature sensor calibration values.
3. Continuous mode means RUN mode or Temperature Sensor ON.

### 5.3.24 $\mathrm{V}_{\text {BAT }}$ monitoring characteristics

Table 76. $\mathrm{V}_{\text {BAT }}$ monitoring characteristics ${ }^{(1)}$

| Symbol | Parameter | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: |
| R | Resistor bridge for $\mathrm{V}_{\mathrm{BAT}}$ | - | $3 \times 39$ | - | k $\Omega$ |
| Q | Ratio on $\mathrm{V}_{\mathrm{BAT}}$ measurement | - | 3 | - | - |
| $\mathrm{Er}^{(2)}$ | Error on Q | -10 | - | 10 | $\%$ |
| $\mathrm{t}_{\mathrm{S} \text { vbat }^{(2)}}$ | ADC sampling time when reading the $\mathrm{V}_{\mathrm{BAT}}$ | 12 | - | - | $\mu \mathrm{s}$ |

1. $1.55 \mathrm{~V}<\mathrm{V}_{\mathrm{BAT}}<3.6 \mathrm{~V}$.
2. Guaranteed by design.

Table 77. $\mathrm{V}_{\text {BAT }}$ charging characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :-- | :-- | :--: | :--: | :--: | :--: |
| $\mathrm{R}_{\mathrm{BC}}$ | Battery <br> charging <br> resistor | VBRS $=0$ | - | 5 | - | k $\Omega$ |
| VBRS $=1$ | - | 1.5 | - |  |  |# 5.3.25 Timer characteristics 

The parameters given in the following tables are guaranteed by design.
Refer to Section 5.3.14: I/O port characteristics for details on the input/output alternate function characteristics (output compare, input capture, external clock, PWM output).

Table 78. TIMx ${ }^{(1)}$ characteristics ${ }^{(2)}$

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {res(TIM) }}$ | Timer resolution time | - | 1 | - | $t_{\text {TIMxCLK }}$ |
|  |  | $t_{\text {TIMxCLK }}=170 \mathrm{MHz}$ | 5.88 | - | ns |
| $t_{\text {EXT }}$ | Timer external clock frequency on CH1 to CH4 | - | 0 | $t_{\text {TIMxCLK }} / 2$ | MHz |
|  |  | $t_{\text {TIMxCLK }}=170 \mathrm{MHz}$ | 0 | 85 | MHz |
| $\operatorname{Res}_{\text {TIM }}$ | Timer resolution | TIMx (except TIM2) | - | 16 | bit |
|  |  | TIM2 | - | 32 |  |
| $t_{\text {COUNTER }}$ | 16-bit counter clock period | - | 1 | 65536 | $t_{\text {TIMxCLK }}$ |
|  |  | $t_{\text {TIMxCLK }}=170 \mathrm{MHz}$ | 0.00588 | 385.5 | $\mu \mathrm{s}$ |
| $t_{\text {MAX_COUNT }}$ | Maximum possible count with 32-bit counter | - | - | $65536 \times 65536$ | $t_{\text {TIMxCLK }}$ |
|  |  | $t_{\text {TIMxCLK }}=170 \mathrm{MHz}$ | - | 25.26 | s |
| $t_{\text {ENC }}$ | Encoder frequency on TI1 and TI2 input pins | - | 0 | $t_{\text {TIMxCLK }} / 4$ | MHz |
|  |  | $t_{\text {TIMxCLK }}=170 \mathrm{MHz}$ | 0 | 42.5 | MHz |
| $t_{W(1)$ nDEX) | Index pulsewidth on ETR input | - | 2 | - | Tck |
| $t_{W(T I 1, T I 2)}$ | Min pulsewidth on TI1 and TI2 inputs in all encoder modes except directional clock x1 | - | 2 | - | Tck |
|  | Min pulsewidth on TI1 and TI2 inputs in directional clock x1 | - | 3 | - | Tck |

1. TIMx is used as a general term in which $x$ stands for $1,2,3,4,6,7,8,15,16$, or 17 .
2. Guaranteed by design.Table 79. IWDG min/max timeout period at $32 \mathrm{kHz}(\mathrm{LSI})^{(1)(2)}$

| Prescaler divider | PR[2:0] bits | Min timeout RL[11:0]= <br> $\mathbf{0 x 0 0 0}$ | Max timeout RL[11:0]= <br> $\mathbf{0 x F F F}$ | Unit |
| :--: | :--: | :--: | :--: | :--: |
| $14$ | 0 | 0.125 | 512 | ms |
| $18$ | 1 | 0.250 | 1024 |  |
| $116$ | 2 | 0.500 | 2048 |  |
| $132$ | 3 | 1.0 | 4096 |  |
| $164$ | 4 | 2.0 | 8192 |  |
| $1128$ | 5 | 4.0 | 16384 |  |
| $1256$ | 6 or 7 | 8.0 | 32768 |  |

1. Guaranteed by design.
2. The exact timings still depend on the phasing of the APB interface clock versus the LSI clock so that there is always a full RC period of uncertainty.

Table 80. WWDG min/max timeout value at $170 \mathrm{MHz}(\mathrm{PCLK})^{(1)}$

| Prescaler | WDGTB | Min timeout value | Max timeout value | Unit |
| :--: | :--: | :--: | :--: | :--: |
| 1 | 0 | 0.0241 | 1.542 | ms |
| 2 | 1 | 0.0482 | 3.084 |  |
| 4 | 2 | 0.0964 | 6.168 |  |
| 8 | 3 | 0.1928 | 12.336 |  |

1. Guaranteed by design.

# 5.3.26 Communication interfaces characteristics 

## $I^{2} \mathrm{C}$ interface characteristics

The I2C interface meets the timings requirements of the $\mathrm{I}^{2} \mathrm{C}$-bus specification and user manual rev. 03 for:

- Standard-mode (Sm): with a bit rate up to $100 \mathrm{kbit} / \mathrm{s}$
- Fast-mode (Fm): with a bit rate up to $400 \mathrm{kbit} / \mathrm{s}$
- Fast-mode Plus (Fm+): with a bit rate up to $1 \mathrm{Mbit} / \mathrm{s}$.

The I2C timings requirements are guaranteed by design when the I2C peripheral is properly configured (refer to reference manual RM0440 "STM32G4 Series advanced Arm ${ }^{\circledR}$-based 32-bit MCUs") and when the I2CCLK frequency is greater than the minimum shown in the table below.Table 81. Minimum I2CCLK frequency in all I2C modes

| Symbol | Parameter | Condition |  | Min | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $f(I 2 C C L K)$ | I2CCLK <br> frequency | Standard mode |  | 2 | MHz |
|  |  | Fast-mode | Analog Filtre ON <br> DNF $=0$ | 8 |  |
|  |  |  | Analog Filtre OFF <br> DNF $=1$ | 9 |  |
|  |  | Fast-mode <br> Plus | Analog Filtre ON <br> DNF $=0$ | 17 |  |
|  |  |  | Analog Filtre OFF <br> DNF $=1$ | 16 |  |

The SDA and SCL I/O requirements are met with the following restrictions:

- The SDA and SCL I/O pins are not "true" open-drain. When configured as open-drain, the PMOS connected between the I/O pin and $\mathrm{V}_{\text {DDIOx }}$ is disabled, but is still present.
- The 20 mA output drive requirement in Fast-mode Plus is supported partially. This limits the maximum load Cload supported in Fm+, which is given by these formulas:
- $t_{r}(\mathrm{SDA} / \mathrm{SCL})=0.8473 \times \mathrm{R}_{\mathrm{p}} \times \mathrm{C}_{\text {load }}$
- $\quad R_{p}(\min )=\left(V_{D D}-V_{O L}(\max )\right) / I_{O L}(\max )$

Where Rp is the I2C lines pull-up. Refer to Section 5.3.14: I/O port characteristics for the I2C I/Os characteristics.

All I2C SDA and SCL I/Os embed an analog filter. Refer to Table 82 below for the analog filter characteristics:

Table 82. I2C analog filter characteristics ${ }^{(1)}$

| Symbol | Parameter | Min | Max | Unit |
| :--: | :-- | :--: | :--: | :--: |
| $\mathrm{t}_{\mathrm{AF}}$ | Maximum pulse width of spikes that <br> are suppressed by the analog filter | $50^{(2)}$ | $90^{(3)}$ | ns |

1. Guaranteed by design.
2. Spikes with widths below $\mathrm{t}_{\mathrm{AF}(\min )}$ are filtered.
3. Spikes with widths above $\mathrm{t}_{\mathrm{AF}(\max )}$ are not filtered

# SPI characteristics 

Unless otherwise specified, the parameters given in Table 83 for SPI are derived from tests performed under the ambient temperature, $\mathrm{f}_{\mathrm{PCLK} \mathrm{K}}$ frequency and supply voltage conditions summarized in Table 17: General operating conditions.

- Output speed is set to OSPEEDRy[1:0] = 11
- Capacitive load $\mathrm{C}=30 \mathrm{pF}$
- Measurement points are done at CMOS levels: $0.5 \times \mathrm{V}_{\mathrm{DD}}$

Refer to Section 5.3.14: I/O port characteristics for more details on the input/output alternate function characteristics (NSS, SCK, MOSI, MISO for SPI).Table 83. SPI characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max ${ }^{(2)}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\begin{aligned} & t_{\text {SCK }} \\ & 1 / t_{\text {C( } S C K)} \end{aligned}$ | SPI clock frequency | Master mode $2.7 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ <br> Voltage Range V1 | - | - | 75 | MHz |
|  |  | Master mode <br> $1.71 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ <br> Voltage Range V1 |  |  | 50 |  |
|  |  | Master transmitter mode $1.71 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ Voltage Range V1 |  |  | 50 |  |
|  |  | Slave receiver mode $1.71 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ Voltage Range V1 |  |  | 50 |  |
|  |  | Slave mode transmitter/full duplex $2.7 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ Voltage Range V1 |  |  | 41 |  |
|  |  | Slave mode transmitter/full duplex $1.71 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ Voltage Range V1 |  |  | 27 |  |
|  |  | $1.71 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ Voltage Range V2 |  |  | 13 |  |
| $t_{\text {su(NSS) }}$ | NSS setup time | Slave mode | $4^{*} T_{\text {pclk }}$ | - | - | - |
| $t_{h(N S S)}$ | NSS hold time | Slave mode | $2^{*} T_{\text {pclk }}$ | - | - | - |
| $\begin{aligned} & t_{w(S C K H)} \\ & t_{w(S C K L)} \end{aligned}$ | SCK high and low time | Master mode, SPI prescaler $=2$ | $T_{\text {pclk }}-1$ | $T_{\text {pclk }}$ | $T_{\text {pclk }}+1$ | ns |
| $t_{\text {su(MI) }}$ | Data input setup time | Master mode | 4 | - | - | ns |
| $t_{\text {su(SI) }}$ |  | Slave mode | 3 | - | - |  |
| $t_{h( }^{(s)}$ | Data input hold time | Master mode | 4 | - | - | ns |
| $t_{h(S)}$ |  | Slave mode | 1 | - | - |  |
| $t_{a(S O)}$ | Data output access time | Slave mode | 9 | - | 34 | ns |
| $t_{\text {dis(SO) }}$ | Data output disable time | Slave mode | 9 | - | 16 | ns |Table 83. SPI characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max ${ }^{(2)}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{v(S O)}$ | Data output valid time | Slave mode <br> $2.7 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ <br> Voltage Range V1 | - | 9 | 12 | ns |
|  |  | Slave mode <br> $1.71 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ <br> Voltage Range V1 | - | 9 | 18 |  |
|  |  | Slave mode <br> $1.71 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ <br> Voltage Range V2 | - | 13 | 22 |  |
| $t_{v(M O)}$ |  | Master mode | - | 3.5 | 4.5 |  |
| $t_{h(S O)}$ | Data output hold time | Slave mode $1.71 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | 6 | - | - |  |
|  |  | Slave mode Range V2 | 9 | - | - |  |
| $t_{h(M O)}$ |  | Master mode | 2 | - | - |  |

1. Guaranteed by characterization results.
2. The maximum frequency in Slave transmitter mode is determined by the sum of $t_{v}(\mathrm{SO})$ and tsu(MI) which has to fit into SCK low or high-phase preceding the SCK sampling edge. This value can be achieved when the SPI communicates with a master having tsu(MI) $=0$ while Duty(SCK) $=50 \%$.

Figure 35. SPI timing diagram - slave mode and CPHA $=0$
![img-35.jpeg](img-35.jpeg)Figure 36. SPI timing diagram - slave mode and CPHA = 1
![img-36.jpeg](img-36.jpeg)

1. Measurement points are done at CMOS levels: $0.3 \mathrm{~V}_{\mathrm{DD}}$ and $0.7 \mathrm{~V}_{\mathrm{DD}}$.

Figure 37. SPI timing diagram - master mode
![img-37.jpeg](img-37.jpeg)

1. Measurement points are done at CMOS levels: $0.3 \mathrm{~V}_{\mathrm{DD}}$ and $0.7 \mathrm{~V}_{\mathrm{DD}}$.# I2S characteristics 

Unless otherwise specified, the parameters given in Table 84 for I2S are derived from tests performed under the ambient temperature, $\mathrm{f}_{\mathrm{PCLKx}}$ frequency and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 17: General operating conditions, with the following configuration:

- Output speed is set to OSPEEDRy[1:0] = 10
- Capacitive load $\mathrm{C}=30 \mathrm{pF}$
- Measurement points are done at CMOS levels: $0.5 \mathrm{~V}_{\mathrm{DD}}$

Refer to Section 5.3.14: I/O port characteristics for more details on the input/output alternate function characteristics (CK,SD,WS).

Table 84. I2S characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {MCLK }}$ | I2S Main clock output | - | $256 \times 8 \mathrm{~K}$ | $256^{\circ} \mathrm{Fs}^{(2)}$ | MHz |
| $f_{C K}$ | I2S clock frequency | Master data | - | $64 \times F s$ | MHz |
|  |  | Slave data | - | $64 \times F s$ |  |
| $D_{C K}$ | I2S clock frequency duty cycle | Slave receiver | 30 | 70 | $\%$ |
| $t_{v(\text { WS) }}$ | WS valid time | Master mode | - | 6 | ns |
| $t_{h(\text { WS) }}$ | WS hold time | Master mode | 3 | - |  |
|  |  | Slave mode | 2 | - |  |
| $t_{\text {su(WS) }}$ | WS setup time | Slave mode | 4 | - |  |
| $t_{\text {su(SD_MR) }}$ | Data input setup time | Master receiver | 3 | - |  |
| $t_{\text {su(SD_SR) }}$ |  | Slave receiver | 4 | - |  |
| $t_{h(\text { SD_MR) }}$ | Data input hold time | Master receiver | 4 | - |  |
| $t_{h(\text { SD_SR) }}$ |  | Slave receiver | 2 | - |  |
| $t_{v(\text { SD_ST) }}$ | Data output valid time | Slave transmitter (after enable edge) | $2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 15 |  |
|  |  |  | $1.65 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 22 |  |
| $t_{v(\text { SD_MT) }}$ |  | Master transmitter (after enable edge) |  | - | 3 |  |
| $t_{h(\text { SD_ST) }}$ | Data output hold time | Slave transmitter (after enable edge) |  | 7 | - |  |
| $t_{h(\text { SD_MT) }}$ |  | Master transmitter (after enable edge) |  | 1 | - |  |

1. Guaranteed by characterization results, not tested in production.
2. 256xFs maximum is 49.152 MHz .

Note: $\quad$ Refer to the reference manual RM0440 "STM32G4 Series advanced Arm ${ }^{\circledR}$-based 32-bit MCUs" I2S section for more details about the sampling frequency (Fs), $f_{M C K}, f_{C K}, D_{C K}$ values reflect only the digital peripheral behavior, source clock precision might slightly change the values $D_{C K}$ depends mainly on ODD bit value. Digital contribution leads to a min of (I2SDIV/(2*I2SDIV+ODD) and a max (I2SDIV+ODD)/(2*I2SDIV+ODD) and Fs max supported for each mode/condition.# SAI characteristics 

Unless otherwise specified, the parameters given in Table 85 for SAI are derived from tests performed under the ambient temperature, $\mathrm{f}_{\text {PCLKx }}$ frequency and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 17: General operating conditions, with the following configuration:

- Output speed is set to OSPEEDRy $[1: 0]=10$
- Capacitive load $\mathrm{C}=30 \mathrm{pF}$
- Measurement points are done at CMOS levels: $0.5 \times \mathrm{V}_{\mathrm{DD}}$

Refer to Section 5.3.14: I/O port characteristics for more details on the input/output alternate function characteristics (CK,SD,FS).Table 85. SAI characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {MCLK }}$ | SAI Main clock output | - | - | 50 | MHz |
| $f_{C K}$ | SAI clock frequency ${ }^{(2)}$ | Master transmitter $2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ <br> Voltage Range 1 | - | 33 | MHz |
|  |  | Master transmitter $1.71 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ Voltage Range 1 | - | 22 |  |
|  |  | Master receiver Voltage Range 1 | - | 22 |  |
|  |  | Slave transmitter $2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ Voltage Range 1 | - | 45 |  |
|  |  | Slave transmitter $1.71 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ Voltage Range 1 | - | 29 |  |
|  |  | Slave receiver Voltage Range 1 | - | 50 |  |
|  |  | Slave transmitter Voltage Range 2 | - | 13 |  |
| $t_{v(F S)}$ | FS valid time | Master mode $2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 15 | ns |
|  |  | Master mode $1.71 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 22 |  |
| $t_{h(F S)}$ | FS hold time | Master mode | 10 | - | ns |
| $t_{\text {su(FS) }}$ | FS setup time | Slave mode | 2 | - | ns |
| $t_{h(F S)}$ | FS hold time | Slave mode | 1 | - | ns |
| $t_{\text {su(SD_A_MR) }}$ | Data input setup time | Master receiver | 2.5 | - | ns |
| $t_{\text {su(SD_B_SR) }}$ |  | Slave receiver | 1 | - |  |
| $t_{h(S D A M R)}$ | Data input hold time | Master receiver | 5 | - | ns |
| $t_{h(S D B S R)}$ |  | Slave receiver | 1 | - |  |
| $t_{v(S D B S T)}$ | Data output valid time | Slave transmitter (after enable edge) $2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 11 | ns |
|  |  | Slave transmitter (after enable edge) $1.71 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 17 |  |
|  |  | Slave transmitter (after enable edge) voltage range V2 | - | 20 |  |
| $t_{h(S D B S T)}$ | Data output hold time | Slave transmitter (after enable edge) | 10 | - | ns |Table 85. SAI characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{v(S D, A, M T)}$ | Data output valid time | Master transmitter (after enable edge) $2.7 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 14 | ns |
|  |  | Master transmitter (after enable edge) $1.71 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ | - | 21 |  |
| $t_{h(S D, A, M T)}$ | Data output hold time | Master transmitter (after enable edge) | 10 | - | ns |

1. Guaranteed by characterization results.
2. APB clock frequency must be at least twice SAI clock frequency.

Figure 38. SAI master timing waveforms
![img-38.jpeg](img-38.jpeg)

Figure 39. SAI slave timing waveforms
![img-39.jpeg](img-39.jpeg)

# CAN (controller area network) interface 

Refer to Section 5.3.14: I/O port characteristics for more details on the input/output alternate function characteristics (FDCANx_TX and FDCANx_RX).# USB characteristics 

The device USB interface is fully compliant with the USB specification version 2.0 and is USB-IF certified (for Full-speed device operation).

Table 86. USB electrical characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{DD}}$ | USB transceiver operating voltage |  | $3.0^{(2)}$ | - | 3.6 | V |
| $\mathrm{t}_{\text {Crystal_less }}$ | USB crystal less operation temperature |  | -15 | - | 85 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{R}_{\text {PUI }}$ | Embedded USB_DP pull-up value during idle |  | 900 | 1250 | 1500 | $\Omega$ |
| $\mathrm{R}_{\text {PUR }}$ | Embedded USB_PD pull-up value during reception |  | 1400 | 2300 | 3200 |  |
| $\mathrm{Z}_{\text {sDRV }}{ }^{(3)}$ | Output driver impedance ${ }^{(4)}$ | Driving high and low | 28 | 36 | 44 | $\Omega$ |

1. $\mathrm{TA}=-40$ to $125^{\circ} \mathrm{C}$ unless otherwise specified.
2. The device USB functionality is ensured down to 2.7 V but not the full USB electrical characteristics, which are degraded in the 2.7 -to-3.0 V voltage range.
3. Guarantee by design.
4. No external termination series resistors are required on USB_PD (D+) and USB_DM (D-); the matching impedance is already included in the embedded driver.

## USART interface characteristics

Unless otherwise specified, the parameters given in Table 87 for USART are derived from tests performed under the ambient temperature, $\mathrm{f}_{\mathrm{PCLKX}}$ frequency and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 87, with the following configuration:

- Output speed is set to OSPEEDRy[1:0] = 10
- Capacitive load $\mathrm{C}=30 \mathrm{pF}$
- Measurement points are done at CMOS levels: $0.5 \mathrm{~V}_{\mathrm{DD}}$

Refer to Section 5.3.14: I/O port characteristics for more details on the input/output alternate function characteristics (NSS, CK, TX, RX for USART).

Table 87. USART electrical characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{C K}$ | USART clock frequency | Master mode | - | - | 21 | MHz |
|  |  | Slave mode | - | - | 22 |  |
| $t_{\text {so }}$ (NSS) | NSS setup time | Slave mode | $t_{\text {ker }}+2$ | - | - | ns |
| $t_{f c}(\mathrm{NSS})$ | NSS hold time | Slave mode | 2 | - | - |  |
| $\begin{aligned} & t_{w}(\mathrm{CKH}) \\ & t_{w}(\mathrm{CKL}) \end{aligned}$ | CK high and low time | Master mode | $1 / f_{c k} / 2-1$ | $1 / f_{c k} / 2$ | $1 / f_{c k} / 2+1$ | ns |
| $t_{\text {su }}(\mathrm{RX})$ | Data input setup time | Master mode | $t_{\text {ker }}+2$ | - | - | ns |
|  |  | Slave mode | 2 | - | - |  |
| $t_{f c}(R X)$ | Data input hold time | Master mode | 1 | - | - |  |
|  |  | Slave mode | 0.5 | - | - |  |Table 87. USART electrical characteristics ${ }^{(1)}$ (continued)

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{t}_{\mathrm{v}}(\mathrm{TX})$ | Data output valid time | Master mode | - | 0.5 | 1.5 | ns |
|  |  | Slave mode | - | 10 | 22 |  |
| $\mathrm{t}_{\mathrm{n}}(\mathrm{RX})$ | Data output hold time | Master mode | 0 | - | - |  |
|  |  | Slave mode | 7 | - | - |  |

1. Based on characterization, not tested in production.

# 5.3.27 UCPD characteristics 

UCPD1 controller complies with USB Type-C Rev.1.2 and USB Power Delivery Rev. 3.0 specifications.

Table 88. UCPD characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :-- | :-- | :-- | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{DD}}$ | UCPD operating supply voltage | Sink mode only | 3.0 | 3.3 | 3.6 | V |
|  |  | Sink and source mode | 3.135 | 3.3 | 3.465 | V |# 6 Package information 

In order to meet environmental requirements, ST offers these devices in different grades of ECOPACK packages, depending on their level of environmental compliance. ECOPACK specifications, grade definitions and product status are available at: www.st.com.
ECOPACK is an ST trademark.

### 6.1 UFQFPN32 package information

This UFQFPN is a 32 -pin, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch ultra thin fine pitch quad flat package.
Figure 40. UFQFPN32 - Outline
![img-40.jpeg](img-40.jpeg)

1. Drawing is not to scale.
2. All leads/pads should also be soldered to the PCB to improve the lead/pad solder joint life.
3. There is an exposed die pad on the underside of the UFQFPN package. It is recommended to connect and solder this backside pad to PCB ground.Table 89. UFQFPN32 - Mechanical data

| Symbol | millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | 0.500 | 0.550 | 0.600 | 0.0197 | 0.0217 | 0.0236 |
| A1 | - | - | 0.050 | - | - | 0.0020 |
| A3 | - | 0.152 | - | - | 0.0060 | - |
| b | 0.180 | 0.230 | 0.280 | 0.0071 | 0.0091 | 0.0110 |
| D | 4.900 | 5.000 | 5.100 | 0.1929 | 0.1969 | 0.2008 |
| D1 | 3.400 | 3.500 | 3.600 | 0.1339 | 0.1378 | 0.1417 |
| D2 | 3.400 | 3.500 | 3.600 | 0.1339 | 0.1378 | 0.1417 |
| E | 4.900 | 5.000 | 5.100 | 0.1929 | 0.1969 | 0.2008 |
| E1 | 3.400 | 3.500 | 3.600 | 0.1339 | 0.1378 | 0.1417 |
| E2 | 3.400 | 3.500 | 3.600 | 0.1339 | 0.1378 | 0.1417 |
| e | - | 0.500 | - | - | 0.0197 | - |
| L | 0.300 | 0.400 | 0.500 | 0.0118 | 0.0157 | 0.0197 |
| ddd | - | - | 0.080 | - | - | 0.0031 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 41. UFQFPN32 - Recommended footprint
![img-41.jpeg](img-41.jpeg)

1. Dimensions are expressed in millimeters.# UFQFPN32 device marking 

The following figure gives an example of topside marking orientation versus pin 1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 42. UFQFPN32 top view example
![img-42.jpeg](img-42.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.2 LQFP32 package information 

This LQFP32 is a 32-pin, $7 \times 7 \mathrm{~mm}$ low-profile quad flat package.
Figure 43. LQFP32 - Outline
![img-43.jpeg](img-43.jpeg)

1. Drawing is not to scale.Table 90. LQFP32 - Mechanical data

| Symbol | millimeters |  |  | inches ${ }^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.600 | - | - | 0.0630 |
| A1 | 0.050 | - | 0.150 | 0.0020 | - | 0.0059 |
| A2 | 1.350 | 1.400 | 1.450 | 0.0531 | 0.0551 | 0.0571 |
| b | 0.300 | 0.370 | 0.450 | 0.0118 | 0.0146 | 0.0177 |
| c | 0.090 | - | 0.200 | 0.0035 | - | 0.0079 |
| D | 8.800 | 9.000 | 9.200 | 0.3465 | 0.3543 | 0.3622 |
| D1 | 6.800 | 7.000 | 7.200 | 0.2677 | 0.2756 | 0.2835 |
| D3 | - | 5.600 | - | - | 0.2205 | - |
| E | 8.800 | 9.000 | 9.200 | 0.3465 | 0.3543 | 0.3622 |
| E1 | 6.800 | 7.000 | 7.200 | 0.2677 | 0.2756 | 0.2835 |
| E3 | - | 5.600 | - | - | 0.2205 | - |
| e | - | 0.800 | - | - | 0.0315 | - |
| L | 0.450 | 0.600 | 0.750 | 0.0177 | 0.0236 | 0.0295 |
| L1 | - | 1.000 | - | - | 0.0394 | - |
| k | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ |
| ccc | - | - | 0.100 | - | - | 0.0039 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 44. LQFP32 - Recommended footprint
![img-44.jpeg](img-44.jpeg)

1. Dimensions are expressed in millimeters.# LQFP32 device marking 

The following figure gives an example of topside marking orientation versus pin 1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 45. LQFP32 top view example
![img-45.jpeg](img-45.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.3 UFQFPN48 package information 

This UFQFPN is a 48-lead, $7 \times 7 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, ultra thin fine pitch quad flat package.
![img-46.jpeg](img-46.jpeg)

1. Drawing is not to scale.
2. All leads/pads should also be soldered to the PCB to improve the lead/pad solder joint life.
3. There is an exposed die pad on the underside of the UFQFPN48 package. It is recommended to connect and solder this back-side pad to PCB ground.Table 91. UFQFPN48 - Mechanical data

| Symbol | millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | 0.500 | 0.550 | 0.600 | 0.0197 | 0.0217 | 0.0236 |
| A1 | 0.000 | 0.020 | 0.050 | 0.0000 | 0.0008 | 0.0020 |
| D | 6.900 | 7.000 | 7.100 | 0.2717 | 0.2756 | 0.2795 |
| E | 6.900 | 7.000 | 7.100 | 0.2717 | 0.2756 | 0.2795 |
| D2 | 5.500 | 5.600 | 5.700 | 0.2165 | 0.2205 | 0.2244 |
| E2 | 5.500 | 5.600 | 5.700 | 0.2165 | 0.2205 | 0.2244 |
| L | 0.300 | 0.400 | 0.500 | 0.0118 | 0.0157 | 0.0197 |
| T | - | 0.152 | - | - | 0.0060 | - |
| b | 0.200 | 0.250 | 0.300 | 0.0079 | 0.0098 | 0.0118 |
| e | - | 0.500 | - | - | 0.0197 | - |
| ddd | - | - | 0.080 | - | - | 0.0031 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 47. UFQFPN48 - Recommended footprint
![img-47.jpeg](img-47.jpeg)

1. Dimensions are expressed in millimeters.# UFQFPN48 device marking 

The following figure gives an example of topside marking orientation versus pin 1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 48. UFQFPN48 top view example
![img-48.jpeg](img-48.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.4 LQFP48 package information 

This LQFP is a 48 -pin, $7 \times 7 \mathrm{~mm}$ low-profile quad flat package.
Figure 49. LQFP48 - Outline
![img-49.jpeg](img-49.jpeg)

1. Drawing is not to scale.Table 92. LQFP48 - Mechanical data

| Symbol | millimeters |  |  | inches ${ }^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.600 | - | - | 0.0630 |
| A1 | 0.050 | - | 0.150 | 0.0020 | - | 0.0059 |
| A2 | 1.350 | 1.400 | 1.450 | 0.0531 | 0.0551 | 0.0571 |
| b | 0.170 | 0.220 | 0.270 | 0.0067 | 0.0087 | 0.0106 |
| c | 0.090 | - | 0.200 | 0.0035 | - | 0.0079 |
| D | 8.800 | 9.000 | 9.200 | 0.3465 | 0.3543 | 0.3622 |
| D1 | 6.800 | 7.000 | 7.200 | 0.2677 | 0.2756 | 0.2835 |
| D3 | - | 5.500 | - | - | 0.2165 | - |
| E | 8.800 | 9.000 | 9.200 | 0.3465 | 0.3543 | 0.3622 |
| E1 | 6.800 | 7.000 | 7.200 | 0.2677 | 0.2756 | 0.2835 |
| E3 | - | 5.500 | - | - | 0.2165 | - |
| e | - | 0.500 | - | - | 0.0197 | - |
| L | 0.450 | 0.600 | 0.750 | 0.0177 | 0.0236 | 0.0295 |
| L1 | - | 1.000 | - | - | 0.0394 | - |
| k | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ |
| ccc | - | - | 0.080 | - | - | 0.0031 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.Figure 50. LQFP48 - Recommended footprint
![img-50.jpeg](img-50.jpeg)

1. Dimensions are expressed in millimeters.# LQFP48 device marking 

The following figure gives an example of topside marking orientation versus pin 1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 51. LQFP48 top view example
![img-51.jpeg](img-51.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.5 WLCSP49 package information 

This WLCSP49 is a 49-ball, $3.15 \times 3.13 \mathrm{~mm}, 0.4 \mathrm{~mm}$ pitch, wafer level chip scale package.
Figure 52. WLCSP49 - Outline
![img-52.jpeg](img-52.jpeg)

1. Drawing is not to scale.
2. Dimension is measured at the maximum bump diameter parallel to primary datum $Z$.
3. Primary datum $Z$ and seating plane are defined by the spherical crowns of the bump.
4. Bump position designation per JESD 95-1, SPP-010.Table 93. WLCSP49 - Mechanical data

| Symbol | millimeters |  |  | inches ${ }^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| $A^{(2)}$ | - | - | 0.59 | - | - | 0.023 |
| A1 | - | 0.18 | - | - | 0.007 | - |
| A2 | - | 0.38 | - | - | 0.015 | - |
| A3 ${ }^{(3)}$ | - | 0.025 | - | - | 0.001 | - |
| $b^{(4)}$ | 0.22 | 0.25 | 0.28 | 0.009 | 0.010 | 0.011 |
| D | 3.13 | 3.15 | 3.17 | 0.123 | 0.124 | 0.125 |
| E | 3.11 | 3.13 | 3.15 | 0.122 | 0.123 | 0.124 |
| e | - | 0.40 | - | - | 0.016 | - |
| e1 | - | 2.40 | - | - | 0.094 | - |
| e2 | - | 2.40 | - | - | 0.094 | - |
| $F^{(5)}$ | - | 0.375 | - | - | 0.015 | - |
| $G^{(5)}$ | - | 0.365 | - | - | 0.014 | - |
| aaa | - | 0.10 | - | - | 0.004 | - |
| bbb | - | 0.10 | - | - | 0.004 | - |
| ccc | - | 0.10 | - | - | 0.004 | - |
| ddd | - | 0.05 | - | - | 0.002 | - |
| eee | - | 0.05 | - | - | 0.002 | - |

1. Values in inches are converted from mm and rounded to 4 decimal digits.
2. The maximum total package height is calculated by the RSS method (Root Sum Square) using nominal and tolerances values of A1 and A2.
3. Back side coating. Nominal dimension is rounded to the 3rd decimal place resulting from process capability.
4. Dimension is measured at the maximum bump diameter parallel to primary datum $Z$.
5. Calculated dimensions are rounded to the 3rd decimal placeFigure 53. WLCSP49 - Recommended footprint
![img-53.jpeg](img-53.jpeg)

1. Dimensions are expressed in millimeters.

Table 94. WLCSP49 - Recommended PCB design rules

| Dimension | Recommended values |
| :-- | :-- |
| Pitch | 0.4 mm |
| Dpad | 0.225 mm |
| Dsm | 0.290 mm typ. (depends on soldermask registration tolerance) |
| Stencil opening | 0.250 mm |
| Stencil thickness | 0.100 mm |# WLCSP49 device marking 

The following figure gives an example of topside marking orientation versus ball A1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 54. WLCSP49 top view example
![img-54.jpeg](img-54.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.6 LQFP64 package information 

This LQFP is a 64-pin, $10 \times 10 \mathrm{~mm}$ low-profile quad flat package.
Figure 55. LQFP64 - Outline
![img-55.jpeg](img-55.jpeg)

1. Drawing is not to scale.

Table 95. LQFP64 - Mechanical data

| Symbol | millimeters |  |  | inches $\left.{ }^{(1)}\right.$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.600 | - | - | 0.0630 |
| A1 | 0.050 | - | 0.150 | 0.0020 | - | 0.0059 |
| A2 | 1.350 | 1.400 | 1.450 | 0.0531 | 0.0551 | 0.0571 |
| b | 0.170 | 0.220 | 0.270 | 0.0067 | 0.0087 | 0.0106 |
| c | 0.090 | - | 0.200 | 0.0035 | - | 0.0079 |
| D | - | 12.000 | - | - | 0.4724 | - |
| D1 | - | 10.000 | - | - | 0.3937 | - |
| D3 | - | 7.500 | - | - | 0.2953 | - |
| E | - | 12.000 | - | - | 0.4724 | - |
| E1 | - | 10.000 | - | - | 0.3937 | - |Table 95. LQFP64 - Mechanical data (continued)

| Symbol | millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| E3 | - | 7.500 | - | - | 0.2953 | - |
| e | - | 0.500 | - | - | 0.0197 | - |
| K | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ |
| L | 0.450 | 0.600 | 0.750 | 0.0177 | 0.0236 | 0.0295 |
| L1 | - | 1.000 | - | - | 0.0394 | - |
| ccc | - | - | 0.080 | - | - | 0.0031 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 56. LQFP64 - Recommended footprint
![img-56.jpeg](img-56.jpeg)

1. Dimensions are expressed in millimeters.# LQFP64 device marking 

The following figure gives an example of topside marking orientation versus pin 1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 57. LQFP64 top view example
![img-57.jpeg](img-57.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.7 UFBGA64 package information 

This UFBGA is a 64 -ball, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch ultra profile fine pitch ball grid array package.

Figure 58. UFBGA64 - Outline
![img-58.jpeg](img-58.jpeg)

1. Drawing is not to scale.

Table 96. UFBGA64 - Mechanical data

| Symbol | millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | 0.460 | 0.530 | 0.600 | 0.0181 | 0.0209 | 0.0236 |
| A1 | 0.050 | 0.080 | 0.110 | 0.0020 | 0.0031 | 0.0043 |
| A2 | 0.400 | 0.450 | 0.500 | 0.0157 | 0.0177 | 0.0197 |
| A3 | 0.080 | 0.130 | 0.180 | 0.0031 | 0.0051 | 0.0071 |
| A4 | 0.270 | 0.320 | 0.370 | 0.0106 | 0.0126 | 0.0146 |
| b | 0.170 | 0.280 | 0.330 | 0.0067 | 0.0110 | 0.0130 |
| D | 4.850 | 5.000 | 5.150 | 0.1909 | 0.1969 | 0.2028 |
| D1 | 3.450 | 3.500 | 3.550 | 0.1358 | 0.1378 | 0.1398 |
| E | 4.850 | 5.000 | 5.150 | 0.1909 | 0.1969 | 0.2028 |
| E1 | 3.450 | 3.500 | 3.550 | 0.1358 | 0.1378 | 0.1398 |
| e | - | 0.500 | - | - | 0.0197 | - |
| F | 0.700 | 0.750 | 0.800 | 0.0276 | 0.0295 | 0.0315 |Table 96. UFBGA64 - Mechanical data (continued)

| Symbol | millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | 0.460 | 0.530 | 0.600 | 0.0181 | 0.0209 | 0.0236 |
| ddd | - | - | 0.080 | - | - | 0.0031 |
| eee | - | - | 0.150 | - | - | 0.0059 |
| fff | - | - | 0.050 | - | - | 0.0020 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 59. UFBGA64 - Recommended footprint
![img-59.jpeg](img-59.jpeg)

A019_FP_V2
Table 97. UFBGA64 - Recommended PCB design rules ( 0.5 mm pitch BGA)

| Dimension | Recommended values |
| :-- | :-- |
| Pitch | 0.5 |
| Dpad | 0.280 mm |
| Dsm | 0.370 mm typ. (depends on the solder mask <br> registration tolerance) |
| Stencil opening | 0.280 mm |
| Stencil thickness | Between 0.100 mm and 0.125 mm |
| Pad trace width | 0.100 mm |# UFBGA64 device marking 

The following figure gives an example of topside marking orientation versus pin 1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 60. UFBGA64 top view example
![img-60.jpeg](img-60.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.8 LQFP80 package information 

This LQFP is a 80-pin, $12 \times 12 \mathrm{~mm}$ low-profile quad flat package.
Figure 61. LQFP80 - Outline
![img-61.jpeg](img-61.jpeg)

1. Drawing is not to scale.

Table 98. LQFP80 - Mechanical data

| Symbol | Millimeters |  |  | inches $\left.{ }^{(1)}\right.$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.600 | - | - | 0.0630 |
| A1 | 0.050 | - | 0.150 | 0.0020 | - | 0.0059 |
| A2 | 1.350 | 1.400 | 1.450 | 0.0531 | 0.0551 | 0.0571 |
| b | 0.170 | 0.220 | 0.270 | 0.0067 | 0.0087 | 0.0106 |
| c | 0.090 | - | 0.200 | 0.0035 | - | 0.0079 |
| D | - | 14.000 | - | - | 0.5512 | - |
| D1 | - | 12.000 | - | - | 0.4724 | - |Table 98. LQFP80 - Mechanical data (continued)

| Symbol | Millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| D2 | - | 9.500 | - | - | 0.3740 | - |
| E | - | 14.000 | - | - | 0.5512 | - |
| E1 | - | 12.000 | - | - | 0.4724 | - |
| E3 | - | 9.500 | - | - | 0.3740 | - |
| e | - | 0.500 | - | - | 0.0197 | - |
| L | 0.450 | 0.600 | 0.750 | 0.0177 | 0.0236 | 0.0295 |
| L1 | - | 1.000 | - | - | 0.0394 | - |
| ccc | - | - | 0.080 | - | - | 0.0031 |
| k | $0.0^{\circ}$ | - | $7.0^{\circ}$ | $0.0^{\circ}$ | - | $7.0^{\circ}$ |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 62. LQFP80 - Recommended footprint
![img-62.jpeg](img-62.jpeg)

1. Dimensions are expressed in millimeters.# LQFP80 device marking 

The following figure gives an example of topside marking orientation versus pin 1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 63. LQFP80 top view example
![img-63.jpeg](img-63.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.9 LQFP100 package information 

This LQFP is a 100-pin, $14 \times 14 \mathrm{~mm}$ low-profile quad flat package.
Figure 64. LQFP100 - Outline
![img-64.jpeg](img-64.jpeg)

1. Drawing is not to scale.

Table 99. LQPF100 - Mechanical data

| Symbol | millimeters |  |  | inches $\left.{ }^{(1)}\right.$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.600 | - | - | 0.0630 |
| A1 | 0.050 | - | 0.150 | 0.0020 | - | 0.0059 |
| A2 | 1.350 | 1.400 | 1.450 | 0.0531 | 0.0551 | 0.0571 |
| b | 0.170 | 0.220 | 0.270 | 0.0067 | 0.0087 | 0.0106 |
| c | 0.090 | - | 0.200 | 0.0035 | - | 0.0079 |
| D | 15.800 | 16.000 | 16.200 | 0.6220 | 0.6299 | 0.6378 |
| D1 | 13.800 | 14.000 | 14.200 | 0.5433 | 0.5512 | 0.5591 |
| D3 | - | 12.000 | - | - | 0.4724 | - |
| E | 15.800 | 16.000 | 16.200 | 0.6220 | 0.6299 | 0.6378 |
| E1 | 13.800 | 14.000 | 14.200 | 0.5433 | 0.5512 | 0.5591 |
| E3 | - | 12.000 | - | - | 0.4724 | - |
| e | - | 0.500 | - | - | 0.0197 | - |
| L | 0.450 | 0.600 | 0.750 | 0.0177 | 0.0236 | 0.0295 |
| L1 | - | 1.000 | - | - | 0.0394 | - |Table 99. LQPF100 - Mechanical data (continued)

| Symbol | millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| k | $0.0^{\circ}$ | $3.5^{\circ}$ | $7.0^{\circ}$ | $0.0^{\circ}$ | $3.5^{\circ}$ | $7.0^{\circ}$ |
| ccc | - | - | 0.080 | - | - | 0.0031 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 65. LQFP100 - Recommended footprint
![img-65.jpeg](img-65.jpeg)

1. Dimensions are expressed in millimeters.# LQFP100 device marking 

The following figure gives an example of topside marking orientation versus pin 1 identifier location.

The printed markings may differ depending on the supply chain.
Other optional marking or inset/upset marks, which identify the parts throughout supply chain operations, are not indicated below.

Figure 66. LQFP100 top view example
![img-66.jpeg](img-66.jpeg)

1. Parts marked as ES or E or accompanied by an engineering sample notification letter are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.10 Thermal characteristics 

The maximum chip-junction temperature, $\mathrm{T}_{\mathrm{J}}$ max, in degrees Celsius, may be calculated using the following equation:
$T_{J} \max =T_{A} \max +\left(P_{D} \max x \Theta_{J A}\right)$
Where:

- $\quad T_{A}$ max is the maximum ambient temperature in ${ }^{\circ} \mathrm{C}$,
- $\quad \Theta_{\mathrm{JA}}$ is the package junction-to-ambient thermal resistance, in ${ }^{\circ} \mathrm{C} / \mathrm{W}$,
- $\quad P_{D}$ max is the sum of $P_{\text {INT }}$ max and $P_{I / O}$ max ( $P_{D} \max =P_{\text {INT }} \max +P_{I / O} \max$ ),
- $\quad P_{\text {INT }}$ max is the product of $\mathrm{I}_{\mathrm{DD}}$ and $\mathrm{V}_{\mathrm{DD}}$, expressed in Watts. This is the maximum chip internal power.
$P_{I / O}$ max represents the maximum power dissipation on output pins where:

$$
P_{I / O} \max =\Sigma\left(V_{O L} \times I_{O L}\right)+\Sigma\left(\left(V_{D D I O x}-V_{O H}\right) \times I_{O H}\right)
$$

taking into account the actual $V_{O L} / I_{O L}$ and $V_{O H} / I_{O H}$ of the I/Os at low and high level in the application.

Table 100. Package thermal characteristics

| Symbol | Parameter | Value | Unit |
| :--: | :--: | :--: | :--: |
| $\Theta_{\text {JA }}$ | Thermal resistance junction-ambient LQFP100 - $14 \times 14 \mathrm{~mm}$ | 48.9 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
|  | Thermal resistance junction-ambient LQFP80 - $12 \times 12 \mathrm{~mm}$ | 49.7 |  |
|  | Thermal resistance junction-ambient LQFP64 - $10 \times 10 \mathrm{~mm}$ | 50.8 |  |
|  | Thermal resistance junction-ambient LQFP48 - $7 \times 7 \mathrm{~mm}$ | 58.4 |  |
|  | Thermal resistance junction-ambient LQFP32 - $7 \times 7 \mathrm{~mm}$ | 58.4 |  |
|  | Thermal resistance junction-ambient UFBGA64 - $5 \times 5 \mathrm{~mm}$ | 44.2 |  |
|  | Thermal resistance junction-ambient UFQFPN48 - $7 \times 7 \mathrm{~mm}$ | 28.6 |  |
|  | Thermal resistance junction-ambient UFQFPN32 - $5 \times 5 \mathrm{~mm}$ | 36.7 |  |
|  | Thermal resistance junction-ambient WLCSP49 - pitch 0.4 | 59 |  |Table 100. Package thermal characteristics (continued)

| Symbol | Parameter | Value | Unit |
| :--: | :--: | :--: | :--: |
|  | Thermal resistance junction-case <br> LQFP100 - $14 \times 14 \mathrm{~mm}$ | 10.3 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
|  | Thermal resistance junction-case <br> LQFP80 - $12 \times 12 \mathrm{~mm}$ | 10.3 |  |
|  | Thermal resistance junction-case <br> LQFP64 - $10 \times 10 \mathrm{~mm}$ | 10.1 |  |
|  | Thermal resistance junction-case <br> LQFP48 - $7 \times 7 \mathrm{~mm}$ | 11.9 |  |
|  | Thermal resistance junction-case <br> LQFP32 - $7 \times 7 \mathrm{~mm}$ | 11.9 |  |
|  | Thermal resistance junction-case <br> UFBGA64 - $5 \times 5 \mathrm{~mm}$ | 14.78 |  |
|  | Thermal resistance junction-case <br> UFQFPN48 - $7 \times 7 \mathrm{~mm}$ | $\begin{gathered} 3.1^{(1)} \\ 9.4 \end{gathered}$ |  |
|  | Thermal resistance junction-case <br> UFQFPN32 - $5 \times 5 \mathrm{~mm}$ | $\begin{gathered} 3.4^{(1)} \\ 14.2 \end{gathered}$ |  |
|  | Thermal resistance junction-case <br> WLCSP49 - pitch 0.4 | 2.33 |  |
|  | Thermal resistance junction-board LQFP100 - $14 \times 14 \mathrm{~mm}$ | 25.7 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
|  | Thermal resistance junction-board LQFP80 - $12 \times 12 \mathrm{~mm}$ | 25.1 |  |
|  | Thermal resistance junction-board LQFP64 - $10 \times 10 \mathrm{~mm}$ | 24.7 |  |
|  | Thermal resistance junction-board LQFP48 - $7 \times 7 \mathrm{~mm}$ | 27.7 |  |
|  | Thermal resistance junction-board LQFP32 - $7 \times 7 \mathrm{~mm}$ | 27.7 |  |
|  | Thermal resistance junction-board UFBGA64 - $5 \times 5 \mathrm{~mm}$ | 22.5 |  |
|  | Thermal resistance junction-board UFQFPN48 - $7 \times 7 \mathrm{~mm}$ | 15.7 |  |
|  | Thermal resistance junction-board UFQFPN32 - $5 \times 5 \mathrm{~mm}$ | 18.6 |  |
|  | Thermal resistance junction-board WLCSP49 - pitch 0.4 | 38.12 |  |

1. Thermal resistance junction-case where the case is the bottom thermal pad on the UFQFPN package.

# 6.10.1 Reference document 

JESD51-2 Integrated Circuits Thermal Test Method Environment Conditions - Natural Convection (Still Air). Available from www.jedec.org# 6.10.2 Selecting the product temperature range 

When ordering the microcontroller, the temperature range is specified in the ordering information scheme shown in Section 7: Ordering information.

Each temperature range suffix corresponds to a specific guaranteed ambient temperature at maximum dissipation and, to a specific maximum junction temperature.

As applications do not commonly use the STM32G431xB at maximum dissipation, it is useful to calculate the exact power consumption and junction temperature to determine which temperature range is best suited to the application.

The following examples show how to calculate the temperature range needed for a given application.

## Example 1: High-performance application

Assuming the following application conditions:
Maximum ambient temperature $\mathrm{T}_{\text {Amax }}=82^{\circ} \mathrm{C}$ (measured according to JESD51-2),
$\mathrm{I}_{\mathrm{DDmax}}=50 \mathrm{~mA}, \mathrm{~V}_{\mathrm{DD}}=3.5 \mathrm{~V}$, maximum $20 \mathrm{I} / \mathrm{Os}$ used at the same time in output at low level with $\mathrm{I}_{\mathrm{OL}}=8 \mathrm{~mA}, \mathrm{~V}_{\mathrm{OL}}=0.4 \mathrm{~V}$ and maximum $8 \mathrm{I} / \mathrm{Os}$ used at the same time in output at low level with $\mathrm{I}_{\mathrm{OL}}=20 \mathrm{~mA}, \mathrm{~V}_{\mathrm{OL}}=1.3 \mathrm{~V}$
$\mathrm{P}_{\mathrm{INT} \max }=50 \mathrm{~mA} \times 3.5 \mathrm{~V}=175 \mathrm{~mW}$
$\mathrm{P}_{\mathrm{IOmax}}=20 \times 8 \mathrm{~mA} \times 0.4 \mathrm{~V}+8 \times 20 \mathrm{~mA} \times 1.3 \mathrm{~V}=272 \mathrm{~mW}$
This gives: $\mathrm{P}_{\mathrm{INT} \max }=175 \mathrm{~mW}$ and $\mathrm{P}_{\mathrm{IO} \max }=272 \mathrm{~mW}$ :
$\mathrm{P}_{\text {Dmax }}=175+272=447 \mathrm{~mW}$
Using the values obtained in $\mathrm{T}_{\text {Jmax }}$ is calculated as follows:

- For LQFP100, $42^{\circ} \mathrm{C} / \mathrm{W}$
$\mathrm{T}_{\text {Jmax }}=82^{\circ} \mathrm{C}+\left(42^{\circ} \mathrm{C} / \mathrm{W} \times 447 \mathrm{~mW}\right)=82^{\circ} \mathrm{C}+18.774^{\circ} \mathrm{C}=100.774^{\circ} \mathrm{C}$
This is within the range of the suffix 6 version parts $\left(-40<\mathrm{T}_{\mathrm{J}}<105^{\circ} \mathrm{C}\right)$ see Section 7: Ordering information.

In this case, parts must be ordered at least with the temperature range suffix 6 (see Section 7: Ordering information).
Note: With this given $P_{\text {Dmax }}$ we can find the TAmax allowed for a given device temperature range (order code suffix 6 or 7 ).
Suffix 6: $T_{\text {Amax }}=T_{\text {Jmax }}-\left(42^{\circ} \mathrm{C} / W \times 447 \mathrm{~mW}\right)=105-18.774=86.226^{\circ} \mathrm{C}$
Suffix 3: $T_{\text {Amax }}=T_{\text {Jmax }}-\left(42^{\circ} \mathrm{C} / W \times 447 \mathrm{~mW}\right)=130-18.774=111.226^{\circ} \mathrm{C}$

## Example 2: High-temperature application

Using the same rules, it is possible to address applications that run at high ambient temperatures with a low dissipation, as long as junction temperature $T_{J}$ remains within the specified range.Assuming the following application conditions:
Maximum ambient temperature $\mathrm{T}_{\text {Amax }}=100^{\circ} \mathrm{C}$ (measured according to JESD51-2),
$\mathrm{I}_{\mathrm{DDmax}}=20 \mathrm{~mA}, \mathrm{~V}_{\mathrm{DD}}=3.5 \mathrm{~V}$, maximum $20 \mathrm{I} / \mathrm{Os}$ used at the same time in output at low level with $\mathrm{I}_{\mathrm{OL}}=8 \mathrm{~mA}, \mathrm{~V}_{\mathrm{OL}}=0.4 \mathrm{~V}$
$\mathrm{P}_{\mathrm{INT} \max }=20 \mathrm{~mA} \times 3.5 \mathrm{~V}=70 \mathrm{~mW}$
$\mathrm{P}_{\mathrm{IOmax}}=20 \times 8 \mathrm{~mA} \times 0.4 \mathrm{~V}=64 \mathrm{~mW}$
This gives: $\mathrm{P}_{\mathrm{INT} \max }=70 \mathrm{~mW}$ and $\mathrm{P}_{\mathrm{IOmax}}=64 \mathrm{~mW}$ :
$\mathrm{P}_{\mathrm{Dmax}}=70+64=134 \mathrm{~mW}$
Thus: $\mathrm{P}_{\mathrm{D} \max }=134 \mathrm{~mW}$
Using the values obtained in $T_{\text {Jmax }}$ is calculated as follows:

- For LQFP100, $42^{\circ} \mathrm{C} / \mathrm{W}$
$\mathrm{T}_{\mathrm{J} \max }=100^{\circ} \mathrm{C}+\left(42^{\circ} \mathrm{C} / \mathrm{W} \times 134 \mathrm{~mW}\right)=100^{\circ} \mathrm{C}+5.628^{\circ} \mathrm{C}=105.628^{\circ} \mathrm{C}$
This is above the range of the suffix 6 version parts $\left(-40<T_{J}<105^{\circ} \mathrm{C}\right)$.
In this case, parts must be ordered at least with the temperature range suffix 3 (see Section 7: Ordering information) unless we reduce the power dissipation in order to be able to use suffix 6 parts.# 7 Ordering information 

Table 101. Ordering information scheme
Example:
![img-67.jpeg](img-67.jpeg)

STM32 = Arm-based 32-bit microcontroller

## Product type

$\mathrm{G}=$ General-purpose

## Sub-family

$431=$ STM32G431x6/x8/xB

## Pin count

$\mathrm{K}=32$ pins
$\mathrm{C}=48 / 49$ pins
$R=64$ pins
$M=80$ pins
$V=100$ pins

## Code size

$6=32$ Kbytes
$8=64$ Kbytes
$B=128$ Kbytes

## Package

I = UFBGA
$\mathrm{T}=\mathrm{LQFP}$
$\mathrm{U}=$ UFQFPN
$Y=$ WLCSP

## Temperature range

$6=$ Industrial temperature range, -40 to $85^{\circ} \mathrm{C}\left(105^{\circ} \mathrm{C}\right.$ junction $)$
$3=$ Industrial temperature range, -40 to $125^{\circ} \mathrm{C}\left(130^{\circ} \mathrm{C}\right.$ junction $)$

## Options

xxx $=$ programmed parts
$\mathrm{TR}=$ tape and reel

For a list of available options (memory, package, and so on) or for further information on any aspect of this device, contact the nearest ST sales office.# 8 Revision history 

Table 102. Document revision history

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 10-May-2019 | 1 | Initial release. |
| 28-Oct-2019 | 2 | Updated: <br> - Section 2: Description and Table 2: STM32G431x6/x8/xB features and peripheral counts removing "-40 to $105^{\circ} \mathrm{C}\left(+125^{\circ} \mathrm{C}\right.$ junction) ${ }^{\prime \prime}$. <br> - Figure 1: STM32G431x6/x8/xB block diagram with 170 MHz . <br> - Section 3.5: Embedded SRAM. <br> - Section 3.20: Voltage reference buffer (VREFBUF). <br> - Table 3: STM32G431x6/x8/xB peripherals interconnect matrix. <br> - Table 17: General operating conditions. <br> - Table 34: Peripheral current consumption. <br> - Table 60: ADC characteristics. <br> - Table 83: SPI characteristics. <br> - Table 100: Package thermal characteristics. <br> - Table 101: Ordering information scheme. <br> - Section 6: Package information. <br> Added: <br> - Table 65: ADC accuracy (Multiple ADCs operation) - limited test conditions 1. <br> - Table 66: ADC accuracy (Multiple ADCs operation) - limited test conditions 2. <br> - Table 67: ADC accuracy (Multiple ADCs operation) - limited test conditions 3. |Table 102. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 20-Nov-2020 | 3 | Updated: <br> - Table 2: STM32G431x6/x8/xB features and peripheral counts timer PWM channel number. <br> - Section 3.5: Embedded SRAM. <br> - Table 12: STM32G431x6/x8/xB pin definition. <br> - Figure 11: STM32G431x6/x8/xB UFBGA64 ballout. <br> - Table 21, Table 22, Table 27, Table 28, Table 29, Table 30, Table 31 Table 32 max current consumptions. <br> - Table 35: Low-power mode wakeup timings adding note. <br> - Table 70: DAC 15MSPS characteristics $\mathrm{T}_{\text {SAMP }}$, dV/dt (hold phase) and $\mathrm{I}_{\mathrm{DDA}}(\mathrm{DAC})$ characteristics. <br> - Table 73: COMP characteristics $\mathrm{I}_{\mathrm{DDA}}(\mathrm{COMP})$. <br> - Table 74: OPAMP characteristics PSRR. <br> - Table 76: $V_{B A T}$ monitoring characteristics. <br> - Table 100: Package thermal characteristics. <br> - Internal voltage reference buffer (VREFBUF) at 2.9 V . <br> Removed: <br> - Current consumption in Run and Low-power run modes, code with data processing running from Flash in single Bank, ART disable table. <br> - Typical current consumption in Run and Low-power run modes, with different codes running from Flash, ART disable table. |
| 22-Feb-2021 | 4 | Updated: <br> - Section 3.11.4: Low-power modes. <br> - Table 29: Current consumption in Stop 1 mode. <br> - Table 30: Current consumption in Stop 0 mode. <br> - Table 31: Current consumption in Standby mode. <br> - Table 32: Current consumption in Shutdown mode. |
| 01-Oct-2021 | 5 | Updated: <br> - Section 2: Description <br> - Table 2: STM32G431x6/x8/xB features and peripheral counts <br> - Table 5: Temperature sensor calibration values <br> - Table 12: STM32G431x6/x8/xB pin definition <br> - Figure 5: STM32G431x6/x8/xB UFQFPN32 pinout <br> - Table 62: ADC accuracy - limited test conditions 1 <br> - Table 63: ADC accuracy - limited test conditions 2 <br> - Table 64: ADC accuracy - limited test conditions 3 <br> - Figure 28: ADC accuracy characteristics <br> - Figure 29: Typical connection diagram when using the ADC with FT/TT pins featuring analog switch function <br> Deleted: <br> - Note 2 of Figure 40: UFQFPN32 - Outline |Table 102. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :-- |
| 22-Oct-2021 | 6 | Updated: <br> - Section 2: Description <br> - Section 3.4: Embedded Flash memory <br> - Table 12: STM32G431x6/x8/xB pin definition |# IMPORTANT NOTICE - PLEASE READ CAREFULLY 

STMicroelectronics NV and its subsidiaries ("ST") reserve the right to make changes, corrections, enhancements, modifications, and improvements to ST products and/or to this document at any time without notice. Purchasers should obtain the latest relevant information on ST products before placing orders. ST products are sold pursuant to ST's terms and conditions of sale in place at the time of order acknowledgement.

Purchasers are solely responsible for the choice, selection, and use of ST products and ST assumes no liability for application assistance or the design of Purchasers' products.

No license, express or implied, to any intellectual property right is granted by ST herein.

Resale of ST products with provisions different from the information set forth herein shall void any warranty granted by ST for such product.

ST and the ST logo are trademarks of ST. For additional information about ST trademarks, please refer to www.st.com/trademarks. All other product or service names are the property of their respective owners.

Information in this document supersedes and replaces information previously supplied in any prior versions of this document.
(c) 2021 STMicroelectronics - All rights reserved