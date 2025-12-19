use std::ops::RangeInclusive;

use chrono::Duration;
use embedded_graphics::prelude::*;

use embedded_graphics_components::schedule_table_style::{Palette, ScheduleTableStyleBuilder};
use epd_waveshare::color::Color as DuoColor;
use epd_waveshare::color::TriColor;
#[cfg(feature = "wokwi")]
use epd_waveshare::epd2in9_v2::{Display2in9 as Display, Epd2in9 as Epd};
#[cfg(not(feature = "wokwi"))]
use epd_waveshare::epd7in5b_v3::{Display7in5 as Display, Epd7in5 as Epd};
use epd_waveshare::prelude::WaveshareDisplay;

use embedded_graphics_components::schedule_table::ScheduleTable;
use esp_backtrace as _;
use esp_eink_schedule::epd_pins::{self, EpdHardwarePins};
use esp_eink_schedule::schedule_api;
use esp_eink_schedule::wifilib;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::{self, PinDriver};
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use log::info;

#[cfg(not(feature = "wokwi"))]
const hours_to_show: u32 = 12;
#[cfg(feature = "wokwi")]
const hours_to_show: u32 = 5;

fn main() -> anyhow::Result<()> {
    #[cfg(not(feature = "wokwi"))]
    let palette = Palette::new(TriColor::Black, TriColor::White, TriColor::Chromatic);
    #[cfg(feature = "wokwi")]
    let palette = Palette::new(DuoColor::Black, DuoColor::White, DuoColor::Black);

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let nvs = EspDefaultNvsPartition::take()?;

    info!("Starting EPD example");

    let (epd_pins, net) = epd_pins::get_pins()?;
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
    let result = wifilib::request_update(net, nvs)?;
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

    // Get display dimensions for calculations
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let schedule = schedule_api::parse_schedule(&result)?;
    let time_intervals = schedule.time_intervals();
    let current_time = schedule.current_time;

    ScheduleTable::new(
        Point::new(0, 0),
        Size::new(display_width, display_height),
        ScheduleTableStyleBuilder::new(palette).build(),
        current_time,
        &time_intervals,
        hours_to_show,
    )?
    .draw(display.as_mut())?;

    epd.update_and_display_frame(&mut spidd, display.buffer(), &mut delay)?;
    info!("Frame updated and displayed");
    delay.delay_ms(1000);

    epd.sleep(&mut spidd, &mut delay)?;

    const SLEEP_SECS: u64 = 30;
    info!("Going to deep sleep for {SLEEP_SECS} seconds...");

    unsafe {
        esp_idf_sys::esp_sleep_enable_timer_wakeup(SLEEP_SECS * 1_000_000);
        esp_idf_sys::esp_deep_sleep_start();
    }

    Ok(())
}

