//! Analog sensing module
//!
//! Handles voltage, current, and temperature measurement.

pub mod adc;

pub use adc::{AdcSensing, SensorReadings};
