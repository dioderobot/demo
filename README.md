# Diode PCB boards

This repository contains schematics and boards built using the `pcb` command line utility by Diode, using the Zener language and open source compiler

## Prerequisites

For the most up to date information, visit our documentation at [`docs.pcb.new`](https://docs.pcb.new)

The open source PCB compiler is available at [`github.com/diodeinc/pcb`](https://github.com/diodeinc/pcb)

1. **KiCad 9.0**
   - Required for PCB layout
   - Download from [KiCad website](https://kicad.org/download)

## Project Structure

```
.
├── README.md
├── components              # Auto-generated component definitions (do not edit)
├── modules                 # User-authored reusable modules (.zen files)
├── boards                  # Board designs that use modules (.zen files)
└── pcb.toml               # Project configuration
```

## Usage Guidelines

### Building Projects
- Build your project using:
  ```bash
  pcb build
  ```

## Important Rules

1. **Dependencies**
   - Always use `pcb add` for dependencies
   - Never modify pcb.toml directly
   - Only add dependencies to directories with a `pcb.toml` file

2. **Best Practices**
   - Components in `components/` are auto-generated - never edit them manually
   - Create reusable modules in `modules/` directory
   - Board designs go in `boards/` directory
   - Use generic components for basic elements (capacitors, resistors, inductors)
   - Consult component datasheets when available

3. **Project Files**
   - Build artifacts are stored in `.pcb/` directory
   - Project configuration is in `pcb.toml`
   - Component files use `.zen` extension

## Zener Language Reference

Zener is a Starlark-based language for defining electronic components, modules, and boards as code. Starlark is a simplified dialect of Python designed for configuration files.

### Key Concepts

1. **Components** (in `components/` directory)
   - Auto-generated using `pcb search <PART_NUMBER>`
   - Never edit these manually
   - Stored as `.zen` files

2. **Modules** (in `modules/` directory)
   - User-authored reusable logic
   - Combine components and define connections
   - Named as `<module_name>.zen`

3. **Boards** (in `boards/` directory)
   - Instantiate modules for real-world use
   - Define the final PCB design
   - Named as `<board_name>.zen`

### Basic Syntax

Since Zener is based on Starlark, it follows Python-like syntax:
- Variables and functions follow Python conventions
- Comments use `#`
- Indentation matters
- No type annotations required

For detailed syntax and examples, refer to the [Diode PCB documentation](https://github.com/diodeinc/pcb)

## Component Management

Components are managed through the `pcb` command line tool:
- Use `pcb pro search <PART_NUMBER>` to find components
- Components are automatically downloaded and stored in `components/`
- Each component includes symbol, footprint, 3D model, and datasheet information

## Contributing

When contributing to this repository, please:
1. Follow the project structure
2. Use the provided tools for component and module creation
3. Ensure all components are properly documented
4. Test your changes with `pcb build`
