//! ADC Driver for BEMF and Sensing
//!
//! Provides ADC readings for:
//! - BEMF sensing (PA4/ADC2, PB11/ADC1, PB12/ADC1)
//! - Bus voltage (PA0/ADC1)
//! - Temperature (PB14/ADC1)
//!
//! Note: This is a simplified implementation. Full ADC support requires
//! proper pin configuration and the specific ADC channel mappings.

use crate::config::{VREF_MV, ADC_MAX, VOLTAGE_DIVIDER_RATIO};
use crate::motor::bemf::BemfSamples;

/// Sensor readings from ADC
#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct SensorReadings {
    /// Bus voltage in millivolts
    pub voltage_mv: u32,
    /// Temperature in degrees Celsius (x10 for precision)
    pub temperature_c10: i16,
    /// BEMF samples
    pub bemf: BemfSamples,
}

/// ADC driver for motor control sensing (software abstraction)
pub struct AdcSensing {
    // Filtered values
    voltage_filtered: u32,
    temp_filtered: u32,
    // Filter coefficient (higher = more filtering)
    filter_shift: u8,
}

impl AdcSensing {
    /// Create a new ADC sensing controller
    pub fn new() -> Self {
        defmt::info!("ADC sensing initialized (software mode)");

        Self {
            voltage_filtered: 0,
            temp_filtered: 0,
            filter_shift: 3, // Divide by 8 for averaging
        }
    }

    /// Update with raw ADC readings
    pub fn update(&mut self, voltage_raw: u16, temp_raw: u16) {
        // Apply exponential moving average filter
        let _coeff = 1u32 << self.filter_shift;
        
        if self.voltage_filtered == 0 {
            self.voltage_filtered = (voltage_raw as u32) << self.filter_shift;
        } else {
            self.voltage_filtered = self.voltage_filtered 
                - (self.voltage_filtered >> self.filter_shift)
                + voltage_raw as u32;
        }
        
        if self.temp_filtered == 0 {
            self.temp_filtered = (temp_raw as u32) << self.filter_shift;
        } else {
            self.temp_filtered = self.temp_filtered 
                - (self.temp_filtered >> self.filter_shift)
                + temp_raw as u32;
        }
    }

    /// Get filtered voltage in millivolts
    pub fn voltage_mv(&self) -> u32 {
        let filtered = self.voltage_filtered >> self.filter_shift;
        let adc_mv = (filtered * VREF_MV) / ADC_MAX;
        (adc_mv as f32 * VOLTAGE_DIVIDER_RATIO) as u32
    }

    /// Get filtered temperature in degrees Celsius
    pub fn temperature_c(&self) -> i16 {
        let filtered = self.temp_filtered >> self.filter_shift;
        self.adc_to_temperature(filtered as u16)
    }

    /// Convert ADC reading to temperature
    fn adc_to_temperature(&self, adc_value: u16) -> i16 {
        // Simplified NTC calculation
        // Assuming 10k NTC with 10k pullup, 25°C at ~1650mV
        let mv = (adc_value as u32 * VREF_MV) / ADC_MAX;
        let temp = 25 + ((1650i32 - mv as i32) / 40);
        temp.clamp(-40, 150) as i16
    }

    /// Set filter strength (0-7, higher = more filtering)
    pub fn set_filter(&mut self, shift: u8) {
        self.filter_shift = shift.min(7);
    }
}

impl Default for AdcSensing {
    fn default() -> Self {
        Self::new()
    }
}

/// Low voltage cutoff controller
pub struct LowVoltageCutoff {
    /// Cutoff voltage in millivolts
    cutoff_mv: u32,
    /// Warning voltage in millivolts
    warning_mv: u32,
    /// Hysteresis in millivolts
    hysteresis_mv: u32,
    /// Current state
    state: LvcState,
}

/// Low voltage cutoff state
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum LvcState {
    Normal,
    Warning,
    Cutoff,
}

impl LowVoltageCutoff {
    /// Create with cell count
    pub fn new(cell_count: u8) -> Self {
        let cutoff_per_cell = 3200; // 3.2V per cell
        let warning_per_cell = 3400; // 3.4V per cell
        
        Self {
            cutoff_mv: cell_count as u32 * cutoff_per_cell,
            warning_mv: cell_count as u32 * warning_per_cell,
            hysteresis_mv: 200,
            state: LvcState::Normal,
        }
    }

    /// Update with current voltage
    pub fn update(&mut self, voltage_mv: u32) -> LvcState {
        match self.state {
            LvcState::Cutoff => {
                if voltage_mv > self.cutoff_mv + self.hysteresis_mv {
                    self.state = LvcState::Normal;
                }
            }
            LvcState::Warning => {
                if voltage_mv < self.cutoff_mv {
                    self.state = LvcState::Cutoff;
                    defmt::warn!("Low voltage cutoff! {}mV", voltage_mv);
                } else if voltage_mv > self.warning_mv + self.hysteresis_mv {
                    self.state = LvcState::Normal;
                }
            }
            LvcState::Normal => {
                if voltage_mv < self.cutoff_mv {
                    self.state = LvcState::Cutoff;
                    defmt::warn!("Low voltage cutoff! {}mV", voltage_mv);
                } else if voltage_mv < self.warning_mv {
                    self.state = LvcState::Warning;
                    defmt::warn!("Low voltage warning! {}mV", voltage_mv);
                }
            }
        }
        self.state
    }

    /// Get current state
    pub fn state(&self) -> LvcState {
        self.state
    }

    /// Detect cell count from voltage
    pub fn detect_cells(voltage_mv: u32) -> u8 {
        // Assume fully charged cells at ~4.2V
        ((voltage_mv + 2100) / 4200) as u8
    }

    /// Set cutoff voltages directly
    pub fn set_voltages(&mut self, cutoff_mv: u32, warning_mv: u32) {
        self.cutoff_mv = cutoff_mv;
        self.warning_mv = warning_mv;
    }
}

// Note: Full hardware ADC implementation requires:
//
// ```rust
// use embassy_stm32::adc::{Adc, AdcConfig};
// use embassy_stm32::peripherals::{ADC1, ADC2};
//
// // Initialize ADC
// let adc1 = Adc::new(p.ADC1, AdcConfig::default());
// let adc2 = Adc::new(p.ADC2, AdcConfig::default());
//
// // Read channels
// let vbus = adc1.blocking_read(&mut p.PA0);
// let bemf_a = adc2.blocking_read(&mut p.PA4);
// ```
