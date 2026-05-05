# Seward

USB-C CMSIS-DAP v2 debug probe. STM32G0B1-based.

Drop-in replacement for a CMSIS-DAP v2 probe for flashing and debugging
Cortex-M targets. Target I/O rails auto-range over 1.8–5.0 V via
VTref-driven level shifters, so one probe covers every common target
voltage without jumpers or firmware switches. Exposes a keyed 10-pin
ARM Cortex-Debug header with bidirectional SWD and target-sourced SWO
capture presented to the host as a CDC-ACM virtual serial port.

## Highlights

- USB-C bus-powered, crystal-less (HSI48 + CRS), no PD negotiation
- STM32G0B1KBU6N (Cortex-M0+, UFQFPN-32 'N' pinout, 128 KB flash) —
  UCPD with internal dead-battery Rd, no external CC pulldowns
- Composite USB: CMSIS-DAP v2 (WinUSB, MS OS 2.0) + CDC-ACM for SWO;
  enumerates out of the box in `pyOCD` / `probe-rs` / `OpenOCD`
- Samtec FTSH-105 10-pin keyed ARM Cortex-Debug header (SWD + SWO)
- 1.8 V / 3.3 V / 5 V target support via 3× `SN74LXC1T45` level
  shifters driven by target VTref; firmware-controlled `SWD_DIR`
- nRESET driven directly from an FT_c (5 V tolerant) GPIO — no FET
  shifter, no body-diode back-feed path
- No probe-side back-feed when USB is unplugged (LXC1T45 Ioff + V₀₀
  disconnect, FT_c nRESET, no VTref buffer)
- Self-reflashable over USB DFU via BOOT0 button + USB cycle;
  Tag-Connect TC2030 SWD pads (unpopulated) for factory / recovery
- Three GPIO-driven status LEDs (STATUS green / DAP amber / SWO blue),
  no power-rail LED
