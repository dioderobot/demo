# DM0002 Debug Probe Firmware

Board-local firmware for [`boards/DM0002`](../../../), derived from [`diodeinc/debugprobe`](https://github.com/diodeinc/debugprobe) at commit `8335b35c9cf264f13b268ef51850dd433fa64174`.

This copy is trimmed into the board bringup tree so `DM0002` can be built and iterated without depending on a separate firmware checkout.

## DM0002 pin map

- SWCLK: GPIO19
- SWDIO: GPIO20
- nRESET: GPIO18 (open-drain behavior in firmware)
- Target UART RX: GPIO28
- Target UART TX: GPIO29
- USB connected LED: GPIO17
- DAP connected LED: GPIO16
- DAP running LED: GPIO15
- UART RX LED: GPIO14
- UART TX LED: GPIO12
- VBUS detect: GPIO5

## Building

From [`boards/DM0002/bringup/v3.0.0`](../):

```bash
just build
just uf2
```

The firmware uses `pico_sdk_import.cmake`, so if `PICO_SDK_PATH` is unset CMake will fetch a compatible Pico SDK automatically.

## Notes

- The USB descriptors are override-capable in this local copy. `DM0002` currently keeps the upstream default VID/PID until board-specific IDs are assigned.
- `TARGET_VDDIO` sensing and target ground detect are present in the hardware but are not yet consumed by the firmware.
- The upstream Pico build path is still available with `-DDEBUG_ON_PICO=ON`.

## License

This directory contains code from the upstream `debugprobe` project and retains its original licensing. See [`LICENSE`](./LICENSE) and the copied dependency trees for details.
