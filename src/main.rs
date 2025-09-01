use embedded_graphics::prelude::*;

use embedded_graphics_components::schedule_table_style::{Palette, ScheduleTableStyleBuilder};
use embedded_graphics_components::unified_color::{IntoPixelColorConverter, UnifiedColor};
use epd_waveshare::color::TriColor;
#[cfg(feature = "wokwi")]
use epd_waveshare::epd2in9_v2::{Display2in9 as Display, Epd2in9 as Epd};
#[cfg(not(feature = "wokwi"))]
use epd_waveshare::epd7in5b_v3::{Display7in5 as Display, Epd7in5 as Epd};
use epd_waveshare::prelude::WaveshareDisplay;

use embedded_graphics_components::battery_indicator::BatteryIndicator;
use embedded_graphics_components::schedule_table::{ScheduleTable, TimeInterval};
use esp_backtrace as _;
use esp_eink_schedule::epd_pins::{self, EpdHardwarePins};
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::{self, PinDriver};
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use log::info;

struct Converter;

impl IntoPixelColorConverter for Converter {
    type Output = TriColor;

    fn convert(color: UnifiedColor) -> Self::Output {
        match color {
            UnifiedColor::Black => TriColor::Black,
            UnifiedColor::White => TriColor::White,
            UnifiedColor::Chromatic => TriColor::Chromatic,
        }
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let _nvs = EspDefaultNvsPartition::take()?;

    info!("Starting EPD example");

    let (epd_pins, _net) = epd_pins::get_pins()?;

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

    // Get display dimensions for calculations
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();
    let day_after_tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 15, 0).unwrap()),
            "xsichkaruk",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 30, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap()),
            "xchaban",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(10, 15, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(10, 45, 0).unwrap(),
            ),
            "xchaban",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(13, 15, 0).unwrap(),
            ),
            "xtodorov",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(13, 30, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
            ),
            "xchaban",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                day_after_tomorrow,
                chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                day_after_tomorrow,
                chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ),
            "xchaban",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                day_after_tomorrow,
                chrono::NaiveTime::from_hms_opt(12, 15, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                day_after_tomorrow,
                chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            ),
            "xchaban",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                day_after_tomorrow,
                chrono::NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                day_after_tomorrow,
                chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            ),
            "xchaban",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(17, 15, 0).unwrap()),
            "xchaban",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(22, 15, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap()),
            "xchaban",
        ),
    ];

    let current_time =
        chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());

    ScheduleTable::new(
        Point::new(0, 0),
        Size::new(display_width, display_height),
        ScheduleTableStyleBuilder::new(Palette::new(
            TriColor::Black,
            TriColor::White,
            TriColor::Chromatic,
        ))
        .build(),
        current_time,
        &time_intervals,
        12,
    )?
    .draw(display.as_mut())?;

    // // Draw battery indicator at the very bottom
    // BatteryIndicator::new(
    //     Point::new(0, 0),
    //     Size::new(display_width, battery_bar_height),
    // )
    // .draw(display.as_mut(), battery_level_percent)?;

    epd.update_and_display_frame(&mut spidd, display.buffer(), &mut delay)?;

    info!("Frame updated and displayed");

    delay.delay_ms(1000);
    epd.sleep(&mut spidd, &mut delay)?;

    Ok(())
}
