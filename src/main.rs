#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};
use embassy_rp::watchdog::Watchdog;


#[embassy_executor::task]
async fn T_WatchdogFree(mut wdog: Watchdog) {
    loop {
        wdog.feed();
        Timer::after(Duration::from_secs(3)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program start");
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);
    //初始化看门狗
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    watchdog.start(Duration::from_secs(8));
    spawner.spawn(T_WatchdogFree(watchdog)).unwrap();
    loop {
        info!("led on!");
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;

        info!("led off!");
        led.set_low();
        Timer::after(Duration::from_secs(1)).await;
    }
}
