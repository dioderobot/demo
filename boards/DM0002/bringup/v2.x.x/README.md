# DM0002 Bringup v2.x.x

This directory contains pre-built firmware for the DM0002 board and provides commands to quickly flash it onto the RP2040.

## Quick Start

### 1. Hardware Setup

First, verify the board is powered correctly:

1. Connect the host PC to [USB_C](pcb://USB_C)
2. Verify 5V is present on [TP_VBUS](pcb://TP_VBUS)
3. Verify [LDO_3V3](pcb://LDO_3V3) is producing 3.3V on [TP_3V3](pcb://TP_3V3)
4. If you're probing a target, connect it to [SWD_HEADER](pcb://SWD_HEADER)

### 2. Enter UF2 Bootloader Mode

Put the RP2040 into UF2 bootloader mode so it appears as a USB mass-storage device:

1. Disconnect USB from [USB_C](pcb://USB_C)
2. Press and hold [MCU.SW_BOOTSEL](pcb://MCU.SW_BOOTSEL)
3. While holding the button, reconnect USB to [USB_C](pcb://USB_C)
4. Release the button after the board enumerates as a USB mass-storage device (typically `/Volumes/RPI-RP2` on macOS)

### 3. Flash the Firmware

Choose one of two methods:

**Method A: Drag and Drop (Easiest)**
- Open the `artifacts/` folder
- Drag `dm0002-debugprobe-v2.x.x.uf2` onto the mounted RPI-RP2 drive

**Method B: Command Line**
```bash
just burn
```

The board will reboot with the new firmware loaded.

## Status Indication

Once running, you can verify the board is working:

- [LED_PWR](pcb://LED_PWR) should light when 3.3V is present
- [LED_B](pcb://LED_B), [LED_G](pcb://LED_G), [LED_O1](pcb://LED_O1), [LED_O2](pcb://LED_O2), and [LED_O3](pcb://LED_O3) indicate firmware status

## Advanced

### Rebuild Firmware

To rebuild from source:

```bash
just build
just artifacts
```

This builds from [boards/DM0002/firmware](../../../firmware/) and updates the binaries in `artifacts/`.

### Custom Version Stamp

To rebuild with a custom version:

```bash
DM0002_FW_VERSION=custom just build
just artifacts
```

### Development Workflow

For repeated flashing during development without recompiling:

```bash
# Edit firmware, compile once
just build
just artifacts

# Then flash repeatedly without recompiling
just burn
just burn
# ... repeat as needed
```

For firmware-specific implementation details, see [boards/DM0002/firmware/README.md](../../../firmware/README.md).
