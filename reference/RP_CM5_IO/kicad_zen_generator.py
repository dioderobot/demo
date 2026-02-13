#!/usr/bin/env python3
"""
Parse KiCad symbol library file and generate a complete Zen module file
with proper pin mappings for CM5_IO carrier board.
"""

import re
import sys
from pathlib import Path

def parse_kicad_pins(file_path):
    """
    Parse a KiCad symbol library file and extract pin information.
    
    Args:
        file_path: Path to the KiCad symbol library file
    
    Returns:
        Dictionary mapping symbol names to lists of pin data
    """
    symbols = {}
    current_symbol = None
    
    with open(file_path, 'r') as f:
        lines = f.readlines()
    
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        
        # Check for symbol definition
        if line.startswith('(symbol "'):
            match = re.search(r'\(symbol "([^"]+)"', line)
            if match:
                current_symbol = match.group(1)
                if current_symbol not in symbols:
                    symbols[current_symbol] = []
        
        # Check for pin definition
        elif line.startswith('(pin ') and current_symbol:
            pin_data = {'line': line}
            
            # Extract pin name and number
            j = i + 1
            while j < len(lines):
                name_line = lines[j].strip()
                if name_line.startswith('(name "'):
                    match = re.search(r'\(name "([^"]+)"', name_line)
                    if match:
                        pin_data['name'] = match.group(1)
                elif name_line.startswith('(number "'):
                    match = re.search(r'\(number "([^"]+)"', name_line)
                    if match:
                        pin_data['number'] = match.group(1)
                    break
                j += 1
            
            if 'name' in pin_data and 'number' in pin_data:
                symbols[current_symbol].append(pin_data)
        
        i += 1
    
    return symbols

def clean_pin_name(name, for_net_name=False):
    """
    Clean pin name for use as variable name or net name while preserving KiCad capitalization
    and handling special characters intelligently.
    
    Args:
        name: Original pin name
        for_net_name: If True, clean for use as a net name (no special chars at all)
        
    Returns:
        Cleaned variable/net name
    """
    # Handle special power naming conventions
    if name.startswith('+'):
        # +5v_(Input) -> 5V_Input
        # +3.3v_(Output) -> 3V3_Output
        # +1.8v_(Output) -> 1V8_Output
        var_name = name[1:]  # Remove the +
        
        # Clean up voltage values
        if '1.8v' in var_name:
            var_name = var_name.replace('1.8v', '1V8')
        elif '3.3v' in var_name:
            var_name = var_name.replace('3.3v', '3V3')
        elif '5v' in var_name:
            var_name = var_name.replace('5v', '5V')
        
        # Remove parentheses and underscores
        var_name = var_name.replace('_(', '_').replace(')', '')
        var_name = var_name.replace('(', '_').replace(')', '')
        
    elif '(3.3v)' in name or '(1.8v)' in name:
        # For things like Ethernet_nLED3(3.3v) -> Ethernet_nLED3_3V3
        var_name = name.replace('(3.3v)', '_3V3').replace('(1.8v)', '_1V8')
        
    elif '(' in name and ')' in name:
        if for_net_name:
            # For net names, replace parenthetical content with cleaned version
            # GPIO_VREF(1.8v/3.3v_Input) -> GPIO_VREF_1V8_3V3_Input
            base = name.split('(')[0].rstrip('_')
            paren_content = name[name.index('(')+1:name.index(')')]
            
            # Clean the parenthetical content
            paren_content = paren_content.replace('1.8v', '1V8').replace('3.3v', '3V3')
            paren_content = paren_content.replace('/', '_').replace('.', '_')
            
            var_name = f"{base}_{paren_content}"
        else:
            # For variable names, just take the part before parentheses
            var_name = name.split('(')[0].rstrip('_')
        
    else:
        var_name = name
    
    # Handle negation prefix ~{} 
    var_name = var_name.replace('~{', 'n').replace('}', '')
    
    # Replace special characters
    var_name = var_name.replace('-', '_')
    var_name = var_name.replace('/', '_')
    var_name = var_name.replace('.', '_')
    var_name = var_name.replace(' ', '_')
    var_name = var_name.replace('+', 'p')
    var_name = var_name.replace('(', '_').replace(')', '')
    
    # Clean up any non-alphanumeric characters
    var_name = re.sub(r'[^a-zA-Z0-9_]', '_', var_name)
    
    # Remove consecutive underscores
    var_name = re.sub(r'_+', '_', var_name)
    
    # Remove leading/trailing underscores
    var_name = var_name.strip('_')
    
    # Make sure variable name doesn't start with a number (but preserve power rail names)
    if var_name and var_name[0].isdigit() and not (var_name.startswith('1V8') or var_name.startswith('3V3') or var_name.startswith('5V')):
        var_name = 'pin_' + var_name
    
    return var_name

def generate_zen_file(symbols, output_file="CM5_IO_generated.zen"):
    """
    Generate a complete Zen module file with proper pin mappings.
    
    Args:
        symbols: Dictionary of symbol data from KiCad file
        output_file: Output filename
    """
    
    # Debug: Print available symbols
    print("Available symbols:")
    for symbol_name, pins in symbols.items():
        print(f"  {symbol_name}: {len(pins)} pins")
    
    # Find the GPIO and HSS symbols
    gpio_pins = []
    hss_pins = []
    
    for symbol_name, pins in symbols.items():
        if "GPIO" in symbol_name:
            gpio_pins = pins
            print(f"Found GPIO symbol with {len(pins)} pins")
        elif "HSS" in symbol_name:
            hss_pins = pins
            print(f"Found HSS symbol with {len(pins)} pins")
    
    # Combine all pins
    all_pins = []
    all_pins.extend(gpio_pins)
    all_pins.extend(hss_pins)
    
    if not all_pins:
        print("Error: Could not find GPIO or HSS symbols with pins")
        return
    
    print(f"Total pins found: {len(all_pins)}")
    
    # Sort pins by number
    sorted_pins = sorted(all_pins, key=lambda x: int(x['number']))
    
    # Generate the Zen file content
    content = []
    
    # Header
    content.append('"""')
    content.append('CM5IO - CM5 Carrier IO (Generated)')
    content.append('Author: Generated from KiCad Symbol')
    content.append('This module maps the two 100-pin high-density connectors')
    content.append('"""')
    content.append('')
    
    # Libraries & Dependencies
    content.append('# ' + '-' * 76)
    content.append('# Libraries & Dependencies')
    content.append('# ' + '-' * 76)
    content.append('')
    content.append('load("@stdlib/units.zen", "Voltage", "Current", "Capacitance", "Inductance", "Frequency", "Resistance")')
    content.append('load("@stdlib/config.zen", "config_unit", "config_properties")')
    content.append('load("@stdlib/interfaces.zen", "Power", "Ground")')
    content.append('load("@stdlib/properties.zen", "Layout")')
    content.append('')
    
    # Component Modules
    content.append('# ' + '-' * 76)
    content.append('# Component Modules')
    content.append('# ' + '-' * 76)
    content.append('')
    content.append('CM5IO_1 = Module("//components/10164227M1001A1RLF/10164227M1001A1RLF.zen")')
    content.append('CM5IO_2 = Module("//components/10164227M1001A1RLF/10164227M1001A1RLF.zen")')
    content.append('')
    
    # IO Interfaces
    content.append('# ' + '-' * 76)
    content.append('# IO Interfaces')
    content.append('# ' + '-' * 76)
    content.append('')
    
    # Create a dictionary to store unique IO declarations and their variable names
    unique_ios = {}  # Maps pin name to (variable_name, net_name)
    pin_to_var = {}  # Maps pin number to variable name
    
    # First pass: identify unique pin names and create IO declarations
    for pin in sorted_pins:
        pin_name = pin.get('name', '')
        pin_num = pin.get('number', '')
        
        if pin_name not in unique_ios:
            # Create clean names for both variable and net
            var_name = f"io_{clean_pin_name(pin_name, for_net_name=False)}"
            net_name = clean_pin_name(pin_name, for_net_name=True)
            unique_ios[pin_name] = (var_name, net_name)
        
        # Map this pin number to the variable
        pin_to_var[pin_num] = unique_ios[pin_name][0]
    
    # Group related IOs together for better organization
    power_ios = []
    gpio_ios = []
    ethernet_ios = []
    hdmi_ios = []
    usb_ios = []
    pcie_ios = []
    mipi_ios = []
    sd_ios = []
    cam_ios = []
    i2c_spi_ios = []
    other_ios = []
    
    for pin_name, (var_name, net_name) in sorted(unique_ios.items()):
        if 'GND' in pin_name:
            power_ios.append((pin_name, var_name, net_name, 'gnd'))
        elif pin_name.startswith('+'):
            power_ios.append((pin_name, var_name, net_name, 'power'))
        elif 'GPIO_VREF' in pin_name or 'VBAT' in pin_name:
            power_ios.append((pin_name, var_name, net_name, 'power'))
        elif pin_name.startswith('GPIO') and 'CAM' not in pin_name and 'VREF' not in pin_name:
            gpio_ios.append((pin_name, var_name, net_name))
        elif 'Ethernet' in pin_name:
            ethernet_ios.append((pin_name, var_name, net_name))
        elif 'HDMI' in pin_name:
            hdmi_ios.append((pin_name, var_name, net_name))
        elif 'USB' in pin_name:
            usb_ios.append((pin_name, var_name, net_name))
        elif 'PCIe' in pin_name or 'PCIE' in pin_name:
            pcie_ios.append((pin_name, var_name, net_name))
        elif 'MIPI' in pin_name:
            mipi_ios.append((pin_name, var_name, net_name))
        elif pin_name.startswith('SD_'):
            sd_ios.append((pin_name, var_name, net_name))
        elif 'CAM' in pin_name:
            cam_ios.append((pin_name, var_name, net_name))
        elif any(x in pin_name for x in ['SCL', 'SDA', 'SPI', 'I2C']):
            i2c_spi_ios.append((pin_name, var_name, net_name))
        else:
            other_ios.append((pin_name, var_name, net_name))
    
    # Write grouped IO declarations
    if power_ios:
        content.append('# Power and Ground')
        # Single GND net for all ground pins
        content.append('io_GND = io("GND", Ground)')
        for pin_name, var_name, net_name, *ptype in power_ios:
            if 'GND' not in pin_name:
                # Handle different power rails
                if '+5v' in pin_name:
                    content.append(f'{var_name} = io("{net_name}", Net, default = Net("5v", symbol = Symbol("@kicad-symbols/power.kicad_sym:+5V")))')
                elif '+3.3v' in pin_name:
                    content.append(f'{var_name} = io("{net_name}", Net, default = Net("3v3", symbol = Symbol("@kicad-symbols/power.kicad_sym:+3.3V")))')
                elif '+1.8v' in pin_name:
                    content.append(f'{var_name} = io("{net_name}", Net, default = Net("1v8", symbol = Symbol("@kicad-symbols/power.kicad_sym:+1V8")))')
                else:
                    content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if gpio_ios:
        content.append('# GPIO Pins')
        # Sort GPIO pins numerically
        gpio_sorted = sorted(gpio_ios, key=lambda x: (int(re.search(r'\d+', x[0]).group()) if re.search(r'\d+', x[0]) else 999, x[0]))
        for pin_name, var_name, net_name in gpio_sorted:
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if ethernet_ios:
        content.append('# Ethernet Interface')
        for pin_name, var_name, net_name in sorted(ethernet_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if hdmi_ios:
        content.append('# HDMI Interfaces')
        for pin_name, var_name, net_name in sorted(hdmi_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if usb_ios:
        content.append('# USB Interfaces')
        for pin_name, var_name, net_name in sorted(usb_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if pcie_ios:
        content.append('# PCIe Interface')
        for pin_name, var_name, net_name in sorted(pcie_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if mipi_ios:
        content.append('# MIPI Camera/Display Interfaces')
        for pin_name, var_name, net_name in sorted(mipi_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if sd_ios:
        content.append('# SD Card Interface')
        for pin_name, var_name, net_name in sorted(sd_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if cam_ios:
        content.append('# Camera Control')
        for pin_name, var_name, net_name in sorted(cam_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if i2c_spi_ios:
        content.append('# I2C/SPI Interfaces')
        for pin_name, var_name, net_name in sorted(i2c_spi_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    if other_ios:
        content.append('# Other Interfaces')
        for pin_name, var_name, net_name in sorted(other_ios):
            content.append(f'{var_name} = io("{net_name}", Net)')
        content.append('')
    
    # Update pin_to_var to use io_GND for all GND pins
    for pin_num, var_name in pin_to_var.items():
        pin = next((p for p in sorted_pins if p['number'] == pin_num), None)
        if pin and 'GND' in pin.get('name', ''):
            pin_to_var[pin_num] = 'io_GND'
    
    # Component instantiation
    content.append('# ' + '-' * 76)
    content.append('# Component Instantiation')
    content.append('# ' + '-' * 76)
    content.append('')
    
    # Generate CM5IO_1 instantiation (pins 1-100)
    content.append('# Connector 1: GPIO and primary interfaces (Pins 1-100)')
    content.append('CM5IO_1(')
    content.append('    name = "J1_CM5",')
    
    pins_1_100 = [pin for pin in sorted_pins if int(pin['number']) <= 100]
    for i, pin in enumerate(pins_1_100):
        pin_num = pin.get('number', '')
        var_name = pin_to_var[pin_num]
        comma = "," if i < len(pins_1_100) - 1 else ""
        comment = f"  # {pin.get('name', '')}" if pin.get('name', '') else ""
        content.append(f'    P{pin_num} = {var_name}{comma}{comment}')
    
    content.append(')')
    content.append('')
    
    # Generate CM5IO_2 instantiation (pins 101-200)
    pins_101_200 = [pin for pin in sorted_pins if int(pin['number']) > 100]
    if pins_101_200:
        content.append('# Connector 2: High-speed serial interfaces (Pins 101-200)')
        content.append('CM5IO_2(')
        content.append('    name = "J2_CM5",')
        
        for i, pin in enumerate(pins_101_200):
            pin_num = pin.get('number', '')
            # Map pins 101-200 to connector pins P1-P100
            connector_pin = int(pin_num) - 100
            var_name = pin_to_var[pin_num]
            comma = "," if i < len(pins_101_200) - 1 else ""
            comment = f"  # {pin.get('name', '')}" if pin.get('name', '') else ""
            content.append(f'    P{connector_pin} = {var_name}{comma}{comment}')
        
        content.append(')')
        content.append('')
    
    # Add schematic layout positions
    content.append('# ' + '-' * 76)
    content.append('# Schematic Layout')
    content.append('# ' + '-' * 76)
    content.append('')
    content.append('# pcb:sch J1_CM5.10164227-1001A1RLF x=800.0 y=400.0 rot=0')
    content.append('# pcb:sch J2_CM5.10164227-1001A1RLF x=2400.0 y=400.0 rot=0')
    
    # Write to file
    with open(output_file, 'w') as f:
        f.write('\n'.join(content))
    
    print(f"\nGenerated Zen file: {output_file}")
    print(f"Total pins processed: {len(sorted_pins)}")
    print(f"Unique IO declarations: {len(unique_ios)}")
    print(f"  - Connector 1 (GPIO): {len(pins_1_100)} pins")
    print(f"  - Connector 2 (HSS): {len(pins_101_200)} pins")

def main():
    if len(sys.argv) < 2:
        print("Usage: python kicad_zen_generator_fixed.py <kicad_symbol_file> [output_file]")
        sys.exit(1)
    
    file_path = Path(sys.argv[1])
    output_file = sys.argv[2] if len(sys.argv) > 2 else "CM5_IO_generated.zen"
    
    if not file_path.exists():
        print(f"Error: File '{file_path}' not found")
        sys.exit(1)
    
    # Parse the file
    symbols = parse_kicad_pins(file_path)
    
    if not symbols:
        print("Error: No symbols found in file")
        sys.exit(1)
    
    # Generate the Zen file
    generate_zen_file(symbols, output_file)

if __name__ == "__main__":
    main()