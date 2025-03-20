#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

use rp_pico::hal::rom_data::reset_to_usb_boot;
use rp_pico::hal::{prelude::*, spi};
use rp_pico::hal::pac;
use rp_pico::hal;
use fugit::RateExtU32;
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::pixelcolor::Rgb565;
const XTAL_FREQ_HZ: u32 = 12_000_000u32;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start");
    // let p = embassy_rp::init(Default::default());
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    // let mut led = Output::new(p.PIN_25, Level::Low);
    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    //spi init
    let sio = hal::Sio::new(pac.SIO);
    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    /*
    GND <=> GND
    VCC <=> 3V3
    SCL <=> SCLK(GPIO6)
    SDA <=> MOSI(GPIO7)
    RES <=> RST(GPIO14)
    DC  <=> DC(GPIO13)
    CS  <=> GND
    BLK <=> 不连接
     */
    let spi_sclk = pins.gpio6.into_function::<hal::gpio::FunctionSpi>();
    let spi_mosi = pins.gpio7.into_function::<hal::gpio::FunctionSpi>();
    let spi_miso = pins.gpio4.into_function::<hal::gpio::FunctionSpi>();
    let spi = spi::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    let dc = pins.gpio13.into_push_pull_output();
    let rst = pins.gpio14.into_push_pull_output();

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        64_000_000u32.Hz(),
        &embedded_hal::spi::MODE_0,
    );
    let di = SPIInterfaceNoCS::new(spi, dc);
    let mut display = mipidsi::Builder::st7789(di)
            .with_display_size(240, 240)
            .with_window_offset_handler(|_| (0, 0))
            .with_framebuffer_size(240, 240)
            .with_invert_colors(mipidsi::ColorInversion::Inverted)
            .init(&mut delay, Some(rst))
            .unwrap();
    display.set_pixel(10, 10, Rgb565::new(255, 0, 0));
    info!("display init.");
    loop {
        info!("main!");
        // led.set_high();
        // Timer::after(Duration::from_secs(1)).await;

        // info!("led off!");
        // led.set_low();
        // Timer::after(Duration::from_secs(1)).await;
    }
}
