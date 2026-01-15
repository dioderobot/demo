//! PWM Servo Input
//!
//! Standard RC servo PWM input (1000-2000us pulse width)

/// Servo input configuration
#[derive(Debug, Clone, Copy)]
pub struct ServoConfig {
    /// Minimum pulse width in microseconds
    pub min_us: u16,
    /// Maximum pulse width in microseconds
    pub max_us: u16,
    /// Center/neutral pulse width in microseconds
    pub center_us: u16,
    /// Dead band around center (for 3D mode)
    pub deadband_us: u16,
}

impl Default for ServoConfig {
    fn default() -> Self {
        Self {
            min_us: 1000,
            max_us: 2000,
            center_us: 1500,
            deadband_us: 25,
        }
    }
}

/// Servo input decoder
pub struct ServoInput {
    config: ServoConfig,
    /// Last measured pulse width in microseconds
    last_pulse_us: u16,
    /// Signal timeout counter
    timeout_count: u16,
    /// Signal valid flag
    valid: bool,
}

impl ServoInput {
    /// Create a new servo input decoder
    pub fn new(config: ServoConfig) -> Self {
        Self {
            config,
            last_pulse_us: 0,
            timeout_count: 0,
            valid: false,
        }
    }
    
    /// Process a captured pulse width
    pub fn process_pulse(&mut self, pulse_us: u16) {
        // Validate pulse width is in reasonable range
        if pulse_us >= 800 && pulse_us <= 2200 {
            self.last_pulse_us = pulse_us;
            self.timeout_count = 0;
            self.valid = true;
        }
    }
    
    /// Get normalized throttle value (0.0 to 1.0)
    pub fn normalized_throttle(&self) -> f32 {
        if !self.valid || self.last_pulse_us < self.config.min_us {
            return 0.0;
        }
        
        let pulse = self.last_pulse_us.min(self.config.max_us);
        let range = self.config.max_us - self.config.min_us;
        
        (pulse - self.config.min_us) as f32 / range as f32
    }
    
    /// Get throttle as duty cycle (0-2000 range)
    pub fn duty_cycle(&self) -> u16 {
        (self.normalized_throttle() * 2000.0) as u16
    }
    
    /// Get throttle for 3D mode (-1.0 to 1.0)
    pub fn throttle_3d(&self) -> f32 {
        if !self.valid {
            return 0.0;
        }
        
        let center = self.config.center_us as i32;
        let pulse = self.last_pulse_us as i32;
        let deadband = self.config.deadband_us as i32;
        
        // Check if in deadband
        if (pulse - center).abs() < deadband {
            return 0.0;
        }
        
        let half_range = (self.config.max_us - self.config.min_us) as f32 / 2.0;
        (pulse - center) as f32 / half_range
    }
    
    /// Check for signal timeout
    pub fn check_timeout(&mut self) -> bool {
        self.timeout_count = self.timeout_count.saturating_add(1);
        if self.timeout_count > 500 { // ~500ms at 1kHz check rate
            self.valid = false;
            true
        } else {
            false
        }
    }
    
    /// Check if signal is valid
    pub fn is_valid(&self) -> bool {
        self.valid
    }
    
    /// Get raw pulse width
    pub fn pulse_us(&self) -> u16 {
        self.last_pulse_us
    }
    
    /// Update configuration
    pub fn set_config(&mut self, config: ServoConfig) {
        self.config = config;
    }
}

impl Default for ServoInput {
    fn default() -> Self {
        Self::new(ServoConfig::default())
    }
}
