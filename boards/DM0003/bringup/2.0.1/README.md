# DM0003 v2.0.1 Bringup

Raspberry Pi CM5 carrier board v2.0.1 requires 2 reworks before bringup. Both are design issues that will be fixed in a future revision.

## Reworks

### 1. GPIO voltage reference resistor selection

The board ships with [R_GPIO_VREF_3V3](pcb://R_GPIO_VREF_3V3) (0Ω 0402) populated, tying GPIO_VREF to the 3.3V rail. [R_GPIO_VREF_1V8](pcb://R_GPIO_VREF_1V8) is DNP. If your CM5 module or peripherals require 1.8V GPIO levels, you must swap the population: remove [R_GPIO_VREF_3V3](pcb://R_GPIO_VREF_3V3) and populate [R_GPIO_VREF_1V8](pcb://R_GPIO_VREF_1V8) with a 0Ω 0402 jumper.

Symptom: I2C or SPI peripherals on the IO connectors fail to communicate when using 1.8V logic devices.

### 2. EEPROM write-protect pulldown too strong for I2C recovery

The 10kΩ pulldown [R_EEPROM_WP](pcb://R_EEPROM_WP) on EEPROM_nWP permanently enables write protection. If you need to flash the HAT EEPROM for device tree configuration, cut or remove [R_EEPROM_WP](pcb://R_EEPROM_WP) and bodge a 0Ω jumper from EEPROM_nWP to 3.3V to disable write protection during programming.

Symptom: EEPROM writes fail silently. `eepromutils` reports success but readback does not match.

After programming, restore [R_EEPROM_WP](pcb://R_EEPROM_WP) to re-enable write protection.

## Power-On Sequence

1. Insert CM5 module into the carrier board connectors [CM5IO.J1_CM5](pcb://CM5IO.J1_CM5) and [CM5IO.J2_CM5](pcb://CM5IO.J2_CM5)
2. Connect USB-C power to [USB_PD](pcb://USB_PD) (5V PD source required, minimum 3A)
3. Press [Power_Button](pcb://Power_Button) to enable GLOBAL_EN

The STMPS2151STR current-limit switch on the HDMI 5V rail and the AP22653W on USB 3.0 VBUS will bring up their respective outputs automatically.

## Power Rail Verification

| Rail   | Expected | Test Point / Net | Source                                    |
| ------ | -------- | ---------------- | ----------------------------------------- |
| VBUS   | 5V       | USB-C VBUS       | USB-C PD via [USB_PD](pcb://USB_PD)      |
| 5V     | 5V       | io_5v            | CM5 module regulator                      |
| 3.3V   | 3.3V     | io_3v3pi         | CM5 module regulator                      |
| 1.8V   | 1.8V     | io_1v8           | CM5 module regulator                      |
| M.2 3V3| 3.3V     | m2_3v3           | AP3441x buck via [Buck](pcb://Buck)       |
| HDMI 5V| 5V       | hdmi_5v          | STMPS2151STR current-limit switch         |
| BATT   | 3.0–4.2V | JST connector    | [JST_BATT](pcb://JST_BATT) (RTC battery) |

## Software Setup

### Prerequisites

Install Raspberry Pi OS on the CM5's eMMC or an NVMe drive in the M.2 slot.

```bash
# Install rpiboot for USB flashing (if CM5 has no SD card)
sudo apt install rpiboot

# On the carrier board, flip BOOT_SWITCH to ON position to pull nRPIBOOT low
# Connect a USB cable to the USB 2.0 port
rpiboot
```

Use the Raspberry Pi Imager to flash the OS onto the CM5 eMMC.

### Boot Switch

[BOOT_SWITCH](pcb://BOOT_SWITCH) (MKM11C04MG013) controls nRPIBOOT:
- **OFF** (default): Normal boot from eMMC/NVMe
- **ON**: USB mass storage mode for flashing

## Testing

### HDMI Output

1. Connect a display to [HDMI_Connector](pcb://HDMI_Connector)
2. Boot the CM5 — console output should appear on the display
3. If no output, verify the [Current_Limit_Switch](pcb://Current_Limit_Switch) HDMI 5V rail is active (~5V on hdmi_5v)

### Ethernet

1. Connect an Ethernet cable to [ETH](pcb://ETH) (CRJ009-ML4-TH RJ45 jack)
2. Verify link LEDs illuminate (green = activity, yellow = speed)
3. The TPD4EUSB30DQAR TVS diodes ([TVS_0](pcb://TVS_0), [TVS_1](pcb://TVS_1)) provide ESD protection on all pairs

```bash
ip link show eth0
ping -c 5 8.8.8.8
```

### USB 2.0

1. Connect a USB device to [USB_Connector](pcb://USB_Connector) (GT-USB-7010ASV Type-C receptacle)
2. The RCLAMP0502BATCT [Voltage_Clamp](pcb://Voltage_Clamp) provides ESD protection on D+/D−

```bash
lsusb
```

### USB 3.0

1. Connect a USB 3.0 device to [USB_PD](pcb://USB_PD) port (SHOUHAN TYPE-C24PQT)
2. The TUSB321RWBR handles CC logic and orientation detection
3. The HD3SS3212IRKSR [SSMUX](pcb://SSMUX) muxes SuperSpeed lanes based on cable orientation
4. The AP22653W [Current_Limit_Switch](pcb://Current_Limit_Switch) limits VBUS to 1.75A

```bash
lsusb -t
```

Verify USB 3.0 speed with a storage device:

```bash
sudo hdparm -t /dev/sda
```

### PCIe / M.2 NVMe

1. Insert an M.2 2230 NVMe SSD into [PCIe_Connector](pcb://PCIe_Connector) (TE 1-2199119-3)
2. Secure with [M_2_Standoff](pcb://M_2_Standoff)
3. The AP3441x [Buck](pcb://Buck) regulator provides 3.3V to the M.2 slot, enabled by PCIE_PWR_EN
4. [LED_Activity](pcb://LED_Activity) (green) indicates M.2 drive activity via DAS_DSS_LED1
5. The ASEK 32.768 kHz [Crystal](pcb://Crystal) provides SUSCLK for low-power states

```bash
lspci
sudo nvme list
sudo nvme smart-log /dev/nvme0n1
```

### CSI/DSI Camera/Display

1. Connect a MIPI CSI-2 camera or DSI display to [DSI_CSI_Connector](pcb://DSI_CSI_Connector) (Hirose FH12M-22S-0.5SH)
2. Camera GPIOs (CAM_GPIO0, CAM_GPIO1) and I2C0 are routed to the connector

```bash
# For a camera module
libcamera-hello --list-cameras
libcamera-still -o test.jpg
```

### I2C Bus Scan

The I2C1 bus is exposed on [I2C_Connector](pcb://I2C_Connector) (JST BM04B) with 2.2kΩ pullups ([R_SDA](pcb://R_SDA), [R_SCL](pcb://R_SCL)) and ESD protection ([ESD_Clamp](pcb://ESD_Clamp), RCLAMP0502BATCT).

The onboard LIS3DH accelerometer ([LIS3DH](pcb://LIS3DH)) is on I2C1 at address 0x18 (or 0x19 depending on SA0).

```bash
sudo i2cdetect -y 1
```

Expected devices on I2C1:
- **0x18/0x19**: LIS3DH accelerometer

### Accelerometer Test

```bash
# Read WHO_AM_I register (should return 0x33)
sudo i2cget -y 1 0x18 0x0F
```

### SPI Interface

The SPI bus is exposed on [SPI_Connector](pcb://SPI_Connector) (JST BM10B-SURS) with MISO, MOSI, CLK, CS, and three GPIO lines (BL, RST, DC) for driving an SPI display.

```bash
# Verify SPI is enabled
ls /dev/spidev*
```

### Fan Connector

[Fan_Connector](pcb://Fan_Connector) (JST BM04B) provides 5V power, PWM control, and tachometer feedback.

```bash
# Check fan speed (requires dtoverlay=pwm-fan)
cat /sys/class/hwmon/hwmon*/fan1_input
```

### LED Verification

| LED                                          | Color | Signal         | Active | Notes                                      |
| -------------------------------------------- | ----- | -------------- | ------ | ------------------------------------------ |
| [LED_PWR](pcb://LED_PWR)                     | Red   | PI_nLED_PWR    | Low    | Inverted by SN74LVC1G14 Schmitt trigger    |
| [LED_ACTIVITY](pcb://LED_ACTIVITY)           | Green | nLED_ACTIVITY  | Low    | Directly driven, active low                |
| [LED_Activity](pcb://LED_Activity) (M.2)     | Green | DAS_DSS_LED1   | Low    | M.2 slot activity                          |

## Troubleshooting

### Board Does Not Power On

1. Verify USB-C PD source provides 5V/3A minimum
2. Press [Power_Button](pcb://Power_Button) — GLOBAL_EN must be asserted
3. Check [JST_BATT](pcb://JST_BATT) RTC battery is not shorted or reversed
4. Measure 5V on io_5v, 3.3V on io_3v3pi, 1.8V on io_1v8

### No HDMI Output

1. Verify hdmi_5v rail is present (~5V) after the STMPS2151STR current-limit switch
2. Check the CM5 is fully seated in both 100-pin connectors
3. Try a different HDMI cable — the [HDMI_Connector](pcb://HDMI_Connector) (XUNPU HDMI-101S) requires a standard HDMI cable
4. Verify `config.txt` has HDMI output enabled

### M.2 Device Not Detected

1. Verify [Buck](pcb://Buck) output is 3.3V (AP3441x)
2. Check [R_EN](pcb://R_EN) (100kΩ pulldown) — PCIE_PWR_EN should be driven high by the CM5
3. Ensure the M.2 module is seated correctly and secured with [M_2_Standoff](pcb://M_2_Standoff)
4. Check `dmesg | grep pcie` for link training errors
5. Verify [LED_Activity](pcb://LED_Activity) flickers during boot

### I2C Devices Not Responding

1. Verify 3.3V on I2C bus pullups [R_SDA](pcb://R_SDA) and [R_SCL](pcb://R_SCL) (2.2kΩ to 3.3V)
2. Check GPIO_VREF is set correctly — if using 1.8V peripherals, apply rework #1
3. Confirm I2C is enabled in `raspi-config` or `/boot/config.txt`
4. Use a logic analyzer on SDA/SCL to check for bus contention

### USB 3.0 Only Enumerating at USB 2.0 Speed

1. Verify the HD3SS3212IRKSR [SSMUX](pcb://SSMUX) orientation select (DIR from TUSB321RWBR) is toggling correctly
2. Check ESD protection ICs [ESD_1](pcb://ESD_1), [ESD_2](pcb://ESD_2), [ESD_3](pcb://ESD_3) (TPD4EUSB30DQAR) for shorts
3. Try a USB 3.0 certified cable
4. Verify 3.3V on [C_SSMUX](pcb://C_SSMUX) decoupling cap (SSMUX VCC)

## Component Summary

| Reference                | Part Number          | Description                        |
| ------------------------ | -------------------- | ---------------------------------- |
| CM5IO                    | RPi CM5 IO Board     | 2x 100-pin CM5 connectors         |
| JST_BATT                 | SM02B-SRSS-TB        | RTC battery connector (JST SH)    |
| HDMI_Connector           | HDMI-101S            | Standard HDMI connector (XUNPU)   |
| Current_Limit_Switch     | STMPS2151STR         | HDMI 5V current-limit switch       |
| ETH                      | CRJ009-ML4-TH       | RJ45 Ethernet jack (Same Sky)     |
| TVS_0, TVS_1             | TPD4EUSB30DQAR       | Quad-channel TVS (Ethernet ESD)   |
| USB_Connector            | GT-USB-7010ASV       | USB-C receptacle (USB 2.0)        |
| Voltage_Clamp            | RCLAMP0502BATCT      | USB 2.0 ESD clamp (Semtech)       |
| USB_PD                   | TYPE-C24PQT          | USB-C receptacle (USB 3.0 PD)     |
| TUSB321RWBR              | TUSB321RWBR          | USB Type-C CC logic (TI)          |
| SSMUX                    | HD3SS3212IRKSR       | USB 3.0 SuperSpeed mux (TI)       |
| ESD_1, ESD_2, ESD_3      | TPD4EUSB30DQAR       | USB 3.0 ESD protection (TI)       |
| Current_Limit_Switch     | AP22653W6M7          | USB 3.0 VBUS current limiter      |
| C_BULK                   | TAJB107K006RNJ       | 100µF 6.3V tantalum (KYOCERA AVX) |
| PCIe_Connector           | 1-2199119-3          | M.2 M-Key connector (TE)          |
| Buck                     | AP3441x              | 5V→3.3V buck for M.2 slot         |
| Crystal                  | ASEK 32.768kHz       | SUSCLK oscillator for M.2         |
| DSI_CSI_Connector        | FH12M-22S-0.5SH     | 22-pin FPC (Hirose)               |
| SPI_Connector            | BM10B-SURS-TF       | 10-pin SPI + GPIO (JST)           |
| Fan_Connector            | BM04B-SRSS-TB        | 4-pin fan header (JST)            |
| I2C_Connector            | BM04B-SRSS-TB        | 4-pin I2C header (JST)            |
| LIS3DH                   | LIS3DH               | 3-axis accelerometer (ST)         |
| ESD_Clamp                | RCLAMP0502BATCT      | I2C ESD clamp (Semtech)           |
| SN74LVC1G14DRLRG4        | SN74LVC1G14DRLRG4   | Schmitt-trigger inverter (TI)     |
| Power_Button             | B3U-1000P            | Tactile button (OMRON)            |
| BOOT_SWITCH              | MKM11C04MG013        | SPST slide switch (boot select)   |
| R_GPIO_VREF_3V3          | 0Ω 0402              | GPIO VREF → 3.3V (populated)      |
| R_GPIO_VREF_1V8          | 0Ω 0402              | GPIO VREF → 1.8V (DNP)           |
| R_EEPROM_WP              | 10kΩ 0402            | EEPROM write-protect pulldown     |

## Design Notes

- **M.2 buck regulator**: The AP3441x requires adequate input capacitance — [C_VIN](pcb://C_VIN) (10µF 0805) must be placed close to the regulator input. Output is filtered by three 10µF caps ([C_VOUT1](pcb://C_VOUT1), [C_VOUT2](pcb://C_VOUT2), [C_VOUT3](pcb://C_VOUT3)).
- **USB-C CC logic**: The TUSB321RWBR handles cable detection and orientation. The DIR output drives the HD3SS3212IRKSR SuperSpeed mux. The PORT pin has a 4.7kΩ pullup to VBUS ([R_PORT](pcb://R_PORT)).
- **HDMI ESD protection**: The PUSB3F96X ESD protection ICs are currently commented out in the design. If ESD issues are observed on HDMI, consider populating them.
- **6-layer stackup**: The board uses a 6-layer stackup at 1.57mm thickness. Ensure controlled-impedance traces are maintained for HDMI TMDS, PCIe, USB 3.0 SuperSpeed, and Ethernet differential pairs.
