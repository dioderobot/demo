//! Motor Controller Integration
//!
//! Integrates all motor control components into a unified state machine.

use crate::motor::commutation::CommutationStep;
use crate::motor::bemf::{BemfSensor, BemfSamples, BemfTiming};
use crate::motor::startup::StartupSequence;
use crate::motor::{Direction, MotorState};
use crate::settings::Settings;

/// Motor controller configuration
#[derive(Debug, Clone, Copy)]
pub struct MotorConfig {
    /// Startup power (0-2000)
    pub startup_power: u16,
    /// Maximum power (0-2000)
    pub max_power: u16,
    /// Timing advance in degrees
    pub timing_advance: u8,
    /// Stall protection enabled
    pub stall_protection: bool,
    /// Stall timeout in control loop cycles
    pub stall_timeout: u32,
    /// Minimum commutation interval (maximum speed limit)
    pub min_interval: u32,
    /// Duty ramp rate per control loop cycle
    pub ramp_rate: u16,
}

impl Default for MotorConfig {
    fn default() -> Self {
        Self {
            startup_power: 350,
            max_power: 2000,
            timing_advance: 22,
            stall_protection: true,
            stall_timeout: 10000, // ~500ms at 20kHz
            min_interval: 100,    // Maximum ~100k eRPM
            ramp_rate: 5,
        }
    }
}

impl From<&Settings> for MotorConfig {
    fn from(settings: &Settings) -> Self {
        Self {
            startup_power: settings.startup_power as u16 * 20, // 0-100% -> 0-2000
            max_power: 2000,
            timing_advance: settings.timing_advance,
            stall_protection: settings.stall_protection,
            stall_timeout: 10000,
            min_interval: 100,
            ramp_rate: 5,
        }
    }
}

/// Motor controller state machine
pub struct MotorController {
    /// Current state
    state: MotorState,
    /// Current commutation step
    step: CommutationStep,
    /// Motor direction
    direction: Direction,
    /// Current duty cycle
    duty: u16,
    /// Target duty cycle
    target_duty: u16,
    /// BEMF sensor
    bemf: BemfSensor,
    /// BEMF timing
    timing: BemfTiming,
    /// Startup sequence
    startup: StartupSequence,
    /// Configuration
    config: MotorConfig,
    /// Control loop counter
    loop_count: u32,
    /// Last commutation time
    last_commutation: u32,
    /// Stall counter
    stall_count: u32,
    /// Zero crossing counter
    zc_count: u32,
    /// Good zero crossings (for sync detection)
    good_zc: u32,
    /// Electrical RPM
    erpm: u32,
}

impl MotorController {
    /// Create a new motor controller
    pub fn new(config: MotorConfig) -> Self {
        Self {
            state: MotorState::Stopped,
            step: CommutationStep::Step1,
            direction: Direction::Forward,
            duty: 0,
            target_duty: 0,
            bemf: BemfSensor::new(),
            timing: BemfTiming::new(),
            startup: StartupSequence::new(),
            config,
            loop_count: 0,
            last_commutation: 0,
            stall_count: 0,
            zc_count: 0,
            good_zc: 0,
            erpm: 0,
        }
    }

    /// Run one iteration of the control loop
    /// 
    /// Returns true if commutation should occur
    pub fn update(&mut self, bemf_samples: BemfSamples) -> ControlAction {
        self.loop_count = self.loop_count.wrapping_add(1);
        self.bemf.update_samples(bemf_samples);

        match self.state {
            MotorState::Stopped => self.update_stopped(),
            MotorState::Starting => self.update_starting(),
            MotorState::Running => self.update_running(),
            MotorState::Braking => self.update_braking(),
            MotorState::Fault => ControlAction::None,
        }
    }

    fn update_stopped(&mut self) -> ControlAction {
        self.duty = 0;
        if self.target_duty > 0 {
            self.start();
            return ControlAction::Start;
        }
        ControlAction::None
    }

    fn update_starting(&mut self) -> ControlAction {
        // Check for BEMF detection
        let bemf_detected = self.check_bemf();
        
        // Update startup sequence
        let elapsed = self.loop_count.wrapping_sub(self.last_commutation);
        if self.startup.update(elapsed, bemf_detected) {
            // Commutation requested
            self.last_commutation = self.loop_count;
            self.step = self.startup.step();
            self.duty = self.startup.duty();
            
            if self.startup.is_complete() {
                self.state = MotorState::Running;
                defmt::info!("Motor running, eRPM={}", self.erpm);
                return ControlAction::Commutate(self.step);
            }
            
            self.startup.advance_step();
            return ControlAction::Commutate(self.step);
        }
        
        if self.startup.is_failed() {
            self.fault();
            return ControlAction::Fault;
        }
        
        ControlAction::None
    }

    fn update_running(&mut self) -> ControlAction {
        // Update duty with ramping
        self.update_duty_ramp();
        
        // Check for BEMF zero crossing
        let rising = self.step.bemf_rising();
        if self.bemf.detect_crossing(rising) {
            // Zero crossing detected
            self.timing.record_crossing(self.loop_count);
            self.zc_count += 1;
            self.good_zc += 1;
            self.stall_count = 0;
            
            // Calculate advance timing
            let advance_delay = self.timing.advance_timing(self.config.timing_advance);
            
            // For now, commutate immediately (TODO: implement advance timing)
            self.commutate();
            
            // Update eRPM
            self.erpm = self.timing.calculate_rpm(20_000); // 20kHz loop
            
            return ControlAction::Commutate(self.step);
        }
        
        // Stall detection
        self.stall_count += 1;
        if self.config.stall_protection && self.stall_count > self.config.stall_timeout {
            defmt::warn!("Stall detected!");
            self.fault();
            return ControlAction::Fault;
        }
        
        ControlAction::None
    }

    fn update_braking(&mut self) -> ControlAction {
        // Ramp down duty
        if self.duty > 0 {
            self.duty = self.duty.saturating_sub(self.config.ramp_rate * 2);
        } else {
            self.state = MotorState::Stopped;
        }
        ControlAction::UpdateDuty(self.duty)
    }

    fn check_bemf(&mut self) -> bool {
        let rising = self.step.bemf_rising();
        self.bemf.set_floating_phase(self.step.floating_phase());
        self.bemf.get_crossing_state(rising)
    }

    fn commutate(&mut self) {
        self.step = self.step.next(self.direction);
        self.bemf.set_floating_phase(self.step.floating_phase());
        self.last_commutation = self.loop_count;
    }

    fn update_duty_ramp(&mut self) {
        if self.duty < self.target_duty {
            self.duty = (self.duty + self.config.ramp_rate).min(self.target_duty);
        } else if self.duty > self.target_duty {
            self.duty = self.duty.saturating_sub(self.config.ramp_rate);
        }
    }

    /// Start the motor
    pub fn start(&mut self) {
        if self.state == MotorState::Stopped {
            self.state = MotorState::Starting;
            self.startup.start(self.direction);
            self.step = CommutationStep::Step1;
            self.stall_count = 0;
            self.zc_count = 0;
            self.good_zc = 0;
            defmt::info!("Motor starting");
        }
    }

    /// Stop the motor
    pub fn stop(&mut self) {
        self.target_duty = 0;
        if self.state == MotorState::Running {
            self.state = MotorState::Braking;
        } else {
            self.state = MotorState::Stopped;
            self.duty = 0;
        }
        defmt::info!("Motor stopping");
    }

    /// Emergency stop (immediate)
    pub fn emergency_stop(&mut self) {
        self.state = MotorState::Stopped;
        self.duty = 0;
        self.target_duty = 0;
        defmt::warn!("Emergency stop!");
    }

    /// Set fault state
    pub fn fault(&mut self) {
        self.state = MotorState::Fault;
        self.duty = 0;
        self.target_duty = 0;
        defmt::error!("Motor fault!");
    }

    /// Clear fault and return to stopped state
    pub fn clear_fault(&mut self) {
        if self.state == MotorState::Fault {
            self.state = MotorState::Stopped;
            self.startup.reset();
            defmt::info!("Fault cleared");
        }
    }

    /// Set target duty cycle
    pub fn set_throttle(&mut self, throttle: u16) {
        self.target_duty = throttle.min(self.config.max_power);
        
        if throttle > 0 && self.state == MotorState::Stopped {
            self.start();
        } else if throttle == 0 && self.state == MotorState::Running {
            self.stop();
        }
    }

    /// Set motor direction
    pub fn set_direction(&mut self, direction: Direction) {
        if self.state == MotorState::Stopped {
            self.direction = direction;
        }
    }

    /// Get current state
    pub fn state(&self) -> MotorState {
        self.state
    }

    /// Get current duty cycle
    pub fn duty(&self) -> u16 {
        self.duty
    }

    /// Get current step
    pub fn step(&self) -> CommutationStep {
        self.step
    }

    /// Get electrical RPM
    pub fn erpm(&self) -> u32 {
        self.erpm
    }

    /// Get zero crossing count
    pub fn zc_count(&self) -> u32 {
        self.zc_count
    }

    /// Update configuration
    pub fn set_config(&mut self, config: MotorConfig) {
        self.config = config;
    }
}

/// Control action to take after update
#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum ControlAction {
    /// No action needed
    None,
    /// Start motor (apply first commutation step)
    Start,
    /// Commutate to specified step
    Commutate(CommutationStep),
    /// Update duty cycle only
    UpdateDuty(u16),
    /// Motor fault occurred
    Fault,
}
