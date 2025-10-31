# Bringup for DM0001 Motor Controller

The DM0001 is a 3-phase BLDC motor controller based on the STM32G431C8T6.

## Microcontroller Specifications

- **MCU:** STM32G431C8T6 (LQFP-48)
- **Flash:** 64 KB
- **RAM:** 32 KB (22KB SRAM1 + 6KB SRAM2 + 10KB CCM)
- **Debug Interface:** SWD via Tag-Connect TC2030-IDC-NL

## Firmware Flashing (SWD via probe-rs)

```bash
# Navigate to firmware directory
cd boards/DM0001/firmware

# Build the firmware
cargo build --release --bin blinky

# Flash using probe-rs
probe-rs download --chip STM32G431C8 target/thumbv7em-none-eabihf/release/blinky
```

## Version 0.0.1 - BOOT0 Issue

Version 0.0.1 of the board has PB8 (BOOT0) connected to the Hall encoder signal (Z_PLUS_H3_OUT). 

**Issue:** If the Hall encoder pulls PB8/BOOT0 high during power-on or reset, the STM32 will boot into the system bootloader (address 0x1FFF0000) instead of running the application firmware from flash (0x08000000).

**Workaround:** Force the device to boot from flash by setting the option bytes, which makes the MCU ignore the BOOT0 pin state. 

>[!CAUTION]
>It is INCREDIBLY IMPORTANT that the option bytes are written only AFTER flashing the firmware, otherwise the device will not be able to be flashed easily and will require careful reset timing or BOOT0 manipulation.

### Force Flash Boot Procedure

```bash
# Step 1: IMPORTANT - Flash the firmware FIRST
probe-rs download --chip STM32G431C8 target/thumbv7em-none-eabihf/release/blinky

# Step 2: Read current option bytes (backup)
probe-rs read --chip STM32G431C8 b32 0x1FFF7800 1

# Step 3: DANGER - Force boot from flash (ignores BOOT0 pin)
# This sets nBOOT_SEL=1 and nBOOT0=1 in FLASH_OPTR
probe-rs write --chip STM32G431C8 b32 0x1FFF7800 0xFBEFF8AA
```

### Restoring Default Boot Behavior

If you need to restore the default boot behavior (reading BOOT0 pin):

```bash
# Restore default option bytes (reads BOOT0 pin at boot)
probe-rs write --chip STM32G431C8 b32 0x1FFF7800 0xFFEFF8AA
```

## GDB Debugging

### Start GDB Server (Terminal 1)

```bash
cd boards/DM0001/firmware
probe-rs gdb --chip STM32G431C8
```

### Connect GDB Client (Terminal 2)

```bash
# From workspace root
arm-none-eabi-gdb \
  -ex "target extended-remote :1337" \
  -ex "load boards/DM0001/firmware/target/thumbv7em-none-eabihf/release/blinky" \
  -ex "monitor reset halt" \
  -ex "break main" \
  -ex "continue"
```

### Useful GDB Commands

```gdb
# Step through code
step                    # Step into
next                    # Step over
continue                # Continue execution

# Breakpoints
break main              # Break at main
break blinky.rs:20      # Break at line 20
info breakpoints        # List breakpoints
delete 1                # Delete breakpoint 1

# Inspect variables
info locals             # Show local variables
print status_led        # Print variable
backtrace              # Show call stack

# Control
monitor reset halt      # Reset and halt
monitor reset run       # Reset and run
kill                   # Stop debugging
quit                   # Exit GDB
```

## RTT Logging

The firmware uses defmt-rtt for real-time logging. View logs with:

```bash
# Option 1: Run and view RTT logs automatically
cargo run --release --bin blinky

# Option 2: Attach to running firmware
probe-rs attach --chip STM32G431C8
```

## Useful Tools

<details>
<summary>probe-rs</summary>

Software for interacting with the device over SWD (ST-Link, CMSIS-DAP, etc.)

```bash
# List connected probes
probe-rs list

# Flash firmware
probe-rs download --chip STM32G431C8 target/thumbv7em-none-eabihf/release/blinky

# Read memory
probe-rs read --chip STM32G431C8 b32 0x08000000 16    # Read 16 words from flash start
probe-rs read --chip STM32G431C8 b32 0x20000000 16    # Read 16 words from RAM start

# Read option bytes
probe-rs read --chip STM32G431C8 b32 0x1FFF7800 1

# Write option bytes
probe-rs write --chip STM32G431C8 b32 0x1FFF7800 0xFFEFF8AA

# Erase chip
probe-rs erase --chip STM32G431C8

# Reset chip
probe-rs reset --chip STM32G431C8
```

</details>

<details>
<summary>STM32CubeProgrammer</summary>

GUI tool for flashing and configuring STM32 devices.

Option bytes for forcing flash boot:
- **nBOOT_SEL:** 1 (ignore BOOT0 pin)
- **nBOOT0:** 1 (boot from flash)
- **FLASH_OPTR:** 0xFBEFF8AA

</details>

## Hardware Notes

### LEDs

- **Power LED (Green):** Connected to V3V3 via 1kΩ resistor - always on when powered
- **Status LED (Red):** Connected to PC15 via 1kΩ resistor - controlled by firmware

### Pin Assignments

Key pins used in blinky example:
- **PC15:** Status LED output
- **PA13:** SWDIO (debug)
- **PA14:** SWCLK (debug)
- **PB8:** BOOT0 (shared with Hall encoder Z signal - **causes boot issue**)

### Power Requirements

- **Input:** VPLUS (motor voltage, typically 12-48V)
- **Buck Converter:** V10V (10V rail)
- **LDO 1:** V5V (5V rail from 10V)
- **LDO 2:** V3V3 (3.3V rail from 5V)

MCU runs on 3.3V rail.

## Troubleshooting

### Chip boots to bootloader (0x1FFF0000) instead of application

**Symptom:** GDB shows PC at addresses like 0x1FFF44E4

**Cause:** PB8/BOOT0 is high at reset due to Hall encoder signal

**Solution:** Follow the "Force Flash Boot Procedure" above

### Load failed in GDB

**Symptom:** "Load failed" when trying to flash via GDB

**Solution:** 
1. Use `monitor flash erase` before `load`
2. Or use `probe-rs download` directly instead of GDB load

### LED not blinking

**Checks:**
1. Is power LED (green) on? If not, check power supply
2. Is firmware actually running? Check with GDB that PC is not in bootloader
3. Try the force flash boot procedure
4. Verify Hall encoder connector is not pulling BOOT0 high

### Cannot connect to probe

**Solution:**
```bash
# Check probe is detected
probe-rs list

# Try with explicit probe selection
probe-rs download --chip STM32G431C8 --probe 2e8a:000c:<SERIAL> firmware.elf
```

## Firmware Size

Current blinky firmware size:
- **Flash usage:** ~17.4 KB / 64 KB (27%)
- **RAM usage:** ~1.5 KB / 32 KB (5%)

Plenty of room for expansion!

