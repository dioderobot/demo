//! Motor Startup Sequence
//!
//! Implements open-loop startup to get the motor spinning before
//! transitioning to closed-loop BEMF commutation.

use super::commutation::CommutationStep;
use super::Direction;

/// Startup mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum StartupMode {
    /// Standard ramp-up startup
    Standard,
    /// Sinusoidal startup for smoother operation (larger motors)
    Sinusoidal,
}

/// Startup sequence state
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum StartupState {
    /// Not started
    Idle,
    /// Aligning rotor to known position
    Align,
    /// Ramping up speed in open-loop
    Ramp,
    /// Transitioning to closed-loop
    Transition,
    /// Startup complete
    Complete,
    /// Startup failed
    Failed,
}

/// Motor startup sequence controller
pub struct StartupSequence {
    state: StartupState,
    mode: StartupMode,
    step: CommutationStep,
    direction: Direction,
    
    /// Current step interval (timer ticks)
    step_interval: u32,
    /// Target step interval for transition
    target_interval: u32,
    /// Minimum step interval (maximum speed during startup)
    min_interval: u32,
    
    /// Current duty cycle during startup
    duty: u16,
    /// Maximum duty during startup
    max_startup_duty: u16,
    
    /// Step counter
    step_count: u32,
    /// Zero-crossing counter (for transition detection)
    good_crossings: u32,
    /// Required good crossings before transition
    required_crossings: u32,
    
    /// Align time in milliseconds
    align_time_ms: u16,
    /// Align duty cycle
    align_duty: u16,
}

impl StartupSequence {
    /// Create a new startup sequence
    pub fn new() -> Self {
        Self {
            state: StartupState::Idle,
            mode: StartupMode::Standard,
            step: CommutationStep::Step1,
            direction: Direction::Forward,
            
            step_interval: 10000, // Start slow
            target_interval: 1000, // Target for transition
            min_interval: 500,    // Don't go faster than this
            
            duty: 0,
            max_startup_duty: 350, // ~17.5% max during startup
            
            step_count: 0,
            good_crossings: 0,
            required_crossings: 10,
            
            align_time_ms: 50,
            align_duty: 100,
        }
    }
    
    /// Start the startup sequence
    pub fn start(&mut self, direction: Direction) {
        self.state = StartupState::Align;
        self.direction = direction;
        self.step = CommutationStep::Step1;
        self.step_count = 0;
        self.good_crossings = 0;
        self.step_interval = 10000;
        self.duty = self.align_duty;
        
        defmt::info!("Startup sequence started, direction={:?}", direction);
    }
    
    /// Reset the startup sequence
    pub fn reset(&mut self) {
        self.state = StartupState::Idle;
        self.step_count = 0;
        self.good_crossings = 0;
        self.duty = 0;
    }
    
    /// Get current state
    pub fn state(&self) -> StartupState {
        self.state
    }
    
    /// Get current step
    pub fn step(&self) -> CommutationStep {
        self.step
    }
    
    /// Get current duty cycle
    pub fn duty(&self) -> u16 {
        self.duty
    }
    
    /// Get current step interval
    pub fn interval(&self) -> u32 {
        self.step_interval
    }
    
    /// Check if startup is complete
    pub fn is_complete(&self) -> bool {
        self.state == StartupState::Complete
    }
    
    /// Check if startup failed
    pub fn is_failed(&self) -> bool {
        self.state == StartupState::Failed
    }
    
    /// Advance to next step (called by timer)
    pub fn advance_step(&mut self) {
        self.step = self.step.next(self.direction);
        self.step_count += 1;
    }
    
    /// Update the startup sequence (called periodically)
    /// 
    /// Returns true if a commutation should occur
    pub fn update(&mut self, elapsed_ticks: u32, bemf_detected: bool) -> bool {
        match self.state {
            StartupState::Idle => false,
            
            StartupState::Align => {
                // Hold position for alignment
                // Transition to ramp after align_time
                if self.step_count > 0 {
                    self.state = StartupState::Ramp;
                    self.duty = self.max_startup_duty / 4; // Start with low duty
                    defmt::debug!("Align complete, starting ramp");
                }
                self.step_count += 1;
                false
            }
            
            StartupState::Ramp => {
                // Check if it's time to commutate
                if elapsed_ticks >= self.step_interval {
                    // Gradually decrease interval (increase speed)
                    if self.step_interval > self.target_interval {
                        self.step_interval = self.step_interval.saturating_sub(
                            self.step_interval / 20 // 5% decrease per step
                        ).max(self.min_interval);
                    }
                    
                    // Gradually increase duty
                    if self.duty < self.max_startup_duty {
                        self.duty = (self.duty + 5).min(self.max_startup_duty);
                    }
                    
                    // Check for BEMF detection
                    if bemf_detected {
                        self.good_crossings += 1;
                        if self.good_crossings >= self.required_crossings {
                            self.state = StartupState::Transition;
                            defmt::info!("BEMF detected, transitioning to closed-loop");
                        }
                    } else {
                        self.good_crossings = 0;
                    }
                    
                    // Check for timeout/failure
                    if self.step_count > 500 && self.good_crossings < 5 {
                        self.state = StartupState::Failed;
                        defmt::error!("Startup failed: no BEMF detected");
                        return false;
                    }
                    
                    return true; // Commutate
                }
                false
            }
            
            StartupState::Transition => {
                // Brief transition period
                if self.good_crossings >= self.required_crossings + 5 {
                    self.state = StartupState::Complete;
                    defmt::info!("Startup complete after {} steps", self.step_count);
                }
                
                if bemf_detected {
                    self.good_crossings += 1;
                    return true; // Commutate on BEMF
                }
                false
            }
            
            StartupState::Complete | StartupState::Failed => false,
        }
    }
    
    /// Set startup parameters
    pub fn configure(
        &mut self,
        max_duty: u16,
        target_interval: u32,
        required_crossings: u32,
    ) {
        self.max_startup_duty = max_duty;
        self.target_interval = target_interval;
        self.required_crossings = required_crossings;
    }
    
    /// Set startup mode
    pub fn set_mode(&mut self, mode: StartupMode) {
        self.mode = mode;
    }
}

impl Default for StartupSequence {
    fn default() -> Self {
        Self::new()
    }
}
