use embedded_graphics::prelude::{Dimensions, Point, Size};
#[cfg(not(feature = "wokwi"))]
use embedded_graphics_components::schedule_table_style::Palette;
use embedded_graphics_components::schedule_table_style::ScheduleTableStyleBuilder;
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
use epd_waveshare::epd2in9_v2::{Display2in9 as Display, Epd2in9 as Epd};
#[cfg(not(feature = "wokwi"))]
use epd_waveshare::epd7in5b_v3::{Display7in5 as Display, Epd7in5 as Epd};

#[cfg(not(feature = "wokwi"))]
const HOURS_TO_SHOW: u32 = 12;
#[cfg(feature = "wokwi")]
const HOURS_TO_SHOW: u32 = 5;

use crate::esp_resource::EpdHardwarePins;
use crate::schedule_api::{self, ParsedSchedule};

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
