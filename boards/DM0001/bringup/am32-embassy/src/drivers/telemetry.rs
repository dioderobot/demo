//! Telemetry Output Driver
//!
//! Supports multiple telemetry protocols:
//! - Bidirectional DSHOT (via input pin)
//! - KISS ESC telemetry (UART)
//! - BLHeli32 telemetry

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

/// Telemetry data signal
pub static TELEMETRY_SIGNAL: Signal<CriticalSectionRawMutex, TelemetryData> = Signal::new();

/// Telemetry data packet
#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct TelemetryData {
    /// Electrical RPM (eRPM)
    pub erpm: u32,
    /// Bus voltage in millivolts
    pub voltage_mv: u16,
    /// Current in milliamps
    pub current_ma: u16,
    /// Temperature in degrees Celsius
    pub temperature_c: i8,
    /// Consumption in mAh
    pub consumption_mah: u16,
    /// Debug value
    pub debug: u16,
}

impl TelemetryData {
    /// Create new telemetry data
    pub fn new() -> Self {
        Self::default()
    }

    /// Update from sensor readings
    pub fn update(&mut self, erpm: u32, voltage_mv: u16, current_ma: u16, temp_c: i8) {
        self.erpm = erpm;
        self.voltage_mv = voltage_mv;
        self.current_ma = current_ma;
        self.temperature_c = temp_c;
    }

    /// Add to consumption counter
    pub fn add_consumption(&mut self, current_ma: u16, dt_ms: u16) {
        // mAh = mA * hours = mA * ms / 3600000
        let mah_increment = (current_ma as u32 * dt_ms as u32) / 3_600_000;
        self.consumption_mah = self.consumption_mah.saturating_add(mah_increment as u16);
    }

    /// Encode for bidirectional DSHOT response
    /// 
    /// Returns 20-bit GCR encoded value
    pub fn encode_dshot_response(&self) -> u32 {
        // Encode eRPM using AM32's method
        let erpm = self.erpm.min(0xFFFF) as u16;
        
        // Calculate shift amount for eRPM encoding
        let mut shift_amount = 0u8;
        for i in (9..=15).rev() {
            if (erpm >> i) == 1 {
                shift_amount = (i + 1 - 9) as u8;
                break;
            }
        }
        
        // Encode: eee mmm mmm mmm (3-bit exponent, 9-bit mantissa)
        let encoded = ((shift_amount as u16) << 9) | (erpm >> shift_amount);
        
        // Calculate CRC
        let mut csum: u16 = 0;
        let mut csum_data = encoded;
        for _ in 0..3 {
            csum ^= csum_data;
            csum_data >>= 4;
        }
        csum = !csum & 0x0F;
        
        let full_number = (encoded << 4) | csum;
        
        // GCR encode
        gcr_encode(full_number)
    }

    /// Encode for KISS ESC telemetry (10 bytes)
    pub fn encode_kiss(&self) -> [u8; 10] {
        let mut buf = [0u8; 10];
        
        // Temperature (°C)
        buf[0] = (self.temperature_c.max(0) as u8).saturating_add(40);
        
        // Voltage (0.01V units)
        let voltage_cv = self.voltage_mv / 10;
        buf[1] = (voltage_cv >> 8) as u8;
        buf[2] = voltage_cv as u8;
        
        // Current (0.01A units)
        let current_ca = self.current_ma / 10;
        buf[3] = (current_ca >> 8) as u8;
        buf[4] = current_ca as u8;
        
        // Consumption (mAh)
        buf[5] = (self.consumption_mah >> 8) as u8;
        buf[6] = self.consumption_mah as u8;
        
        // eRPM (100 RPM units)
        let erpm_100 = (self.erpm / 100) as u16;
        buf[7] = (erpm_100 >> 8) as u8;
        buf[8] = erpm_100 as u8;
        
        // CRC8
        buf[9] = crc8_kiss(&buf[..9]);
        
        buf
    }
}

/// GCR encoding table for DSHOT telemetry
const GCR_ENCODE_TABLE: [u8; 16] = [
    0b11001, 0b11011, 0b10010, 0b10011,
    0b11101, 0b10101, 0b10110, 0b10111,
    0b11010, 0b01001, 0b01010, 0b01011,
    0b11110, 0b01101, 0b01110, 0b01111,
];

/// GCR encode a 16-bit value to 20-bit
fn gcr_encode(value: u16) -> u32 {
    let gcr = (GCR_ENCODE_TABLE[(value >> 12) as usize] as u32) << 15
        | (GCR_ENCODE_TABLE[((value >> 8) & 0x0F) as usize] as u32) << 10
        | (GCR_ENCODE_TABLE[((value >> 4) & 0x0F) as usize] as u32) << 5
        | (GCR_ENCODE_TABLE[(value & 0x0F) as usize] as u32);
    gcr
}

/// CRC8 for KISS telemetry
fn crc8_kiss(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0xD5;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Extended telemetry types for bidirectional DSHOT
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
#[repr(u8)]
pub enum ExtendedTelemetryType {
    Temperature = 0x02,
    Voltage = 0x04,
    Current = 0x06,
    Debug1 = 0x08,
    Debug2 = 0x0A,
    Debug3 = 0x0C,
    State = 0x0E,
}

/// Telemetry output mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum TelemetryMode {
    /// No telemetry
    Disabled,
    /// Bidirectional DSHOT
    BidirectionalDshot,
    /// KISS ESC protocol
    Kiss,
    /// BLHeli32 protocol
    BLHeli32,
}

/// Telemetry controller
pub struct TelemetryController {
    mode: TelemetryMode,
    data: TelemetryData,
    /// Extended telemetry enabled
    extended_enabled: bool,
    /// Current extended telemetry type to send
    extended_type: ExtendedTelemetryType,
    /// Telemetry request pending
    request_pending: bool,
}

impl TelemetryController {
    /// Create new telemetry controller
    pub fn new() -> Self {
        Self {
            mode: TelemetryMode::Disabled,
            data: TelemetryData::default(),
            extended_enabled: false,
            extended_type: ExtendedTelemetryType::Temperature,
            request_pending: false,
        }
    }

    /// Set telemetry mode
    pub fn set_mode(&mut self, mode: TelemetryMode) {
        self.mode = mode;
        defmt::info!("Telemetry mode: {:?}", mode);
    }

    /// Update telemetry data
    pub fn update(&mut self, data: TelemetryData) {
        self.data = data;
    }

    /// Request telemetry transmission
    pub fn request(&mut self) {
        self.request_pending = true;
    }

    /// Check if telemetry is pending
    pub fn is_pending(&self) -> bool {
        self.request_pending
    }

    /// Get response for bidirectional DSHOT
    pub fn get_dshot_response(&mut self) -> u32 {
        self.request_pending = false;
        
        if self.extended_enabled {
            // Send extended telemetry
            let value = match self.extended_type {
                ExtendedTelemetryType::Temperature => {
                    (self.data.temperature_c.max(0) as u16).saturating_add(40)
                }
                ExtendedTelemetryType::Voltage => {
                    self.data.voltage_mv / 10 // 0.01V units
                }
                ExtendedTelemetryType::Current => {
                    self.data.current_ma / 10 // 0.01A units
                }
                _ => self.data.debug,
            };
            
            // Rotate to next type
            self.extended_type = match self.extended_type {
                ExtendedTelemetryType::Temperature => ExtendedTelemetryType::Voltage,
                ExtendedTelemetryType::Voltage => ExtendedTelemetryType::Current,
                ExtendedTelemetryType::Current => ExtendedTelemetryType::Temperature,
                _ => ExtendedTelemetryType::Temperature,
            };
            
            // Encode extended telemetry
            let encoded = ((self.extended_type as u16) << 8) | (value & 0xFF);
            gcr_encode(encoded)
        } else {
            // Send eRPM
            self.data.encode_dshot_response()
        }
    }

    /// Enable extended telemetry
    pub fn enable_extended(&mut self) {
        self.extended_enabled = true;
        defmt::info!("Extended telemetry enabled");
    }

    /// Disable extended telemetry
    pub fn disable_extended(&mut self) {
        self.extended_enabled = false;
        defmt::info!("Extended telemetry disabled");
    }

    /// Get current telemetry data
    pub fn data(&self) -> &TelemetryData {
        &self.data
    }
}

impl Default for TelemetryController {
    fn default() -> Self {
        Self::new()
    }
}
