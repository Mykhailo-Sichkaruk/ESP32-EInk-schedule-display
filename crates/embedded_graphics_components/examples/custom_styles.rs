use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, Point, Size},
};
use embedded_graphics_components::{
    schedule_table::{ScheduleTable, TimeInterval},
    schedule_table_style::{Palette, ScheduleTableStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> anyhow::Result<()> {
    // Style 1: Classic Black & White with Red accents
    example_classic_style()?;
    
    // Style 2: Dark theme with blue accents
    example_dark_theme()?;
    
    // Style 3: Colorful theme
    example_colorful_theme()?;

    Ok(())
}

fn example_classic_style() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 800));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            "Morning Work",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap()),
            "Afternoon",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
            "Planning",
        ),
    ];

    let current_time = chrono::NaiveDateTime::new(
        today,
        chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap()
    );

    ScheduleTable::new(
        Point::new(40, 40),
        Size::new(display_width - 80, display_height - 80),
        ScheduleTableStyleBuilder::new(Palette::new(
            Rgb565::new(0, 0, 0),       // Black
            Rgb565::new(255, 255, 255), // White
            Rgb565::new(255, 0, 0),     // Red
        ))
        .build(),
        current_time,
        &time_intervals,
        12,
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Classic Style", &output_settings);
    window.show_static(&display);

    Ok(())
}

fn example_dark_theme() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 800));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            "Task A",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(15, 30, 0).unwrap()),
            "Task B",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap()),
            "Task C",
        ),
    ];

    let current_time = chrono::NaiveDateTime::new(
        today,
        chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()
    );

    ScheduleTable::new(
        Point::new(40, 40),
        Size::new(display_width - 80, display_height - 80),
        ScheduleTableStyleBuilder::new(Palette::new(
            Rgb565::new(200, 220, 255), // Light blue text
            Rgb565::new(20, 20, 40),    // Dark blue background
            Rgb565::new(0, 150, 255),   // Bright blue accent
        ))
        .build(),
        current_time,
        &time_intervals,
        10,
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Dark Theme", &output_settings);
    window.show_static(&display);

    Ok(())
}

fn example_colorful_theme() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 800));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap()),
            "Exercise",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
            "Study",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(15, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap()),
            "Hobby",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
            "Project",
        ),
    ];

    let current_time = chrono::NaiveDateTime::new(
        today,
        chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap()
    );

    ScheduleTable::new(
        Point::new(40, 40),
        Size::new(display_width - 80, display_height - 80),
        ScheduleTableStyleBuilder::new(Palette::new(
            Rgb565::new(50, 50, 50),      // Dark gray text
            Rgb565::new(255, 250, 240),   // Warm white background
            Rgb565::new(255, 100, 50),    // Orange accent
        ))
        .build(),
        current_time,
        &time_intervals,
        12,
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Colorful Theme", &output_settings);
    window.show_static(&display);

    Ok(())
}
