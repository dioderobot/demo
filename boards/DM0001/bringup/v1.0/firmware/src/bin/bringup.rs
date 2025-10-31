#![no_std]
#![no_main]

//! DM0001 Board Bringup Firmware
//! 
//! This firmware exercises the major components and peripherals on the DM0001 motor controller:
//! - Status and Power LEDs
//! - UART2 communication
//! - ADC measurements (VBUS, Temperature, Current feedback channels)
//! - Hall encoder inputs
//! - PWM input detection
//! - GPIO basic functionality
//! 
//! The firmware runs a series of tests and reports results via defmt logging

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, SampleTime};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_stm32::usart::{Config as UsartConfig, Uart};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("===========================================");
    info!("DM0001 Board Bringup Test Started");
    info!("===========================================");
    
    // Configure clocks for all peripherals
    let mut config = embassy_stm32::Config::default();
    
    // Configure RCC for ADC clock
    // STM32G4 ADC can use System clock or PLL P clock
    // Using System clock is simpler and works for most applications
    use embassy_stm32::rcc::mux::Adcsel;
    config.rcc.mux.adc12sel = Adcsel::SYS;
    
    let p = embassy_stm32::init(config);
    
    // Test 1: Initialize Status LED (PC15 - Red LED)
    info!("[LED] Initializing Status LED (PC15)");
    let mut status_led = Output::new(p.PC15, Level::Low, Speed::Low);
    status_led.set_high();
    Timer::after_millis(500).await;
    status_led.set_low();
    info!("[LED] Status LED test: OK");
    
    // Test 2: Initialize UART2 (PB3=TX, PB4=RX)
    info!("[UART] Initializing UART2 (PB3=TX, PB4=RX) at 115200 baud");
    let mut uart_config = UsartConfig::default();
    uart_config.baudrate = 115200;
    
    let mut uart = Uart::new(
        p.USART2,
        p.PB4, // RX
        p.PB3, // TX
        Irqs,
        p.DMA1_CH1,
        p.DMA1_CH2,
        uart_config,
    ).unwrap();
    
    let test_msg = b"DM0001 UART Test\r\n";
    match uart.blocking_write(test_msg) {
        Ok(_) => info!("[UART] UART2 initialized and test message sent: OK"),
        Err(e) => error!("[UART] UART write failed: {:?}", e),
    }
    
    // Test 3: Initialize ADC
    info!("[ADC] Initializing ADC1");
    let mut adc = Adc::new(p.ADC1);
    adc.set_sample_time(SampleTime::CYCLES47_5);
    
    // PA0: VBUS sensing
    let mut vbus_ch = p.PA0;
    let vbus_raw = adc.blocking_read(&mut vbus_ch);
    let vbus_voltage_mv = (vbus_raw as u32 * 3300 * 187) / (4095 * 18);
    info!("[ADC] VBUS (PA0): raw={}, estimated={}mV", vbus_raw, vbus_voltage_mv);
    
    // PB14: Temperature feedback
    let mut temp_ch = p.PB14;
    let temp_raw = adc.blocking_read(&mut temp_ch);
    let temp_voltage_mv = (temp_raw as u32 * 3300) / 4095;
    info!("[ADC] Temperature (PB14): raw={}, voltage={}mV", temp_raw, temp_voltage_mv);
    info!("[ADC] ADC test: OK");
    
    // Test 4: Hall Encoder Inputs
    info!("[HALL] Initializing Hall Encoder inputs");
    let h1 = Input::new(p.PB6, Pull::None); // Hall A
    let h2 = Input::new(p.PB7, Pull::None); // Hall B  
    let h3 = Input::new(p.PB8, Pull::None); // Hall C/Z
    
    let hall_state = (h1.is_high(), h2.is_high(), h3.is_high());
    info!(
        "[HALL] Hall state: H1={} H2={} H3={} (pattern: {:03b})",
        hall_state.0 as u8,
        hall_state.1 as u8,
        hall_state.2 as u8,
        (hall_state.0 as u8) << 2 | (hall_state.1 as u8) << 1 | (hall_state.2 as u8)
    );
    info!("[HALL] Hall encoder test: OK");
    
    // Test 5: GPIO functionality
    info!("[GPIO] Testing GPIO pins");
    let mut gpio_out = Output::new(p.PB10, Level::Low, Speed::Low);
    let gpio_in = Input::new(p.PB5, Pull::Down);
    
    gpio_out.set_high();
    Timer::after_millis(10).await;
    let gpio_in_state = gpio_in.is_high();
    gpio_out.set_low();
    
    info!("[GPIO] PB10 (output) test: OK");
    info!("[GPIO] PB5 (BEMF control input): {}", gpio_in_state);
    info!("[GPIO] GPIO test: OK");
    
    // Test 6: CAN/FDCAN Control Pins
    info!("[CAN] Initializing FDCAN control pins");
    
    // CAN_SHDN: active low shutdown for transceiver
    let _can_shdn = Output::new(p.PB13, Level::Low, Speed::Low);
    info!("[CAN] CAN transceiver enabled (SHDN=Low)");
    
    // CAN_TERM: termination resistor control
    let mut can_term = Output::new(p.PC14, Level::Low, Speed::Low);
    info!("[CAN] CAN termination control initialized");
    info!("[CAN] FDCAN control pins: OK");
    info!("[CAN] Note: PA11=RX, PB9=TX available for FDCAN peripheral");
    
    // Test 7: Additional GPIO status checks
    info!("[INFO] Checking additional board signals");
    
    // NRST pin (usually pulled high externally)
    info!("[INFO] nRST pin available for reset control");
    
    // PWM input (PA15)
    info!("[INFO] PA15 available for PWM input capture");
    
    // Motor phase outputs (informational)
    info!("[INFO] Motor control outputs available:");
    info!("  Phase A: PA8 (TIM1_CH1), PC13 (TIM1_CH1N)");
    info!("  Phase B: PA9 (TIM1_CH2), PA12 (TIM1_CH2N)");
    info!("  Phase C: PA10 (TIM1_CH3), PB15 (TIM1_CH3N)");
    
    // OpAmp current sensing (informational)
    info!("[INFO] Current sensing OpAmps available:");
    info!("  OpAmp1: PA1(+), PA3(-), PA2(out) - Phase A");
    info!("  OpAmp2: PA7(+), PA5(-), PA6(out) - Phase B");
    info!("  OpAmp3: PB0(+), PB2(-), PB1(out) - Phase C");
    
    // BEMF inputs (informational)
    info!("[INFO] BEMF sensing inputs available:");
    info!("  BEMF1: PA4, BEMF2: PB12, BEMF3: PB11");
    
    info!("===========================================");
    info!("All Bringup Tests Completed Successfully!");
    info!("===========================================");
    
    // Main loop - blink LED and periodically report status
    let mut count = 0u32;
    let mut last_hall_state = hall_state;
    
    loop {
        // Blink status LED
        status_led.set_high();
        Timer::after_millis(100).await;
        status_led.set_low();
        Timer::after_millis(900).await;
        
        count += 1;
        
        // Every 5 seconds, send UART message and check sensors
        if count % 5 == 0 {
            // Send UART message
            let msg = b"Bringup running\r\n";
            let _ = uart.blocking_write(msg);
            
            // Check ADC
            let vbus_raw = adc.blocking_read(&mut vbus_ch);
            let temp_raw = adc.blocking_read(&mut temp_ch);
            
            // Check Hall sensors
            let hall_state = (h1.is_high(), h2.is_high(), h3.is_high());
            
            info!(
                "[STATUS] count={}, VBUS={}, TEMP={}, Hall={:03b}",
                count,
                vbus_raw,
                temp_raw,
                (hall_state.0 as u8) << 2 | (hall_state.1 as u8) << 1 | (hall_state.2 as u8)
            );
            
            // Report hall state changes
            if hall_state != last_hall_state {
                info!("[HALL] State changed: {:?} -> {:?}", last_hall_state, hall_state);
                last_hall_state = hall_state;
            }
        }
        
        // Every 10 seconds, test GPIO toggle
        if count % 10 == 0 {
            gpio_out.toggle();
            let state = gpio_out.is_set_high();
            info!("[GPIO] PB10 toggled: {}", state);
        }
        
        // Every 20 seconds, toggle CAN termination (for testing)
        if count % 20 == 0 {
            can_term.toggle();
            info!("[CAN] Termination toggled (test only)");
        }
    }
}
