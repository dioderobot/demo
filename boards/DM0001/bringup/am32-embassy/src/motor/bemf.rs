//! ADC-Based BEMF Zero-Crossing Detection
//!
//! Implements software-based zero-crossing detection by sampling BEMF voltages
//! via ADC and comparing to a calculated virtual neutral point.
//!
//! This is a port of the C implementation in comparator_adc.c

use super::commutation::Phase;

/// Hysteresis threshold in ADC counts to prevent noise-induced false crossings
const BEMF_HYSTERESIS: u16 = 20;

/// ADC resolution (12-bit)
const ADC_MAX: u16 = 4095;

/// BEMF samples for all three phases
#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct BemfSamples {
    /// Phase A BEMF (PA4 -> ADC2_IN17)
    pub phase_a: u16,
    /// Phase B BEMF (PB12 -> ADC1_IN11)
    pub phase_b: u16,
    /// Phase C BEMF (PB11 -> ADC1_IN14)
    pub phase_c: u16,
}

impl BemfSamples {
    /// Calculate the virtual neutral point
    /// Vn = (Va + Vb + Vc) / 3
    pub fn neutral(&self) -> u16 {
        ((self.phase_a as u32 + self.phase_b as u32 + self.phase_c as u32) / 3) as u16
    }
    
    /// Get the value for a specific phase
    pub fn get(&self, phase: Phase) -> u16 {
        match phase {
            Phase::A => self.phase_a,
            Phase::B => self.phase_b,
            Phase::C => self.phase_c,
        }
    }
}

/// BEMF sensor for zero-crossing detection
pub struct BemfSensor {
    /// Last sampled values
    samples: BemfSamples,
    /// Currently monitored (floating) phase
    floating_phase: Phase,
    /// Previous crossing state for edge detection
    prev_state: bool,
    /// Number of consecutive crossings detected (for filtering)
    crossing_count: u8,
    /// Filter level (number of consistent readings required)
    filter_level: u8,
}

impl BemfSensor {
    /// Create a new BEMF sensor
    pub fn new() -> Self {
        Self {
            samples: BemfSamples::default(),
            floating_phase: Phase::C,
            prev_state: false,
            crossing_count: 0,
            filter_level: 2,
        }
    }
    
    /// Update samples from ADC readings
    pub fn update_samples(&mut self, samples: BemfSamples) {
        self.samples = samples;
    }
    
    /// Set which phase to monitor for zero-crossing
    pub fn set_floating_phase(&mut self, phase: Phase) {
        self.floating_phase = phase;
        self.crossing_count = 0;
        self.prev_state = false;
        defmt::trace!("BEMF monitoring phase {:?}", phase);
    }
    
    /// Get the current floating phase value
    pub fn floating_value(&self) -> u16 {
        self.samples.get(self.floating_phase)
    }
    
    /// Get the virtual neutral point
    pub fn neutral(&self) -> u16 {
        self.samples.neutral()
    }
    
    /// Check if zero-crossing is detected
    /// 
    /// # Arguments
    /// * `rising` - true if looking for rising BEMF (falling edge on comparator)
    /// 
    /// # Returns
    /// * `true` if zero-crossing detected with sufficient filtering
    pub fn detect_crossing(&mut self, rising: bool) -> bool {
        let floating = self.floating_value();
        let neutral = self.neutral();
        
        let current_state = if rising {
            // Looking for rising BEMF: floating phase going above neutral
            floating > neutral.saturating_add(BEMF_HYSTERESIS)
        } else {
            // Looking for falling BEMF: floating phase going below neutral
            floating < neutral.saturating_sub(BEMF_HYSTERESIS)
        };
        
        // Check if state changed
        if current_state != self.prev_state {
            self.crossing_count = 1;
            self.prev_state = current_state;
        } else if current_state {
            self.crossing_count = self.crossing_count.saturating_add(1);
        }
        
        // Return true only if we have enough consistent readings
        current_state && self.crossing_count >= self.filter_level
    }
    
    /// Get raw crossing state without filtering (for polling mode)
    pub fn get_crossing_state(&self, rising: bool) -> bool {
        let floating = self.floating_value();
        let neutral = self.neutral();
        
        if rising {
            floating > neutral.saturating_add(BEMF_HYSTERESIS)
        } else {
            floating < neutral.saturating_sub(BEMF_HYSTERESIS)
        }
    }
    
    /// Set the filter level (number of consistent readings required)
    pub fn set_filter_level(&mut self, level: u8) {
        self.filter_level = level.max(1);
    }
    
    /// Get current samples
    pub fn samples(&self) -> &BemfSamples {
        &self.samples
    }
}

impl Default for BemfSensor {
    fn default() -> Self {
        Self::new()
    }
}

/// BEMF timing calculator
pub struct BemfTiming {
    /// Last zero-crossing timestamp (in timer ticks)
    last_crossing_time: u32,
    /// Commutation interval (time between crossings)
    interval: u32,
    /// Average interval (filtered)
    avg_interval: u32,
}

impl BemfTiming {
    pub fn new() -> Self {
        Self {
            last_crossing_time: 0,
            interval: 0,
            avg_interval: 65535,
        }
    }
    
    /// Record a zero-crossing event
    pub fn record_crossing(&mut self, current_time: u32) {
        if self.last_crossing_time > 0 {
            self.interval = current_time.wrapping_sub(self.last_crossing_time);
            // Simple exponential moving average
            self.avg_interval = (self.avg_interval * 7 + self.interval) / 8;
        }
        self.last_crossing_time = current_time;
    }
    
    /// Get the commutation interval
    pub fn interval(&self) -> u32 {
        self.interval
    }
    
    /// Get the average interval
    pub fn avg_interval(&self) -> u32 {
        self.avg_interval
    }
    
    /// Calculate advance timing (when to commutate after zero-crossing)
    /// 
    /// Standard is 30 electrical degrees, but can be adjusted for timing advance.
    /// Returns the delay in timer ticks.
    pub fn advance_timing(&self, advance_degrees: u8) -> u32 {
        // Each step is 60 electrical degrees
        // Zero-crossing occurs at 30 degrees into the step
        // So we wait (30 - advance) degrees worth of time
        let degrees_to_wait = 30u32.saturating_sub(advance_degrees as u32);
        (self.avg_interval * degrees_to_wait) / 60
    }
    
    /// Calculate electrical RPM from interval
    /// 
    /// RPM = 60 / (interval_seconds * 6)
    /// where 6 is the number of steps per electrical revolution
    pub fn calculate_rpm(&self, timer_freq_hz: u32) -> u32 {
        if self.avg_interval == 0 {
            return 0;
        }
        // RPM = (timer_freq * 60) / (interval * 6)
        // Simplified: RPM = timer_freq * 10 / interval
        (timer_freq_hz as u64 * 10 / self.avg_interval as u64) as u32
    }
    
    /// Reset timing (e.g., after desync)
    pub fn reset(&mut self) {
        self.last_crossing_time = 0;
        self.interval = 0;
        self.avg_interval = 65535;
    }
}

impl Default for BemfTiming {
    fn default() -> Self {
        Self::new()
    }
}
