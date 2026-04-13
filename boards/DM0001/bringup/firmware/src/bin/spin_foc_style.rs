#![no_std]
#![no_main]

use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::asm;
use defmt::{info, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcConfig, SampleTime, VREF_CALIB_MV};
use embassy_stm32::bind_interrupts;
use embassy_stm32::gpio::{Input, Level, Output, OutputType, Pull, Speed};
use embassy_stm32::opamp::{OpAmp, OpAmpGain, OpAmpSpeed};
use embassy_stm32::rcc::mux::Adcsel;
use embassy_stm32::rcc::{AHBPrescaler, APBPrescaler, Pll, PllMul, PllPreDiv, PllRDiv, PllSource, Sysclk};
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::Channel;
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::pwm_input::PwmInput;
use embassy_stm32::timer::simple_pwm::PwmPin;
use embassy_stm32::{Peri, peripherals, timer};
use panic_probe as _;

static DEFMT_TICKS: AtomicU32 = AtomicU32::new(0);

bind_interrupts!(struct Irqs {
    TIM2 => timer::CaptureCompareInterruptHandler<peripherals::TIM2>;
});

const ADC_FULL_SCALE: u32 = 4095;
const AUTO_ARM_AT_BOOT: bool = false;
const ARM_DELAY_MS: u32 = 2_000;
const ARM_IDLE_CURRENT_LIMIT_MA: i32 = 1_500;
const ARM_LOG_PERIOD_MS: u32 = 250;
const ARM_VBUS_MAX_MV: u32 = 16_500;
const ARM_VBUS_MIN_MV: u32 = 7_000;
const BEMF_FLUX_SHIFT_IDX: u8 = 64;
const BOOTSTRAP_PRECHARGE_US: u32 = 8_000;
const BUS_ABORT_DROOP_MV: u32 = 2_500;
const BUS_BACKOFF_DROOP_MV: u32 = 1_250;
const CONTROL_UPDATE_HZ: u32 = 40_000;
const CONTROL_UPDATE_US: u32 = 1_000_000 / CONTROL_UPDATE_HZ;
const CPU_HZ: u32 = 170_000_000;
const CURRENT_OUTPUT_UV_PER_AMP: i32 = 27_429;
const CURRENT_ZERO_CAL_SAMPLES: usize = 64;
const CURRENT_ZERO_UV_NOMINAL: i32 = 2_057_143;
const CURRENT_ZERO_UV_TOLERANCE: i32 = 250_000;
const DEADTIME_TICKS: u16 = 8;
const ESC_ARMING_HOLD_MS: u32 = 500;
const ESC_CAPTURE_HZ: u32 = 1_000_000;
const ESC_MAX_SPEED_RPM: u32 = 12_000;
const ESC_MIN_SPEED_RPM: u32 = 1_000;
const ESC_POLE_PAIRS: u32 = 7;
const ESC_PWM_MAX_US: u32 = 1_860;
const ESC_PWM_MIN_US: u32 = 1_060;
const ESC_PWM_ARMING_US: u32 = 800;
const ESC_PWM_PERIOD_MAX_US: u32 = 2_500;
const ESC_PWM_PERIOD_MIN_US: u32 = 1_500;
const ESC_STOP_TICKS: u16 = 500;
const ELECTRICAL_FREQ_START_HZ_X100: u32 = 2_000;
const FOC_ALIGN_HOLD_US: u32 = 250_000;
const FOC_ALIGN_ID_REF_MA: i32 = 1_400;
const FOC_ALIGN_VECTOR_LIMIT_PCT: u32 = 8;
const FOC_CLOSED_LOOP_HOLD_UPDATES: usize = CONTROL_UPDATE_HZ as usize * 8;
const FOC_CLOSED_LOOP_TARGET_HZ_X100: u32 = 32_000;
const FOC_CLOSED_LOOP_VECTOR_LIMIT_PCT: u32 = 18;
const FOC_MIN_IQ_REF_MA: i32 = 700;
const FOC_MAX_IQ_REF_MA: i32 = 3_200;
const FOC_REVUP_DURATION_UPDATES: usize = CONTROL_UPDATE_HZ as usize * 2;
const FOC_REVUP_END_HZ_X100: u32 = 20_000;
const FOC_REVUP_IQ_END_MA: i32 = 2_200;
const FOC_REVUP_IQ_START_MA: i32 = 1_200;
const FOC_REVUP_VECTOR_LIMIT_PCT: u32 = 14;
const NTC_PULLDOWN_OHMS: u32 = 4_700;
const OBSERVER_LOCK_BEMF_MIN_MV: u32 = 180;
const OBSERVER_LOCK_CYCLES: u16 = 96;
const OBSERVER_LOCK_ERR_IDX: i32 = 10;
const OBSERVER_PLL_INTEGRAL_DIV: i32 = 64;
const OBSERVER_PLL_KI_NUM: i32 = 1;
const OBSERVER_PLL_KP_NUM: i32 = 4;
const OBSERVER_UNLOCK_CYCLES: u16 = 24;
const PWM_FREQ_HZ: u32 = 40_000;
const SAMPLE_TIME: SampleTime = SampleTime::CYCLES640_5;
const SPEED_LOOP_DIVIDER: usize = (CONTROL_UPDATE_HZ / 1_000) as usize;
const SPEED_LOOP_MIN_TARGET_HZ_X100: u32 = 4_000;
const SQRT3_HALF_Q15: i32 = 28_378;
const SQRT3_INV_Q15: i32 = 18_919;
const TELEMETRY_EVERY_UPDATES: usize = 400;
const TS_CAL1_TEMP_MC: i32 = 30_000;
const TS_CAL2_TEMP_MC: i32 = 130_000;
const TS_CAL1_ADDR: *const u16 = 0x1FFF_75A8 as *const u16;
const TS_CAL2_ADDR: *const u16 = 0x1FFF_75CA as *const u16;

const SINE_TABLE: [u8; 256] = [
    127, 130, 133, 136, 139, 143, 146, 149, 152, 155, 158, 161, 164, 167, 170, 173, 176, 178,
    181, 184, 187, 190, 192, 195, 198, 200, 203, 205, 208, 210, 212, 215, 217, 219, 221, 223,
    225, 227, 229, 231, 233, 234, 236, 238, 239, 240, 242, 243, 244, 245, 247, 248, 249, 249,
    250, 251, 252, 252, 253, 253, 253, 254, 254, 254, 254, 254, 254, 254, 253, 253, 253, 252,
    252, 251, 250, 249, 249, 248, 247, 245, 244, 243, 242, 240, 239, 238, 236, 234, 233, 231,
    229, 227, 225, 223, 221, 219, 217, 215, 212, 210, 208, 205, 203, 200, 198, 195, 192, 190,
    187, 184, 181, 178, 176, 173, 170, 167, 164, 161, 158, 155, 152, 149, 146, 143, 139, 136,
    133, 130, 127, 124, 121, 118, 115, 111, 108, 105, 102, 99, 96, 93, 90, 87, 84, 81, 78, 76, 73,
    70, 67, 64, 62, 59, 56, 54, 51, 49, 46, 44, 42, 39, 37, 35, 33, 31, 29, 27, 25, 23, 21, 20, 18,
    16, 15, 14, 12, 11, 10, 9, 7, 6, 5, 5, 4, 3, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2,
    2, 3, 4, 5, 5, 6, 7, 9, 10, 11, 12, 14, 15, 16, 18, 20, 21, 23, 25, 27, 29, 31, 33, 35, 37,
    39, 42, 44, 46, 49, 51, 54, 56, 59, 62, 64, 67, 70, 73, 76, 78, 81, 84, 87, 90, 93, 96, 99,
    102, 105, 108, 111, 115, 118, 121, 124,
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
struct PiController {
    kp_num: i32,
    kp_div: i32,
    ki_num: i32,
    ki_div: i32,
    integral: i32,
    out_min: i32,
    out_max: i32,
}

#[derive(Clone, Copy, Default)]
struct PllObserverState {
    theta_q8: u32,
    omega_q8: i32,
    integral: i32,
    angle_error_idx: i16,
    bemf_mag_mv: u32,
    lock_counter: u16,
    unlock_counter: u16,
    locked: bool,
    estimated_hz_x100: u32,
}

#[derive(Clone, Copy, Default)]
struct ControlStepResult {
    duty_pct: [u32; 3],
    id_ma: i32,
    iq_ma: i32,
    vd_counts: i32,
    vq_counts: i32,
    alpha_counts: i32,
    beta_counts: i32,
}

#[derive(Clone, Copy)]
struct MotorSample {
    vdda_mv: u32,
    bus_mv: u32,
    bemf_phase_mv: [i32; 3],
    bemf_alpha_mv: i32,
    bemf_beta_mv: i32,
    bemf_mag_mv: u32,
    phase_current_ma: [i32; 3],
    current_alpha_ma: i32,
    current_beta_ma: i32,
    current_output_mv: [u32; 3],
    ntc_mv: u32,
    ntc_ohms: i32,
    mcu_temp_mc: i32,
    bemf_gpio: bool,
    hall_a: bool,
    hall_b: bool,
    hall_c: bool,
}

#[derive(Clone, Copy, Default)]
struct EscCommand {
    valid: bool,
    width_us: u32,
    period_us: u32,
    arming_request: bool,
    throttle_active: bool,
    speed_target_hz_x100: u32,
}

#[derive(Clone, Copy)]
struct TelemetryFrame {
    seq: u32,
    state: ControlState,
    arm_ready: bool,
    esc_valid: bool,
    esc_width_us: u32,
    esc_period_us: u32,
    observer_locked: bool,
    theta_cmd_idx: u8,
    theta_obs_idx: u8,
    speed_target_hz_x100: u32,
    speed_est_hz_x100: u32,
    id_ref_ma: i32,
    iq_ref_ma: i32,
    id_ma: i32,
    iq_ma: i32,
    vd_counts: i32,
    vq_counts: i32,
    alpha_counts: i32,
    beta_counts: i32,
    vector_limit_pct: u32,
    vdda_mv: u32,
    bus_mv: u32,
    bemf_mv: [i32; 3],
    bemf_mag_mv: u32,
    current_ma: [i32; 3],
    current_out_mv: [u32; 3],
    duty_pct: [u32; 3],
    ntc_mv: u32,
    ntc_ohms: i32,
    mcu_temp_mc: i32,
    bemf_gpio: bool,
    hall_a: bool,
    hall_b: bool,
    hall_c: bool,
    fault: FaultReason,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlState {
    Booting,
    Calibrating,
    Disarmed,
    Armed,
    Aligning,
    RevUp,
    ClosedLoop,
    Faulted,
}

impl ControlState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Booting => "booting",
            Self::Calibrating => "calibrating",
            Self::Disarmed => "disarmed",
            Self::Armed => "armed",
            Self::Aligning => "aligning",
            Self::RevUp => "rev_up",
            Self::ClosedLoop => "closed_loop",
            Self::Faulted => "faulted",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum FaultReason {
    #[default]
    None,
    ArmConditions,
    ArmRequestMissing,
    BusAbort,
    ObserverUnlock,
    ObserverNeverLocked,
}

impl FaultReason {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ArmConditions => "arm_conditions",
            Self::ArmRequestMissing => "arm_request_missing",
            Self::BusAbort => "bus_abort",
            Self::ObserverUnlock => "observer_unlock",
            Self::ObserverNeverLocked => "observer_never_locked",
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

impl PiController {
    const fn new(
        kp_num: i32,
        kp_div: i32,
        ki_num: i32,
        ki_div: i32,
        out_min: i32,
        out_max: i32,
    ) -> Self {
        Self {
            kp_num,
            kp_div,
            ki_num,
            ki_div,
            integral: 0,
            out_min,
            out_max,
        }
    }

    fn reset(&mut self) {
        self.integral = 0;
    }

    fn update(&mut self, error: i32) -> i32 {
        let p_term = error * self.kp_num / self.kp_div.max(1);
        self.integral += error * self.ki_num / self.ki_div.max(1);
        self.integral = self.integral.clamp(self.out_min, self.out_max);
        (p_term + self.integral).clamp(self.out_min, self.out_max)
    }
}

#[embassy_executor::main(
    executor = "embassy_stm32::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(_spawner: Spawner) {
    info!("spin_foc_style_start");
    info!("state_transition state={}", ControlState::Booting.as_str());

    let mut config = embassy_stm32::Config::default();
    config.rcc.hsi = true;
    config.rcc.hsi48 = None;
    config.rcc.pll = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV4,
        mul: PllMul::MUL85,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2),
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.boost = true;
    config.rcc.mux.adc12sel = Adcsel::SYS;

    let p = embassy_stm32::init(config);

    let mut esc_input = PwmInput::new_ch1(p.TIM2, p.PA15, Irqs, Pull::Down, Hertz::hz(ESC_CAPTURE_HZ));
    esc_input.enable();
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

    let sample = sample_motor(
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

    let mut esc_command = if AUTO_ARM_AT_BOOT {
        EscCommand {
            valid: true,
            width_us: ESC_PWM_MIN_US,
            period_us: 2_041,
            arming_request: true,
            throttle_active: true,
            speed_target_hz_x100: FOC_CLOSED_LOOP_TARGET_HZ_X100,
        }
    } else {
        read_esc_command(&esc_input)
    };

    let arm_ready = current_calibration.is_plausible()
        && (ARM_VBUS_MIN_MV..=ARM_VBUS_MAX_MV).contains(&sample.bus_mv)
        && sample.phase_current_ma[0].abs() <= ARM_IDLE_CURRENT_LIMIT_MA
        && sample.phase_current_ma[1].abs() <= ARM_IDLE_CURRENT_LIMIT_MA
        && sample.phase_current_ma[2].abs() <= ARM_IDLE_CURRENT_LIMIT_MA;

    info!("state_transition state={}", ControlState::Disarmed.as_str());
    log_telemetry(TelemetryFrame {
        seq: 0,
        state: ControlState::Disarmed,
        arm_ready,
        esc_valid: esc_command.valid,
        esc_width_us: esc_command.width_us,
        esc_period_us: esc_command.period_us,
        observer_locked: false,
        theta_cmd_idx: 0,
        theta_obs_idx: 0,
        speed_target_hz_x100: esc_command.speed_target_hz_x100,
        speed_est_hz_x100: 0,
        id_ref_ma: 0,
        iq_ref_ma: 0,
        id_ma: 0,
        iq_ma: 0,
        vd_counts: 0,
        vq_counts: 0,
        alpha_counts: 0,
        beta_counts: 0,
        vector_limit_pct: 0,
        vdda_mv: sample.vdda_mv,
        bus_mv: sample.bus_mv,
        bemf_mv: sample.bemf_phase_mv,
        bemf_mag_mv: sample.bemf_mag_mv,
        current_ma: sample.phase_current_ma,
        current_out_mv: sample.current_output_mv,
        duty_pct: [0, 0, 0],
        ntc_mv: sample.ntc_mv,
        ntc_ohms: sample.ntc_ohms,
        mcu_temp_mc: sample.mcu_temp_mc,
        bemf_gpio: sample.bemf_gpio,
        hall_a: sample.hall_a,
        hall_b: sample.hall_b,
        hall_c: sample.hall_c,
        fault: if arm_ready {
            FaultReason::None
        } else {
            FaultReason::ArmConditions
        },
    });

    hold_disarmed(&mut status_led, ARM_DELAY_MS);

    if !arm_ready {
        warn!("arm_conditions_not_met staying_disarmed");
        loop {
            let sample = sample_motor(
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
            esc_command = read_esc_command(&esc_input);
            log_telemetry(TelemetryFrame {
                seq: 0,
                state: ControlState::Disarmed,
                arm_ready: false,
                esc_valid: esc_command.valid,
                esc_width_us: esc_command.width_us,
                esc_period_us: esc_command.period_us,
                observer_locked: false,
                theta_cmd_idx: 0,
                theta_obs_idx: 0,
                speed_target_hz_x100: esc_command.speed_target_hz_x100,
                speed_est_hz_x100: 0,
                id_ref_ma: 0,
                iq_ref_ma: 0,
                id_ma: 0,
                iq_ma: 0,
                vd_counts: 0,
                vq_counts: 0,
                alpha_counts: 0,
                beta_counts: 0,
                vector_limit_pct: 0,
                vdda_mv: sample.vdda_mv,
                bus_mv: sample.bus_mv,
                bemf_mv: sample.bemf_phase_mv,
                bemf_mag_mv: sample.bemf_mag_mv,
                current_ma: sample.phase_current_ma,
                current_out_mv: sample.current_output_mv,
                duty_pct: [0, 0, 0],
                ntc_mv: sample.ntc_mv,
                ntc_ohms: sample.ntc_ohms,
                mcu_temp_mc: sample.mcu_temp_mc,
                bemf_gpio: sample.bemf_gpio,
                hall_a: sample.hall_a,
                hall_b: sample.hall_b,
                hall_c: sample.hall_c,
                fault: FaultReason::ArmConditions,
            });
            hold_disarmed(&mut status_led, ARM_LOG_PERIOD_MS);
        }
    }

    if !AUTO_ARM_AT_BOOT {
        let mut arming_hold_ms = 0u32;
        info!("waiting_for_arm_request source=PA15 mode=esc_pwm");
        loop {
            esc_command = read_esc_command(&esc_input);
            let sample = sample_motor(
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

            if esc_command.valid && esc_command.arming_request {
                arming_hold_ms = arming_hold_ms.saturating_add(ARM_LOG_PERIOD_MS);
            } else {
                arming_hold_ms = 0;
            }

            log_telemetry(TelemetryFrame {
                seq: 0,
                state: ControlState::Disarmed,
                arm_ready: true,
                esc_valid: esc_command.valid,
                esc_width_us: esc_command.width_us,
                esc_period_us: esc_command.period_us,
                observer_locked: false,
                theta_cmd_idx: 0,
                theta_obs_idx: 0,
                speed_target_hz_x100: esc_command.speed_target_hz_x100,
                speed_est_hz_x100: 0,
                id_ref_ma: 0,
                iq_ref_ma: 0,
                id_ma: 0,
                iq_ma: 0,
                vd_counts: 0,
                vq_counts: 0,
                alpha_counts: 0,
                beta_counts: 0,
                vector_limit_pct: 0,
                vdda_mv: sample.vdda_mv,
                bus_mv: sample.bus_mv,
                bemf_mv: sample.bemf_phase_mv,
                bemf_mag_mv: sample.bemf_mag_mv,
                current_ma: sample.phase_current_ma,
                current_out_mv: sample.current_output_mv,
                duty_pct: [0, 0, 0],
                ntc_mv: sample.ntc_mv,
                ntc_ohms: sample.ntc_ohms,
                mcu_temp_mc: sample.mcu_temp_mc,
                bemf_gpio: sample.bemf_gpio,
                hall_a: sample.hall_a,
                hall_b: sample.hall_b,
                hall_c: sample.hall_c,
                fault: FaultReason::ArmRequestMissing,
            });

            if arming_hold_ms >= ESC_ARMING_HOLD_MS {
                info!(
                    "esc_armed width_us={} period_us={} arming_hold_ms={}",
                    esc_command.width_us, esc_command.period_us, arming_hold_ms
                );
                break;
            }

            hold_disarmed(&mut status_led, ARM_LOG_PERIOD_MS);
        }

        info!("waiting_for_throttle_above_min source=PA15");
        while !esc_command.throttle_active {
            esc_command = read_esc_command(&esc_input);
            let sample = sample_motor(
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
            log_telemetry(TelemetryFrame {
                seq: 0,
                state: ControlState::Armed,
                arm_ready: true,
                esc_valid: esc_command.valid,
                esc_width_us: esc_command.width_us,
                esc_period_us: esc_command.period_us,
                observer_locked: false,
                theta_cmd_idx: 0,
                theta_obs_idx: 0,
                speed_target_hz_x100: esc_command.speed_target_hz_x100,
                speed_est_hz_x100: 0,
                id_ref_ma: 0,
                iq_ref_ma: 0,
                id_ma: 0,
                iq_ma: 0,
                vd_counts: 0,
                vq_counts: 0,
                alpha_counts: 0,
                beta_counts: 0,
                vector_limit_pct: 0,
                vdda_mv: sample.vdda_mv,
                bus_mv: sample.bus_mv,
                bemf_mv: sample.bemf_phase_mv,
                bemf_mag_mv: sample.bemf_mag_mv,
                current_ma: sample.phase_current_ma,
                current_out_mv: sample.current_output_mv,
                duty_pct: [0, 0, 0],
                ntc_mv: sample.ntc_mv,
                ntc_ohms: sample.ntc_ohms,
                mcu_temp_mc: sample.mcu_temp_mc,
                bemf_gpio: sample.bemf_gpio,
                hall_a: sample.hall_a,
                hall_b: sample.hall_b,
                hall_c: sample.hall_c,
                fault: FaultReason::ArmRequestMissing,
            });
            hold_disarmed(&mut status_led, ARM_LOG_PERIOD_MS);
        }
    }

    info!("state_transition state={}", ControlState::Armed.as_str());

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

    let max_duty = u32::from(pwm.get_max_duty());
    let center_counts = max_duty / 2;
    let mut seq = 1u32;
    let initial_bus_mv = sample.bus_mv;
    let bus_backoff_mv = initial_bus_mv.saturating_sub(BUS_BACKOFF_DROOP_MV);
    let bus_abort_mv = initial_bus_mv.saturating_sub(BUS_ABORT_DROOP_MV);

    let mut observer = PllObserverState {
        theta_q8: u32::from(0u8) << 8,
        omega_q8: electrical_hz_x100_to_phase_step_q8(ELECTRICAL_FREQ_START_HZ_X100) as i32,
        ..Default::default()
    };
    let mut speed_pi = PiController::new(1, 8, 1, 256, FOC_MIN_IQ_REF_MA, FOC_MAX_IQ_REF_MA);
    let mut id_pi = PiController::new(-1, 18, 1, 160, -120, 120);
    let mut iq_pi = PiController::new(1, 20, 1, 128, -180, 180);

    pwm.set_master_output_enable(true);
    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);
    pwm.set_duty(Channel::Ch1, center_counts);
    pwm.set_duty(Channel::Ch2, center_counts);
    pwm.set_duty(Channel::Ch3, center_counts);

    info!(
        "bootstrap_precharge_start duration_us={} mode=6pwm_centered",
        BOOTSTRAP_PRECHARGE_US
    );
    delay_us(BOOTSTRAP_PRECHARGE_US);
    info!("bootstrap_precharge_complete");

    info!("state_transition state={}", ControlState::Aligning.as_str());
    let mut last_step = ControlStepResult::default();
    for update_index in 0..align_updates() {
        let sample = sample_motor(
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

        let step = run_current_loop(
            &mut pwm,
            max_duty,
            center_counts,
            0,
            FOC_ALIGN_ID_REF_MA,
            0,
            vector_limit_counts(max_duty, FOC_ALIGN_VECTOR_LIMIT_PCT),
            sample.current_alpha_ma,
            sample.current_beta_ma,
            &mut id_pi,
            &mut iq_pi,
        );
        last_step = step;

        if update_index % TELEMETRY_EVERY_UPDATES == 0 {
            log_telemetry(TelemetryFrame {
                seq,
                state: ControlState::Aligning,
                arm_ready: true,
                esc_valid: esc_command.valid,
                esc_width_us: esc_command.width_us,
                esc_period_us: esc_command.period_us,
                observer_locked: false,
                theta_cmd_idx: 0,
                theta_obs_idx: 0,
                speed_target_hz_x100: esc_command.speed_target_hz_x100,
                speed_est_hz_x100: 0,
                id_ref_ma: FOC_ALIGN_ID_REF_MA,
                iq_ref_ma: 0,
                id_ma: step.id_ma,
                iq_ma: step.iq_ma,
                vd_counts: step.vd_counts,
                vq_counts: step.vq_counts,
                alpha_counts: step.alpha_counts,
                beta_counts: step.beta_counts,
                vector_limit_pct: FOC_ALIGN_VECTOR_LIMIT_PCT,
                vdda_mv: sample.vdda_mv,
                bus_mv: sample.bus_mv,
                bemf_mv: sample.bemf_phase_mv,
                bemf_mag_mv: sample.bemf_mag_mv,
                current_ma: sample.phase_current_ma,
                current_out_mv: sample.current_output_mv,
                duty_pct: step.duty_pct,
                ntc_mv: sample.ntc_mv,
                ntc_ohms: sample.ntc_ohms,
                mcu_temp_mc: sample.mcu_temp_mc,
                bemf_gpio: sample.bemf_gpio,
                hall_a: sample.hall_a,
                hall_b: sample.hall_b,
                hall_c: sample.hall_c,
                fault: FaultReason::None,
            });
            seq = seq.wrapping_add(1);
        }

        status_led.toggle();
        delay_us(CONTROL_UPDATE_US);
    }

    let mut theta_cmd_q8 = 0u32;
    let mut open_loop_step_q8 = electrical_hz_x100_to_phase_step_q8(ELECTRICAL_FREQ_START_HZ_X100);
    observer.theta_q8 = theta_cmd_q8;
    observer.omega_q8 = open_loop_step_q8 as i32;
    observer.locked = false;
    speed_pi.reset();
    id_pi.reset();
    iq_pi.reset();

    info!(
        "state_transition state={} update_hz={} pwm_hz={} bus_backoff_mv={} bus_abort_mv={} auto_arm={}",
        ControlState::RevUp.as_str(),
        CONTROL_UPDATE_HZ,
        PWM_FREQ_HZ,
        bus_backoff_mv,
        bus_abort_mv,
        AUTO_ARM_AT_BOOT
    );

    let mut fault = FaultReason::None;
    let mut closed_loop_entered = false;
    let mut speed_target_hz_x100 = esc_command
        .speed_target_hz_x100
        .clamp(SPEED_LOOP_MIN_TARGET_HZ_X100, FOC_CLOSED_LOOP_TARGET_HZ_X100);
    let mut iq_ref_ma = FOC_REVUP_IQ_START_MA;
    let mut esc_stop_counter = 0u16;

    for update_index in 0..FOC_REVUP_DURATION_UPDATES {
        if update_index % SPEED_LOOP_DIVIDER == 0 {
            esc_command = read_esc_command(&esc_input);
            if esc_command.valid && esc_command.throttle_active {
                esc_stop_counter = 0;
            } else {
                esc_stop_counter = esc_stop_counter.saturating_add(1);
                if esc_stop_counter >= ESC_STOP_TICKS {
                    fault = FaultReason::ArmRequestMissing;
                    warn!("esc_stop_request stage=rev_up idx={}", update_index);
                    break;
                }
            }
        }

        let commanded_target_hz_x100 = esc_command
            .speed_target_hz_x100
            .clamp(SPEED_LOOP_MIN_TARGET_HZ_X100, FOC_REVUP_END_HZ_X100);
        let open_loop_hz_x100 = interpolate(
            ELECTRICAL_FREQ_START_HZ_X100,
            commanded_target_hz_x100,
            update_index,
            FOC_REVUP_DURATION_UPDATES,
        );
        speed_target_hz_x100 = open_loop_hz_x100.max(SPEED_LOOP_MIN_TARGET_HZ_X100);
        iq_ref_ma = interpolate_i32(
            FOC_REVUP_IQ_START_MA,
            FOC_REVUP_IQ_END_MA,
            update_index,
            FOC_REVUP_DURATION_UPDATES,
        );

        open_loop_step_q8 = electrical_hz_x100_to_phase_step_q8(open_loop_hz_x100);
        theta_cmd_q8 = theta_cmd_q8.wrapping_add(open_loop_step_q8);

        let sample = sample_motor(
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

        if sample.bus_mv < bus_abort_mv {
            fault = FaultReason::BusAbort;
            warn!(
                "bus_abort stage=rev_up idx={} bus_mv={} target_hz_x100={}",
                update_index, sample.bus_mv, open_loop_hz_x100
            );
            break;
        }

        update_observer(
            &mut observer,
            sample.bemf_alpha_mv,
            sample.bemf_beta_mv,
            theta_cmd_q8,
            open_loop_step_q8,
        );

        let theta_for_control = if observer.locked {
            observer.theta_q8
        } else {
            theta_cmd_q8
        };

        let limit_pct = if sample.bus_mv < bus_backoff_mv {
            FOC_REVUP_VECTOR_LIMIT_PCT.saturating_sub(2)
        } else {
            FOC_REVUP_VECTOR_LIMIT_PCT
        };

        let step = run_current_loop(
            &mut pwm,
            max_duty,
            center_counts,
            theta_for_control,
            0,
            iq_ref_ma,
            vector_limit_counts(max_duty, limit_pct),
            sample.current_alpha_ma,
            sample.current_beta_ma,
            &mut id_pi,
            &mut iq_pi,
        );
        last_step = step;

        if observer.locked && !closed_loop_entered {
            closed_loop_entered = true;
            info!(
                "observer_lock idx={} speed_est_hz_x100={} bemf_mag_mv={}",
                update_index, observer.estimated_hz_x100, observer.bemf_mag_mv
            );
            speed_pi.reset();
        }

        if update_index % TELEMETRY_EVERY_UPDATES == 0 {
            log_telemetry(TelemetryFrame {
                seq,
                state: ControlState::RevUp,
                arm_ready: true,
                esc_valid: esc_command.valid,
                esc_width_us: esc_command.width_us,
                esc_period_us: esc_command.period_us,
                observer_locked: observer.locked,
                theta_cmd_idx: (theta_cmd_q8 >> 8) as u8,
                theta_obs_idx: (observer.theta_q8 >> 8) as u8,
                speed_target_hz_x100,
                speed_est_hz_x100: observer.estimated_hz_x100,
                id_ref_ma: 0,
                iq_ref_ma,
                id_ma: step.id_ma,
                iq_ma: step.iq_ma,
                vd_counts: step.vd_counts,
                vq_counts: step.vq_counts,
                alpha_counts: step.alpha_counts,
                beta_counts: step.beta_counts,
                vector_limit_pct: limit_pct,
                vdda_mv: sample.vdda_mv,
                bus_mv: sample.bus_mv,
                bemf_mv: sample.bemf_phase_mv,
                bemf_mag_mv: sample.bemf_mag_mv,
                current_ma: sample.phase_current_ma,
                current_out_mv: sample.current_output_mv,
                duty_pct: step.duty_pct,
                ntc_mv: sample.ntc_mv,
                ntc_ohms: sample.ntc_ohms,
                mcu_temp_mc: sample.mcu_temp_mc,
                bemf_gpio: sample.bemf_gpio,
                hall_a: sample.hall_a,
                hall_b: sample.hall_b,
                hall_c: sample.hall_c,
                fault: FaultReason::None,
            });
            seq = seq.wrapping_add(1);
        }

        status_led.toggle();
        delay_us(CONTROL_UPDATE_US);
    }

    if fault == FaultReason::None && !observer.locked {
        fault = FaultReason::ObserverNeverLocked;
        warn!("observer_never_locked by_end_of_revup");
    }

    if fault == FaultReason::None {
        info!("state_transition state={}", ControlState::ClosedLoop.as_str());
        for update_index in 0..FOC_CLOSED_LOOP_HOLD_UPDATES {
            if update_index % SPEED_LOOP_DIVIDER == 0 {
                esc_command = read_esc_command(&esc_input);
                if esc_command.valid && esc_command.throttle_active {
                    esc_stop_counter = 0;
                    speed_target_hz_x100 = esc_command
                        .speed_target_hz_x100
                        .clamp(SPEED_LOOP_MIN_TARGET_HZ_X100, FOC_CLOSED_LOOP_TARGET_HZ_X100);
                } else {
                    esc_stop_counter = esc_stop_counter.saturating_add(1);
                    if esc_stop_counter >= ESC_STOP_TICKS {
                        fault = FaultReason::ArmRequestMissing;
                        warn!("esc_stop_request stage=closed_loop idx={}", update_index);
                        break;
                    }
                }
            }

            let sample = sample_motor(
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

            if sample.bus_mv < bus_abort_mv {
                fault = FaultReason::BusAbort;
                warn!(
                    "bus_abort stage=closed_loop idx={} bus_mv={} speed_est_hz_x100={}",
                    update_index, sample.bus_mv, observer.estimated_hz_x100
                );
                break;
            }

            let observer_theta_q8 = observer.theta_q8;
            let observer_omega_q8 = observer.omega_q8.max(32) as u32;
            update_observer(
                &mut observer,
                sample.bemf_alpha_mv,
                sample.bemf_beta_mv,
                observer_theta_q8,
                observer_omega_q8,
            );

            if !observer.locked {
                fault = FaultReason::ObserverUnlock;
                warn!(
                    "observer_unlock idx={} speed_est_hz_x100={} bemf_mag_mv={}",
                    update_index, observer.estimated_hz_x100, observer.bemf_mag_mv
                );
                break;
            }

            if update_index % SPEED_LOOP_DIVIDER == 0 {
                let speed_error = i32::try_from(speed_target_hz_x100).unwrap_or(i32::MAX)
                    - i32::try_from(observer.estimated_hz_x100).unwrap_or_default();
                iq_ref_ma = speed_pi.update(speed_error).clamp(FOC_MIN_IQ_REF_MA, FOC_MAX_IQ_REF_MA);
            }

            let limit_pct = if sample.bus_mv < bus_backoff_mv {
                FOC_CLOSED_LOOP_VECTOR_LIMIT_PCT.saturating_sub(2)
            } else {
                FOC_CLOSED_LOOP_VECTOR_LIMIT_PCT
            };

            let step = run_current_loop(
                &mut pwm,
                max_duty,
                center_counts,
                observer.theta_q8,
                0,
                iq_ref_ma,
                vector_limit_counts(max_duty, limit_pct),
                sample.current_alpha_ma,
                sample.current_beta_ma,
                &mut id_pi,
                &mut iq_pi,
            );
            last_step = step;

            if update_index % TELEMETRY_EVERY_UPDATES == 0 {
                log_telemetry(TelemetryFrame {
                    seq,
                    state: ControlState::ClosedLoop,
                    arm_ready: true,
                    esc_valid: esc_command.valid,
                    esc_width_us: esc_command.width_us,
                    esc_period_us: esc_command.period_us,
                    observer_locked: observer.locked,
                    theta_cmd_idx: (observer.theta_q8 >> 8) as u8,
                    theta_obs_idx: (observer.theta_q8 >> 8) as u8,
                    speed_target_hz_x100,
                    speed_est_hz_x100: observer.estimated_hz_x100,
                    id_ref_ma: 0,
                    iq_ref_ma,
                    id_ma: step.id_ma,
                    iq_ma: step.iq_ma,
                    vd_counts: step.vd_counts,
                    vq_counts: step.vq_counts,
                    alpha_counts: step.alpha_counts,
                    beta_counts: step.beta_counts,
                    vector_limit_pct: limit_pct,
                    vdda_mv: sample.vdda_mv,
                    bus_mv: sample.bus_mv,
                    bemf_mv: sample.bemf_phase_mv,
                    bemf_mag_mv: sample.bemf_mag_mv,
                    current_ma: sample.phase_current_ma,
                    current_out_mv: sample.current_output_mv,
                    duty_pct: step.duty_pct,
                    ntc_mv: sample.ntc_mv,
                    ntc_ohms: sample.ntc_ohms,
                    mcu_temp_mc: sample.mcu_temp_mc,
                    bemf_gpio: sample.bemf_gpio,
                    hall_a: sample.hall_a,
                    hall_b: sample.hall_b,
                    hall_c: sample.hall_c,
                    fault: FaultReason::None,
                });
                seq = seq.wrapping_add(1);
            }

            status_led.toggle();
            delay_us(CONTROL_UPDATE_US);
        }
    }

    pwm.set_master_output_enable(false);
    disable_all_channels(&mut pwm);

    let final_state = if fault == FaultReason::None {
        ControlState::Disarmed
    } else {
        ControlState::Faulted
    };
    info!("state_transition state={}", final_state.as_str());

    loop {
        let sample = sample_motor(
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
        log_telemetry(TelemetryFrame {
            seq,
            state: final_state,
            arm_ready: false,
            esc_valid: esc_command.valid,
            esc_width_us: esc_command.width_us,
            esc_period_us: esc_command.period_us,
            observer_locked: observer.locked,
            theta_cmd_idx: 0,
            theta_obs_idx: (observer.theta_q8 >> 8) as u8,
            speed_target_hz_x100,
            speed_est_hz_x100: observer.estimated_hz_x100,
            id_ref_ma: 0,
            iq_ref_ma,
            id_ma: last_step.id_ma,
            iq_ma: last_step.iq_ma,
            vd_counts: last_step.vd_counts,
            vq_counts: last_step.vq_counts,
            alpha_counts: last_step.alpha_counts,
            beta_counts: last_step.beta_counts,
            vector_limit_pct: 0,
            vdda_mv: sample.vdda_mv,
            bus_mv: sample.bus_mv,
            bemf_mv: sample.bemf_phase_mv,
            bemf_mag_mv: sample.bemf_mag_mv,
            current_ma: sample.phase_current_ma,
            current_out_mv: sample.current_output_mv,
            duty_pct: [0, 0, 0],
            ntc_mv: sample.ntc_mv,
            ntc_ohms: sample.ntc_ohms,
            mcu_temp_mc: sample.mcu_temp_mc,
            bemf_gpio: sample.bemf_gpio,
            hall_a: sample.hall_a,
            hall_b: sample.hall_b,
            hall_c: sample.hall_c,
            fault,
        });
        seq = seq.wrapping_add(1);
        hold_disarmed(&mut status_led, ARM_LOG_PERIOD_MS);
    }
}

fn align_updates() -> usize {
    (FOC_ALIGN_HOLD_US as usize / CONTROL_UPDATE_US as usize).max(1)
}

fn read_esc_command(esc_input: &PwmInput<'_, peripherals::TIM2>) -> EscCommand {
    let period_us = esc_input.get_period_ticks();
    let width_us = esc_input.get_width_ticks();
    let valid = (ESC_PWM_PERIOD_MIN_US..=ESC_PWM_PERIOD_MAX_US).contains(&period_us)
        && (ESC_PWM_ARMING_US..=ESC_PWM_MAX_US).contains(&width_us);
    let arming_request = valid && (ESC_PWM_ARMING_US..ESC_PWM_MIN_US).contains(&width_us);
    let throttle_active = valid && width_us >= ESC_PWM_MIN_US;

    let speed_target_hz_x100 = if throttle_active {
        let clipped_width_us = width_us.min(ESC_PWM_MAX_US);
        let mech_rpm = ((clipped_width_us - ESC_PWM_MIN_US)
            * (ESC_MAX_SPEED_RPM - ESC_MIN_SPEED_RPM)
            / (ESC_PWM_MAX_US - ESC_PWM_MIN_US))
            + ESC_MIN_SPEED_RPM;
        mech_rpm * ESC_POLE_PAIRS * 100 / 60
    } else {
        0
    };

    EscCommand {
        valid,
        width_us,
        period_us,
        arming_request,
        throttle_active,
        speed_target_hz_x100,
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

fn clamp_i32(value: i32, min_value: i32, max_value: i32) -> i32 {
    value.clamp(min_value, max_value)
}

fn clarke_beta(b: i32, c: i32) -> i32 {
    ((i64::from(b - c) * i64::from(SQRT3_INV_Q15)) / 32_768) as i32
}

fn center_i32_triplet(values: [i32; 3]) -> [i32; 3] {
    let average = (values[0] + values[1] + values[2]) / 3;
    [
        values[0] - average,
        values[1] - average,
        values[2] - average,
    ]
}

fn compute_phase_duty(
    phase_counts: i32,
    center_counts: u32,
    max_duty: u32,
) -> u16 {
    let duty = i32::try_from(center_counts).unwrap_or_default() + phase_counts;
    duty.clamp(0, i32::try_from(max_duty).unwrap_or(i32::MAX)) as u16
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
        0
    } else {
        u32::from(duty_counts) * 100 / max_duty
    }
}

fn electrical_hz_x100_to_phase_step_q8(electrical_hz_x100: u32) -> u32 {
    let numerator =
        u64::from(electrical_hz_x100) * 65_536 + u64::from((CONTROL_UPDATE_HZ * 100) / 2);
    let denominator = u64::from(CONTROL_UPDATE_HZ * 100);
    (numerator / denominator) as u32
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

fn interpolate_i32(start: i32, end: i32, index: usize, len: usize) -> i32 {
    if len <= 1 {
        return end;
    }

    let start = i64::from(start);
    let end = i64::from(end);
    let delta = end - start;
    let step = delta * index as i64 / (len as i64 - 1);
    (start + step) as i32
}

fn inverse_park(vd: i32, vq: i32, angle_idx: u8) -> (i32, i32) {
    let (sin_theta, cos_theta) = sin_cos_q7(angle_idx);
    let alpha = (i64::from(vd) * i64::from(cos_theta) - i64::from(vq) * i64::from(sin_theta))
        / 127;
    let beta = (i64::from(vd) * i64::from(sin_theta) + i64::from(vq) * i64::from(cos_theta))
        / 127;
    (alpha as i32, beta as i32)
}

fn limit_phase_vector(alpha: i32, beta: i32, limit_counts: i32) -> (i32, i32) {
    let phase = center_i32_triplet([
        alpha,
        -alpha / 2 + ((i64::from(beta) * i64::from(SQRT3_HALF_Q15)) / 32_768) as i32,
        -alpha / 2 - ((i64::from(beta) * i64::from(SQRT3_HALF_Q15)) / 32_768) as i32,
    ]);
    let max_abs = phase
        .iter()
        .map(|value| value.abs())
        .max()
        .unwrap_or_default()
        .max(1);

    if max_abs <= limit_counts {
        return (alpha, beta);
    }

    (
        (i64::from(alpha) * i64::from(limit_counts) / i64::from(max_abs)) as i32,
        (i64::from(beta) * i64::from(limit_counts) / i64::from(max_abs)) as i32,
    )
}

fn log_telemetry(frame: TelemetryFrame) {
    info!(
        "telemetry={{seq:{},state:{},arm_ready:{},esc_valid:{},esc_width_us:{},esc_period_us:{},obs_locked:{},theta_cmd_idx:{},theta_obs_idx:{},speed_target_hz_x100:{},speed_est_hz_x100:{},id_ref_ma:{},iq_ref_ma:{},id_ma:{},iq_ma:{},vd_counts:{},vq_counts:{},ab_counts:[{},{}],vector_limit_pct:{},vdda_mv:{},bus_mv:{},bemf_mv:[{},{},{}],bemf_mag_mv:{},current_ma:[{},{},{}],current_out_mv:[{},{},{}],duty_pct:[{},{},{}],ntc_mv:{},ntc_ohms:{},mcu_temp_mc:{},bemf_gpio:{},hall:[{},{},{}],fault:{}}}",
        frame.seq,
        frame.state.as_str(),
        frame.arm_ready,
        frame.esc_valid,
        frame.esc_width_us,
        frame.esc_period_us,
        frame.observer_locked,
        frame.theta_cmd_idx,
        frame.theta_obs_idx,
        frame.speed_target_hz_x100,
        frame.speed_est_hz_x100,
        frame.id_ref_ma,
        frame.iq_ref_ma,
        frame.id_ma,
        frame.iq_ma,
        frame.vd_counts,
        frame.vq_counts,
        frame.alpha_counts,
        frame.beta_counts,
        frame.vector_limit_pct,
        frame.vdda_mv,
        frame.bus_mv,
        frame.bemf_mv[0],
        frame.bemf_mv[1],
        frame.bemf_mv[2],
        frame.bemf_mag_mv,
        frame.current_ma[0],
        frame.current_ma[1],
        frame.current_ma[2],
        frame.current_out_mv[0],
        frame.current_out_mv[1],
        frame.current_out_mv[2],
        frame.duty_pct[0],
        frame.duty_pct[1],
        frame.duty_pct[2],
        frame.ntc_mv,
        frame.ntc_ohms,
        frame.mcu_temp_mc,
        frame.bemf_gpio,
        frame.hall_a,
        frame.hall_b,
        frame.hall_c,
        frame.fault.as_str(),
    );
}

fn park_transform(alpha: i32, beta: i32, angle_idx: u8) -> (i32, i32) {
    let (sin_theta, cos_theta) = sin_cos_q7(angle_idx);
    let d =
        (i64::from(alpha) * i64::from(cos_theta) + i64::from(beta) * i64::from(sin_theta)) / 127;
    let q =
        (-i64::from(alpha) * i64::from(sin_theta) + i64::from(beta) * i64::from(cos_theta))
            / 127;
    (d as i32, q as i32)
}

fn phase_step_q8_to_electrical_hz_x100(step_q8: u32) -> u32 {
    ((u64::from(step_q8) * u64::from(CONTROL_UPDATE_HZ) * 100) / 65_536) as u32
}

fn read_calibration_word(addr: *const u16) -> u16 {
    unsafe { ptr::read_volatile(addr) }
}

fn run_current_loop(
    pwm: &mut ComplementaryPwm<'_, peripherals::TIM1>,
    max_duty: u32,
    center_counts: u32,
    theta_q8: u32,
    id_ref_ma: i32,
    iq_ref_ma: i32,
    vector_limit_counts: i32,
    current_alpha_ma: i32,
    current_beta_ma: i32,
    id_pi: &mut PiController,
    iq_pi: &mut PiController,
) -> ControlStepResult {
    let theta_idx = (theta_q8 >> 8) as u8;
    let (id_ma, iq_ma) = park_transform(current_alpha_ma, current_beta_ma, theta_idx);

    let vd_counts = id_pi.update(id_ref_ma - id_ma);
    let vq_counts = iq_pi.update(iq_ref_ma - iq_ma);
    let (alpha_counts, beta_counts) = inverse_park(vd_counts, vq_counts, theta_idx);
    let (alpha_counts, beta_counts) =
        limit_phase_vector(alpha_counts, beta_counts, vector_limit_counts);
    let duty_pct = set_phase_vector_duties(
        pwm,
        alpha_counts,
        beta_counts,
        center_counts,
        max_duty,
    );

    ControlStepResult {
        duty_pct,
        id_ma,
        iq_ma,
        vd_counts,
        vq_counts,
        alpha_counts,
        beta_counts,
    }
}

#[allow(clippy::too_many_arguments)]
fn sample_motor(
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
) -> MotorSample {
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

    let bemf_phase_mv = center_i32_triplet([
        adc_raw_to_mv(bemf_a_raw, vdda_mv) as i32,
        adc_raw_to_mv(bemf_b_raw, vdda_mv) as i32,
        adc_raw_to_mv(bemf_c_raw, vdda_mv) as i32,
    ]);
    let phase_current_ma = center_i32_triplet([
        estimate_phase_current_ma(current_a_raw, vdda_mv, current_calibration.zero_a_uv),
        estimate_phase_current_ma(current_b_raw, vdda_mv, current_calibration.zero_b_uv),
        estimate_phase_current_ma(current_c_raw, vdda_mv, current_calibration.zero_c_uv),
    ]);
    let bemf_alpha_mv = bemf_phase_mv[0];
    let bemf_beta_mv = clarke_beta(bemf_phase_mv[1], bemf_phase_mv[2]);
    let current_alpha_ma = phase_current_ma[0];
    let current_beta_ma = clarke_beta(phase_current_ma[1], phase_current_ma[2]);

    MotorSample {
        vdda_mv,
        bus_mv: estimate_bus_mv(bus_raw, vdda_mv),
        bemf_phase_mv,
        bemf_alpha_mv,
        bemf_beta_mv,
        bemf_mag_mv: approx_vector_mag(bemf_alpha_mv, bemf_beta_mv),
        phase_current_ma,
        current_alpha_ma,
        current_beta_ma,
        current_output_mv: [
            adc_raw_to_mv(current_a_raw, vdda_mv),
            adc_raw_to_mv(current_b_raw, vdda_mv),
            adc_raw_to_mv(current_c_raw, vdda_mv),
        ],
        ntc_mv: adc_raw_to_mv(ntc_raw, vdda_mv),
        ntc_ohms: estimate_ntc_ohms(ntc_raw),
        mcu_temp_mc: estimate_mcu_temp_mc(mcu_temp_raw, vdda_mv, calibration),
        bemf_gpio: bemf_gpio.is_high(),
        hall_a: hall_a.is_high(),
        hall_b: hall_b.is_high(),
        hall_c: hall_c.is_high(),
    }
}

fn set_phase_vector_duties(
    pwm: &mut ComplementaryPwm<'_, peripherals::TIM1>,
    alpha_counts: i32,
    beta_counts: i32,
    center_counts: u32,
    max_duty: u32,
) -> [u32; 3] {
    let phase = center_i32_triplet([
        alpha_counts,
        -alpha_counts / 2 + ((i64::from(beta_counts) * i64::from(SQRT3_HALF_Q15)) / 32_768) as i32,
        -alpha_counts / 2 - ((i64::from(beta_counts) * i64::from(SQRT3_HALF_Q15)) / 32_768) as i32,
    ]);

    let duty_a = compute_phase_duty(phase[0], center_counts, max_duty);
    let duty_b = compute_phase_duty(phase[1], center_counts, max_duty);
    let duty_c = compute_phase_duty(phase[2], center_counts, max_duty);

    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);
    pwm.set_duty(Channel::Ch1, u32::from(duty_a));
    pwm.set_duty(Channel::Ch2, u32::from(duty_b));
    pwm.set_duty(Channel::Ch3, u32::from(duty_c));

    [
        duty_counts_to_pct(duty_a, max_duty),
        duty_counts_to_pct(duty_b, max_duty),
        duty_counts_to_pct(duty_c, max_duty),
    ]
}

fn sin_cos_q7(angle_idx: u8) -> (i32, i32) {
    (
        i32::from(SINE_TABLE[angle_idx as usize]) - 127,
        i32::from(SINE_TABLE[angle_idx.wrapping_add(64) as usize]) - 127,
    )
}

fn vector_limit_counts(max_duty: u32, limit_pct: u32) -> i32 {
    i32::try_from((max_duty * limit_pct / 100) / 2).unwrap_or_default().max(1)
}

fn approx_vector_mag(alpha: i32, beta: i32) -> u32 {
    let alpha_abs = alpha.unsigned_abs();
    let beta_abs = beta.unsigned_abs();
    alpha_abs.max(beta_abs) + alpha_abs.min(beta_abs) / 2
}

fn atan2_idx(beta: i32, alpha: i32) -> u8 {
    if alpha == 0 && beta == 0 {
        return 0;
    }

    let abs_x = alpha.abs().max(1);
    let abs_y = beta.abs();
    let octant = if abs_x >= abs_y {
        (abs_y * 32 / abs_x) as u8
    } else {
        (64 - (abs_x * 32 / abs_y.max(1))) as u8
    };

    match (alpha >= 0, beta >= 0) {
        (true, true) => octant,
        (false, true) => 128u8.wrapping_sub(octant),
        (false, false) => 128u8.wrapping_add(octant),
        (true, false) => 0u8.wrapping_sub(octant),
    }
}

fn adc_raw_to_mv(raw: u16, vdda_mv: u32) -> u32 {
    u32::from(raw) * vdda_mv / ADC_FULL_SCALE
}

fn adc_raw_to_uv(raw: u16, vdda_mv: u32) -> i32 {
    (u64::from(raw) * u64::from(vdda_mv) * 1_000 / u64::from(ADC_FULL_SCALE)) as i32
}

fn wrap_angle_error(target_idx: u8, current_idx: u8) -> i32 {
    i32::from(target_idx.wrapping_sub(current_idx) as i8)
}

fn update_observer(
    observer: &mut PllObserverState,
    bemf_alpha_mv: i32,
    bemf_beta_mv: i32,
    open_loop_theta_q8: u32,
    open_loop_step_q8: u32,
) {
    observer.bemf_mag_mv = approx_vector_mag(bemf_alpha_mv, bemf_beta_mv);

    if observer.bemf_mag_mv < OBSERVER_LOCK_BEMF_MIN_MV {
        observer.lock_counter = 0;
        observer.unlock_counter = observer.unlock_counter.saturating_add(1);
        observer.theta_q8 = open_loop_theta_q8;
        observer.omega_q8 = open_loop_step_q8 as i32;
        observer.estimated_hz_x100 = phase_step_q8_to_electrical_hz_x100(open_loop_step_q8);
        observer.angle_error_idx = 0;
        observer.locked = false;
        return;
    }

    let bemf_angle_idx = atan2_idx(bemf_beta_mv, bemf_alpha_mv);
    let flux_angle_idx = bemf_angle_idx.wrapping_sub(BEMF_FLUX_SHIFT_IDX);
    let predicted_idx = (observer.theta_q8 >> 8) as u8;
    let angle_error_idx = wrap_angle_error(flux_angle_idx, predicted_idx);

    observer.integral = clamp_i32(
        observer.integral + angle_error_idx * OBSERVER_PLL_KI_NUM,
        -8_192,
        8_192,
    );

    let base_step = if observer.locked {
        observer.omega_q8
    } else {
        open_loop_step_q8 as i32
    };
    let corrected_step = clamp_i32(
        base_step + angle_error_idx * OBSERVER_PLL_KP_NUM + observer.integral / OBSERVER_PLL_INTEGRAL_DIV,
        32,
        8_192,
    );

    observer.theta_q8 = observer.theta_q8.wrapping_add(corrected_step as u32);
    observer.omega_q8 = corrected_step;
    observer.estimated_hz_x100 = phase_step_q8_to_electrical_hz_x100(corrected_step as u32);
    observer.angle_error_idx = angle_error_idx as i16;

    if angle_error_idx.abs() <= OBSERVER_LOCK_ERR_IDX {
        observer.lock_counter = observer.lock_counter.saturating_add(1);
        observer.unlock_counter = 0;
        if observer.lock_counter >= OBSERVER_LOCK_CYCLES {
            observer.locked = true;
        }
    } else {
        observer.lock_counter = 0;
        if observer.locked {
            observer.unlock_counter = observer.unlock_counter.saturating_add(1);
            if observer.unlock_counter >= OBSERVER_UNLOCK_CYCLES {
                observer.locked = false;
            }
        }
    }
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
