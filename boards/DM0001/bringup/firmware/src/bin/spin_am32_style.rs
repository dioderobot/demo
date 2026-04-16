#![no_std]
#![no_main]

use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::asm;
use cortex_m::peripheral::DWT;
use defmt::{info, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcConfig, SampleTime, VREF_CALIB_MV};
use embassy_stm32::gpio::{Input, Level, Output, OutputType, Pull, Speed};
use embassy_stm32::opamp::{OpAmp, OpAmpGain, OpAmpSpeed};
use embassy_stm32::rcc::mux::Adcsel;
use embassy_stm32::rcc::{AHBPrescaler, APBPrescaler, Sysclk};
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::Channel;
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::simple_pwm::PwmPin;
use embassy_stm32::{Peri, peripherals};
use panic_probe as _;

static DEFMT_TICKS: AtomicU32 = AtomicU32::new(0);

const ADC_FULL_SCALE: u32 = 4095;
const ARM_IDLE_CURRENT_LIMIT_MA: i32 = 1_500;
const ARM_VBUS_MAX_MV: u32 = 12_000;
const ARM_VBUS_MIN_MV: u32 = 7_000;
const ALIGN_DUTY_PERCENT: u32 = 6;
const ALIGN_HOLD_US: u32 = 180_000;
const ALIGN_STEP: usize = 0;
const ARMED_HOLD_MS: u32 = 750;
const AUTO_ARM_AT_BOOT: bool = true;
const BOOTSTRAP_PRECHARGE_US: u32 = 8_000;
const BUS_ABORT_MV: u32 = 10_800;
const BUS_BACKOFF_MV: u32 = 11_400;
const CLOSED_LOOP_BASE_DUTY_PERCENT: u32 = 8;
const CLOSED_LOOP_LOCK_ZC: u32 = 6;
const CLOSED_LOOP_MAX_COMMUTATIONS: usize = 6_000;
const CLOSED_LOOP_MAX_DUTY_PERCENT: u32 = 12;
const CLOSED_LOOP_MAX_INTERVAL_US: u32 = 4_000;
const CLOSED_LOOP_MIN_DUTY_PERCENT: u32 = 5;
const CLOSED_LOOP_MIN_INTERVAL_US: u32 = 800;
const CLOSED_LOOP_TARGET_ELECTRICAL_HZ_X100: u32 = 14_000;
const CLOSED_LOOP_TARGET_INTERVAL_US: u32 =
    100_000_000 / (6 * CLOSED_LOOP_TARGET_ELECTRICAL_HZ_X100);
const CLOSED_LOOP_TELEMETRY_EVERY_STEPS: usize = 8;
const CPU_HZ: u32 = 16_000_000;
const CURRENT_OUTPUT_UV_PER_AMP: i32 = 27_429;
const CURRENT_ZERO_CAL_SAMPLES: usize = 64;
const CURRENT_ZERO_UV_NOMINAL: i32 = 2_057_143;
const CURRENT_ZERO_UV_TOLERANCE: i32 = 250_000;
const DEADTIME_TICKS: u16 = 8;
const FORWARD_DIRECTION: bool = true;
const MAX_ZC_MISSES: u32 = 8;
const MISS_DUTY_BACKOFF_START: u32 = 4;
const MIN_WAIT_AFTER_ZC_US: u32 = 60;
const NTC_PULLDOWN_OHMS: u32 = 4_700;
const OPEN_LOOP_DUTY_END_PERCENT: u32 = 8;
const OPEN_LOOP_DUTY_START_PERCENT: u32 = 5;
const OPEN_LOOP_DWELL_END_US: u32 = 1_000;
const OPEN_LOOP_DWELL_START_US: u32 = 30_000;
const OPEN_LOOP_STEPS: usize = 160;
const OPEN_LOOP_TELEMETRY_EVERY_STEPS: usize = 8;
const POST_RUN_LOG_FRAMES: usize = 8;
const PWM_FREQ_HZ: u32 = 20_000;
const SAMPLE_TIME: SampleTime = SampleTime::CYCLES640_5;
const CLOSED_LOOP_ADVANCE_NUM: u32 = 8;
const CLOSED_LOOP_INTERVAL_MAX_PCT: u32 = 125;
const CLOSED_LOOP_INTERVAL_MIN_PCT: u32 = 80;
const ENABLE_SPEED_TRIM: bool = false;
const SPEED_PI_ENABLE_ZC: u32 = 8;
const SPEED_TRIM_I_DIV: i32 = 1_600;
const SPEED_TRIM_INTEGRAL_MAX: i32 = 8_000;
const SPEED_TRIM_P_US_PER_PERCENT: i32 = 180;
const TS_CAL1_TEMP_MC: i32 = 30_000;
const TS_CAL2_TEMP_MC: i32 = 130_000;
const TS_CAL1_ADDR: *const u16 = 0x1FFF_75A8 as *const u16;
const TS_CAL2_ADDR: *const u16 = 0x1FFF_75CA as *const u16;
const ZC_BLANKING_US: u32 = 120;
const ZC_HYSTERESIS_MV: u32 = 180;
const ZC_LOCK_INTERVAL_MAX_DELTA_PCT: u32 = 15;
const ZC_WINDOW_CLOSE_LOCKED_PCT: u32 = 68;
const ZC_WINDOW_CLOSE_SEARCH_PCT: u32 = 78;
const ZC_WINDOW_OPEN_LOCKED_PCT: u32 = 36;
const ZC_WINDOW_OPEN_SEARCH_PCT: u32 = 26;

defmt::timestamp!("{=u32}", DEFMT_TICKS.fetch_add(1, Ordering::Relaxed));

#[defmt::panic_handler]
fn defmt_panic() -> ! {
    panic_probe::hard_fault()
}

#[derive(Clone, Copy)]
struct FactoryCalibration {
    vrefint: u16,
    ts_cal1: u16,
    ts_cal2: u16,
}

#[derive(Clone, Copy)]
struct CurrentCalibration {
    zero_a_uv: i32,
    zero_b_uv: i32,
    zero_c_uv: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    A,
    B,
    C,
}

impl Default for Phase {
    fn default() -> Self {
        Self::A
    }
}

impl Phase {
    fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlState {
    Booting,
    Calibrating,
    Disarmed,
    Armed,
    Aligning,
    OpenLoop,
    ClosedLoop,
}

impl ControlState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Booting => "booting",
            Self::Calibrating => "calibrating",
            Self::Disarmed => "disarmed",
            Self::Armed => "armed",
            Self::Aligning => "aligning",
            Self::OpenLoop => "open_loop",
            Self::ClosedLoop => "closed_loop",
        }
    }
}

#[derive(Clone, Copy, Default)]
struct ControlSnapshot {
    duty_pct: u32,
    duty_target_pct: u32,
    duty_trim_pct: i32,
    comm_step: usize,
    dwell_us: u32,
    closed_loop: bool,
    lock_zc_count: u32,
    zero_crosses: u32,
    zc_observable: bool,
    comm_interval_us: u32,
    wait_us: u32,
    advance_us: u32,
    floating_phase: Phase,
    zc_expected_rising: bool,
    zc_detected: bool,
    zc_elapsed_us: u32,
    zc_sample_mv: u32,
    zc_threshold_mv: u32,
    zc_window_open_us: u32,
    zc_window_close_us: u32,
    zc_miss_count: u32,
}

#[derive(Clone, Copy)]
struct TelemetryFrame {
    seq: u32,
    state: ControlState,
    arm_requested: bool,
    duty_pct: u32,
    duty_target_pct: u32,
    duty_trim_pct: i32,
    comm_step: usize,
    dwell_us: u32,
    closed_loop: bool,
    lock_zc_count: u32,
    zero_crosses: u32,
    zc_observable: bool,
    comm_interval_us: u32,
    wait_us: u32,
    advance_us: u32,
    floating_phase: Phase,
    zc_expected_rising: bool,
    zc_detected: bool,
    zc_elapsed_us: u32,
    zc_sample_mv: u32,
    zc_threshold_mv: u32,
    zc_window_open_us: u32,
    zc_window_close_us: u32,
    zc_miss_count: u32,
    vdda_mv: u32,
    bus_mv: u32,
    bemf_a_mv: u32,
    bemf_b_mv: u32,
    bemf_c_mv: u32,
    current_a_ma: i32,
    current_b_ma: i32,
    current_c_ma: i32,
    current_a_output_mv: u32,
    current_b_output_mv: u32,
    current_c_output_mv: u32,
    ntc_mv: u32,
    ntc_ohms: i32,
    mcu_temp_mc: i32,
    bemf_gpio: bool,
    hall_a: bool,
    hall_b: bool,
    hall_c: bool,
}

#[derive(Clone, Copy)]
struct ZeroCrossEvent {
    elapsed_us: u32,
    sample_mv: u32,
    threshold_mv: u32,
}

#[derive(Clone, Copy)]
struct ZeroCrossWaitResult {
    event: Option<ZeroCrossEvent>,
    elapsed_us: u32,
}

impl CurrentCalibration {
    fn is_plausible(self) -> bool {
        (self.zero_a_uv - CURRENT_ZERO_UV_NOMINAL).abs() <= CURRENT_ZERO_UV_TOLERANCE
            && (self.zero_b_uv - CURRENT_ZERO_UV_NOMINAL).abs() <= CURRENT_ZERO_UV_TOLERANCE
            && (self.zero_c_uv - CURRENT_ZERO_UV_NOMINAL).abs() <= CURRENT_ZERO_UV_TOLERANCE
    }
}

#[embassy_executor::main(
    executor = "embassy_stm32::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(_spawner: Spawner) {
    info!("spin_am32_style_start");
    info!("am32_reference mode=software_zero_cross_floating_phase");
    info!("state_transition state={}", ControlState::Booting.as_str());

    let mut config = embassy_stm32::Config::default();
    config.rcc.hsi = true;
    config.rcc.hsi48 = None;
    config.rcc.pll = None;
    config.rcc.sys = Sysclk::HSI;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.mux.adc12sel = Adcsel::SYS;

    let p = embassy_stm32::init(config);
    enable_cycle_counter();

    let arm_input = Input::new(p.PA15, Pull::Down);
    let bemf_gpio = Input::new(p.PB5, Pull::None);
    let hall_a = Input::new(p.PB6, Pull::None);
    let hall_b = Input::new(p.PB7, Pull::None);
    let hall_c = Input::new(p.PB8, Pull::None);

    let mut adc1 = Adc::new(p.ADC1, AdcConfig::default());
    let mut adc2 = Adc::new(p.ADC2, AdcConfig::default());

    let mut vbus = p.PA0;
    let mut bemf_a = p.PA4;
    let mut bemf_b = p.PB12;
    let mut bemf_c = p.PB11;
    let mut board_ntc = p.PB14;

    let mut vrefint = adc1.enable_vrefint();
    let mut mcu_temp = adc1.enable_temperature();
    let calibration = FactoryCalibration::read();

    let mut opamp1 = OpAmp::new(p.OPAMP1, OpAmpSpeed::Normal);
    let mut opamp2 = OpAmp::new(p.OPAMP2, OpAmpSpeed::Normal);
    let mut opamp3 = OpAmp::new(p.OPAMP3, OpAmpSpeed::Normal);
    opamp1.calibrate();
    opamp2.calibrate();
    opamp3.calibrate();

    let mut current_a = opamp1.pga_ext(p.PA1, p.PA2, OpAmpGain::Mul16);
    let mut current_b = opamp2.pga_ext(p.PA7, p.PA6, OpAmpGain::Mul16);
    let mut current_c = opamp3.pga_ext(p.PB0, p.PB1, OpAmpGain::Mul16);

    let mut status_led = Output::new(p.PC15, Level::Low, Speed::Low);

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
        Hertz::hz(PWM_FREQ_HZ),
        CountingMode::CenterAlignedBothInterrupts,
    );
    pwm.set_dead_time(DEADTIME_TICKS);
    pwm.set_master_output_enable(false);
    disable_all_channels(&mut pwm);

    info!("bridge_inputs_hiz_using_driver_pulldowns");
    info!(
        "state_transition state={}",
        ControlState::Calibrating.as_str()
    );
    let current_calibration = calibrate_current_offsets(
        &mut adc1,
        &mut adc2,
        &mut current_a,
        &mut current_b,
        &mut current_c,
        &mut vrefint,
        calibration,
    );
    info!(
        "current_zero_calibration nominal_zero_mv={} measured_zero_mv=[{},{},{}] output_mv_per_amp={}",
        CURRENT_ZERO_UV_NOMINAL / 1_000,
        current_calibration.zero_a_uv / 1_000,
        current_calibration.zero_b_uv / 1_000,
        current_calibration.zero_c_uv / 1_000,
        CURRENT_OUTPUT_UV_PER_AMP / 1_000
    );
    info!(
        "arm_policy auto_arm_at_boot={} arm_input_pin=PA15 target_comm_interval_us={}",
        AUTO_ARM_AT_BOOT, CLOSED_LOOP_TARGET_INTERVAL_US
    );

    let mut seq = 0u32;
    info!("state_transition state={}", ControlState::Disarmed.as_str());
    loop {
        let arm_requested = AUTO_ARM_AT_BOOT || arm_input.is_high();
        let frame = measure_frame(
            seq,
            ControlState::Disarmed,
            arm_requested,
            ControlSnapshot::default(),
            &mut adc1,
            &mut adc2,
            &mut vbus,
            &mut bemf_a,
            &mut bemf_b,
            &mut bemf_c,
            &mut board_ntc,
            &mut current_a,
            &mut current_b,
            &mut current_c,
            &mut mcu_temp,
            &mut vrefint,
            &bemf_gpio,
            &hall_a,
            &hall_b,
            &hall_c,
            current_calibration,
            calibration,
        );
        let arm_ready = is_arm_ready(frame, current_calibration);
        log_frame(frame, arm_ready);
        seq = seq.wrapping_add(1);
        if arm_ready && arm_requested {
            break;
        }
        hold_disarmed(&mut status_led, 250);
    }

    info!("state_transition state={}", ControlState::Armed.as_str());
    let arm_requested = AUTO_ARM_AT_BOOT || arm_input.is_high();
    let armed_frame = measure_frame(
        seq,
        ControlState::Armed,
        arm_requested,
        ControlSnapshot::default(),
        &mut adc1,
        &mut adc2,
        &mut vbus,
        &mut bemf_a,
        &mut bemf_b,
        &mut bemf_c,
        &mut board_ntc,
        &mut current_a,
        &mut current_b,
        &mut current_c,
        &mut mcu_temp,
        &mut vrefint,
        &bemf_gpio,
        &hall_a,
        &hall_b,
        &hall_c,
        current_calibration,
        calibration,
    );
    log_frame(armed_frame, true);
    seq = seq.wrapping_add(1);
    hold_disarmed(&mut status_led, ARMED_HOLD_MS);

    pwm.set_master_output_enable(true);
    info!(
        "bootstrap_precharge_start duration_us={} mode=all_low_sides_on",
        BOOTSTRAP_PRECHARGE_US
    );
    pwm.set_duty(Channel::Ch1, 0);
    pwm.set_duty(Channel::Ch2, 0);
    pwm.set_duty(Channel::Ch3, 0);
    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);
    delay_us(BOOTSTRAP_PRECHARGE_US);
    disable_all_channels(&mut pwm);
    info!("bootstrap_precharge_complete");

    let max_duty = pwm.get_max_duty();
    let mut control = ControlSnapshot {
        duty_pct: ALIGN_DUTY_PERCENT,
        duty_target_pct: ALIGN_DUTY_PERCENT,
        duty_trim_pct: 0,
        comm_step: ALIGN_STEP,
        dwell_us: ALIGN_HOLD_US,
        closed_loop: false,
        lock_zc_count: 0,
        zero_crosses: 0,
        zc_observable: true,
        comm_interval_us: OPEN_LOOP_DWELL_START_US,
        wait_us: 0,
        advance_us: 0,
        floating_phase: floating_phase_for_step(ALIGN_STEP),
        zc_expected_rising: expected_zc_rising(FORWARD_DIRECTION, ALIGN_STEP),
        zc_detected: false,
        zc_elapsed_us: 0,
        zc_sample_mv: 0,
        zc_threshold_mv: 0,
        zc_window_open_us: 0,
        zc_window_close_us: 0,
        zc_miss_count: 0,
    };

    info!("state_transition state={}", ControlState::Aligning.as_str());
    info!(
        "alignment_start step={} duty_pct={} hold_us={} direction={}",
        ALIGN_STEP,
        ALIGN_DUTY_PERCENT,
        ALIGN_HOLD_US,
        if FORWARD_DIRECTION {
            "forward"
        } else {
            "reverse"
        }
    );
    apply_commutation_step(
        &mut pwm,
        ALIGN_STEP,
        duty_to_counts(max_duty, ALIGN_DUTY_PERCENT),
    );
    let align_frame = measure_frame(
        seq,
        ControlState::Aligning,
        true,
        control,
        &mut adc1,
        &mut adc2,
        &mut vbus,
        &mut bemf_a,
        &mut bemf_b,
        &mut bemf_c,
        &mut board_ntc,
        &mut current_a,
        &mut current_b,
        &mut current_c,
        &mut mcu_temp,
        &mut vrefint,
        &bemf_gpio,
        &hall_a,
        &hall_b,
        &hall_c,
        current_calibration,
        calibration,
    );
    log_frame(align_frame, true);
    seq = seq.wrapping_add(1);
    delay_us(ALIGN_HOLD_US);

    info!(
        "open_loop_profile steps={} duty_pct_start={} duty_pct_end={} dwell_us_start={} dwell_us_end={}",
        OPEN_LOOP_STEPS,
        OPEN_LOOP_DUTY_START_PERCENT,
        OPEN_LOOP_DUTY_END_PERCENT,
        OPEN_LOOP_DWELL_START_US,
        OPEN_LOOP_DWELL_END_US
    );
    info!("state_transition state={}", ControlState::OpenLoop.as_str());

    let mut last_open_loop_step = ALIGN_STEP;
    let mut next_step = next_comm_step(ALIGN_STEP, FORWARD_DIRECTION);
    let mut last_open_loop_dwell = OPEN_LOOP_DWELL_END_US;
    let mut last_open_loop_duty = OPEN_LOOP_DUTY_END_PERCENT;
    let mut abort_run = false;

    for open_idx in 0..OPEN_LOOP_STEPS {
        let arm_requested = AUTO_ARM_AT_BOOT || arm_input.is_high();
        if !arm_requested {
            warn!(
                "run_abort reason=arm_input_low stage=open_loop idx={}",
                open_idx
            );
            abort_run = true;
            break;
        }

        let (vdda_mv, bus_mv) = sample_bus_vdda_mv(&mut adc1, &mut vbus, &mut vrefint, calibration);
        if bus_mv < BUS_ABORT_MV {
            warn!(
                "run_abort reason=bus_abort stage=open_loop idx={} bus_mv={}",
                open_idx, bus_mv
            );
            abort_run = true;
            break;
        }

        last_open_loop_step = next_step;
        last_open_loop_dwell = interpolate(
            OPEN_LOOP_DWELL_START_US,
            OPEN_LOOP_DWELL_END_US,
            open_idx,
            OPEN_LOOP_STEPS,
        );
        last_open_loop_duty = interpolate(
            OPEN_LOOP_DUTY_START_PERCENT,
            OPEN_LOOP_DUTY_END_PERCENT,
            open_idx,
            OPEN_LOOP_STEPS,
        );
        control = ControlSnapshot {
            duty_pct: last_open_loop_duty,
            duty_target_pct: last_open_loop_duty,
            duty_trim_pct: 0,
            comm_step: next_step,
            dwell_us: last_open_loop_dwell,
            closed_loop: false,
            lock_zc_count: 0,
            zero_crosses: 0,
            zc_observable: true,
            comm_interval_us: last_open_loop_dwell,
            wait_us: 0,
            advance_us: 0,
            floating_phase: floating_phase_for_step(next_step),
            zc_expected_rising: expected_zc_rising(FORWARD_DIRECTION, next_step),
            zc_detected: false,
            zc_elapsed_us: 0,
            zc_sample_mv: vdda_mv / 2,
            zc_threshold_mv: vdda_mv / 2,
            zc_window_open_us: 0,
            zc_window_close_us: 0,
            zc_miss_count: 0,
        };

        if bus_mv < BUS_BACKOFF_MV && control.duty_pct > CLOSED_LOOP_MIN_DUTY_PERCENT {
            control.duty_pct -= 1;
            control.duty_target_pct = control.duty_pct;
        }

        apply_commutation_step(
            &mut pwm,
            next_step,
            duty_to_counts(max_duty, control.duty_pct),
        );

        if open_idx % OPEN_LOOP_TELEMETRY_EVERY_STEPS == 0 || open_idx + 1 == OPEN_LOOP_STEPS {
            let frame = measure_frame(
                seq,
                ControlState::OpenLoop,
                arm_requested,
                control,
                &mut adc1,
                &mut adc2,
                &mut vbus,
                &mut bemf_a,
                &mut bemf_b,
                &mut bemf_c,
                &mut board_ntc,
                &mut current_a,
                &mut current_b,
                &mut current_c,
                &mut mcu_temp,
                &mut vrefint,
                &bemf_gpio,
                &hall_a,
                &hall_b,
                &hall_c,
                current_calibration,
                calibration,
            );
            log_frame(frame, true);
            seq = seq.wrapping_add(1);
        }

        status_led.toggle();
        delay_us(control.dwell_us);
        next_step = next_comm_step(next_step, FORWARD_DIRECTION);
    }

    if !abort_run {
        info!(
            "closed_loop_profile max_commutations={} target_interval_us={} base_duty_pct={} max_duty_pct={} zc_blanking_us={} zc_hysteresis_mv={}",
            CLOSED_LOOP_MAX_COMMUTATIONS,
            CLOSED_LOOP_TARGET_INTERVAL_US,
            CLOSED_LOOP_BASE_DUTY_PERCENT,
            CLOSED_LOOP_MAX_DUTY_PERCENT,
            ZC_BLANKING_US,
            ZC_HYSTERESIS_MV
        );
        info!(
            "state_transition state={}",
            ControlState::ClosedLoop.as_str()
        );

        let mut active_step = last_open_loop_step;
        let mut speed_trim_integral = 0i32;
        let mut zc_miss_count = 0u32;
        control = ControlSnapshot {
            duty_pct: last_open_loop_duty.max(CLOSED_LOOP_BASE_DUTY_PERCENT),
            duty_target_pct: last_open_loop_duty.max(CLOSED_LOOP_BASE_DUTY_PERCENT),
            duty_trim_pct: 0,
            comm_step: active_step,
            dwell_us: last_open_loop_dwell,
            closed_loop: true,
            lock_zc_count: 0,
            zero_crosses: 0,
            zc_observable: true,
            comm_interval_us: last_open_loop_dwell,
            wait_us: last_open_loop_dwell / 2,
            advance_us: 0,
            floating_phase: floating_phase_for_step(active_step),
            zc_expected_rising: expected_zc_rising(FORWARD_DIRECTION, active_step),
            zc_detected: false,
            zc_elapsed_us: 0,
            zc_sample_mv: 0,
            zc_threshold_mv: 0,
            zc_window_open_us: 0,
            zc_window_close_us: 0,
            zc_miss_count: 0,
        };

        for closed_idx in 0..CLOSED_LOOP_MAX_COMMUTATIONS {
            let arm_requested = AUTO_ARM_AT_BOOT || arm_input.is_high();
            if !arm_requested {
                warn!(
                    "run_abort reason=arm_input_low stage=closed_loop idx={}",
                    closed_idx
                );
                break;
            }

            let (vdda_mv, bus_mv) =
                sample_bus_vdda_mv(&mut adc1, &mut vbus, &mut vrefint, calibration);
            if bus_mv < BUS_ABORT_MV {
                warn!(
                    "run_abort reason=bus_abort stage=closed_loop idx={} bus_mv={}",
                    closed_idx, bus_mv
                );
                break;
            }

            if bus_mv < BUS_BACKOFF_MV && control.duty_target_pct > CLOSED_LOOP_MIN_DUTY_PERCENT {
                control.duty_target_pct -= 1;
            }

            control.comm_step = active_step;
            control.floating_phase = floating_phase_for_step(active_step);
            control.zc_expected_rising = expected_zc_rising(FORWARD_DIRECTION, active_step);
            control.zc_observable = observable_zc_step(FORWARD_DIRECTION, active_step);
            control.zc_detected = false;
            control.zc_elapsed_us = 0;
            control.zc_sample_mv = 0;
            control.zc_threshold_mv = vdda_mv / 2;
            control.zc_window_open_us = 0;
            control.zc_window_close_us = 0;
            control.zc_miss_count = zc_miss_count;

            let zc_locked = control.lock_zc_count >= CLOSED_LOOP_LOCK_ZC;
            if control.zc_observable {
                let window_open_pct = if zc_locked {
                    ZC_WINDOW_OPEN_LOCKED_PCT
                } else {
                    ZC_WINDOW_OPEN_SEARCH_PCT
                };
                let window_close_pct = if zc_locked {
                    ZC_WINDOW_CLOSE_LOCKED_PCT
                } else {
                    ZC_WINDOW_CLOSE_SEARCH_PCT
                };
                let window_open_us = max_u32(
                    ZC_BLANKING_US,
                    control.comm_interval_us.saturating_mul(window_open_pct) / 100,
                );
                let mut window_close_us =
                    control.comm_interval_us.saturating_mul(window_close_pct) / 100;
                window_close_us = clamp_u32(
                    window_close_us,
                    window_open_us.saturating_add(MIN_WAIT_AFTER_ZC_US),
                    control
                        .comm_interval_us
                        .saturating_sub(MIN_WAIT_AFTER_ZC_US)
                        .max(window_open_us.saturating_add(MIN_WAIT_AFTER_ZC_US)),
                );
                control.zc_window_open_us = window_open_us;
                control.zc_window_close_us = window_close_us;

                let wait_result = wait_for_zero_cross(
                    &mut adc1,
                    &mut adc2,
                    &mut bemf_a,
                    &mut bemf_b,
                    &mut bemf_c,
                    control.floating_phase,
                    control.zc_expected_rising,
                    vdda_mv,
                    ZC_BLANKING_US,
                    window_open_us,
                    window_close_us,
                );

                match wait_result.event {
                    Some(zc) => {
                        let sample_interval_us = clamp_u32(
                            zc.elapsed_us.saturating_mul(2),
                            CLOSED_LOOP_MIN_INTERVAL_US,
                            CLOSED_LOOP_MAX_INTERVAL_US,
                        );
                        let interval_consistent = within_percent_u32(
                            sample_interval_us,
                            control.comm_interval_us,
                            ZC_LOCK_INTERVAL_MAX_DELTA_PCT,
                        );

                        zc_miss_count = 0;
                        control.zero_crosses = control.zero_crosses.saturating_add(1);
                        control.lock_zc_count = if interval_consistent {
                            control
                                .lock_zc_count
                                .saturating_add(1)
                                .min(CLOSED_LOOP_LOCK_ZC)
                        } else {
                            1
                        };
                        control.zc_detected = true;
                        control.zc_elapsed_us = zc.elapsed_us;
                        control.zc_sample_mv = zc.sample_mv;
                        control.zc_threshold_mv = zc.threshold_mv;
                        let filtered_sample_interval_us = clamp_u32(
                            sample_interval_us,
                            control
                                .comm_interval_us
                                .saturating_mul(CLOSED_LOOP_INTERVAL_MIN_PCT)
                                / 100,
                            control
                                .comm_interval_us
                                .saturating_mul(CLOSED_LOOP_INTERVAL_MAX_PCT)
                                / 100,
                        );
                        control.comm_interval_us =
                            low_pass_u32(control.comm_interval_us, filtered_sample_interval_us);
                        control.advance_us = control
                            .comm_interval_us
                            .saturating_mul(CLOSED_LOOP_ADVANCE_NUM)
                            / 64;
                        control.wait_us = max_u32(
                            MIN_WAIT_AFTER_ZC_US,
                            control
                                .comm_interval_us
                                .saturating_sub(zc.elapsed_us)
                                .saturating_sub(control.advance_us),
                        );

                        if ENABLE_SPEED_TRIM
                            && zc_locked
                            && control.zero_crosses >= SPEED_PI_ENABLE_ZC
                        {
                            control.duty_trim_pct =
                                speed_trim_pct(control.comm_interval_us, &mut speed_trim_integral);
                        } else {
                            speed_trim_integral = 0;
                            control.duty_trim_pct = 0;
                        }

                        control.duty_pct = clamp_pct(
                            control.duty_target_pct as i32 + control.duty_trim_pct,
                            CLOSED_LOOP_MIN_DUTY_PERCENT,
                            CLOSED_LOOP_MAX_DUTY_PERCENT,
                        );
                    }
                    None => {
                        zc_miss_count = zc_miss_count.saturating_add(1);
                        control.lock_zc_count = control.lock_zc_count.saturating_sub(1);
                        control.duty_trim_pct = 0;
                        control.zc_elapsed_us = wait_result.elapsed_us;
                        control.wait_us = max_u32(
                            MIN_WAIT_AFTER_ZC_US,
                            control
                                .comm_interval_us
                                .saturating_sub(wait_result.elapsed_us),
                        );
                        if zc_miss_count >= MISS_DUTY_BACKOFF_START
                            && control.duty_target_pct > CLOSED_LOOP_BASE_DUTY_PERCENT
                        {
                            control.duty_target_pct -= 1;
                        }
                        control.duty_pct =
                            control.duty_target_pct.max(CLOSED_LOOP_BASE_DUTY_PERCENT);
                        warn!(
                            "zc_miss idx={} step={} floating={} rising={} elapsed_us={} wait_us={} misses={}",
                            closed_idx,
                            control.comm_step,
                            control.floating_phase.as_str(),
                            control.zc_expected_rising,
                            wait_result.elapsed_us,
                            control.wait_us,
                            zc_miss_count
                        );
                        if zc_miss_count >= MAX_ZC_MISSES {
                            warn!(
                                "run_abort reason=zc_timeout stage=closed_loop idx={}",
                                closed_idx
                            );
                            break;
                        }
                    }
                }
            } else {
                zc_miss_count = 0;
                control.zc_detected = false;
                control.zc_elapsed_us = 0;
                control.zc_sample_mv = 0;
                control.zc_window_open_us = 0;
                control.zc_window_close_us = 0;
                control.advance_us = control
                    .comm_interval_us
                    .saturating_mul(CLOSED_LOOP_ADVANCE_NUM)
                    / 64;
                control.wait_us = max_u32(
                    MIN_WAIT_AFTER_ZC_US,
                    control.comm_interval_us.saturating_sub(control.advance_us),
                );
                control.duty_trim_pct = 0;
                control.duty_pct = control.duty_target_pct.max(CLOSED_LOOP_BASE_DUTY_PERCENT);
            }
            control.zc_miss_count = zc_miss_count;

            if closed_idx % CLOSED_LOOP_TELEMETRY_EVERY_STEPS == 0
                || control.zc_detected
                || zc_miss_count > 0
                || control.zero_crosses <= CLOSED_LOOP_LOCK_ZC
            {
                let frame = measure_frame(
                    seq,
                    ControlState::ClosedLoop,
                    arm_requested,
                    control,
                    &mut adc1,
                    &mut adc2,
                    &mut vbus,
                    &mut bemf_a,
                    &mut bemf_b,
                    &mut bemf_c,
                    &mut board_ntc,
                    &mut current_a,
                    &mut current_b,
                    &mut current_c,
                    &mut mcu_temp,
                    &mut vrefint,
                    &bemf_gpio,
                    &hall_a,
                    &hall_b,
                    &hall_c,
                    current_calibration,
                    calibration,
                );
                log_frame(frame, true);
                seq = seq.wrapping_add(1);
            }

            delay_us(control.wait_us);

            active_step = next_comm_step(active_step, FORWARD_DIRECTION);
            apply_commutation_step(
                &mut pwm,
                active_step,
                duty_to_counts(max_duty, control.duty_pct),
            );
            status_led.toggle();
        }
    }

    pwm.set_master_output_enable(false);
    disable_all_channels(&mut pwm);
    info!("state_transition state={}", ControlState::Disarmed.as_str());

    for _ in 0..POST_RUN_LOG_FRAMES {
        let arm_requested = AUTO_ARM_AT_BOOT || arm_input.is_high();
        let frame = measure_frame(
            seq,
            ControlState::Disarmed,
            arm_requested,
            ControlSnapshot::default(),
            &mut adc1,
            &mut adc2,
            &mut vbus,
            &mut bemf_a,
            &mut bemf_b,
            &mut bemf_c,
            &mut board_ntc,
            &mut current_a,
            &mut current_b,
            &mut current_c,
            &mut mcu_temp,
            &mut vrefint,
            &bemf_gpio,
            &hall_a,
            &hall_b,
            &hall_c,
            current_calibration,
            calibration,
        );
        let arm_ready = is_arm_ready(frame, current_calibration);
        log_frame(frame, arm_ready);
        seq = seq.wrapping_add(1);
        hold_disarmed(&mut status_led, 250);
    }

    info!("spin_am32_style_complete idling_disarmed");
    loop {
        hold_disarmed(&mut status_led, 500);
    }
}

fn enable_cycle_counter() {
    let mut cp = unsafe { cortex_m::Peripherals::steal() };
    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();
}

fn apply_commutation_step(
    pwm: &mut ComplementaryPwm<'_, peripherals::TIM1>,
    step: usize,
    duty_counts: u32,
) {
    disable_all_channels(pwm);
    match step % 6 {
        0 => {
            pwm.set_duty(Channel::Ch1, duty_counts);
            pwm.set_duty(Channel::Ch2, 0);
            pwm.enable(Channel::Ch1);
            pwm.enable(Channel::Ch2);
        }
        1 => {
            pwm.set_duty(Channel::Ch1, duty_counts);
            pwm.set_duty(Channel::Ch3, 0);
            pwm.enable(Channel::Ch1);
            pwm.enable(Channel::Ch3);
        }
        2 => {
            pwm.set_duty(Channel::Ch2, duty_counts);
            pwm.set_duty(Channel::Ch3, 0);
            pwm.enable(Channel::Ch2);
            pwm.enable(Channel::Ch3);
        }
        3 => {
            pwm.set_duty(Channel::Ch2, duty_counts);
            pwm.set_duty(Channel::Ch1, 0);
            pwm.enable(Channel::Ch2);
            pwm.enable(Channel::Ch1);
        }
        4 => {
            pwm.set_duty(Channel::Ch3, duty_counts);
            pwm.set_duty(Channel::Ch1, 0);
            pwm.enable(Channel::Ch3);
            pwm.enable(Channel::Ch1);
        }
        _ => {
            pwm.set_duty(Channel::Ch3, duty_counts);
            pwm.set_duty(Channel::Ch2, 0);
            pwm.enable(Channel::Ch3);
            pwm.enable(Channel::Ch2);
        }
    }
}

fn calibrate_current_offsets(
    adc1: &mut Adc<'_, embassy_stm32::peripherals::ADC1>,
    adc2: &mut Adc<'_, embassy_stm32::peripherals::ADC2>,
    current_a: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP1>,
    current_b: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP2>,
    current_c: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP3>,
    vrefint: &mut embassy_stm32::adc::VrefInt,
    calibration: FactoryCalibration,
) -> CurrentCalibration {
    let mut sum_a_uv = 0i64;
    let mut sum_b_uv = 0i64;
    let mut sum_c_uv = 0i64;

    for _ in 0..CURRENT_ZERO_CAL_SAMPLES {
        let vdda_mv = estimate_vdda_mv(
            adc1.blocking_read(vrefint, SAMPLE_TIME),
            calibration.vrefint,
        );
        sum_a_uv += i64::from(adc_raw_to_uv(
            adc1.blocking_read(current_a, SAMPLE_TIME),
            vdda_mv,
        ));
        sum_b_uv += i64::from(adc_raw_to_uv(
            adc2.blocking_read(current_b, SAMPLE_TIME),
            vdda_mv,
        ));
        sum_c_uv += i64::from(adc_raw_to_uv(
            adc1.blocking_read(current_c, SAMPLE_TIME),
            vdda_mv,
        ));
    }

    CurrentCalibration {
        zero_a_uv: (sum_a_uv / CURRENT_ZERO_CAL_SAMPLES as i64) as i32,
        zero_b_uv: (sum_b_uv / CURRENT_ZERO_CAL_SAMPLES as i64) as i32,
        zero_c_uv: (sum_c_uv / CURRENT_ZERO_CAL_SAMPLES as i64) as i32,
    }
}

fn disable_all_channels(pwm: &mut ComplementaryPwm<'_, peripherals::TIM1>) {
    pwm.set_duty(Channel::Ch1, 0);
    pwm.set_duty(Channel::Ch2, 0);
    pwm.set_duty(Channel::Ch3, 0);
    pwm.disable(Channel::Ch1);
    pwm.disable(Channel::Ch2);
    pwm.disable(Channel::Ch3);
}

fn delay_us(us: u32) {
    let cycles = ((u64::from(CPU_HZ) * u64::from(us)) / 1_000_000) as u32;
    asm::delay(cycles.max(1));
}

fn duty_to_counts(max_duty: u32, duty_pct: u32) -> u32 {
    max_duty.saturating_mul(duty_pct) / 100
}

fn wait_for_zero_cross(
    adc1: &mut Adc<'_, embassy_stm32::peripherals::ADC1>,
    adc2: &mut Adc<'_, embassy_stm32::peripherals::ADC2>,
    bemf_a: &mut Peri<'_, peripherals::PA4>,
    bemf_b: &mut Peri<'_, peripherals::PB12>,
    bemf_c: &mut Peri<'_, peripherals::PB11>,
    floating_phase: Phase,
    expected_rising: bool,
    vdda_mv: u32,
    blanking_us: u32,
    window_open_us: u32,
    window_close_us: u32,
) -> ZeroCrossWaitResult {
    let start_cycles = DWT::cycle_count();
    let blanking_cycles = us_to_cycles(blanking_us);
    while DWT::cycle_count().wrapping_sub(start_cycles) < blanking_cycles {
        asm::nop();
    }

    let threshold_mv = vdda_mv / 2;
    let threshold_high_mv = threshold_mv.saturating_add(ZC_HYSTERESIS_MV);
    let threshold_low_mv = threshold_mv.saturating_sub(ZC_HYSTERESIS_MV);
    let window_open_cycles = us_to_cycles(window_open_us);
    let window_close_cycles = us_to_cycles(window_close_us);
    let mut previous_sample_mv =
        read_bemf_phase_mv(adc1, adc2, bemf_a, bemf_b, bemf_c, floating_phase, vdda_mv);

    while DWT::cycle_count().wrapping_sub(start_cycles) < window_close_cycles {
        let sample_mv =
            read_bemf_phase_mv(adc1, adc2, bemf_a, bemf_b, bemf_c, floating_phase, vdda_mv);
        let elapsed_cycles = DWT::cycle_count().wrapping_sub(start_cycles);
        if elapsed_cycles < window_open_cycles {
            previous_sample_mv = sample_mv;
            continue;
        }
        let crossed = if expected_rising {
            previous_sample_mv <= threshold_low_mv && sample_mv >= threshold_high_mv
        } else {
            previous_sample_mv >= threshold_high_mv && sample_mv <= threshold_low_mv
        };
        if crossed {
            let confirm_mv =
                read_bemf_phase_mv(adc1, adc2, bemf_a, bemf_b, bemf_c, floating_phase, vdda_mv);
            let confirmed = if expected_rising {
                confirm_mv >= threshold_high_mv
            } else {
                confirm_mv <= threshold_low_mv
            };
            if confirmed {
                return ZeroCrossWaitResult {
                    event: Some(ZeroCrossEvent {
                        elapsed_us: cycles_to_us(elapsed_cycles),
                        sample_mv: confirm_mv,
                        threshold_mv,
                    }),
                    elapsed_us: cycles_to_us(elapsed_cycles),
                };
            }
            previous_sample_mv = confirm_mv;
            continue;
        }
        previous_sample_mv = sample_mv;
    }

    ZeroCrossWaitResult {
        event: None,
        elapsed_us: window_close_us,
    }
}

fn read_bemf_phase_mv(
    adc1: &mut Adc<'_, embassy_stm32::peripherals::ADC1>,
    adc2: &mut Adc<'_, embassy_stm32::peripherals::ADC2>,
    bemf_a: &mut Peri<'_, peripherals::PA4>,
    bemf_b: &mut Peri<'_, peripherals::PB12>,
    bemf_c: &mut Peri<'_, peripherals::PB11>,
    phase: Phase,
    vdda_mv: u32,
) -> u32 {
    match phase {
        Phase::A => adc_raw_to_mv(adc2.blocking_read(bemf_a, SAMPLE_TIME), vdda_mv),
        Phase::B => adc_raw_to_mv(adc1.blocking_read(bemf_b, SAMPLE_TIME), vdda_mv),
        Phase::C => adc_raw_to_mv(adc2.blocking_read(bemf_c, SAMPLE_TIME), vdda_mv),
    }
}

fn sample_bus_vdda_mv(
    adc1: &mut Adc<'_, embassy_stm32::peripherals::ADC1>,
    vbus: &mut Peri<'_, peripherals::PA0>,
    vrefint: &mut embassy_stm32::adc::VrefInt,
    calibration: FactoryCalibration,
) -> (u32, u32) {
    let vdda_mv = estimate_vdda_mv(
        adc1.blocking_read(vrefint, SAMPLE_TIME),
        calibration.vrefint,
    );
    let bus_mv = estimate_bus_mv(adc1.blocking_read(vbus, SAMPLE_TIME), vdda_mv);
    (vdda_mv, bus_mv)
}

fn expected_zc_rising(forward: bool, step: usize) -> bool {
    let rising = step % 2 == 1;
    if forward { rising } else { !rising }
}

fn observable_zc_step(forward: bool, step: usize) -> bool {
    let observable = step % 2 == 0;
    if forward { observable } else { !observable }
}

fn floating_phase_for_step(step: usize) -> Phase {
    match step % 6 {
        0 | 3 => Phase::C,
        1 | 4 => Phase::B,
        _ => Phase::A,
    }
}

fn hold_disarmed(status_led: &mut Output<'_>, ms: u32) {
    status_led.toggle();
    delay_us(ms * 1_000);
}

fn interpolate(start: u32, end: u32, index: usize, len: usize) -> u32 {
    if len <= 1 {
        return end;
    }

    let start = i64::from(start);
    let end = i64::from(end);
    let delta = end - start;
    let step = delta * index as i64 / (len as i64 - 1);

    (start + step) as u32
}

fn is_arm_ready(frame: TelemetryFrame, current_calibration: CurrentCalibration) -> bool {
    current_calibration.is_plausible()
        && (ARM_VBUS_MIN_MV..=ARM_VBUS_MAX_MV).contains(&frame.bus_mv)
        && frame.current_a_ma.abs() <= ARM_IDLE_CURRENT_LIMIT_MA
        && frame.current_b_ma.abs() <= ARM_IDLE_CURRENT_LIMIT_MA
        && frame.current_c_ma.abs() <= ARM_IDLE_CURRENT_LIMIT_MA
}

fn log_frame(frame: TelemetryFrame, arm_ready: bool) {
    info!(
        "telemetry={{seq:{},state:{},arm_ready:{},arm_req:{},closed_loop:{},lock_zc:{},zero_crosses:{},zc_observable:{},zc_miss_count:{},duty_pct:{},duty_target_pct:{},duty_trim_pct:{},comm_step:{},dwell_us:{},comm_interval_us:{},wait_us:{},advance_us:{},floating:{},zc_rising:{},zc_detected:{},zc_elapsed_us:{},zc_window_open_us:{},zc_window_close_us:{},zc_mv:{},zc_threshold_mv:{},vdda_mv:{},bus_mv:{},bemf_mv:[{},{},{}],current_ma:[{},{},{}],current_out_mv:[{},{},{}],ntc_mv:{},ntc_ohms:{},mcu_temp_mc:{},bemf_gpio:{},hall:[{},{},{}]}}",
        frame.seq,
        frame.state.as_str(),
        arm_ready,
        frame.arm_requested,
        frame.closed_loop,
        frame.lock_zc_count,
        frame.zero_crosses,
        frame.zc_observable,
        frame.zc_miss_count,
        frame.duty_pct,
        frame.duty_target_pct,
        frame.duty_trim_pct,
        frame.comm_step,
        frame.dwell_us,
        frame.comm_interval_us,
        frame.wait_us,
        frame.advance_us,
        frame.floating_phase.as_str(),
        frame.zc_expected_rising,
        frame.zc_detected,
        frame.zc_elapsed_us,
        frame.zc_window_open_us,
        frame.zc_window_close_us,
        frame.zc_sample_mv,
        frame.zc_threshold_mv,
        frame.vdda_mv,
        frame.bus_mv,
        frame.bemf_a_mv,
        frame.bemf_b_mv,
        frame.bemf_c_mv,
        frame.current_a_ma,
        frame.current_b_ma,
        frame.current_c_ma,
        frame.current_a_output_mv,
        frame.current_b_output_mv,
        frame.current_c_output_mv,
        frame.ntc_mv,
        frame.ntc_ohms,
        frame.mcu_temp_mc,
        frame.bemf_gpio,
        frame.hall_a,
        frame.hall_b,
        frame.hall_c
    );
}

fn low_pass_u32(previous: u32, sample: u32) -> u32 {
    ((u64::from(previous) * 3 + u64::from(sample) + 2) / 4) as u32
}

#[allow(clippy::too_many_arguments)]
fn measure_frame(
    seq: u32,
    state: ControlState,
    arm_requested: bool,
    control: ControlSnapshot,
    adc1: &mut Adc<'_, embassy_stm32::peripherals::ADC1>,
    adc2: &mut Adc<'_, embassy_stm32::peripherals::ADC2>,
    vbus: &mut Peri<'_, peripherals::PA0>,
    bemf_a: &mut Peri<'_, peripherals::PA4>,
    bemf_b: &mut Peri<'_, peripherals::PB12>,
    bemf_c: &mut Peri<'_, peripherals::PB11>,
    board_ntc: &mut Peri<'_, peripherals::PB14>,
    current_a: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP1>,
    current_b: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP2>,
    current_c: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP3>,
    mcu_temp: &mut embassy_stm32::adc::Temperature,
    vrefint: &mut embassy_stm32::adc::VrefInt,
    bemf_gpio: &Input<'_>,
    hall_a: &Input<'_>,
    hall_b: &Input<'_>,
    hall_c: &Input<'_>,
    current_calibration: CurrentCalibration,
    calibration: FactoryCalibration,
) -> TelemetryFrame {
    let vdda_mv = estimate_vdda_mv(
        adc1.blocking_read(vrefint, SAMPLE_TIME),
        calibration.vrefint,
    );
    let bus_raw = adc1.blocking_read(vbus, SAMPLE_TIME);
    let bemf_a_raw = adc2.blocking_read(bemf_a, SAMPLE_TIME);
    let bemf_b_raw = adc1.blocking_read(bemf_b, SAMPLE_TIME);
    let bemf_c_raw = adc2.blocking_read(bemf_c, SAMPLE_TIME);
    let ntc_raw = adc1.blocking_read(board_ntc, SAMPLE_TIME);
    let current_a_raw = adc1.blocking_read(current_a, SAMPLE_TIME);
    let current_b_raw = adc2.blocking_read(current_b, SAMPLE_TIME);
    let current_c_raw = adc1.blocking_read(current_c, SAMPLE_TIME);
    let mcu_temp_raw = adc1.blocking_read(mcu_temp, SAMPLE_TIME);

    TelemetryFrame {
        seq,
        state,
        arm_requested,
        duty_pct: control.duty_pct,
        duty_target_pct: control.duty_target_pct,
        duty_trim_pct: control.duty_trim_pct,
        comm_step: control.comm_step,
        dwell_us: control.dwell_us,
        closed_loop: control.closed_loop,
        lock_zc_count: control.lock_zc_count,
        zero_crosses: control.zero_crosses,
        zc_observable: control.zc_observable,
        comm_interval_us: control.comm_interval_us,
        wait_us: control.wait_us,
        advance_us: control.advance_us,
        floating_phase: control.floating_phase,
        zc_expected_rising: control.zc_expected_rising,
        zc_detected: control.zc_detected,
        zc_elapsed_us: control.zc_elapsed_us,
        zc_sample_mv: control.zc_sample_mv,
        zc_threshold_mv: control.zc_threshold_mv,
        zc_window_open_us: control.zc_window_open_us,
        zc_window_close_us: control.zc_window_close_us,
        zc_miss_count: control.zc_miss_count,
        vdda_mv,
        bus_mv: estimate_bus_mv(bus_raw, vdda_mv),
        bemf_a_mv: adc_raw_to_mv(bemf_a_raw, vdda_mv),
        bemf_b_mv: adc_raw_to_mv(bemf_b_raw, vdda_mv),
        bemf_c_mv: adc_raw_to_mv(bemf_c_raw, vdda_mv),
        current_a_ma: estimate_phase_current_ma(
            current_a_raw,
            vdda_mv,
            current_calibration.zero_a_uv,
        ),
        current_b_ma: estimate_phase_current_ma(
            current_b_raw,
            vdda_mv,
            current_calibration.zero_b_uv,
        ),
        current_c_ma: estimate_phase_current_ma(
            current_c_raw,
            vdda_mv,
            current_calibration.zero_c_uv,
        ),
        current_a_output_mv: adc_raw_to_mv(current_a_raw, vdda_mv),
        current_b_output_mv: adc_raw_to_mv(current_b_raw, vdda_mv),
        current_c_output_mv: adc_raw_to_mv(current_c_raw, vdda_mv),
        ntc_mv: adc_raw_to_mv(ntc_raw, vdda_mv),
        ntc_ohms: estimate_ntc_ohms(ntc_raw),
        mcu_temp_mc: estimate_mcu_temp_mc(mcu_temp_raw, vdda_mv, calibration),
        bemf_gpio: bemf_gpio.is_high(),
        hall_a: hall_a.is_high(),
        hall_b: hall_b.is_high(),
        hall_c: hall_c.is_high(),
    }
}

fn next_comm_step(step: usize, forward: bool) -> usize {
    if forward {
        (step + 1) % 6
    } else {
        (step + 5) % 6
    }
}

fn max_u32(a: u32, b: u32) -> u32 {
    if a > b { a } else { b }
}

fn clamp_i32(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn clamp_pct(value: i32, min: u32, max: u32) -> u32 {
    clamp_i32(value, min as i32, max as i32) as u32
}

fn clamp_u32(value: u32, min: u32, max: u32) -> u32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn within_percent_u32(sample: u32, reference: u32, delta_pct: u32) -> bool {
    let min = reference.saturating_mul(100u32.saturating_sub(delta_pct)) / 100;
    let max = reference.saturating_mul(100u32.saturating_add(delta_pct)) / 100;
    (min..=max).contains(&sample)
}

fn speed_trim_pct(comm_interval_us: u32, integral: &mut i32) -> i32 {
    let error_us = i32::try_from(comm_interval_us).unwrap_or(i32::MAX)
        - i32::try_from(CLOSED_LOOP_TARGET_INTERVAL_US).unwrap_or(i32::MAX);
    *integral = clamp_i32(
        integral.saturating_add(error_us),
        -SPEED_TRIM_INTEGRAL_MAX,
        SPEED_TRIM_INTEGRAL_MAX,
    );
    clamp_i32(
        error_us / SPEED_TRIM_P_US_PER_PERCENT + *integral / SPEED_TRIM_I_DIV,
        -2,
        4,
    )
}

fn us_to_cycles(us: u32) -> u32 {
    ((u64::from(CPU_HZ) * u64::from(us)) / 1_000_000) as u32
}

fn cycles_to_us(cycles: u32) -> u32 {
    ((u64::from(cycles) * 1_000_000) / u64::from(CPU_HZ)) as u32
}

fn adc_raw_to_mv(raw: u16, vdda_mv: u32) -> u32 {
    u32::from(raw) * vdda_mv / ADC_FULL_SCALE
}

fn adc_raw_to_uv(raw: u16, vdda_mv: u32) -> i32 {
    (u64::from(raw) * u64::from(vdda_mv) * 1_000 / u64::from(ADC_FULL_SCALE)) as i32
}

fn estimate_bus_mv(bus_raw: u16, vdda_mv: u32) -> u32 {
    adc_raw_to_mv(bus_raw, vdda_mv) * 187 / 18
}

fn estimate_mcu_temp_mc(raw: u16, vdda_mv: u32, calibration: FactoryCalibration) -> i32 {
    let compensated = i32::try_from(u32::from(raw) * vdda_mv / VREF_CALIB_MV).unwrap_or_default();
    let cal_span = i32::from(calibration.ts_cal2) - i32::from(calibration.ts_cal1);
    if cal_span == 0 {
        return 0;
    }

    TS_CAL1_TEMP_MC
        + (TS_CAL2_TEMP_MC - TS_CAL1_TEMP_MC) * (compensated - i32::from(calibration.ts_cal1))
            / cal_span
}

fn estimate_ntc_ohms(raw: u16) -> i32 {
    if raw == 0 {
        return -1;
    }
    let raw = u32::from(raw);
    (NTC_PULLDOWN_OHMS * (ADC_FULL_SCALE - raw) / raw) as i32
}

fn estimate_phase_current_ma(raw: u16, vdda_mv: u32, zero_uv: i32) -> i32 {
    let delta_uv = i64::from(adc_raw_to_uv(raw, vdda_mv) - zero_uv);
    (delta_uv * 1_000 / i64::from(CURRENT_OUTPUT_UV_PER_AMP)) as i32
}

fn estimate_vdda_mv(vref_raw: u16, vrefint_cal: u16) -> u32 {
    if vref_raw == 0 {
        return VREF_CALIB_MV;
    }
    VREF_CALIB_MV * u32::from(vrefint_cal) / u32::from(vref_raw)
}

impl FactoryCalibration {
    fn read() -> Self {
        Self {
            vrefint: embassy_stm32::adc::VrefInt {}.calibrated_value(),
            ts_cal1: read_calibration_word(TS_CAL1_ADDR),
            ts_cal2: read_calibration_word(TS_CAL2_ADDR),
        }
    }
}

fn read_calibration_word(addr: *const u16) -> u16 {
    unsafe { ptr::read_volatile(addr) }
}
