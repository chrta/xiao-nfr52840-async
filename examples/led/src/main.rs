#![no_std]
#![no_main]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use xiao_nrf52840_async::*;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

#[embassy_executor::task(pool_size = 3)]
async fn blinker(mut led: Led, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = XiaoNrf52840::default();

    let led_red = board.led_red;
    let led_green = board.led_green;
    let led_blue = board.led_blue;

    spawner
        .spawn(blinker(led_red, Duration::from_millis(700)))
        .unwrap();
    spawner
        .spawn(blinker(led_green, Duration::from_millis(900)))
        .unwrap();
    spawner
        .spawn(blinker(led_blue, Duration::from_millis(1200)))
        .unwrap();
}
