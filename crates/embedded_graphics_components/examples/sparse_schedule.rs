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

    // Sparse schedule - only a few events
    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap()),
            "Quick Standup",
        ),
        TimeInterval::new(
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(tomorrow, chrono::NaiveTime::from_hms_opt(15, 30, 0).unwrap()),
            "1-on-1 Meeting",
        ),
    ];

    let current_time = chrono::NaiveDateTime::new(
        today,
        chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap()
    );

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
        10,
    )?
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Sparse Schedule", &output_settings);
    window.show_static(&display);

    Ok(())
}
