# WV0001 Firmware

Comprehensive firmware for the WV0001 Weave Power Distribution Board (STM32G431CBU6) with Embassy async framework.

## Quick Start

```bash
# Build and flash comprehensive test suite
cargo run --bin bringup --release

# View debug output (separate terminal)
probe-rs rtt attach
```

## Hardware Overview

- **Power**: 6 high-power relays (A41AS12VDC), 12V load switch, 48V→12V→3.3V conversion
- **Sensing**: Battery/charger voltage (80V max), 3x NTC temperature, INA228 current sensor
- **Communication**: CAN (TCAN3404), I2C (BME280), SPI/UART connectors
- **Safety**: ESTOP with hardware AND gates, comprehensive diagnostics

## Firmware Features

### Bringup Test Suite (`bringup.rs`)

Comprehensive electrical validation covering:

- **Temperature**: Real Celsius calculations from NTC thermistors
- **Voltage**: Battery/charger sensing with range validation
- **Current**: INA228 monitoring with relay switching detection
- **Environmental**: BME280 sensor readings
- **Communication**: CAN, I2C, SPI, UART interface testing
- **Safety**: ESTOP and relay control verification

### Key Capabilities

- **Real-time diagnostics** with RTT debug output
- **Hardware issue detection** (e.g., ADC pin compatibility)
- **Current monitoring** during relay operations
- **Voltage range validation** for 48V systems
- **Temperature calculation** using Steinhart-Hart equation

## Critical Hardware Issue ⚠️

**NTC3 ADC Incompatibility**: NTC3 is connected to PB3, which is **NOT ADC-capable** on STM32G431.

- **Impact**: Temperature sensor 3 cannot be read
- **Workaround**: Firmware skips NTC3 with clear error messages
- **Fix Required**: Board rework to connect NTC3 to ADC-capable pin (PA0, PA1, PA4, etc.)

## Pin Mapping (Key Signals)


| Function          | Pin       | Notes                          |
| ------------------- | ----------- | -------------------------------- |
| Output Relays 1-4 | PB4-7     | Active high enables            |
| Charger Relay     | PB9       | Through ESTOP AND gate         |
| Robot Relay       | PB10      | Through ESTOP AND gate         |
| Load Switch 12V   | PB11      | Active high enable             |
| Battery Voltage   | PB12      | 25:1 divider (80V max)         |
| Charger Voltage   | PB14      | 25:1 divider (80V max)         |
| NTC1, NTC2        | PB0, PB1  | 10kΩ @ 25°C, 4.7kΩ pulldown |
| NTC3              | PB3       | ⚠️**NOT ADC-capable**        |
| Current Alert     | PA10      | INA228 active-low alert        |
| I2C               | PA8/PA9   | BME280, INA228 (0x41)          |
| CAN               | PA11/PA12 | TCAN3404 transceiver           |
| Debug LED         | PB13      | Blue LED, active high          |

## Testing Checklist

### Power System

- [ ] Debug LED heartbeat active
- [ ] Battery voltage 36-58V (48V system)
- [ ] Charger voltage 48-65V when connected
- [ ] Current readings during relay switching

### Sensors

- [ ] NTC1/NTC2 temperature readings (°C)
- [ ] BME280 environmental data
- [ ] INA228 current/voltage measurements
- [ ] All I2C devices detected

### Control

- [ ] All 6 relays activate with current change
- [ ] ESTOP safety system functional
- [ ] Load switch operation verified
- [ ] CAN communication working

## Safety & Usage Notes

⚠️ **High-Power System**: 48V/30A capable

- **ESTOP Required**: Short ESTOP connector to 3.3V for relay operation
- **Current Monitoring**: Use INA228 readings to verify relay switching
- **Temperature Limits**: Monitor NTC readings during operation
- **Hardware Issue**: NTC3 requires board rework for functionality

## Build Requirements

```bash
# Install Rust toolchain
rustup target add thumbv7em-none-eabi
cargo install probe-rs-tools

# Build and flash
cargo build --bin bringup --release
cargo run --bin bringup --release
```

Debug output via RTT only (USB pins used for CAN).
