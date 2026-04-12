# DM0001 Bringup

This directory contains the staged bringup artifacts for the DM0001 ESC:

- `hardware_map.json`: machine-readable pin/sensor/driver map generated from `DM0001.zen`
- `tools/extract_hw_map.py`: reproducible map generator
- `logs/`: session logs and hardware interaction notes
- `firmware/`: Embassy-based bringup firmware
- `dashboard/`: host-side telemetry view

The workflow is intentionally gated:

1. Generate and review the hardware map.
2. Build telemetry-only firmware.
3. Power the board at low voltage/current and verify sensor telemetry.
4. Only then enable commutation experiments.
