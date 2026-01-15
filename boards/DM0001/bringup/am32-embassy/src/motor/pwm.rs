//! PWM Driver for 3-Phase Motor Control
//!
//! Uses TIM1 with complementary outputs and dead-time insertion.
//! 
//! Pin mapping for DM0001:
//! - Phase A: PA8 (CH1) / PC13 (CH1N)
//! - Phase B: PA9 (CH2) / PA12 (CH2N)
//! - Phase C: PA10 (CH3) / PB15 (CH3N)

use super::commutation::{CommutationStep, CommutationTable, PhaseState};

/// PWM frequency in Hz (24kHz is typical for ESCs)
pub const PWM_FREQUENCY_HZ: u32 = 24_000;

/// Dead time in nanoseconds
pub const DEAD_TIME_NS: u32 = 500;

/// Maximum duty cycle value
pub const MAX_DUTY: u16 = 2000;

/// Motor PWM controller
/// 
/// This is a software abstraction. The actual hardware initialization
/// requires passing the TIM1 peripheral and GPIO pins.
pub struct MotorPwm {
    /// Current duty cycle (0 to max_duty)
    duty: u16,
    /// Maximum duty (timer period)
    max_duty: u16,
    /// Current phase states
    phase_states: [PhaseState; 3],
}

impl MotorPwm {
    /// Create a new motor PWM controller (software only)
    pub fn new() -> Self {
        defmt::info!("PWM controller created (software mode)");

        Self {
            duty: 0,
            max_duty: MAX_DUTY,
            phase_states: [PhaseState::Float; 3],
        }
    }

    /// Set the duty cycle (0 to MAX_DUTY)
    pub fn set_duty(&mut self, duty: u16) {
        self.duty = duty.min(MAX_DUTY);
        defmt::trace!("PWM duty: {}", self.duty);
    }

    /// Get current duty cycle
    pub fn duty(&self) -> u16 {
        self.duty
    }

    /// Apply a commutation step
    pub fn apply_step(&mut self, step: CommutationStep) {
        let entry = CommutationTable::get(step);
        self.set_phase_states(entry.phase_a, entry.phase_b, entry.phase_c);
        defmt::trace!("Applied step {}", step.number());
    }

    /// Set individual phase states
    pub fn set_phase_states(&mut self, a: PhaseState, b: PhaseState, c: PhaseState) {
        self.phase_states = [a, b, c];
        
        // In a real implementation, this would:
        // 1. Configure TIM1 CCR registers for duty
        // 2. Enable/disable outputs via CCER register
        // 3. Use MOE bit in BDTR for master output enable
    }

    /// Turn all phases off (floating)
    pub fn all_off(&mut self) {
        self.set_phase_states(PhaseState::Float, PhaseState::Float, PhaseState::Float);
        defmt::debug!("All phases off");
    }

    /// Full brake (all low sides on)
    pub fn full_brake(&mut self) {
        self.set_phase_states(PhaseState::Low, PhaseState::Low, PhaseState::Low);
        defmt::debug!("Full brake applied");
    }

    /// Proportional brake
    pub fn proportional_brake(&mut self, strength: u16) {
        self.duty = strength.min(MAX_DUTY);
        defmt::debug!("Proportional brake at {}", strength);
    }

    /// Get maximum duty value
    pub fn max_duty(&self) -> u16 {
        self.max_duty
    }

    /// Get phase states
    pub fn phase_states(&self) -> &[PhaseState; 3] {
        &self.phase_states
    }
}

impl Default for MotorPwm {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate timer prescaler and ARR for desired PWM frequency
pub fn calculate_timer_params(sysclk_hz: u32, pwm_freq_hz: u32) -> (u16, u16) {
    // TIM1 is on APB2, assume no prescaler on APB2
    let timer_clk = sysclk_hz;
    
    // For center-aligned mode, effective frequency is halved
    // pwm_freq = timer_clk / (2 * (ARR + 1))
    let arr = (timer_clk / (2 * pwm_freq_hz)) - 1;
    
    if arr <= 65535 {
        (0, arr as u16)
    } else {
        // Need prescaler
        let psc = (arr / 65536) as u16;
        let arr = (timer_clk / (2 * (psc as u32 + 1) * pwm_freq_hz)) - 1;
        (psc, arr as u16)
    }
}

/// Calculate dead time register value
/// 
/// For STM32G4, dead time is configured in TIM1_BDTR register.
pub fn calculate_dead_time(timer_clk_hz: u32, dead_time_ns: u32) -> u8 {
    let t_dts = 1_000_000_000 / timer_clk_hz; // DTS period in ns
    let dt_clocks = dead_time_ns / t_dts;
    
    // Simplified: assume DT[7:5] = 0xx (DT = DTG[7:0] * t_DTS)
    (dt_clocks as u8).min(127)
}

// Note: Full hardware PWM implementation requires:
// 
// ```rust
// use embassy_stm32::timer::complementary_pwm::ComplementaryPwm;
// use embassy_stm32::timer::simple_pwm::PwmPin;
// use embassy_stm32::timer::complementary_pwm::ComplementaryPwmPin;
// 
// // Create PWM pins
// let ch1 = PwmPin::new(pa8, OutputType::PushPull);
// let ch1n = ComplementaryPwmPin::new(pc13, OutputType::PushPull);
// // ... etc for ch2, ch2n, ch3, ch3n
// 
// // Create complementary PWM
// let pwm = ComplementaryPwm::new(
//     tim1,
//     Some(ch1), Some(ch1n),
//     Some(ch2), Some(ch2n),
//     Some(ch3), Some(ch3n),
//     None, None,
//     khz(24),
//     CountingMode::CenterAlignedBothInterrupts,
// );
// ```
