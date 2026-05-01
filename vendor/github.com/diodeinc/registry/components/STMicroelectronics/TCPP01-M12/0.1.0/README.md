# TCPP01-M12

STMicroelectronics USB Type-C port protection IC for sink applications. Provides
ESD, short-to-VBUS OVP on CC1/CC2, programmable VBUS OVP via an external
N-MOSFET, dead-battery management, and open-drain fault reporting in a 3 × 3 mm
QFN-12L.

## Usage

```python
TCPP01_M12 = Module("@github/diodeinc/registry/components/STMicroelectronics/TCPP01-M12/TCPP01-M12.zen")

TCPP01_M12(
    name="U1",
    VCC=vcc_3v3,
    GND=gnd,
    # Connector side
    CC1C=usb_cc1,
    CC2C=usb_cc2,
    IN_GD=usb_vbus,
    # System side
    CC1=mcu_cc1,
    CC2=mcu_cc2,
    SOURCE=sys_vbus,
    GATE=fet_gate,
    VBUS_CTRL=ovp_set,
    DB=dead_batt,
    FLT=fault_n,
)
```
