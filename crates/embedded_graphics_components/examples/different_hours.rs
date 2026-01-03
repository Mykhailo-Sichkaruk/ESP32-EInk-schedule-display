use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, Point, Size},
};
use embedded_graphics_components::{
    schedule_table::ScheduleTable,
    schedule_table_style::{Palette, ScheduleTableStyleBuilder},
    time_interval::TimeInterval,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> anyhow::Result<()> {
    // Example 1: Short view - 6 hours
    example_short_view()?;

    // Example 2: Medium view - 12 hours (default)
    example_medium_view()?;

    // Example 3: Long view - 18 hours
    example_long_view()?;

    Ok(())
}

fn example_short_view() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 800));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap()),
            "Meeting 1",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
            "Meeting 2",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(13, 30, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
            "Meeting 3",
        )?,
    ];

    let current_time =
        chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap());

    ScheduleTable::new(
        Point::new(40, 40),
        Size::new(display_width - 80, display_height - 80),
        ScheduleTableStyleBuilder::new(Palette::new(
            Rgb565::new(0, 0, 0),
            Rgb565::new(255, 255, 255),
            Rgb565::new(255, 0, 0),
        ))
        .build(),
        current_time,
        &time_intervals,
        6, // Show only 6 hours
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Short View - 6 Hours", &output_settings);
    window.show_static(&display);

    Ok(())
}

fn example_medium_view() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 800));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap()),
            "Morning Block",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
            "Midday Block",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap()),
            "Afternoon Block",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
            ),
            "Tomorrow",
        )?,
    ];

    let current_time =
        chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap());

    ScheduleTable::new(
        Point::new(40, 40),
        Size::new(display_width - 80, display_height - 80),
        ScheduleTableStyleBuilder::new(Palette::new(
            Rgb565::new(0, 0, 0),
            Rgb565::new(255, 255, 255),
            Rgb565::new(255, 0, 0),
        ))
        .build(),
        current_time,
        &time_intervals,
        12, // Show 12 hours (default)
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Medium View - 12 Hours", &output_settings);
    window.show_static(&display);

    Ok(())
}

fn example_long_view() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(800, 480));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
            "Early Morning",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
            "Late Morning",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap()),
            "Afternoon",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(18, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(21, 0, 0).unwrap()),
            "Evening",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            "Next Day",
        )?,
    ];

    let current_time =
        chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap());

    ScheduleTable::new(
        Point::new(40, 40),
        Size::new(display_width - 80, display_height - 40),
        ScheduleTableStyleBuilder::new(Palette::new(
            Rgb565::new(0, 0, 0),
            Rgb565::new(255, 255, 255),
            Rgb565::new(255, 0, 0),
        ))
        .build(),
        current_time,
        &time_intervals,
        18, // Show 18 hours - extended view
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Long View - 18 Hours", &output_settings);
    window.show_static(&display);

    Ok(())
}
