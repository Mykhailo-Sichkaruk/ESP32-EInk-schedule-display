use chrono::Duration;
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
use std::thread;

fn main() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 800));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();
    let day_after_tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 15, 0).unwrap()),
            "Morning Meeting",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap()),
            "Team Sync",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap(),
            ),
            "Review",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
            ),
            "Workshop",
        )?,
    ];

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Animated Schedule", &output_settings);

    let start =
        chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let end =
        chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(23, 0, 0).unwrap());

    let mut current_time = start;
    while current_time <= end {
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
            12,
        )?
        .draw(&mut display)?;

        window.update(&display);

        for event in window.events() {
            if event == embedded_graphics_simulator::SimulatorEvent::Quit {
                return Ok(());
            }
        }

        thread::sleep(std::time::Duration::from_millis(50));
        current_time = current_time + Duration::minutes(15);
    }

    window.show_static(&display);
    Ok(())
}
