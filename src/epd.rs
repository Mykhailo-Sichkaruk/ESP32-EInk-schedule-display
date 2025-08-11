use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder};
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{LineHeight, Text};
use embedded_graphics::{prelude::*, primitives::Rectangle};
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};
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

use crate::epd_pins::EpdHardwarePins;
use crate::unified_color::UnifiedColor;

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
    text: String,
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

    display
        .clear(UnifiedColor::White.to_color())
        .expect("Failed to clear display buffer");

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(UnifiedColor::Chromatic.to_color())
        .build();
    let bounds = Rectangle::new(Point::new(10, 20), Size::new(760, 440));
    let tb_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Left) // Left/Center/Right/Justified
        .line_height(LineHeight::Percent(110)) // tweak spacing
        .build();
    TextBox::with_textbox_style(&text, bounds, text_style, tb_style)
        .draw(display.as_mut())
        .expect("textbox draw");

    epd.update_and_display_frame(&mut spidd, display.buffer(), &mut delay)
        .expect("Failed to update and display EPD frame");

    info!("Frame updated and displayed");

    delay.delay_ms(1000);
    epd.sleep(&mut spidd, &mut delay)
        .expect("Failed to put EPD to sleep");
    Ok(())
}
