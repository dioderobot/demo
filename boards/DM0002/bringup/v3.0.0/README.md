# DM0002 Bringup v3.0.0

This bringup bundle captures the `v3.0.0` firmware artifacts for `DM0002` and exposes a local `justfile` entry point to rebuild or flash them.

The reusable firmware source now lives in [boards/DM0002/firmware](/Users/davide/src/diodeinc/customers/demo/boards/DM0002/firmware#L1). Versioned binaries for this bringup live in [boards/DM0002/bringup/v3.0.0/artifacts](/Users/davide/src/diodeinc/customers/demo/boards/DM0002/bringup/v3.0.0/artifacts#L1).

## Bringup

1. Connect the host PC to [USB_C](pcb://USB_C).
2. Verify 5V is present on [TP_VBUS](pcb://TP_VBUS).
3. Verify the onboard regulator [LDO_3V3](pcb://LDO_3V3) is producing 3.3V on [TP_3V3](pcb://TP_3V3).
4. If you are probing a target, connect it to [SWD_HEADER](pcb://SWD_HEADER).

From this directory, run:

```bash
just build
just artifacts
```

That builds from `boards/DM0002/firmware` and refreshes the checked-in outputs in `artifacts/`.

## UF2 Update Mode

To force the RP2040 into UF2 bootloader mode:

1. Disconnect USB from [USB_C](pcb://USB_C).
2. Press and hold [MCU.SW_BOOTSEL](pcb://MCU.SW_BOOTSEL).
3. While holding the button, reconnect USB to [USB_C](pcb://USB_C).
4. Release the button after the board enumerates as a USB mass-storage device.

Then either copy `artifacts/dm0002-debugprobe-v3.0.0.uf2` to the mounted UF2 drive manually, or run:

```bash
just flash
```

`just flash` defaults to `/Volumes/RPI-RP2`. To use a different mount point:

```bash
just flash /path/to/mount
```

Status indication during bringup:

- [LED_PWR](pcb://LED_PWR) should light when 3.3V is present.
- [LED_B](pcb://LED_B), [LED_G](pcb://LED_G), [LED_O1](pcb://LED_O1), [LED_O2](pcb://LED_O2), and [LED_O3](pcb://LED_O3) are firmware-controlled status LEDs.

If you want to rebuild from scratch first, run:

```bash
just clean
just build
just artifacts
```

For firmware-specific details, see [boards/DM0002/firmware/README.md](/Users/davide/src/diodeinc/customers/demo/boards/DM0002/firmware/README.md#L1).
