use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, Point, Size},
};
use embedded_graphics_components::{
    battery_indicator::BatteryIndicator,
    schedule_table::ScheduleTable,
    unified_color::{IntoPixelColorConverter, UnifiedColor},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

struct Converter;

impl IntoPixelColorConverter for Converter {
    type Output = Rgb565;

    fn convert(color: UnifiedColor) -> Self::Output {
        match color {
            UnifiedColor::Black => Rgb565::new(0, 0, 0),
            UnifiedColor::White => Rgb565::new(255, 255, 255),
            UnifiedColor::Chromatic => Rgb565::new(255, 0, 0),
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Create a simulator display
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 800));

    // // Simulate the drawing process
    // display.set_rotation(epd_waveshare::prelude::DisplayRotation::Rotate90);

    // Get display dimensions for calculations
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    // --- ScheduleTable parameters ---
    let header_height = 40;
    let time_col_width = 80;
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

    let battery_bar_height: u32 = 10; // Высота полосы батареи внизу

    let y_pos_offset = 10;
    let nowline_time = 13.5;

    let header_texts = ["01.01.2025", "02.01.2025", "03.01.2025"];
    let time_range = 6..=17; // From 6:00 to 18:00, which is 13 hours/rows effectively

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

    ScheduleTable::<Converter>::new(
        Point::new(0, battery_bar_height as i32), // Table starts at top-left of the display
        Size::new(display_width, display_height - battery_bar_height), // Table occupies full display
        header_height,
        time_col_width,
        y_pos_offset,
        nowline_time,
        &header_texts,
        time_range,
        &time_intervals,
    )
    .draw(&mut display)?;

    let battery_level_percent = 19;

    BatteryIndicator::<Converter>::new(
        Point::new(0, 0),
        Size::new(display_width, battery_bar_height),
    )
    .draw(&mut display, battery_level_percent)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
