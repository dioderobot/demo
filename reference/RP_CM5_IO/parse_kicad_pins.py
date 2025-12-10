#!/usr/bin/env python3
"""
Parse KiCad symbol library file and extract pin information
in the specified format for use with io() declarations.
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

def format_pin_output(pins):
    """
    Format pins in the requested output style.
    
    Args:
        pins: List of pin dictionaries
    
    Returns:
        Formatted string with io() declarations
    """
    output = []
    seen_pins = set()
    
    # Sort pins by their number (convert to int for proper sorting)
    sorted_pins = sorted(pins, key=lambda x: int(x['number']))
    
    for pin in sorted_pins:
        name = pin.get('name', '')
        
        # Skip duplicate pin names
        if name in seen_pins:
            continue
        seen_pins.add(name)
        
        # Create variable name by replacing special characters
        var_name = name.replace('+', 'plus').replace('-', '_').replace('(', '_').replace(')', '')
        var_name = var_name.replace('/', '_').replace('.', '_').replace(' ', '_')
        var_name = re.sub(r'[^a-zA-Z0-9_]', '_', var_name)
        
        # Make sure variable name doesn't start with a number
        if var_name and var_name[0].isdigit():
            var_name = '_' + var_name
        
        output.append(f'io_{var_name} = io("{name}", Net)')
    
    return '\n'.join(output)

def main():
    if len(sys.argv) != 2:
        print("Usage: python parse_kicad_pins.py <kicad_symbol_file>")
        sys.exit(1)
    
    file_path = Path(sys.argv[1])
    
    if not file_path.exists():
        print(f"Error: File '{file_path}' not found")
        sys.exit(1)
    
    # Parse the file
    symbols = parse_kicad_pins(file_path)
    
    # Process each symbol
    for symbol_name, pins in symbols.items():
        if pins:  # Only process symbols with pins
            print(f"\n# {symbol_name}")
            print("-" * 50)
            formatted = format_pin_output(pins)
            print(formatted)

if __name__ == "__main__":
    main()