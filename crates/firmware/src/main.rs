//! Comms board application.

#![no_std]
#![no_main]

use communication::Message;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut control = firmware::init(spawner).await;

    let mut counter: u8 = 0;
    loop {
        let bytes = Message { counter }.pack();
        info!("sending {=[u8]:x}", bytes);

        control.gpio_set(0, true).await;
        Timer::after_millis(100).await;
        control.gpio_set(0, false).await;
        Timer::after_millis(900).await;

        counter = counter.wrapping_add(1);
    }
}
