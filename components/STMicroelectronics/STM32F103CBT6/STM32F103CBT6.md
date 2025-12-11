# 5 

## STATI- AUG- REV 2018

## STM32F103x8

Medium-density performance line Arm ${ }^{\circledR}$-based 32-bit MCU with 64 or 128 KB Flash, USB, CAN, 7 timers, 2 ADCs, 9 com. interfaces

Datasheet - production data

## Features

## Includes ST state-of-the-art patented technology

- Arm ${ }^{\circledR} 32$-bit Cortex ${ }^{\circledR}$-M3 CPU core
- 72 MHz maximum frequency, 1.25

DMIPS/MHz (Dhrystone 2.1) performance at 0 wait state memory access

- Single-cycle multiplication and hardware division
- Memories
- 64 or 128 Kbytes of Flash memory
- 20 Kbytes of SRAM
- Clock, reset and supply management
- 2.0 to 3.6 V application supply and I/Os
- POR, PDR, and programmable voltage detector (PVD)
- 4 to 16 MHz crystal oscillator
- Internal 8 MHz factory-trimmed RC
- Internal 40 kHz RC
- PLL for CPU clock
- 32 kHz oscillator for RTC with calibration
- Low-power
- Sleep, Stop and Standby modes
- $\mathrm{V}_{\text {BAT }}$ supply for RTC and backup registers
- $2 \times 12$-bit, $1 \mu \mathrm{~s}$ A/D converters (up to 16 channels)
- Conversion range: 0 to 3.6 V
- Dual-sample and hold capability
- Temperature sensor
- DMA
- 7-channel DMA controller
- Peripherals supported: timers, ADC, SPIs, $I^{2} \mathrm{Cs}$ and USARTs
- Up to 80 fast I/O ports
- 26/37/51/80 I/Os, all mappable on 16 external interrupt vectors and almost all 5 V-tolerant
![img-0.jpeg](img-0.jpeg)
- Debug mode:
- Serial wire debug (SWD) and JTAG interfaces
- Seven timers
- Three 16-bit timers, each with up to 4 IC/OC/PWM or pulse counter and quadrature (incremental) encoder input
- 16-bit, motor control PWM timer with dead-time generation and emergency stop
- Two watchdog timers (independent and window)
- SysTick timer 24-bit downcounter
- Up to nine communication interfaces
- Up to two $\mathrm{I}^{2} \mathrm{C}$ interfaces (SMBus/PMBus ${ }^{\circledR}$ )
- Up to three USARTs (ISO 7816 interface, LIN, IrDA capability, modem control)
- Up to two SPIs (18 Mbit/s)
- CAN interface (2.0B Active)
- USB 2.0 full-speed interface
- CRC calculation unit, 96-bit unique ID
- Packages are ECOPACK ${ }^{\circledR}$

Table 1. Device summary

| Reference | Part number |
| :-- | :-- |
| STM32F103x8 | STM32F103C8, STM32F103R8 <br> STM32F103V8, STM32F103T8 |
| STM32F103xB | STM32F103RB STM32F103VB, <br> STM32F103CB, STM32F103TB |1 Introduction ..... 9
2 Description ..... 9
2.1 Device overview ..... 10
2.2 Full compatibility throughout the family ..... 13
2.3 Overview ..... 14
2.3.1 Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M3 core with embedded flash and SRAM ..... 14
2.3.2 Embedded flash memory ..... 14
2.3.3 CRC (cyclic redundancy check) calculation unit ..... 14
2.3.4 Embedded SRAM ..... 14
2.3.5 Nested vectored interrupt controller (NVIC) ..... 14
2.3.6 External interrupt/event controller (EXTI) ..... 15
2.3.7 Clocks and startup ..... 15
2.3.8 Boot modes ..... 15
2.3.9 Power supply schemes ..... 15
2.3.10 Power supply supervisor ..... 15
2.3.11 Voltage regulator ..... 16
2.3.12 Low-power modes ..... 16
2.3.13 DMA ..... 17
2.3.14 RTC (real-time clock) and backup registers ..... 17
2.3.15 Timers and watchdogs ..... 17
2.3.16 I²C bus ..... 19
2.3.17 Universal synchronous/asynchronous receiver transmitter (USART) ..... 19
2.3.18 Serial peripheral interface (SPI) ..... 19
2.3.19 Controller area network (CAN) ..... 19
2.3.20 Universal serial bus (USB) ..... 19
2.3.21 GPIOs (general-purpose inputs/outputs) ..... 20
2.3.22 ADC (analog-to-digital converter) ..... 20
2.3.23 Temperature sensor ..... 20
2.3.24 Serial wire JTAG debug port (SWJ-DP) ..... 20
3 Pinouts and pin description ..... 21
4 Memory mapping ..... 34
5 Electrical characteristics ..... 35
5.1 Parameter conditions ..... 355.1.1 Minimum and maximum values ..... 35
5.1.2 Typical values ..... 35
5.1.3 Typical curves ..... 35
5.1.4 Loading capacitor ..... 35
5.1.5 Pin input voltage ..... 35
5.1.6 Power supply scheme ..... 36
5.1.7 Current consumption measurement ..... 36
5.2 Absolute maximum ratings ..... 37
5.3 Operating conditions ..... 38
5.3.1 General operating conditions ..... 38
5.3.2 Operating conditions at power-up / power-down ..... 39
5.3.3 Embedded reset and power control block characteristics ..... 39
5.3.4 Embedded reference voltage ..... 40
5.3.5 Supply current characteristics ..... 40
5.3.6 External clock source characteristics ..... 50
5.3.7 Internal clock source characteristics ..... 54
5.3.8 PLL characteristics ..... 56
5.3.9 Memory characteristics ..... 56
5.3.10 EMC characteristics ..... 57
5.3.11 Absolute maximum ratings (electrical sensitivity) ..... 59
5.3.12 I/O current injection characteristics ..... 60
5.3.13 I/O port characteristics ..... 61
5.3.14 NRST pin characteristics ..... 66
5.3.15 TIM timer characteristics ..... 67
5.3.16 Communications interfaces ..... 68
5.3.17 CAN (controller area network) interface ..... 73
5.3.18 12-bit ADC characteristics ..... 74
5.3.19 Temperature sensor characteristics ..... 78
6 Package information ..... 79
6.1 Device marking ..... 79
6.2 VFQFPN36 package information (ZR) ..... 80
6.3 UFQFPN48 package information (A0B9) ..... 83
6.4 LFBGA100 package information (H0) ..... 85
6.5 LQFP100 package information (1L) ..... 88
6.6 UFBGA100 package information (A0C2) ..... 916.7 LQFP64 package information (5W) ..... 94
6.8 TFBGA64 package information (R8) ..... 97
6.9 LQFP48 package information (5B) ..... 99
6.10 Thermal characteristics ..... 102
6.10.1 Reference document ..... 102
6.10.2 Selecting the product temperature range ..... 103
7 Ordering information scheme ..... 105
8 Important security notice ..... 106
9 Revision history ..... 107# List of tables 

Table 1. Device summary ..... 1
Table 2. STM32F103xx medium-density device features and peripheral counts ..... 10
Table 3. STM32F103xx family ..... 13
Table 4. Timer feature comparison ..... 17
Table 5. Medium-density STM32F103xx pin definitions ..... 28
Table 6. Voltage characteristics ..... 37
Table 7. Current characteristics ..... 37
Table 8. Thermal characteristics ..... 37
Table 9. General operating conditions ..... 38
Table 10. Operating conditions at power-up / power-down ..... 39
Table 11. Embedded reset and power control block characteristics. ..... 39
Table 12. Embedded internal reference voltage ..... 40
Table 13. Maximum current consumption in Run mode, code with data processing running from Flash ..... 41
Table 14. Maximum current consumption in Run mode, code with data processing running from RAM ..... 41
Table 15. Maximum current consumption in Sleep mode, code running from Flash or RAM ..... 43
Table 16. Typical and maximum current consumptions in Stop and Standby modes ..... 44
Table 17. Typical current consumption in Run mode, code with data processing running from Flash ..... 47
Table 18. Typical current consumption in Sleep mode, code running from Flash or RAM ..... 48
Table 19. Peripheral current consumption ..... 49
Table 20. High-speed external user clock characteristics. ..... 50
Table 21. Low-speed external user clock characteristics ..... 51
Table 22. HSE 4-16 MHz oscillator characteristics ..... 52
Table 23. LSE oscillator characteristics ( $\mathrm{f}_{\mathrm{LSE}}=32.768 \mathrm{kHz}$ ) ..... 53
Table 24. HSI oscillator characteristics ..... 54
Table 25. LSI oscillator characteristics ..... 55
Table 26. Low-power mode wakeup timings ..... 56
Table 27. PLL characteristics ..... 56
Table 28. Flash memory characteristics ..... 56
Table 29. Flash memory endurance and data retention ..... 57
Table 30. EMS characteristics ..... 58
Table 31. EMI characteristics for $\mathrm{fHSE}=8 \mathrm{MHz}$ and $\mathrm{fHCLK}=48 \mathrm{MHz}$ ..... 58
Table 32. EMI characteristics for $\mathrm{fHSE}=8 \mathrm{MHz}$ and $\mathrm{fHCLK}=72 \mathrm{MHz}$ ..... 59
Table 33. ESD absolute maximum ratings ..... 59
Table 34. Electrical sensitivities ..... 59
Table 35. I/O current injection susceptibility ..... 60
Table 36. I/O static characteristics ..... 61
Table 37. Output voltage characteristics ..... 64
Table 38. I/O AC characteristics ..... 65
Table 39. NRST pin characteristics ..... 66
Table 40. TIMx characteristics ..... 67
Table 41. $\mathrm{I}^{2} \mathrm{C}$ characteristics ..... 68
Table 42. SCL frequency ( $\mathrm{f}_{\text {PCLK1 }}=36 \mathrm{MHz}, \mathrm{V}_{\mathrm{DD} \_12 \mathrm{C}}=3.3 \mathrm{~V}$ ) ..... 69
Table 43. SPI characteristics ..... 70Table 44. USB startup time ..... 72
Table 45. USB DC electrical characteristics ..... 73
Table 46. USB: Full-speed electrical characteristics. ..... 73
Table 47. ADC characteristics ..... 74
Table 48. $\mathrm{R}_{\mathrm{AIN}}$ max for $\mathrm{f}_{\mathrm{ADC}}=14 \mathrm{MHz}$ ..... 75
Table 49. ADC accuracy - Limited test conditions ..... 75
Table 50. ADC accuracy ..... 76
Table 51. TS characteristics ..... 78
Table 52. VFQFPN - 36 pin, $6 \times 6 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch very thin profile fine pitch quad flat package mechanical data ..... 81
Table 53. UFQFPN48 - Mechanical data ..... 84
Table 54. LFBGA100 - 100-ball low profile fine pitch ball grid array, $10 \times 10 \mathrm{~mm}$, 0.8 mm pitch, package mechanical data ..... 86
Table 55. LFBGA100 recommended PCB design rules ( 0.8 mm pitch BGA) ..... 87
Table 56. LQFP100 - Mechanical data ..... 89
Table 57. UFBGA100 - Mechanical data ..... 92
Table 58. UFBGA100 - Example of PCB design rules ( 0.5 mm pitch BGA) ..... 93
Table 59. LQFP64 - Mechanical data ..... 95
Table 60. TFBGA64 - 64-ball, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, thin profile fine pitch ball grid array package mechanical data ..... 97
Table 61. TFBGA64 recommended PCB design rules ( 0.5 mm pitch BGA) ..... 98
Table 62. LQFP48 - Mechanical data ..... 100
Table 63. Package thermal characteristics ..... 102
Table 64. Document revision history ..... 107# List of figures 

Figure 1. STM32F103xx performance line block diagram ..... 11
Figure 2. Clock tree ..... 12
Figure 3. STM32F103xx performance line LFBGA100 ballout ..... 21
Figure 4. STM32F103xx performance line LQFP100 pinout ..... 22
Figure 5. STM32F103xx performance line UFBGA100 pinout ..... 23
Figure 6. STM32F103xx performance line LQFP64 pinout ..... 24
Figure 7. STM32F103xx performance line TFBGA64 ballout ..... 25
Figure 8. STM32F103xx performance line LQFP48 pinout ..... 26
Figure 9. STM32F103xx performance line UFQFPN48 pinout ..... 26
Figure 10. STM32F103xx performance line VFQFPN36 pinout ..... 27
Figure 11. Memory map ..... 34
Figure 12. Pin loading conditions ..... 35
Figure 13. Pin input voltage ..... 35
Figure 14. Power supply scheme ..... 36
Figure 15. Current consumption measurement scheme ..... 36
Figure 16. Typical current consumption in Run mode versus frequency (at 3.6 V ), code with data processing running from RAM, peripherals enabled. ..... 42
Figure 17. Typical current consumption in Run mode versus frequency (at 3.6 V ), code with data processing running from RAM, peripherals disabled ..... 42
Figure 18. Typical current consumption on $\mathrm{V}_{\mathrm{BAT}}$ (RTC on) ..... 44
Figure 19. Typical current consumption in Stop mode, with regulator in Run mode ..... 45
Figure 20. Typical current consumption in Stop mode, with regulator in Low-power mode ..... 45
Figure 21. Typical current consumption in Standby mode ..... 46
Figure 22. High-speed external clock source AC timing diagram ..... 51
Figure 23. Low-speed external clock source AC timing diagram ..... 52
Figure 24. Typical application with an 8 MHz crystal ..... 53
Figure 25. Typical application with a 32.768 kHz crystal ..... 54
Figure 26. Standard I/O input characteristics - CMOS port ..... 62
Figure 27. Standard I/O input characteristics - TTL port ..... 62
Figure 28. 5 V tolerant I/O input characteristics - CMOS port ..... 63
Figure 29. 5 V tolerant I/O input characteristics - TTL port ..... 63
Figure 30. I/O AC characteristics definition ..... 66
Figure 31. Recommended NRST pin protection ..... 67
Figure 32. $\mathrm{I}^{2} \mathrm{C}$ bus AC waveforms and measurement circuit ..... 69
Figure 33. SPI timing diagram - slave mode and CPHA $=0$ ..... 71
Figure 34. SPI timing diagram - slave mode and CPHA $=1$ ..... 71
Figure 35. SPI timing diagram - master mode ..... 72
Figure 36. USB timings: definition of data signal rise and fall time ..... 73
Figure 37. ADC accuracy characteristics ..... 76
Figure 38. Typical connection diagram using the ADC ..... 77
Figure 39. Power supply and reference decoupling ( $\mathrm{V}_{\text {REF }}$, not connected to $\mathrm{V}_{\text {DDA }}$ ). ..... 77
Figure 40. Power supply and reference decoupling ( $\mathrm{V}_{\text {REF }}$, connected to $\mathrm{V}_{\text {DDA }}$ ). ..... 78
Figure 41. VFQFPN - 36 pin, $6 \times 6 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch very thin profile fine pitch quad flat package outline. ..... 80
Figure 42. VFQFPN - 36 pin, $6 \times 6 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch very thin profile fine pitch quad flat package recommended footprint ..... 82
Figure 43. UFQFPN48 - Outline ..... 83
Figure 44. UFQFPN48 - Footprint example ..... 85Figure 45. LFBGA100 - 100-ball low profile fine pitch ball grid array, $10 \times 10 \mathrm{~mm}$, 0.8 mm pitch, package outline ..... 85
Figure 46. LFBGA100 - 100-ball low profile fine pitch ball grid array, $10 \times 10 \mathrm{~mm}$, 0.8 mm pitch, package recommended footprint ..... 86
Figure 47. LQFP100 - Outline ${ }^{(15)}$ ..... 88
Figure 48. LQFP100 - Footprint example ..... 90
Figure 49. UFBGA100 - Outline ${ }^{(13)}$ ..... 91
Figure 50. UFBGA100 - Footprint example ..... 93
Figure 51. LQFP64 - Outline ${ }^{(15)}$ ..... 94
Figure 52. LQFP64 - Footprint example ..... 96
Figure 53. TFBGA64 - 64-ball, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch thin profile fine pitch ball grid array package outline. ..... 97
Figure 54. TFBGA64 - 64-ball, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, thin profile fine pitch ball grid array , recommended footprint ..... 98
Figure 55. LQFP48 - Outline ${ }^{(15)}$ ..... 99
Figure 56. LQFP48 - Footprint example ..... 101
Figure 57. LQFP100 $\mathrm{P}_{\mathrm{D}}$ max vs. $\mathrm{T}_{\mathrm{A}}$ ..... 104# 1 Introduction 

This document provides the ordering information and mechanical device characteristics of the STM32F103x8 and STM32F103xB medium-density performance line microcontrollers. For more details on the whole STMicroelectronics STM32F103xx family, refer to Section 2.2: Full compatibility throughout the family.

The medium-density STM32F103xx datasheet must be read in conjunction with the low-, medium-, and high-density STM32F10xxx reference manual. For information on the device errata with respect to the datasheet and reference manual, refer to the STM32F103x8/B errata sheet (ES096). The errata sheet, reference manual, and flash programming manual are all available on the STMicroelectronics website www.st.com.
For information on the Arm ${ }^{\circledR(\mathrm{a})}$ Cortex ${ }^{\circledR}$-M3 core refer to the Cortex ${ }^{\circledR}$-M3 Technical Reference Manual, available from the www.arm.com website.

## 2 Description

The STM32F103xx medium-density performance line family incorporates the high-performance Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M3 32-bit RISC core operating at a 72 MHz frequency, high-speed embedded memories (Flash memory up to 128 Kbytes and SRAM up to 20 Kbytes ), and an extensive range of enhanced I/Os and peripherals connected to two APB buses. All devices offer two 12-bit ADCs, three general purpose 16-bit timers plus one PWM timer, as well as standard and advanced communication interfaces: up to two $\mathrm{I}^{2} \mathrm{Cs}$ and SPIs, three USARTs, an USB and a CAN.

The devices operate from a 2.0 to 3.6 V power supply. They are available in both the -40 to $+85^{\circ} \mathrm{C}$ temperature range and the -40 to $+105^{\circ} \mathrm{C}$ extended temperature range. A comprehensive set of power-saving mode allows the design of low-power applications.

The STM32F103xx medium-density performance line family includes devices in six different package types: from 36 pins to 100 pins. Depending on the device chosen, different sets of peripherals are included, the description below gives an overview of the complete range of peripherals proposed in this family.

These features make the STM32F103xx medium-density performance line microcontroller family suitable for a wide range of applications such as motor drives, application control, medical and handheld equipment, PC and gaming peripherals, GPS platforms, industrial applications, PLCs, inverters, printers, scanners, alarm systems, video intercoms, and HVACs.

## arm

[^0]
[^0]:    a. Arm is a registered trademark of Arm Limited (or its subsidiaries) in the US and/or elsewhere.# 2.1 Device overview 

Table 2. STM32F103xx medium-density device features and peripheral counts

| Peripheral | STM32F103Tx |  | STM32F103Cx | STM32F103Rx |  | STM32F103Vx |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Flash - Kbytes | 64 | 128 | 64 | 128 | 64 | 128 | 64 | 128 |
| SRAM - Kbytes | 20 |  | 20 |  | 20 |  | 20 |
| Timers | General-purpose | 3 |  | 3 |  | 3 |  | 3 |
|  | Advanced-control | 1 |  | 1 |  | 1 |  | 1 |
| Communication | SPI | 1 |  | 2 |  | 2 |  | 2 |
|  | $I^{2} \mathrm{C}$ | 1 |  | 2 |  | 2 |  | 2 |
|  | USART | 2 |  | 3 |  | 3 |  | 3 |
|  | USB | 1 |  | 1 |  | 1 |  | 1 |
|  | CAN | 1 |  | 1 |  | 1 |  | 1 |
| GPIOs | 26 |  | 37 |  | 51 |  | 80 |  |
| 12-bit synchronized ADC Number of channels | $\begin{gathered} 2 \\ 10 \text { channels } \end{gathered}$ |  | $\begin{gathered} 2 \\ 10 \text { channels } \end{gathered}$ |  | $\begin{gathered} 2 \\ 16 \text { channels } \end{gathered}$ |  | $\begin{gathered} 2 \\ 16 \text { channels } \end{gathered}$ |  |
| CPU frequency | 72 MHz |  |  |  |  |  |  |  |
| Operating voltage | 2.0 to 3.6 V |  |  |  |  |  |  |  |
| Operating temperatures | Ambient temperatures: -40 to $+85^{\circ} \mathrm{C} /-40$ to $+105^{\circ} \mathrm{C}$ (see Table 9) Junction temperature: -40 to $+125^{\circ} \mathrm{C}$ (see Table 9) |  |  |  |  |  |  |  |
| Packages | VFQFPN36 |  | LQFP48, <br> UFQFPN48 |  | LQFP64, <br> TFBGA64 |  | LQFP100, <br> LFBGA100, <br> UFBGA100 |  |

1. On the TFBGA64 package only 15 channels are available (one analog input pin has been replaced by $\mathrm{V}_{\text {REF+ }}$ ).
![img-1.jpeg](img-1.jpeg)Figure 1. STM32F103xx performance line block diagram
![img-2.jpeg](img-2.jpeg)

1. $T_{A}=-40^{\circ} \mathrm{C}$ to $+105^{\circ} \mathrm{C}$ (junction temperature up to $125^{\circ} \mathrm{C}$ ).
2. $A F=$ alternate function on I/O port pin.Figure 2. Clock tree
![img-3.jpeg](img-3.jpeg)

1. When the HSI is used as a PLL clock input, the maximum system clock frequency that can be achieved is 64 MHz .
2. For the availability of the USB function both HSE and PLL must be enabled, with USBCLK running at 48 MHz .
3. To have an ADC conversion time of $1 \mu \mathrm{~s}$, APB2 must be at $14 \mathrm{MHz}, 28 \mathrm{MHz}$, or 56 MHz .# 2.2 Full compatibility throughout the family 

STM32F103xx is a complete family whose members are fully pin-to-pin, software, and feature compatible. In the reference manual, STM32F103x4 and STM32F103x6 are identified as low-density devices, STM32F103x8 and STM32F103xB are referred to as medium-density devices, and STM32F103xC, STM32F103xD, and STM32F103xE are referred to as high-density devices.

Low- and high-density devices are an extension of the STM32F103x8/B devices; they are specified in the STM32F103x4/6 and STM32F103xC/D/E datasheets, respectively. Low-density devices feature lower flash memory and RAM capacities, and fewer timers and peripherals. High-density devices have higher flash memory and RAM capacities, and additional peripherals like SDIO, FSMC, I²S, and DAC, while remaining fully compatible with the other members of the STM32F103xx family.

The STM32F103x4, STM32F103x6, STM32F103xC, STM32F103xD and STM32F103xE are a drop-in replacement for STM32F103x8/B medium-density devices, allowing the user to try different memory densities and providing a greater degree of freedom during the development cycle.

Moreover, the STM32F103xx performance line family is fully compatible with all existing STM32F101xx access line and STM32F102xx USB access line devices.

Table 3. STM32F103xx family

| Pinout | Low-density devices |  | Medium-density devices |  | High-density devices |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | 16 KB <br> Flash | 32 KB <br> Flash | 64 KB <br> Flash | 128 KB <br> Flash | 256 KB <br> Flash | 384 KB <br> Flash | 512 KB <br> Flash |
|  | 6 KB RAM | 10 KB RAM | 20 KB RAM | 20 KB RAM | 48 KB RAM | 64 KB RAM | 64 KB RAM |
| 144 | - | - | - | - | 5× USARTs |  |  |
| 100 | - | - | 3× USARTs <br> $3 \times 16$-bit timers <br> $2 \times$ SPIs, $2 \times I^{2} \mathrm{Cs}$, USB, <br> CAN, $1 \times$ PWM timer <br> $2 \times$ ADCs |  | 4× 16-bit timers, $2 \times$ basic timers $3 \times$ SPIs, $2 \times I^{2} \mathrm{Ss}, 2 \times 12 \mathrm{Cs}$ <br> USB, CAN, $2 \times$ PWM timers $3 \times$ ADCs, $2 \times$ DACs, $1 \times$ SDIO <br> FSMC (100 and 144 pins) |  |  |
| 64 | 2× USARTs <br> $2 \times 16$-bit timers <br> $1 \times$ SPI, $1 \times I^{2} \mathrm{C}$, USB, <br> CAN, $1 \times$ PWM timer <br> $2 \times$ ADCs |  |  |  | - | - | - |
| 48 |  |  |  |  | - | - | - |
| 36 |  |  |  |  | - | - | - |# 2.3 Overview 

### 2.3.1 Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M3 core with embedded flash and SRAM

The Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M3 processor is the latest generation of Arm ${ }^{\circledR}$ processors for embedded systems. It has been developed to provide a low-cost platform that meets the needs of MCU implementation, with a reduced pin count and low-power consumption, while delivering outstanding computational performance and an advanced system response to interrupts.
The Arm ${ }^{\circledR}$ Cortex ${ }^{\circledR}$-M3 32-bit RISC processor features exceptional code-efficiency, delivering the high-performance expected from an Arm ${ }^{\circledR}$ core in the memory size usually associated with 8 - and 16-bit devices.
The STM32F103xx performance line family having an embedded Arm ${ }^{\circledR}$ core, it is compatible with all Arm ${ }^{\circledR}$ tools and software.
Figure 1 shows the general block diagram of the device family.

### 2.3.2 Embedded flash memory

64 or 128 Kbytes of embedded flash memory is available for storing programs and data.

### 2.3.3 CRC (cyclic redundancy check) calculation unit

The CRC (cyclic redundancy check) calculation unit is used to get a CRC code from a 32-bit data word and a fixed generator polynomial.
Among other applications, CRC-based techniques are used to verify data transmission or storage integrity. In the scope of the EN/IEC 60335-1 standard, they offer a means of verifying the flash memory integrity. The CRC calculation unit helps compute a signature of the software during runtime, to be compared with a reference signature generated at linktime and stored at a given memory location.

### 2.3.4 Embedded SRAM

Twenty Kbytes of embedded SRAM accessed (read/write) at CPU clock speed with 0 wait states.

### 2.3.5 Nested vectored interrupt controller (NVIC)

The STM32F103xx performance line embeds a nested vectored interrupt controller able to handle up to 43 maskable interrupt channels (not including the 16 interrupt lines of Cortex ${ }^{\circledR}$ M3) and 16 priority levels.

- Closely coupled NVIC gives low-latency interrupt processing
- Interrupt entry vector table address passed directly to the core
- Closely coupled NVIC core interface
- Allows early processing of interrupts
- Processing of late arriving higher priority interrupts
- Support for tail-chaining
- Processor state automatically saved
- Interrupt entry restored on interrupt exit with no instruction overheadThis hardware block provides flexible interrupt management features with minimal interrupt latency.

# 2.3.6 External interrupt/event controller (EXTI) 

The external interrupt/event controller consists of 19 edge detector lines used to generate interrupt/event requests. Each line can be independently configured to select the trigger event (rising edge, falling edge, both) and can be masked independently. A pending register maintains the status of the interrupt requests. The EXTI can detect an external line with a pulse width shorter than the internal APB2 clock period. Up to 80 GPIOs can be connected to the 16 external interrupt lines.

### 2.3.7 Clocks and startup

System clock selection is performed on startup, but the internal RC 8 MHz oscillator is selected as the default CPU clock on reset. An external 4-16 MHz clock can be selected, in which case it is monitored for failure. If failure is detected, the system automatically switches back to the internal RC oscillator. A software interrupt is generated if enabled. Similarly, full interrupt management of the PLL clock entry is available when necessary (for example, on failure of an indirectly used external crystal, resonator, or oscillator).

Several prescalers allow the configuration of the AHB frequency, the high-speed APB (APB2) and the low-speed APB (APB1) domains. The maximum frequency of the AHB and the high-speed APB domains is 72 MHz . The maximum allowed frequency of the low-speed APB domain is 36 MHz . See Figure 2 for details on the clock tree.

### 2.3.8 Boot modes

At startup, boot pins are used to select one of three boot options:

- Boot from user Flash
- Boot from system memory
- Boot from embedded SRAM

The bootloader is located in the system memory. It is used to reprogram the flash memory by using USART1. For further details, refer to AN2606, available on www.st.com.

### 2.3.9 Power supply schemes

- $\mathrm{V}_{\mathrm{DD}}=2.0$ to 3.6 V : external power supply for I/Os and the internal regulator. Provided externally through $\mathrm{V}_{\mathrm{DD}}$ pins.
- $\mathrm{V}_{\mathrm{SSA}}, \mathrm{V}_{\mathrm{DDA}}=2.0$ to 3.6 V : external analog power supplies for ADC , reset blocks, RCs , and PLL (minimum voltage to be applied to $\mathrm{V}_{\mathrm{DDA}}$ is 2.4 V when the ADC is used). $\mathrm{V}_{\mathrm{DDA}}$ and $\mathrm{V}_{\mathrm{SSA}}$ must be connected to $\mathrm{V}_{\mathrm{DD}}$ and $\mathrm{V}_{\mathrm{SS}}$, respectively.
- $\quad \mathrm{V}_{\mathrm{BAT}}=1.8$ to 3.6 V : power supply for RTC, external clock 32 kHz oscillator and backup registers (through power switch) when $\mathrm{V}_{\mathrm{DD}}$ is not present.

For more details on how to connect power pins, refer to Figure 14: Power supply scheme.

### 2.3.10 Power supply supervisor

The device has an integrated power-on reset (POR)/power-down reset (PDR) circuitry. It is always active, and ensures proper operation starting from/down to 2 V . The device remainsin reset mode when $\mathrm{V}_{\mathrm{DD}}$ is below a specified threshold, $\mathrm{V}_{\text {POR/PDR }}$, without the need for an external reset circuit.

The device features an embedded programmable voltage detector (PVD) that monitors the $\mathrm{V}_{\mathrm{DD}} / \mathrm{V}_{\mathrm{DDA}}$ power supply and compares it to the $\mathrm{V}_{\mathrm{PVD}}$ threshold. An interrupt can be generated when $\mathrm{V}_{\mathrm{DD}} / \mathrm{V}_{\mathrm{DDA}}$ drops below the $\mathrm{V}_{\mathrm{PVD}}$ threshold and/or when $\mathrm{V}_{\mathrm{DD}} / \mathrm{V}_{\mathrm{DDA}}$ is higher than the $\mathrm{V}_{\mathrm{PVD}}$ threshold. The interrupt service routine can then generate a warning message and/or put the MCU into a safe state. The PVD is enabled by software.
Refer to Table 11 for the values of $\mathrm{V}_{\text {POR/PDR }}$ and $\mathrm{V}_{\text {PVD }}$.

# 2.3.11 Voltage regulator 

The regulator has three operation modes: main (MR), low-power (LPR) and power down.

- MR is used in the nominal regulation mode (Run)
- LPR is used in the Stop mode
- Power down is used in Standby mode: the regulator output is in high impedance: the kernel circuitry is powered down, inducing zero consumption (but the contents of the registers and SRAM are lost)

This regulator is always enabled after reset. It is disabled in Standby mode, providing high impedance output.

### 2.3.12 Low-power modes

The STM32F103xx performance line supports three low-power modes to achieve the best compromise between low-power consumption, short startup time and available wakeup sources:

- Sleep mode

In Sleep mode, only the CPU is stopped. All peripherals continue to operate and can wake up the CPU when an interrupt/event occurs.

- Stop mode

The Stop mode achieves the lowest power consumption while retaining the content of SRAM and registers. All clocks in the 1.8 V domain are stopped, the PLL, the HSI RC and the HSE crystal oscillators are disabled. The voltage regulator can also be put either in normal or in low-power mode.
The device can be woken up from Stop mode by any of the EXTI lines. The EXTI line source can be one of the 16 external lines, the PVD output, the RTC alarm or the USB wakeup.

- Standby mode

The Standby mode is used to achieve the lowest power consumption. The internal voltage regulator is switched off so that the entire 1.8 V domain is powered off. The PLL, the HSI RC and the HSE crystal oscillators are also switched off. After entering Standby mode, SRAM and register contents are lost except for registers in the Backup domain and Standby circuitry.
The device exits Standby mode when an external reset (NRST pin), an IWDG reset, a rising edge on the WKUP pin, or an RTC alarm occurs.
Note: The RTC, the IWDG, and the corresponding clock sources are not stopped by entering Stop or Standby mode.# 2.3.13 DMA 

The flexible 7-channel general-purpose DMA is able to manage memory-to-memory, peripheral-to-memory and memory-to-peripheral transfers. The DMA controller supports circular buffer management avoiding the generation of interrupts when the controller reaches the end of the buffer.

Each channel is connected to dedicated hardware DMA requests, with support for software trigger on each channel. Configuration is made by software and transfer sizes between source and destination are independent.
The DMA can be used with the main peripherals: SPI, I²C, USART, general-purpose and advanced-control timers TIMx and ADC.

### 2.3.14 RTC (real-time clock) and backup registers

The RTC and the backup registers are supplied through a switch that takes power either on $\mathrm{V}_{\mathrm{DD}}$ supply when present or through the $\mathrm{V}_{\mathrm{BAT}}$ pin. The backup registers are ten 16-bit registers used to store 20 bytes of user application data when $\mathrm{V}_{\mathrm{DD}}$ power is not present.
The real-time clock provides a set of continuously running counters which can be used with suitable software to provide a clock calendar function, and provides an alarm interrupt and a periodic interrupt. It is clocked by a 32.768 kHz external crystal, resonator or oscillator, the internal low-power RC oscillator or the high-speed external clock divided by 128. The internal low-power RC has a typical frequency of 40 kHz . The RTC can be calibrated using an external 512 Hz output to compensate for any natural crystal deviation. The RTC features a 32-bit programmable counter for long-term measurement using the Compare register to generate an alarm. A 20-bit prescaler is used for the time base clock and is by default configured to generate a time base of 1 second from a clock at 32.768 kHz .

### 2.3.15 Timers and watchdogs

The medium-density STM32F103xx performance line devices include an advanced-control timer, three general-purpose timers, two watchdog timers and a SysTick timer.

Table 4 compares the features of the advanced-control and general-purpose timers.
Table 4. Timer feature comparison

| Timer | Counter <br> resolution | Counter <br> type | Prescaler <br> factor | DMA request <br> generation | Capture/compare <br> channels | Complementary <br> outputs |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| TIM1 | 16-bit | Up, <br> down, <br> up/down | Any integer <br> between 1 <br> and 65536 | Yes | 4 | Yes |
| TIM2, <br> TIM3, <br> TIM4 | 16-bit | Up, <br> down, <br> up/down | Any integer <br> between 1 <br> and 65536 | Yes | 4 | No |# Advanced-control timer (TIM1) 

The advanced-control timer (TIM1) can be seen as a three-phase PWM multiplexed on 6 channels. It has complementary PWM outputs with programmable inserted dead-times. It can also be seen as a complete general-purpose timer. The 4 independent channels can be used for

- Input capture
- Output compare
- PWM generation (edge- or center-aligned modes)
- One-pulse mode output

If configured as a general-purpose 16-bit timer, it has the same features as the TIMx timer. If configured as the 16-bit PWM generator, it has full modulation capability ( $0-100 \%$ ).

In debug mode, the advanced-control timer counter can be frozen and the PWM outputs disabled to turn off any power switch driven by these outputs.

Many features are shared with those of the general-purpose TIM timers which have the same architecture. The advanced-control timer can therefore work together with the TIM timers via the Timer Link feature for synchronization or event chaining.

## General-purpose timers (TIMx)

There are up to three synchronizable general-purpose timers embedded in the STM32F103xx performance line devices. These timers are based on a 16-bit auto-reload up/down counter, a 16-bit prescaler and feature four independent channels each for input capture/output compare, PWM or one-pulse mode output. This gives up to 12 input captures/output compares/PWMs on the largest packages.

The general-purpose timers can work together with the advanced-control timer via the Timer Link feature for synchronization or event chaining. Their counter can be frozen in debug mode. Any of the general-purpose timers can be used to generate PWM outputs. They all have independent DMA request generation.

These timers are capable of handling quadrature (incremental) encoder signals and the digital outputs from one to three Hall-effect sensors.

## Independent watchdog

The independent watchdog is based on a 12-bit downcounter and 8-bit prescaler. It is clocked from an independent 40 kHz internal RC and as it operates independently of the main clock, it can operate in Stop and Standby modes. It can be used either as a watchdog to reset the device when a problem occurs, or as a free-running timer for application timeout management. It is hardware- or software-configurable through the option bytes. The counter can be frozen in debug mode.

## Window watchdog

The window watchdog is based on a 7-bit downcounter that can be set as free-running. It can be used as a watchdog to reset the device when a problem occurs. It is clocked from the main clock. It has an early warning interrupt capability and the counter can be frozen in debug mode.# SysTick timer 

This timer is dedicated for OS, but can be used also as a standard downcounter. It features:

- A 24-bit downcounter
- Autoreload capability
- Maskable system interrupt generation when the counter reaches 0
- Programmable clock source


### 2.3.16 $\mathrm{I}^{2} \mathrm{C}$ bus

Up to two $\mathrm{I}^{2} \mathrm{C}$ bus interfaces can operate in multimaster and slave modes. They can support standard and fast modes.

They support dual slave addressing (7-bit only) and both 7/10-bit addressing in master mode. A hardware CRC generation/verification is embedded.

They can be served by DMA and they support SM Bus 2.0/PM Bus.

### 2.3.17 Universal synchronous/asynchronous receiver transmitter (USART)

One of the USART interfaces is able to communicate at speeds of up to $4.5 \mathrm{Mbit} / \mathrm{s}$. The other available interfaces communicate at up to $2.25 \mathrm{Mbit} / \mathrm{s}$. They provide hardware management of the CTS and RTS signals, IrDA SIR ENDEC support, are ISO 7816 compliant and have LIN Master/Slave capability.

All USART interfaces can be served by the DMA controller.

### 2.3.18 Serial peripheral interface (SPI)

Up to two SPIs are able to communicate up to $18 \mathrm{Mbits} / \mathrm{s}$ in slave and master modes in full-duplex and simplex communication modes. The 3-bit prescaler gives 8 master mode frequencies and the frame is configurable to 8 bits or 16 bits. The hardware CRC generation/verification supports basic SD Card/MMC modes.

Both SPIs can be served by the DMA controller.

### 2.3.19 Controller area network (CAN)

The CAN is compliant with specifications 2.0A and B (active) with a bit rate up to $1 \mathrm{Mbit} / \mathrm{s}$. It can receive and transmit standard frames with 11-bit identifiers as well as extended frames with 29-bit identifiers. It has three transmit mailboxes, two receive FIFOs with three stages and 14 scalable filter banks.

### 2.3.20 Universal serial bus (USB)

The STM32F103xx performance line embeds a USB device peripheral compatible with the USB full-speed 12 Mbs . The USB interface implements a full-speed ( $12 \mathrm{Mbit} / \mathrm{s}$ ) function interface. It has software-configurable endpoint setting and suspend/resume support. The dedicated 48 MHz clock is generated from the internal main PLL (the clock source must use a HSE crystal oscillator).# 2.3.21 GPIOs (general-purpose inputs/outputs) 

Each of the GPIO pins can be configured by software as output (push-pull or open-drain), as input (with or without pull-up or pull-down) or as peripheral alternate function. Most of the GPIO pins are shared with digital or analog alternate functions. All GPIOs are high currentcapable.

The I/Os alternate function configuration can be locked if needed following a specific sequence in order to avoid spurious writing to the I/Os registers.
I/Os on APB2 with up to 18 MHz toggling speed.

### 2.3.22 ADC (analog-to-digital converter)

Two 12-bit analog-to-digital converters are embedded into STM32F103xx performance line devices and each ADC shares up to 16 external channels, performing conversions in singleshot or scan modes. In scan mode, automatic conversion is performed on a selected group of analog inputs.

Additional logic functions embedded in the ADC interface allow:

- Simultaneous sample and hold
- Interleaved sample and hold
- Single shunt

The ADC can be served by the DMA controller.
An analog watchdog feature allows very precise monitoring of the converted voltage of one, some or all selected channels. An interrupt is generated when the converted voltage is outside the programmed thresholds.

The events generated by the general-purpose timers (TIMx) and the advanced-control timer (TIM1) can be internally connected to the ADC start trigger, injection trigger, and DMA trigger respectively, to allow the application to synchronize A/D conversion and timers.

### 2.3.23 Temperature sensor

The temperature sensor has to generate a voltage that varies linearly with temperature. The conversion range is between $2 \mathrm{~V}<\mathrm{V}_{\mathrm{DDA}}<3.6 \mathrm{~V}$. The temperature sensor is internally connected to the ADC12_IN16 input channel which is used to convert the sensor output voltage into a digital value.

### 2.3.24 Serial wire JTAG debug port (SWJ-DP)

The Arm SWJ-DP Interface is embedded. and is a combined JTAG and serial wire debug port that enables either a serial wire debug or a JTAG probe to be connected to the target. The JTAG TMS and TCK pins are shared with SWDIO and SWCLK, respectively, and a specific sequence on the TMS pin is used to switch between JTAG-DP and SW-DP.# 3 Pinouts and pin description 

Figure 3. STM32F103xx performance line LFBGA100 ballout
![img-4.jpeg](img-4.jpeg)Figure 4. STM32F103xx performance line LQFP100 pinout
![img-5.jpeg](img-5.jpeg)Figure 5. STM32F103xx performance line UFBGA100 pinout
![img-6.jpeg](img-6.jpeg)Figure 6. STM32F103xx performance line LQFP64 pinout
![img-7.jpeg](img-7.jpeg)Figure 7. STM32F103xx performance line TFBGA64 ballout
![img-8.jpeg](img-8.jpeg)Figure 8. STM32F103xx performance line LQFP48 pinout
![img-9.jpeg](img-9.jpeg)

Figure 9. STM32F103xx performance line UFQFPN48 pinout
![img-10.jpeg](img-10.jpeg)Figure 10. STM32F103xx performance line VFQFPN36 pinout
![img-11.jpeg](img-11.jpeg)Table 5. Medium-density STM32F103xx pin definitions

|  |  |  |  |  |  |  | Pin name |  |  |  | Alternate functions ${ }^{(4)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  |  |  |  |  |  |  | Main function ${ }^{(3)}$ (after reset) | Default |  | Remap |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| A3 | B2 | - | - | - | 1 | - | PE2 | I/O | FT | PE2 | TRACECK |  | - |
| B3 | A1 | - | - | - | 2 | - | PE3 | I/O | FT | PE3 | TRACED0 |  | - |
| C3 | B1 | - | - | - | 3 | - | PE4 | I/O | FT | PE4 | TRACED1 |  | - |
| D3 | C2 | - | - | - | 4 | - | PE5 | I/O | FT | PE5 | TRACED2 |  | - |
| E3 | D2 | - | - | - | 5 | - | PE6 | I/O | FT | PE6 | TRACED3 |  | - |
| B2 | E2 | 1 | B2 | 1 | 6 | - | $\mathrm{V}_{\text {BAT }}$ | S | - | $\mathrm{V}_{\text {BAT }}$ | - |  | - |
| A2 | C1 | 2 | A2 | 2 | 7 | - | PC13-TAMPER- <br> RTC ${ }^{(5)}$ | I/O | - | PC13 ${ }^{(6)}$ | TAMPER-RTC |  | - |
| A1 | D1 | 3 | A1 | 3 | 8 | - | PC14-OSC32_IN ${ }^{(5)}$ | I/O | - | PC14 ${ }^{(6)}$ | OSC32_IN |  | - |
| B1 | E1 | 4 | B1 | 4 | 9 | - | $\begin{gathered} \text { PC15- } \\ \text { OSC32_OUT }^{(5)} \end{gathered}$ | I/O | - | PC15 ${ }^{(6)}$ | OSC32_OUT |  | - |
| C2 | F2 | - | - | - | 10 | - | $\mathrm{V}_{\text {SS_5 }}$ | S | - | $\mathrm{V}_{\text {SS_5 }}$ | - |  | - |
| D2 | G2 | - | - | - | 11 | - | $\mathrm{V}_{\text {DD_5 }}$ | S | - | $\mathrm{V}_{\text {DD_5 }}$ | - |  |  |
| C1 | F1 | 5 | C1 | 5 | 12 | 2 | OSC_IN | I | - | OSC_IN | - | PD0 ${ }^{(7)}$ |  |
| D1 | G1 | 6 | D1 | 6 | 13 | 3 | OSC_OUT | 0 | - | OSC_OUT |  | PD1 ${ }^{(7)}$ |  |
| E1 | H2 | 7 | E1 | 7 | 14 | 4 | NRST | I/O | - | NRST | - |  | - |
| F1 | H1 | - | E3 | 8 | 15 | - | PC0 | I/O | - | PC0 | ADC12_IN10 |  | - |
| F2 | J2 | - | E2 | 9 | 16 | - | PC1 | I/O | - | PC1 | ADC12_IN11 |  | - |
| E2 | J3 | - | F2 | 10 | 17 | - | PC2 | I/O | - | PC2 | ADC12_IN12 |  | - |
| F3 | K2 | - | ${ }^{(8)}$ | 11 | 18 | - | PC3 | I/O | - | PC3 | ADC12_IN13 |  | - |
| G1 | J1 | 8 | F1 | 12 | 19 | 5 | $\mathrm{V}_{\text {SSA }}$ | S | - | $\mathrm{V}_{\text {SSA }}$ | - |  | - |
| H1 | K1 | - | - | - | 20 | - | $\mathrm{V}_{\text {REF }}$ | S | - | $\mathrm{V}_{\text {REF }}$ | - |  | - |
| J1 | L1 | - | G1 ${ }^{(8)}$ | - | 21 | - | $\mathrm{V}_{\text {REF }}$ | S | - | $\mathrm{V}_{\text {REF }}$ | - |  | - |
| K1 | M1 | 9 | H1 | 13 | 22 | 6 | $\mathrm{V}_{\text {DDA }}$ | S | - | $\mathrm{V}_{\text {DDA }}$ | - |  | - |Table 5. Medium-density STM32F103xx pin definitions (continued)

| Pins |  |  |  |  |  |  | Pin name | Type ${ }^{(1)}$ | $\begin{aligned} & \text { I/O Level }^{(2)} \\ & \text { I/O } \\ & \text { I } \end{aligned}$ | Main function ${ }^{(3)}$ (after reset) | Alternate functions ${ }^{(4)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  |  |  |  |  |  |  |  | Default |  | Remap |
| G2 | L2 | 10 | G2 | 14 | 23 | 7 | PA0-WKUP | I/O | - | PA0 | WKUP/ <br> USART2_CTS ${ }^{(9)} /$ <br> ADC12_IN0/ <br> TIM2_CH1_ <br> ETR ${ }^{(9)}$ | - |  |
| H2 | M2 | 11 | H2 | 15 | 24 | 8 | PA1 | I/O | - | PA1 | USART2_RTS ${ }^{(9)} /$ <br> ADC12_IN1/ <br> TIM2_CH2 ${ }^{(9)}$ | - |  |
| J2 | K3 | 12 | F3 | 16 | 25 | 9 | PA2 | I/O | - | PA2 | USART2_TX ${ }^{(9)} /$ <br> ADC12_IN2/ <br> TIM2_CH3 ${ }^{(9)}$ | - |  |
| K2 | L3 | 13 | G3 | 17 | 26 | 10 | PA3 | I/O | - | PA3 | USART2_RX ${ }^{(9)} /$ <br> ADC12_IN3/ <br> TIM2_CH4 ${ }^{(9)}$ | - |  |
| E4 | E3 | - | C2 | 18 | 27 | - | $\mathrm{V}_{\text {SS_4 }}$ | S | - | $\mathrm{V}_{\text {SS_4 }}$ | - | - |  |
| F4 | H3 | - | D2 | 19 | 28 | - | $\mathrm{V}_{\mathrm{DD} _4}$ | S | - | $\mathrm{V}_{\mathrm{DD} _4}$ | - | - |  |
| G3 | M3 | 14 | H3 | 20 | 29 | 11 | PA4 | I/O | - | PA4 | SPI1_NSS ${ }^{(9)} /$ <br> USART2_CK ${ }^{(9)} /$ <br> ADC12_IN4 | - |  |
| H3 | K4 | 15 | F4 | 21 | 30 | 12 | PA5 | I/O | - | PA5 | SPI1_SCK ${ }^{(9)} /$ <br> ADC12_IN5 | - |  |
| J3 | L4 | 16 | G4 | 22 | 31 | 13 | PA6 | I/O | - | PA6 | SPI1_MISO ${ }^{(9)} /$ <br> ADC12_IN6/ <br> TIM3_CH1 ${ }^{(9)}$ | TIM1_BKIN |  |
| K3 | M4 | 17 | H4 | 23 | 32 | 14 | PA7 | I/O | - | PA7 | SPI1_MOSI ${ }^{(9)} /$ <br> ADC12_IN7/ <br> TIM3_CH2 ${ }^{(9)}$ | TIM1_CH1N |  |
| G4 | K5 | - | H5 | 24 | 33 |  | PC4 | I/O | - | PC4 | ADC12_IN14 | - |  |
| H4 | L5 | - | H6 | 25 | 34 |  | PC5 | I/O | - | PC5 | ADC12_IN15 | - |  |
| J4 | M5 | 18 | F5 | 26 | 35 | 15 | PB0 | I/O | - | PB0 | ADC12_IN8/ <br> TIM3_CH3 ${ }^{(9)}$ | TIM1_CH2N |  |
| K4 | M6 | 19 | G5 | 27 | 36 | 16 | PB1 | I/O | - | PB1 | ADC12_IN9/ <br> TIM3_CH4 ${ }^{(9)}$ | TIM1_CH3N |  |Table 5. Medium-density STM32F103xx pin definitions (continued)

| Pins |  |  |  |  |  |  | Pin name | ![img-12.jpeg](img-12.jpeg) | Main function ${ }^{(3)}$ (after reset) | Alternate functions ${ }^{(4)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  |  |  |  |  |  |  | Default | Remap |  |
| G5 | L6 | 20 | G6 | 28 | 37 | 17 | PB2 | I/O | FT | PB2/BOOT1 | - | - |
| H5 | M7 | - | - | - | 38 | - | PE7 | I/O | FT | PE7 | - | TIM1_ETR |
| J5 | L7 | - | - | - | 39 | - | PE8 | I/O | FT | PE8 | - | TIM1_CH1N |
| K5 | M8 | - | - | - | 40 | - | PE9 | I/O | FT | PE9 | - | TIM1_CH1 |
| G6 | L8 | - | - | - | 41 | - | PE10 | I/O | FT | PE10 | - | TIM1_CH2N |
| H6 | M9 | - | - | - | 42 | - | PE11 | I/O | FT | PE11 | - | TIM1_CH2 |
| J6 | L9 | - | - | - | 43 | - | PE12 | I/O | FT | PE12 | - | TIM1_CH3N |
| K6 | M10 | - | - | - | 44 | - | PE13 | I/O | FT | PE13 | - | TIM1_CH3 |
| G7 | M11 | - | - | - | 45 | - | PE14 | I/O | FT | PE14 | - | TIM1_CH4 |
| H7 | M12 | - | - | - | 46 | - | PE15 | I/O | FT | PE15 | - | TIM1_BKIN |
| J7 | L10 | 21 | G7 | 29 | 47 | - | PB10 | I/O | FT | PB10 | I2C2_SCL/ <br> USART3_TX ${ }^{(9)}$ | TIM2_CH3 |
| K7 | L11 | 22 | H7 | 30 | 48 | - | PB11 | I/O | FT | PB11 | I2C2_SDA/ <br> USART3_RX ${ }^{(9)}$ | TIM2_CH4 |
| E7 | F12 | 23 | D6 | 31 | 49 | 18 | $\mathrm{V}_{\text {SS_1 }}$ | S | - | $\mathrm{V}_{\text {SS_1 }}$ | - | - |
| F7 | G12 | 24 | E6 | 32 | 50 | 19 | $\mathrm{V}_{\text {DD_1 }}$ | S | - | $\mathrm{V}_{\text {DD_1 }}$ | - | - |
| K8 | L12 | 25 | H8 | 33 | 51 | - | PB12 | I/O | FT | PB12 | SPI2_NSS/ <br> I2C2_SMBAI/ <br> USART3_CK ${ }^{(9)} /$ <br> TIM1_BKIN ${ }^{(9)}$ | - |
| J8 | K12 | 26 | G8 | 34 | 52 | - | PB13 | I/O | FT | PB13 | SPI2_SCK/ <br> USART3_CTS ${ }^{(9)} /$ <br> TIM1_CH1N ${ }^{(9)}$ | - |
| H8 | K11 | 27 | F8 | 35 | 53 | - | PB14 | I/O | FT | PB14 | SPI2_MISO/ <br> USART3_RTS ${ }^{(9)}$ <br> TIM1_CH2N ${ }^{(9)}$ | - |
| G8 | K10 | 28 | F7 | 36 | 54 | - | PB15 | I/O | FT | PB15 | SPI2_MOSI/ <br> TIM1_CH3N ${ }^{(9)}$ | - |
| K9 | K9 | - | - | - | 55 | - | PD8 | I/O | FT | PD8 | - | USART3_TX |
| J9 | K8 | - | - | - | 56 | - | PD9 | I/O | FT | PD9 | - | USART3_RX |Table 5. Medium-density STM32F103xx pin definitions (continued)

| Pins |  |  |  |  |  |  | Pin name | $\begin{aligned} & \text { ㅁ } \\ & \text { ㅇ } \\ & \text { ㅇ } \\ & \text { ㅇ } \end{aligned}$ | Main function ${ }^{(3)}$ (after reset) | Alternate functions ${ }^{(4)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  |  |  |  |  |  |  |  | Default | Remap |
| H9 | J12 | - | - | - | 57 | - | PD10 | I/O | FT | PD10 | - | USART3_CK |
| G9 | J11 | - | - | - | 58 | - | PD11 | I/O | FT | PD11 | - | USART3_CTS |
| K10 | J10 | - | - | - | 59 | - | PD12 | I/O | FT | PD12 | - | TIM4_CH1 / USART3_RTS |
| J10 | H12 | - | - | - | 60 | - | PD13 | I/O | FT | PD13 | - | TIM4_CH2 |
| H10 | H11 | - | - | - | 61 | - | PD14 | I/O | FT | PD14 | - | TIM4_CH3 |
| G10 | H10 | - | - | - | 62 | - | PD15 | I/O | FT | PD15 | - | TIM4_CH4 |
| F10 | E12 | - | F6 | 37 | 63 | - | PC6 | I/O | FT | PC6 | - | TIM3_CH1 |
| E10 | E11 |  | E7 | 38 | 64 | - | PC7 | I/O | FT | PC7 | - | TIM3_CH2 |
| F9 | E10 |  | E8 | 39 | 65 | - | PC8 | I/O | FT | PC8 | - | TIM3_CH3 |
| E9 | D12 | - | D8 | 40 | 66 | - | PC9 | I/O | FT | PC9 | - | TIM3_CH4 |
| D9 | D11 | 29 | D7 | 41 | 67 | 20 | PA8 | I/O | FT | PA8 | USART1_CK/ <br> TIM1_CH1 ${ }^{(9)} /$ <br> MCO | - |
| C9 | D10 | 30 | C7 | 42 | 68 | 21 | PA9 | I/O | FT | PA9 | USART1_TX ${ }^{(9)} /$ <br> TIM1_CH2 ${ }^{(9)}$ | - |
| D10 | C12 | 31 | C6 | 43 | 69 | 22 | PA10 | I/O | FT | PA10 | USART1_RX ${ }^{(9)} /$ <br> TIM1_CH3 ${ }^{(9)}$ | - |
| C10 | B12 | 32 | C8 | 44 | 70 | 23 | PA11 | I/O | FT | PA11 | USART1_CTS/ <br> CANRX ${ }^{(9)} /$ <br> USBDM/ <br> TIM1_CH4 ${ }^{(9)}$ | - |
| B10 | A12 | 33 | B8 | 45 | 71 | 24 | PA12 | I/O | FT | PA12 | USART1_RTS/ <br> CANTX ${ }^{(9)}$ <br> /USBDP <br> TIM1_ETR ${ }^{(9)}$ | - |
| A10 | A11 | 34 | A8 | 46 | 72 | 25 | PA13 | I/O | FT | JTMS/SWDIO | - | PA13 |
| F8 | C11 | - | - | - | 73 | - | Not connected |  |  |  |  | - |
| E6 | F11 | 35 | D5 | 47 | 74 | 26 | $\mathrm{V}_{\text {SS_2 }}$ | S | - | $\mathrm{V}_{\text {SS_2 }}$ | - | - |
| F6 | G11 | 36 | E5 | 48 | 75 | 27 | $\mathrm{V}_{\mathrm{DD} _2}$ | S | - | $\mathrm{V}_{\mathrm{DD} _2}$ | - | - |Table 5. Medium-density STM32F103xx pin definitions (continued)

| Pins |  |  |  |  |  |  | Pin name | $\begin{aligned} & \text { ㅇ } \\ & \text { ㅇ } \\ & \text { ㅇ } \\ & \text { ㅇ } \end{aligned}$ | Main function ${ }^{(3)}$ (after reset) | Alternate functions ${ }^{(4)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  |  |  |  |  |  |  |  | Default | Remap |
| A9 | A10 | 37 | A7 | 49 | 76 | 28 | PA14 | I/O | FT | JTCK/SWCLK | - | PA14 |
| A8 | A9 | 38 | A6 | 50 | 77 | 29 | PA15 | I/O | FT | JTDI | - | TIM2_CH1_ <br> ETR/ PA15 <br> /SPI1_NSS |
| B9 | B11 | - | B7 | 51 | 78 |  | PC10 | I/O | FT | PC10 | - | USART3_TX |
| B8 | C10 | - | B6 | 52 | 79 |  | PC11 | I/O | FT | PC11 | - | USART3_RX |
| C8 | B10 | - | C5 | 53 | 80 |  | PC12 | I/O | FT | PC12 | - | USART3_CK |
| D8 | C9 | - | C1 | - | 81 | 2 | PD0 | I/O | FT | PD0 | - | CANRX |
| E8 | B9 | - | D1 | - | 82 | 3 | PD1 | I/O | FT | PD1 | - | CANTX |
| B7 | C8 |  | B5 | 54 | 83 | - | PD2 | I/O | FT | PD2 | TIM3_ETR | - |
| C7 | B8 | - | - | - | 84 | - | PD3 | I/O | FT | PD3 | - | USART2_CTS |
| D7 | B7 | - | - | - | 85 | - | PD4 | I/O | FT | PD4 | - | USART2_RTS |
| B6 | A6 | - | - | - | 86 | - | PD5 | I/O | FT | PD5 | - | USART2_TX |
| C6 | B6 | - | - | - | 87 | - | PD6 | I/O | FT | PD6 | - | USART2_RX |
| D6 | A5 | - | - | - | 88 | - | PD7 | I/O | FT | PD7 | - | USART2_CK |
| A7 | A8 | 39 | A5 | 55 | 89 | 30 | PB3 | I/O | FT | JTDO | - | TIM2_CH2 / <br> PB3 <br> TRACESWO <br> SPI1_SCK |
| A6 | A7 | 40 | A4 | 56 | 90 | 31 | PB4 | I/O | FT | JNTRST | - | TIM3_CH1/ <br> PB4/ <br> SPI1_MISO |
| C5 | C5 | 41 | C4 | 57 | 91 | 32 | PB5 | I/O |  | PB5 | I2C1_SMBAI | TIM3_CH2 / <br> SPI1_MOSI |
| B5 | B5 | 42 | D3 | 58 | 92 | 33 | PB6 | I/O | FT | PB6 | $\begin{aligned} & \text { I2C1_SCL }^{(9)} / \\ & \text { TIM4_CH1 }{ }^{(9)} \end{aligned}$ | USART1_TX |
| A5 | B4 | 43 | C3 | 59 | 93 | 34 | PB7 | I/O | FT | PB7 | $\begin{aligned} & \text { I2C1_SDA }^{(9)} / \\ & \text { TIM4_CH2 }{ }^{(9)} \end{aligned}$ | USART1_RX |
| D5 | A4 | 44 | B4 | 60 | 94 | 35 | BOOT0 | I |  | BOOT0 | - | - |Table 5. Medium-density STM32F103xx pin definitions (continued)

| Pins |  |  |  |  |  |  | Pin name | $\begin{aligned} & \text { ㄷ } \\ & \text { ㄷ } \\ & \text { ㄷ } \\ & 0 \end{aligned}$ | Main function ${ }^{(3)}$ (after reset) | Alternate functions ${ }^{(4)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  |  |  |  |  |  |  | Default |  | Remap |
| B4 | A3 | 45 | B3 | 61 | 95 | - | PB8 | I/O | FT | PB8 | TIM4_CH3 ${ }^{(9)}$ | I2C1_SCL / <br> CANRX |
| A4 | B3 | 46 | A3 | 62 | 96 | - | PB9 | I/O | FT | PB9 | TIM4_CH4 ${ }^{(9)}$ | I2C1_SDA/ <br> CANTX |
| D4 | C3 | - | - | - | 97 | - | PE0 | I/O | FT | PE0 | TIM4_ETR | - |
| C4 | A2 | - | - | - | 98 | - | PE1 | I/O | FT | PE1 | - | - |
| E5 | D3 | 47 | D4 | 63 | 99 | 36 | $\mathrm{V}_{\mathrm{SS}, 3}$ | S | - | $\mathrm{V}_{\mathrm{SS}, 3}$ | - | - |
| F5 | C4 | 48 | E4 | 64 | 100 | 1 | $\mathrm{V}_{\mathrm{DD}, 3}$ | S | - | $\mathrm{V}_{\mathrm{DD}, 3}$ | - | - |

1. $\mathrm{I}=$ input, $\mathrm{O}=$ output, $\mathrm{S}=$ supply.
2. $\mathrm{FT}=5 \mathrm{~V}$ tolerant.
3. Function availability depends upon the chosen device. For devices having reduced peripheral counts, it is always the lower number of peripheral that is included. For example, if a device has only one SPI and two USARTs, they are called SPI1 and USART1 and USART2, respectively. Refer to Table 2.
4. If several peripherals share the same I/O pin, to avoid conflict between these alternate functions only one peripheral should be enabled at a time through the peripheral clock enable bit (in the corresponding RCC peripheral clock enable register).
5. PC13, PC14 and PC15 are supplied through the power switch. Since the switch only sinks a limited amount of current ( 3 mA ), the use of GPIOs PC13 to PC15 in output mode is limited: the speed should not exceed 2 MHz with a maximum load of 30 pF and these IOs must not be used as a current source (e.g. to drive a LED).
6. Main function after the first backup domain power-up. Later on, it depends on the contents of the Backup registers even after reset (because these registers are not reset by the main reset). For details on how to manage these IOs, refer to the Battery backup domain and BKP register description sections in the STM32F10xxx reference manual, available from the STMicroelectronics website: www.st.com.
7. The pins number 2 and 3 in the VFQFPN36 package, 5 and 6 in the LQFP48, UFQFP48 and LQFP64 packages, and C1 and C2 in the TFBGA64 package are configured as OSC_IN/OSC_OUT after reset, however the functionality of PD0 and PD1 can be remapped by software on these pins. For the LQFP100 package, PD0 and PD1 are available by default, so there is no need for remapping. For more details, refer to the Alternate function I/O and debug configuration section in the STM32F10xxx reference manual.
The use of PD0 and PD1 in output mode is limited as they can only be used at 50 MHz in output mode.
8. Unlike in the LQFP64 package, there is no PC3 in the TFBGA64 package. The $\mathrm{V}_{\text {REF }}$, functionality is provided instead.
9. This alternate function can be remapped by software to some other port pins (if available on the used package). For more details, refer to the Alternate function I/O and debug configuration section in the STM32F10xxx reference manual, available from the STMicroelectronics website: www.st.com.# 4 Memory mapping 

The memory map is shown in Figure 11.
Figure 11. Memory map
![img-13.jpeg](img-13.jpeg)# 5 Electrical characteristics 

### 5.1 Parameter conditions

Unless otherwise specified, all voltages are referenced to $\mathrm{V}_{\mathrm{SS}}$.

### 5.1.1 Minimum and maximum values

Unless otherwise specified the minimum and maximum values are guaranteed in the worst conditions of ambient temperature, supply voltage and frequencies by tests in production on $100 \%$ of the devices with an ambient temperature at $T_{A}=25^{\circ} \mathrm{C}$ and $T_{A}=T_{A} \max$ (given by the selected temperature range).

Data based on characterization results, design simulation and/or technology characteristics are indicated in the table footnotes and are not tested in production. Based on characterization, the minimum and maximum values refer to sample tests and represent the mean value plus or minus three times the standard deviation (mean $\pm 3 \sigma$ ).

### 5.1.2 Typical values

Unless otherwise specified, typical data are based on $T_{A}=25^{\circ} \mathrm{C}, V_{D D}=3.3 \mathrm{~V}$ (for the $2 \mathrm{~V} \leq \mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V}$ voltage range). They are given only as design guidelines and are not tested.

Typical ADC accuracy values are determined by characterization of a batch of samples from a standard diffusion lot over the full temperature range, where $95 \%$ of the devices have an error less than or equal to the value indicated (mean $\pm 2 \sigma$ ).

### 5.1.3 Typical curves

Unless otherwise specified, all typical curves are given only as design guidelines and are not tested.

### 5.1.4 Loading capacitor

The loading conditions used for pin parameter measurement are shown in Figure 12.

### 5.1.5 Pin input voltage

The input voltage measurement on a pin of the device is described in Figure 13.
![img-14.jpeg](img-14.jpeg)# 5.1.6 Power supply scheme 

Figure 14. Power supply scheme
![img-15.jpeg](img-15.jpeg)

Caution: In Figure 14, the $4.7 \mu \mathrm{~F}$ capacitor must be connected to $\mathrm{V}_{\mathrm{DD} 3}$.

### 5.1.7 Current consumption measurement

Figure 15. Current consumption measurement scheme
![img-16.jpeg](img-16.jpeg)# 5.2 Absolute maximum ratings 

Stresses above the absolute maximum ratings listed in Table 6, Table 7, and Table 8 may cause permanent damage to the device. These are stress ratings only and functional operation of the device at these conditions is not implied. Exposure to maximum rating conditions for extended periods may affect device reliability.

Table 6. Voltage characteristics

| Symbol | Ratings | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{DD}}-\mathrm{V}_{\mathrm{SS}}$ | External main supply voltage (including $\mathrm{V}_{\mathrm{DDA}}$ and $\mathrm{V}_{\mathrm{DD}}$ ) ${ }^{(1)}$ | $-0.3$ | 4.0 | V |
| $\mathrm{V}_{\mathrm{IN}}{ }^{(2)}$ | Input voltage on 5 V tolerant pin | $\mathrm{V}_{\mathrm{SS}}-0.3$ | $\mathrm{V}_{\mathrm{DD}}+4.0$ |  |
|  | Input voltage on any other pin | $\mathrm{V}_{\mathrm{SS}}-0.3$ | 4.0 |  |
| $\left\|\Delta \mathrm{V}_{\mathrm{DDx}}\right\|$ | Variations between different $\mathrm{V}_{\mathrm{DD}}$ power pins | - | 50 | mV |
| $\left\|\mathrm{V}_{\mathrm{SSX}}-\mathrm{V}_{\mathrm{SS}}\right\|$ | Variations between all the different ground pins | - | 50 |  |
| $\mathrm{V}_{\text {ESD(HBM) }}$ | Electrostatic discharge voltage (human body model) | See Section 5.3.11 |  |  |

1. All main power $\left(\mathrm{V}_{\mathrm{DD}}, \mathrm{V}_{\mathrm{DDA}}\right)$ and ground $\left(\mathrm{V}_{\mathrm{SS}}, \mathrm{V}_{\mathrm{SSA}}\right)$ pins must always be connected to the external power supply, in the permitted range.
2. $\mathrm{V}_{\text {IN }}$ maximum must always be respected. Refer to Table 7 for the maximum allowed injected current values.

Table 7. Current characteristics

| Symbol | Ratings | Max. | Unit |
| :--: | :-- | :--: | :--: |
| $\mathrm{I}_{\text {VDD }}$ | Total current into $\mathrm{V}_{\mathrm{DD}} / \mathrm{V}_{\mathrm{DDA}}$ power lines (source) ${ }^{(1)}$ | 150 | mA |
| $\mathrm{I}_{\text {VSS }}$ | Total current out of $\mathrm{V}_{\mathrm{SS}}$ ground lines (sink) ${ }^{(1)}$ | 150 |  |
| $\mathrm{I}_{\mathrm{IO}}$ | Output current sunk by any I/O and control pin | 25 |  |
|  | Output current source by any I/Os and control pin | $-25$ |  |
| $\mathrm{I}_{\mathrm{INJ}(\mathrm{PIN})}{ }^{(2)}$ | Injected current on five volt tolerant pins ${ }^{(3)}$ | $-5 /+0$ |  |
|  | Injected current on any other pin ${ }^{(4)}$ | $\pm 5$ |  |
| $\Sigma \mathrm{I}_{\mathrm{INJ}(\mathrm{PIN})}$ | Total injected current (sum of all I/O and control pins) ${ }^{(5)}$ | $\pm 25$ |  |

1. All main power $\left(\mathrm{V}_{\mathrm{DD}}, \mathrm{V}_{\mathrm{DDA}}\right)$ and ground $\left(\mathrm{V}_{\mathrm{SS}}, \mathrm{V}_{\mathrm{SSA}}\right)$ pins must always be connected to the external power supply, in the permitted range.
2. Negative injection disturbs the analog performance of the device. See footnote 2 of Table 49.
3. Positive injection is not possible on these I/Os. A negative injection is induced by $\mathrm{V}_{\mathrm{IN}}<\mathrm{V}_{\mathrm{SS}}$. $\mathrm{I}_{\mathrm{INJ}(\mathrm{PIN})}$ must never be exceeded. Refer to Table 6 for the maximum allowed input voltage values.
4. A positive injection is induced by $\mathrm{V}_{\mathrm{IN}}>\mathrm{V}_{\mathrm{DD}}$, while a negative injection is induced by $\mathrm{V}_{\mathrm{IN}}<\mathrm{V}_{\mathrm{SS}}$. $\mathrm{I}_{\mathrm{INJ}(\mathrm{PIN})}$ must never be exceeded. Refer to Table 6 for the maximum allowed input voltage values.
5. When several inputs are submitted to a current injection, the maximum $\Sigma \mathrm{I}_{\mathrm{INJ}(\mathrm{PIN})}$ is the absolute sum of the positive and negative injected currents (instantaneous values).

Table 8. Thermal characteristics

| Symbol | Ratings | Value | Unit |
| :--: | :-- | :--: | :--: |
| $\mathrm{T}_{\text {STG }}$ | Storage temperature range | -65 to +150 | ${ }^{\circ} \mathrm{C}$ |
| $\mathrm{T}_{\mathrm{J}}$ | Maximum junction temperature | 150 |  |# 5.3 Operating conditions 

### 5.3.1 General operating conditions

Table 9. General operating conditions

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {HCLK }}$ | Internal AHB clock frequency | - | 0 | 72 | MHz |
| $f_{\text {PCLK1 }}$ | Internal APB1 clock frequency | - | 0 | 36 |  |
| $f_{\text {PCLK2 }}$ | Internal APB2 clock frequency | - | 0 | 72 |  |
| $V_{D D}$ | Standard operating voltage | - | 2 | 3.6 | V |
| $V_{D D A}{ }^{(1)}$ | Analog operating voltage (ADC not used) | Must be the same potential as $\mathrm{V}_{\mathrm{DD}}{ }^{(2)}$ | 2 | 3.6 |  |
|  | Analog operating voltage (ADC used) |  | 2.4 | 3.6 |  |
| $V_{B A T}$ | Backup operating voltage | - | 1.8 | 3.6 |  |
| $V_{I N}$ | I/O input voltage | Standard IO | $-0.3$ | $\begin{aligned} & \mathrm{V}_{\mathrm{DD}}{ }^{*} \\ & 0.3 \end{aligned}$ | V |
|  |  | FT IO ${ }^{(3)}$ | $\begin{aligned} & 2 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}} \leq 3.6 \mathrm{~V} \\ & \mathrm{~V}_{\mathrm{DD}}=2 \mathrm{~V} \end{aligned}$ | $\begin{aligned} & -0.3 \\ & -0.3 \end{aligned}$ | $\begin{aligned} & 5.5 \\ & 5.2 \end{aligned}$ |  |
|  |  | BOOT0 | 0 | 5.5 |  |
| $P_{D}$ | Power dissipation at $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}$ for suffix 6 or $\mathrm{T}_{\mathrm{A}}=105^{\circ} \mathrm{C}$ for suffix $7^{(4)}$ | LFBGA100 | - | 454 | mW |
|  |  | LQFP100 | - | 434 |  |
|  |  | UFBGA100 | - | 339 |  |
|  |  | TFBGA64 | - | 308 |  |
|  |  | LQFP64 | - | 444 |  |
|  |  | LQFP48 | - | 363 |  |
|  |  | UFQFPN48 | - | 624 |  |
|  |  | VFQFPN36 | - | 1000 |  |
| TA | Ambient temperature for 6 suffix version | Maximum power dissipation | $-40$ | 85 | ${ }^{\circ} \mathrm{C}$ |
|  |  | Low-power dissipation ${ }^{(5)}$ | $-40$ | 105 |  |
|  | Ambient temperature for 7 suffix version | Maximum power dissipation | $-40$ | 105 |  |
|  |  | Low-power dissipation ${ }^{(5)}$ | $-40$ | 125 |  |
| TJ | Junction temperature range | 6 suffix version | $-40$ | 105 |  |
|  |  | 7 suffix version | $-40$ | 125 |  |

1. When the ADC is used, refer to Table 47.
2. It is recommended to power $\mathrm{V}_{\mathrm{DD}}$ and $\mathrm{V}_{\mathrm{DDA}}$ from the same source. A maximum difference of 300 mV between $V_{D D}$ and $V_{D D A}$ can be tolerated during power-up and operation.
3. To sustain a voltage higher than $V_{D D}+0.3 \mathrm{~V}$, the internal pull-up/pull-down resistors must be disabled.
4. If $T_{A}$ is lower, higher $P_{D}$ values are allowed as long as $T_{J}$ does not exceed $T_{J}$ max (see Section 6.10).
5. In low-power dissipation state, $T_{A}$ can be extended to this range as long as $T_{J}$ does not exceed $T_{J}$ max (see Section 6.10).# 5.3.2 Operating conditions at power-up / power-down 

Subject to general operating conditions for $\mathrm{T}_{\mathrm{A}}$.
Table 10. Operating conditions at power-up / power-down

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {VDD }}$ | $V_{D D}$ rise time rate | - | 0 | $\infty$ | $\mu \mathrm{s} / \mathrm{V}$ |
|  | $V_{D D}$ fall time rate |  | 20 | $\infty$ |  |

### 5.3.3 Embedded reset and power control block characteristics

The parameters given in Table 11 are derived from tests performed under ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.

Table 11. Embedded reset and power control block characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{\text {PVD }}$ | Programmable voltage detector level selection | PLS[2:0] = 000 (rising edge) | 2.10 | 2.18 | 2.26 | V |
|  |  | PLS[2:0] = 000 (falling edge) | 2,00 | 2.08 | 2.16 |  |
|  |  | PLS[2:0] = 001 (rising edge) | 2.19 | 2.28 | 2.37 |  |
|  |  | PLS[2:0] = 001 (falling edge) | 2.09 | 2.18 | 2.27 |  |
|  |  | PLS[2:0] = 010 (rising edge) | 2.28 | 2.38 | 2.48 |  |
|  |  | PLS[2:0] = 010 (falling edge) | 2.18 | 2.28 | 2.38 |  |
|  |  | PLS[2:0] = 011 (rising edge) | 2.38 | 2.48 | 2.58 |  |
|  |  | PLS[2:0] = 011 (falling edge) | 2.28 | 2.38 | 2.48 |  |
|  |  | PLS[2:0] = 100 (rising edge) | 2.47 | 2.58 | 2.69 |  |
|  |  | PLS[2:0] = 100 (falling edge) | 2.37 | 2.48 | 2.59 |  |
|  |  | PLS[2:0] = 101 (rising edge) | 2.57 | 2.68 | 2.79 |  |
|  |  | PLS[2:0] = 101 (falling edge) | 2.47 | 2.58 | 2.69 |  |
|  |  | PLS[2:0] = 110 (rising edge) | 2.66 | 2.78 | 2.90 |  |
|  |  | PLS[2:0] = 110 (falling edge) | 2.56 | 2.68 | 2.80 |  |
|  |  | PLS[2:0] = 111 (rising edge) | 2.76 | 2.88 | 3.00 |  |
|  |  | PLS[2:0] = 111 (falling edge) | 2.66 | 2.78 | 2.90 |  |
| $V_{\text {PVDhyst }}{ }^{(2)}$ | PVD hysteresis | - | - | 100 | - | mV |
| $V_{\text {POR/PDR }}$ | Power on/power down reset threshold | Falling edge | $1.8^{(1)}$ | 1.88 | 1.96 | V |
|  |  | Rising edge | 1.84 | 1.92 | 2.0 |  |
| $V_{\text {PDRhyst }}{ }^{(2)}$ | PDR hysteresis | - | - | 40 | - | mV |
| $T_{\text {RSTTEMPO }}{ }^{(2)}$ | Reset temporization | - | 1.0 | 2.5 | 4.5 | ms |

1. The product behavior is specified by design down to the minimum $\mathrm{V}_{\text {POR/POR }}$ value.
2. Specified by design, not tested in production.# 5.3.4 Embedded reference voltage 

The parameters given in Table 12 are derived from tests performed under ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.

Table 12. Embedded internal reference voltage

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {REFINT }}$ | Internal reference voltage | $-40^{\circ} \mathrm{C}<\mathrm{T}_{\mathrm{A}}<+105^{\circ} \mathrm{C}$ | 1.16 | 1.20 | 1.26 | V |
|  |  | $-40^{\circ} \mathrm{C}<\mathrm{T}_{\mathrm{A}}<+85^{\circ} \mathrm{C}$ | 1.16 | 1.20 | 1.24 |  |
| $\mathrm{T}_{\mathrm{S} \text { _vrefint }}{ }^{(1)}$ | ADC sampling time when reading the internal reference voltage | - | - | 5.1 | $17.1^{(2)}$ | $\mu \mathrm{s}$ |
| $\mathrm{V}_{\text {RERINT }}{ }^{(2)}$ | Internal reference voltage spread over the temperature range | $\mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V} \pm 10 \mathrm{mV}$ | - | - | 10 | mV |
| $\mathrm{T}_{\text {Coeff }}{ }^{(2)}$ | Temperature coefficient | - | - | - | 100 | $\mathrm{ppm} /{ }^{\circ} \mathrm{C}$ |

1. Shortest sampling time can be determined in the application by multiple iterations.
2. Specified by design, not tested in production.

### 5.3.5 Supply current characteristics

The current consumption is a function of several parameters and factors such as operating voltage, ambient temperature, I/O pin loading, device software configuration, operating frequencies, I/O pin switching rate, program location in memory, and executed binary code.
The current consumption is measured as described in Figure 15.
All Run-mode current consumption measurements given in this section are performed with a reduced code that gives a consumption equivalent to Dhrystone 2.1 code.

## Maximum current consumption

The MCU is placed under the following conditions:

- All I/O pins are in input mode with a static value at $\mathrm{V}_{\mathrm{DD}}$ or $\mathrm{V}_{\mathrm{SS}}$ (no load)
- All peripherals are disabled except when explicitly mentioned
- The flash memory access time is adjusted to the $\mathrm{f}_{\text {HCLK }}$ frequency ( 0 wait state from 0 to 24 MHz , one wait state from 24 to 48 MHz and two wait states above)
- Prefetch in ON (reminder: this bit must be set before clock setting and bus prescaling)
- When the peripherals are enabled $\mathrm{f}_{\text {PCLK1 }}=\mathrm{f}_{\text {HCLK }} / 2, \mathrm{f}_{\text {PCLK2 }}=\mathrm{f}_{\text {HCLK }}$

The parameters given in Table 13, Table 14 and Table 15 are derived from tests performed under ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.Table 13. Maximum current consumption in Run mode, code with data processing running from Flash

| Symbol | Parameter | Conditions | $f_{\text {HCLK }}$ | $\operatorname{Max}^{(1)}$ |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | $T_{A}=85^{\circ} \mathrm{C}$ | $T_{A}=105^{\circ} \mathrm{C}$ |  |
| $I_{D D}$ | Supply current in Run mode | External clock ${ }^{(2)}$, all peripherals enabled | 72 MHz | 50.0 | 50.3 | mA |
|  |  |  | 48 MHz | 36.1 | 36.2 |  |
|  |  |  | 36 MHz | 28.6 | 28.7 |  |
|  |  |  | 24 MHz | 19.9 | 20.1 |  |
|  |  |  | 16 MHz | 14.7 | 14.9 |  |
|  |  |  | 8 MHz | 8.6 | 8.9 |  |
|  |  | External clock ${ }^{(2)}$, all peripherals disabled | 72 MHz | 32.8 | 32.9 |  |
|  |  |  | 48 MHz | 24.4 | 24.5 |  |
|  |  |  | 36 MHz | 19.8 | 19.9 |  |
|  |  |  | 24 MHz | 13.9 | 14.2 |  |
|  |  |  | 16 MHz | 10.7 | 11.0 |  |
|  |  |  | 8 MHz | 6.8 | 7.1 |  |

1. Evaluated by characterization, not tested in production, unless otherwise specified.
2. External clock is 8 MHz and PLL is on when $f_{\text {HCLK }}>8 \mathrm{MHz}$.

Table 14. Maximum current consumption in Run mode, code with data processing running from RAM

| Symbol | Parameter | Conditions | $f_{\text {HCLK }}$ | $\operatorname{Max}^{(1)}$ |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | $T_{A}=85^{\circ} \mathrm{C}$ | $T_{A}=105^{\circ} \mathrm{C}$ |  |
| $I_{D D}$ | Supply current in Run mode | External clock ${ }^{(2)}$, all peripherals enabled | 72 MHz | 48 | 50 | mA |
|  |  |  | 48 MHz | 31.5 | 32 |  |
|  |  |  | 36 MHz | 24 | 25.5 |  |
|  |  |  | 24 MHz | 17.5 | 18 |  |
|  |  |  | 16 MHz | 12.5 | 13 |  |
|  |  |  | 8 MHz | 7.5 | 8 |  |
|  |  | External clock ${ }^{(2)}$, all peripherals disabled | 72 MHz | 29 | 29.5 |  |
|  |  |  | 48 MHz | 20.5 | 21 |  |
|  |  |  | 36 MHz | 16 | 16.5 |  |
|  |  |  | 24 MHz | 11.5 | 12 |  |
|  |  |  | 16 MHz | 8.5 | 9 |  |
|  |  |  | 8 MHz | 5.5 | 6 |  |

1. Based on characterization, tested in production at $\mathrm{V}_{\mathrm{DD}} \max , \mathrm{f}_{\mathrm{HCLK}}$ max.
2. External clock is 8 MHz and PLL is on when $f_{\text {HCLK }}>8 \mathrm{MHz}$.Figure 16. Typical current consumption in Run mode versus frequency (at 3.6 V ), code with data processing running from RAM, peripherals enabled
![img-17.jpeg](img-17.jpeg)

Figure 17. Typical current consumption in Run mode versus frequency (at 3.6 V ), code with data processing running from RAM, peripherals disabled
![img-18.jpeg](img-18.jpeg)Table 15. Maximum current consumption in Sleep mode, code running from Flash or RAM

| Symbol | Parameter | Conditions | $f_{\text {HCLK }}$ | $\operatorname{Max}^{(1)}$ |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | $T_{A}=85^{\circ} \mathrm{C}$ | $T_{A}=105^{\circ} \mathrm{C}$ |  |
| $I_{D D}$ | Supply current in Sleep mode | External clock ${ }^{(2)}$, all peripherals enabled | 72 MHz | 30 | 32 | mA |
|  |  |  | 48 MHz | 20 | 20.5 |  |
|  |  |  | 36 MHz | 15.5 | 16 |  |
|  |  |  | 24 MHz | 11.5 | 12 |  |
|  |  |  | 16 MHz | 8.5 | 9 |  |
|  |  |  | 8 MHz | 5.5 | 6 |  |
|  |  | External clock ${ }^{(2)}$, all peripherals disabled | 72 MHz | 7.5 | 8 |  |
|  |  |  | 48 MHz | 6 | 6.5 |  |
|  |  |  | 36 MHz | 5 | 5.5 |  |
|  |  |  | 24 MHz | 4.5 | 5 |  |
|  |  |  | 16 MHz | 4 | 4.5 |  |
|  |  |  | 8 MHz | 3 | 4 |  |

1. Based on characterization, tested in production at $\mathrm{V}_{\mathrm{DD} \text { max }}, \mathrm{f}_{\mathrm{HCLK}}$ max with peripherals enabled.
2. External clock is 8 MHz and PLL is on when $f_{\text {HCLK }}>8 \mathrm{MHz}$.Table 16. Typical and maximum current consumptions in Stop and Standby modes

| Symbol | Parameter | Conditions | Typ ${ }^{(1)}$ |  |  | Max |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | $\begin{aligned} & V_{D D} / V_{B A T} \\ & =2.0 \mathrm{~V} \end{aligned}$ | $\begin{aligned} & V_{D D} / V_{B A T} \\ & =2.4 \mathrm{~V} \end{aligned}$ | $\begin{aligned} & V_{D D} / V_{B A T} \\ & =3.3 \mathrm{~V} \end{aligned}$ | $\begin{aligned} & T_{A}= \\ & 85^{\circ} \mathrm{C} \end{aligned}$ | $\begin{aligned} & T_{A}= \\ & 105^{\circ} \mathrm{C} \end{aligned}$ |  |
| $\mathrm{I}_{\text {DD }}$ | Supply current in Stop mode | Regulator in Run mode, low-speed and high-speed internal RC oscillators and high-speed oscillator OFF (no independent watchdog) | - | 23.5 | 24 | 200 | 370 | $\mu \mathrm{A}$ |
|  |  | Regulator in Low-power mode, lowspeed and high-speed internal RC oscillators and high-speed oscillator OFF (no independent watchdog) | - | 13.5 | 14 | 180 | 340 |  |
|  | Supply current in Standby mode | Low-speed internal RC oscillator and independent watchdog ON | - | 2.6 | 3.4 | - | - |  |
|  |  | Low-speed internal RC oscillator ON, independent watchdog OFF | - | 2.4 | 3.2 | - | - |  |
|  |  | Low-speed internal RC oscillator and independent watchdog OFF, low-speed oscillator and RTC OFF | - | 1.7 | 2 | 4 | 5 |  |
| $\mathrm{I}_{\text {DD_VBAT }}$ | Backup domain supply current | Low-speed oscillator and RTC ON | 0.9 | 1.1 | 1.4 | $1.9^{(2)}$ | 2.2 |  |

1. Typical values are measured at $T_{A}=25^{\circ} \mathrm{C}$.
2. Evaluated by characterization, not tested in production, unless otherwise specified.

Figure 18. Typical current consumption on $\mathrm{V}_{\text {BAT }}$ (RTC on)
![img-19.jpeg](img-19.jpeg)Figure 19. Typical current consumption in Stop mode, with regulator in Run mode
![img-20.jpeg](img-20.jpeg)

Figure 20. Typical current consumption in Stop mode, with regulator in Low-power mode
![img-21.jpeg](img-21.jpeg)Figure 21. Typical current consumption in Standby mode
![img-22.jpeg](img-22.jpeg)

# Typical current consumption 

The MCU is placed under the following conditions:

- All I/O pins are in input mode with a static value at $\mathrm{V}_{\mathrm{DD}}$ or $\mathrm{V}_{\mathrm{SS}}$ (no load)
- All peripherals are disabled except if explicitly mentioned
- The flash access time is adjusted to $f_{\text {HCLK }}$ frequency ( 0 wait state from 0 to 24 MHz , one wait state from 24 to 48 MHz and two wait states above)
- Ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9
- Prefetch is ON (this bit must be set before clock setting and bus prescaling)
- When the peripherals are enabled $f_{P C L K 1}=f_{H C L K} / 4, f_{P C L K 2}=f_{H C L K} / 2$, $f_{\text {ADCCLK }}=f_{\text {PCLK2 }} / 4$Table 17. Typical current consumption in Run mode, code with data processing running from Flash

| Symbol | Parameter | Conditions | $f_{\text {HCLK }}$ | Typ ${ }^{(1)}$ |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | All peripherals enabled ${ }^{(2)}$ | All peripherals disabled |  |
| $t_{D D}$ | Supply current in Run mode | External clock ${ }^{(3)}$ | 72 MHz | 36 | 27 | mA |
|  |  |  | 48 MHz | 24.2 | 18.6 |  |
|  |  |  | 36 MHz | 19.0 | 14.8 |  |
|  |  |  | 24 MHz | 12.9 | 10.1 |  |
|  |  |  | 16 MHz | 9.3 | 7.4 |  |
|  |  |  | 8 MHz | 5.5 | 4.6 |  |
|  |  |  | 4 MHz | 3.3 | 2.8 |  |
|  |  |  | 2 MHz | 2.2 | 1.9 |  |
|  |  |  | 1 MHz | 1.6 | 1.45 |  |
|  |  |  | 500 kHz | 1.3 | 1.25 |  |
|  |  |  | 125 kHz | 1.08 | 1.06 |  |
|  |  | Running on high speed internal RC (HSI), AHB prescaler used to reduce the frequency | 64 MHz | 31.4 | 23.9 | mA |
|  |  |  | 48 MHz | 23.5 | 17.9 |  |
|  |  |  | 36 MHz | 18.3 | 14.1 |  |
|  |  |  | 24 MHz | 12.2 | 9.5 |  |
|  |  |  | 16 MHz | 8.5 | 6.8 |  |
|  |  |  | 8 MHz | 4.9 | 4.0 |  |
|  |  |  | 4 MHz | 2.7 | 2.2 |  |
|  |  |  | 2 MHz | 1.6 | 1.4 |  |
|  |  |  | 1 MHz | 1.02 | 0.9 |  |
|  |  |  | 500 kHz | 0.73 | 0.67 |  |
|  |  |  | 125 kHz | 0.5 | 0.48 |  |

1. Typical values are measures at $T_{A}=25^{\circ} \mathrm{C}, V_{D D}=3.3 \mathrm{~V}$.
2. Add an additional power consumption of 0.8 mA per ADC for the analog part. In applications, this consumption occurs only while the ADC is on (ADON bit is set in the ADC_CR2 register).
3. External clock is 8 MHz and PLL is on when $f_{\text {HCLK }}>8 \mathrm{MHz}$.Table 18. Typical current consumption in Sleep mode, code running from Flash or RAM

| Symbol | Parameter | Conditions | $f_{\text {HCLK }}$ | Typ ${ }^{(1)}$ |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  |  | All peripherals enabled ${ }^{(2)}$ | All peripherals disabled |  |
| $I_{D D}$ | Supply current in Sleep mode | External clock ${ }^{(3)}$ | 72 MHz | 14.4 | 5.5 | mA |
|  |  |  | 48 MHz | 9.9 | 3.9 |  |
|  |  |  | 36 MHz | 7.6 | 3.1 |  |
|  |  |  | 24 MHz | 5.3 | 2.3 |  |
|  |  |  | 16 MHz | 3.8 | 1.8 |  |
|  |  |  | 8 MHz | 2.1 | 1.2 |  |
|  |  |  | 4 MHz | 1.6 | 1.1 |  |
|  |  |  | 2 MHz | 1.3 | 1.0 |  |
|  |  |  | 1 MHz | 1.11 | 0.98 |  |
|  |  |  | 500 kHz | 1.04 | 0.96 |  |
|  |  |  | 125 kHz | 0.98 | 0.95 |  |
|  |  | Running on high speed internal RC (HSI), AHB prescaler used to reduce the frequency | 64 MHz | 12.3 | 4.4 |  |
|  |  |  | 48 MHz | 9.3 | 3.3 |  |
|  |  |  | 36 MHz | 7 | 2.5 |  |
|  |  |  | 24 MHz | 4.8 | 1.8 |  |
|  |  |  | 16 MHz | 3.2 | 1.2 |  |
|  |  |  | 8 MHz | 1.6 | 0.6 |  |
|  |  |  | 4 MHz | 1.0 | 0.5 |  |
|  |  |  | 2 MHz | 0.72 | 0.47 |  |
|  |  |  | 1 MHz | 0.56 | 0.44 |  |
|  |  |  | 500 kHz | 0.49 | 0.42 |  |
|  |  |  | 125 kHz | 0.43 | 0.41 |  |

1. Typical values are measures at $T_{A}=25^{\circ} \mathrm{C}, \mathrm{V}_{\mathrm{DD}}=3.3 \mathrm{~V}$.
2. Add an additional power consumption of 0.8 mA per ADC for the analog part. In applications, this consumption occurs only while the ADC is on (ADON bit is set in the ADC_CR2 register).
3. External clock is 8 MHz and PLL is on when $f_{\text {HCLK }}>8 \mathrm{MHz}$.# On-chip peripheral current consumption 

The current consumption of the on-chip peripherals is given in Table 19. The MCU is put under the following conditions:

- all I/O pins are in input mode with a static value at $\mathrm{V}_{\mathrm{DD}}$ or $\mathrm{V}_{\mathrm{SS}}$ (no load)
- all peripherals are disabled unless otherwise mentioned
- the given value is calculated by measuring the current consumption
- with all peripherals clocked off
- with only one peripheral clocked on
- ambient operating temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 6

Table 19. Peripheral current consumption

| Peripherals |  | $\mu \mathrm{A} / \mathrm{MHz}$ |
| :--: | :--: | :--: |
| AHB (up to 72 MHz ) | DMA1 | 16.53 |
|  | BusMatrix ${ }^{(1)}$ | 8.33 |
| APB1 (up to 36 MHz ) | APB1-Bridge | 10.28 |
|  | TIM2 | 32.50 |
|  | TIM3 | 31.39 |
|  | TIM4 | 31.94 |
|  | SPI2 | 4.17 |
|  | USART2 | 12.22 |
|  | USART3 | 12.22 |
|  | I2C1 | 10.00 |
|  | I2C2 | 10.00 |
|  | USB | 17.78 |
|  | CAN1 | 18.06 |
|  | WWDG | 2.50 |
|  | PWR | 1.67 |
|  | BKP | 2.50 |
|  | IWDG | 11.67 |Table 19. Peripheral current consumption (continued)

| Peripherals |  | $\mu \mathrm{A} / \mathrm{MHz}$ |
| :--: | :--: | :--: |
|  | APB2-Bridge | 3.75 |
|  | GPIOA | 6.67 |
|  | GPIOB | 6.53 |
|  | GPIOC | 6.53 |
|  | GPIOD | 6.53 |
|  | GPIOE | 6.39 |
|  | SPI1 | 4.72 |
|  | USART1 | 11.94 |
|  | TIM1 | 23.33 |
|  | $\mathrm{ADC} 1^{(2)}$ | 17.50 |
|  | $\mathrm{ADC} 2^{(2)}$ | 16.07 |

1. The BusMatrix is automatically active when at least one master peripheral is ON (CPU or DMA).
2. Specific conditions for measuring ADC current consumption: $f_{\text {HCLK }}=56 \mathrm{MHz}, f_{\text {APB1 }}=f_{\text {HCLK }} / 2$, $f_{\text {APB2 }}=f_{\text {HCLK }}, f_{\text {ADCCLK }}=f_{\text {APB2 }} / 4$. When ADON bit in the $\mathrm{ADCx}_{-} \mathrm{CR} 2$ register is set to 1 , a current consumption of analog part equal to 0.65 mA must be added for each ADC .

# 5.3.6 External clock source characteristics 

## High-speed external user clock generated from an external source

The characteristics given in Table 20 result from tests performed using a high-speed external clock source, and under ambient temperature and supply voltage conditions summarized in Table 9.

Table 20. High-speed external user clock characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {HSE ext }}$ | User external clock source frequency ${ }^{(1)}$ | - | 1 | 8 | 25 | MHz |
| $\mathrm{V}_{\text {HSEH }}$ | OSC_IN input pin high level voltage |  | $0.7 \mathrm{~V}_{\mathrm{DD}}$ | - | $\mathrm{V}_{\mathrm{DD}}$ | V |
| $\mathrm{V}_{\text {HSEL }}$ | OSC_IN input pin low level voltage |  | $\mathrm{V}_{\mathrm{SS}}$ | - | $0.3 \mathrm{~V}_{\mathrm{DD}}$ |  |
| $\begin{aligned} & \mathrm{t}_{\mathrm{w}(\mathrm{HSE})} \\ & \mathrm{t}_{\mathrm{w}(\mathrm{HSE})} \end{aligned}$ | OSC_IN high or low time ${ }^{(1)}$ |  | 5 | - | - | ns |
| $\begin{gathered} \mathrm{t}_{\mathrm{r}(\mathrm{HSE})} \\ \mathrm{t}_{\mathrm{f}(\mathrm{HSE})} \end{gathered}$ | OSC_IN rise or fall time ${ }^{(1)}$ |  | - | - | 20 |  |
| $\mathrm{C}_{\text {in }(\mathrm{HSE})}$ | OSC_IN input capacitance ${ }^{(1)}$ | - | - | 5 | - | pF |
| $\mathrm{DuCy}_{(\mathrm{HSE})}$ | Duty cycle | - | 45 | - | 55 | \% |
| $I_{L}$ | OSC_IN Input leakage current | $\mathrm{V}_{\mathrm{SS}} \leq \mathrm{V}_{\mathrm{IN}} \leq \mathrm{V}_{\mathrm{DD}}$ | - | - | $\pm 1$ | $\mu \mathrm{A}$ |

1. Specified by design, not tested in production.# Low-speed external user clock generated from an external source 

The characteristics given in Table 21 result from tests performed using a low-speed external clock source, and under ambient temperature and supply voltage conditions summarized in Table 9.

Table 21. Low-speed external user clock characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {LSE_ext }}$ | User external clock source frequency ${ }^{(1)}$ | - |  | 32.768 | 1000 | kHz |
| $\mathrm{V}_{\text {LSEH }}$ | OSC32_IN input pin high level voltage |  | $0.7 \mathrm{~V}_{\mathrm{DD}}$ | - | $\mathrm{V}_{\mathrm{DD}}$ | V |
| $\mathrm{V}_{\text {LSEL }}$ | OSC32_IN input pin low level voltage |  | $\mathrm{V}_{\mathrm{SS}}$ | - | $0.3 \mathrm{~V}_{\mathrm{DD}}$ |  |
| $\begin{aligned} & \mathrm{t}_{\mathrm{w}(\text { LSE) }} \\ & \mathrm{t}_{\mathrm{w}(\text { LSE) }} \end{aligned}$ | OSC32_IN high or low time ${ }^{(1)}$ |  | 450 | - | - | ns |
| $\begin{gathered} \mathrm{t}_{\mathrm{r}(\text { LSE) }} \\ \mathrm{t}_{\mathrm{r}(\text { LSE) }} \end{gathered}$ | OSC32_IN rise or fall time ${ }^{(1)}$ |  | - | - | 50 |  |
| $\mathrm{C}_{\text {in(LSE) }}$ | OSC32_IN input capacitance ${ }^{(1)}$ | - | - | 5 | - | pF |
| $\operatorname{DuCy}_{\text {(LSE) }}$ | Duty cycle | - | 30 | - | 70 | \% |
| $I_{L}$ | OSC32_IN Input leakage current | $\mathrm{V}_{\mathrm{SS}} \leq \mathrm{V}_{\mathrm{IN}} \leq \mathrm{V}_{\mathrm{DD}}$ | - | - | $\pm 1$ | $\mu \mathrm{A}$ |

1. Specified by design, not tested in production.

Figure 22. High-speed external clock source AC timing diagram
![img-23.jpeg](img-23.jpeg)Figure 23. Low-speed external clock source AC timing diagram
![img-24.jpeg](img-24.jpeg)

# High-speed external clock generated from a crystal/ceramic resonator 

The high-speed external (HSE) clock can be supplied with a 4 to 16 MHz crystal/ceramic resonator oscillator. All the information given in this paragraph is based on characterization results obtained with typical external components specified in Table 22. In the application, the resonator and the load capacitors have to be placed as close as possible to the oscillator pins in order to minimize output distortion and startup stabilization time. Refer to the crystal resonator manufacturer for more details on the resonator characteristics (frequency, package, accuracy).

Table 22. HSE 4-16 MHz oscillator characteristics ${ }^{(1)(2)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| $f_{\text {OSC_IN }}$ | Oscillator frequency | - | 4 | 8 | 16 | MHz |
| $R_{F}$ | Feedback resistor | - | - | 200 | - | k 3 |
| $\mathrm{I}_{2}$ | HSE driving current | $\begin{gathered} V_{D D}=3.3 \mathrm{~V}, V_{I N}=V_{S S} \\ \text { with } 30 \mathrm{pF} \text { load } \end{gathered}$ | - | - | 1 | mA |
| $g_{m}$ | Oscillator transconductance | Startup | 25 | - | - | mA/V |
| $t_{\text {SU(HSE }}{ }^{(3)}$ | Startup time | $V_{D D}$ is stabilized | - | 2 | - | ms |

1. Resonator characteristics given by the crystal/ceramic resonator manufacturer.
2. Evaluated by characterization, not tested in production, unless otherwise specified.
3. $t_{\text {SU(HSE) }}$ is the startu.p time measured from the moment it is enabled (by software) to a stabilized 8 MHz oscillation is reached. This value is measured for a standard crystal resonator and it can vary significantly with the crystal manufacturer

For $C_{L 1}$ and $C_{L 2}$, it is recommended to use high-quality external ceramic capacitors in the 5 pF to 25 pF range (typ.), designed for high-frequency applications, and selected to match the requirements of the crystal or resonator (see Figure 24). $\mathrm{C}_{\mathrm{L} 1}$ and $\mathrm{C}_{\mathrm{L} 2}$ are usually the same size. The crystal manufacturer typically specifies a load capacitance that is the series combination of $C_{L 1}$ and $C_{L 2}$. PCB and MCU pin capacitance must be included ( 10 pF can be used as a rough estimate of the combined pin and board capacitance) when sizing $\mathrm{C}_{\mathrm{L} 1}$ and$\mathrm{C}_{\mathrm{L} 2}$. Refer to AN2867 "Oscillator design guide for ST microcontrollers", available from the STMicroelectronics website www.st.com.

Figure 24. Typical application with an 8 MHz crystal
![img-25.jpeg](img-25.jpeg)

1. $\mathrm{R}_{\mathrm{EXT}}$ value depends on the crystal characteristics.

# Low-speed external clock generated from a crystal/ceramic resonator 

The low-speed external (LSE) clock can be supplied with a 32.768 kHz crystal/ceramic resonator oscillator. All the information given in this paragraph is based on characterization results obtained with typical external components specified in Table 23. In the application, the resonator and the load capacitors have to be placed as close as possible to the oscillator pins tortion and startup stabilization time. Refer to the crystal resonator manufacturer for more details on the resonator characteristics (frequency, package, accuracy).

Table 23. LSE oscillator characteristics $\left(f_{\text {LSE }}=32.768 \mathrm{kHz}\right)^{(1)}(2)$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $R_{F}$ | Feedback resistor | - | - | 5 | - | M $\Omega$ |
| $I_{2}$ | LSE driving current | $\mathrm{V}_{\mathrm{DD}}=3.3 \mathrm{~V}, \mathrm{~V}_{\mathrm{IN}}=\mathrm{V}_{\mathrm{SS}}$ | - | - | 1.4 | $\mu \mathrm{A}$ |
| $g_{m}$ | Oscillator transconductance | - | 5 | - | - | $\mu \mathrm{A} / \mathrm{V}$ |
| $\mathrm{t}_{\mathrm{SU}(\mathrm{LSE})^{(3)}}$ | Startup time | $\begin{gathered} \mathrm{V}_{\mathrm{DD}} \text { is } \\ \text { stabilized } \end{gathered}$ | $\mathrm{T}_{\mathrm{A}}=50^{\circ} \mathrm{C}$ | - | 1.5 | - |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=25^{\circ} \mathrm{C}$ | - | 2.5 | - |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=10^{\circ} \mathrm{C}$ | - | 4 | - |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=0^{\circ} \mathrm{C}$ | - | 6 | - |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=-10^{\circ} \mathrm{C}$ | - | 10 | - |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=-20^{\circ} \mathrm{C}$ | - | 17 | - |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=-30^{\circ} \mathrm{C}$ | - | 32 | - |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=-40^{\circ} \mathrm{C}$ | - | 60 | - |

1. Evaluated by characterization, not tested in production, unless otherwise specified.
2. Refer to the note and caution paragraphs below the table, and to AN2867 "Oscillator design guide for ST microcontrollers".
3. $\mathrm{t}_{\mathrm{SU}(\mathrm{LSE})}$ is the startup time measured from the moment it is enabled (by softwinformation given in this paragraph is) to a stabilized 32.768 kHz oscillation is reached. This value is measured for a standard crystal and it can vary significantly with the crystal manufacturer

Note: $\quad$ For $C_{L 1}$ and $C_{L 2}$ it is recommended to use high-quality ceramic capacitors in the 5 to 15 pF range selected to match the requirements of the crystal or resonator. $C_{L 1}$ and $C_{L 2}$ areusually the same size. The crystal manufacturer typically specifies a load capacitance, which is the series combination of $C_{L 1}$ and $C_{L 2}$.
Load capacitance $C_{L}$ has the following formula: $C_{L}=C_{L 1} \times C_{L 2} /\left(C_{L 1}+C_{L 2}\right)+C_{\text {stray }}$, where $C_{\text {stray }}$ is the pin capacitance and board or trace PCB-related capacitance. Typically, it is between 2 and 7 pF .

Caution: To avoid exceeding the maximum value of $C_{L 1}$ and $C_{L 2}(15 \mathrm{pF})$ it is strongly recommended to use a resonator with a load capacitance $C_{L} \leq 7 \mathrm{pF}$. Never use a resonator with a load capacitance of 12.5 pF .
Example: when choosing a resonator with a load capacitance of $C_{L}=6 \mathrm{pF}$ and $C_{\text {stray }}=2 \mathrm{pF}$, then $C_{L 1}=C_{L 2}=8 \mathrm{pF}$.

Figure 25. Typical application with a 32.768 kHz crystal
![img-26.jpeg](img-26.jpeg)

# 5.3.7 Internal clock source characteristics 

The parameters given in Table 24 are derived from tests performed under ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.

## High-speed internal (HSI) RC oscillator

Table 24. HSI oscillator characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {HSI }}$ | Frequency | - | - | 8 | - | MHz |
| $\operatorname{DuCy}_{(\text {(HSI) }}$ | Duty cycle | - | 45 | - | 55 | \% |
| $\mathrm{ACC}_{\text {HSI }}$ | Accuracy of the HSI oscillator | User-trimmed with the RCC_CR register ${ }^{(2)}$ | - | - | $1^{(3)}$ |  |
|  |  | Factorycalibrated $(\mathrm{F}(\mathrm{S})$ | $\mathrm{T}_{\mathrm{A}}=-40$ to $105^{\circ} \mathrm{C}$ | $-2$ | - | 2.5 |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=-10$ to $85^{\circ} \mathrm{C}$ | $-1.5$ | - | 2.2 |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=0$ to $70^{\circ} \mathrm{C}$ | $-1.3$ | - | 2 |
|  |  |  | $\mathrm{T}_{\mathrm{A}}=25^{\circ} \mathrm{C}$ | $-1.1$ | - | 1.8 |
| $\mathrm{t}_{\text {su(HSI) }}{ }^{(4)}$ | HSI oscillator startup time | - | 1 | - | 2 | $\mu \mathrm{s}$ |
| $\mathrm{I}_{\mathrm{DD}(\mathrm{HSI})}{ }^{(4)}$ | HSI oscillator power consumption | - | - | 80 | 100 | $\mu \mathrm{A}$ |

1. $\mathrm{V}_{\mathrm{DD}}=3.3 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=-40$ to $105^{\circ} \mathrm{C}$ unless otherwise specified.
2. Refer to AN2868 "STM32F10xxx internal RC oscillator (HSI) calibration" available from www.st.com.3. Specified by design, not tested in production.
4. Evaluated by characterization, not tested in production, unless otherwise specified.
5. The actual frequency of HSI oscillator may be impacted by a reflow, but does not drift out of the specified range.

# Low-speed internal (LSI) RC oscillator 

Table 25. LSI oscillator characteristics ${ }^{(1)}$

| Symbol | Parameter | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: |
| $\mathrm{f}_{\mathrm{LSI}}{ }^{(2)}$ | Frequency | 30 | 40 | 60 | kHz |
| $\mathrm{t}_{\mathrm{su}(\mathrm{LSI})^{(3)}}$ | LSI oscillator startup time | - | - | 85 | $\mu \mathrm{s}$ |
| $\mathrm{I}_{\mathrm{DD}(\mathrm{LSI})^{(3)}}$ | LSI oscillator power consumption | - | 0.65 | 1.2 | $\mu \mathrm{A}$ |

1. $\mathrm{V}_{\mathrm{DD}}=3 \mathrm{~V}, \mathrm{~T}_{\mathrm{A}}=-40$ to $105^{\circ} \mathrm{C}$ unless otherwise specified.
2. Evaluated by characterization, not tested in production, unless otherwise specified.
3. Specified by design, not tested in production.

## Wakeup time from low-power mode

The wakeup times given in Table 26 are measured on a wakeup phase with an $8-\mathrm{MHz}$ HSI RC oscillator. The clock source used to wake up the device depends from the current operating mode:

- Stop or Standby mode: the clock source is the RC oscillator
- Sleep mode: the clock source is the clock that was set before entering Sleep mode.

All timings are derived from tests performed under ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.Table 26. Low-power mode wakeup timings

| Symbol | Parameter | Typ | Unit |
| :--: | :--: | :--: | :--: |
| $\mathrm{t}_{\text {WUSLEEP }}{ }^{(1)}$ | Wakeup from Sleep mode | 1.8 | $\mu \mathrm{s}$ |
| $\mathrm{t}_{\text {WUSTOP }}{ }^{(1)}$ | Wakeup from Stop mode (regulator in run mode) | 3.6 |  |
|  | Wakeup from Stop mode (regulator in low-power mode) | 5.4 |  |
| $\mathrm{t}_{\text {WUSTDBY }}{ }^{(1)}$ | Wakeup from Standby mode | 50 |  |

1. The wakeup times are measured from the wakeup event to the point in which the user application code reads the first instruction.

# 5.3.8 PLL characteristics 

The parameters given in Table 27 are derived from tests performed under ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.

Table 27. PLL characteristics

| Symbol | Parameter | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | Min $^{(1)}$ | Typ | Max $^{(1)}$ |  |
| $t_{\text {PLL_IN }}$ | PLL input clock ${ }^{(2)}$ | 1 | 8.0 | 25 | MHz |
|  | PLL input clock duty cycle | 40 | - | 60 | $\%$ |
| $t_{\text {PLL_OUT }}$ | PLL multiplier output clock | 16 | - | 72 | MHz |
| $t_{\text {LOCK }}$ | PLL lock time | - | - | 200 | $\mu \mathrm{s}$ |
| Jitter | Cycle-to-cycle jitter | - | - | 300 | ps |

1. Evaluated by characterization, not tested in production, unless otherwise specified.
2. Take care of using the appropriate multiplier factors so as to have PLL input clock values compatible with the range defined by $\mathrm{f}_{\text {PLL_OUT }}$.

### 5.3.9 Memory characteristics

## Flash memory

The characteristics are given at $T_{A}=-40$ to $105^{\circ} \mathrm{C}$ unless otherwise specified.
Table 28. Flash memory characteristics

| Symbol | Parameter | Conditions | Min $^{(1)}$ | Typ | Max $^{(1)}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {prog }}$ | 16-bit programming time | $T_{A}=-40$ to $+105^{\circ} \mathrm{C}$ | 40 | 52.5 | 70 | $\mu \mathrm{s}$ |
| $t_{\text {ERASE }}$ | Page $(1 \mathrm{~KB})$ erase time | $T_{A}=-40$ to $+105^{\circ} \mathrm{C}$ | 20 | - | 40 | ms |
| $t_{M E}$ | Mass erase time | $T_{A}=-40$ to $+105^{\circ} \mathrm{C}$ | 20 | - | 40 |  |Table 28. Flash memory characteristics (continued)

| Symbol | Parameter | Conditions | Min $^{(1)}$ | Typ | Max $^{(1)}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{I}_{\mathrm{DD}}$ | Supply current | Read mode $\mathrm{f}_{\mathrm{HCLK}}=72 \mathrm{MHz}$ with two wait states, $\mathrm{V}_{\mathrm{DD}}=3.3 \mathrm{~V}$ | - | - | 20 | mA |
|  |  | Write / Erase modes $\mathrm{f}_{\mathrm{HCLK}}=72 \mathrm{MHz}, \mathrm{V}_{\mathrm{DD}}=3.3 \mathrm{~V}$ | - | - | 5 |  |
|  |  | Power-down mode / Halt, $\mathrm{V}_{\mathrm{DD}}=3.0$ to 3.6 V | - | - | 50 | $\mu \mathrm{~A}$ |
| $\mathrm{V}_{\text {prog }}$ | Programming voltage | - | 2 | - | 3.6 | V |

1. Specified by design, not tested in production.

Table 29. Flash memory endurance and data retention

| Symbol | Parameter | Conditions | Value |  |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  |  | Min $^{(1)}$ | Typ | Max |  |
| $\mathrm{N}_{\text {END }}$ | Endurance | $\begin{aligned} & \mathrm{T}_{\mathrm{A}}=-40 \text { to }+85^{\circ} \mathrm{C} \text { (6 suffix versions) } \\ & \mathrm{T}_{\mathrm{A}}=-40 \text { to }+105^{\circ} \mathrm{C} \text { (7 suffix versions) } \end{aligned}$ | 10 | - | - | kcycles |
| $\mathrm{t}_{\text {RET }}$ | Data retention | 1 kcycle $^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}$ | 30 | - | - | Years |
|  |  | 1 kcycle $^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=105^{\circ} \mathrm{C}$ | 10 | - | - |  |
|  |  | 10 kcycles $^{(2)}$ at $\mathrm{T}_{\mathrm{A}}=55^{\circ} \mathrm{C}$ | 20 | - | - |  |

1. Evaluated by characterization, not tested in production, unless otherwise specified.
2. Cycling performed over the whole temperature range.

# 5.3.10 EMC characteristics 

Susceptibility tests are performed on a sample basis during device characterization.

## Functional EMS (electromagnetic susceptibility)

While a simple application is executed on the device (toggling 2 LEDs through I/O ports). the device is stressed by two electromagnetic events until a failure occurs. The failure is indicated by the LEDs:

- Electrostatic discharge (ESD) (positive and negative) is applied to all device pins until a functional disturbance occurs. This test is compliant with the IEC 61000-4-2 standard.
- FTB: A Burst of Fast Transient voltage (positive and negative) is applied to $\mathrm{V}_{\mathrm{DD}}$ and $\mathrm{V}_{\mathrm{SS}}$ through a 100 pF capacitor, until a functional disturbance occurs. This test is compliant with the IEC 61000-4-4 standard.

A device reset allows normal operations to be resumed.
The test results are given in Table 30. They are based on the EMS levels and classes defined in application note AN1709, available on www.st.com.Table 30. EMS characteristics

| Symbol | Parameter | Conditions | Level/Class |
| :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {FESD }}$ | Voltage limits to be applied on any I/O pin to induce a functional disturbance | $\begin{aligned} & V_{D D}=3.3 \mathrm{~V}, T_{A}=+25^{\circ} \mathrm{C}, \\ & f_{\text {HCLK }}=72 \mathrm{MHz} \\ & \text { conforms to IEC 61000-4-2 } \end{aligned}$ | 2B |
| $\mathrm{V}_{\text {EFTB }}$ | Fast transient voltage burst limits to be applied through 100 pF on $\mathrm{V}_{\mathrm{DD}}$ and $\mathrm{V}_{\mathrm{SS}}$ pins to induce a functional disturbance | $\begin{aligned} & V_{D D}=3.3 \mathrm{~V}, T_{A}=+25^{\circ} \mathrm{C}, \\ & f_{\text {HCLK }}=72 \mathrm{MHz} \\ & \text { conforms to IEC 61000-4-4 } \end{aligned}$ | 4A |

# Designing hardened software to avoid noise problems 

EMC characterization and optimization are performed at component level with a typical application environment and simplified MCU software. It should be noted that good EMC performance is highly dependent on the user application and the software in particular.

Therefore it is recommended that the user applies EMC software optimization and prequalification tests in relation with the EMC level requested for his application.

## Software recommendations

The software flow must include the management of runaway conditions such as:

- Corrupted program counter
- Unexpected reset
- Critical data corruption (control registers...)


## Prequalification trials

Most of the common failures (unexpected reset and program counter corruption) can be reproduced by manually forcing a low state on the NRST pin or the Oscillator pins for 1 second.

To complete these trials, ESD stress can be applied directly on the device, over the range of specification values. When unexpected behavior is detected, the software can be hardened to prevent unrecoverable errors occurring (see application note AN1015, available on www.st.com).

## Electromagnetic Interference (EMI)

The electromagnetic field emitted by the device are monitored while a simple application is executed (toggling 2 LEDs through the I/O ports). This emission test is compliant with IEC 61967-2 standard which specifies the test board and the pin loading.

Table 31. EMI characteristics for $f_{\text {HSE }}=8 \mathrm{MHz}$ and $f_{\text {HCLK }}=48 \mathrm{MHz}$

| Symbol | Parameter | Conditions | Monitored frequency band | Value | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $S_{\text {EMI }}$ | Peak ${ }^{(1)}$ | $\begin{gathered} V_{D D}=3.3 \mathrm{~V}, T_{A}=25^{\circ} \mathrm{C} \\ \text { LQFP100 package } \\ \text { compliant with } \\ \text { IEC 61967-2 } \end{gathered}$ | 0.1 to 30 MHz | 12 | dB $\mu \mathrm{V}$ |
|  |  |  | 30 to 130 MHz | 22 |  |
|  |  |  | 130 MHz to 1 GHz | 23 |  |
|  | Level ${ }^{(2)}$ |  | 0.1 MHz to 1 GHz | 4 | - |

1. Refer to AN1709 "EMI radiated test" chapter.
2. Refer to AN1709 "EMI level classification" chapter.Table 32. EMI characteristics for $f_{\text {HSE }}=8 \mathrm{MHz}$ and $f_{\text {HCLK }}=72 \mathrm{MHz}$

| Symbol | Parameter | Conditions | Monitored <br> frequency band | Value | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | $V_{D D}=3.3 \mathrm{~V}, T_{A}=25^{\circ} \mathrm{C}$, <br> LQFP100 package <br> compliant with <br> IEC 61967-2 | 0.1 to 30 MHz | 12 |  |
|  | Peak $\left.{ }^{(1)}\right)$ | 30 to 130 MHz | 19 | $\mathrm{dB} \mu \mathrm{V}$ |  |
|  | Level $\left.{ }^{(2)}\right)$ | 130 MHz to 1 GHz | 29 |  |  |
|  |  | 0.1 MHz to 1 GHz | 4 | - |  |

1. Refer to AN1709 "EMI radiated test" chapter.
2. Refer to AN1709 "EMI level classification" chapter.

# 5.3.11 Absolute maximum ratings (electrical sensitivity) 

Based on three different tests (ESD, LU) using specific measurement methods, the device is stressed in order to determine its performance in terms of electrical sensitivity.

## Electrostatic discharge (ESD)

Electrostatic discharges (a positive then a negative pulse separated by 1 second) are applied to the pins of each sample according to each pin combination. The sample size depends on the number of supply pins in the device ( 3 parts $\times(n+1)$ supply pins). This test conforms to the JESD22-A114/C101 standard.

Table 33. ESD absolute maximum ratings

| Symbol | Ratings | Conditions | Class | Maximum value ${ }^{(1)}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{\text {ESD(HBM) }}$ | Electrostatic discharge voltage (human body model) | $\mathrm{T}_{\mathrm{A}}=+25^{\circ} \mathrm{C}$ conforming to JESD22-A114 | 2 | 2000 | V |
| $V_{\text {ESD(CDM) }}$ | Electrostatic discharge voltage (charge device model) | $\mathrm{T}_{\mathrm{A}}=+25^{\circ} \mathrm{C}$ conforming to ANSI/ESD STM5.3.1 | II | 500 |  |

1. Guaranteed based on test during characterization

## Static latch-up

Two complementary static tests are required on six parts to assess the latch-up performance:

- A supply overvoltage is applied to each power supply pin
- A current injection is applied to each input, output and configurable I/O pin

These tests are compliant with EIA/JESD 78A IC latch-up standard.
Table 34. Electrical sensitivities

| Symbol | Parameter | Conditions | Class |
| :--: | :--: | :--: | :--: |
| LU | Static latch-up class | $\mathrm{T}_{\mathrm{A}}=+105^{\circ} \mathrm{C}$ conforming to JESD78A | II level A |# 5.3.12 I/O current injection characteristics 

As a general rule, current injection to the I/O pins, due to external voltage below $\mathrm{V}_{\mathrm{SS}}$ or above $\mathrm{V}_{\mathrm{DD}}$ (for standard, 3 V -capable I/O pins) should be avoided during normal product operation. However, in order to give an indication of the robustness of the microcontroller in cases when abnormal injection accidentally happens, susceptibility tests are performed on a sample basis during device characterization.

## Functional susceptibilty to I/O current injection

While a simple application is executed on the device, the device is stressed by injecting current into the I/O pins programmed in floating input mode. While current is injected into the I/O pin, one at a time, the device is checked for functional failures.

The failure is indicated by an out of range parameter: ADC error above a certain limit ( $>5$ LSB TUE), out of spec current injection on adjacent pins or other functional failure (for example reset, oscillator frequency deviation).
The test results are given in Table 35
Table 35. I/O current injection susceptibility

| Symbol | Description | Functional susceptibility |  | Unit |
| :--: | :--: | :--: | :--: | :--: |
|  |  | Negative injection | Positive injection |  |
| $t_{\text {INJ }}$ | Injected current on OSC_IN32, OSC_OUT32, PA4, PA5, PC13 | $-0$ | $+0$ | mA |
|  | Injected current on all FT pins | $-5$ | $+0$ |  |
|  | Injected current on any other pin | $-5$ | $+5$ |  |# 5.3.13 I/O port characteristics 

## General input/output characteristics

Unless otherwise specified, the parameters given in Table 36 are derived from tests performed under the conditions summarized in Table 9. All I/Os are CMOS and TTL compliant.

Table 36. I/O static characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $V_{I L}$ | Low level input voltage | Standard IO input low level voltage | - | - | $0.28^{*}\left(V_{D D}-2 \mathrm{~V}\right)+0.8 \mathrm{~V}^{(1)}$ | V |
|  |  | IO FT ${ }^{(3)}$ input low level voltage | - | - | $0.32^{*}\left(V_{D D}-2 V\right)+0.75 V^{(1)}$ |  |
|  |  | All I/Os except BOOT0 | - | - | $0.35 \mathrm{~V}_{\mathrm{DD}}{ }^{(2)}$ |  |
| $V_{I H}$ | High level input voltage | Standard IO input high level voltage | $0.41^{*}\left(V_{D D}-2 \mathrm{~V}\right)+1.3 \mathrm{~V}^{(1)}$ | - | - |  |
|  |  | IO FT ${ }^{(3)}$ input high level voltage | $0.42^{*}\left(V_{D D}-2 \mathrm{~V}\right)+1 \mathrm{~V}^{(1)}$ | - | - |  |
|  |  | All I/Os except BOOT0 | $0.65 \mathrm{~V}_{\mathrm{DD}}{ }^{(2)}$ | - | - |  |
| $V_{\text {hys }}$ | Standard IO <br> Schmitt trigger voltage hysteresis ${ }^{(4)}$ | - | 200 | - | - | mV |
|  | IO FT Schmitt trigger voltage hysteresis ${ }^{(4)}$ | - | $5 \% \mathrm{~V}_{\mathrm{DD}}{ }^{(5)}$ | - | - |  |
| $I_{\text {Ikg }}$ | Input leakage current ${ }^{(6)}$ | $\begin{gathered} \mathrm{V}_{\mathrm{SS}} \leq \mathrm{V}_{\mathrm{IN}} \leq \mathrm{V}_{\mathrm{DD}} \\ \text { Standard I/Os } \end{gathered}$ | - | - | $\pm 1$ | $\mu \mathrm{~A}$ |
|  |  | $\begin{gathered} \mathrm{V}_{\mathrm{IN}}=5 \mathrm{~V} \\ \text { I/O FT } \end{gathered}$ | - | - | 3 |  |
| $R_{P U}$ | Weak pull-up equivalent resistor ${ }^{(7)}$ | $V_{I N}=V_{S S}$ | 30 | 40 | 50 | kD |
| $R_{P D}$ | Weak pull-down equivalent resistor ${ }^{(7)}$ | $V_{I N}=V_{D D}$ | 30 | 40 | 50 |  |
| $\mathrm{C}_{\text {IO }}$ | I/O pin capacitance |  | - | 5 | - | pF |

1. Data based on design simulation.
2. Tested in production.
3. $\mathrm{FT}=5 \mathrm{~V}$ tolerant. To sustain a voltage higher than $\mathrm{V}_{\mathrm{DD}}+0.3 \mathrm{~V}$ the internal pull-up/pull-down resistors must be disabled.
4. Hysteresis voltage between Schmitt trigger switching levels. Evaluated by characterization, not tested in production, unless otherwise specified.
5. With a minimum of 100 mV .
6. Leakage can be higher than Max if negative current is injected on adjacent pins.7. Pull-up and pull-down resistors are designed with a true resistance in series with a switchable PMOS/NMOS. This PMOS/NMOS contribution to the series resistance is minimum ( $\sim 10 \%$ ).

All I/Os are CMOS and TTL compliant (no software configuration required). Their characteristics cover more than the strict CMOS-technology or TTL parameters. The coverage of these requirements is shown in Figure 26 and Figure 27 for standard I/Os, and in Figure 28 and Figure 29 for 5 V tolerant I/Os.

Figure 26. Standard I/O input characteristics - CMOS port
![img-27.jpeg](img-27.jpeg)

Figure 27. Standard I/O input characteristics - TTL port
![img-28.jpeg](img-28.jpeg)Figure 28. 5 V tolerant I/O input characteristics - CMOS port
![img-29.jpeg](img-29.jpeg)

Figure 29. 5 V tolerant I/O input characteristics - TTL port
![img-30.jpeg](img-30.jpeg)# Output driving current 

The GPIOs (general-purpose inputs/outputs) can sink or source up to $\pm 8 \mathrm{~mA}$, and sink or source up to $\pm 20 \mathrm{~mA}$ (with a relaxed $\mathrm{V}_{\mathrm{OL}} / \mathrm{V}_{\mathrm{OH}}$ ) except PC13, PC14 and PC15, which can sink or source up to $\pm 3 \mathrm{~mA}$. When using the GPIOs PC13 to PC15 in output mode, the speed should not exceed 2 MHz with a maximum load of 30 pF .

In the user application, the number of I/O pins which can drive current must be limited to respect the absolute maximum rating specified in Section 5.2:

- The sum of the currents sourced by all the I/Os on $\mathrm{V}_{\mathrm{DD}}$ plus the maximum Run consumption of the MCU sourced on $\mathrm{V}_{\mathrm{DD}}$, cannot exceed the absolute maximum rating $\mathrm{I}_{\mathrm{VDD}}$ (see Table 7).
- The sum of the currents sunk by all the I/Os on $\mathrm{V}_{\mathrm{SS}}$ plus the maximum Run consumption of the MCU sunk on $\mathrm{V}_{\mathrm{SS}}$ cannot exceed the absolute maximum rating $\mathrm{I}_{\mathrm{VSS}}$ (see Table 7).


## Output voltage levels

Unless otherwise specified, the parameters given in Table 37 are derived from tests performed under ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9. All I/Os are CMOS and TTL compliant.

Table 37. Output voltage characteristics

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{OL}}{ }^{(1)}$ | Output low level voltage for an I/O pin when 8 pins are sunk at same time | CMOS port ${ }^{(2)}$, $\mathrm{I}_{\mathrm{IO}}=+8 \mathrm{~mA}$ | - | 0.4 | V |
| $\mathrm{V}_{\mathrm{OH}}{ }^{(3)}$ | Output high level voltage for an I/O pin when 8 pins are sourced at same time | $2.7 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | $\mathrm{V}_{\mathrm{DD}}-0.4$ | - |  |
| $\mathrm{V}_{\mathrm{OL}}{ }^{(1)}$ | Output low level voltage for an I/O pin when 8 pins are sunk at same time | TTL port ${ }^{(2)}$ $\mathrm{I}_{\mathrm{IO}}=+8 \mathrm{~mA}$ | - | 0.4 |  |
| $\mathrm{V}_{\mathrm{OH}}{ }^{(3)}$ | Output high level voltage for an I/O pin when 8 pins are sourced at same time | $2.7 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | 2.4 | - |  |
| $\mathrm{V}_{\mathrm{OL}}{ }^{(1)(4)}$ | Output low level voltage for an I/O pin when 8 pins are sunk at same time | $\mathrm{I}_{\mathrm{IO}}=+20 \mathrm{~mA}$ | - | 1.3 |  |
| $\mathrm{V}_{\mathrm{OH}}{ }^{(3)(4)}$ | Output high level voltage for an I/O pin when 8 pins are sourced at same time | $2.7 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<3.6 \mathrm{~V}$ | $\mathrm{V}_{\mathrm{DD}}-1.3$ | - |  |
| $\mathrm{V}_{\mathrm{OL}}{ }^{(1)(4)}$ | Output low level voltage for an I/O pin when 8 pins are sunk at same time | $\mathrm{I}_{\mathrm{IO}}=+6 \mathrm{~mA}$ $2 \mathrm{~V}<\mathrm{V}_{\mathrm{DD}}<2.7 \mathrm{~V}$ | - | 0.4 |  |
| $\mathrm{V}_{\mathrm{OH}}{ }^{(3)(4)}$ | Output high level voltage for an I/O pin when 8 pins are sourced at same time |  | $\mathrm{V}_{\mathrm{DD}}-0.4$ | - |  |

1. The $\mathrm{I}_{\mathrm{IO}}$ current sunk by the device must always respect the absolute maximum rating specified in Table 7 and the sum of $\mathrm{I}_{\mathrm{IO}}$ (I/O ports and control pins) must not exceed $\mathrm{I}_{\mathrm{VSS}}$.
2. TTL and CMOS outputs are compatible with JEDEC standards JESD36 and JESD52.
3. The $\mathrm{I}_{\mathrm{IO}}$ current sourced by the device must always respect the absolute maximum rating specified in Table 7 and the sum of $\mathrm{I}_{\mathrm{IO}}$ (I/O ports and control pins) must not exceed $\mathrm{I}_{\mathrm{VDD}}$.
4. Evaluated by characterization, not tested in production, unless otherwise specified.# Input/output AC characteristics 

The definition and values of input/output AC characteristics are given in Figure 30 and Table 38, respectively.

Unless otherwise specified, the parameters given in Table 38 are derived from tests performed under the ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.

Table 38. I/O AC characteristics ${ }^{(1)}$

| MODEx[1:0] bit value ${ }^{(1)}$ | Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| 10 | $f_{\text {max(IO)out }}$ | Maximum frequency ${ }^{(2)}$ | $C_{L}=50 \mathrm{pF}, V_{D D}=2 \mathrm{~V}$ to 3.6 V | - | 2 | MHz |
|  | $t_{f(10) o u t}$ | Output high to low level fall time | $C_{L}=50 \mathrm{pF}, V_{D D}=2 \mathrm{~V}$ to 3.6 V | - | $125^{(3)}$ | ns |
|  | $t_{r(10) o u t}$ | Output low to high level rise time |  | - | $125^{(3)}$ |  |
| 01 | $f_{\text {max(IO)out }}$ | Maximum frequency ${ }^{(2)}$ | $C_{L}=50 \mathrm{pF}, V_{D D}=2 \mathrm{~V}$ to 3.6 V | - | 10 | MHz |
|  | $t_{f(10) o u t}$ | Output high to low level fall time | $C_{L}=50 \mathrm{pF}, V_{D D}=2 \mathrm{~V}$ to 3.6 V | - | $25^{(3)}$ | ns |
|  | $t_{r(10) o u t}$ | Output low to high level rise time |  | - | $25^{(3)}$ |  |
| 11 | $F_{\text {max(IO)out }}$ | Maximum frequency ${ }^{(2)}$ | $C_{L}=30 \mathrm{pF}, V_{D D}=2.7 \mathrm{~V}$ to 3.6 V | - | 50 | MHz |
|  |  |  | $C_{L}=50 \mathrm{pF}, V_{D D}=2.7 \mathrm{~V}$ to 3.6 V | - | 30 |  |
|  |  |  | $C_{L}=50 \mathrm{pF}, V_{D D}=2 \mathrm{~V}$ to 2.7 V | - | 20 |  |
|  | $t_{f(10) o u t}$ | Output high to low level fall time | $C_{L}=30 \mathrm{pF}, V_{D D}=2.7 \mathrm{~V}$ to 3.6 V | - | $5^{(3)}$ | ns |
|  |  |  | $C_{L}=50 \mathrm{pF}, V_{D D}=2.7 \mathrm{~V}$ to 3.6 V | - | $8^{(3)}$ |  |
|  |  |  | $C_{L}=50 \mathrm{pF}, V_{D D}=2 \mathrm{~V}$ to 2.7 V | - | $12^{(3)}$ |  |
|  | $t_{r(10) o u t}$ | Output low to high level rise time | $C_{L}=30 \mathrm{pF}, V_{D D}=2.7 \mathrm{~V}$ to 3.6 V | - | $5^{(3)}$ |  |
|  |  |  | $C_{L}=50 \mathrm{pF}, V_{D D}=2.7 \mathrm{~V}$ to 3.6 V | - | $8^{(3)}$ |  |
|  |  |  | $C_{L}=50 \mathrm{pF}, V_{D D}=2 \mathrm{~V}$ to 2.7 V | - | $12^{(3)}$ |  |
| - | $t_{\text {EXTipw }}$ | Pulse width of external signals detected by the EXTI controller | - | 10 | - | ns |

1. The I/O speed is configured using the MODEx[1:0] bits. Refer to the STM32F10xxx reference manual for a description of GPIO port configuration register.
2. The maximum frequency is defined in Figure 30.
3. Specified by design, not tested in production.Figure 30. I/O AC characteristics definition
![img-31.jpeg](img-31.jpeg)

# 5.3.14 NRST pin characteristics 

The NRST pin input driver uses CMOS technology. It is connected to a permanent pull-up resistor, $\mathrm{R}_{\mathrm{PU}}$ (see Table 36).
Unless otherwise specified, the parameters given in Table 39 are derived from tests performed under the ambient temperature and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.

Table 39. NRST pin characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\mathrm{IL}(\mathrm{NRST})}{ }^{(1)}$ | NRST Input low level voltage | - | $-0.5$ | - | 0.8 | V |
| $\mathrm{V}_{\mathrm{IH}(\mathrm{NRST})}{ }^{(1)}$ | NRST Input high level voltage | - | 2 | - | $\mathrm{V}_{\mathrm{DD}}+0.5$ |  |
| $\mathrm{V}_{\text {hys(NRST) }}$ | NRST Schmitt trigger voltage hysteresis | - | - | 200 | - | mV |
| $R_{P U}$ | Weak pull-up equivalent resistor ${ }^{(2)}$ | $\mathrm{V}_{\mathrm{IN}}=\mathrm{V}_{\mathrm{SS}}$ | 30 | 40 | 50 | $\mathrm{k} \Omega$ |
| $\mathrm{V}_{\mathrm{F}(\mathrm{NRST})}{ }^{(1)}$ | NRST Input filtered pulse | - | - | - | 100 | ns |
| $\mathrm{V}_{\mathrm{NF}(\mathrm{NRST})}{ }^{(1)}$ | NRST Input not filtered pulse | - | 300 | - | - | ns |

1. Specified by design, not tested in production.
2. The pull-up is designed with a true resistance in series with a switchable PMOS. This PMOS contribution to the series resistance must be minimum $(<10 \%)$.Figure 31. Recommended NRST pin protection
![img-32.jpeg](img-32.jpeg)
2. The reset network protects the device against parasitic resets.
3. The user must ensure that the level on the NRST pin can go below the $\mathrm{V}_{\mathrm{IL}(\mathrm{NRST})}$ max level specified in Table 39, otherwise the reset is not taken into account by the device.

# 5.3.15 TIM timer characteristics 

The parameters given in Table 40 are specified by design, not tested in production.
Refer to Section 5.3.12 for details on the input/output alternate function characteristics (output compare, input capture, external clock, PWM output).

Table 40. TIMx ${ }^{(1)}$ characteristics

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {res(TIM) }}$ | Timer resolution time | - | 1 | - | $t_{\text {TIMxCLK }}$ |
|  |  | $t_{\text {TIMxCLK }}=72 \mathrm{MHz}$ | 13.9 | - | ns |
| $f_{\text {EXT }}$ | Timer external clock frequency on CH1 to CH4 | - | 0 | $t_{\text {TIMxCLK } / 2}$ | MHz |
|  |  | $t_{\text {TIMxCLK }}=72 \mathrm{MHz}$ | 0 | 36 | MHz |
| $\operatorname{Res}_{\text {TIM }}$ | Timer resolution | - | - | 16 | bit |
| $t_{\text {COUNTER }}$ | 16-bit counter clock period when internal clock is selected | - | 1 | 65536 | $t_{\text {TIMxCLK }}$ |
|  |  | $t_{\text {TIMxCLK }}=72 \mathrm{MHz}$ | 0.0139 | 910 | $\mu \mathrm{s}$ |
| $t_{\text {MAX_COUNT }}$ | Maximum possible count | - | - | $65536 \times 65536$ | $t_{\text {TIMxCLK }}$ |
|  |  | $t_{\text {TIMxCLK }}=72 \mathrm{MHz}$ | - | 59.6 | s |

1. TIMx is used as a general term to refer to the TIM1, TIM2, TIM3 and TIM4 timers.# 5.3.16 Communications interfaces 

## $I^{2} \mathrm{C}$ interface characteristics

The STM32F103xx performance line $I^{2} \mathrm{C}$ interface meets the requirements of the standard $I^{2} \mathrm{C}$ communication protocol with the following restrictions: the I/O pins SDA and SCL are mapped to are not "true" open-drain. When configured as open-drain, the PMOS connected between the I/O pin and $\mathrm{V}_{\mathrm{DD}}$ is disabled, but is still present.
The $I^{2} \mathrm{C}$ characteristics are described in Table 41. Refer also to Section 5.3.12 for more details on the input/output alternate function characteristics (SDA and SCL),

Table 41. $I^{2} \mathrm{C}$ characteristics

| Symbol | Parameter | Standard mode $I^{2} C^{(1)(2)}$ |  | Fast mode $I^{2} C^{(1)(2)}$ |  | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | Min | Max | Min | Max |  |
| $t_{w(\text { SCL })}$ | SCL clock low time | 4.7 | - | 1.3 | - | $\mu \mathrm{s}$ |
| $t_{w(\text { SCLH })}$ | SCL clock high time | 4.0 | - | 0.6 |  |  |
| $t_{\text {su(SDA) }}$ | SDA setup time | 250 | - | 100 | - | ns |
| $t_{h(\text { SDA })}$ | SDA data hold time | - | $3450^{(3)}$ | - | $900^{(3)}$ |  |
| $\begin{gathered} t_{t(\text { SDA })} \\ t_{t(\text { SCL })} \end{gathered}$ | SDA and SCL rise time | - | 1000 | - | 300 |  |
| $\begin{gathered} t_{t(\text { SDA })} \\ t_{t(\text { SCL })} \end{gathered}$ | SDA and SCL fall time | - | 300 | - | 300 |  |
| $t_{h(\text { STA })}$ | Start condition hold time | 4.0 | - | 0.6 | - | $\mu \mathrm{s}$ |
| $t_{\text {su(STA) }}$ | Repeated Start condition setup time | 4.7 | - | 0.6 | - |  |
| $t_{\text {su(STO) }}$ | Stop condition setup time | 4.0 | - | 0.6 | - | $\mu \mathrm{s}$ |
| $t_{w(\text { STO:STA })}$ | Stop to Start condition time (bus free) | 4.7 | - | 1.3 | - | $\mu \mathrm{s}$ |
| $C_{b}$ | Capacitive load for each bus line | - | 400 | - | 400 | pF |
| $t_{S P}$ | Pulse width of spikes suppressed by the analog filter | 0 | $50^{(4)}$ | 0 | $50^{(4)}$ | ns |

1. Specified by design, not tested in production.
2. $t_{\text {SCLK1 }}$ must be at least 2 MHz to achieve standard mode $I^{2} \mathrm{C}$ frequencies. It must be at least 4 MHz to achieve fast mode $I^{2} \mathrm{C}$ frequencies. It must be a multiple of 10 MHz to reach the 400 kHz maximum I2C fast mode clock.
3. The maximum Data hold time must be met if the interface does not stretch the low period of SCL signal.
4. The minimum width of the spikes filtered by the analog filter is above $t_{S P}(\max )$.Figure 32. $I^{2} \mathrm{C}$ bus AC waveforms and measurement circuit
![img-33.jpeg](img-33.jpeg)

1. Measurement points are done at CMOS levels: $0.3 \mathrm{~V}_{\mathrm{DD}}$ and $0.7 \mathrm{~V}_{\mathrm{DD}}$.
2. $\mathrm{Rs}=$ Series protection resistors, $\mathrm{Rp}=$ Pull-up resistors, $\mathrm{V}_{\mathrm{DD} \_12 \mathrm{C}}=12 \mathrm{C}$ bus supply.

Table 42. SCL frequency $\left(f_{\text {PCLK1 }}=36 \mathrm{MHz}, V_{\mathrm{DD} \_12 \mathrm{C}}=3.3 \mathrm{~V}\right)^{(1)(2)}$

| $\mathbf{f}_{\text {SCL }}(\mathrm{kHz})$ | I2C_CCR value |
| :--: | :--: |
|  | $\mathbf{R}_{\mathbf{P}}=\mathbf{4 . 7} \mathbf{k} \boldsymbol{\Omega}$ |
| 400 | $0 \times 801 \mathrm{E}$ |
| 300 | $0 \times 8028$ |
| 200 | $0 \times 803 \mathrm{C}$ |
| 100 | $0 \times 00 B 4$ |
| 50 | $0 \times 0168$ |
| 20 | $0 \times 0384$ |

1. $R_{P}=$ External pull-up resistance, $f_{S C L}=I^{2} C$ speed,
2. For speeds around 200 kHz , the tolerance on the achieved speed is $\leq 5 \%$. For other speed ranges, the tolerance on the achieved speed is $\leq 2 \%$. These variations depend upon the accuracy of the external components used to design the application.# SPI interface characteristics 

Unless otherwise specified, the parameters given in Table 43 are derived from tests performed under the ambient temperature, $\mathrm{f}_{\mathrm{PCLKx}}$ frequency and $\mathrm{V}_{\mathrm{DD}}$ supply voltage conditions summarized in Table 9.

Refer to Section 5.3.12 for more details on the input/output alternate function characteristics (NSS, SCK, MOSI, MISO).

Table 43. SPI characteristics

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\begin{gathered} \mathrm{f}_{\text {SCK }} \\ 1 / \mathrm{t}_{\mathrm{c}(\text { SCK) }} \end{gathered}$ | SPI clock frequency | Master mode | - | 18 | MHz |
|  |  | Slave mode | - | 18 |  |
| $\begin{gathered} \mathrm{t}_{\mathrm{r}(\text { SCK) }} \\ \mathrm{t}_{\mathrm{f}(\text { SCK) }} \end{gathered}$ | SPI clock rise and fall time | Capacitive load: $\mathrm{C}=30 \mathrm{pF}$ | - | 8 | ns |
| DuCy(SCK) | SPI slave input clock duty cycle | Slave mode | 30 | 70 | $\%$ |
| $\mathrm{t}_{\text {su(NSS) }}{ }^{(1)}$ | NSS setup time | Slave mode | $4 \mathrm{t}_{\text {PCLK }}$ | - | ns |
| $\mathrm{t}_{\mathrm{h}(\text { NSS) }}{ }^{(1)}$ | NSS hold time | Slave mode | $2 \mathrm{t}_{\text {PCLK }}$ | - |  |
| $\begin{gathered} \mathrm{t}_{\mathrm{w}(\text { SCKH) }}{ }^{(1)} \\ \mathrm{t}_{\mathrm{w}(\text { SCKL) }} \end{gathered}$ | SCK high and low time | Master mode, $\mathrm{f}_{\mathrm{PCLK}}=36 \mathrm{MHz}$, presc $=4$ | 50 | 60 |  |
| $\begin{gathered} \mathrm{t}_{\mathrm{su}(\mathrm{MI})} \\ \mathrm{t}_{\mathrm{su}(\mathrm{SI})} \end{gathered}$ | Data input setup time | Master mode | 5 | - |  |
|  |  | Slave mode | 5 | - |  |
| $\mathrm{t}_{\mathrm{h}(\mathrm{MI})}{ }^{(1)}$ | Data input hold time | Master mode | 5 | - |  |
| $\mathrm{t}_{\mathrm{h}(\mathrm{SI})}{ }^{(1)}$ |  | Slave mode | 4 | - |  |
| $\mathrm{t}_{\mathrm{a}(\mathrm{SO})}{ }^{(1)(2)}$ | Data output access time | Slave mode, $\mathrm{f}_{\mathrm{PCLK}}=20 \mathrm{MHz}$ | 0 | $3 \mathrm{t}_{\text {PCLK }}$ |  |
| $\mathrm{t}_{\text {dis(SO) }}{ }^{(1)(3)}$ | Data output disable time | Slave mode | 2 | 10 |  |
| $\mathrm{t}_{\mathrm{v}(\mathrm{SO})}{ }^{(1)}$ | Data output valid time | Slave mode (after enable edge) | - | 25 |  |
| $\mathrm{t}_{\mathrm{v}(\mathrm{MO})}{ }^{(1)}$ | Data output valid time | Master mode (after enable edge) | - | 5 |  |
| $\mathrm{t}_{\mathrm{h}(\mathrm{SO})}{ }^{(1)}$ | Data output hold time | Slave mode (after enable edge) | 15 | - |  |
| $\mathrm{t}_{\mathrm{h}(\mathrm{MO})}{ }^{(1)}$ |  | Master mode (after enable edge) | 2 | - |  |

1. Evaluated by characterization, not tested in production, unless otherwise specified.
2. Min time is for the minimum time to drive the output and the max time is for the maximum time to validate the data.
3. Min time is for the minimum time to invalidate the output and the max time is for the maximum time to put the data in Hi-Z.Figure 33. SPI timing diagram - slave mode and CPHA $=0$
![img-34.jpeg](img-34.jpeg)

Figure 34. SPI timing diagram - slave mode and CPHA $=1$
![img-35.jpeg](img-35.jpeg)Figure 35. SPI timing diagram - master mode
![img-36.jpeg](img-36.jpeg)

# USB characteristics 

The USB interface is USB-IF certified (Full Speed).
Table 44. USB startup time

| Symbol | Parameter | Max | Unit |
| :-- | :-- | :--: | :--: |
| $\mathrm{t}_{\text {STARTUP }}{ }^{(1)}$ | USB transceiver startup time | 1 | $\mu \mathrm{s}$ |

1. Guaranteed by design.Table 45. USB DC electrical characteristics

| Symbol | Parameter | Conditions | Min. ${ }^{(1)}$ | Max. ${ }^{(1)}$ | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| Input levels |  |  |  |  |  |
| $V_{D D}$ | USB operating voltage ${ }^{(2)}$ |  | $3.0^{(3)}$ | 3.6 | V |
| $V_{D I}{ }^{(4)}$ | Differential input sensitivity | I(USBDP, USBDM) | 0.2 | - | V |
| $V_{C M}{ }^{(4)}$ | Differential common mode range | Includes $\mathrm{V}_{\mathrm{DI}}$ range | 0.8 | 2.5 |  |
| $V_{S E}{ }^{(4)}$ | Single ended receiver threshold |  | 1.3 | 2.0 |  |
| Output levels |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{OL}}$ | Static output level low | $R_{L}$ of $1.5 \mathrm{k} \Omega$ to $3.6 \mathrm{~V}^{(5)}$ | - | 0.3 | V |
| $V_{O H}$ | Static output level high | $R_{L}$ of $15 \mathrm{k} \Omega$ to $\mathrm{V}_{\mathrm{SS}}{ }^{(5)}$ | 2.8 | 3.6 |  |

1. All the voltages are measured from the local ground potential.
2. To be compliant with the USB 2.0 full-speed electrical specification, the USBDP ( $\mathrm{D}+$ ) pin must be pulled up with a $1.5 \mathrm{k} \Omega$ resistor to a 3.0 to 3.6 V voltage range.
3. The STM32F103xx USB functionality is ensured down to 2.7 V , but not the full USB electrical characteristics, which are degraded in the 2.7 to $3.0 \mathrm{~V} \mathrm{~V}_{\mathrm{DD}}$ voltage range.
4. Specified by design, not tested in production.
5. $R_{L}$ is the load connected on the USB drivers.

Figure 36. USB timings: definition of data signal rise and fall time
![img-37.jpeg](img-37.jpeg)

Table 46. USB: Full-speed electrical characteristics ${ }^{(1)}$

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| Driver characteristics |  |  |  |  |  |
| $t_{r}$ | Rise time ${ }^{(2)}$ | $C_{L}=50 \mathrm{pF}$ | 4 | 20 | ns |
| $t_{f}$ | Fall time ${ }^{(2)}$ | $C_{L}=50 \mathrm{pF}$ | 4 | 20 | ns |
| $t_{r m}$ | Rise/ fall time matching | $t_{r} / t_{f}$ | 90 | 110 | \% |
| $\mathrm{V}_{\text {CRS }}$ | Output signal crossover voltage | - | 1.3 | 2.0 | V |

1. Specified by design, not tested in production.
2. Measured from $10 \%$ to $90 \%$ of the data signal. For more detailed informations, refer to USB specification Section 7 (version 2.0).

# 5.3.17 CAN (controller area network) interface 

Refer to Section 5.3.12 for more details on the input/output alternate function characteristics (CAN_TX and CAN_RX).# 5.3.18 12-bit ADC characteristics 

Unless otherwise specified, the parameters given in Table 47 are derived from tests performed under the ambient temperature, $\mathrm{f}_{\text {PCLK2 }}$ frequency and $\mathrm{V}_{\text {DDA }}$ supply voltage conditions summarized in Table 9.

Note: It is recommended to perform a calibration after each power-up.
Table 47. ADC characteristics

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {DDA }}$ | Power supply | - | 2.4 | - | 3.6 | V |
| $\mathrm{V}_{\text {REF }}$ | Positive reference voltage | - | 2.4 | - | $\mathrm{V}_{\text {DDA }}$ | V |
| $\mathrm{I}_{\text {VREF }}$ | Current on the $\mathrm{V}_{\text {REF }}$ input pin | - | - | $160^{(1)}$ | $220^{(1)}$ | $\mu \mathrm{A}$ |
| $f_{\text {ADC }}$ | ADC clock frequency | - | 0.6 | - | 14 | MHz |
| $f_{S}{ }^{(2)}$ | Sampling rate | - | 0.05 | - | 1 | MHz |
| $f_{\text {TRIG }}{ }^{(2)}$ | External trigger frequency | $\mathrm{f}_{\text {ADC }}=14 \mathrm{MHz}$ | - | - | 823 | kHz |
|  |  |  | - | - | 17 | $1 / f_{\text {ADC }}$ |
| $\mathrm{V}_{\text {AIN }}{ }^{(3)}$ | Conversion voltage range |  | $0\left(\mathrm{~V}_{\text {SSA }}\right.$ or $\mathrm{V}_{\text {REF }}$. tied to ground) | - | $\mathrm{V}_{\text {REF }}$ | V |
| $R_{\text {AIN }}{ }^{(2)}$ | External input impedance | See Equation 1 and Table 48 for details | - | - | 50 | $\mathrm{k} \Omega$ |
| $R_{\text {ADC }}{ }^{(2)}$ | Sampling switch resistance | - | - | - | 1 | $\mathrm{k} \Omega$ |
| $\mathrm{C}_{\text {ADC }}{ }^{(2)}$ | Internal sample and hold capacitor | - | - | - | 8 | pF |
| $\mathrm{t}_{\mathrm{CAL}}{ }^{(2)}$ | Calibration time | $\mathrm{f}_{\text {ADC }}=14 \mathrm{MHz}$ | 5.9 |  |  | $\mu \mathrm{s}$ |
|  |  | - | 83 |  |  | $1 / f_{\text {ADC }}$ |
| $t_{\text {lat }}{ }^{(2)}$ | Injection trigger conversion latency | $\mathrm{f}_{\text {ADC }}=14 \mathrm{MHz}$ | - | - | 0.214 | $\mu \mathrm{s}$ |
|  |  | - | - | - | $3^{(4)}$ | $1 / f_{\text {ADC }}$ |
| $t_{\text {lab }}{ }^{(2)}$ | Regular trigger conversion latency | $\mathrm{f}_{\text {ADC }}=14 \mathrm{MHz}$ | - | - | 0.143 | $\mu \mathrm{s}$ |
|  |  | - | - | - | $2^{(4)}$ | $1 / f_{\text {ADC }}$ |
| $t_{S}{ }^{(2)}$ | Sampling time | $\mathrm{f}_{\text {ADC }}=14 \mathrm{MHz}$ | 0.107 | - | 17.1 | $\mu \mathrm{s}$ |
|  |  | - | 1.5 | - | 239.5 | $1 / f_{\text {ADC }}$ |
| $t_{\text {STAB }}{ }^{(2)}$ | Power-up time | - | 0 | 0 | 1 | $\mu \mathrm{s}$ |
| $\mathrm{t}_{\text {CONV }}{ }^{(2)}$ | Total conversion time (including sampling time) | $\mathrm{f}_{\text {ADC }}=14 \mathrm{MHz}$ | 1 | - | 18 | $\mu \mathrm{s}$ |
|  |  | - | 14 to $252\left(t_{S}\right.$ for sampling +12.5 for successive approximation) |  |  | $1 / f_{\text {ADC }}$ |

1. Evaluated by characterization, not tested in production, unless otherwise specified.
2. Specified by design, not tested in production.
3. In devices delivered in VFQFPN and LQFP packages, $\mathrm{V}_{\mathrm{REF}}$, is internally connected to $\mathrm{V}_{\mathrm{DDA}}$ and $\mathrm{V}_{\mathrm{REF}}$, is internally connected to $\mathrm{V}_{\mathrm{SSA}}$. Devices that come in the TFBGA64 package have a $\mathrm{V}_{\mathrm{REF}}$, pin but no $\mathrm{V}_{\mathrm{REF}}$, pin ( $\mathrm{V}_{\mathrm{REF}}$, is internally connected to $\mathrm{V}_{\mathrm{SSA}}$ ), see Table 5 and Figure 7.
4. For external triggers, a delay of $1 / f_{\text {PCLK2 }}$ must be added to the latency specified in Table 47.Equation 1: $\mathrm{R}_{\text {AIN }}$ max formula:

$$
R_{A I N}<\frac{T_{S}}{f_{A D C} \times C_{A D C} \times \ln \left(2^{N+2}\right)}-R_{A D C}
$$

The formula above (Equation 1) is used to determine the maximum external impedance allowed for an error below $1 / 4$ of LSB. Here $\mathrm{N}=12$ (from 12-bit resolution).

Table 48. $\mathrm{R}_{\text {AIN }}$ max for $\mathrm{f}_{\text {ADC }}=14 \mathrm{MHz}^{(1)}$

| $\mathbf{T}_{\mathbf{s}}$ (cycles) | $\mathbf{t}_{\mathbf{S}}(\boldsymbol{\mu s})$ | $\mathbf{R}_{\text {AIN }} \max (\mathrm{k} \Omega)$ |
| :-- | :-- | :-- |
| 1.5 | 0.11 | 0.4 |
| 7.5 | 0.54 | 5.9 |
| 13.5 | 0.96 | 11.4 |
| 28.5 | 2.04 | 25.2 |
| 41.5 | 2.96 | 37.2 |
| 55.5 | 3.96 | 50 |
| 71.5 | 5.11 | NA |
| 239.5 | 17.1 | NA |

1. Evaluated by characterization, not tested in production, unless otherwise specified.

Table 49. ADC accuracy - Limited test conditions ${ }^{(1)}$ (2)

| Symbol | Parameter | Test conditions | Typ | Max $^{(3)}$ | Unit |
| :--: | :-- | :-- | :--: | :--: | :--: |
| ET | Total unadjusted error | $f_{\text {PCLK2 }}=56 \mathrm{MHz}$, | $\pm 1.3$ | $\pm 2$ | LSB |
| EO | Offset error | $f_{\text {ADC }}=14 \mathrm{MHz}, R_{\text {AIN }}<10 \mathrm{k} \Omega$, | $\pm 1.0$ | $\pm 1.5$ |  |
| EG | Gain error | $V_{\text {DDA }}=3 \mathrm{~V}$ to 3.6 V, | $\pm 0.5$ | $\pm 1.5$ |  |
| ED | Differential linearity error | Measurements made after | $\pm 0.7$ | $\pm 1.0$ |  |
| EL | Integral linearity error | ADC calibration | $\pm 0.8$ | $\pm 1.5$ |  |

1. ADC DC accuracy values are measured after internal calibration.
2. Injecting a negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to analog pins that may potentially inject negative currents. Any positive injection current within the limits specified for $I_{I N J(P I N)}$ and $\Sigma I_{I N J(P I N)}$ in Section 5.3.12 does not affect the ADC accuracy.
3. Evaluated by characterization, not tested in production, unless otherwise specified.Table 50. ADC accuracy ${ }^{(1)}(2)(3)$

| Symbol | Parameter | Test conditions | Typ | Max $^{(4)}$ | Unit |
| :--: | :-- | :-- | :--: | :--: | :--: |
| ET | Total unadjusted error | $\begin{aligned} & \mathrm{f}_{\text {PCLK2 }}=56 \mathrm{MHz}, \\ & \mathrm{f}_{\mathrm{ADC}}=14 \mathrm{MHz}, \mathrm{R}_{\mathrm{AIN}}<10 \mathrm{k} \Omega, \end{aligned}$ | $\pm 2$ | $\pm 5$ | LSB |
| EO | Offset error |  | $\pm 1.5$ | $\pm 2.5$ |  |
| EG | Gain error | $\mathrm{V}_{\text {DDA }}=2.4 \mathrm{~V}$ to 3.6 V <br> Measurements made after <br> ADC calibration | $\pm 1.5$ | $\pm 3$ |  |
| ED | Differential linearity error |  | $\pm 1$ | $\pm 2$ |  |
| EL | Integral linearity error |  | $\pm 1.5$ | $\pm 3$ |  |

1. ADC DC accuracy values are measured after internal calibration.
2. Better performance can be achieved in restricted $\mathrm{V}_{\mathrm{DD}}$, frequency and temperature ranges.
3. Injecting a negative current on any analog input pins should be avoided as this significantly reduces the accuracy of the conversion being performed on another analog input. It is recommended to add a Schottky diode (pin to ground) to standard analog pins which may potentially inject negative current. Any positive injection current within the limits specified for $\mathrm{I}_{\left(\mathrm{NJ}(\mathrm{PIN})\right.}$ and $\Sigma_{\mathrm{I}(\mathrm{NJ}(\mathrm{PIN})}$ in Section 5.3.12 does not affect the ADC accuracy.
4. Evaluated by characterization, not tested in production, unless otherwise specified.

Figure 37. ADC accuracy characteristics
![img-38.jpeg](img-38.jpeg)Figure 38. Typical connection diagram using the ADC
![img-39.jpeg](img-39.jpeg)

1. Refer to Table 47 for the values of $R_{A N}, R_{A D C}$ and $C_{A D C}$.
2. $\mathrm{C}_{\text {parasitic }}$ represents the capacitance of the PCB (dependent on soldering and PCB layout quality) plus the pad capacitance (refer to Table 36 for the value of the pad capacitance). A high $\mathrm{C}_{\text {parasitic }}$ value will downgrade conversion accuracy. To remedy this, $f_{A D C}$ should be reduced.
3. Refer to Table 36 for the values of $\mathrm{I}_{\mathrm{ag}}$.
4. Refer to Figure 14.

# General PCB design guidelines 

Power supply decoupling must be performed as shown in Figure 39 or Figure 40, depending on whether $\mathrm{V}_{\mathrm{REF}+}$ is connected to $\mathrm{V}_{\mathrm{DDA}}$ or not. The 10 nF capacitors should be ceramic (good quality), and placed as close as possible to the chip.

Figure 39. Power supply and reference decoupling ( $\mathrm{V}_{\mathrm{REF}+}$ not connected to $\mathrm{V}_{\mathrm{DDA}}$ )
![img-40.jpeg](img-40.jpeg)

1. $\mathrm{V}_{\mathrm{REF}+}$ and $\mathrm{V}_{\mathrm{REF}-}$ inputs are available only on 100-pin packages.Figure 40. Power supply and reference decoupling ( $\mathrm{V}_{\mathrm{REF}+}$ connected to $\mathrm{V}_{\mathrm{ODA}}$ )
![img-41.jpeg](img-41.jpeg)

1. $\mathrm{V}_{\mathrm{REF}+}$ and $\mathrm{V}_{\mathrm{REF}-}$ inputs are available only on 100-pin packages.

# 5.3.19 Temperature sensor characteristics 

Table 51. TS characteristics

| Symbol | Parameter | Min | Typ | Max | Unit |
| :--: | :-- | :--: | :--: | :--: | :--: |
| $\mathrm{T}_{\mathrm{L}}{ }^{(1)}$ | $\mathrm{V}_{\text {SENSE }}$ linearity with temperature | - | $\pm 1$ | $\pm 2$ | ${ }^{\circ} \mathrm{C}$ |
| Avg_Slope ${ }^{(1)}$ | Average slope | 4.0 | 4.3 | 4.6 | $\mathrm{mV} /{ }^{\circ} \mathrm{C}$ |
| $\mathrm{V}_{25}{ }^{(1)}$ | Voltage at $25^{\circ} \mathrm{C}$ | 1.34 | 1.43 | 1.52 | V |
| $\mathrm{t}_{\text {START }}{ }^{(2)}$ | Startup time | 4 | - | 10 | $\mu \mathrm{s}$ |
| $\mathrm{T}_{\mathrm{S} \text { _temp }}{ }^{(3)(2)}$ | ADC sampling time when reading the temperature | - | - | 17.1 |  |

1. Evaluated by characterization, not tested in production, unless otherwise specified.
2. Specified by design, not tested in production.
3. Shortest sampling time can be determined in the application by multiple iterations.# 6 Package information 

In order to meet environmental requirements, ST offers these devices in different grades of ECOPACK ${ }^{\circledR}$ packages, depending on their level of environmental compliance. ECOPACK ${ }^{\circledR}$ specifications, grade definitions and product status are available at: www.st.com. ECOPACK ${ }^{\circledR}$ is an ST trademark.

### 6.1 Device marking

Refer to technical note "Reference device marking schematics for STM32 microcontrollers and microprocessors" (TN1433), available on www.st.com, for the location of pin 1/ ball A1, as well as the location and orientation of the marking areas versus pin 1/ball A1.

Parts marked as "ES", "E", or accompanied by an engineering sample notification letter, are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST's Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.# 6.2 VFQFPN36 package information (ZR) 

Figure 41. VFQFPN - 36 pin, $6 \times 6 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch very thin profile fine pitch quad flat package outline
![img-42.jpeg](img-42.jpeg)

1. Drawing is not to scale.Table 52. VFQFPN - 36 pin, $6 \times 6 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch very thin profile fine pitch quad flat package mechanical data

| Symbol | millimeters |  |  | inches ${ }^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | 0.800 | 0.900 | 1.000 | 0.0315 | 0.0354 | 0.0394 |
| A1 | - | 0.020 | 0.050 | - | 0.0008 | 0.0020 |
| A2 | - | 0.650 | 1.000 | - | 0.0256 | 0.0394 |
| A3 | - | 0.200 | - | - | 0.0079 | - |
| b | 0.180 | 0.230 | 0.300 | 0.0071 | 0.0091 | 0.0118 |
| D | 5.875 | 6.000 | 6.125 | 0.2313 | 0.2362 | 0.2411 |
| D2 | 1.750 | 3.700 | 4.250 | 0.0689 | 0.1457 | 0.1673 |
| E | 5.875 | 6.000 | 6.125 | 0.2313 | 0.2362 | 0.2411 |
| E2 | 1.750 | 3.700 | 4.250 | 0.0689 | 0.1457 | 0.1673 |
| e | 0.450 | 0.500 | 0.550 | 0.0177 | 0.0197 | 0.0217 |
| L | 0.350 | 0.550 | 0.750 | 0.0138 | 0.0217 | 0.0295 |
| K | 0.250 | - | - | 0.0098 | - | - |
| ddd | - | - | 0.080 | - | - | 0.0031 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.Figure 42. VFQFPN - 36 pin, $6 \times 6 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch very thin profile fine pitch quad flat package recommended footprint
![img-43.jpeg](img-43.jpeg)

1. Dimensions are expressed in millimeters.# 6.3 UFQFPN48 package information (A0B9) 

This UFQFPN is a 48-lead, $7 \times 7 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, ultra thin fine pitch quad flat package.
Figure 43. UFQFPN48 - Outline
![img-44.jpeg](img-44.jpeg)

1. Drawing is not to scale.
2. All leads/pads should also be soldered to the PCB to improve the lead/pad solder joint life.
3. There is an exposed die pad on the underside of the UFQFPN48 package. It is recommended to connect and solder this back-side pad to PCB ground.Table 53. UFQFPN48 - Mechanical data

| Symbol | millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | 0.500 | 0.550 | 0.600 | 0.0197 | 0.0217 | 0.0236 |
| A1 | 0.000 | 0.020 | 0.050 | 0.0000 | 0.0008 | 0.0020 |
| A3 | - | 0.152 | - | - | 0.0060 | - |
| b | 0.200 | 0.250 | 0.300 | 0.0079 | 0.0098 | 0.0118 |
| $D^{(2)}$ | 6.900 | 7.000 | 7.100 | 0.2717 | 0.2756 | 0.2795 |
| D1 | 5.400 | 5.500 | 5.600 | 0.2126 | 0.2165 | 0.2205 |
| D2 ${ }^{(3)}$ | 5.500 | 5.600 | 5.700 | 0.2165 | 0.2205 | 0.2244 |
| $E^{(2)}$ | 6.900 | 7.000 | 7.100 | 0.2717 | 0.2756 | 0.2795 |
| E1 | 5.400 | 5.500 | 5.600 | 0.2126 | 0.2165 | 0.2205 |
| E2 ${ }^{(3)}$ | 5.500 | 5.600 | 5.700 | 0.2165 | 0.2205 | 0.2244 |
| e | - | 0.500 | - | - | 0.0197 | - |
| L | 0.300 | 0.400 | 0.500 | 0.0118 | 0.0157 | 0.0197 |
| ddd | - | - | 0.080 | - | - | 0.0031 |

1. Values in inches are converted from mm and rounded to four decimal digits.
2. Dimensions D and E do not include mold protrusion, not exceed 0.15 mm .
3. Dimensions D2 and E2 are not in accordance with JEDEC.

Figure 44. UFQFPN48 - Footprint example
![img-45.jpeg](img-45.jpeg)

1. Dimensions are expressed in millimeters.# 6.4 LFBGA100 package information (H0) 

Figure 45. LFBGA100 - 100-ball low profile fine pitch ball grid array, $10 \times 10 \mathrm{~mm}$, 0.8 mm pitch, package outline
![img-46.jpeg](img-46.jpeg)

1. Drawing is not to scale.

Table 54. LFBGA100 - 100-ball low profile fine pitch ball grid array, $10 \times 10 \mathrm{~mm}$, 0.8 mm pitch, package mechanical data

| Symbol | millimeters |  |  | inches $\left.{ }^{(1)}\right.$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Typ | Min | Max |
| A | - | - | 1.700 |  |  | 0.0669 |
| A1 | 0.270 | - | - | 0.0106 |  |  |
| A2 | - | 0.300 | - |  | 0.0118 |  |
| A4 | - | - | 0.800 | - | - | 0.0315 |
| b | 0.450 | 0.500 | 0.550 | 0.0177 | 0.0197 | 0.0217 |
| D | 9.850 | 10.000 | 10.150 | 0.3878 | 0.3937 | 0.3996 |
| D1 | - | 7.200 | - | - | 0.2835 | - |
| E | 9.850 | 10.000 | 10.150 | 0.3878 | 0.3937 | 0.3996 |
| E1 | - | 7.200 | - | - | 0.2835 | - |
| e | - | 0.800 | - | - | 0.0315 | - |
| F | - | 1.400 | - | - | 0.0551 | - |
| ddd | - | - | 0.120 | - | - | 0.0047 |Table 54. LFBGA100 - 100-ball low profile fine pitch ball grid array, $10 \times 10 \mathrm{~mm}$, 0.8 mm pitch, package mechanical data (continued)

| Symbol | millimeters |  |  | inches $\left({ }^{1}\right)$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Typ | Min | Max |
| eee | - | - | 0.150 | - | - | 0.0059 |
| fff | - | - | 0.080 | - | - | 0.0031 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 46. LFBGA100 - 100-ball low profile fine pitch ball grid array, $10 \times 10 \mathrm{~mm}$, 0.8 mm pitch, package recommended footprint
![img-47.jpeg](img-47.jpeg)

Table 55. LFBGA100 recommended PCB design rules ( 0.8 mm pitch BGA)

| Dimension | Recommended values |
| :-- | :-- |
| Pitch | 0.8 |
| Dpad | 0.500 mm |
| Dsm | 0.570 mm typ. (depends on the soldermask <br> registration tolerance) |
| Stencil opening | 0.500 mm |
| Stencil thickness | Between 0.100 mm and 0.125 mm |
| Pad trace width | 0.120 mm |# 6.5 LQFP100 package information (1L) 

This LQFP is 100 lead, $14 \times 14 \mathrm{~mm}$ low-profile quad flat package.
Note: See list of notes in the notes section.
Figure 47. LQFP100 - Outline ${ }^{(15)}$
![img-48.jpeg](img-48.jpeg)Table 56. LQFP100 - Mechanical data

| Symbol | millimeters |  |  | inches ${ }^{(14)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | 1.50 | 1.60 | - | 0.0590 | 0.0630 |
| A1 ${ }^{(12)}$ | 0.05 | - | 0.15 | 0.0019 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0570 |
| $b^{(9)(11)}$ | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1 ${ }^{(11)}$ | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0090 |
| c ${ }^{(11)}$ | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1 ${ }^{(11)}$ | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| $D^{(4)}$ | 16.00 BSC |  |  | 0.6299 BSC |  |  |
| D1 ${ }^{(2)(5)}$ | 14.00 BSC |  |  | 0.5512 BSC |  |  |
| $E^{(4)}$ | 16.00 BSC |  |  | 0.6299 BSC |  |  |
| E1 ${ }^{(2)(5)}$ | 14.00 BSC |  |  | 0.5512 BSC |  |  |
| e | 0.50 BSC |  |  | 0.0197 BSC |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.177 | 0.0236 | 0.0295 |
| L1 ${ }^{(1)(11)}$ | 1.00 |  |  | - | 0.0394 | - |
| $\mathrm{N}^{(13)}$ | 100 |  |  |  |  |  |
| $\theta$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ |
| $\theta 1$ | $0^{\circ}$ | - | - | $0^{\circ}$ | - | - |
| $\theta 2$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ |
| $\theta 3$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ |
| R1 | 0.08 | - | - | 0.0031 | - | - |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | - | 0.0079 | - | - |
| aaa ${ }^{(1)}$ | 0.20 |  |  | 0.0079 |  |  |
| bbb $^{(1)}$ | 0.20 |  |  | 0.0079 |  |  |
| $\mathrm{ccc}^{(1)}$ | 0.08 |  |  | 0.0031 |  |  |
| $\mathrm{ddd}^{(1)}$ | 0.08 |  |  | 0.0031 |  |  |# Notes: 

1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The Top package body size may be smaller than the bottom package size by as much as 0.15 mm .
3. Datums A-B and D to be determined at datum plane H .
4. To be determined at seating datum plane $C$.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash or protrusions is " 0.25 mm " per side. D1 and E1 are Maximum plastic body size dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All Dimensions are in millimeters.
8. No intrusion allowed inwards the leads.
9. Dimension "b" does not include dambar protrusion. Allowable dambar protrusion shall not cause the lead width to exceed the maximum "b" dimension by more than 0.08 mm . Dambar cannot be located on the lower radius or the foot. Minimum space between protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. Exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead between 0.10 mm and 0.25 mm from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package body.
13. " N " is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to 4 decimal digits.
15. Drawing is not to scale.

Figure 48. LQFP100 - Footprint example
![img-49.jpeg](img-49.jpeg)

1. Dimensions are expressed in millimeters.# 6.6 UFBGA100 package information (AOC2) 

This UFBGA is a 100-ball, $7 \times 7 \mathrm{~mm}, 0.50 \mathrm{~mm}$ pitch, ultra fine pitch ball grid array package.
Note: See list of notes in the notes section.
Figure 49. UFBGA100 - Outline ${ }^{(13)}$
![img-50.jpeg](img-50.jpeg)Table 57. UFBGA100 - Mechanical data

| Symbol | millimeters ${ }^{(1)}$ |  |  | inches ${ }^{(12)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min. | Typ. | Max. | Min. | Typ. | Max. |
| $A^{(2)(3)}$ | - | - | 0.60 | - | - | 0.0236 |
| $A 1^{(4)}$ | 0.05 | - | - | 0.0020 | - | - |
| A2 | - | 0.43 | - | - | 0.0169 | - |
| $b^{(5)}$ | 0.23 | 0.28 | 0.33 | 0.0090 | 0.0110 | 0.0130 |
| $D^{(6)}$ | 7.00 BSC |  |  | 0.2756 BSC |  |  |
| D1 | 5.50 BSC |  |  | 0.2165 BSC |  |  |
| E | 7.00 BSC |  |  | 0.2756 BSC |  |  |
| E1 | 5.50 BSC |  |  | 0.2165 BSC |  |  |
| $e^{(9)}$ | 0.50 BSC |  |  | 0.0197 BSC |  |  |
| $N^{(11)}$ | 100 |  |  |  |  |  |
| $\mathrm{SD}^{(12)}$ | 0.25 BSC |  |  | 0.0098 BSC |  |  |
| $\mathrm{SE}^{(12)}$ | 0.25 BSC |  |  | 0.0098 BSC |  |  |
| aaa | 0.15 |  |  | 0.0059 |  |  |
| ccc | 0.20 |  |  | 0.0079 |  |  |
| ddd | 0.08 |  |  | 0.0031 |  |  |
| eee | 0.15 |  |  | 0.0059 |  |  |
| fff | 0.05 |  |  | 0.0020 |  |  |

# Notes: 

1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-2009 apart European projection.
2. UFBGA stands for ulta profile fine pitch ball grid array: $0.50 \mathrm{~mm}<\mathrm{A} \leq 0.65 \mathrm{~mm} /$ fine pitch $\mathrm{e}<1.00 \mathrm{~mm}$.
3. The profile height, $A$, is the distance from the seating plane to the highest point on the package. It is measured perpendicular to the seating plane.
4. A1 is defined as the distance from the seating plane to the lowest point on the package body.
5. Dimension b is measured at the maximum diameter of the terminal (ball) in a plane parallel to primary datum C .
6. BSC stands for BASIC dimensions. It corresponds to the nominal value and has no tolerance. For tolerances refer to form and position table. On the drawing these dimensions are framed.
7. Primary datum $C$ is defined by the plane established by the contact points of three or more solder balls that support the device when it is placed on top of a planar surface.
8. The terminal (ball) A1 corner must be identified on the top surface of the package by using a corner chamfer, ink or metalized markings, or other feature of package body orintegral heat slug. A distinguish feature is allowable on the bottom surface of the package to identify the terminal A1 corner. Exact shape of each corner is optional.
9. e represents the solder ball grid pitch.
10. N represents the total number of balls on the BGA.
11. Basic dimensions SD and SE are defined with respect to datums $A$ and $B$. It defines the position of the centre ball(s) in the outer row or column of a fully populated matrix.
12. Values in inches are converted from mm and rounded to 4 decimal digits.
13. Drawing is not to scale.

Figure 50. UFBGA100 - Footprint example
![img-51.jpeg](img-51.jpeg)

Table 58. UFBGA100 - Example of PCB design rules ( 0.5 mm pitch BGA)

| Dimension | Values |
| :-- | :-- |
| Pitch | 0.50 mm |
| Dpad | 0.280 mm |
| Dsm | 0.370 mm typ. (depends on the solder mask <br> registration tolerance) |
| Stencil opening | 0.280 mm |
| Stencil thickness | Between 0.100 mm and 0.125 mm |# 6.7 LQFP64 package information (5W) 

This LQFP is 64-pin, $10 \times 10 \mathrm{~mm}$ low-profile quad flat package.
Note: $\quad$ See list of notes in the notes section.
Figure 51. LQFP64 - Outline ${ }^{(15)}$
![img-52.jpeg](img-52.jpeg)Table 59. LQFP64 - Mechanical data

| Symbol | millimeters |  |  | inches ${ }^{(14)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.60 | - | - | 0.0630 |
| A1 ${ }^{(12)}$ | 0.05 | - | 0.15 | 0.0020 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0570 |
| $b^{(9)(11)}$ | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1 ${ }^{(11)}$ | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0091 |
| $c^{(11)}$ | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1 ${ }^{(11)}$ | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| $D^{(4)}$ | 12.00 BSC |  |  | 0.4724 BSC |  |  |
| D1 ${ }^{(2)(5)}$ | 10.00 BSC |  |  | 0.3937 BSC |  |  |
| $E^{(4)}$ | 12.00 BSC |  |  | 0.4724 BSC |  |  |
| E1 ${ }^{(2)(5)}$ | 10.00 BSC |  |  | 0.3937 BSC |  |  |
| e | 0.50 BSC |  |  | 0.1970 BSC |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.0177 | 0.0236 | 0.0295 |
| L1 | 1.00 REF |  |  | 0.0394 REF |  |  |
| $\mathrm{N}^{(13)}$ | 64 |  |  |  |  |  |
| $\theta$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ |
| $\theta 1$ | $0^{\circ}$ | - | - | $0^{\circ}$ | - | - |
| $\theta 2$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ |
| $\theta 3$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ |
| R1 | 0.08 | - | - | 0.0031 | - | - |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | - | 0.0079 | - | - |
| aaa ${ }^{(1)}$ | 0.20 |  |  | 0.0079 |  |  |
| bbb ${ }^{(1)}$ | 0.20 |  |  | 0.0079 |  |  |
| $\mathrm{ccc}^{(1)}$ | 0.08 |  |  | 0.0031 |  |  |
| $\mathrm{ddd}^{(1)}$ | 0.08 |  |  | 0.0031 |  |  |# Notes: 

1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The Top package body size may be smaller than the bottom package size by as much as 0.15 mm .
3. Datums A-B and D to be determined at datum plane H .
4. To be determined at seating datum plane $C$.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash or protrusions is " 0.25 mm " per side. D1 and E1 are Maximum plastic body size dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All Dimensions are in millimeters.
8. No intrusion allowed inwards the leads.
9. Dimension "b" does not include dambar protrusion. Allowable dambar protrusion shall not cause the lead width to exceed the maximum "b" dimension by more than 0.08 mm . Dambar cannot be located on the lower radius or the foot. Minimum space between protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. Exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead between 0.10 mm and 0.25 mm from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package body.
13. " $N$ " is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to 4 decimal digits.
15. Drawing is not to scale.

Figure 52. LQFP64 - Footprint example
![img-53.jpeg](img-53.jpeg)

1. Dimensions are expressed in millimeters.# 6.8 TFBGA64 package information (R8) 

Figure 53. TFBGA64 - 64-ball, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch thin profile fine pitch ball grid array
![img-54.jpeg](img-54.jpeg)

1. Drawing is not to scale.

Table 60. TFBGA64 - 64-ball, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, thin profile fine pitch ball grid array
package mechanical data

| Symbol | millimeters |  |  | inches $\left.{ }^{(1)}\right.$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.200 | - | - | 0.0472 |
| A1 | 0.150 | - | - | 0.0059 | - | - |
| A2 | - | 0.200 | - | - | 0.0079 | - |
| A4 | - | - | 0.600 | - | - | 0.0236 |
| b | 0.250 | 0.300 | 0.350 | 0.0098 | 0.0118 | 0.0138 |
| D | 4.850 | 5.000 | 5.150 | 0.1909 | 0.1969 | 0.2028 |
| D1 | - | 3.500 | - | - | 0.1378 | - |
| E | 4.850 | 5.000 | 5.150 | 0.1909 | 0.1969 | 0.2028 |Table 60. TFBGA64 - 64-ball, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, thin profile fine pitch ball grid array package mechanical data (continued)

| Symbol | millimeters |  |  | inches $^{(1)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| E1 | - | 3.500 | - | - | 0.1378 | - |
| e | - | 0.500 | - | - | 0.0197 | - |
| F | - | 0.750 | - | - | 0.0295 | - |
| ddd | - | - | 0.080 | - | - | 0.0031 |
| eee | - | - | 0.150 | - | - | 0.0059 |
| fff | - | - | 0.050 | - | - | 0.0020 |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 54. TFBGA64 - 64-ball, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, thin profile fine pitch ball grid array , recommended footprint
![img-55.jpeg](img-55.jpeg)

R8_FP_V1
Table 61. TFBGA64 recommended PCB design rules ( 0.5 mm pitch BGA)

| Dimension | Recommended values |
| :-- | :-- |
| Pitch | 0.5 |
| Dpad | 0.280 mm |
| Dsm | 0.370 mm typ. (depends on the soldermask <br> registration tolerance) |
| Stencil opening | 0.280 mm |
| Stencil thickness | Between 0.100 mm and 1.125 mm |
| Pad trace width | 0.100 mm |# 6.9 LQFP48 package information (5B) 

This LQFP is a 48 -pin, $7 \times 7 \mathrm{~mm}$ low-profile quad flat package
Note: $\quad$ See list of notes in the notes section.
Figure 55. LQFP48 - Outline ${ }^{(15)}$
![img-56.jpeg](img-56.jpeg)Table 62. LQFP48 - Mechanical data

| Symbol | millimeters |  |  | inches ${ }^{(14)}$ |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.60 | - | - | 0.0630 |
| A1 ${ }^{(12)}$ | 0.05 | - | 0.15 | 0.0020 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0571 |
| $b^{(9)(11)}$ | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1 ${ }^{(11)}$ | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0090 |
| $c^{(11)}$ | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1 ${ }^{(11)}$ | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| $D^{(4)}$ | 9.00 BSC |  |  | 0.3543 BSC |  |  |
| D1 ${ }^{(2)(5)}$ | 7.00 BSC |  |  | 0.2756 BSC |  |  |
| $E^{(4)}$ | 9.00 BSC |  |  | 0.3543 BSC |  |  |
| E1 ${ }^{(2)(5)}$ | 7.00 BSC |  |  | 0.2756 BSC |  |  |
| e | 0.50 BSC |  |  | 0.1970 BSC |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.0177 | 0.0236 | 0.0295 |
| L1 | 1.00 REF |  |  | 0.0394 REF |  |  |
| $N^{(13)}$ | 48 |  |  |  |  |  |
| $\theta$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ | $0^{\circ}$ | $3.5^{\circ}$ | $7^{\circ}$ |
| $\theta 1$ | $0^{\circ}$ | - | - | $0^{\circ}$ | - | - |
| $\theta 2$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ |
| $\theta 3$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ | $10^{\circ}$ | $12^{\circ}$ | $14^{\circ}$ |
| R1 | 0.08 | - | - | 0.0031 | - | - |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | - | 0.0079 | - | - |
| aaa ${ }^{(1)(7)}$ | 0.20 |  |  | 0.0079 |  |  |
| bbb ${ }^{(1)(7)}$ | 0.20 |  |  | 0.0079 |  |  |
| $\operatorname{ccc}^{(1)(7)}$ | 0.08 |  |  | 0.0031 |  |  |
| $\operatorname{ddd}^{(1)(7)}$ | 0.08 |  |  | 0.0031 |  |  |# Notes: 

1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The Top package body size may be smaller than the bottom package size by as much as 0.15 mm .
3. Datums A-B and D to be determined at datum plane H .
4. To be determined at seating datum plane $C$.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash or protrusions is " 0.25 mm " per side. D1 and E1 are Maximum plastic body size dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All Dimensions are in millimeters.
8. No intrusion allowed inwards the leads.
9. Dimension "b" does not include dambar protrusion. Allowable dambar protrusion shall not cause the lead width to exceed the maximum "b" dimension by more than 0.08 mm . Dambar cannot be located on the lower radius or the foot. Minimum space between protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. Exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead between 0.10 mm and 0.25 mm from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package body.
13. " N " is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to 4 decimal digits.
15. Drawing is not to scale.

Figure 56. LQFP48 - Footprint example
![img-57.jpeg](img-57.jpeg)

1. Dimensions are expressed in millimeters.# 6.10 Thermal characteristics 

The maximum chip junction temperature ( $\mathrm{T}_{\mathrm{J}} \max$ ) must never exceed the values given in Table 9: General operating conditions.

The maximum chip-junction temperature, $\mathrm{T}_{\mathrm{J}}$ max, in degrees Celsius, may be calculated using the following equation:

$$
\mathrm{T}_{\mathrm{J}} \max =\mathrm{T}_{\mathrm{A}} \max +\left(\mathrm{P}_{\mathrm{D}} \max \times \Theta_{\mathrm{IA}}\right)
$$

where:

- $\quad T_{A}$ max is the maximum ambient temperature in ${ }^{\circ} \mathrm{C}$,
- $\Theta_{\text {IA }}$ is the package junction-to-ambient thermal resistance, in ${ }^{\circ} \mathrm{C} / \mathrm{W}$,
- $\quad \mathrm{P}_{\mathrm{D}}$ max is the sum of $\mathrm{P}_{\mathrm{INT}}$ max and $\mathrm{P}_{\mathrm{I} / \mathrm{O}}$ max ( $\mathrm{P}_{\mathrm{D}} \max =\mathrm{P}_{\mathrm{INT}} \max +\mathrm{P}_{\mathrm{I} / \mathrm{O}} \max$ ),
- $\quad \mathrm{P}_{\mathrm{INT}}$ max is the product of $\mathrm{I}_{\mathrm{DD}}$ and $\mathrm{V}_{\mathrm{DD}}$, expressed in Watts. This is the maximum chip internal power.
$\mathrm{P}_{\mathrm{I} / \mathrm{O}}$ max represents the maximum power dissipation on output pins where:

$$
P_{I / O} \max =\Sigma\left(V_{O L} \times I_{O L}\right)+\Sigma\left(\left(V_{D D}-V_{O H}\right) \times I_{O H}\right)
$$

taking into account the actual $\mathrm{V}_{\mathrm{OL}} / \mathrm{I}_{\mathrm{OL}}$ and $\mathrm{V}_{\mathrm{OH}} / \mathrm{I}_{\mathrm{OH}}$ of the $\mathrm{I} / \mathrm{Os}$ at low and high level in the application.

Table 63. Package thermal characteristics

| Symbol | Parameter | Value | Unit |
| :--: | :--: | :--: | :--: |
| $\Theta_{\text {IA }}$ | Thermal resistance junction-ambient LFBGA100 - $10 \times 10 \mathrm{~mm} / 0.8 \mathrm{~mm}$ pitch | 44 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$ |
|  | Thermal resistance junction-ambient LQFP100 - $14 \times 14 \mathrm{~mm} / 0.5 \mathrm{~mm}$ pitch | 46 |  |
|  | Thermal resistance junction-ambient UFBGA100 - $7 \times 7 \mathrm{~mm} / 0.5 \mathrm{~mm}$ pitch | 59 |  |
|  | Thermal resistance junction-ambient LQFP64 - $10 \times 10 \mathrm{~mm} / 0.5 \mathrm{~mm}$ pitch | 45 |  |
|  | Thermal resistance junction-ambient TFBGA64 - $5 \times 5 \mathrm{~mm} / 0.5 \mathrm{~mm}$ pitch | 65 |  |
|  | Thermal resistance junction-ambient LQFP48 - $7 \times 7 \mathrm{~mm} / 0.5 \mathrm{~mm}$ pitch | 55 |  |
|  | Thermal resistance junction-ambient UFQFPN 48 - $7 \times 7 \mathrm{~mm} / 0.5 \mathrm{~mm}$ pitch | 32 |  |
|  | Thermal resistance junction-ambient VFQFPN 36 - $6 \times 6 \mathrm{~mm} / 0.5 \mathrm{~mm}$ pitch | 18 |  |

### 6.10.1 Reference document

JESD51-2 Integrated Circuits Thermal Test Method Environment Conditions - Natural Convection (Still Air). Available from www.jedec.org.# 6.10.2 Selecting the product temperature range 

When ordering the microcontroller, the temperature range is specified in the ordering information scheme shown in Section 7.

Each temperature range suffix corresponds to a specific guaranteed ambient temperature at maximum dissipation and, to a specific maximum junction temperature.

As applications do not commonly use the STM32F103xx at maximum dissipation, it is useful to calculate the exact power consumption and junction temperature to determine which temperature range will be best suited to the application.

The following examples show how to calculate the temperature range needed for a given application.

## Example 1: High-performance application

Assuming the following application conditions:
Maximum ambient temperature $\mathrm{T}_{\text {Amax }}=82^{\circ} \mathrm{C}$ (measured according to JESD51-2),
$\mathrm{I}_{\mathrm{DDmax}}=50 \mathrm{~mA}, \mathrm{~V}_{\mathrm{DD}}=3.5 \mathrm{~V}$, maximum $20 \mathrm{I} / \mathrm{Os}$ used at the same time in output at low level with $\mathrm{I}_{\mathrm{OL}}=8 \mathrm{~mA}, \mathrm{~V}_{\mathrm{OL}}=0.4 \mathrm{~V}$ and maximum $8 \mathrm{I} / \mathrm{Os}$ used at the same time in output at low level with $\mathrm{I}_{\mathrm{OL}}=20 \mathrm{~mA}, \mathrm{~V}_{\mathrm{OL}}=1.3 \mathrm{~V}$
$\mathrm{P}_{\mathrm{INT} \max }=50 \mathrm{~mA} \times 3.5 \mathrm{~V}=175 \mathrm{~mW}$
$\mathrm{P}_{\mathrm{IOmax}}=20 \times 8 \mathrm{~mA} \times 0.4 \mathrm{~V}+8 \times 20 \mathrm{~mA} \times 1.3 \mathrm{~V}=272 \mathrm{~mW}$
This gives: $\mathrm{P}_{\mathrm{INT} \max }=175 \mathrm{~mW}$ and $\mathrm{P}_{\mathrm{IO} \max }=272 \mathrm{~mW}$ :
$\mathrm{P}_{\mathrm{Dmax}}=175+272=447 \mathrm{~mW}$
Thus: $\mathrm{P}_{\mathrm{D} \max }=447 \mathrm{~mW}$
Using the values obtained in Table $63 \mathrm{~T}_{\text {Jmax }}$ is calculated as follows:

- For LQFP100, $46^{\circ} \mathrm{C} / \mathrm{W}$
$\mathrm{T}_{\mathrm{J} \max }=82^{\circ} \mathrm{C}+\left(46^{\circ} \mathrm{C} / \mathrm{W} \times 447 \mathrm{~mW}\right)=82^{\circ} \mathrm{C}+20.6^{\circ} \mathrm{C}=102.6^{\circ} \mathrm{C}$
This is within the range of the suffix 6 version parts $\left(-40<\mathrm{T}_{\mathrm{J}}<105^{\circ} \mathrm{C}\right)$.
In this case, parts must be ordered at least with the temperature range suffix 6 (see Section 7).


## Example 2: High-temperature application

Using the same rules, it is possible to address applications that run at high ambient temperatures with a low dissipation, as long as junction temperature $\mathrm{T}_{\mathrm{J}}$ remains within the specified range.
Assuming the following application conditions:
Maximum ambient temperature $\mathrm{T}_{\text {Amax }}=115^{\circ} \mathrm{C}$ (measured according to JESD51-2),
$\mathrm{I}_{\mathrm{DD} \max }=20 \mathrm{~mA}, \mathrm{~V}_{\mathrm{DD}}=3.5 \mathrm{~V}$, maximum $20 \mathrm{I} / \mathrm{Os}$ used at the same time in output at low level with $\mathrm{I}_{\mathrm{OL}}=8 \mathrm{~mA}, \mathrm{~V}_{\mathrm{OL}}=0.4 \mathrm{~V}$
$\mathrm{P}_{\mathrm{INT} \max }=20 \mathrm{~mA} \times 3.5 \mathrm{~V}=70 \mathrm{~mW}$
$\mathrm{P}_{\mathrm{IO} \max }=20 \times 8 \mathrm{~mA} \times 0.4 \mathrm{~V}=64 \mathrm{~mW}$
This gives: $\mathrm{P}_{\mathrm{INT} \max }=70 \mathrm{~mW}$ and $\mathrm{P}_{\mathrm{IO} \max }=64 \mathrm{~mW}$ :
$\mathrm{P}_{\mathrm{D} \max }=70+64=134 \mathrm{~mW}$
Thus: $\mathrm{P}_{\mathrm{D} \max }=134 \mathrm{~mW}$Using the values obtained in Table $63 T_{\text {Jmax }}$ is calculated as follows:

- For LQFP100, $46^{\circ} \mathrm{C} / \mathrm{W}$

$$
T_{\text {Jmax }}=115^{\circ} \mathrm{C}+\left(46^{\circ} \mathrm{C} / \mathrm{W} \times 134 \mathrm{~mW}\right)=115^{\circ} \mathrm{C}+6.2^{\circ} \mathrm{C}=121.2^{\circ} \mathrm{C}
$$

This is within the range of the suffix 7 version parts $\left(-40<T_{J}<125^{\circ} \mathrm{C}\right)$.
In this case, parts must be ordered at least with the temperature range suffix 7 (see Section 7).

Figure 57. LQFP100 $\mathrm{P}_{\mathrm{D}}$ max vs. $\mathrm{T}_{\mathrm{A}}$
![img-58.jpeg](img-58.jpeg)# 7 Ordering information scheme 

![img-59.jpeg](img-59.jpeg)# 8 Important security notice 

The STMicroelectronics group of companies (ST) places a high value on product security, which is why the ST product(s) identified in this documentation may be certified by various security certification bodies and/or may implement our own security measures as set forth herein. However, no level of security certification and/or built-in security measures can guarantee that ST products are resistant to all forms of attacks. As such, it is the responsibility of each of ST's customers to determine if the level of security provided in an ST product meets the customer needs both in relation to the ST product alone, as well as when combined with other components and/or software for the customer end product or application. In particular, take note that:

- ST products may have been certified by one or more security certification bodies, such as Platform Security Architecture (www.psacertified.org) and/or Security Evaluation standard for loT Platforms (www.trustcb.com). For details concerning whether the ST product(s) referenced herein have received security certification along with the level and current status of such certification, either visit the relevant certification standards website or go to the relevant product page on www.st.com for the most up to date information. As the status and/or level of security certification for an ST product can change from time to time, customers should re-check security certification status/level as needed. If an ST product is not shown to be certified under a particular security standard, customers should not assume it is certified.
- Certification bodies have the right to evaluate, grant and revoke security certification in relation to ST products. These certification bodies are therefore independently responsible for granting or revoking security certification for an ST product, and ST does not take any responsibility for mistakes, evaluations, assessments, testing, or other activity carried out by the certification body with respect to any ST product.
- Industry-based cryptographic algorithms (such as AES, DES, or MD5) and other open standard technologies which may be used in conjunction with an ST product are based on standards which were not developed by ST. ST does not take responsibility for any flaws in such cryptographic algorithms or open technologies or for any methods which have been or may be developed to bypass, decrypt or crack such algorithms or technologies.
- While robust security testing may be done, no level of certification can absolutely guarantee protections against all attacks, including, for example, against advanced attacks which have not been tested for, against new or unidentified forms of attack, or against any form of attack when using an ST product outside of its specification or intended use, or in conjunction with other components or software which are used by customer to create their end product or application. ST is not responsible for resistance against such attacks. As such, regardless of the incorporated security features and/or any information or support that may be provided by ST, each customer is solely responsible for determining if the level of attacks tested for meets their needs, both in relation to the ST product alone and when incorporated into a customer end product or application.
- All security features of ST products (inclusive of any hardware, software, documentation, and the like), including but not limited to any enhanced security features added by ST, are provided on an "AS IS" BASIS. AS SUCH, TO THE EXTENT PERMITTED BY APPLICABLE LAW, ST DISCLAIMS ALL WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED WARRANTIES OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, unless the applicable written and signed contract terms specifically provide otherwise.# 9 Revision history 

Table 64. Document revision history

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 01-Jun-2007 | 1 | Initial release. |
| 20-Jul-2007 | 2 | Flash memory size modified in Note 9, Note 5, Note 7, Note 7 and BGA100 pins added to Table 5: Medium-density STM32F103xx pin definitions. <br> Figure 3: STM32F103xx performance line LFBGA100 ballout added. $\mathrm{T}_{\text {HSE }}$ changed to $\mathrm{T}_{\mathrm{LSE}}$ in Figure 23: Low-speed external clock source AC timing diagram. $\mathrm{V}_{\mathrm{BAT}}$ ranged modified in Power supply schemes. $\mathrm{t}_{\mathrm{SU(LSE)}}$ changed to $\mathrm{t}_{\mathrm{SU(HSE)}}$ in Table 22: HSE 4-16 MHz oscillator characteristics. $\mathrm{I}_{\mathrm{DD}(\mathrm{HSI})}$ max value added to Table 24: HSI oscillator characteristics. <br> Sample size modified and machine model removed in Electrostatic discharge (ESD). <br> Number of parts modified and standard reference updated in Static latchup. $25^{\circ} \mathrm{C}$ and $85^{\circ} \mathrm{C}$ conditions removed and class name modified in Table 33: Electrical sensitivities. $R_{P U}$ and $R_{P D}$ min and max values added to Table 35: I/O static characteristics. $R_{P U}$ min and max values added to Table 38: NRST pin characteristics. <br> Figure 32: I2C bus AC waveforms and measurement circuit and Figure 31: Recommended NRST pin protection corrected. <br> Notes removed below Table 9, Table 38, Table 44. <br> $\mathrm{I}_{\mathrm{DD}}$ typical values changed in Table 11: Maximum current consumption in Run and Sleep modes. Table 39: TIMx characteristics modified. $\mathrm{t}_{\text {STAB }}, \mathrm{V}_{\text {REF }}$, value, $\mathrm{t}_{\text {lat }}$ and $\mathrm{f}_{\text {TRIG }}$ added to Table 46: ADC characteristics. In Table: typical endurance and data retention for $\mathrm{T}_{\mathrm{A}}=85^{\circ} \mathrm{C}$ added, data retention for $\mathrm{T}_{\mathrm{A}}=25^{\circ} \mathrm{C}$ removed. <br> $\mathrm{V}_{\mathrm{BG}}$ changed to $\mathrm{V}_{\text {REFINT }}$ in Table 12: Embedded internal reference voltage. Document title changed. Controller area network (CAN) section modified. <br> Figure 14: Power supply scheme modified. <br> Features on page 1 list optimized. Small text changes. |Table 64. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 18-Oct-2007 | 3 | STM32F103CBT6, STM32F103T6 and STM32F103T8 root part numbers added (see Table 2: STM32F103xx medium-density device features and peripheral counts) <br> VFQFPN36 package added (see Section 6: Package information). All packages are ECOPACK® compliant. Package mechanical data inch values are calculated from mm and rounded to 4 decimal digits (see Section 6: Package information). <br> Table 5: Medium-density STM32F103xx pin definitions updated and clarified. <br> Table 26: Low-power mode wakeup timings updated. $\mathrm{T}_{\mathrm{A}}$ min corrected in Table 12: Embedded internal reference voltage. Note 2 added below Table 22: HSE 4-16 MHz oscillator characteristics. $\mathrm{V}_{\text {ESD(CDM) }}$ value added to Table 32: ESD absolute maximum ratings. Note 4 added and $\mathrm{V}_{\mathrm{CM}}$ parameter description modified in Table 36: Output voltage characteristics. <br> Note 1 modified under Table 37: I/O AC characteristics. <br> Equation 1 and Table 47: $\mathrm{R}_{\mathrm{AIN}}$ max for $\mathrm{f}_{\mathrm{ADC}}=14 \mathrm{MHz}$ added to Section 5.3.18: 12-bit ADC characteristics. $\mathrm{V}_{\mathrm{AIN}}, \mathrm{t}_{\mathrm{S}}$ max, $\mathrm{t}_{\mathrm{CONV}}, \mathrm{V}_{\mathrm{REF}+}$ min and $\mathrm{t}_{\text {lat }}$ max modified, notes modified and $\mathrm{t}_{\text {latr }}$ added in Table 46: ADC characteristics. <br> Figure 37: ADC accuracy characteristics updated. Note 1 modified below Figure 38: Typical connection diagram using the ADC. <br> Electrostatic discharge (ESD) on page 59 modified. <br> Number of TIM4 channels modified in Figure 1: STM32F103xx performance line block diagram. <br> Maximum current consumption Table 13, Table 14 and Table 15 updated. $\mathrm{V}_{\text {hys }}$ modified in Table 35: I/O static characteristics. <br> Table 49: ADC accuracy updated. $\mathrm{V}_{\text {FESD }}$ value added in Table 30: EMS characteristics. Values corrected, note 2 modified and note 3 removed in Table 26: Low-power mode wakeup timings. <br> Table 16: Typical and maximum current consumptions in Stop and Standby modes: Typical values added for $\mathrm{V}_{\mathrm{DD}} / \mathrm{V}_{\mathrm{BAT}}=2.4 \mathrm{~V}$, Note 2 modified, Note 2 added. <br> Table 21: Typical current consumption in Standby mode added. On-chip peripheral current consumption on page 49 added. $\mathrm{ACC}_{\text {HSI }}$ values updated in Table 24: HSI oscillator characteristics. $\mathrm{V}_{\text {prog }}$ added to Table 28: Flash memory characteristics. <br> Upper option byte address modified in Figure 11: Memory map. <br> Typical $f_{\text {LSI }}$ value added in Table 25: LSI oscillator characteristics and internal RC value corrected from 32 to 40 kHz in entire document. $\mathrm{T}_{\mathrm{S} \text { _temp }}$ added to Table 50: TS characteristics. $\mathrm{N}_{\text {END }}$ modified in Table. $\mathrm{T}_{\mathrm{S} \text { _vrefint }}$ added to Table 12: Embedded internal reference voltage. Handling of unused pins specified in General input/output characteristics on page 61. All I/Os are CMOS and TTL compliant. <br> Figure 39: Power supply and reference decoupling ( $\mathrm{V}_{\mathrm{REF}+}$ not connected to $\mathrm{V}_{\mathrm{DDA}}$ ) modified. <br> $\mathrm{t}_{\text {JITTER }}$ and $\mathrm{f}_{\mathrm{VCO}}$ removed from Table 27: PLL characteristics. Appendix A: Important notes on page 81 added. <br> Added Figure 16, Figure 17, Figure 19 and Figure 21. |Table 64. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 22-Nov-2007 | 4 | Document status promoted from preliminary data to datasheet. STM32F103xx is USB certified. Small text changes. <br> Power supply schemes on page 15 modified. Number of communication peripherals corrected for STM32F103Tx and number of GPIOs corrected for LQFP package in Table 2: STM32F103xx medium-density device features and peripheral counts. <br> Main function and default alternate function modified for PC14 and PC15 in, Note 6 added and Remap column added in Table 5: Medium-density STM32F103xx pin definitions. <br> $\mathrm{V}_{\mathrm{DD}}-\mathrm{V}_{\mathrm{SS}}$ ratings and Note 1 modified in Table 6: Voltage characteristics, Note 1 modified in Table 7: Current characteristics. <br> Note 1 and Note 2 added in Table 11: Embedded reset and power control block characteristics. <br> $\mathrm{I}_{\mathrm{DD}}$ value at 72 MHz with peripherals enabled modified in Table 14: Maximum current consumption in Run mode, code with data processing running from RAM. <br> $\mathrm{I}_{\mathrm{DD}}$ value at 72 MHz with peripherals enabled modified in Table 15: Maximum current consumption in Sleep mode, code running from Flash or RAM on page 43. <br> $\mathrm{I}_{\mathrm{DD} \_ \text {VBBAT }}$ typical value at 2.4 V modified and $\mathrm{I}_{\mathrm{DD} \_ \text {VBAT }}$ maximum values added in Table 16: Typical and maximum current consumptions in Stop and Standby modes. Note added in Table 17 on page 47 and Table 18 on page 48. ADC1 and ADC2 consumption and notes modified in Table 19: Peripheral current consumption. <br> $\mathrm{t}_{\mathrm{SU}(\mathrm{HSE})}$ and $\mathrm{t}_{\mathrm{SU}(\mathrm{LSE})}$ conditions modified in Table 22 and Table 23, respectively. <br> Maximum values removed from Table 26: Low-power mode wakeup timings. $\mathrm{t}_{\text {RET }}$ conditions modified in Table. Figure 14: Power supply scheme corrected. <br> Figure 20: Typical current consumption in Stop mode, with regulator in Low-power mode added. <br> Note removed below Figure 33: SPI timing diagram - slave mode and CPHA $=0$. Note added below Figure 34: SPI timing diagram - slave mode and CPHA $=1(1)$. <br> Details on unused pins removed from General input/output characteristics on page 61. <br> Table 42: SPI characteristics updated. Table 43: USB startup time added. $\mathrm{V}_{\mathrm{AIN}}, \mathrm{I}_{\text {lat }}$ and $\mathrm{I}_{\text {latr }}$ modified, note added and $\mathrm{I}_{\text {lkg }}$ removed in Table 46: ADC characteristics. Test conditions modified and note added in Table 49: ADC accuracy. Note added below Table 47 and Table 50. <br> Inch values corrected in Table 55: LQPF100 mechanical data, Table 58: LQFP64 mechanical data and Table 60: LQFP48, $7 \times 7 \mathrm{~mm}, 48$-pin lowprofile quad flat package mechanical data. <br> $\Theta_{J A}$ value for VFQFPN36 package added in Table 62: Package thermal characteristics. <br> Order codes replaced by Section 7: Ordering information scheme. <br> MCU 's operating conditions modified in Typical current consumption on page 46. Avg_Slope and $\mathrm{V}_{2 \mathrm{~S}}$ modified in Table 50: TS characteristics. I2C interface characteristics on page 68 modified. <br> Impedance specified in A.4: Voltage glitch on ADC input 0 on page 81. |Table 64. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 14-Mar-2008 | 5 | Figure 2: Clock tree on page 12 added. <br> Maximum $T_{J}$ value given in Table 8: Thermal characteristics on page 37. <br> CRC feature added (see CRC (cyclic redundancy check) calculation unit on page 9 and Figure 11: Memory map on page 34 for address). <br> $\mathrm{I}_{\mathrm{DD}}$ modified in Table 16: Typical and maximum current consumptions in Stop and Standby modes. <br> ACC $_{\text {HSI }}$ modified in Table 24: HSI oscillator characteristics on page 54, note 2 removed. <br> $P_{D}, T_{A}$ and $T_{J}$ added, $t_{\text {prog }}$ values modified and $t_{\text {prog }}$ description clarified in Table 28: Flash memory characteristics on page 56. <br> $\mathrm{t}_{\text {RET }}$ modified in Table. <br> $\mathrm{V}_{\mathrm{NF}(\mathrm{NRST})}$ unit corrected in Table 38: NRST pin characteristics on page 66. <br> Table 42: SPI characteristics on page 70 modified. <br> $\mathrm{I}_{\mathrm{VREF}}$ added to Table 46: ADC characteristics on page 74. <br> Table 48: ADC accuracy - Limited test conditions added. Table 49: ADC accuracy modified. <br> LQFP100 package specifications updated (see Section 6: Package information on page 79). <br> Recommended LQFP100, LQFP 64, LQFP48 and VFQFPN36 footprints added (see Figure 55, Figure 60, Figure 64 and Figure 44). <br> Section 6.9: Thermal characteristics on page 104 modified, Section 6.9.1 and Section 6.9.2 added. <br> Appendix A: Important notes on page 81 removed. |
| 21-Mar-2008 | 6 | Small text changes. Figure 11: Memory map clarified. In Table: <br> $\mathrm{N}_{\mathrm{END}}$ tested over the whole temperature range, cycling conditions specified for $\mathrm{t}_{\mathrm{RET}}, \mathrm{t}_{\mathrm{RET}} \mathrm{min}$ modified at $\mathrm{T}_{\mathrm{A}}=55^{\circ} \mathrm{C}$ <br> $\mathrm{V}_{25}$, Avg_Slope and $\mathrm{T}_{\mathrm{L}}$ modified in Table 50: TS characteristics. <br> CRC feature removed. |
| 22-May-2008 | 7 | CRC feature added back. Small text changes. Section 1: Introduction modified. Section 2.2: Full compatibility throughout the family added. $\mathrm{I}_{\mathrm{DD}}$ at $\mathrm{T}_{\mathrm{A}} \max =105^{\circ} \mathrm{C}$ added to Table 16: Typical and maximum current consumptions in Stop and Standby modes on page 44. <br> $\mathrm{I}_{\mathrm{DD} \_\text {VBAT }}$ removed from Table 21: Typical current consumption in Standby mode on page 47. <br> Values added to Table 41: SCL frequency $\left(f_{\text {PCLK1 }}=36 \mathrm{MHz}, V_{\mathrm{DD} \_\mathrm{I2C}}=\right.$ $3.3 \mathrm{~V})$ on page 69 . <br> Figure 33: SPI timing diagram - slave mode and CPHA $=0$ on page 71 modified. Equation 1 corrected. <br> $\mathrm{t}_{\mathrm{RET}}$ at $\mathrm{T}_{\mathrm{A}}=105^{\circ} \mathrm{C}$ modified in Table on page 57. <br> $\mathrm{V}_{\text {USB }}$ added to Table 44: USB DC electrical characteristics on page 73. <br> Figure 65: LQFP100 $\mathrm{P}_{\mathrm{D}}$ max vs. $\mathrm{T}_{\mathrm{A}}$ on page 106 modified. <br> Axx option added to Table 63: Ordering information scheme on page 110. |Table 64. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 21-Jul-2008 | 8 | Power supply supervisor updated and $\mathrm{V}_{\text {DDA }}$ added to Table 9: General operating conditions. <br> Capacitance modified in Figure 14: Power supply scheme on page 36. <br> Table notes revised in Section 5: Electrical characteristics. <br> Table 16: Typical and maximum current consumptions in Stop and Standby modes modified. <br> Data added to Table 16: Typical and maximum current consumptions in Stop and Standby modes and Table 21: Typical current consumption in Standby mode removed. <br> $\mathrm{f}_{\text {HSE_ext }}$ modified in Table 20: High-speed external user clock characteristics on page 50. $\mathrm{f}_{\text {PLL_IN }}$ modified in Table 27: PLL characteristics on page 56. <br> Minimum SDA and SCL fall time value for Fast mode removed from Table 40: I2C characteristics on page 68, note 1 modified. <br> $\mathrm{I}_{\text {NINSS) }}$ modified in Table 42: SPI characteristics on page 70 and Figure 33: SPI timing diagram - slave mode and CPHA $=0$ on page 71. <br> $\mathrm{C}_{\text {ADC }}$ modified in Table 46: ADC characteristics on page 74 and Figure 38: Typical connection diagram using the ADC modified. <br> Typical $\mathrm{T}_{\mathrm{S} \text { _temp }}$ value removed from Table 50: TS characteristics on page 78. <br> LQFP48 package specifications updated (see Table 60 and Table 64), Section 6: Package information revised. <br> Axx option removed from Table 63: Ordering information scheme on page 110. <br> Small text changes. |
| 22-Sep-2008 | 9 | STM32F103x6 part numbers removed (see Table 63: Ordering information scheme). Small text changes. <br> General-purpose timers (TIMx) and Advanced-control timer (TIM1) on page 18 updated. <br> Notes updated in Table 5: Medium-density STM32F103xx pin definitions on page 28. <br> Note 2 modified below Table 6: Voltage characteristics on page 37, $\left\|\Delta \mathrm{V}_{\mathrm{DDx}}\right\|$ min and $\left\|\Delta \mathrm{V}_{\mathrm{DDx}}\right\|$ min removed. <br> Measurement conditions specified in Section 5.3.5: Supply current characteristics on page 40. <br> $\mathrm{I}_{\mathrm{DD}}$ in standby mode at $85^{\circ} \mathrm{C}$ modified in Table 16: Typical and maximum current consumptions in Stop and Standby modes on page 44. <br> General input/output characteristics on page 61 modified. <br> $\mathrm{f}_{\text {HCLK }}$ conditions modified in Table 30: EMS characteristics on page 58. <br> $\Theta_{J A}$ and pitch value modified for LFBGA100 package in Table 62: <br> Package thermal characteristics. Small text changes. |Table 64. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 23-Apr-2009 | 10 | I/O information clarified on page 1. <br> Figure 3: STM32F103xx performance line LFBGA100 ballout modified. <br> Figure 11: Memory map modified. <br> Table 4: Timer feature comparison added. <br> PB4, PB13, PB14, PB15, PB3/TRACESWO moved from Default column <br> to Remap column in Table 5: Medium-density STM32F103xx pin <br> definitions. <br> $\mathrm{P}_{\mathrm{D}}$ for LFBGA100 corrected in Table 9: General operating conditions. <br> Note modified in Table 13: Maximum current consumption in Run mode, <br> code with data processing running from Flash and Table 15: Maximum <br> current consumption in Sleep mode, code running from Flash or RAM. <br> Table 20: High-speed external user clock characteristics and Table 21: <br> Low-speed external user clock characteristics modified. <br> Figure 20 shows a typical curve (title modified). ACC $_{\text {HSI }}$ max values <br> modified in Table 24: HSI oscillator characteristics. <br> TFBGA64 package added (see Table 59 and Table 60). Small text <br> changes. |
| 22-Sep-2009 | 11 | Note 5 updated and Note 4 added in Table 5: Medium-density <br> STM32F103xx pin definitions. <br> $\mathrm{V}_{\text {RERINT }}$ and $\mathrm{T}_{\text {Coeff }}$ added to Table 12: Embedded internal reference <br> voltage. $\mathrm{I}_{\mathrm{DD} \_ \text {VBAT }}$ value added to Table 16: Typical and maximum current <br> consumptions in Stop and Standby modes. Figure 18: Typical current <br> consumption on $\mathrm{V}_{\mathrm{BAT}}$ (RTC on) added. <br> $\mathrm{f}_{\text {HSE ext }}$ min modified in Table 20: High-speed external user clock <br> characteristics. <br> $\mathrm{C}_{\mathrm{L} 1}$ and $\mathrm{C}_{\mathrm{L} 2}$ replaced by C in Table 22: HSE 4-16 MHz oscillator <br> characteristics and Table 23: LSE oscillator characteristics ( $\mathrm{f}_{\mathrm{LSE}}=32.768$ <br> kHz ), notes modified and moved below the tables. Table 24: HSI oscillator <br> characteristics modified. Conditions removed from Table 26: Low-power <br> mode wakeup timings. <br> Note 1 modified below Figure 24: Typical application with an 8 MHz <br> crystal. <br> IEC 1000 standard updated to IEC 61000 and SAE J1752/3 updated to <br> IEC 61967-2 in Section 5.3.10: EMC characteristics on page 57. <br> Jitter added to Table 27: PLL characteristics. <br> Table 42: SPI characteristics modified. <br> $\mathrm{C}_{\text {ADC }}$ and $\mathrm{R}_{\text {AIN }}$ parameters modified in Table 46: ADC characteristics. <br> $\mathrm{R}_{\text {AIN }}$ max values modified in Table 47: $\mathrm{R}_{\text {AIN }}$ max for $\mathrm{f}_{\text {ADC }}=14 \mathrm{MHz}$. <br> Figure 47: LFBGA100 outline updated. |
| 03-Jun-2010 | 12 | Added STM32F103TB devices. <br> Added VFQFPN48 package. <br> Updated note 2 below Table 40: I²C characteristics <br> Updated Figure 32: I²C bus AC waveforms and measurement circuit <br> Updated Figure 31: Recommended NRST pin protection <br> Updated Section 5.3.12: I/O current injection characteristics |Table 64. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 19-Apr-2011 | 13 | Updated footnotes below Table 6: Voltage characteristics on page 37 and Table 7: Current characteristics on page 37 <br> Updated tw min in Table 20: High-speed external user clock characteristics on page 50 <br> Updated startup time in Table 23: LSE oscillator characteristics ( $\mathrm{f}_{\mathrm{LSE}}=$ 32.768 kHz ) on page 53 <br> Added Section 5.3.12: I/O current injection characteristics <br> Updated Section 5.3.13: I/O port characteristics |
| 07-Dec-2012 | 14 | Added UFBGA100 $7 \times 7 \mathrm{~mm}$. <br> Updated Figure 59: LQFP64, $10 \times 10 \mathrm{~mm}, 64$-pin low-profile quad flat package outline to add pin 1 identification. |
| 14-May-2013 | 15 | Replaced VQFN48 package with UQFN48 in cover page packages, Table 2: STM32F103xx medium-density device features and peripheral counts, Figure 9: STM32F103xx performance line UFQFPN48 pinout, Table 2: STM32F103xx medium-density device features and peripheral counts, Table 56: UFBGA100 mechanical data, Table 63: Ordering information scheme and updated Table 62: Package thermal characteristics Added footnote for TFBGA ADC channels in Table 2: STM32F103xx medium-density device features and peripheral counts <br> Updated 'All GPIOs are high current...' in Section 2.3.21: GPIOs (generalpurpose inputs/outputs) <br> Updated Table 5: Medium-density STM32F103xx pin definitions <br> Corrected Sigma letter in Section 5.1.1: Minimum and maximum values <br> Removed the first sentence in Section 5.3.16: Communications interfaces <br> Added ' $\mathrm{V}_{\mathrm{IN}}$ ' in Table 9: General operating conditions <br> Updated first sentence in Output driving current <br> Added note 5. in Table 24: HSI oscillator characteristics <br> Updated ' $V_{I L}$ ' and ' $V_{I H}$ ' in Table 35: I/O static characteristics <br> Added notes to Figure 26: Standard I/O input characteristics - CMOS port, Figure 27: Standard I/O input characteristics - TTL port, Figure 28: 5 V tolerant I/O input characteristics - CMOS port and Figure 29: 5 V tolerant I/O input characteristics - TTL port <br> Updated Figure 32: $I^{2} \mathrm{C}$ bus AC waveforms and measurement circuit <br> Updated notes 2 and 3,removed note "the device must internally..." in Table 40: $\mathrm{I}^{2} \mathrm{C}$ characteristics <br> Updated title of Table 41: SCL frequency ( $\mathrm{f}_{\text {PCLK1 }}=36 \mathrm{MHz}, \mathrm{V}_{\mathrm{DD} \_12 \mathrm{C}}=3.3$ V) <br> Updated note 2. in Table 49: ADC accuracy <br> Updated Figure 53: UFBGA100 - 100-ball, $7 \times 7 \mathrm{~mm}, 0.50 \mathrm{~mm}$ pitch, ultra fine pitch ball grid array package outline and Table 56: UFBGA100 mechanical data <br> Updated Figure 47: LFBGA100 outline and Table 53: LFBGA100 mechanical data <br> Updated Figure 60: TFBGA64 - $8 \times 8$ active ball array, $5 \times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, package outline and Table 59: TFBGA64 - $8 \times 8$ active ball array, 5 $\times 5 \mathrm{~mm}, 0.5 \mathrm{~mm}$ pitch, package mechanical data |Table 64. Document revision history (continued)

| Date | Revision | Changes |
| :--: | :--: | :--: |
| 05-Aug-2013 | 16 | Updated the reference for ' $\mathrm{V}_{\text {ESD(CDM) }}$ ' in Table 32: ESD absolute maximum ratings <br> Corrected ' $\mathrm{t} f(\mathrm{IO})$ out' in Figure 30: I/O AC characteristics definition Updated Table 52: UFQFPN48 mechanical data |
| 21-Aug-2015 | 17 | Updated Table 3: STM32F103xx family removing the note. <br> Updated Table 63: Ordering information scheme removing the note. <br> Updated Section 6: Package information and added Section: Marking of engineering samples for all packages. <br> Updated $\mathrm{I}^{2} \mathrm{C}$ characteristics, added $\mathrm{t}_{\mathrm{SP}}$ parameter and note 4 in Table 40: $I^{2} \mathrm{C}$ characteristics. <br> Updated Figure 32: $\mathrm{I}^{2} \mathrm{C}$ bus AC waveforms and measurement circuit swapping SCLL and SCLH. <br> Updated Figure 33: SPI timing diagram - slave mode and CPHA $=0$. <br> Updated min/max value notes replacing 'Guaranteed by design, not tested in production" by "guaranteed by design". <br> Updated min/max value notes replacing 'based on characterization, not tested in production" by "Guaranteed based on test during characterization". <br> Updated Table 19: Peripheral current consumption. |
| 29-Mar-2022 | 18 | Updated Table 5: Medium-density STM32F103xx pin definitions. <br> Updated Figure 37: ADC accuracy characteristics, Figure 38: Typical connection diagram using the ADC and its footnotes. <br> Minor text edits across the whole document. |
| 18-Sep-2023 | 19 | Updated Features. <br> Updated Section 1: Introduction. <br> Updated Figure 11: Memory map. <br> Updated Table 24: HSE 4-16 MHz oscillator characteristics and Table 25: <br> LSE oscillator characteristics ( $f_{L S E}=32.768 \mathrm{kHz}$ ). <br> Updated Table 33: EMI characteristics for fHSE $=8 \mathrm{MHz}$ and $\mathrm{fHCLK}=48$ <br> MHz and created Table 34: EMI characteristics for fHSE $=8 \mathrm{MHz}$ and <br> fHCLK $=72 \mathrm{MHz}$. <br> Updated Figure 33: SPI timing diagram - slave mode and CPHA $=0$, <br> Figure 34: SPI timing diagram - slave mode and CPHA $=1$, and <br> Figure 35: SPI timing diagram - master mode. <br> Updated Table 46: USB startup time. <br> Added Section 8: Important security notice. <br> Updated all packages in Section 6: Package information. |
| 28-Jul-2025 | 20 | Updated Section 7: Ordering information scheme. |# IMPORTANT NOTICE - PLEASE READ CAREFULLY 

STMicroelectronics NV and its subsidiaries ("ST") reserve the right to make changes, corrections, enhancements, modifications, and improvements to ST products and/or to this document at any time without notice. Purchasers should obtain the latest relevant information on ST products before placing orders. ST products are sold pursuant to ST's terms and conditions of sale in place at the time of order acknowledgement.

Purchasers are solely responsible for the choice, selection, and use of ST products and ST assumes no liability for application assistance or the design of Purchasers' products.

No license, express or implied, to any intellectual property right is granted by ST herein.

Resale of ST products with provisions different from the information set forth herein shall void any warranty granted by ST for such product.

ST and the ST logo are trademarks of ST. For additional information about ST trademarks, please refer to www.st.com/trademarks. All other product or service names are the property of their respective owners.

Information in this document supersedes and replaces information previously supplied in any prior versions of this document.
(c) 2025 STMicroelectronics - All rights reserved