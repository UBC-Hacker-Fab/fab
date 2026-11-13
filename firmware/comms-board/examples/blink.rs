//! Blinks the Pico 2 W LED.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut control = comms_board::init(spawner).await;

    loop {
        control.gpio_set(0, true).await;
        Timer::after_millis(250).await;
        control.gpio_set(0, false).await;
        Timer::after_millis(250).await;
    }
}
