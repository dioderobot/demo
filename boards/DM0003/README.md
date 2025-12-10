# DM0003 - Raspberry Pi CM5 Carrier Board

## Overview
DM0003 is a comprehensive carrier board for the Raspberry Pi Compute Module 5, offering extensive connectivity and expansion options.

## Features
- Raspberry Pi Compute Module 5 support
- Dual display outputs (HDMI 2.0 + MIPI DSI)
- PCIe Gen 3.0 x1 M.2 slot
- USB 3.0 Type-C with Power Delivery
- Gigabit Ethernet
- MIPI CSI-2 camera interface
- 40-pin GPIO header
- Integrated LIS3DH accelerometer
- Power management and user controls

## Board Structure
- `DM0003.zen` - Main board definition
- `src/` - Module source files
  - `CM5.zen` - CM5 interface module
  - `Ethernet.zen` - Gigabit Ethernet module
  - `HDMI.zen` - HDMI output module
  - `USB_PI.zen` - USB 2.0 interface
  - `USB_3PI.zen` - USB 3.0 interface
  - `PCIe-M2.zen` - M.2 PCIe slot module
  - `DSI_CSI.zen` - Display and camera interfaces
  - `IO.zen` - GPIO and control interfaces
- `layout/DM0003/` - KiCad PCB layout files
- `docs/` - Documentation

## Note
This board was imported from the ZenPi5 project. Some component dependencies may need to be resolved by:
1. Adding missing components using `pcb search <component_name>`
2. Copying components from the original ZenPi5 repository
3. Finding equivalent components in the registry

## Documentation
See `docs/DM0003.md` for detailed specifications and block diagram.










