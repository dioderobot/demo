# DM0001 Motor Controller Bringup

STM32G431C8T6-based 3-phase BLDC motor controller.

## Firmware

Three test programs available:

### blinky.rs
Basic LED blink test. Status LED on PC15.

```bash
just blink
```

### bringup.rs  
Comprehensive peripheral test:
- LED (PC15)
- UART2 (PB3/PB4, 115200 baud)
- ADC (PA0=VBUS, PB14=Temperature)
- Hall encoder inputs (PB6/PB7/PB8)
- GPIO (PB10/PB5)
- FDCAN control (PB13/PC14)

```bash
just bringup
```

### sine.rs
Generates 3-phase 120° sine waves on motor outputs:
- 20 kHz PWM, 5 Hz sine, 500ns deadtime, 80% max duty
- Phase A: PA8/PC13, Phase B: PA9/PA12, Phase C: PA10/PB15
- ⚠️ **Energizes motor phases** - disconnect motor or ensure safe operation

Tune parameters in code:
```rust
const PWM_FREQ_KHZ: u32 = 20;
const SINE_FREQ_HZ: u32 = 5;
const DEADTIME_NS: u16 = 500;
const MAX_DUTY_PERCENT: u8 = 80;
```

```bash
just sine
```

## Hardware Notes

**v0.0.1 BOOT0 Issue:** PB8 shared with Hall encoder can cause bootloader entry on reset.

Fix (after flashing firmware):
```bash
probe-rs write --chip STM32G431C8 b32 0x1FFF7800 0xFBEFF8AA
```

Restore default:
```bash
probe-rs write --chip STM32G431C8 b32 0x1FFF7800 0xFBEFF8AA
probe-rs erase --chip STM32G431C8
```

## Pin Map

**Motor Control:**
- Phase A: PA8 (CH1), PC13 (CH1N)
- Phase B: PA9 (CH2), PA12 (CH2N)  
- Phase C: PA10 (CH3), PB15 (CH3N)

**Current Sensing (OpAmp outputs):**
- PA2, PA6, PB1

**Hall/Encoder:**
- PB6 (A), PB7 (B), PB8 (Z/BOOT0)

**BEMF:**
- PA4, PB11, PB12

**Communication:**
- UART2: PB3 (TX), PB4 (RX)
- FDCAN: PA11 (RX), PB9 (TX)
- SWD: PA13/PA14

**ADC:**
- PA0 (VBUS), PB14 (Temperature)

**LEDs:**
- PC15 (Status, red), V3V3 (Power, green)
