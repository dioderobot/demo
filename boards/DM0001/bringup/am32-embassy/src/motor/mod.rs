//! Motor control module
//!
//! Implements 6-step BLDC commutation with ADC-based BEMF sensing.

pub mod commutation;
pub mod pwm;
pub mod bemf;
pub mod startup;

pub use commutation::{CommutationStep, CommutationTable};
pub use pwm::MotorPwm;
pub use bemf::BemfSensor;
pub use startup::StartupSequence;

/// Motor state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum MotorState {
    /// Motor is stopped, all phases floating
    Stopped,
    /// Motor is in startup sequence (open-loop)
    Starting,
    /// Motor is running (closed-loop BEMF commutation)
    Running,
    /// Motor is braking
    Braking,
    /// Motor detected a fault (stall, desync, etc.)
    Fault,
}

/// Motor control structure
pub struct Motor {
    state: MotorState,
    step: CommutationStep,
    duty_cycle: u16,
    target_duty: u16,
    direction: Direction,
    zero_crosses: u32,
    commutation_interval: u32,
}

/// Motor rotation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum Direction {
    Forward,
    Reverse,
}

impl Motor {
    pub fn new() -> Self {
        Self {
            state: MotorState::Stopped,
            step: CommutationStep::Step1,
            duty_cycle: 0,
            target_duty: 0,
            direction: Direction::Forward,
            zero_crosses: 0,
            commutation_interval: 0,
        }
    }
    
    /// Get current motor state
    pub fn state(&self) -> MotorState {
        self.state
    }
    
    /// Get current commutation step
    pub fn step(&self) -> CommutationStep {
        self.step
    }
    
    /// Set target duty cycle (0-2000 range like AM32)
    pub fn set_duty(&mut self, duty: u16) {
        self.target_duty = duty.min(2000);
    }
    
    /// Get current duty cycle
    pub fn duty(&self) -> u16 {
        self.duty_cycle
    }
    
    /// Advance to next commutation step
    pub fn advance_step(&mut self) {
        self.step = self.step.next(self.direction);
        self.zero_crosses = self.zero_crosses.saturating_add(1);
    }
    
    /// Start the motor
    pub fn start(&mut self) {
        if self.state == MotorState::Stopped {
            self.state = MotorState::Starting;
            self.zero_crosses = 0;
            defmt::info!("Motor starting");
        }
    }
    
    /// Stop the motor
    pub fn stop(&mut self) {
        self.state = MotorState::Stopped;
        self.duty_cycle = 0;
        self.target_duty = 0;
        defmt::info!("Motor stopped");
    }
    
    /// Transition from startup to running
    pub fn transition_to_running(&mut self) {
        if self.state == MotorState::Starting {
            self.state = MotorState::Running;
            defmt::info!("Motor now running, zero_crosses={}", self.zero_crosses);
        }
    }
    
    /// Report a fault
    pub fn fault(&mut self) {
        self.state = MotorState::Fault;
        self.duty_cycle = 0;
        defmt::error!("Motor fault!");
    }
    
    /// Update duty cycle with ramping
    pub fn update_duty(&mut self, max_change: u16) {
        if self.duty_cycle < self.target_duty {
            self.duty_cycle = (self.duty_cycle + max_change).min(self.target_duty);
        } else if self.duty_cycle > self.target_duty {
            self.duty_cycle = self.duty_cycle.saturating_sub(max_change).max(self.target_duty);
        }
    }
}

impl Default for Motor {
    fn default() -> Self {
        Self::new()
    }
}
