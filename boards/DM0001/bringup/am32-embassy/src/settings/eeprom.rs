//! Flash-based EEPROM emulation
//!
//! Stores settings in the last page of flash memory with wear leveling.

use super::Settings;

/// Flash page size for STM32G4 (2KB)
const FLASH_PAGE_SIZE: usize = 2048;

/// EEPROM storage in flash
pub struct EepromStorage {
    /// Current settings
    settings: Settings,
    /// Settings modified flag
    modified: bool,
    /// Write counter for wear leveling
    write_count: u32,
}

impl EepromStorage {
    /// Create a new EEPROM storage
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            modified: false,
            write_count: 0,
        }
    }
    
    /// Load settings from flash
    /// 
    /// Returns default settings if flash is empty or corrupted.
    pub fn load(&mut self) -> &Settings {
        // TODO: Implement actual flash reading
        // For now, return defaults
        defmt::info!("Loading settings from flash (using defaults)");
        &self.settings
    }
    
    /// Save settings to flash
    pub fn save(&mut self) -> Result<(), EepromError> {
        if !self.modified {
            return Ok(());
        }
        
        // TODO: Implement actual flash writing
        // 1. Erase flash page
        // 2. Write settings with CRC
        // 3. Increment write counter
        
        defmt::info!("Saving settings to flash");
        self.modified = false;
        self.write_count += 1;
        
        Ok(())
    }
    
    /// Get current settings
    pub fn settings(&self) -> &Settings {
        &self.settings
    }
    
    /// Get mutable settings (marks as modified)
    pub fn settings_mut(&mut self) -> &mut Settings {
        self.modified = true;
        &mut self.settings
    }
    
    /// Update a setting
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Settings),
    {
        f(&mut self.settings);
        self.modified = true;
    }
    
    /// Check if settings have been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    /// Get write count
    pub fn write_count(&self) -> u32 {
        self.write_count
    }
    
    /// Reset to defaults
    pub fn reset_defaults(&mut self) {
        self.settings = Settings::default();
        self.modified = true;
    }
}

impl Default for EepromStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// EEPROM error types
#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum EepromError {
    /// Flash erase failed
    EraseFailed,
    /// Flash write failed
    WriteFailed,
    /// Flash read failed
    ReadFailed,
    /// CRC mismatch
    CrcError,
    /// Flash is locked
    Locked,
}

/// Calculate CRC-8 for settings validation
pub fn calculate_crc(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xFF;
    for byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}
