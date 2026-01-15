# AM32-Embassy

Rust implementation of AM32 ESC firmware using the Embassy async framework for the DM0001 ZenDrive motor controller.

## Features

- **6-step BLDC commutation** with configurable timing advance
- **ADC-based BEMF sensing** for sensorless operation
- **DSHOT protocol** support (150/300/600)
- **PWM servo input** support
- **RTT logging** via defmt for debugging
- **Async architecture** using Embassy

## Hardware

- **MCU**: STM32G431C8T6 (Cortex-M4, 170MHz, 64KB Flash, 32KB RAM)
- **Gate Driver**: L6387ED (3x for 3-phase)
- **MOSFETs**: STL180N6F7 (6x - high/low side for each phase)

## Pin Mapping

### PWM Outputs (TIM1)
| Phase | High Side | Low Side |
|-------|-----------|----------|
| A     | PA8 (CH1) | PC13 (CH1N) |
| B     | PA9 (CH2) | PA12 (CH2N) |
| C     | PA10 (CH3) | PB15 (CH3N) |

### BEMF Sensing (ADC)
| Phase | Pin | ADC Channel |
|-------|-----|-------------|
| A     | PA4 | ADC2_IN17 |
| B     | PB12 | ADC1_IN11 |
| C     | PB11 | ADC1_IN14 |

### Other Signals
| Function | Pin |
|----------|-----|
| PWM Input | PA15 (TIM2_CH1) |
| VBUS Sense | PA0 (ADC1_IN1) |
| CAN RX | PA11 |
| CAN TX | PB9 |

## Building

### Prerequisites

```bash
# Install Rust and the thumbv7em target
rustup target add thumbv7em-none-eabihf

# Install probe-rs for flashing
cargo install probe-rs --features cli
```

### Build

```bash
cd boards/DM0001/bringup/am32-embassy
cargo build --release
```

### Flash

```bash
cargo run --release
# or
probe-rs run --chip STM32G431CBUx target/thumbv7em-none-eabihf/release/am32-embassy
```

## RTT Logging

Connect with probe-rs to view RTT logs:

```bash
probe-rs attach --chip STM32G431CBUx
```

Or use the `defmt-print` tool:

```bash
cargo install defmt-print
probe-rs run --chip STM32G431CBUx target/thumbv7em-none-eabihf/release/am32-embassy 2>&1 | defmt-print -e target/thumbv7em-none-eabihf/release/am32-embassy
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Embassy Executor                        │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │ Control  │  │  Input   │  │Telemetry │  │   ADC    │    │
│  │  Loop    │  │  Task    │  │  Task    │  │  Task    │    │
│  │ (20kHz)  │  │ (DSHOT)  │  │ (100Hz)  │  │ (1kHz)   │    │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘    │
│       │             │             │             │           │
├───────┼─────────────┼─────────────┼─────────────┼───────────┤
│       ▼             ▼             ▼             ▼           │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                  Motor Control Core                  │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐          │   │
│  │  │Commutator│  │   BEMF   │  │ Startup  │          │   │
│  │  │ (6-step) │  │  Sensor  │  │ Sequence │          │   │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘          │   │
│  └───────┼─────────────┼─────────────┼─────────────────┘   │
│          ▼             ▼             ▼                      │
├─────────────────────────────────────────────────────────────┤
│                    Hardware Abstraction                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │   TIM1   │  │   ADC    │  │   TIM2   │  │  Flash   │    │
│  │   PWM    │  │ Sensing  │  │  Input   │  │ Storage  │    │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Status

This is a work in progress. Current status:

- [x] Project structure
- [x] Embassy setup with RTT logging
- [x] 6-step commutation table
- [x] BEMF zero-crossing detection algorithm
- [x] DSHOT protocol decoder
- [x] Settings/EEPROM structure
- [ ] TIM1 PWM driver implementation
- [ ] ADC DMA sampling
- [ ] Input capture for DSHOT
- [ ] Full motor control integration
- [ ] CAN bus support
- [ ] Telemetry output

## License

MIT OR Apache-2.0
