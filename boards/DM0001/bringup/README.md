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

### BEMF Sensing (ADC-based)
| Phase | Pin | ADC Instance | ADC Channel |
|-------|-----|--------------|-------------|
| A     | PA4 | ADC2 | IN17 |
| B     | PB12 | ADC1 | IN11 |
| C     | PB11 | ADC1 | IN14 |

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

## ADC-Based BEMF Detection

The DM0001 uses ADC-based BEMF sensing instead of hardware comparators. This is because
the BEMF pins (PA4, PB12, PB11) are not all connected to comparator inputs on the 
STM32G431C8T6 (LQFP48 package).

### How It Works

1. **Voltage Divider Circuit**: Each phase has a voltage divider that scales the motor
   phase voltage down to ADC-safe levels (0-3.3V).

2. **Virtual Neutral Point**: The firmware calculates a virtual neutral point by 
   averaging all three phase voltages: `Vn = (Va + Vb + Vc) / 3`

3. **Zero-Crossing Detection**: The floating phase voltage is compared to the virtual
   neutral point. A zero-crossing is detected when the floating phase crosses the
   neutral point.

4. **Hysteresis**: A small hysteresis band prevents noise-induced false crossings.

### Implementation Files

- `Mcu/g431/Inc/comparator_adc.h` - ADC BEMF interface definitions
- `Mcu/g431/Src/comparator_adc.c` - ADC BEMF implementation
- `Inc/targets.h` - `USE_ADC_BEMF` define enables ADC mode

### Tuning Parameters

In `comparator_adc.c`:
- `BEMF_HYSTERESIS` - ADC counts for noise rejection (default: 20)

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

1. **ADC Sampling Timing**: The ADC BEMF sampling should ideally be synchronized with
   the PWM off-time for accurate readings. Current implementation uses polling which
   may need optimization for high-speed operation.

2. **Input Timer**: Using TIM2_CH1 on PA15 instead of the typical TIM15. May need 
   adjustments in IO.c for DMA configuration.

3. **Current Sensing**: The board uses the STM32G4's internal op-amps. The gain and 
   offset need calibration for accurate current measurement.

4. **Hall Sensor Mode**: Should work without modification as it doesn't rely on BEMF.

## References

- [AM32 GitHub](https://github.com/AlkaMotors/AM32-MultiRotor-ESC-firmware)
- [STM32G431 Reference Manual](https://www.st.com/resource/en/reference_manual/rm0440-stm32g4-series-advanced-armbased-32bit-mcus-stmicroelectronics.pdf)
