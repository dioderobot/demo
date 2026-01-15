# AM32 Port for DM0001 (ZenDrive Motor Controller)

## Overview

This is an AM32 firmware port for the DM0001 ZenDrive motor controller board.

## Hardware

- **MCU**: STM32G431C8T6 (Cortex-M4, 170MHz, 64KB Flash, 32KB RAM)
- **Gate Driver**: L6387ED (3x for 3-phase)
- **MOSFETs**: STL180N6F7 (6x - high/low side for each phase)
- **Shunt Resistors**: 3mΩ 2512 (3x for current sensing)

## Pin Mapping

### PWM Outputs (TIM1)
| Phase | High Side | Low Side |
|-------|-----------|----------|
| A     | PA8 (CH1) | PC13 (CH1N) |
| B     | PA9 (CH2) | PA12 (CH2N) |
| C     | PA10 (CH3) | PB15 (CH3N) |

### BEMF Sensing
| Phase | Pin | ADC Channel |
|-------|-----|-------------|
| A     | PA4 | ADC1_IN17 |
| B     | PB12 | ADC1_IN11 |
| C     | PB11 | ADC1_IN14 |

### Other Signals
| Function | Pin | Notes |
|----------|-----|-------|
| PWM Input | PA15 | TIM2_CH1, DSHOT/PWM |
| VBUS Sense | PA0 | ADC1_IN1, 169k/18k divider |
| Temp Sense | PB14 | NTC thermistor |
| CAN RX | PA11 | FDCAN1 |
| CAN TX | PB9 | FDCAN1 |
| SWDIO | PA13 | Debug |
| SWCLK | PA14 | Debug |

## Building

```bash
# Install ARM toolchain first (see env_setup_scripts/)
make DM0001_G431
```

The output will be in `obj/AM32_DM0001_G431_x.xx.bin`

## Flashing

Use ST-Link or the TagConnect header on the board:

```bash
# Using OpenOCD
openocd -f interface/stlink.cfg -f target/stm32g4x.cfg -c "program obj/AM32_DM0001_G431_x.xx.bin 0x08000000 verify reset exit"

# Using STM32CubeProgrammer
STM32_Programmer_CLI -c port=SWD -w obj/AM32_DM0001_G431_x.xx.bin 0x08000000 -v -rst
```

## Configuration Notes

### Voltage Divider
- R_top = 169kΩ, R_bottom = 18kΩ
- Ratio = (169 + 18) / 18 = 10.39:1
- TARGET_VOLTAGE_DIVIDER = 104

### Current Sensing
- Shunt = 3mΩ
- Uses STM32G4 internal op-amps for amplification
- MILLIVOLT_PER_AMP = 3 (needs calibration)

### Dead Time
- DEAD_TIME = 80 (adjust based on gate driver characteristics)

## Known Issues / TODO

1. **BEMF Sensing**: The DM0001 uses a different BEMF topology than typical AM32 targets. 
   Only PA4 (Phase A) is directly connected to a comparator input. Phases B and C 
   (PB12, PB11) need ADC-based zero crossing detection.

2. **Input Timer**: Using TIM2_CH1 on PA15 instead of the typical TIM15. May need 
   adjustments in IO.c for DMA configuration.

3. **Current Sensing**: The board uses the STM32G4's internal op-amps. The gain and 
   offset need calibration for accurate current measurement.

## References

- [AM32 GitHub](https://github.com/AlkaMotors/AM32-MultiRotor-ESC-firmware)
- [STM32G431 Reference Manual](https://www.st.com/resource/en/reference_manual/rm0440-stm32g4-series-advanced-armbased-32bit-mcus-stmicroelectronics.pdf)
