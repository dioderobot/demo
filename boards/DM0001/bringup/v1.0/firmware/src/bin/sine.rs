#![no_std]
#![no_main]

//! 3-Phase Sine Wave Generator for DM0001 Motor Controller
//! 
//! Generates three 120° phase-shifted sine waves on the motor phase outputs:
//! - Phase A: PA8 (TIM1_CH1), PC13 (TIM1_CH1N)
//! - Phase B: PA9 (TIM1_CH2), PA12 (TIM1_CH2N)
//! - Phase C: PA10 (TIM1_CH3), PB15 (TIM1_CH3N)
//!
//! Uses Timer1 with complementary PWM outputs and deadtime insertion
//! for safe gate driver operation.
//!
//! WARNING: This will energize the motor phases. Ensure no motor is connected
//! or that the motor can safely handle low voltage sine wave drive.

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, OutputType, Speed};
use embassy_stm32::time::khz;
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::simple_pwm::PwmPin;
use embassy_stm32::timer::Channel;
use embassy_time::{Duration, Ticker};
use {defmt_rtt as _, panic_probe as _};

// Sine wave lookup table (256 entries, 0-255 range for 8-bit precision)
// This gives smoother sine waves than calculating on the fly
const SINE_TABLE: [u8; 256] = [
    127, 130, 133, 136, 139, 143, 146, 149, 152, 155, 158, 161, 164, 167, 170, 173,
    176, 178, 181, 184, 187, 190, 192, 195, 198, 200, 203, 205, 208, 210, 212, 215,
    217, 219, 221, 223, 225, 227, 229, 231, 233, 234, 236, 238, 239, 240, 242, 243,
    244, 245, 247, 248, 249, 249, 250, 251, 252, 252, 253, 253, 253, 254, 254, 254,
    254, 254, 254, 254, 253, 253, 253, 252, 252, 251, 250, 249, 249, 248, 247, 245,
    244, 243, 242, 240, 239, 238, 236, 234, 233, 231, 229, 227, 225, 223, 221, 219,
    217, 215, 212, 210, 208, 205, 203, 200, 198, 195, 192, 190, 187, 184, 181, 178,
    176, 173, 170, 167, 164, 161, 158, 155, 152, 149, 146, 143, 139, 136, 133, 130,
    127, 124, 121, 118, 115, 111, 108, 105, 102,  99,  96,  93,  90,  87,  84,  81,
     78,  76,  73,  70,  67,  64,  62,  59,  56,  54,  51,  49,  46,  44,  42,  39,
     37,  35,  33,  31,  29,  27,  25,  23,  21,  20,  18,  16,  15,  14,  12,  11,
     10,   9,   7,   6,   5,   5,   4,   3,   2,   2,   1,   1,   1,   0,   0,   0,
      0,   0,   0,   0,   1,   1,   1,   2,   2,   3,   4,   5,   5,   6,   7,   9,
     10,  11,  12,  14,  15,  16,  18,  20,  21,  23,  25,  27,  29,  31,  33,  35,
     37,  39,  42,  44,  46,  49,  51,  54,  56,  59,  62,  64,  67,  70,  73,  76,
     78,  81,  84,  87,  90,  93,  96,  99, 102, 105, 108, 111, 115, 118, 121, 124,
];

// Configuration
const PWM_FREQ_KHZ: u32 = 20; // 20 kHz PWM frequency (typical for motor control)
const SINE_FREQ_HZ: u32 = 5;  // 5 Hz sine wave frequency (slow for testing)
const UPDATE_RATE_HZ: u32 = 256 * SINE_FREQ_HZ; // Update rate for smooth sine wave
const DEADTIME_NS: u16 = 500; // 500ns deadtime for gate drivers
const MAX_DUTY_PERCENT: u8 = 80; // Maximum duty cycle (80% for safety margin)

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("===========================================");
    info!("DM0001 3-Phase Sine Wave Generator");
    info!("===========================================");
    
    let p = embassy_stm32::init(Default::default());
    
    // Initialize status LED for heartbeat
    let mut status_led = Output::new(p.PC15, Level::Low, Speed::Low);
    info!("[LED] Status LED initialized");
    
    // Configure Timer1 for complementary PWM
    info!("[PWM] Configuring TIM1 for 3-phase complementary PWM");
    info!("[PWM] PWM Frequency: {} kHz", PWM_FREQ_KHZ);
    info!("[PWM] Sine Frequency: {} Hz", SINE_FREQ_HZ);
    info!("[PWM] Deadtime: {} ns", DEADTIME_NS);
    info!("[PWM] Max Duty: {}%", MAX_DUTY_PERCENT);
    
    // Create complementary PWM channels
    // Phase A: PA8 (CH1), PC13 (CH1N)
    let ch1 = PwmPin::new(p.PA8, OutputType::PushPull);
    let ch1n = ComplementaryPwmPin::new(p.PC13, OutputType::PushPull);
    
    // Phase B: PA9 (CH2), PA12 (CH2N)
    let ch2 = PwmPin::new(p.PA9, OutputType::PushPull);
    let ch2n = ComplementaryPwmPin::new(p.PA12, OutputType::PushPull);
    
    // Phase C: PA10 (CH3), PB15 (CH3N)
    let ch3 = PwmPin::new(p.PA10, OutputType::PushPull);
    let ch3n = ComplementaryPwmPin::new(p.PB15, OutputType::PushPull);
    
    let mut pwm = ComplementaryPwm::new(
        p.TIM1,
        Some(ch1),
        Some(ch1n),
        Some(ch2),
        Some(ch2n),
        Some(ch3),
        Some(ch3n),
        None,
        None,
        khz(PWM_FREQ_KHZ),
        embassy_stm32::timer::low_level::CountingMode::CenterAlignedBothInterrupts,
    );
    
    // Set deadtime for safe gate driver operation
    // Deadtime prevents shoot-through in the half-bridges
    pwm.set_dead_time(DEADTIME_NS);
    
    // Get max duty value
    let max_duty = pwm.get_max_duty();
    info!("[PWM] Max duty value: {}", max_duty);
    
    // Calculate actual max duty based on percentage
    let max_duty_value = (max_duty as u32 * MAX_DUTY_PERCENT as u32) / 100;
    
    // Enable all channels
    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);
    
    info!("[PWM] All channels enabled");
    info!("===========================================");
    info!("Generating 3-phase sine waves...");
    info!("WARNING: Motor phases are now energized!");
    info!("===========================================");
    
    // Create ticker for sine wave updates
    let mut ticker = Ticker::every(Duration::from_hz(UPDATE_RATE_HZ as u64));
    
    let mut angle: u8 = 0; // 0-255 representing 0-360°
    let mut heartbeat_counter = 0u32;
    
    loop {
        ticker.next().await;
        
        // Calculate phase angles (120° = 85 in 0-255 scale)
        let angle_a = angle;
        let angle_b = angle.wrapping_add(85); // +120°
        let angle_c = angle.wrapping_add(170); // +240°
        
        // Get sine values from lookup table (0-255)
        let sine_a = SINE_TABLE[angle_a as usize] as u32;
        let sine_b = SINE_TABLE[angle_b as usize] as u32;
        let sine_c = SINE_TABLE[angle_c as usize] as u32;
        
        // Scale to PWM duty cycle (0 to max_duty_value)
        let duty_a = (sine_a * max_duty_value) / 255;
        let duty_b = (sine_b * max_duty_value) / 255;
        let duty_c = (sine_c * max_duty_value) / 255;
        
        // Set PWM duty cycles
        pwm.set_duty(Channel::Ch1, duty_a as u16);
        pwm.set_duty(Channel::Ch2, duty_b as u16);
        pwm.set_duty(Channel::Ch3, duty_c as u16);
        
        // Increment angle for next iteration
        angle = angle.wrapping_add(1);
        
        // Heartbeat LED (blink slowly)
        heartbeat_counter += 1;
        if heartbeat_counter % (UPDATE_RATE_HZ / 2) == 0 {
            status_led.toggle();
        }
        
        // Log status periodically
        if heartbeat_counter % UPDATE_RATE_HZ == 0 {
            info!(
                "[SINE] angle={}, duties: A={} B={} C={} (max={})",
                angle, duty_a, duty_b, duty_c, max_duty_value
            );
        }
    }
}

