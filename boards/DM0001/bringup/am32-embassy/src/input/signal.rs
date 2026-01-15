//! Input Signal Abstraction
//!
//! Provides a unified interface for different input protocols.

use super::dshot::{DshotFrame, DshotCommand};

/// Input protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum InputProtocol {
    /// No signal detected
    None,
    /// DSHOT digital protocol
    Dshot,
    /// Standard PWM servo
    Servo,
    /// Oneshot125
    Oneshot125,
    /// Oneshot42
    Oneshot42,
    /// Multishot
    Multishot,
}

/// Unified input signal
#[derive(Debug, Clone, Copy, Default)]
pub struct InputSignal {
    /// Detected protocol
    pub protocol: InputProtocol,
    /// Throttle value (0-2000)
    pub throttle: u16,
    /// Telemetry requested
    pub telemetry_request: bool,
    /// Signal valid
    pub valid: bool,
    /// Last command received (DSHOT only)
    pub command: Option<DshotCommand>,
}

impl Default for InputProtocol {
    fn default() -> Self {
        Self::None
    }
}

impl InputSignal {
    /// Create from DSHOT frame
    pub fn from_dshot(frame: &DshotFrame) -> Self {
        Self {
            protocol: InputProtocol::Dshot,
            throttle: frame.duty_cycle(),
            telemetry_request: frame.telemetry,
            valid: frame.valid,
            command: frame.command(),
        }
    }
    
    /// Create from servo pulse
    pub fn from_servo(duty: u16, valid: bool) -> Self {
        Self {
            protocol: InputProtocol::Servo,
            throttle: duty,
            telemetry_request: false,
            valid,
            command: None,
        }
    }
    
    /// Check if motor should be armed
    pub fn should_arm(&self) -> bool {
        self.valid && self.throttle == 0
    }
    
    /// Check if this is a stop command
    pub fn is_stop(&self) -> bool {
        self.valid && self.throttle == 0
    }
}

/// Input signal processor
pub struct SignalProcessor {
    /// Current signal
    current: InputSignal,
    /// Signal timeout counter
    timeout_count: u16,
    /// Armed state
    armed: bool,
    /// Arming counter (must see zero throttle for some time)
    arm_count: u16,
    /// Required arming time (in process cycles)
    arm_time: u16,
}

impl SignalProcessor {
    /// Create a new signal processor
    pub fn new() -> Self {
        Self {
            current: InputSignal::default(),
            timeout_count: 0,
            armed: false,
            arm_count: 0,
            arm_time: 1000, // ~1 second at 1kHz
        }
    }
    
    /// Process a new input signal
    pub fn process(&mut self, signal: InputSignal) {
        self.current = signal;
        
        if signal.valid {
            self.timeout_count = 0;
            
            // Arming logic
            if signal.throttle == 0 {
                self.arm_count = self.arm_count.saturating_add(1);
                if self.arm_count >= self.arm_time {
                    if !self.armed {
                        defmt::info!("ESC armed");
                    }
                    self.armed = true;
                }
            } else {
                self.arm_count = 0;
            }
        } else {
            self.timeout_count = self.timeout_count.saturating_add(1);
        }
    }
    
    /// Check for signal timeout
    pub fn check_timeout(&mut self, timeout_threshold: u16) -> bool {
        if self.timeout_count > timeout_threshold {
            if self.armed {
                defmt::warn!("Signal timeout, disarming");
            }
            self.armed = false;
            self.current.valid = false;
            true
        } else {
            false
        }
    }
    
    /// Get current signal
    pub fn signal(&self) -> &InputSignal {
        &self.current
    }
    
    /// Check if armed
    pub fn is_armed(&self) -> bool {
        self.armed
    }
    
    /// Get throttle (only if armed)
    pub fn throttle(&self) -> u16 {
        if self.armed && self.current.valid {
            self.current.throttle
        } else {
            0
        }
    }
    
    /// Force disarm
    pub fn disarm(&mut self) {
        self.armed = false;
        self.arm_count = 0;
        defmt::info!("ESC disarmed");
    }
    
    /// Set arming time
    pub fn set_arm_time(&mut self, cycles: u16) {
        self.arm_time = cycles;
    }
}

impl Default for SignalProcessor {
    fn default() -> Self {
        Self::new()
    }
}
