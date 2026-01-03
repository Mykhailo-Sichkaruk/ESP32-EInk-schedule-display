use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::Rgb565,
    prelude::{Dimensions, Point, Size},
    primitives::PrimitiveStyleBuilder,
};
use embedded_graphics_components::{
    error_banner::ErrorBanner,
    schedule_table::ScheduleTable,
    schedule_table_style::{Palette, ScheduleTableStyleBuilder},
    time_interval::TimeInterval,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(800, 600));
    let display_width = display.bounding_box().size.width;
    let display_height = display.bounding_box().size.height;

    let today = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

    let time_intervals = vec![
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            "Standup",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap()),
            chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap()),
            "Review",
        )?,
        TimeInterval::new(
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            ),
            chrono::NaiveDateTime::new(
                tomorrow,
                chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ),
            "Planning",
        )?,
    ];

    let current_time =
        chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap());

    // Draw schedule in top half
    let schedule_height = display_height / 2 - 20;
    match ScheduleTable::new(
        Point::new(20, 20),
        Size::new(display_width - 40, schedule_height),
        ScheduleTableStyleBuilder::new(Palette::new(
            Rgb565::new(0, 0, 0),
            Rgb565::new(255, 255, 255),
            Rgb565::new(255, 0, 0),
        ))
        .build(),
        current_time,
        &time_intervals,
        10,
    ) {
        Ok(table) => {
            table.draw(&mut display)?;
        }
        Err(e) => {
            // If schedule fails, show error banner instead
            ErrorBanner::new(
                Point::new(20, 20),
                Size::new(display_width - 40, schedule_height),
                MonoTextStyleBuilder::new()
                    .font(&FONT_6X10)
                    .text_color(Rgb565::new(255, 0, 0))
                    .build(),
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::new(255, 255, 200))
                    .stroke_color(Rgb565::new(255, 0, 0))
                    .stroke_width(2)
                    .build(),
                10,
            )
            .draw(&mut display, "Schedule Error:", &e.to_string())?;
        }
    }

    // Draw various error banners in bottom half
    let banner_y = display_height / 2 + 10;
    let banner_width = (display_width - 60) / 3;
    let banner_height = 120;

    // Error banner
    ErrorBanner::new(
        Point::new(20, banner_y as i32),
        Size::new(banner_width, banner_height),
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::new(255, 0, 0))
            .build(),
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(255, 200, 200))
            .stroke_color(Rgb565::new(255, 0, 0))
            .stroke_width(2)
            .build(),
        8,
    )
    .draw(&mut display, "ERROR", "Connection failed")?;

    // Warning banner
    ErrorBanner::new(
        Point::new((20 + banner_width + 20) as i32, banner_y as i32),
        Size::new(banner_width, banner_height),
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::new(200, 100, 0))
            .build(),
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(255, 220, 180))
            .stroke_color(Rgb565::new(200, 100, 0))
            .stroke_width(2)
            .build(),
        8,
    )
    .draw(&mut display, "WARNING", "Low battery")?;

    // Info banner
    ErrorBanner::new(
        Point::new((20 + (banner_width + 20) * 2) as i32, banner_y as i32),
        Size::new(banner_width, banner_height),
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::new(0, 0, 200))
            .build(),
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(200, 220, 255))
            .stroke_color(Rgb565::new(0, 0, 200))
            .stroke_width(2)
            .build(),
        8,
    )
    .draw(&mut display, "INFO", "Update available")?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Combined Demo - Schedule & Error Banners", &output_settings);
    window.show_static(&display);

    Ok(())
}
