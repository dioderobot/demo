# Feign

USB-C to 3.3 V UART bridge. STM32G0B1-based.

Enumerates over USB as a standard CDC-ACM virtual serial port. An
optional firmware build emulates the FTDI UART-mode protocol for
`pyftdi` / `libftdi` compatibility.

## Highlights

- USB-C bus-powered, crystal-less (HSI48 + CRS), no PD negotiation
- STM32G0B1 (Cortex-M0+, UFQFPN-32 'N' pinout) — UCPD with internal
  dead-battery Rd, no external CC pulldowns required
- 1×6 SMD right-angle header, SparkFun FTDI Basic pinout:
  `GND · CTS · VCC · TXD · RXD · DTR`
- 5 V VCC pin to target, current-limited to ~400 mA with auto-retry
  (TPS2553) and firmware enable + fault flag
- 3.3 V CMOS UART signals; RX and CTS routed to FT_c pins so they
  tolerate 5 V from an externally-powered target while Feign is unpowered
- BOOT0 + RESET tactile buttons, Tag-Connect TC2030 SWD pads
- Three indicator LEDs (power / TX / RX), brightness-equalised
