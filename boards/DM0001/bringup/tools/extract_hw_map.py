#!/usr/bin/env python3
"""Generate a machine-readable bringup map for the DM0001 ESC."""

from __future__ import annotations

import json
import math
import re
from pathlib import Path


THIS_FILE = Path(__file__).resolve()
BOARD_DIR = THIS_FILE.parents[2]
BRINGUP_DIR = THIS_FILE.parents[1]
BOARD_ZEN = BOARD_DIR / "DM0001.zen"


PIN_CAPS = {
    "PA0": {"adc_channel": "ADC12_IN1", "functions": ["TIM2_CH1", "USART2_CTS", "TIM8_BKIN", "TIM8_ETR"]},
    "PA1": {"adc_channel": "ADC12_IN2", "functions": ["TIM2_CH2", "USART2_RTS_DE", "TIM15_CH1N", "OPAMP1_VINP"]},
    "PA2": {"adc_channel": "ADC1_IN3", "functions": ["TIM2_CH3", "USART2_TX", "TIM15_CH1", "OPAMP1_VOUT"]},
    "PA3": {"adc_channel": "ADC1_IN4", "functions": ["TIM2_CH4", "USART2_RX", "TIM15_CH2", "OPAMP1_VINM"]},
    "PA4": {"adc_channel": "ADC2_IN17", "functions": ["TIM3_CH2", "SPI1_NSS", "SPI3_NSS", "USART2_CK"]},
    "PA5": {"adc_channel": "ADC2_IN13", "functions": ["TIM2_CH1", "TIM2_ETR", "SPI1_SCK", "OPAMP2_VINM"]},
    "PA6": {"adc_channel": "ADC2_IN3", "functions": ["TIM16_CH1", "TIM3_CH1", "SPI1_MISO", "TIM1_BKIN", "OPAMP2_VOUT"]},
    "PA7": {"adc_channel": "ADC2_IN4", "functions": ["TIM17_CH1", "TIM3_CH2", "SPI1_MOSI", "TIM1_CH1N", "OPAMP2_VINP"]},
    "PA8": {"functions": ["TIM1_CH1", "I2C3_SCL", "I2C2_SDA", "USART1_CK"]},
    "PA9": {"functions": ["TIM1_CH2", "I2C3_SMBA", "I2C2_SCL", "USART1_TX"]},
    "PA10": {"functions": ["TIM1_CH3", "SPI2_MISO", "USART1_RX", "TIM2_CH4"]},
    "PA11": {"functions": ["TIM1_CH1N", "FDCAN1_RX", "TIM1_CH4", "TIM1_BKIN2"]},
    "PA12": {"functions": ["TIM1_CH2N", "FDCAN1_TX", "TIM1_ETR", "TIM16_CH1"]},
    "PA13": {"functions": ["SWDIO", "TIM16_CH1N", "I2C1_SCL", "USART3_CTS"]},
    "PA14": {"functions": ["SWCLK", "TIM1_BKIN", "I2C1_SDA", "USART2_TX"]},
    "PA15": {"functions": ["TIM2_CH1", "TIM8_CH1", "SPI1_NSS", "SPI3_NSS", "USART2_RX", "TIM1_BKIN"]},
    "PB0": {"adc_channel": "ADC1_IN15", "functions": ["TIM3_CH3", "TIM8_CH2N", "TIM1_CH2N", "OPAMP3_VINP"]},
    "PB1": {"adc_channel": "ADC1_IN12", "functions": ["TIM3_CH4", "TIM8_CH3N", "TIM1_CH3N", "OPAMP3_VOUT"]},
    "PB2": {"adc_channel": "ADC2_IN12", "functions": ["RTC_OUT2", "LPTIM1_OUT", "I2C3_SMBA", "OPAMP3_VINM"]},
    "PB3": {"functions": ["TRACE_SWO", "TIM2_CH2", "TIM4_ETR", "SPI1_SCK", "SPI3_SCK", "USART2_TX"]},
    "PB4": {"functions": ["JTRST", "TIM16_CH1", "TIM3_CH1", "SPI1_MISO", "SPI3_MISO", "USART2_RX"]},
    "PB5": {"functions": ["TIM16_BKIN", "TIM3_CH2", "SPI1_MOSI", "SPI3_MOSI", "USART2_CK", "I2C3_SDA", "TIM17_CH1"]},
    "PB6": {"functions": ["TIM16_CH1N", "TIM4_CH1", "TIM8_CH1", "USART1_TX", "LPTIM1_ETR"]},
    "PB7": {"functions": ["TIM17_CH1N", "TIM4_CH2", "I2C1_SDA", "USART1_RX", "TIM3_CH4", "LPTIM1_IN2"]},
    "PB8_BOOT0": {"functions": ["TIM16_CH1", "TIM4_CH3", "I2C1_SCL", "USART3_RX", "FDCAN1_RX", "TIM8_CH2", "TIM1_BKIN"]},
    "PB9": {"functions": ["TIM17_CH1", "TIM4_CH4", "I2C1_SDA", "USART3_TX", "FDCAN1_TX", "TIM8_CH3", "TIM1_CH3N"]},
    "PB10": {"functions": ["TIM2_CH3", "USART3_TX", "LPUART1_RX", "TIM1_BKIN"]},
    "PB11": {"adc_channel": "ADC12_IN14", "functions": ["TIM2_CH4", "USART3_RX", "LPUART1_TX"]},
    "PB12": {"adc_channel": "ADC1_IN11", "functions": ["I2C2_SMBA", "SPI2_NSS", "TIM1_BKIN", "USART3_CK"]},
    "PB13": {"functions": ["SPI2_SCK", "TIM1_CH1N", "USART3_CTS"]},
    "PB14": {"adc_channel": "ADC1_IN5", "functions": ["TIM15_CH1", "SPI2_MISO", "TIM1_CH2N", "USART3_RTS_DE"]},
    "PB15": {"adc_channel": "ADC2_IN15", "functions": ["TIM15_CH2", "TIM15_CH1N", "TIM1_CH3N", "SPI2_MOSI"]},
    "PC13": {"functions": ["TIM1_BKIN", "TIM1_CH1N", "TIM8_CH4N"]},
    "PC14_OSC32_IN": {"functions": ["RTC_OSC32_IN"]},
    "PC15_OSC32_OUT": {"functions": ["RTC_OSC32_OUT"]},
    "NRST": {"functions": ["NRST"]},
}


def parse_mcu_pin_map() -> dict[str, dict[str, str]]:
    text = BOARD_ZEN.read_text()
    lines = text.splitlines()

    in_mcu = False
    mcu_pins: dict[str, dict[str, str]] = {}
    pin_re = re.compile(r"^\s*(?P<pin>[A-Z0-9_]+)\s*=\s*(?P<net>[A-Z0-9_+]+),\s*(?:#\s*(?P<comment>.*))?$")

    for line in lines:
        if not in_mcu and line.strip() == "Mcu(":
            in_mcu = True
            continue
        if in_mcu and line.strip() == ")":
            break
        if not in_mcu:
            continue

        match = pin_re.match(line)
        if not match:
            continue
        pin = match.group("pin")
        net = match.group("net")
        comment = (match.group("comment") or "").strip()
        if pin in {"name", "V3V3", "GND", "schematic"}:
            continue
        mcu_pins[pin] = {"net": net, "comment": comment}

    return mcu_pins


def pin_entry(pin_map: dict[str, dict[str, str]], pin: str) -> dict[str, object]:
    caps = PIN_CAPS.get(pin, {})
    entry = {
        "pin": pin,
        "net": pin_map[pin]["net"],
        "comment": pin_map[pin]["comment"],
    }
    if "adc_channel" in caps:
        entry["adc_channel"] = caps["adc_channel"]
    if "functions" in caps:
        entry["functions"] = caps["functions"]
    return entry


def ohms(value: float) -> int:
    return int(round(value))


def shunt_bias_volts(vdda: float = 3.3) -> float:
    g_vdda = 1.0 / 22000.0
    g_gnd = 1.0 / 2200.0
    g_shunt = 1.0 / 1500.0
    return (vdda * g_vdda) / (g_vdda + g_gnd + g_shunt)


def shunt_input_gain() -> float:
    return 1.0 / (1.0 + 1.5 / 2.2 + 1.5 / 22.0)


def current_zero_output_volts(vdda: float = 3.3, pga_gain: float = 16.0) -> float:
    return shunt_bias_volts(vdda) * pga_gain


def current_sensitivity_volts_per_amp(shunt_res_ohms: float = 0.003, pga_gain: float = 16.0) -> float:
    return shunt_input_gain() * shunt_res_ohms * pga_gain


def vbus_scale() -> float:
    return (169.0 + 18.0) / 18.0


def build_map() -> dict[str, object]:
    pin_map = parse_mcu_pin_map()

    opamp_pga_gain = 16.0
    shunt_resistor = 0.003

    return {
        "sources": {
            "board_zen": str(BOARD_ZEN.relative_to(BOARD_DIR.parent.parent)),
            "mcu_datasheet_markdown": "components/STMicroelectronics/STM32G431CBU6/STM32G431CBU6.md",
            "gate_driver_markdown": "components/STMicroelectronics/L6387ED/L6387ED.md",
        },
        "mcu": {
            "board_declared_part": "STM32G431C8T6",
            "board_docs_part": "STM32G431CBU6",
            "firmware_target_family": "STM32G431CB",
            "package_note": "Board sources disagree on exact SKU. Firmware uses STM32G431CB family assumptions because the BOM/docs and local datasheet point to the 128 KiB CBU6 device.",
            "pins": pin_map,
        },
        "pwm": {
            "timer": "TIM1",
            "phases": {
                "A": {
                    "high": pin_entry(pin_map, "PA8"),
                    "low": pin_entry(pin_map, "PC13"),
                    "phase_net": "PHASE_A",
                    "gate_driver": "L6387ED",
                },
                "B": {
                    "high": pin_entry(pin_map, "PA9"),
                    "low": pin_entry(pin_map, "PA12"),
                    "phase_net": "PHASE_B",
                    "gate_driver": "L6387ED",
                },
                "C": {
                    "high": pin_entry(pin_map, "PA10"),
                    "low": pin_entry(pin_map, "PB15"),
                    "phase_net": "PHASE_C",
                    "gate_driver": "L6387ED",
                },
            },
            "safety_note": "L6387 HIN/LIN inputs have internal pulldowns. Initial bringup should keep PWM pins as GPIO/Hi-Z until TIM1 is configured and outputs are explicitly enabled.",
        },
        "adc": {
            "bus_voltage": pin_entry(pin_map, "PA0"),
            "phase_voltage": {
                "A": pin_entry(pin_map, "PA4"),
                "B": pin_entry(pin_map, "PB12"),
                "C": pin_entry(pin_map, "PB11"),
            },
            "temperature": {
                "board_ntc": pin_entry(pin_map, "PB14"),
                "mcu_internal": {"adc_channel": "ADC1_IN16", "name": "TS"},
            },
            "current_raw_inputs": {
                "A": pin_entry(pin_map, "PA1"),
                "B": pin_entry(pin_map, "PA7"),
                "C": pin_entry(pin_map, "PB0"),
            },
            "current_opamp_outputs": {
                "A": pin_entry(pin_map, "PA2"),
                "B": pin_entry(pin_map, "PA6"),
                "C": pin_entry(pin_map, "PB1"),
            },
            "internal": {
                "vrefint": {"adc_channel": "ADC1_IN18"},
                "vbat_internal_div3": {"adc_channel": "ADC1_IN17", "note": "Available on STM32, not used by DM0001 power tree."},
            },
        },
        "temp": {
            "board_ntc": {
                "pin": "PB14",
                "net": "TEMP_FEEDBACK",
                "adc_channel": "ADC1_IN5",
                "sensor": "TDK NTCG103JF103FT1",
                "pullup_ohms": ohms(10000),
                "pulldown_ohms": ohms(4700),
                "filter_cap_nF": 10,
                "note": "Use as a raw divider voltage first; convert to temperature after validating the exact thermistor beta against the part datasheet or calibration data.",
            },
            "internal": {
                "adc_channel": "ADC1_IN16",
                "note": "Use factory TS_CAL1/TS_CAL2 constants for MCU die temperature.",
            },
        },
        "current": {
            "architecture": "3x low-side shunt, buffered/amplified by STM32 internal OPAMP1/2/3",
            "shunt_resistor_ohms": shunt_resistor,
            "channels": {
                "A": {
                    "shunt_net": "VSHUNTP_A",
                    "opamp_input": pin_entry(pin_map, "PA1"),
                    "opamp_output": pin_entry(pin_map, "PA2"),
                    "opamp_instance": "OPAMP1",
                },
                "B": {
                    "shunt_net": "VSHUNTP_B",
                    "opamp_input": pin_entry(pin_map, "PA7"),
                    "opamp_output": pin_entry(pin_map, "PA6"),
                    "opamp_instance": "OPAMP2",
                },
                "C": {
                    "shunt_net": "VSHUNTP_C",
                    "opamp_input": pin_entry(pin_map, "PB0"),
                    "opamp_output": pin_entry(pin_map, "PB1"),
                    "opamp_instance": "OPAMP3",
                },
            },
            "front_end": {
                "bias_network": {
                    "top_to_vdda_ohms": ohms(22000),
                    "bottom_to_gnd_ohms": ohms(2200),
                    "feedback_from_shunt_ohms": ohms(1500),
                },
                "firmware_inference": {
                    "mode": "non_inverting_pga",
                    "gain": opamp_pga_gain,
                    "reason": "The external resistor network only produces a usable ADC range if the internal OPAMP PGA is enabled. Follower mode would leave the current signal too small for practical telemetry.",
                },
                "estimated_zero_current_output_volts": round(current_zero_output_volts(pga_gain=opamp_pga_gain), 6),
                "estimated_output_sensitivity_volts_per_amp": round(
                    current_sensitivity_volts_per_amp(shunt_res_ohms=shunt_resistor, pga_gain=opamp_pga_gain), 6
                ),
                "estimated_current_formula_amps": {
                    "expression": "(v_opamp - v_zero) / volts_per_amp",
                    "v_zero": round(current_zero_output_volts(pga_gain=opamp_pga_gain), 6),
                    "volts_per_amp": round(
                        current_sensitivity_volts_per_amp(shunt_res_ohms=shunt_resistor, pga_gain=opamp_pga_gain), 6
                    ),
                },
            },
        },
        "voltage": {
            "bus": {
                "pin": "PA0",
                "net": "VBUS",
                "adc_channel": "ADC12_IN1",
                "divider_top_ohms": ohms(169000),
                "divider_bottom_ohms": ohms(18000),
                "scale_to_input": round(vbus_scale(), 6),
                "clamp_to_vdda": True,
                "max_linear_input_volts_at_vdda_3v3": round(3.3 * vbus_scale(), 3),
            },
            "phase": {
                "A": {
                    "pin": "PA4",
                    "net": "BEMF1",
                    "adc_channel": "ADC2_IN17",
                },
                "B": {
                    "pin": "PB12",
                    "net": "BEMF2",
                    "adc_channel": "ADC1_IN11",
                },
                "C": {
                    "pin": "PB11",
                    "net": "BEMF3",
                    "adc_channel": "ADC12_IN14",
                },
                "frontend": {
                    "series_from_phase_ohms": ohms(10000),
                    "pull_to_gpio_bemf_ohms": ohms(2200),
                    "clamp_to_vdda": True,
                    "note": "These are clamped back-EMF sense nodes, not full-range divided phase-voltage channels. Treat them as protected BEMF telemetry and expect saturation near VDDA.",
                },
            },
            "supply_rail": {
                "method": "ADC1_IN18 VREFINT-based VDDA estimate",
                "note": "Use factory VREFINT calibration to infer the actual 3V3 analog rail.",
            },
        },
        "enable": {
            "power_stage_enable": None,
            "kill_signal": None,
            "can_shutdown": {
                "pin": "PB13",
                "net": "CAN_SHDN",
            },
            "status_led": {
                "pin": "PC15_OSC32_OUT",
                "net": "STATUS",
            },
            "note": "No dedicated global motor-enable net is defined in DM0001. Safe bringup depends on keeping TIM1 outputs disabled until explicitly commanded.",
        },
        "gate_driver": {
            "type": "3x L6387ED half-bridge drivers",
            "gpio_bemf_or": {
                "pin": "PB5",
                "net": "GPIO_BEMF",
                "note": "Shared diode-OR style BEMF GPIO from the three phase-driver subcircuits.",
            },
        },
        "digital_inputs": {
            "hall": {
                "A": pin_entry(pin_map, "PB6"),
                "B": pin_entry(pin_map, "PB7"),
                "Z": pin_entry(pin_map, "PB8_BOOT0"),
            },
        },
        "interfaces": {
            "can": {
                "rx": pin_entry(pin_map, "PA11"),
                "tx": pin_entry(pin_map, "PB9"),
                "shutdown": pin_entry(pin_map, "PB13"),
                "termination_select": pin_entry(pin_map, "PC14_OSC32_IN"),
            },
            "uart2": {
                "tx": pin_entry(pin_map, "PB3"),
                "rx": pin_entry(pin_map, "PB4"),
                "pwm_header_input": pin_entry(pin_map, "PA15"),
            },
            "swd": {
                "swdio": pin_entry(pin_map, "PA13"),
                "swclk": pin_entry(pin_map, "PA14"),
                "nrst": pin_entry(pin_map, "NRST"),
            },
            "i2c_spi_sensors": [],
        },
    }


def main() -> int:
    hw_map = build_map()
    output_path = BRINGUP_DIR / "hardware_map.json"
    output_path.write_text(json.dumps(hw_map, indent=2, sort_keys=False) + "\n")
    print(output_path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
