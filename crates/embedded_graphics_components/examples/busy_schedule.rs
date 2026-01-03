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
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(800, 480));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();
    let day_after = chrono::NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();

    // Very busy schedule with many overlapping events
    let time_intervals = vec![
        // Today - packed schedule
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
            "Early Standup",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(8, 15, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(9, 45, 0).unwrap()),
            "Design Review",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap()),
            "Sprint Planning",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 45, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 15, 0).unwrap()),
            "Quick Sync",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
            "Tech Talk",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(14, 45, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap()),
            "Client Demo",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(16, 15, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(17, 30, 0).unwrap()),
            "Team Retro",
        ),
        // Tomorrow
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            "Standup",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(9, 15, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(10, 45, 0).unwrap()),
            "Code Review",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(12, 30, 0).unwrap()),
            "Architecture",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(13, 30, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(15, 0, 0).unwrap()),
            "Pair Program",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(15, 15, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap()),
            "Workshop",
        ),
        // Day after
        TimeInterval::new(
            chrono::NaiveDateTime::new(day_after, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(day_after, chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap()),
            "All Hands",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(day_after, chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(day_after, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
            "1-on-1",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(day_after, chrono::NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(day_after, chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
            "Training",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(day_after, chrono::NaiveTime::from_hms_opt(15, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(day_after, chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap()),
            "Deep Work",
        ),
    ];

    let current_time = chrono::NaiveDateTime::new(
        today,
        chrono::NaiveTime::from_hms_opt(12, 30, 0).unwrap()
    );

    ScheduleTable::new(
        Point::new(20, 20),
        Size::new(display_width - 40, display_height - 40),
        ScheduleTableStyleBuilder::new(Palette::new(
            Rgb565::new(0, 0, 0),
            Rgb565::new(255, 255, 255),
            Rgb565::new(255, 0, 0),
        ))
        .build(),
        current_time,
        &time_intervals,
        14,
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Busy Schedule", &output_settings);
    window.show_static(&display);

    Ok(())
}
