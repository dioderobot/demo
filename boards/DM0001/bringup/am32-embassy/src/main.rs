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
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Ticker, Timer};

mod config;
mod motor;
mod input;
mod sensing;
mod settings;
mod sounds;
mod drivers;

use config::BoardConfig;
use motor::{MotorController, MotorConfig, ControlAction};
use settings::Settings;
use drivers::telemetry::{TelemetryData, TelemetryController};

// Bind interrupts (empty for now, add as needed)
bind_interrupts!(struct Irqs {});

/// Global settings
static mut SETTINGS: Option<Settings> = None;

/// Global motor controller
static MOTOR_CONTROLLER: Mutex<CriticalSectionRawMutex, Option<MotorController>> = Mutex::new(None);

/// Global telemetry controller
static TELEMETRY: Mutex<CriticalSectionRawMutex, Option<TelemetryController>> = Mutex::new(None);

/// Signal for new throttle commands
static THROTTLE_SIGNAL: Signal<CriticalSectionRawMutex, u16> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("AM32-Embassy v0.1.0 starting...");
    
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
    let settings = Settings::default();
    unsafe { SETTINGS = Some(settings); }
    info!("Settings loaded: motor_kv={}, dead_time={}", settings.motor_kv, settings.dead_time);
    
    // Initialize board configuration
    let board = BoardConfig::new();
    info!("Board: {}", board.name);
    
    // Initialize motor controller
    let motor_config = MotorConfig::from(&settings);
    {
        let mut mc = MOTOR_CONTROLLER.lock().await;
        *mc = Some(MotorController::new(motor_config));
    }
    
    // Initialize telemetry controller
    {
        let mut tc = TELEMETRY.lock().await;
        *tc = Some(TelemetryController::new());
    }
    
    // Spawn tasks
    spawner.spawn(control_loop_task()).unwrap();
    spawner.spawn(input_task()).unwrap();
    spawner.spawn(telemetry_task()).unwrap();
    spawner.spawn(adc_task()).unwrap();
    
    info!("All tasks spawned");
    
    // Play startup sound (TODO: implement via PWM)
    info!("Ready!");
    
    // Main task monitors system health
    let mut ticker = Ticker::every(Duration::from_secs(1));
    let mut last_state = motor::MotorState::Stopped;
    
    loop {
        ticker.next().await;
        
        // Log status periodically
        let mc = MOTOR_CONTROLLER.lock().await;
        if let Some(ref controller) = *mc {
            let state = controller.state();
            let erpm = controller.erpm();
            let duty = controller.duty();
            
            if state != last_state {
                info!("Motor state: {:?}", state);
                last_state = state;
            }
            
            trace!("Status: state={:?}, eRPM={}, duty={}", state, erpm, duty);
        }
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
        
        // Check for new throttle command
        if let Some(throttle) = THROTTLE_SIGNAL.try_take() {
            let mut mc = MOTOR_CONTROLLER.lock().await;
            if let Some(ref mut controller) = *mc {
                controller.set_throttle(throttle);
            }
        }
        
        // Run motor control update
        // TODO: Get actual BEMF samples from ADC
        let bemf_samples = motor::bemf::BemfSamples::default();
        
        let mut mc = MOTOR_CONTROLLER.lock().await;
        if let Some(ref mut controller) = *mc {
            let action = controller.update(bemf_samples);
            
            match action {
                ControlAction::Commutate(step) => {
                    // TODO: Apply commutation via PWM driver
                    trace!("Commutate to step {}", step.number());
                }
                ControlAction::UpdateDuty(duty) => {
                    // TODO: Update PWM duty cycle
                    trace!("Duty update: {}", duty);
                }
                ControlAction::Start => {
                    info!("Motor starting");
                }
                ControlAction::Fault => {
                    error!("Motor fault!");
                }
                ControlAction::None => {}
            }
            
            // Periodic logging
            if loop_count % 20000 == 0 {
                let erpm = controller.erpm();
                let zc = controller.zc_count();
                trace!("Control loop: eRPM={}, ZC={}", erpm, zc);
            }
        }
    }
}

/// Input signal handling (DSHOT/PWM)
#[embassy_executor::task]
async fn input_task() {
    info!("Input task started");
    
    // TODO: Initialize input capture driver
    // let input_driver = InputCaptureDriver::new(tim2, pa15);
    
    let mut armed = false;
    let mut arm_count: u16 = 0;
    const ARM_TIME: u16 = 1000; // 1 second at 1kHz
    
    loop {
        // TODO: Wait for input capture event
        Timer::after(Duration::from_millis(1)).await;
        
        // Simulated input for now
        let throttle: u16 = 0;
        
        // Arming logic
        if throttle == 0 {
            arm_count = arm_count.saturating_add(1);
            if arm_count >= ARM_TIME && !armed {
                armed = true;
                info!("ESC armed");
            }
        } else {
            arm_count = 0;
        }
        
        // Send throttle command if armed
        if armed {
            THROTTLE_SIGNAL.signal(throttle);
        }
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
        
        // Get motor data
        let mc = MOTOR_CONTROLLER.lock().await;
        let (erpm, duty) = if let Some(ref controller) = *mc {
            (controller.erpm(), controller.duty())
        } else {
            (0, 0)
        };
        drop(mc);
        
        // Update telemetry
        let mut tc = TELEMETRY.lock().await;
        if let Some(ref mut telem) = *tc {
            let mut data = TelemetryData::new();
            data.erpm = erpm;
            // TODO: Get actual voltage/current/temp from ADC
            data.voltage_mv = 14800; // Placeholder
            data.current_ma = duty * 10; // Rough estimate
            data.temperature_c = 25;
            telem.update(data);
        }
    }
}

/// ADC sensing task for voltage, current, temperature
#[embassy_executor::task]
async fn adc_task() {
    info!("ADC task started");
    
    // TODO: Initialize ADC driver
    // let adc_driver = AdcDriver::new(adc1, adc2, ...);
    
    // Sample ADC at 1kHz
    let mut ticker = Ticker::every(Duration::from_millis(1));
    
    loop {
        ticker.next().await;
        
        // TODO: Read ADC values
        // let readings = adc_driver.read_all();
        
        // TODO: Update telemetry with readings
    }
}
