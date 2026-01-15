//! ADC Sensing for Voltage, Current, and Temperature
//!
//! Provides filtered readings for:
//! - Bus voltage (PA0 -> ADC1_IN1)
//! - Phase currents (via op-amp outputs)
//! - Temperature (PB14 -> NTC thermistor)

use crate::config::{VOLTAGE_DIVIDER_RATIO, VREF_MV, ADC_MAX};

/// Sensor readings
#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct SensorReadings {
    /// Bus voltage in millivolts
    pub voltage_mv: u32,
    /// Current in milliamps
    pub current_ma: u32,
    /// Temperature in degrees Celsius
    pub temperature_c: i16,
    /// MCU internal temperature in degrees Celsius
    pub mcu_temp_c: i16,
}

/// ADC sensing controller
pub struct AdcSensing {
    /// Filtered voltage reading
    voltage_filtered: u32,
    /// Filtered current reading
    current_filtered: u32,
    /// Filtered temperature reading
    temp_filtered: u32,
    /// Filter coefficient (higher = more filtering)
    filter_coeff: u8,
    /// Voltage divider ratio * 100 (for integer math)
    voltage_divider_x100: u32,
    /// Current sense gain (mV per amp)
    mv_per_amp: u16,
    /// Current sense offset (ADC counts at 0A)
    current_offset: u16,
}

impl AdcSensing {
    /// Create a new ADC sensing controller
    pub fn new() -> Self {
        Self {
            voltage_filtered: 0,
            current_filtered: 2048, // Mid-scale for bidirectional sensing
            temp_filtered: 0,
            filter_coeff: 8,
            voltage_divider_x100: (VOLTAGE_DIVIDER_RATIO * 100.0) as u32,
            mv_per_amp: 3, // 3mΩ shunt
            current_offset: 2048,
        }
    }
    
    /// Update with new ADC readings
    pub fn update(&mut self, voltage_raw: u16, current_raw: u16, temp_raw: u16) {
        // Exponential moving average filter
        let coeff = self.filter_coeff as u32;
        
        self.voltage_filtered = (self.voltage_filtered * (coeff - 1) + voltage_raw as u32) / coeff;
        self.current_filtered = (self.current_filtered * (coeff - 1) + current_raw as u32) / coeff;
        self.temp_filtered = (self.temp_filtered * (coeff - 1) + temp_raw as u32) / coeff;
    }
    
    /// Get processed sensor readings
    pub fn readings(&self) -> SensorReadings {
        SensorReadings {
            voltage_mv: self.calculate_voltage(),
            current_ma: self.calculate_current(),
            temperature_c: self.calculate_temperature(),
            mcu_temp_c: 25, // TODO: Read from internal temp sensor
        }
    }
    
    /// Calculate bus voltage in millivolts
    fn calculate_voltage(&self) -> u32 {
        // ADC to mV, then apply divider ratio
        let adc_mv = (self.voltage_filtered * VREF_MV) / ADC_MAX;
        (adc_mv * self.voltage_divider_x100) / 100
    }
    
    /// Calculate current in milliamps
    fn calculate_current(&self) -> u32 {
        // Current = (ADC_voltage - offset) / (shunt_resistance * gain)
        let adc_mv = (self.current_filtered * VREF_MV) / ADC_MAX;
        let offset_mv = (self.current_offset as u32 * VREF_MV) / ADC_MAX;
        
        if adc_mv > offset_mv {
            ((adc_mv - offset_mv) * 1000) / self.mv_per_amp as u32
        } else {
            0
        }
    }
    
    /// Calculate temperature in degrees Celsius
    /// 
    /// Uses a simplified NTC calculation. For accuracy, use a lookup table.
    fn calculate_temperature(&self) -> i16 {
        // Simplified linear approximation
        // Real implementation should use Steinhart-Hart equation or lookup table
        let adc_mv = (self.temp_filtered * VREF_MV) / ADC_MAX;
        
        // Assuming 10k NTC with 10k pullup, 25°C at ~1650mV
        // Rough approximation: -40mV per degree
        let temp = 25 + ((1650i32 - adc_mv as i32) / 40);
        temp.clamp(-40, 150) as i16
    }
    
    /// Get raw filtered voltage ADC value
    pub fn voltage_raw(&self) -> u16 {
        self.voltage_filtered as u16
    }
    
    /// Get raw filtered current ADC value
    pub fn current_raw(&self) -> u16 {
        self.current_filtered as u16
    }
    
    /// Get raw filtered temperature ADC value
    pub fn temp_raw(&self) -> u16 {
        self.temp_filtered as u16
    }
    
    /// Set filter coefficient
    pub fn set_filter(&mut self, coeff: u8) {
        self.filter_coeff = coeff.max(1);
    }
    
    /// Calibrate current offset (call with motor stopped)
    pub fn calibrate_current_offset(&mut self) {
        self.current_offset = self.current_filtered as u16;
        defmt::info!("Current offset calibrated to {}", self.current_offset);
    }
    
    /// Set current sense parameters
    pub fn set_current_params(&mut self, mv_per_amp: u16, offset: u16) {
        self.mv_per_amp = mv_per_amp;
        self.current_offset = offset;
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
    cutoff_active: bool,
    /// Warning active
    warning_active: bool,
}

impl LowVoltageCutoff {
    /// Create with cell count auto-detection
    pub fn new(cell_count: u8) -> Self {
        let cutoff_per_cell = 3200; // 3.2V per cell
        let warning_per_cell = 3400; // 3.4V per cell
        
        Self {
            cutoff_mv: cell_count as u32 * cutoff_per_cell,
            warning_mv: cell_count as u32 * warning_per_cell,
            hysteresis_mv: 200,
            cutoff_active: false,
            warning_active: false,
        }
    }
    
    /// Update with current voltage
    pub fn update(&mut self, voltage_mv: u32) -> LvcState {
        if self.cutoff_active {
            // Need voltage to rise above cutoff + hysteresis to recover
            if voltage_mv > self.cutoff_mv + self.hysteresis_mv {
                self.cutoff_active = false;
            }
        } else if voltage_mv < self.cutoff_mv {
            self.cutoff_active = true;
            defmt::warn!("Low voltage cutoff! {}mV", voltage_mv);
        }
        
        self.warning_active = voltage_mv < self.warning_mv && !self.cutoff_active;
        
        if self.cutoff_active {
            LvcState::Cutoff
        } else if self.warning_active {
            LvcState::Warning
        } else {
            LvcState::Normal
        }
    }
    
    /// Set cutoff voltage directly
    pub fn set_cutoff(&mut self, cutoff_mv: u32, warning_mv: u32) {
        self.cutoff_mv = cutoff_mv;
        self.warning_mv = warning_mv;
    }
    
    /// Detect cell count from voltage
    pub fn detect_cells(voltage_mv: u32) -> u8 {
        // Assume fully charged cells at ~4.2V
        ((voltage_mv + 2100) / 4200) as u8
    }
}

/// Low voltage cutoff state
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum LvcState {
    Normal,
    Warning,
    Cutoff,
}
