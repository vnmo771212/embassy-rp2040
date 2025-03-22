#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
// use {defmt_rtt as _, panic_probe as _};
use panic_probe as _;
use embassy_rp::watchdog::Watchdog;
use embassy_rp::uart;
use core::fmt::Write;
use embassy_rp::uart::Blocking;

// 定义全局日志记录器
use embassy_rp::peripherals::UART0;

struct UartLogger {
    uart: uart::Uart<
        'static,     // 生命周期参数
        UART0,       // 外设类型参数（必须与初始化时使用的外设一致）
        Blocking     // 工作模式参数
    >,
}

impl Write for UartLogger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.uart.blocking_write(s.as_bytes()).map_err(|_| core::fmt::Error)?;
        Ok(())
    }
}

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
    //串口初始化
    let mut config = uart::Config::default();
    config.baudrate = 115200;
    let mut uart0_G = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);
    uart0_G.blocking_write("UART0 INIT\r\n".as_bytes()).unwrap();
    let mut logger = UartLogger{ uart: uart0_G};
    // logger.write_str("System Ready\r\n");
    // write!(&mut logger, "System Ready\r\n").unwrap();
    loop {
        info!("led on!");
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;

        info!("led off!");
        led.set_low();
        Timer::after(Duration::from_secs(1)).await;
    }
}
