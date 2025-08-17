use embedded_graphics::prelude::*;

use embedded_graphics_components::unified_color::UnifiedColor;
use epd_waveshare::color::TriColor;
#[cfg(feature = "wokwi")]
use epd_waveshare::epd2in9_v2::{Display2in9 as Display, Epd2in9 as Epd};
#[cfg(not(feature = "wokwi"))]
use epd_waveshare::epd7in5b_v3::{Display7in5 as Display, Epd7in5 as Epd};
use epd_waveshare::prelude::WaveshareDisplay;

use embedded_graphics_components::battery_indicator::BatteryIndicator;
use embedded_graphics_components::schedule_table::ScheduleTable;
use esp_backtrace as _;
use esp_eink_schedule::epd_pins::{self, EpdHardwarePins};
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::{self, PinDriver};
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use log::info;

fn unif_color_converter(color: UnifiedColor) -> TriColor {
    match color {
        UnifiedColor::Black => TriColor::Black,
        UnifiedColor::White => TriColor::White,
        UnifiedColor::Chromatic => TriColor::Chromatic,
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

    // --- ScheduleTable parameters ---
    let header_height = 40;
    let time_col_width = 80;
    let num_date_cols = 3;
    // Number of data rows: this now controls how many rows are allocated visually.
    // Ensure this value is chosen such that `(display_height - header_height)` is divisible by `num_data_rows`
    // to avoid rounding issues if you want perfect pixel alignment.
    // For a 128px height display with 40px header, you have 88px left.
    // 88 / 12 = 7.33, so 12 is problematic.
    // Let's re-evaluate for clean division: if display_height is 128 and header_height is 40,
    // we have 88px for rows.
    // If you want to show, say, 11 hours (6 to 17), that's 11 rows. 88 / 11 = 8px per hour.
    // If you want to show 12 hours (6 to 18), 88 / 12 = 7.33. Let's stick with 12 if you desire that range,
    // and accept potential rounding that `embedded-graphics` handles.
    // Or adjust range, or adjust header_height/total_height to make it divisible.
    // For simplicity with given values, we'll keep num_data_rows = 12 as per original table height scaling.
    let num_data_rows = 12;

    let battery_bar_height: u32 = 5; // Высота полосы батареи внизу

    let y_pos_offset = 10;
    let nowline_time = 13.5;

    let header_texts = ["Time", "01.01.2025", "02.01.2025", "03.01.2025"];
    let time_range = 6..=18; // From 6:00 to 18:00, which is 13 hours/rows effectively

    let time_intervals = [
        ("01.01.2025", 6.0, 12.25, "xsichkaruk"),
        ("01.01.2025", 12.5, 14.0, "xchaban"),
        ("01.01.2025", 14.5, 17.0, "xchaban"),
        ("02.01.2025", 10.25, 10.75, "xchaban"),
        ("02.01.2025", 11.5, 13.25, "xtodorov"),
        ("02.01.2025", 13.5, 15.0, "xchaban"),
        ("03.01.2025", 10.0, 12.0, "xchaban"),
        ("03.01.2025", 12.25, 14.5, "xchaban"),
        ("03.01.2025", 15.0, 16.0, "xchaban"),
        ("01.01.2025", 17.0, 17.25, "xchaban"),
        ("02.01.2025", 17.0, 17.5, "xchaban"),
        ("03.01.2025", 17.0, 18.00, "xchaban"),
    ];

    ScheduleTable::new(
        Point::new(0, battery_bar_height as i32), // Table starts at top-left of the display
        Size::new(display_width, display_height - battery_bar_height), // Table occupies full display
        header_height,
        time_col_width,
        num_date_cols,
        num_data_rows, // Use dynamic row count
        y_pos_offset,
        nowline_time,
        header_texts,
        time_range,
        time_intervals,
        unif_color_converter,
    )
    .draw(display.as_mut())?;

    let battery_level_percent = 19; // Example battery level

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
