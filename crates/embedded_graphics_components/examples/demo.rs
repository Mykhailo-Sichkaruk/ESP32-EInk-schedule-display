use std::{
    ops::{Range, RangeInclusive},
    thread,
};

use chrono::Duration;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, Point, Size},
    primitives::PrimitiveStyleBuilder,
};
use embedded_graphics_components::{
    battery_indicator::BatteryIndicator,
    schedule_table::{ScheduleTable, TimeInterval},
    schedule_table_style::{Palette, ScheduleTableStyle, ScheduleTableStyleBuilder},
    unified_color::{IntoPixelColorConverter, UnifiedColor},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

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
    // let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(800, 480));

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

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Schedule", &output_settings);
    for current_time in ChronoRange::from(
        chrono::NaiveDateTime::new(today, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            ..=chrono::NaiveDateTime::new(
                day_after_tomorrow,
                chrono::NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
            ),
    )
    .iter(Duration::minutes(15))
    {
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
                // Handle window close event
                return Ok(());
            }
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }

    window.show_static(&display);

    Ok(())
}

// Utils

#[derive(Debug, Clone)]
pub struct ChronoRange<T> {
    start: T,
    end: T,
    inclusive: bool,
}

#[derive(Debug, Clone)]
pub struct ChronoRangeIter<T> {
    step: chrono::Duration,
    current: T,
    end: T,
    inclusive: bool,
}

impl<T> Iterator for ChronoRangeIter<T>
where
    T: Copy + PartialOrd + std::ops::Add<chrono::Duration, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inclusive {
            if self.current > self.end {
                None
            } else {
                let next = self.current;
                // Saturating add
                let current = self.current + self.step;
                if current < self.current {
                    return None;
                } else {
                    self.current = current;
                }
                Some(next)
            }
        } else {
            if self.current >= self.end {
                None
            } else {
                let next = self.current;
                // Saturating add
                let current = self.current + self.step;
                if current < self.current {
                    return None;
                } else {
                    self.current = current;
                }
                Some(next)
            }
        }
    }
}

impl<T> ChronoRange<T>
where
    T: Copy + Clone,
{
    pub fn iter(&self, step: chrono::Duration) -> ChronoRangeIter<T> {
        ChronoRangeIter {
            current: self.start,
            end: self.end,
            step,
            inclusive: self.inclusive,
        }
    }

    pub fn iter_days(&self) -> ChronoRangeIter<T> {
        self.iter(Duration::days(1))
    }

    pub fn iter_hours(&self) -> ChronoRangeIter<T> {
        self.iter(Duration::hours(1))
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> &T {
        &self.end
    }
}

impl<T> From<RangeInclusive<T>> for ChronoRange<T>
where
    T: Copy,
{
    fn from(range: RangeInclusive<T>) -> Self {
        ChronoRange {
            start: *range.start(),
            end: *range.end(),
            inclusive: true,
        }
    }
}

impl<T> From<Range<T>> for ChronoRange<T>
where
    T: Copy + Clone,
{
    fn from(range: Range<T>) -> Self {
        ChronoRange {
            start: range.start,
            end: range.end,
            inclusive: false,
        }
    }
}
