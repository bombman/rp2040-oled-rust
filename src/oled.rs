#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate embedded_graphics;
extern crate fugit;
extern crate panic_halt;
extern crate rp2040_hal as hal;
extern crate ssd1306;

use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use fugit::RateExtU32;
use hal::pac;
use hal::prelude::*;
use hal::I2C;
use rt::entry;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

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

    // Setup delay
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Set up the I2C pins and I2C driver
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin = pins.gpio4.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio5.into_mode::<hal::gpio::FunctionI2C>();

    let i2c = I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400_u32.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    // Set up the SSD1306 driver
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    // Clear the display
    display.clear();

    // Create a text style
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    // Calculate the starting position to center the text
    let text = "Hello Rust";
    let width = 6 * text.len() as i32;
    let height = 10;
    let x = (128 - width) / 2;
    let y = (64 - height) / 2;

    // Display the text
    Text::new(text, Point::new(x, y), text_style)
        .draw(&mut display)
        .unwrap();

    // Flush the display to show the text
    display.flush().unwrap();

    // Infinite loop to keep the program running
    loop {
        delay.delay_ms(1000);
    }
}
