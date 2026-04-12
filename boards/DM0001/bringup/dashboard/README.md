# DM0001 Dashboard

Real-time host-side dashboard for the DM0001 bringup firmware.

It consumes RTT telemetry from the flashed `dm0001-bringup` ELF. DPS-150 polling is optional so the default dashboard path stays fast and reliable.

## Usage

Attach to an already-running bringup image:

```bash
python3 boards/DM0001/bringup/dashboard/rtt_dashboard.py
```

Flash/reset the board before opening the dashboard:

```bash
python3 boards/DM0001/bringup/dashboard/rtt_dashboard.py --mode run
```

Plain-text mode for testing or logging:

```bash
python3 boards/DM0001/bringup/dashboard/rtt_dashboard.py --plain --duration 5
```

Enable DPS polling when live supply voltage/current/power is needed:

```bash
python3 boards/DM0001/bringup/dashboard/rtt_dashboard.py --psu
```

## What It Shows

- board state: `booting`, `calibrating`, `disarmed`, `armed`
- `arm_ready`
- bus voltage, `3V3` estimate, NTC, MCU temperature
- per-phase current and raw opamp output voltage
- BEMF sense voltages
- hall inputs and shared BEMF GPIO state
- DPS-150 output voltage/current/power when `--psu` is enabled and available

The current firmware is still sensor-only, so duty, RPM, and commutation are shown as unavailable until motor-control firmware is added.
