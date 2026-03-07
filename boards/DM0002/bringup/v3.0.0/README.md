# DM0002 Bringup v3.0.0

This bringup bundle vendors the `debugprobe` firmware alongside `DM0002` and exposes a local `justfile` entry point.

## Bringup

Connect the host PC to the board's USB-C receptacle `USB_C`.

- Schematic reference: [boards/DM0002/DM0002.zen](/Users/davide/src/diodeinc/customers/demo/boards/DM0002/DM0002.zen#L131)
- PCB reference: [boards/DM0002/layout/layout.kicad_pcb](/Users/davide/src/diodeinc/customers/demo/boards/DM0002/layout/layout.kicad_pcb)
- Board overview: [boards/DM0002/docs/DM0002.md](/Users/davide/src/diodeinc/customers/demo/boards/DM0002/docs/DM0002.md#L1)

From this directory, run:

```bash
just build
just uf2
```

That produces `firmware/build/dm0002-debugprobe.uf2`.

If you want to rebuild from scratch first, run:

```bash
just clean
just build
just uf2
```

For firmware-specific details, see [boards/DM0002/bringup/v3.0.0/firmware/README.md](/Users/davide/src/diodeinc/customers/demo/boards/DM0002/bringup/v3.0.0/firmware/README.md#L1).
