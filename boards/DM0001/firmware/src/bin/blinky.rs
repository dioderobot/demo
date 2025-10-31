#![no_std]
#![no_main]

//! Simple blinky example for DM0001 board
//! 
//! This example blinks the Status LED (PC15, red) to verify basic functionality
//! The Power LED (V3V3, green) should be always on if the board is powered correctly

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("DM0001 Blinky Started!");

    // PC15: Status LED (red, active high)
    let mut status_led = Output::new(p.PC15, Level::Low, Speed::Low);

    info!("Starting blink loop...");
    loop {
        info!("Status LED ON");
        status_led.set_high();
        Timer::after_millis(500).await;

        info!("Status LED OFF");
        status_led.set_low();
        Timer::after_millis(500).await;
    }
}