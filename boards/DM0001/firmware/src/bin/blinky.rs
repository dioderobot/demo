#![no_std]
#![no_main]

//! Simple blinky example for WV0001 board
//! 
//! This example blinks the debug LED (PB13) to verify basic functionality

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("WV0001 Blinky Started!");

    // PB13: Debug LED (blue, active high)
    let mut debug_led = Output::new(p.PC15, Level::Low, Speed::Low);

    loop {
        info!("Debug LED ON");
        debug_led.set_high();
        Timer::after_millis(500).await;

        info!("Debug LED OFF");
        debug_led.set_low();
        Timer::after_millis(500).await;
    }
}