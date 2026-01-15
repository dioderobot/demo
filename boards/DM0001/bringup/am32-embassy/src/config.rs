//! Board configuration for DM0001 ZenDrive Motor Controller
//!
//! Pin mapping and hardware configuration specific to the DM0001 board.

use embassy_stm32::gpio::Pin;

/// DM0001 board configuration
pub struct BoardConfig {
    pub name: &'static str,
    pub mcu: &'static str,
}

impl BoardConfig {
    pub fn new() -> Self {
        Self {
            name: "DM0001 ZenDrive",
            mcu: "STM32G431C8T6",
        }
    }
}

/// PWM output pins for 3-phase motor control
/// 
/// TIM1 is used for complementary PWM with dead-time insertion:
/// - Channel 1: Phase A (PA8 high, PC13 low)
/// - Channel 2: Phase B (PA9 high, PA12 low)
/// - Channel 3: Phase C (PA10 high, PB15 low)
pub mod pwm_pins {
    // Phase A
    pub const PHASE_A_HIGH: u8 = 8;   // PA8 - TIM1_CH1
    pub const PHASE_A_LOW: u8 = 13;   // PC13 - TIM1_CH1N
    
    // Phase B
    pub const PHASE_B_HIGH: u8 = 9;   // PA9 - TIM1_CH2
    pub const PHASE_B_LOW: u8 = 12;   // PA12 - TIM1_CH2N
    
    // Phase C
    pub const PHASE_C_HIGH: u8 = 10;  // PA10 - TIM1_CH3
    pub const PHASE_C_LOW: u8 = 15;   // PB15 - TIM1_CH3N
}

/// BEMF sensing ADC channels
/// 
/// ADC-based BEMF zero-crossing detection:
/// - Phase A: PA4 -> ADC2_IN17
/// - Phase B: PB12 -> ADC1_IN11
/// - Phase C: PB11 -> ADC1_IN14
pub mod bemf_pins {
    pub const BEMF_A_PIN: u8 = 4;     // PA4
    pub const BEMF_A_ADC_CH: u8 = 17; // ADC2_IN17
    
    pub const BEMF_B_PIN: u8 = 12;    // PB12
    pub const BEMF_B_ADC_CH: u8 = 11; // ADC1_IN11
    
    pub const BEMF_C_PIN: u8 = 11;    // PB11
    pub const BEMF_C_ADC_CH: u8 = 14; // ADC1_IN14
}

/// Input signal pin
/// 
/// DSHOT/PWM input on PA15 using TIM2_CH1
pub mod input_pins {
    pub const INPUT_PIN: u8 = 15;     // PA15
    pub const INPUT_TIMER_CH: u8 = 1; // TIM2_CH1
}

/// Analog sensing pins
pub mod sensing_pins {
    pub const VBUS_PIN: u8 = 0;       // PA0 - ADC1_IN1
    pub const VBUS_ADC_CH: u8 = 1;
    
    pub const TEMP_PIN: u8 = 14;      // PB14 - NTC thermistor
    pub const TEMP_ADC_CH: u8 = 5;
}

/// CAN bus pins
pub mod can_pins {
    pub const CAN_RX: u8 = 11;        // PA11 - FDCAN1_RX
    pub const CAN_TX: u8 = 9;         // PB9 - FDCAN1_TX
}

/// Debug pins
pub mod debug_pins {
    pub const SWDIO: u8 = 13;         // PA13
    pub const SWCLK: u8 = 14;         // PA14
}

/// Voltage divider configuration for VBUS sensing
/// R_top = 169kΩ, R_bottom = 18kΩ
/// Ratio = (169 + 18) / 18 = 10.39:1
pub const VOLTAGE_DIVIDER_RATIO: f32 = 10.39;

/// ADC reference voltage (3.3V)
pub const VREF_MV: u32 = 3300;

/// ADC resolution (12-bit)
pub const ADC_MAX: u32 = 4095;

/// Convert ADC reading to millivolts (before divider)
pub fn adc_to_mv(adc_value: u16) -> u32 {
    (adc_value as u32 * VREF_MV) / ADC_MAX
}

/// Convert ADC reading to bus voltage in millivolts
pub fn adc_to_vbus_mv(adc_value: u16) -> u32 {
    let mv = adc_to_mv(adc_value);
    (mv as f32 * VOLTAGE_DIVIDER_RATIO) as u32
}
