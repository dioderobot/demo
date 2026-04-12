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
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::simple_pwm::PwmPin;
use embassy_stm32::timer::Channel;
use embassy_stm32::{Peri, peripherals};
use panic_probe as _;

static DEFMT_TICKS: AtomicU32 = AtomicU32::new(0);

const ADC_FULL_SCALE: u32 = 4095;
const ARM_DELAY_MS: u32 = 2_000;
const ARMED_HOLD_MS: u32 = 1_000;
const ARM_IDLE_CURRENT_LIMIT_MA: i32 = 1_500;
const ARM_VBUS_MAX_MV: u32 = 12_000;
const ARM_VBUS_MIN_MV: u32 = 7_000;
const ALIGN_AMPLITUDE_PERCENT: u32 = 6;
const ALIGN_ANGLE_IDX: u8 = 0;
const ALIGN_HOLD_US: u32 = 180_000;
const BOOTSTRAP_PRECHARGE_US: u32 = 8_000;
const BUS_ABORT_MV: u32 = 10_800;
const BUS_BACKOFF_MV: u32 = 11_400;
const BUS_FEEDBACK_RECOVERY_UPDATES: usize = 256;
const BUS_FEEDBACK_SAMPLE_UPDATES: usize = 16;
const CONTROL_UPDATE_HZ: u32 = 4_000;
const CONTROL_UPDATE_US: u32 = 1_000_000 / CONTROL_UPDATE_HZ;
const CPU_HZ: u32 = 16_000_000;
const CURRENT_OUTPUT_UV_PER_AMP: i32 = 27_429;
const CURRENT_ZERO_CAL_SAMPLES: usize = 64;
const CURRENT_ZERO_UV_NOMINAL: i32 = 2_057_143;
const CURRENT_ZERO_UV_TOLERANCE: i32 = 250_000;
const DEADTIME_TICKS: u16 = 8;
const ELECTRICAL_FREQ_END_HZ_X100: u32 = 17_000;
const ELECTRICAL_FREQ_START_HZ_X100: u32 = 400;
const HOLD_UPDATES: usize = 28_000;
const MIN_RUNNING_AMPLITUDE_PERCENT: u32 = 4;
const OBSERVER_ENABLE_HZ_X100: u32 = 8_000;
const OBSERVER_FEEDBACK_SAMPLE_UPDATES: usize = 8;
const OBSERVER_I_DIV: i32 = 24;
const OBSERVER_MIN_RESIDUAL_MV: i32 = 250;
const OBSERVER_P_DIV: i32 = 3;
const OBSERVER_TRIM_DECAY_DEN: i32 = 16;
const OBSERVER_TRIM_DECAY_NUM: i32 = 15;
const OBSERVER_TRIM_MAX_Q8: i32 = 768;
const NTC_PULLDOWN_OHMS: u32 = 4_700;
const POST_SPIN_LOG_FRAMES: usize = 8;
const PWM_FREQ_HZ: u32 = 20_000;
const RAMP_AMPLITUDE_END_PERCENT: u32 = 10;
const RAMP_AMPLITUDE_START_PERCENT: u32 = 5;
const RAMP_UPDATES: usize = 12_000;
const SAMPLE_TIME: SampleTime = SampleTime::CYCLES640_5;
const TELEMETRY_EVERY_UPDATES: usize = 250;
const TOTAL_UPDATES: usize = RAMP_UPDATES + HOLD_UPDATES;
const SQRT3_INV_Q15: i32 = 18_919;
const TS_CAL1_TEMP_MC: i32 = 30_000;
const TS_CAL2_TEMP_MC: i32 = 130_000;
const TS_CAL1_ADDR: *const u16 = 0x1FFF_75A8 as *const u16;
const TS_CAL2_ADDR: *const u16 = 0x1FFF_75CA as *const u16;

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

#[derive(Clone, Copy, Default)]
struct ObserverState {
    locked: bool,
    phase_error: i32,
    step_trim_q8: i32,
    residual_alpha_mv: i32,
    residual_beta_mv: i32,
    residual_mag_mv: u32,
    id_ma: i32,
    iq_ma: i32,
}

#[derive(Clone, Copy)]
struct TelemetryFrame {
    seq: u32,
    state: ControlState,
    amplitude_pct: u32,
    electrical_hz_x100: u32,
    angle_idx: u8,
    duty_a_pct: u32,
    duty_b_pct: u32,
    duty_c_pct: u32,
    observer_locked: bool,
    observer_phase_error: i32,
    observer_trim_q8: i32,
    observer_residual_alpha_mv: i32,
    observer_residual_beta_mv: i32,
    observer_residual_mag_mv: u32,
    id_ma: i32,
    iq_ma: i32,
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
enum ControlState {
    Booting,
    Calibrating,
    Disarmed,
    Armed,
    Aligning,
    Spinning,
}

impl ControlState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Booting => "booting",
            Self::Calibrating => "calibrating",
            Self::Disarmed => "disarmed",
            Self::Armed => "armed",
            Self::Aligning => "aligning",
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
    info!("spin_sine_open_loop_start");
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
    let mut observer_state = ObserverState::default();

    info!("bridge_inputs_hiz_using_driver_pulldowns");
    info!("state_transition state={}", ControlState::Calibrating.as_str());
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
        ControlState::Disarmed,
        0,
        0,
        0,
        [0, 0, 0],
        observer_state,
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
    info!("state_transition state={}", ControlState::Disarmed.as_str());
    log_frame(frame, arm_ready);
    hold_disarmed(&mut status_led, ARM_DELAY_MS);

    if !arm_ready {
        warn!("arm_conditions_not_met staying_disarmed");
        loop {
            let frame = measure_frame(
                0,
                ControlState::Disarmed,
                0,
                0,
                0,
                [0, 0, 0],
                observer_state,
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

    info!("state_transition state={}", ControlState::Armed.as_str());
    let armed_frame = measure_frame(
        1,
        ControlState::Armed,
        0,
        0,
        0,
        [0, 0, 0],
        observer_state,
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

    info!("state_transition state={}", ControlState::Aligning.as_str());
    pwm.set_master_output_enable(true);
    pwm.set_duty(Channel::Ch1, 0);
    pwm.set_duty(Channel::Ch2, 0);
    pwm.set_duty(Channel::Ch3, 0);
    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);
    info!(
        "bootstrap_precharge_start duration_us={} mode=all_low_sides_on",
        BOOTSTRAP_PRECHARGE_US
    );
    delay_us(BOOTSTRAP_PRECHARGE_US);
    info!("bootstrap_precharge_complete");

    let max_duty = u32::from(pwm.get_max_duty());
    let center_counts = max_duty / 2;
    let mut seq = 2u32;
    let mut adaptive_amplitude_pct = RAMP_AMPLITUDE_START_PERCENT;
    let mut last_bus_mv = armed_frame.bus_mv;

    let align_amplitude_counts = max_duty * ALIGN_AMPLITUDE_PERCENT / 100;
    let align_duties = set_sine_duties(
        &mut pwm,
        ALIGN_ANGLE_IDX,
        align_amplitude_counts,
        center_counts,
        max_duty,
    );
    info!(
        "alignment_start angle_idx={} amplitude_pct={} hold_us={}",
        ALIGN_ANGLE_IDX,
        ALIGN_AMPLITUDE_PERCENT,
        ALIGN_HOLD_US
    );
    let align_frame = measure_frame(
        seq,
        ControlState::Aligning,
        ALIGN_AMPLITUDE_PERCENT,
        0,
        ALIGN_ANGLE_IDX,
        duty_triplet_to_pct(align_duties, max_duty),
        observer_state,
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
    info!("alignment_complete");

    let mut phase_acc_q8 = u32::from(ALIGN_ANGLE_IDX) << 8;
    let mut last_angle_idx = ALIGN_ANGLE_IDX;
    let mut last_duties = align_duties;
    info!("state_transition state={}", ControlState::Spinning.as_str());
    info!(
        "sine_profile total_updates={} ramp_updates={} hold_updates={} amp_pct_start={} amp_pct_end={} elec_hz_x100_start={} elec_hz_x100_end={} update_hz={} pwm_hz={} bus_backoff_mv={} bus_abort_mv={}",
        TOTAL_UPDATES,
        RAMP_UPDATES,
        HOLD_UPDATES,
        RAMP_AMPLITUDE_START_PERCENT,
        RAMP_AMPLITUDE_END_PERCENT,
        ELECTRICAL_FREQ_START_HZ_X100,
        ELECTRICAL_FREQ_END_HZ_X100,
        CONTROL_UPDATE_HZ,
        PWM_FREQ_HZ,
        BUS_BACKOFF_MV,
        BUS_ABORT_MV
    );

    for update_index in 0..TOTAL_UPDATES {
        let ramp_index = update_index.min(RAMP_UPDATES.saturating_sub(1));
        let target_amplitude_pct = interpolate(
            RAMP_AMPLITUDE_START_PERCENT,
            RAMP_AMPLITUDE_END_PERCENT,
            ramp_index,
            RAMP_UPDATES,
        );
        let electrical_hz_x100 = interpolate(
            ELECTRICAL_FREQ_START_HZ_X100,
            ELECTRICAL_FREQ_END_HZ_X100,
            ramp_index,
            RAMP_UPDATES,
        );

        if update_index % BUS_FEEDBACK_SAMPLE_UPDATES == 0 {
            let bus_mv = sample_bus_mv(&mut adc1, &mut vbus, &mut vrefint, calibration);
            last_bus_mv = bus_mv;

            if bus_mv < BUS_ABORT_MV {
                warn!(
                    "bus_abort idx={} bus_mv={} amp_pct={} elec_hz_x100={}",
                    update_index,
                    bus_mv,
                    adaptive_amplitude_pct,
                    electrical_hz_x100
                );
                break;
            }

            if bus_mv < BUS_BACKOFF_MV {
                let next_amplitude_pct = adaptive_amplitude_pct
                    .saturating_sub(1)
                    .max(MIN_RUNNING_AMPLITUDE_PERCENT);
                if next_amplitude_pct != adaptive_amplitude_pct {
                    adaptive_amplitude_pct = next_amplitude_pct;
                    warn!(
                        "bus_backoff idx={} bus_mv={} amp_pct={} target_amp_pct={} elec_hz_x100={}",
                        update_index,
                        bus_mv,
                        adaptive_amplitude_pct,
                        target_amplitude_pct,
                        electrical_hz_x100
                    );
                }
            } else if adaptive_amplitude_pct < target_amplitude_pct {
                adaptive_amplitude_pct += 1;
            }
        } else if adaptive_amplitude_pct < target_amplitude_pct
            && update_index % BUS_FEEDBACK_RECOVERY_UPDATES == 0
        {
            adaptive_amplitude_pct += 1;
        }

        if update_index % OBSERVER_FEEDBACK_SAMPLE_UPDATES == 0 {
            observer_state = update_observer(
                observer_state,
                electrical_hz_x100,
                last_bus_mv,
                last_angle_idx,
                last_duties,
                max_duty,
                &mut adc1,
                &mut adc2,
                &mut bemf_a,
                &mut bemf_b,
                &mut bemf_c,
                &mut current_a,
                &mut current_b,
                &mut current_c,
                &mut vrefint,
                current_calibration,
                calibration,
            );
        }

        adaptive_amplitude_pct = adaptive_amplitude_pct.min(target_amplitude_pct);
        let amplitude_counts = max_duty * adaptive_amplitude_pct / 100;
        let step_q8 = electrical_hz_x100_to_phase_step_q8(electrical_hz_x100);
        let phase_correction_q8 = observer_state.step_trim_q8
            + if observer_state.locked {
                observer_state.phase_error / OBSERVER_P_DIV
            } else {
                0
            };
        let phase_step_q8 = clamp_i32(i32::try_from(step_q8).unwrap_or(i32::MAX) + phase_correction_q8, 64, 8_192) as u32;
        phase_acc_q8 = phase_acc_q8.wrapping_add(phase_step_q8);
        let angle_idx = (phase_acc_q8 >> 8) as u8;
        let duties = set_sine_duties(
            &mut pwm,
            angle_idx,
            amplitude_counts,
            center_counts,
            max_duty,
        );
        last_angle_idx = angle_idx;
        last_duties = duties;
        let duty_pct = duty_triplet_to_pct(duties, max_duty);
        let verbose_update = update_index % TELEMETRY_EVERY_UPDATES == 0 || update_index + 1 == TOTAL_UPDATES;

        if verbose_update {
            info!(
                "sine_update idx={} angle_idx={} amp_pct={} target_amp_pct={} elec_hz_x100={} step_q8={} phase_trim_q8={} phase_err={} obs_locked={} bus_mv_sample={} duty_pct=[{},{},{}]",
                update_index,
                angle_idx,
                adaptive_amplitude_pct,
                target_amplitude_pct,
                electrical_hz_x100,
                phase_step_q8,
                observer_state.step_trim_q8,
                observer_state.phase_error,
                observer_state.locked,
                last_bus_mv,
                duty_pct[0],
                duty_pct[1],
                duty_pct[2]
            );
        }

        if update_index % TELEMETRY_EVERY_UPDATES == 0 {
            let frame = measure_frame(
                seq,
                ControlState::Spinning,
                adaptive_amplitude_pct,
                electrical_hz_x100,
                angle_idx,
                duty_pct,
                observer_state,
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
        delay_us(CONTROL_UPDATE_US);
    }

    pwm.set_master_output_enable(false);
    disable_all_channels(&mut pwm);
    info!("state_transition state={}", ControlState::Disarmed.as_str());

    for _ in 0..POST_SPIN_LOG_FRAMES {
        let frame = measure_frame(
            seq,
            ControlState::Disarmed,
            0,
            0,
            0,
            [0, 0, 0],
            observer_state,
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

    info!("sine_test_complete idling_disarmed");
    loop {
        hold_disarmed(&mut status_led, 500);
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
        let vdda_mv = estimate_vdda_mv(adc1.blocking_read(vrefint, SAMPLE_TIME), calibration.vrefint);
        sum_a_uv += i64::from(adc_raw_to_uv(adc1.blocking_read(current_a, SAMPLE_TIME), vdda_mv));
        sum_b_uv += i64::from(adc_raw_to_uv(adc2.blocking_read(current_b, SAMPLE_TIME), vdda_mv));
        sum_c_uv += i64::from(adc_raw_to_uv(adc1.blocking_read(current_c, SAMPLE_TIME), vdda_mv));
    }

    CurrentCalibration {
        zero_a_uv: (sum_a_uv / CURRENT_ZERO_CAL_SAMPLES as i64) as i32,
        zero_b_uv: (sum_b_uv / CURRENT_ZERO_CAL_SAMPLES as i64) as i32,
        zero_c_uv: (sum_c_uv / CURRENT_ZERO_CAL_SAMPLES as i64) as i32,
    }
}

fn delay_us(us: u32) {
    let cycles = ((u64::from(CPU_HZ) * u64::from(us)) / 1_000_000) as u32;
    asm::delay(cycles.max(1));
}

fn disable_all_channels(pwm: &mut ComplementaryPwm<'_, peripherals::TIM1>) {
    pwm.set_duty(Channel::Ch1, 0);
    pwm.set_duty(Channel::Ch2, 0);
    pwm.set_duty(Channel::Ch3, 0);
    pwm.disable(Channel::Ch1);
    pwm.disable(Channel::Ch2);
    pwm.disable(Channel::Ch3);
}

fn duty_counts_to_pct(duty_counts: u16, max_duty: u32) -> u32 {
    if max_duty == 0 {
        return 0;
    }

    u32::from(duty_counts) * 100 / max_duty
}

fn duty_triplet_to_pct(duties: [u16; 3], max_duty: u32) -> [u32; 3] {
    [
        duty_counts_to_pct(duties[0], max_duty),
        duty_counts_to_pct(duties[1], max_duty),
        duty_counts_to_pct(duties[2], max_duty),
    ]
}

fn duty_triplet_to_phase_mv(duties: [u16; 3], max_duty: u32, bus_mv: u32) -> [i32; 3] {
    if max_duty == 0 {
        return [0, 0, 0];
    }

    let max_duty_i64 = i64::from(max_duty);
    let scale = 2 * max_duty_i64;
    let phase_mv = [
        ((i64::from(duties[0]) * 2 - max_duty_i64) * i64::from(bus_mv) / scale) as i32,
        ((i64::from(duties[1]) * 2 - max_duty_i64) * i64::from(bus_mv) / scale) as i32,
        ((i64::from(duties[2]) * 2 - max_duty_i64) * i64::from(bus_mv) / scale) as i32,
    ];

    center_i32_triplet(phase_mv)
}

fn electrical_hz_x100_to_phase_step_q8(electrical_hz_x100: u32) -> u32 {
    (electrical_hz_x100 * 65_536 + (CONTROL_UPDATE_HZ * 100) / 2) / (CONTROL_UPDATE_HZ * 100)
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

fn center_i32_triplet(values: [i32; 3]) -> [i32; 3] {
    let average = (values[0] + values[1] + values[2]) / 3;
    [values[0] - average, values[1] - average, values[2] - average]
}

fn clarke_beta(b: i32, c: i32) -> i32 {
    ((i64::from(b - c) * i64::from(SQRT3_INV_Q15)) / 32_768) as i32
}

fn approx_vector_mag(alpha: i32, beta: i32) -> u32 {
    let alpha_abs = alpha.unsigned_abs();
    let beta_abs = beta.unsigned_abs();
    alpha_abs.max(beta_abs) + alpha_abs.min(beta_abs) / 2
}

fn phase_sample_centered(angle_idx: u8) -> i32 {
    i32::from(SINE_TABLE[angle_idx as usize]) - 127
}

fn phase_vector_from_angle(angle_idx: u8) -> (i32, i32) {
    let phase_b = phase_sample_centered(angle_idx.wrapping_add(85));
    let phase_c = phase_sample_centered(angle_idx.wrapping_add(170));
    (phase_sample_centered(angle_idx), clarke_beta(phase_b, phase_c))
}

#[allow(clippy::too_many_arguments)]
fn update_observer(
    previous: ObserverState,
    electrical_hz_x100: u32,
    bus_mv: u32,
    angle_idx: u8,
    duties: [u16; 3],
    max_duty: u32,
    adc1: &mut Adc<'_, embassy_stm32::peripherals::ADC1>,
    adc2: &mut Adc<'_, embassy_stm32::peripherals::ADC2>,
    bemf_a: &mut Peri<'_, peripherals::PA4>,
    bemf_b: &mut Peri<'_, peripherals::PB12>,
    bemf_c: &mut Peri<'_, peripherals::PB11>,
    current_a: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP1>,
    current_b: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP2>,
    current_c: &mut embassy_stm32::opamp::OpAmpOutput<'_, peripherals::OPAMP3>,
    vrefint: &mut embassy_stm32::adc::VrefInt,
    current_calibration: CurrentCalibration,
    calibration: FactoryCalibration,
) -> ObserverState {
    let vdda_mv = estimate_vdda_mv(adc1.blocking_read(vrefint, SAMPLE_TIME), calibration.vrefint);
    let phase_mv = center_i32_triplet([
        adc_raw_to_mv(adc2.blocking_read(bemf_a, SAMPLE_TIME), vdda_mv) as i32,
        adc_raw_to_mv(adc1.blocking_read(bemf_b, SAMPLE_TIME), vdda_mv) as i32,
        adc_raw_to_mv(adc2.blocking_read(bemf_c, SAMPLE_TIME), vdda_mv) as i32,
    ]);
    let current_ma = center_i32_triplet([
        estimate_phase_current_ma(
            adc1.blocking_read(current_a, SAMPLE_TIME),
            vdda_mv,
            current_calibration.zero_a_uv,
        ),
        estimate_phase_current_ma(
            adc2.blocking_read(current_b, SAMPLE_TIME),
            vdda_mv,
            current_calibration.zero_b_uv,
        ),
        estimate_phase_current_ma(
            adc1.blocking_read(current_c, SAMPLE_TIME),
            vdda_mv,
            current_calibration.zero_c_uv,
        ),
    ]);

    let commanded_phase_mv = duty_triplet_to_phase_mv(duties, max_duty, bus_mv);
    let residual_phase_mv = [
        phase_mv[0] - commanded_phase_mv[0],
        phase_mv[1] - commanded_phase_mv[1],
        phase_mv[2] - commanded_phase_mv[2],
    ];
    let residual_alpha_mv = residual_phase_mv[0];
    let residual_beta_mv = clarke_beta(residual_phase_mv[1], residual_phase_mv[2]);
    let residual_mag_mv = approx_vector_mag(residual_alpha_mv, residual_beta_mv);

    let current_alpha_ma = current_ma[0];
    let current_beta_ma = clarke_beta(current_ma[1], current_ma[2]);
    let (ref_alpha, ref_beta) = phase_vector_from_angle(angle_idx);
    let ref_mag = i32::try_from(approx_vector_mag(ref_alpha, ref_beta))
        .unwrap_or(i32::MAX)
        .max(1);
    let id_ma = ((i64::from(current_alpha_ma) * i64::from(ref_alpha)
        + i64::from(current_beta_ma) * i64::from(ref_beta))
        / i64::from(ref_mag)) as i32;
    let iq_ma = ((i64::from(current_beta_ma) * i64::from(ref_alpha)
        - i64::from(current_alpha_ma) * i64::from(ref_beta))
        / i64::from(ref_mag)) as i32;

    let mut step_trim_q8 = previous.step_trim_q8 * OBSERVER_TRIM_DECAY_NUM / OBSERVER_TRIM_DECAY_DEN;
    let residual_mag_i32 = i32::try_from(residual_mag_mv).unwrap_or(i32::MAX);
    let observer_enabled =
        electrical_hz_x100 >= OBSERVER_ENABLE_HZ_X100 && residual_mag_i32 >= OBSERVER_MIN_RESIDUAL_MV;
    let mut locked = false;
    let mut phase_error = 0;

    if observer_enabled {
        let cross = i64::from(ref_alpha) * i64::from(residual_beta_mv)
            - i64::from(ref_beta) * i64::from(residual_alpha_mv);
        phase_error = clamp_i32((cross / i64::from(residual_mag_i32.max(1))) as i32, -127, 127);
        step_trim_q8 += phase_error / OBSERVER_I_DIV;
        step_trim_q8 = clamp_i32(step_trim_q8, -OBSERVER_TRIM_MAX_Q8, OBSERVER_TRIM_MAX_Q8);
        locked = true;
    }

    ObserverState {
        locked,
        phase_error,
        step_trim_q8,
        residual_alpha_mv,
        residual_beta_mv,
        residual_mag_mv,
        id_ma,
        iq_ma,
    }
}

fn clamp_i32(value: i32, min_value: i32, max_value: i32) -> i32 {
    value.clamp(min_value, max_value)
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
        "telemetry={{seq:{},state:{},arm_ready:{},amp_pct:{},elec_hz_x100:{},angle_idx:{},duty_pct:[{},{},{}],obs_locked:{},obs_err:{},obs_trim_q8:{},obs_res_mv:[{},{},{}],dq_ma:[{},{}],vdda_mv:{},bus_mv:{},bemf_mv:[{},{},{}],current_ma:[{},{},{}],current_out_mv:[{},{},{}],ntc_mv:{},ntc_ohms:{},mcu_temp_mc:{},bemf_gpio:{},hall:[{},{},{}]}}",
        frame.seq,
        frame.state.as_str(),
        arm_ready,
        frame.amplitude_pct,
        frame.electrical_hz_x100,
        frame.angle_idx,
        frame.duty_a_pct,
        frame.duty_b_pct,
        frame.duty_c_pct,
        frame.observer_locked,
        frame.observer_phase_error,
        frame.observer_trim_q8,
        frame.observer_residual_alpha_mv,
        frame.observer_residual_beta_mv,
        frame.observer_residual_mag_mv,
        frame.id_ma,
        frame.iq_ma,
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
    state: ControlState,
    amplitude_pct: u32,
    electrical_hz_x100: u32,
    angle_idx: u8,
    duty_pct: [u32; 3],
    observer: ObserverState,
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
    let vdda_mv = estimate_vdda_mv(adc1.blocking_read(vrefint, SAMPLE_TIME), calibration.vrefint);
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
        amplitude_pct,
        electrical_hz_x100,
        angle_idx,
        duty_a_pct: duty_pct[0],
        duty_b_pct: duty_pct[1],
        duty_c_pct: duty_pct[2],
        observer_locked: observer.locked,
        observer_phase_error: observer.phase_error,
        observer_trim_q8: observer.step_trim_q8,
        observer_residual_alpha_mv: observer.residual_alpha_mv,
        observer_residual_beta_mv: observer.residual_beta_mv,
        observer_residual_mag_mv: observer.residual_mag_mv,
        id_ma: observer.id_ma,
        iq_ma: observer.iq_ma,
        vdda_mv,
        bus_mv: estimate_bus_mv(bus_raw, vdda_mv),
        bemf_a_mv: adc_raw_to_mv(bemf_a_raw, vdda_mv),
        bemf_b_mv: adc_raw_to_mv(bemf_b_raw, vdda_mv),
        bemf_c_mv: adc_raw_to_mv(bemf_c_raw, vdda_mv),
        current_a_ma: estimate_phase_current_ma(current_a_raw, vdda_mv, current_calibration.zero_a_uv),
        current_b_ma: estimate_phase_current_ma(current_b_raw, vdda_mv, current_calibration.zero_b_uv),
        current_c_ma: estimate_phase_current_ma(current_c_raw, vdda_mv, current_calibration.zero_c_uv),
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

fn sample_bus_mv(
    adc1: &mut Adc<'_, embassy_stm32::peripherals::ADC1>,
    vbus: &mut Peri<'_, peripherals::PA0>,
    vrefint: &mut embassy_stm32::adc::VrefInt,
    calibration: FactoryCalibration,
) -> u32 {
    let vdda_mv = estimate_vdda_mv(adc1.blocking_read(vrefint, SAMPLE_TIME), calibration.vrefint);
    let bus_raw = adc1.blocking_read(vbus, SAMPLE_TIME);
    estimate_bus_mv(bus_raw, vdda_mv)
}

fn adc_raw_to_mv(raw: u16, vdda_mv: u32) -> u32 {
    u32::from(raw) * vdda_mv / ADC_FULL_SCALE
}

fn adc_raw_to_uv(raw: u16, vdda_mv: u32) -> i32 {
    (u64::from(raw) * u64::from(vdda_mv) * 1_000 / u64::from(ADC_FULL_SCALE)) as i32
}

fn compute_phase_duty(angle_idx: u8, amplitude_counts: u32, center_counts: u32, max_duty: u32) -> u16 {
    let sine_sample = i32::from(SINE_TABLE[angle_idx as usize]) - 127;
    let center_counts = i32::try_from(center_counts).unwrap_or_default();
    let amplitude_counts = i32::try_from(amplitude_counts).unwrap_or_default();
    let max_duty = i32::try_from(max_duty).unwrap_or(i32::MAX);

    let duty = center_counts + sine_sample * amplitude_counts / 127;
    duty.clamp(0, max_duty) as u16
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
        + (TS_CAL2_TEMP_MC - TS_CAL1_TEMP_MC) * (compensated - i32::from(calibration.ts_cal1)) / cal_span
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

fn set_sine_duties(
    pwm: &mut ComplementaryPwm<'_, peripherals::TIM1>,
    angle_idx: u8,
    amplitude_counts: u32,
    center_counts: u32,
    max_duty: u32,
) -> [u16; 3] {
    let duty_a = compute_phase_duty(angle_idx, amplitude_counts, center_counts, max_duty);
    let duty_b = compute_phase_duty(angle_idx.wrapping_add(85), amplitude_counts, center_counts, max_duty);
    let duty_c = compute_phase_duty(angle_idx.wrapping_add(170), amplitude_counts, center_counts, max_duty);

    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);
    pwm.set_duty(Channel::Ch1, u32::from(duty_a));
    pwm.set_duty(Channel::Ch2, u32::from(duty_b));
    pwm.set_duty(Channel::Ch3, u32::from(duty_c));

    [duty_a, duty_b, duty_c]
}
