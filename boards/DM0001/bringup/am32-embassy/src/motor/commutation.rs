//! 6-Step Commutation Table
//!
//! Implements the standard 6-step commutation sequence for BLDC motors.
//! Each step defines which phases are PWM, LOW, or FLOAT.

use super::Direction;

/// Commutation step (1-6)
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
#[repr(u8)]
pub enum CommutationStep {
    Step1 = 1, // A-B: A=PWM, B=LOW, C=FLOAT
    Step2 = 2, // C-B: A=FLOAT, B=LOW, C=PWM
    Step3 = 3, // C-A: A=LOW, B=FLOAT, C=PWM
    Step4 = 4, // B-A: A=LOW, B=PWM, C=FLOAT
    Step5 = 5, // B-C: A=FLOAT, B=PWM, C=LOW
    Step6 = 6, // A-C: A=PWM, B=FLOAT, C=LOW
}

impl CommutationStep {
    /// Get the next step in the sequence
    pub fn next(self, direction: Direction) -> Self {
        match direction {
            Direction::Forward => match self {
                Self::Step1 => Self::Step2,
                Self::Step2 => Self::Step3,
                Self::Step3 => Self::Step4,
                Self::Step4 => Self::Step5,
                Self::Step5 => Self::Step6,
                Self::Step6 => Self::Step1,
            },
            Direction::Reverse => match self {
                Self::Step1 => Self::Step6,
                Self::Step2 => Self::Step1,
                Self::Step3 => Self::Step2,
                Self::Step4 => Self::Step3,
                Self::Step5 => Self::Step4,
                Self::Step6 => Self::Step5,
            },
        }
    }
    
    /// Get the previous step in the sequence
    pub fn prev(self, direction: Direction) -> Self {
        self.next(match direction {
            Direction::Forward => Direction::Reverse,
            Direction::Reverse => Direction::Forward,
        })
    }
    
    /// Get step from number (1-6)
    pub fn from_number(n: u8) -> Option<Self> {
        match n {
            1 => Some(Self::Step1),
            2 => Some(Self::Step2),
            3 => Some(Self::Step3),
            4 => Some(Self::Step4),
            5 => Some(Self::Step5),
            6 => Some(Self::Step6),
            _ => None,
        }
    }
    
    /// Get step number (1-6)
    pub fn number(self) -> u8 {
        self as u8
    }
    
    /// Get which phase is floating in this step
    pub fn floating_phase(self) -> Phase {
        match self {
            Self::Step1 | Self::Step4 => Phase::C,
            Self::Step2 | Self::Step5 => Phase::A,
            Self::Step3 | Self::Step6 => Phase::B,
        }
    }
    
    /// Is BEMF rising on the floating phase?
    pub fn bemf_rising(self) -> bool {
        match self {
            Self::Step1 | Self::Step3 | Self::Step5 => false, // Falling BEMF
            Self::Step2 | Self::Step4 | Self::Step6 => true,  // Rising BEMF
        }
    }
}

/// Motor phase identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum Phase {
    A,
    B,
    C,
}

/// Phase output state
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum PhaseState {
    /// Phase is driven with PWM (high-side switching)
    Pwm,
    /// Phase low-side is ON (sinking current)
    Low,
    /// Phase is floating (both switches off)
    Float,
}

/// Commutation table entry
#[derive(Debug, Clone, Copy)]
pub struct CommutationEntry {
    pub phase_a: PhaseState,
    pub phase_b: PhaseState,
    pub phase_c: PhaseState,
}

/// Static commutation table
pub struct CommutationTable;

impl CommutationTable {
    /// Get the phase states for a given commutation step
    pub fn get(step: CommutationStep) -> CommutationEntry {
        match step {
            CommutationStep::Step1 => CommutationEntry {
                phase_a: PhaseState::Pwm,
                phase_b: PhaseState::Low,
                phase_c: PhaseState::Float,
            },
            CommutationStep::Step2 => CommutationEntry {
                phase_a: PhaseState::Float,
                phase_b: PhaseState::Low,
                phase_c: PhaseState::Pwm,
            },
            CommutationStep::Step3 => CommutationEntry {
                phase_a: PhaseState::Low,
                phase_b: PhaseState::Float,
                phase_c: PhaseState::Pwm,
            },
            CommutationStep::Step4 => CommutationEntry {
                phase_a: PhaseState::Low,
                phase_b: PhaseState::Pwm,
                phase_c: PhaseState::Float,
            },
            CommutationStep::Step5 => CommutationEntry {
                phase_a: PhaseState::Float,
                phase_b: PhaseState::Pwm,
                phase_c: PhaseState::Low,
            },
            CommutationStep::Step6 => CommutationEntry {
                phase_a: PhaseState::Pwm,
                phase_b: PhaseState::Float,
                phase_c: PhaseState::Low,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_step_sequence_forward() {
        let mut step = CommutationStep::Step1;
        for expected in [2, 3, 4, 5, 6, 1] {
            step = step.next(Direction::Forward);
            assert_eq!(step.number(), expected);
        }
    }
    
    #[test]
    fn test_step_sequence_reverse() {
        let mut step = CommutationStep::Step1;
        for expected in [6, 5, 4, 3, 2, 1] {
            step = step.next(Direction::Reverse);
            assert_eq!(step.number(), expected);
        }
    }
}
