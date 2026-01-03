use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::prelude::{Dimensions, Point, Size};
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics_components::error_banner::ErrorBanner;
use embedded_graphics_components::schedule_table_style::{Palette, ScheduleTableStyleBuilder};
#[cfg(not(feature = "wokwi"))]
use epd_waveshare::color::TriColor;
use epd_waveshare::prelude::WaveshareDisplay;

use embedded_graphics_components::schedule_table::ScheduleTable;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::{self};
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use log::info;

#[cfg(feature = "wokwi")]
use epd_waveshare::color::Color as DuoColor;
#[cfg(feature = "wokwi")]
use epd_waveshare::epd2in9_v2::{Display2in9 as Display, Epd2in9 as Epd};
#[cfg(not(feature = "wokwi"))]
use epd_waveshare::epd7in5b_v3::{Display7in5 as Display, Epd7in5 as Epd};

#[cfg(not(feature = "wokwi"))]
const HOURS_TO_SHOW: u32 = 12;
#[cfg(feature = "wokwi")]
const HOURS_TO_SHOW: u32 = 5;

const ERROR_LINES: i32 = 2;
const ERROR_PADDING: i32 = 4;
const ERROR_HELP_TEXT: &str = "Please contact Ynet members";

use crate::esp_resource::EpdHardwarePins;
use crate::schedule_api::ParsedSchedule;

pub fn render_schedule(epd_pins: EpdHardwarePins, schedule: ParsedSchedule) -> anyhow::Result<()> {
    #[cfg(not(feature = "wokwi"))]
    let palette = Palette::new(TriColor::Black, TriColor::White, TriColor::Chromatic);
    #[cfg(feature = "wokwi")]
    let palette = Palette::new(DuoColor::Black, DuoColor::White, DuoColor::Black);

    let EpdHardwarePins {
        spi,
        sclk,
        mosi,
        cs,
        busy_in,
        rst,
        dc,
        pwr,
    }: EpdHardwarePins = epd_pins;
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

    let mut display = Box::new(Display::default());
    display.set_rotation(epd_waveshare::prelude::DisplayRotation::Rotate90);

    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let time_intervals = schedule.time_intervals();
    let current_time = schedule.current_time;

    ScheduleTable::new(
        Point::new(0, 0),
        Size::new(display_width, display_height),
        ScheduleTableStyleBuilder::new(palette).build(),
        current_time,
        &time_intervals,
        HOURS_TO_SHOW,
    )?
    .draw(display.as_mut())?;

    epd.update_and_display_frame(&mut spidd, display.buffer(), &mut delay)?;
    info!("Frame updated and displayed");
    delay.delay_ms(1000);
    epd.sleep(&mut spidd, &mut delay)?;
    Ok(())
}

pub fn render_error(epd_pins: EpdHardwarePins, message: &str) -> anyhow::Result<()> {
    #[cfg(not(feature = "wokwi"))]
    let palette = Palette::new(TriColor::Black, TriColor::White, TriColor::Chromatic);
    #[cfg(feature = "wokwi")]
    let palette = Palette::new(DuoColor::Black, DuoColor::White, DuoColor::Black);

    let EpdHardwarePins {
        spi,
        sclk,
        mosi,
        cs,
        busy_in,
        rst,
        dc,
        pwr,
    }: EpdHardwarePins = epd_pins;
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

    let mut display = Box::new(Display::default());
    display.set_rotation(epd_waveshare::prelude::DisplayRotation::Rotate90);

    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(palette.accent)
        .build();
    let line_height = text_style.font.character_size.height as i32;
    let banner_height = (line_height * ERROR_LINES + ERROR_PADDING * 2) as u32;
    let banner_top_left = Point::new(0, display_height as i32 - banner_height as i32);
    let banner_size = Size::new(display_width, banner_height);

    let max_chars = (banner_size.width as usize)
        .saturating_div(text_style.font.character_size.width as usize);
    let line1 = truncate_line(message, max_chars);
    let line2 = truncate_line(ERROR_HELP_TEXT, max_chars);

    let banner = ErrorBanner::new(
        banner_top_left,
        banner_size,
        text_style,
        PrimitiveStyleBuilder::new()
            .fill_color(palette.secondary)
            .build(),
        ERROR_PADDING,
    );
    banner.draw(display.as_mut(), &line1, &line2)?;

    epd.update_and_display_frame(&mut spidd, display.buffer(), &mut delay)?;
    info!("Error frame updated and displayed");
    delay.delay_ms(1000);
    epd.sleep(&mut spidd, &mut delay)?;
    Ok(())
}

fn truncate_line(line: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    if line.chars().count() <= max_chars {
        return line.to_string();
    }
    if max_chars <= 3 {
        return line.chars().take(max_chars).collect();
    }
    let mut out = String::with_capacity(max_chars);
    for ch in line.chars().take(max_chars - 3) {
        out.push(ch);
    }
    out.push_str("...");
    out
}
