#![no_std]
#![no_main]

use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

use defmt::{debug, info, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcConfig, SampleTime, VREF_CALIB_MV};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::opamp::{OpAmp, OpAmpGain, OpAmpSpeed};
use embassy_stm32::rcc::mux::Adcsel;
use embassy_stm32::rcc::{AHBPrescaler, APBPrescaler, Sysclk};
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::complementary_pwm::ComplementaryPwm;
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::{Peri, peripherals};
use embassy_time::{Duration, Ticker};
use panic_probe as _;

static DEFMT_TICKS: AtomicU32 = AtomicU32::new(0);

const ADC_FULL_SCALE: u32 = 4095;
const ARM_AT_BOOT: bool = false;
const ARM_IDLE_CURRENT_LIMIT_MA: i32 = 1_500;
const ARM_VBUS_MAX_MV: u32 = 12_000;
const ARM_VBUS_MIN_MV: u32 = 7_000;
const CURRENT_OUTPUT_UV_PER_AMP: i32 = 27_429;
const CURRENT_ZERO_CAL_SAMPLES: usize = 64;
const CURRENT_ZERO_UV_NOMINAL: i32 = 2_057_143;
const CURRENT_ZERO_UV_TOLERANCE: i32 = 250_000;
const NTC_PULLDOWN_OHMS: u32 = 4_700;
const SAMPLE_TIME: SampleTime = SampleTime::CYCLES640_5;
const TIM1_STANDBY_PWM_HZ: u32 = 20_000;
const TELEMETRY_PERIOD_MS: u64 = 250;
const TS_CAL1_TEMP_MC: i32 = 30_000;
const TS_CAL2_TEMP_MC: i32 = 130_000;
const TS_CAL1_ADDR: *const u16 = 0x1FFF_75A8 as *const u16;
const TS_CAL2_ADDR: *const u16 = 0x1FFF_75CA as *const u16;

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
struct TelemetryFrame {
    seq: u32,
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
    arm_ready: bool,
    state: BringupState,
}

#[derive(Clone, Copy)]
struct CurrentCalibration {
    zero_a_uv: i32,
    zero_b_uv: i32,
    zero_c_uv: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BringupState {
    Booting,
    Calibrating,
    Disarmed,
    Armed,
}

impl BringupState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Booting => "booting",
            Self::Calibrating => "calibrating",
            Self::Disarmed => "disarmed",
            Self::Armed => "armed",
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
    info!("boot_entry");
    info!("state_transition state={}", BringupState::Booting.as_str());

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
    info!("clocks_and_hal_ready");

    let _phase_a_high = Output::new(p.PA8, Level::Low, Speed::Low);
    let _phase_a_low = Output::new(p.PC13, Level::Low, Speed::Low);
    let _phase_b_high = Output::new(p.PA9, Level::Low, Speed::Low);
    let _phase_b_low = Output::new(p.PA12, Level::Low, Speed::Low);
    let _phase_c_high = Output::new(p.PA10, Level::Low, Speed::Low);
    let _phase_c_low = Output::new(p.PB15, Level::Low, Speed::Low);
    let mut status_led = Output::new(p.PC15, Level::Low, Speed::Low);
    info!("safe_gate_gpio_ready");

    let bemf_gpio = Input::new(p.PB5, Pull::None);
    let hall_a = Input::new(p.PB6, Pull::None);
    let hall_b = Input::new(p.PB7, Pull::None);
    let hall_c = Input::new(p.PB8, Pull::None);
    info!("digital_inputs_ready");

    let mut tim1 = ComplementaryPwm::new(
        p.TIM1,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Hertz::hz(TIM1_STANDBY_PWM_HZ),
        CountingMode::EdgeAlignedUp,
    );
    tim1.set_master_output_enable(false);
    tim1.disable(embassy_stm32::timer::Channel::Ch1);
    tim1.disable(embassy_stm32::timer::Channel::Ch2);
    tim1.disable(embassy_stm32::timer::Channel::Ch3);
    info!("tim1_standby_ready");

    let mut adc1 = Adc::new(p.ADC1, AdcConfig::default());
    let mut adc2 = Adc::new(p.ADC2, AdcConfig::default());
    info!("adc_ready");

    let mut vbus = p.PA0;
    let mut bemf_a = p.PA4;
    let mut bemf_b = p.PB12;
    let mut bemf_c = p.PB11;
    let mut board_ntc = p.PB14;

    let mut vrefint = adc1.enable_vrefint();
    let mut mcu_temp = adc1.enable_temperature();
    info!("adc_internal_channels_ready");

    let calibration = FactoryCalibration::read();
    info!(
        "factory_cal_loaded vrefint={} ts_cal1={} ts_cal2={}",
        calibration.vrefint, calibration.ts_cal1, calibration.ts_cal2
    );

    let mut opamp1 = OpAmp::new(p.OPAMP1, OpAmpSpeed::Normal);
    let mut opamp2 = OpAmp::new(p.OPAMP2, OpAmpSpeed::Normal);
    let mut opamp3 = OpAmp::new(p.OPAMP3, OpAmpSpeed::Normal);
    info!("opamps_created");
    info!("opamp1_calibrate_start");
    opamp1.calibrate();
    info!("opamp1_calibrate_done");
    info!("opamp2_calibrate_start");
    opamp2.calibrate();
    info!("opamp2_calibrate_done");
    info!("opamp3_calibrate_start");
    opamp3.calibrate();
    info!("opamp3_calibrate_done");

    let mut current_a = opamp1.pga_ext(p.PA1, p.PA2, OpAmpGain::Mul16);
    let mut current_b = opamp2.pga_ext(p.PA7, p.PA6, OpAmpGain::Mul16);
    let mut current_c = opamp3.pga_ext(p.PB0, p.PB1, OpAmpGain::Mul16);
    info!("opamp_pga_outputs_ready");
    info!(
        "state_transition state={}",
        BringupState::Calibrating.as_str()
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
    if !current_calibration.is_plausible() {
        warn!("current_zero_calibration_out_of_range");
    }

    info!(
        "bringup_start mode=sensor_only mcu=STM32G431C8 sysclk_hz=16000000 tim1_pwm_hz={} tim1_moe={} opamp_gain=16",
        TIM1_STANDBY_PWM_HZ,
        tim1.get_master_output_enable()
    );
    info!(
        "factory_cal vrefint={} ts_cal1={} ts_cal2={}",
        calibration.vrefint, calibration.ts_cal1, calibration.ts_cal2
    );
    warn!("tim1_outputs_are_not_muxed_to_pins gate_inputs_are_forced_low_by_gpio");

    let mut ticker = Ticker::every(Duration::from_millis(TELEMETRY_PERIOD_MS));
    let mut seq = 0u32;
    let startup_state = if ARM_AT_BOOT && current_calibration.is_plausible() {
        BringupState::Armed
    } else {
        BringupState::Disarmed
    };
    info!("state_transition state={}", startup_state.as_str());

    loop {
        ticker.next().await;
        status_led.toggle();

        let frame = TelemetryFrame {
            seq,
            vdda_mv: estimate_vdda_mv(
                adc1.blocking_read(&mut vrefint, SAMPLE_TIME),
                calibration.vrefint,
            ),
            bus_mv: 0,
            bemf_a_mv: 0,
            bemf_b_mv: 0,
            bemf_c_mv: 0,
            current_a_ma: 0,
            current_b_ma: 0,
            current_c_ma: 0,
            current_a_output_mv: 0,
            current_b_output_mv: 0,
            current_c_output_mv: 0,
            ntc_mv: 0,
            ntc_ohms: -1,
            mcu_temp_mc: 0,
            bemf_gpio: bemf_gpio.is_high(),
            hall_a: hall_a.is_high(),
            hall_b: hall_b.is_high(),
            hall_c: hall_c.is_high(),
            arm_ready: false,
            state: startup_state,
        };
        let frame = populate_measurements(
            frame,
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
            current_calibration,
            calibration,
        );
        let arm_ready = is_arm_ready(frame, current_calibration);
        let state = if ARM_AT_BOOT && arm_ready {
            BringupState::Armed
        } else {
            BringupState::Disarmed
        };
        let frame = TelemetryFrame {
            arm_ready,
            state,
            ..frame
        };

        debug!(
            "raw seq={} ntc_mv={} current_out_mv=[{},{},{}]",
            frame.seq,
            frame.ntc_mv,
            frame.current_a_output_mv,
            frame.current_b_output_mv,
            frame.current_c_output_mv
        );
        info!(
            "telemetry={{seq:{},state:{},arm_ready:{},vdda_mv:{},bus_mv:{},bemf_mv:[{},{},{}],current_ma:[{},{},{}],current_out_mv:[{},{},{}],ntc_mv:{},ntc_ohms:{},mcu_temp_mc:{},bemf_gpio:{},hall:[{},{},{}]}}",
            frame.seq,
            frame.state.as_str(),
            frame.arm_ready,
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

        seq = seq.wrapping_add(1);
    }
}

fn populate_measurements(
    mut frame: TelemetryFrame,
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
    current_calibration: CurrentCalibration,
    calibration: FactoryCalibration,
) -> TelemetryFrame {
    let bus_raw = adc1.blocking_read(vbus, SAMPLE_TIME);
    let bemf_a_raw = adc2.blocking_read(bemf_a, SAMPLE_TIME);
    let bemf_b_raw = adc1.blocking_read(bemf_b, SAMPLE_TIME);
    let bemf_c_raw = adc2.blocking_read(bemf_c, SAMPLE_TIME);
    let ntc_raw = adc1.blocking_read(board_ntc, SAMPLE_TIME);
    let current_a_raw = adc1.blocking_read(current_a, SAMPLE_TIME);
    let current_b_raw = adc2.blocking_read(current_b, SAMPLE_TIME);
    let current_c_raw = adc1.blocking_read(current_c, SAMPLE_TIME);
    let mcu_temp_raw = adc1.blocking_read(mcu_temp, SAMPLE_TIME);

    frame.bus_mv = estimate_bus_mv(bus_raw, frame.vdda_mv);
    frame.bemf_a_mv = adc_raw_to_mv(bemf_a_raw, frame.vdda_mv);
    frame.bemf_b_mv = adc_raw_to_mv(bemf_b_raw, frame.vdda_mv);
    frame.bemf_c_mv = adc_raw_to_mv(bemf_c_raw, frame.vdda_mv);
    frame.current_a_output_mv = adc_raw_to_mv(current_a_raw, frame.vdda_mv);
    frame.current_b_output_mv = adc_raw_to_mv(current_b_raw, frame.vdda_mv);
    frame.current_c_output_mv = adc_raw_to_mv(current_c_raw, frame.vdda_mv);
    frame.current_a_ma =
        estimate_phase_current_ma(current_a_raw, frame.vdda_mv, current_calibration.zero_a_uv);
    frame.current_b_ma =
        estimate_phase_current_ma(current_b_raw, frame.vdda_mv, current_calibration.zero_b_uv);
    frame.current_c_ma =
        estimate_phase_current_ma(current_c_raw, frame.vdda_mv, current_calibration.zero_c_uv);
    frame.ntc_mv = adc_raw_to_mv(ntc_raw, frame.vdda_mv);
    frame.ntc_ohms = estimate_ntc_ohms(ntc_raw);
    frame.mcu_temp_mc = estimate_mcu_temp_mc(mcu_temp_raw, frame.vdda_mv, calibration);
    frame
}

fn adc_raw_to_mv(raw: u16, vdda_mv: u32) -> u32 {
    u32::from(raw) * vdda_mv / ADC_FULL_SCALE
}

fn adc_raw_to_uv(raw: u16, vdda_mv: u32) -> i32 {
    (u64::from(raw) * u64::from(vdda_mv) * 1_000 / u64::from(ADC_FULL_SCALE)) as i32
}

fn estimate_vdda_mv(vref_raw: u16, vrefint_cal: u16) -> u32 {
    if vref_raw == 0 {
        return VREF_CALIB_MV;
    }

    VREF_CALIB_MV * u32::from(vrefint_cal) / u32::from(vref_raw)
}

fn estimate_bus_mv(bus_raw: u16, vdda_mv: u32) -> u32 {
    adc_raw_to_mv(bus_raw, vdda_mv) * 187 / 18
}

fn estimate_phase_current_ma(raw: u16, vdda_mv: u32, zero_uv: i32) -> i32 {
    let delta_uv = i64::from(adc_raw_to_uv(raw, vdda_mv) - zero_uv);
    (delta_uv * 1_000 / i64::from(CURRENT_OUTPUT_UV_PER_AMP)) as i32
}

fn estimate_ntc_ohms(raw: u16) -> i32 {
    if raw == 0 {
        return -1;
    }

    let raw = u32::from(raw);
    (NTC_PULLDOWN_OHMS * (ADC_FULL_SCALE - raw) / raw) as i32
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

fn is_arm_ready(frame: TelemetryFrame, current_calibration: CurrentCalibration) -> bool {
    current_calibration.is_plausible()
        && (ARM_VBUS_MIN_MV..=ARM_VBUS_MAX_MV).contains(&frame.bus_mv)
        && frame.current_a_ma.abs() <= ARM_IDLE_CURRENT_LIMIT_MA
        && frame.current_b_ma.abs() <= ARM_IDLE_CURRENT_LIMIT_MA
        && frame.current_c_ma.abs() <= ARM_IDLE_CURRENT_LIMIT_MA
}
