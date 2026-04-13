#![no_std]
#![no_main]

use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::asm;
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
const ARM_DELAY_MS: u32 = 2_000;
const ARMED_HOLD_MS: u32 = 1_000;
const ARM_IDLE_CURRENT_LIMIT_MA: i32 = 1_500;
const ARM_VBUS_MAX_MV: u32 = 12_000;
const ARM_VBUS_MIN_MV: u32 = 7_000;
const ALIGN_DUTY_PERCENT: u32 = 6;
const ALIGN_HOLD_US: u32 = 180_000;
const ALIGN_STEP: usize = 0;
const BOOTSTRAP_PRECHARGE_US: u32 = 8_000;
const CPU_HZ: u32 = 16_000_000;
const CURRENT_OUTPUT_UV_PER_AMP: i32 = 27_429;
const CURRENT_ZERO_CAL_SAMPLES: usize = 64;
const CURRENT_ZERO_UV_NOMINAL: i32 = 2_057_143;
const CURRENT_ZERO_UV_TOLERANCE: i32 = 250_000;
const DEADTIME_TICKS: u16 = 8;
const NTC_PULLDOWN_OHMS: u32 = 4_700;
const SAMPLE_TIME: SampleTime = SampleTime::CYCLES640_5;
const PWM_FREQ_HZ: u32 = 20_000;
const POST_SPIN_LOG_FRAMES: usize = 8;
const SPIN_DUTY_END_PERCENT: u32 = 7;
const SPIN_DUTY_START_PERCENT: u32 = 5;
const SPIN_END_DWELL_US: u32 = 4_000;
const SPIN_STEPS: usize = 72;
const SPIN_START_DWELL_US: u32 = 40_000;
const TS_CAL1_TEMP_MC: i32 = 30_000;
const TS_CAL2_TEMP_MC: i32 = 130_000;
const TS_CAL1_ADDR: *const u16 = 0x1FFF_75A8 as *const u16;
const TS_CAL2_ADDR: *const u16 = 0x1FFF_75CA as *const u16;
const TELEMETRY_EVERY_STEPS: usize = 6;

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

#[derive(Clone, Copy)]
struct TelemetryFrame {
    seq: u32,
    state: SpinState,
    duty_pct: u32,
    comm_step: usize,
    dwell_us: u32,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpinState {
    Booting,
    Calibrating,
    Disarmed,
    Armed,
    Spinning,
}

impl SpinState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Booting => "booting",
            Self::Calibrating => "calibrating",
            Self::Disarmed => "disarmed",
            Self::Armed => "armed",
            Self::Spinning => "spinning",
        }
    }
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
    info!("spin_open_loop_start");
    info!("state_transition state={}", SpinState::Booting.as_str());

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

    info!("bridge_inputs_hiz_using_driver_pulldowns");
    info!("state_transition state={}", SpinState::Calibrating.as_str());
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

    let frame = measure_frame(
        0,
        SpinState::Disarmed,
        false,
        0,
        0,
        0,
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
    info!("state_transition state={}", SpinState::Disarmed.as_str());
    log_frame(frame, arm_ready);
    hold_disarmed(&mut status_led, ARM_DELAY_MS);

    if !arm_ready {
        warn!("arm_conditions_not_met staying_disarmed");
        loop {
            let frame = measure_frame(
                0,
                SpinState::Disarmed,
                false,
                0,
                0,
                0,
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
            log_frame(frame, false);
            hold_disarmed(&mut status_led, 250);
        }
    }

    info!("state_transition state={}", SpinState::Armed.as_str());
    let armed_frame = measure_frame(
        1,
        SpinState::Armed,
        true,
        0,
        0,
        0,
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
    hold_disarmed(&mut status_led, ARMED_HOLD_MS);

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

    info!("state_transition state={}", SpinState::Spinning.as_str());
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
    let mut seq = 2u32;
    let align_duty_counts = max_duty * ALIGN_DUTY_PERCENT / 100;
    info!(
        "alignment_start step={} duty_pct={} hold_us={}",
        ALIGN_STEP, ALIGN_DUTY_PERCENT, ALIGN_HOLD_US
    );
    apply_commutation_step(&mut pwm, ALIGN_STEP, align_duty_counts);
    let align_frame = measure_frame(
        seq,
        SpinState::Armed,
        true,
        ALIGN_DUTY_PERCENT,
        ALIGN_STEP,
        ALIGN_HOLD_US,
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
    disable_all_channels(&mut pwm);
    info!("alignment_complete next_step={}", (ALIGN_STEP + 1) % 6);

    info!(
        "spin_profile steps={} duty_pct_start={} duty_pct_end={} dwell_us_start={} dwell_us_end={} pwm_hz={}",
        SPIN_STEPS,
        SPIN_DUTY_START_PERCENT,
        SPIN_DUTY_END_PERCENT,
        SPIN_START_DWELL_US,
        SPIN_END_DWELL_US,
        PWM_FREQ_HZ
    );
    for spin_index in 0..SPIN_STEPS {
        let duty_pct = interpolate(
            SPIN_DUTY_START_PERCENT,
            SPIN_DUTY_END_PERCENT,
            spin_index,
            SPIN_STEPS,
        );
        let dwell_us = interpolate(
            SPIN_START_DWELL_US,
            SPIN_END_DWELL_US,
            spin_index,
            SPIN_STEPS,
        );
        let duty_counts = max_duty * duty_pct / 100;
        let comm_step = (spin_index + ALIGN_STEP + 1) % 6;
        let verbose_step = spin_index % TELEMETRY_EVERY_STEPS == 0 || spin_index + 1 == SPIN_STEPS;

        if verbose_step {
            info!(
                "comm_step_apply idx={} step={} duty_pct={} duty_counts={} dwell_us={}",
                spin_index, comm_step, duty_pct, duty_counts, dwell_us
            );
        }
        apply_commutation_step(&mut pwm, comm_step, duty_counts);
        if verbose_step {
            info!("comm_step_applied idx={} step={}", spin_index, comm_step);
        }
        if spin_index % TELEMETRY_EVERY_STEPS == 0 {
            let frame = measure_frame(
                seq,
                SpinState::Spinning,
                true,
                duty_pct,
                comm_step,
                dwell_us,
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
        delay_us(dwell_us);
        if verbose_step {
            info!("comm_step_complete idx={} step={}", spin_index, comm_step);
        }
    }

    pwm.set_master_output_enable(false);
    disable_all_channels(&mut pwm);
    info!("state_transition state={}", SpinState::Disarmed.as_str());

    for _ in 0..POST_SPIN_LOG_FRAMES {
        let frame = measure_frame(
            seq,
            SpinState::Disarmed,
            false,
            0,
            0,
            0,
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
        log_frame(frame, false);
        seq = seq.wrapping_add(1);
        hold_disarmed(&mut status_led, 250);
    }

    info!("spin_test_complete idling_disarmed");
    loop {
        hold_disarmed(&mut status_led, 500);
    }
}

fn apply_commutation_step(
    pwm: &mut ComplementaryPwm<'_, peripherals::TIM1>,
    step: usize,
    duty_counts: u32,
) {
    disable_all_channels(pwm);
    match step {
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
        "telemetry={{seq:{},state:{},arm_ready:{},duty_pct:{},comm_step:{},dwell_us:{},vdda_mv:{},bus_mv:{},bemf_mv:[{},{},{}],current_ma:[{},{},{}],current_out_mv:[{},{},{}],ntc_mv:{},ntc_ohms:{},mcu_temp_mc:{},bemf_gpio:{},hall:[{},{},{}]}}",
        frame.seq,
        frame.state.as_str(),
        arm_ready,
        frame.duty_pct,
        frame.comm_step,
        frame.dwell_us,
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

#[allow(clippy::too_many_arguments)]
fn measure_frame(
    seq: u32,
    state: SpinState,
    _arm_ready: bool,
    duty_pct: u32,
    comm_step: usize,
    dwell_us: u32,
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
        duty_pct,
        comm_step,
        dwell_us,
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
