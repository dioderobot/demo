//! Input Capture Driver for DSHOT/PWM
//!
//! Uses TIM2 CH1 (PA15) for input signal capture.
//! Supports auto-detection of DSHOT150/300/600 and PWM servo.
//!
//! Note: This is a simplified implementation. Full input capture requires
//! proper timer configuration and interrupt handling.

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

use crate::input::dshot::{DshotFrame, DshotVariant};
use crate::input::signal::{InputSignal, InputProtocol};

/// Input capture buffer size (enough for DSHOT frame + margin)
const CAPTURE_BUFFER_SIZE: usize = 32;

/// Signal for new input data
pub static INPUT_SIGNAL: Signal<CriticalSectionRawMutex, InputSignal> = Signal::new();

/// Input capture driver (software abstraction)
pub struct InputCaptureDriver {
    /// Capture buffer
    buffer: [u32; CAPTURE_BUFFER_SIZE],
    /// Buffer index
    index: usize,
    /// Last capture value (for calculating deltas)
    last_capture: u32,
    /// Detected protocol
    protocol: InputProtocol,
    /// Frame start detected
    frame_started: bool,
    /// Good frame counter
    good_frames: u32,
    /// Bad frame counter
    bad_frames: u32,
}

impl InputCaptureDriver {
    /// Create a new input capture driver
    pub fn new() -> Self {
        defmt::info!("Input capture initialized (software mode)");

        Self {
            buffer: [0; CAPTURE_BUFFER_SIZE],
            index: 0,
            last_capture: 0,
            protocol: InputProtocol::None,
            frame_started: false,
            good_frames: 0,
            bad_frames: 0,
        }
    }

    /// Process a capture event
    pub fn process_capture(&mut self, capture_value: u32) -> Option<InputSignal> {
        let delta = capture_value.wrapping_sub(self.last_capture);
        self.last_capture = capture_value;

        // Detect frame start (long gap)
        if delta > 50 { // > 50us gap indicates frame boundary
            if self.frame_started && self.index >= 16 {
                // Process completed frame
                let signal = self.decode_frame();
                self.index = 0;
                self.frame_started = true;
                return signal;
            }
            self.index = 0;
            self.frame_started = true;
            return None;
        }

        // Store pulse width
        if self.index < CAPTURE_BUFFER_SIZE {
            self.buffer[self.index] = delta;
            self.index += 1;
        }

        None
    }

    /// Decode the captured frame
    fn decode_frame(&mut self) -> Option<InputSignal> {
        if self.index < 16 {
            return None;
        }

        // Calculate total frame time
        let frame_time: u32 = self.buffer[..16].iter().sum();

        // Detect protocol based on frame time
        let variant = self.detect_dshot_variant(frame_time);

        if let Some(variant) = variant {
            self.protocol = InputProtocol::Dshot;
            return self.decode_dshot(variant);
        }

        // Check for PWM servo (1000-2000us pulse)
        if self.index >= 1 && self.buffer[0] >= 800 && self.buffer[0] <= 2200 {
            self.protocol = InputProtocol::Servo;
            return self.decode_servo();
        }

        self.bad_frames += 1;
        None
    }

    /// Detect DSHOT variant from frame time
    fn detect_dshot_variant(&self, frame_time: u32) -> Option<DshotVariant> {
        // Frame times (16 bits):
        // DSHOT600: ~27us (16 * 1.67us)
        // DSHOT300: ~53us (16 * 3.33us)
        // DSHOT150: ~107us (16 * 6.67us)
        
        if frame_time >= 20 && frame_time <= 35 {
            Some(DshotVariant::Dshot600)
        } else if frame_time >= 45 && frame_time <= 65 {
            Some(DshotVariant::Dshot300)
        } else if frame_time >= 95 && frame_time <= 120 {
            Some(DshotVariant::Dshot150)
        } else {
            None
        }
    }

    /// Decode DSHOT frame
    fn decode_dshot(&mut self, variant: DshotVariant) -> Option<InputSignal> {
        // Calculate threshold for 0/1 bit detection
        let bit_period = variant.bit_period_ns() / 1000; // Convert to us
        let threshold = bit_period * 3 / 4; // 75% of bit period

        // Decode 16 bits
        let mut value: u16 = 0;
        for i in 0..16 {
            value <<= 1;
            if self.buffer[i] > threshold {
                value |= 1;
            }
        }

        // Extract fields
        let throttle = (value >> 5) & 0x7FF; // 11 bits
        let telemetry = (value >> 4) & 0x01 != 0;
        let crc = value & 0x0F;

        // Verify CRC
        let calc_crc = self.calculate_dshot_crc(throttle, telemetry);
        if crc != calc_crc {
            self.bad_frames += 1;
            return None;
        }

        self.good_frames += 1;

        let frame = DshotFrame {
            throttle,
            telemetry,
            valid: true,
        };

        Some(InputSignal::from_dshot(&frame))
    }

    /// Calculate DSHOT CRC
    fn calculate_dshot_crc(&self, throttle: u16, telemetry: bool) -> u16 {
        let value = (throttle << 1) | (telemetry as u16);
        (value ^ (value >> 4) ^ (value >> 8)) & 0x0F
    }

    /// Decode PWM servo signal
    fn decode_servo(&mut self) -> Option<InputSignal> {
        let pulse_us = self.buffer[0];
        
        if pulse_us < 800 || pulse_us > 2200 {
            self.bad_frames += 1;
            return None;
        }

        self.good_frames += 1;

        // Convert to duty cycle (0-2000)
        let duty = if pulse_us <= 1000 {
            0
        } else if pulse_us >= 2000 {
            2000
        } else {
            ((pulse_us - 1000) * 2) as u16
        };

        Some(InputSignal::from_servo(duty, true))
    }

    /// Get detected protocol
    pub fn protocol(&self) -> InputProtocol {
        self.protocol
    }

    /// Get good frame count
    pub fn good_frames(&self) -> u32 {
        self.good_frames
    }

    /// Get bad frame count
    pub fn bad_frames(&self) -> u32 {
        self.bad_frames
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.good_frames = 0;
        self.bad_frames = 0;
    }
}

impl Default for InputCaptureDriver {
    fn default() -> Self {
        Self::new()
    }
}

// Note: Full hardware input capture implementation requires:
//
// ```rust
// use embassy_stm32::timer::input_capture::{InputCapture, CapturePin};
// use embassy_stm32::peripherals::{TIM2, PA15};
//
// // Configure input capture
// let ch1_pin = CapturePin::new(pa15, Pull::Down);
// let capture = InputCapture::new(
//     tim2,
//     Some(ch1_pin),
//     None, None, None,
//     Hertz(1_000_000), // 1MHz = 1us resolution
// );
//
// // Wait for capture
// let value = capture.wait_for_rising_edge(Channel::Ch1).await;
// ```
