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

    let style = ScheduleTableStyleBuilder::new(Palette::new(
        Rgb565::new(0, 0, 0),
        Rgb565::new(255, 255, 255),
        Rgb565::new(255, 0, 0),
    ))
    .build();

    let current_time =
        chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 30, 0).unwrap());

    ScheduleTable::new(
        Point::new(40, 40),
        Size::new(display_width - 80, display_height - 80),
        style,
        current_time,
        &time_intervals,
        12,
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Schedule Demo", &output_settings);
    window.show_static(&display);

    Ok(())
}
