## DM001: ZenDrive

A 3-phase BLDC motor controller reference design built with Zen modules and KiCad assets. This repository doubles as a template for new hardware projects that follow a modular, reusable structure.

### Highlights
- STM32G431 MCU (QFN-48) with SWD via Tag-Connect (TC2030)
- CAN transceiver interface and termination
- 3 half-bridge phase drivers with PWM (TIM1 CH1/2/3 and CH1N/2N/3N)
- Hall encoder interface (A/B/Z)
- Shunt-based current sensing with op-amps and ADC nets
- Power rails: VPLUS → 10V buck → 5V LDO → 3V3 LDO

### Repository layout
- `ZenDrive.zen`: Top-level board composition
- `src/`: Reusable subsystems (PhaseDriver, ShuntSense, HallEncoder, PowerCaps, CAN)
- `components/`: Vendor parts with symbol/footprint/datasheet and `.zen`
- `reference/`: Stable wrappers around components for consistent interfaces
- `boards/`: Reference schematics/BOMs for known variants
