# DM0001 Bringup Firmware

## Overview

This firmware performs comprehensive testing of the DM0001 motor controller board peripherals and components. It's designed to verify board functionality during initial bringup and testing.

## What It Tests

### 1. **Status LED (PC15)**
- Blinks the red status LED at 1 Hz
- Verifies GPIO output functionality

### 2. **UART2 Communication (PB3=TX, PB4=RX)**
- Initializes UART at 115200 baud
- Sends test messages every 5 seconds
- Available on the PWM header connector

### 3. **ADC Measurements**
- **PA0 (VBUS)**: Voltage bus sensing with voltage divider compensation
- **PB14 (Temperature)**: Thermistor temperature feedback
- Reports raw ADC values and calculated voltages

### 4. **Hall Encoder Inputs (PB6, PB7, PB8)**
- Reads Hall sensor/encoder inputs
- Reports 3-bit Hall state pattern
- Detects and logs state changes

### 5. **GPIO Functionality**
- **PB10**: Test output (toggles every 10 seconds)
- **PB5**: BEMF control input monitoring

### 6. **FDCAN Control Pins**
- **PB13 (CAN_SHDN)**: Enables CAN transceiver (active low)
- **PC14 (CAN_TERM)**: Controls CAN termination resistor
- Notes availability of PA11 (RX) and PB9 (TX) for FDCAN peripheral

### 7. **Informational Checks**
Reports availability of additional peripherals:
- Motor control outputs (TIM1 6-channel complementary PWM)
- OpAmp current sensing (3 channels for 3-phase measurement)
- BEMF sensing inputs (PA4, PB11, PB12)
- PWM input capture (PA15)

## Building

```bash
cd boards/DM0001/bringup/v1.0/firmware
cargo build --bin bringup --release
```

## Flashing

Using probe-rs:
```bash
probe-rs run --chip STM32G431CBTx target/thumbv7em-none-eabihf/release/bringup
```

Or using the Justfile (if configured):
```bash
just flash-bringup
```

## Expected Output

The firmware will:
1. Run through all tests on startup
2. Report "All Bringup Tests Completed Successfully!" if all tests pass
3. Enter continuous monitoring loop with:
   - Status LED blinking at 1 Hz
   - Periodic UART messages every 5 seconds
   - ADC readings every 5 seconds
   - GPIO toggling every 10 seconds
   - CAN termination toggling every 20 seconds

## Monitoring Output

Connect via probe-rs or defmt-rtt to see detailed logging:
```bash
probe-rs run --chip STM32G431CBTx target/thumbv7em-none-eabihf/release/bringup
```

Example output:
```
===========================================
DM0001 Board Bringup Test Started
===========================================
[LED] Initializing Status LED (PC15)
[LED] Status LED test: OK
[UART] Initializing UART2 (PB3=TX, PB4=RX) at 115200 baud
[UART] UART2 initialized and test message sent: OK
[ADC] Initializing ADC1
[ADC] VBUS (PA0): raw=2048, estimated=12000mV
[ADC] Temperature (PB14): raw=1024, voltage=825mV
[ADC] ADC test: OK
...
===========================================
All Bringup Tests Completed Successfully!
===========================================
```

## Hardware Connections

### Required:
- Power supply connected to VPLUS (12-24V typical)
- SWD debugger connected via TagConnect

### Optional (for full testing):
- UART-to-USB adapter on PWM header (PB3/PB4)
- Hall sensors or encoder connected to PB6/PB7/PB8
- Motor phases for motor control testing (future)
- CAN bus connection for communication testing (future)

## Next Steps

After verifying bringup:
1. Test motor control functionality with PWM generation
2. Implement current sensing with OpAmps
3. Add FDCAN communication
4. Implement closed-loop motor control
5. Test BEMF sensing for sensorless control

## Troubleshooting

### LED doesn't blink
- Check power supply connection
- Verify LED assembly on PC15
- Check SWD connection for debugging

### No UART output
- Verify UART connections on PWM header
- Check baud rate (115200)
- Ensure proper ground connection

### ADC values seem wrong
- VBUS reading should reflect input voltage (scaled)
- Temperature reading depends on ambient temperature
- Check voltage divider components

### Hall encoder not working
- Verify encoder power supply (V5V)
- Check encoder connections to PB6/PB7/PB8
- Ensure proper pull-up/pull-down configuration

## Board Information

- **MCU**: STM32G431C8T6 (Cortex-M4F, 170 MHz)
- **Memory**: 128 KB Flash, 32 KB RAM
- **Peripherals**: FDCAN, USART, ADC, TIM1, OpAmps
- **Power**: 3.3V logic, 12-24V motor supply
- **Motor Control**: 3-phase BLDC with gate drivers

## License

Same as parent project.

