//! Default settings values
//!
//! Matches AM32 EEPROM structure for compatibility.

/// ESC settings structure
/// 
/// This matches the AM32 eepromBuffer structure for compatibility.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    // Motor settings
    /// Motor KV rating
    pub motor_kv: u16,
    /// Number of motor poles
    pub motor_poles: u8,
    /// Direction reversed
    pub dir_reversed: bool,
    /// Bidirectional mode (3D)
    pub bi_direction: bool,
    
    // Timing settings
    /// Commutation timing advance (degrees)
    pub timing_advance: u8,
    /// Dead time (timer ticks)
    pub dead_time: u8,
    /// PWM frequency (kHz)
    pub pwm_frequency: u8,
    
    // Startup settings
    /// Startup power (0-100%)
    pub startup_power: u8,
    /// Use sinusoidal startup
    pub use_sine_start: bool,
    /// Sine mode power
    pub sine_mode_power: u8,
    /// Sine mode range
    pub sine_mode_range: u8,
    
    // Protection settings
    /// Stall protection enabled
    pub stall_protection: bool,
    /// Temperature limit (°C)
    pub temp_limit: u8,
    /// Current limit (A)
    pub current_limit: u8,
    /// Low voltage cutoff per cell (mV)
    pub low_voltage_cutoff: u16,
    
    // Brake settings
    /// Brake on stop mode (0=off, 1=proportional, 2=full)
    pub brake_on_stop: u8,
    /// Running brake strength
    pub running_brake: u8,
    /// Active brake power
    pub active_brake_power: u8,
    
    // Input settings
    /// Servo low pulse (us)
    pub servo_low: u16,
    /// Servo high pulse (us)
    pub servo_high: u16,
    /// Servo neutral pulse (us)
    pub servo_neutral: u16,
    /// Servo dead band (us)
    pub servo_dead_band: u8,
    
    // Telemetry settings
    /// Serial telemetry enabled
    pub use_serial_telemetry: bool,
    /// Telemetry interval (ms)
    pub telemetry_interval: u8,
    
    // Advanced settings
    /// Complementary PWM enabled
    pub comp_pwm: bool,
    /// Variable PWM enabled
    pub variable_pwm: bool,
    /// Stuck rotor protection
    pub stuck_rotor_protection: bool,
    
    // Beep settings
    /// Beacon delay (seconds)
    pub beacon_delay: u8,
    /// Beacon volume
    pub beacon_volume: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            // Motor settings
            motor_kv: 2000,
            motor_poles: 14,
            dir_reversed: false,
            bi_direction: false,
            
            // Timing settings
            timing_advance: 22, // 22.5 degrees
            dead_time: 80,
            pwm_frequency: 24, // 24kHz
            
            // Startup settings
            startup_power: 50,
            use_sine_start: false,
            sine_mode_power: 5,
            sine_mode_range: 5,
            
            // Protection settings
            stall_protection: true,
            temp_limit: 120,
            current_limit: 0, // Disabled
            low_voltage_cutoff: 3200, // 3.2V per cell
            
            // Brake settings
            brake_on_stop: 0,
            running_brake: 0,
            active_brake_power: 0,
            
            // Input settings
            servo_low: 1000,
            servo_high: 2000,
            servo_neutral: 1500,
            servo_dead_band: 25,
            
            // Telemetry settings
            use_serial_telemetry: true,
            telemetry_interval: 32,
            
            // Advanced settings
            comp_pwm: true,
            variable_pwm: true,
            stuck_rotor_protection: true,
            
            // Beep settings
            beacon_delay: 30,
            beacon_volume: 5,
        }
    }
}

impl Settings {
    /// Create settings for DM0001 board
    pub fn dm0001_defaults() -> Self {
        Self {
            // DM0001 specific defaults
            dead_time: 80,
            pwm_frequency: 24,
            ..Default::default()
        }
    }
    
    /// Validate settings and clamp to valid ranges
    pub fn validate(&mut self) {
        self.motor_kv = self.motor_kv.clamp(100, 10000);
        self.motor_poles = self.motor_poles.clamp(2, 36);
        self.timing_advance = self.timing_advance.clamp(0, 30);
        self.dead_time = self.dead_time.clamp(10, 255);
        self.pwm_frequency = self.pwm_frequency.clamp(8, 48);
        self.startup_power = self.startup_power.clamp(10, 100);
        self.temp_limit = self.temp_limit.clamp(70, 150);
        self.servo_low = self.servo_low.clamp(800, 1200);
        self.servo_high = self.servo_high.clamp(1800, 2200);
    }
    
    /// Calculate voltage divider value for this KV
    pub fn voltage_divider(&self) -> u16 {
        // DM0001: 169k/18k = 10.39:1, scaled by 10 = 104
        104
    }
    
    /// Get electrical RPM from mechanical RPM
    pub fn electrical_rpm(&self, mechanical_rpm: u32) -> u32 {
        mechanical_rpm * (self.motor_poles as u32 / 2)
    }
    
    /// Get mechanical RPM from electrical RPM
    pub fn mechanical_rpm(&self, electrical_rpm: u32) -> u32 {
        electrical_rpm / (self.motor_poles as u32 / 2)
    }
}
