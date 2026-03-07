# Logo Module

Adds decorative logos to PCB silkscreen.

## Usage

```starlark
Logo = Module("//modules/Logo/Logo.zen")

Logo(
    name = "LOGO",
    config_logo = Logo("Diode"),  # Currently only "Diode" supported
)
```

## Available Logos

- **Diode**: Diode company logo

## Notes

- Logos are DNP (Do Not Place), skip BOM, and skip position file
- Logos appear only on PCB silkscreen
- Add additional logos by placing `.kicad_mod` files in the `eda/` directory and updating the enum

