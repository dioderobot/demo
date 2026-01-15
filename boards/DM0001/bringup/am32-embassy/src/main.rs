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
use embassy_stm32::adc::{Adc, SampleTime};
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::{khz, Hertz};
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::simple_pwm::PwmPin;
use embassy_stm32::timer::Channel;
use embassy_stm32::{bind_interrupts, peripherals, Config};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Ticker, Timer};

mod config;
mod drivers;
mod input;
mod motor;
mod sensing;
mod settings;
mod sounds;

use config::BoardConfig;
use motor::bemf::BemfSamples;
use motor::commutation::{CommutationStep, CommutationTable, PhaseState};
use motor::{ControlAction, MotorConfig, MotorController};
use settings::Settings;

// Bind interrupts
bind_interrupts!(struct Irqs {
    // Add timer interrupt for input capture when needed
    // TIM2 => embassy_stm32::timer::CaptureCompareInterruptHandler<peripherals::TIM2>;
});

/// Global settings
static mut SETTINGS: Option<Settings> = None;

/// Global motor controller
static MOTOR_CONTROLLER: Mutex<CriticalSectionRawMutex, Option<MotorController>> = Mutex::new(None);

/// Signal for new throttle commands
static THROTTLE_SIGNAL: Signal<CriticalSectionRawMutex, u16> = Signal::new();

/// Signal for BEMF samples from ADC task
static BEMF_SIGNAL: Signal<CriticalSectionRawMutex, BemfSamples> = Signal::new();

/// Signal for commutation from control loop
static COMMUTATION_SIGNAL: Signal<CriticalSectionRawMutex, (CommutationStep, u16)> = Signal::new();

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

        // Configure ADC clock
        config.rcc.mux.adc12sel = mux::Adcsel::SYS;
    }

    let p = embassy_stm32::init(config);

    info!("Clock configured at 170MHz");

    // Load settings
    let settings = Settings::default();
    unsafe {
        SETTINGS = Some(settings);
    }
    info!(
        "Settings loaded: motor_kv={}, dead_time={}",
        settings.motor_kv, settings.dead_time
    );

    // Initialize board configuration
    let board = BoardConfig::new();
    info!("Board: {}", board.name);

    // Initialize motor controller
    let motor_config = MotorConfig::from(&settings);
    {
        let mut mc = MOTOR_CONTROLLER.lock().await;
        *mc = Some(MotorController::new(motor_config));
    }

    // =========================================================================
    // Initialize TIM1 for 3-phase complementary PWM
    // =========================================================================
    // Phase A: PA8 (CH1) / PC13 (CH1N)
    // Phase B: PA9 (CH2) / PA12 (CH2N)
    // Phase C: PA10 (CH3) / PB15 (CH3N)

    let ch1 = PwmPin::new(p.PA8, OutputType::PushPull);
    let ch1n = ComplementaryPwmPin::new(p.PC13, OutputType::PushPull);
    let ch2 = PwmPin::new(p.PA9, OutputType::PushPull);
    let ch2n = ComplementaryPwmPin::new(p.PA12, OutputType::PushPull);
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
        khz(24), // 24kHz PWM frequency
        Default::default(),
    );

    let max_duty = pwm.get_max_duty();
    info!("PWM initialized: max_duty={}", max_duty);

    // Set dead time (~500ns at 170MHz)
    // dead_time = DTG * t_DTS, where t_DTS = 1/170MHz ≈ 5.88ns
    // For 500ns: DTG ≈ 85
    pwm.set_dead_time(85);

    // Start with all outputs disabled
    pwm.set_duty(Channel::Ch1, 0);
    pwm.set_duty(Channel::Ch2, 0);
    pwm.set_duty(Channel::Ch3, 0);

    // Enable all channels
    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);

    info!("PWM outputs enabled");

    // =========================================================================
    // Initialize ADCs for BEMF and sensing
    // =========================================================================
    // BEMF A: PA4 -> ADC2
    // BEMF B: PB12 -> ADC1
    // BEMF C: PB11 -> ADC1
    // VBUS: PA0 -> ADC1

    let adc1 = Adc::new(p.ADC1, Default::default());
    let adc2 = Adc::new(p.ADC2, Default::default());

    info!("ADC initialized");

    // Spawn tasks
    spawner.spawn(control_loop_task()).unwrap();
    spawner.spawn(input_task()).unwrap();
    spawner
        .spawn(pwm_task(pwm, max_duty as u16))
        .unwrap();
    spawner
        .spawn(adc_task(adc1, adc2, p.PA0, p.PA4, p.PB11, p.PB12))
        .unwrap();

    info!("All tasks spawned - Ready!");

    // Main task monitors system health
    let mut ticker = Ticker::every(Duration::from_secs(1));
    let mut last_state = motor::MotorState::Stopped;

    loop {
        ticker.next().await;

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

        // Get BEMF samples (use default if not available yet)
        let bemf_samples = BEMF_SIGNAL.try_take().unwrap_or_default();

        // Run motor control update
        let mut mc = MOTOR_CONTROLLER.lock().await;
        if let Some(ref mut controller) = *mc {
            let action = controller.update(bemf_samples);

            match action {
                ControlAction::Commutate(step) => {
                    let duty = controller.duty();
                    COMMUTATION_SIGNAL.signal((step, duty));
                    trace!("Commutate to step {}, duty={}", step.number(), duty);
                }
                ControlAction::UpdateDuty(duty) => {
                    // Send current step with new duty
                    let step = controller.step();
                    COMMUTATION_SIGNAL.signal((step, duty));
                }
                ControlAction::Start => {
                    info!("Motor starting");
                    let step = controller.step();
                    let duty = controller.duty();
                    COMMUTATION_SIGNAL.signal((step, duty));
                }
                ControlAction::Fault => {
                    error!("Motor fault!");
                    // Signal all-off
                    COMMUTATION_SIGNAL.signal((CommutationStep::Step1, 0));
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

/// PWM output task - applies commutation steps to TIM1
#[embassy_executor::task]
async fn pwm_task(
    mut pwm: ComplementaryPwm<'static, peripherals::TIM1>,
    max_duty: u16,
) {
    info!("PWM task started");

    loop {
        // Wait for commutation signal
        let (step, duty) = COMMUTATION_SIGNAL.wait().await;

        // Scale duty from 0-2000 to 0-max_duty
        let scaled_duty = (duty as u32 * max_duty as u32 / 2000) as u16;

        // Get phase states for this step
        let entry = CommutationTable::get(step);

        // Apply phase A
        apply_phase(&mut pwm, Channel::Ch1, entry.phase_a, scaled_duty, max_duty);

        // Apply phase B
        apply_phase(&mut pwm, Channel::Ch2, entry.phase_b, scaled_duty, max_duty);

        // Apply phase C
        apply_phase(&mut pwm, Channel::Ch3, entry.phase_c, scaled_duty, max_duty);
    }
}

/// Apply phase state to PWM channel
fn apply_phase(
    pwm: &mut ComplementaryPwm<'static, peripherals::TIM1>,
    channel: Channel,
    state: PhaseState,
    duty: u16,
    _max_duty: u16,
) {
    match state {
        PhaseState::Pwm => {
            // PWM output with current duty
            pwm.set_duty(channel, duty as u32);
            pwm.enable(channel);
        }
        PhaseState::Low => {
            // Low side on (duty = 0, complementary output will be high)
            pwm.set_duty(channel, 0);
            pwm.enable(channel);
        }
        PhaseState::Float => {
            // Both outputs off
            pwm.disable(channel);
        }
    }
}

/// ADC sensing task - samples BEMF and voltage
#[embassy_executor::task]
async fn adc_task(
    mut adc1: Adc<'static, peripherals::ADC1>,
    mut adc2: Adc<'static, peripherals::ADC2>,
    mut vbus_pin: embassy_stm32::Peri<'static, peripherals::PA0>,
    mut bemf_a_pin: embassy_stm32::Peri<'static, peripherals::PA4>,
    mut bemf_c_pin: embassy_stm32::Peri<'static, peripherals::PB11>,
    mut bemf_b_pin: embassy_stm32::Peri<'static, peripherals::PB12>,
) {
    info!("ADC task started");

    // Fast sample time for BEMF
    let bemf_sample_time = SampleTime::CYCLES12_5;
    // Slower sample time for voltage (more accurate)
    let sense_sample_time = SampleTime::CYCLES47_5;

    // Voltage filter
    let mut voltage_filtered: u32 = 0;

    // Sample at 10kHz (every 100us) - fast enough for BEMF at reasonable speeds
    let mut ticker = Ticker::every(Duration::from_micros(100));

    loop {
        ticker.next().await;

        // Read BEMF channels
        // PA4 -> ADC2_IN17, PB11 -> ADC1_IN14, PB12 -> ADC1_IN11
        let phase_a = adc2.blocking_read(&mut bemf_a_pin, bemf_sample_time);
        let phase_b = adc1.blocking_read(&mut bemf_b_pin, bemf_sample_time);
        let phase_c = adc1.blocking_read(&mut bemf_c_pin, bemf_sample_time);

        // Send BEMF samples to control loop
        let samples = BemfSamples {
            phase_a,
            phase_b,
            phase_c,
        };
        BEMF_SIGNAL.signal(samples);

        // Read voltage less frequently (every 10th sample)
        static mut SAMPLE_COUNT: u8 = 0;
        unsafe {
            SAMPLE_COUNT = SAMPLE_COUNT.wrapping_add(1);
            if SAMPLE_COUNT % 10 == 0 {
                let vbus_raw = adc1.blocking_read(&mut vbus_pin, sense_sample_time);

                // Apply filter
                if voltage_filtered == 0 {
                    voltage_filtered = (vbus_raw as u32) << 3;
                } else {
                    voltage_filtered = voltage_filtered - (voltage_filtered >> 3) + vbus_raw as u32;
                }

                // Convert to millivolts (with voltage divider)
                // ADC: 12-bit (0-4095), Vref: 3.3V, Divider: 10.39:1
                let filtered = voltage_filtered >> 3;
                let _voltage_mv = (filtered * 3300 * 1039 / 4095 / 100) as u32;
            }
        }
    }
}

/// Input signal handling (DSHOT/PWM)
#[embassy_executor::task]
async fn input_task() {
    info!("Input task started");

    // TODO: Implement input capture for DSHOT
    // For now, simulate arming sequence

    let mut armed = false;
    let mut arm_count: u16 = 0;
    const ARM_TIME: u16 = 1000; // 1 second at 1kHz

    loop {
        Timer::after(Duration::from_millis(1)).await;

        // Simulated input (zero throttle)
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
