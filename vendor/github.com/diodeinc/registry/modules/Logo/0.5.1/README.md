# Logo Module

Adds decorative logos to PCB silkscreen.

## Usage

```starlark
Logo = Module("github.com/diodeinc/registry/modules/Logo/Logo.zen")

Logo(
    name = "LOGO",
    logo = Logo("Diode"),  # Currently only "Diode" supported
)
```

## Available Logos

- **Diode**: Diode company logo

## Notes

- Logos are DNP (Do Not Place), skip BOM, and skip position file
- Logos appear only on PCB silkscreen
- Add additional logos by placing `.kicad_mod` files in this directory and updating the enum
