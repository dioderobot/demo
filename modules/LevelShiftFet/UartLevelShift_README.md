# UartLevelShift Module

A UART level shifter module using two SN74AXC1T45QDCKRQ1 automotive-qualified level translators for bidirectional UART communication between different voltage domains.

## Features

- **Dual voltage support**: 0.65V to 3.6V on both sides
- **Automotive qualified**: -40°C to +125°C operation
- **High-speed**: Up to 10 Mbps baud rate support
- **Proper UART interfaces**: Uses stdlib Uart interfaces for clean connections
- **Automatic direction control**: No manual DIR pin management required
- **VCC isolation**: I/Os become high-Z if either VCC < 100mV

## Usage

```zen
load("@stdlib/interfaces.zen", "Ground", "Power", "Uart")

UartLevelShift = Module("//modules/basic/UartLevelShift.zen")

# Power supplies
power_3v3 = Power("power_3v3")
power_1v8 = Power("power_1v8") 
ground = Ground("ground")

# UART interfaces
mcu_uart = Uart("mcu_uart")      # 3.3V device
sensor_uart = Uart("sensor_uart") # 1.8V device

# Level shifter
UartLevelShift(
    name = "UART_LevelShifter",
    vcc_a = power_3v3,           # Higher voltage side
    vcc_b = power_1v8,           # Lower voltage side
    gnd = ground,
    uart_a = mcu_uart,           # Higher voltage UART
    uart_b = sensor_uart,        # Lower voltage UART
    config_vcc_a_voltage = "3.3V",
    config_vcc_b_voltage = "1.8V",
)
```

## Configuration

- `config_vcc_a_voltage`: Higher voltage domain (default: "3.3V")
- `config_vcc_b_voltage`: Lower voltage domain (default: "1.8V")

## Signal Routing

The module automatically handles bidirectional UART communication:

- **TX Path**: `uart_a.TX` → `uart_b.RX` (A→B direction, DIR=HIGH)
- **RX Path**: `uart_b.TX` → `uart_a.RX` (B→A direction, DIR=LOW)

## Components Used

- 2x SN74AXC1T45QDCKRQ1 level translators
- Integrated bypass capacitors (100nF each VCC)
- Automatic pull-up/pull-down resistors for direction control

## Layout Considerations

- Place close to UART devices to minimize trace lengths
- Each translator includes integrated decoupling capacitors
- Automotive qualified for harsh environments
