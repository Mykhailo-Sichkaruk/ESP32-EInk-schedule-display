use embedded_graphics::image::Image;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Line;
use embedded_graphics::text::{LineHeight, Text};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};
use epd_waveshare::color::TriColor;
#[cfg(feature = "wokwi")]
use epd_waveshare::epd2in9_v2::{Display2in9 as Display, Epd2in9 as Epd};
#[cfg(not(feature = "wokwi"))]
use epd_waveshare::epd7in5b_v3::{Display7in5 as Display, Epd7in5 as Epd};
use epd_waveshare::prelude::WaveshareDisplay;
use esp_backtrace as _;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::AnyOutputPin;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_hal::spi::SPI3;
use log::info;
use tinybmp::Bmp;

use crate::epd_pins::EpdHardwarePins;
// use crate::unified_color::UnifiedColor;

pub fn epd_start_render_text(
    EpdHardwarePins {
        spi,
        sclk,
        mosi,
        cs,
        busy_in,
        rst,
        dc,
        pwr,
    }: EpdHardwarePins,
    text: &str,
    point: Point,
) -> anyhow::Result<()> {
    let mut pwr = PinDriver::output(pwr)?;
    pwr.set_high()?;

    let mut spidd = spi::SpiDeviceDriver::new_single(
        spi,
        sclk,
        mosi,
        Option::<gpio::AnyIOPin>::None,
        Some(cs),
        &spi::config::DriverConfig::new(),
        &spi::config::Config::new().baudrate(115200.Hz()),
    )?;

    let mut delay = Delay::new(100);

    let mut epd = Epd::new(
        &mut spidd,
        PinDriver::input(busy_in)?,
        PinDriver::output(dc)?,
        PinDriver::output(rst)?,
        &mut delay,
        None,
    )?;

    epd.wake_up(&mut spidd, &mut delay)?;

    epd.clear_frame(&mut spidd, &mut delay)?;

    let display = Box::new(Display::default());

    epd.update_and_display_frame(&mut spidd, display.buffer(), &mut delay)
        .expect("Failed to update and display EPD frame");

    info!("Frame updated and displayed");

    delay.delay_ms(1000);
    epd.sleep(&mut spidd, &mut delay)
        .expect("Failed to put EPD to sleep");
    Ok(())
}

pub fn epd_start_render_bmp(
    EpdHardwarePins {
        spi,
        sclk,
        mosi,
        cs,
        busy_in,
        rst,
        dc,
        pwr,
    }: EpdHardwarePins,
) -> anyhow::Result<()> {
    let mut pwr = PinDriver::output(pwr)?;
    pwr.set_high()?;

    let mut spidd = spi::SpiDeviceDriver::new_single(
        spi,
        sclk,
        mosi,
        Option::<gpio::AnyIOPin>::None,
        Some(cs),
        &spi::config::DriverConfig::new(),
        &spi::config::Config::new().baudrate(115200.Hz()),
    )?;

    let mut delay = Delay::new(100);

    let mut epd = Epd::new(
        &mut spidd,
        PinDriver::input(busy_in)?,
        PinDriver::output(dc)?,
        PinDriver::output(rst)?,
        &mut delay,
        None,
    )?;

    epd.wake_up(&mut spidd, &mut delay)?;

    epd.clear_frame(&mut spidd, &mut delay)?;

    let mut display = Box::new(Display::default());

    // display
    //     .clear(UnifiedColor::White.into())
    //     .expect("Failed to clear display buffer");

    // let bmp: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("./assets/rust-pride.bmp")).unwrap();

    // display.draw_iter(bmp.pixels().map(|pixel| {
    //     let point = pixel.0;
    //     let color = pixel.1;

    //     let new_color = UnifiedColor::from_rgb565(color).into();

    //     Pixel(point, new_color)
    // }))?;

    // // image.draw(display.as_mut()).expect("Failed to draw image");

    // epd.update_and_display_frame(&mut spidd, display.buffer(), &mut delay)
    //     .expect("Failed to update and display EPD frame");

    // info!("Frame updated and displayed");

    // delay.delay_ms(1000);
    // epd.sleep(&mut spidd, &mut delay)
    //     .expect("Failed to put EPD to sleep");
    Ok(())
}
