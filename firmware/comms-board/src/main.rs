//! Comms board application.

#![no_std]
#![no_main]

use communication::to_slice_cobs_crc32;
use communication::tool_messages::{TestPing, ToolTestMessage};
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut control = comms_board::init(spawner).await;

    let mut counter: u8 = 0;
    loop {
        let mut buf = [0u8; 32];
        let msg = ToolTestMessage::from(TestPing::Ping(counter));
        let bytes = to_slice_cobs_crc32(&msg, &mut buf).expect("buffer fits message");
        info!("sending {=[u8]:x}", bytes);

        control.gpio_set(0, true).await;
        Timer::after_millis(100).await;
        control.gpio_set(0, false).await;
        Timer::after_millis(900).await;

        counter = counter.wrapping_add(1);
    }
}
