//! DSHOT Protocol Implementation
//!
//! DSHOT is a digital protocol for ESC communication that provides:
//! - 11-bit throttle value (0-2047)
//! - Telemetry request bit
//! - 4-bit CRC
//!
//! Supported variants: DSHOT150, DSHOT300, DSHOT600

/// DSHOT protocol variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum DshotVariant {
    Dshot150,
    Dshot300,
    Dshot600,
}

impl DshotVariant {
    /// Get the bit period in nanoseconds
    pub fn bit_period_ns(&self) -> u32 {
        match self {
            Self::Dshot150 => 6670,  // 150kbit/s
            Self::Dshot300 => 3330,  // 300kbit/s
            Self::Dshot600 => 1670,  // 600kbit/s
        }
    }
    
    /// Get the frame time in microseconds
    pub fn frame_time_us(&self) -> u32 {
        // 16 bits * bit_period
        (16 * self.bit_period_ns()) / 1000
    }
}

/// Decoded DSHOT frame
#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct DshotFrame {
    /// Throttle value (0-2047, where 48-2047 is the usable range)
    pub throttle: u16,
    /// Telemetry request flag
    pub telemetry: bool,
    /// CRC valid flag
    pub valid: bool,
}

impl DshotFrame {
    /// Minimum throttle value (motor off)
    pub const THROTTLE_MIN: u16 = 48;
    /// Maximum throttle value
    pub const THROTTLE_MAX: u16 = 2047;
    
    /// Check if this is a command (throttle 1-47)
    pub fn is_command(&self) -> bool {
        self.throttle > 0 && self.throttle < Self::THROTTLE_MIN
    }
    
    /// Get the command if this is a command frame
    pub fn command(&self) -> Option<DshotCommand> {
        if self.is_command() {
            DshotCommand::from_value(self.throttle as u8)
        } else {
            None
        }
    }
    
    /// Get normalized throttle (0.0 to 1.0)
    pub fn normalized_throttle(&self) -> f32 {
        if self.throttle < Self::THROTTLE_MIN {
            0.0
        } else {
            (self.throttle - Self::THROTTLE_MIN) as f32 
                / (Self::THROTTLE_MAX - Self::THROTTLE_MIN) as f32
        }
    }
    
    /// Get throttle as duty cycle (0-2000 range like AM32)
    pub fn duty_cycle(&self) -> u16 {
        if self.throttle < Self::THROTTLE_MIN {
            0
        } else {
            let range = Self::THROTTLE_MAX - Self::THROTTLE_MIN;
            ((self.throttle - Self::THROTTLE_MIN) as u32 * 2000 / range as u32) as u16
        }
    }
}

/// DSHOT special commands (throttle values 1-47)
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
#[repr(u8)]
pub enum DshotCommand {
    // Beacons
    Beacon1 = 1,
    Beacon2 = 2,
    Beacon3 = 3,
    Beacon4 = 4,
    Beacon5 = 5,
    
    // ESC info
    EscInfo = 6,
    
    // Direction
    SpinDirection1 = 7,  // Normal
    SpinDirection2 = 8,  // Reversed
    
    // 3D mode
    Mode3dOff = 9,
    Mode3dOn = 10,
    
    // Settings save
    SettingsSave = 12,
    
    // Extended telemetry
    ExtendedTelemetryEnable = 13,
    ExtendedTelemetryDisable = 14,
    
    // Direction (alternative)
    SpinDirectionNormal = 20,
    SpinDirectionReversed = 21,
}

impl DshotCommand {
    /// Convert from raw value
    pub fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Beacon1),
            2 => Some(Self::Beacon2),
            3 => Some(Self::Beacon3),
            4 => Some(Self::Beacon4),
            5 => Some(Self::Beacon5),
            6 => Some(Self::EscInfo),
            7 => Some(Self::SpinDirection1),
            8 => Some(Self::SpinDirection2),
            9 => Some(Self::Mode3dOff),
            10 => Some(Self::Mode3dOn),
            12 => Some(Self::SettingsSave),
            13 => Some(Self::ExtendedTelemetryEnable),
            14 => Some(Self::ExtendedTelemetryDisable),
            20 => Some(Self::SpinDirectionNormal),
            21 => Some(Self::SpinDirectionReversed),
            _ => None,
        }
    }
}

/// GCR encoding table for DSHOT telemetry response
pub const GCR_ENCODE_TABLE: [u8; 16] = [
    0b11001, 0b11011, 0b10010, 0b10011,
    0b11101, 0b10101, 0b10110, 0b10111,
    0b11010, 0b01001, 0b01010, 0b01011,
    0b11110, 0b01101, 0b01110, 0b01111,
];

/// DSHOT receiver
/// 
/// Uses input capture with DMA to decode DSHOT frames.
pub struct DshotReceiver {
    /// DMA buffer for pulse timings
    dma_buffer: [u32; 32],
    /// Detected protocol variant
    variant: Option<DshotVariant>,
    /// Last valid frame
    last_frame: DshotFrame,
    /// Good frame counter
    good_count: u32,
    /// Bad frame counter
    bad_count: u32,
    /// Telemetry enabled flag
    telemetry_enabled: bool,
}

impl DshotReceiver {
    /// Create a new DSHOT receiver
    pub fn new() -> Self {
        Self {
            dma_buffer: [0; 32],
            variant: None,
            last_frame: DshotFrame::default(),
            good_count: 0,
            bad_count: 0,
            telemetry_enabled: false,
        }
    }
    
    /// Process DMA buffer and decode frame
    pub fn process_buffer(&mut self) -> Option<DshotFrame> {
        // Calculate frame time to detect variant
        let frame_time = self.dma_buffer[31].wrapping_sub(self.dma_buffer[0]);
        
        // Detect variant based on frame time
        self.variant = self.detect_variant(frame_time);
        
        if self.variant.is_none() {
            self.bad_count += 1;
            return None;
        }
        
        // Calculate half-pulse time for bit decoding
        let half_pulse = frame_time >> 5; // frame_time / 32
        
        // Decode 16 bits
        let mut bits = [false; 16];
        for i in 0..16 {
            let pulse_width = self.dma_buffer[(i << 1) + 1]
                .wrapping_sub(self.dma_buffer[i << 1]) as u16;
            bits[i] = pulse_width > half_pulse as u16;
        }
        
        // Extract throttle (11 bits), telemetry (1 bit), CRC (4 bits)
        let throttle = (bits[0] as u16) << 10
            | (bits[1] as u16) << 9
            | (bits[2] as u16) << 8
            | (bits[3] as u16) << 7
            | (bits[4] as u16) << 6
            | (bits[5] as u16) << 5
            | (bits[6] as u16) << 4
            | (bits[7] as u16) << 3
            | (bits[8] as u16) << 2
            | (bits[9] as u16) << 1
            | (bits[10] as u16);
        
        let telemetry = bits[11];
        
        let received_crc = (bits[12] as u8) << 3
            | (bits[13] as u8) << 2
            | (bits[14] as u8) << 1
            | (bits[15] as u8);
        
        // Calculate CRC
        let calc_crc = self.calculate_crc(throttle, telemetry);
        
        // Handle inverted DSHOT (bidirectional)
        let valid = if self.telemetry_enabled {
            received_crc == (!calc_crc & 0x0F)
        } else {
            received_crc == calc_crc
        };
        
        let frame = DshotFrame {
            throttle,
            telemetry,
            valid,
        };
        
        if valid {
            self.good_count += 1;
            self.last_frame = frame;
            Some(frame)
        } else {
            self.bad_count += 1;
            None
        }
    }
    
    /// Calculate DSHOT CRC
    fn calculate_crc(&self, throttle: u16, telemetry: bool) -> u8 {
        let value = (throttle << 1) | (telemetry as u16);
        let crc = (value ^ (value >> 4) ^ (value >> 8)) & 0x0F;
        crc as u8
    }
    
    /// Detect DSHOT variant from frame time
    fn detect_variant(&self, frame_time: u32) -> Option<DshotVariant> {
        // Frame time ranges (with some tolerance)
        // DSHOT600: ~27us (16 * 1.67us)
        // DSHOT300: ~53us (16 * 3.33us)
        // DSHOT150: ~107us (16 * 6.67us)
        
        // Assuming timer is running at 1MHz (1us per tick)
        if frame_time > 20 && frame_time < 35 {
            Some(DshotVariant::Dshot600)
        } else if frame_time > 45 && frame_time < 65 {
            Some(DshotVariant::Dshot300)
        } else if frame_time > 95 && frame_time < 120 {
            Some(DshotVariant::Dshot150)
        } else {
            None
        }
    }
    
    /// Get the DMA buffer for filling
    pub fn buffer_mut(&mut self) -> &mut [u32; 32] {
        &mut self.dma_buffer
    }
    
    /// Get last valid frame
    pub fn last_frame(&self) -> &DshotFrame {
        &self.last_frame
    }
    
    /// Get good frame count
    pub fn good_count(&self) -> u32 {
        self.good_count
    }
    
    /// Get bad frame count
    pub fn bad_count(&self) -> u32 {
        self.bad_count
    }
    
    /// Enable bidirectional DSHOT telemetry
    pub fn enable_telemetry(&mut self) {
        self.telemetry_enabled = true;
    }
    
    /// Disable bidirectional DSHOT telemetry
    pub fn disable_telemetry(&mut self) {
        self.telemetry_enabled = false;
    }
}

impl Default for DshotReceiver {
    fn default() -> Self {
        Self::new()
    }
}

/// Create DSHOT telemetry response packet
/// 
/// Encodes eRPM or extended telemetry data using GCR encoding.
pub fn make_dshot_response(erpm: u16) -> u32 {
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
    let gcr = (GCR_ENCODE_TABLE[(full_number >> 12) as usize] as u32) << 15
        | (GCR_ENCODE_TABLE[((full_number >> 8) & 0x0F) as usize] as u32) << 10
        | (GCR_ENCODE_TABLE[((full_number >> 4) & 0x0F) as usize] as u32) << 5
        | (GCR_ENCODE_TABLE[(full_number & 0x0F) as usize] as u32);
    
    gcr
}
