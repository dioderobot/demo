//! AM32 Embassy - BLDC Motor Controller Firmware
//!
//! Rust implementation of AM32 ESC firmware using Embassy async framework
//! for the DM0001 ZenDrive motor controller board.
//!
//! Target: STM32G431C8T6

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, Config};
use embassy_time::{Duration, Ticker, Timer};

mod config;
mod motor;
mod input;
mod sensing;
mod settings;
mod sounds;

use config::BoardConfig;
use settings::Settings;

/// Global settings loaded from flash
static SETTINGS: static_cell::StaticCell<Settings> = static_cell::StaticCell::new();

bind_interrupts!(struct Irqs {
    // Interrupts will be bound as needed
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("AM32-Embassy starting...");
    
    // Configure clocks for 170MHz operation
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        
        // Configure HSE (8MHz external oscillator)
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Oscillator,
        });
        
        // Configure PLL for 170MHz
        // HSE (8MHz) / 2 * 85 / 2 = 170MHz
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL85,
            divp: None,
            divq: Some(PllQDiv::DIV2),
            divr: Some(PllRDiv::DIV2),
        });
        
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV1;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
    }
    
    let _p = embassy_stm32::init(config);
    
    info!("Clock configured at 170MHz");
    
    // Load settings from flash (or use defaults)
    let settings = SETTINGS.init(Settings::default());
    info!("Settings loaded: motor_kv={}, dead_time={}", settings.motor_kv, settings.dead_time);
    
    // Initialize board configuration
    let board = BoardConfig::new();
    info!("Board: {}", board.name);
    
    // Spawn the control loop task
    spawner.spawn(control_loop_task()).unwrap();
    
    // Spawn input handling task
    spawner.spawn(input_task()).unwrap();
    
    // Spawn telemetry task
    spawner.spawn(telemetry_task()).unwrap();
    
    // Spawn ADC sensing task
    spawner.spawn(adc_task()).unwrap();
    
    info!("All tasks spawned, entering main loop");
    
    // Main task just monitors system health
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        ticker.next().await;
        trace!("Heartbeat");
    }
}

/// Motor control loop running at 20kHz
#[embassy_executor::task]
async fn control_loop_task() {
    info!("Control loop task started");
    
    // 20kHz = 50us period
    let mut ticker = Ticker::every(Duration::from_micros(50));
    let mut loop_count: u32 = 0;
    
    loop {
        ticker.next().await;
        loop_count = loop_count.wrapping_add(1);
        
        // TODO: Implement motor control logic
        // 1. Read input command
        // 2. Update duty cycle with ramping
        // 3. Check BEMF for commutation
        // 4. Handle stall protection
        
        if loop_count % 20000 == 0 {
            trace!("Control loop running, count={}", loop_count);
        }
    }
}

/// Input signal handling (DSHOT/PWM)
#[embassy_executor::task]
async fn input_task() {
    info!("Input task started");
    
    loop {
        // TODO: Implement DSHOT/PWM input capture
        Timer::after(Duration::from_millis(1)).await;
    }
}

/// Telemetry output task
#[embassy_executor::task]
async fn telemetry_task() {
    info!("Telemetry task started");
    
    // Telemetry at ~100Hz
    let mut ticker = Ticker::every(Duration::from_millis(10));
    
    loop {
        ticker.next().await;
        // TODO: Send telemetry data
    }
}

/// ADC sensing task for voltage, current, temperature
#[embassy_executor::task]
async fn adc_task() {
    info!("ADC task started");
    
    // Sample ADC at 1kHz
    let mut ticker = Ticker::every(Duration::from_millis(1));
    
    loop {
        ticker.next().await;
        // TODO: Sample voltage, current, temperature
    }
}
