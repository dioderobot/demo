//! Input signal handling
//!
//! Supports DSHOT and PWM servo input protocols.

pub mod dshot;
pub mod servo;
pub mod signal;

pub use dshot::{DshotFrame, DshotCommand, DshotReceiver};
pub use servo::ServoInput;
pub use signal::{InputSignal, InputProtocol};
