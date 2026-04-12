#![no_std]
#![no_main]

use cortex_m::asm;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use panic_probe as _;

#[embassy_executor::main(
    executor = "embassy_stm32::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let _phase_a_high = Output::new(p.PA8, Level::Low, Speed::Low);
    let _phase_a_low = Output::new(p.PC13, Level::Low, Speed::Low);
    let _phase_b_high = Output::new(p.PA9, Level::Low, Speed::Low);
    let _phase_b_low = Output::new(p.PA12, Level::Low, Speed::Low);
    let _phase_c_high = Output::new(p.PA10, Level::Low, Speed::Low);
    let _phase_c_low = Output::new(p.PB15, Level::Low, Speed::Low);
    let mut status_led = Output::new(p.PC15, Level::Low, Speed::Low);

    info!("rtt_smoke_start");

    let mut tick = 0u32;
    loop {
        status_led.toggle();
        info!("rtt_smoke_tick {}", tick);
        tick = tick.wrapping_add(1);
        asm::delay(8_000_000);
    }
}
