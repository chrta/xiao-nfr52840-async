#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::{config, interrupt, peripherals::USBD, usb, usb::PowerUsb};
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::task]
async fn logger_task(driver: usb::Driver<'static, USBD, PowerUsb>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config: config::Config = Default::default();
    config.hfclk_source = config::HfclkSource::ExternalXtal;

    let p = embassy_nrf::init(config);

    let irq = interrupt::take!(USBD);
    let power_irq = interrupt::take!(POWER_CLOCK);
    let driver = usb::Driver::new(p.USBD, irq, PowerUsb::new(power_irq));

    spawner.spawn(logger_task(driver)).unwrap();

    let mut counter = 0;
    loop {
        counter += 1;
        log::info!("Tick {}", counter);
        Timer::after(Duration::from_secs(1)).await;
    }
}
