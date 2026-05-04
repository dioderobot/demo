# Renfield

USB-C PD sink dev board for STM32G0B1 UCPD firmware development.

The MCU runs the full PD sink stack itself — TCPP01-M12 is a
*protection* chip, not a PD controller. Plug into any USB-PD source,
negotiate a contract up to 20 V / 5 A, and pass the rail through a
firmware-gated OVP FET to a WAGO screw terminal.

## Highlights

- STM32G0B1KBU6N (Cortex-M0+, UFQFPN-32 'N' pinout, 128 KB flash)
- TCPP01-M12 + CSD17318Q2: hardware VBUS OVP @ 22 V, CC OVP @ 6 V,
  IEC 61000-4-2 L4 ESD on CC, connector-side dead-battery Rd
- Firmware-gated load rail (SSM3K15ACT shunt FET, default-off at
  reset) — firmware drives PA4 low to enable after PD contract
- Crystalless USB (HSI48 + CRS), USB-C data + CDC-ACM
- TPS70933 LDO from VBUS_RAW (always hot) → MCU survives OVP events
- WAGO 2060-452 push-in terminal for the negotiated rail
- Round Ø1.0 mm test pads: CC1/CC2 (connector-side, raw BMC), UART,
  scope trigger, OVP divider tap, GND
- Tag-Connect TC2030 SWD pads (no header populated — Cortex-M0+
  has no SWO)
- Four indicator LEDs (fault / contract / enum / heartbeat)
