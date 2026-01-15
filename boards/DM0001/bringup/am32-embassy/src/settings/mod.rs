//! Settings and EEPROM storage
//!
//! Flash-based persistent settings storage.

pub mod eeprom;
pub mod defaults;

pub use eeprom::EepromStorage;
pub use defaults::Settings;
