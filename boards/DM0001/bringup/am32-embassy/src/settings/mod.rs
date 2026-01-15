//! Settings and EEPROM storage
//!
//! Flash-based persistent settings storage.

#![allow(dead_code)]

pub mod eeprom;
pub mod defaults;

pub use defaults::Settings;
