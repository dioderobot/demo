//! FDCAN Driver for CAN Bus Communication
//!
//! Uses FDCAN1 on PA11 (RX) and PB9 (TX).
//! Supports standard CAN 2.0 and CAN FD modes.
//!
//! Note: This is a simplified implementation. Full CAN support requires
//! proper interrupt handling and the fdcan feature in embassy-stm32.

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

/// CAN message buffer size
const CAN_BUFFER_SIZE: usize = 8;

/// CAN RX channel for received messages
pub static CAN_RX_CHANNEL: Channel<CriticalSectionRawMutex, CanMessage, CAN_BUFFER_SIZE> = Channel::new();

/// CAN TX channel for messages to send
pub static CAN_TX_CHANNEL: Channel<CriticalSectionRawMutex, CanMessage, CAN_BUFFER_SIZE> = Channel::new();

/// CAN message wrapper
#[derive(Debug, Clone, Copy)]
pub struct CanMessage {
    /// CAN ID (11-bit standard or 29-bit extended)
    pub id: u32,
    /// Extended ID flag
    pub extended: bool,
    /// Data length (0-8 for CAN 2.0, 0-64 for CAN FD)
    pub len: u8,
    /// Message data
    pub data: [u8; 8],
}

impl Default for CanMessage {
    fn default() -> Self {
        Self {
            id: 0,
            extended: false,
            len: 0,
            data: [0; 8],
        }
    }
}

impl CanMessage {
    /// Create a new CAN message
    pub fn new(id: u32, data: &[u8]) -> Self {
        let mut msg = Self {
            id,
            extended: id > 0x7FF,
            len: data.len().min(8) as u8,
            data: [0; 8],
        };
        msg.data[..msg.len as usize].copy_from_slice(&data[..msg.len as usize]);
        msg
    }

    /// Create ESC status message
    pub fn esc_status(esc_id: u8, rpm: u16, voltage: u16, current: u16, temp: u8) -> Self {
        let mut data = [0u8; 8];
        data[0] = esc_id;
        data[1] = (rpm >> 8) as u8;
        data[2] = rpm as u8;
        data[3] = (voltage >> 8) as u8;
        data[4] = voltage as u8;
        data[5] = (current >> 8) as u8;
        data[6] = current as u8;
        data[7] = temp;
        
        Self::new(0x100 + esc_id as u32, &data)
    }
}

/// CAN error types
#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum CanError {
    /// Invalid frame format
    InvalidFrame,
    /// Transmit failed
    TransmitFailed,
    /// Receive failed
    ReceiveFailed,
    /// Bus off
    BusOff,
    /// Timeout
    Timeout,
    /// Not initialized
    NotInitialized,
}

/// CAN command IDs
pub mod commands {
    /// Set throttle command
    pub const CMD_SET_THROTTLE: u32 = 0x200;
    /// Request status
    pub const CMD_REQUEST_STATUS: u32 = 0x201;
    /// Set direction
    pub const CMD_SET_DIRECTION: u32 = 0x202;
    /// Emergency stop
    pub const CMD_EMERGENCY_STOP: u32 = 0x203;
    /// Set settings
    pub const CMD_SET_SETTINGS: u32 = 0x210;
    /// Save settings
    pub const CMD_SAVE_SETTINGS: u32 = 0x211;
}

/// Process incoming CAN command
pub fn process_can_command(msg: &CanMessage) -> Option<CanCommand> {
    match msg.id {
        commands::CMD_SET_THROTTLE => {
            if msg.len >= 2 {
                let throttle = ((msg.data[0] as u16) << 8) | (msg.data[1] as u16);
                Some(CanCommand::SetThrottle(throttle))
            } else {
                None
            }
        }
        commands::CMD_REQUEST_STATUS => Some(CanCommand::RequestStatus),
        commands::CMD_SET_DIRECTION => {
            if msg.len >= 1 {
                Some(CanCommand::SetDirection(msg.data[0] != 0))
            } else {
                None
            }
        }
        commands::CMD_EMERGENCY_STOP => Some(CanCommand::EmergencyStop),
        _ => None,
    }
}

/// CAN command types
#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum CanCommand {
    /// Set throttle (0-2000)
    SetThrottle(u16),
    /// Request status telemetry
    RequestStatus,
    /// Set direction (true = reverse)
    SetDirection(bool),
    /// Emergency stop
    EmergencyStop,
}

// Note: Full CAN driver implementation requires:
// 1. Enable "fdcan" feature in embassy-stm32
// 2. Bind FDCAN interrupts
// 3. Configure CAN timing for desired bitrate
// 4. Implement async read/write methods
//
// Example initialization (when fdcan feature is enabled):
// ```
// use embassy_stm32::can::{Can, Rx, Tx};
// use embassy_stm32::peripherals::{FDCAN1, PA11, PB9};
// 
// let can = Can::new(fdcan1, rx_pin, tx_pin, Irqs);
// can.set_bitrate(500_000);
// let (tx, rx) = can.split();
// ```
