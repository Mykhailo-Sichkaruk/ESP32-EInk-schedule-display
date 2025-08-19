use std::marker::PhantomData;
use std::ops::{RangeInclusive, Sub};

use chrono::{Duration, prelude::*};
use embedded_graphics::mono_font::ascii::{FONT_6X12, FONT_10X20};
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    CornerRadiiBuilder, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle,
};
use embedded_graphics::text::Text;

use crate::unified_color::{IntoPixelColorConverter, UnifiedColor};

const FONT_HEIGHT: i32 = FONT_10X20.character_size.height as i32;
const FONT_WIDTH: i32 = FONT_10X20.character_size.width as i32;
const SMALL_FONT_HEIGHT: i32 = FONT_6X12.character_size.height as i32;
const SMALL_FONT_WIDTH: i32 = FONT_6X12.character_size.width as i32;

const TIME_ROW_HEADER: &str = "Time";

pub struct TimeInterval<'a> {
    start: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime,
    label: &'a str,
}

impl<'a> TimeInterval<'a> {
    pub fn new(start: chrono::NaiveDateTime, end: chrono::NaiveDateTime, label: &'a str) -> Self {
        TimeInterval { start, end, label }
    }
}

pub struct ScheduleTable<'a, T>
where
    T: IntoPixelColorConverter,
{
    top_left: Point,
    size: Size,
    header_height: i32,
    time_col_width: i32,
    y_pos_offset: i32, // FIXME: MAGIC
    current_time: f32,
    time_range: ChronoRange<NaiveDateTime>,
    time_intervals: &'a [TimeInterval<'a>],

    // Styles
    text_style_black: MonoTextStyle<'a, T::Output>,
    text_small_style_black: MonoTextStyle<'a, T::Output>,
    thin_style: PrimitiveStyle<T::Output>,
    bold_style: PrimitiveStyle<T::Output>,
    red_bold_style: PrimitiveStyle<T::Output>,
    interval_style: PrimitiveStyle<T::Output>,

    _phantom: PhantomData<T>,
}

impl<'a, T> ScheduleTable<'a, T>
where
    T: IntoPixelColorConverter,
    T::Output: PixelColor,
{
    #[allow(clippy::too_many_arguments)] // This many arguments are justified for a schedule table
    pub fn new(
        top_left: Point,
        size: Size,
        header_height: i32,
        time_col_width: i32,
        y_pos_offset: i32,
        nowline_time: f32,
        time_range: RangeInclusive<NaiveDateTime>,
        time_intervals: &'a [TimeInterval<'a>],
    ) -> Self {
        let text_style_black: MonoTextStyle<T::Output> = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(T::convert(UnifiedColor::Black))
            .build();

        let text_small_style_black: MonoTextStyle<T::Output> = MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(T::convert(UnifiedColor::Black))
            .background_color(T::convert(UnifiedColor::White))
            .build();

        let thin_style = PrimitiveStyleBuilder::new()
            .stroke_color(T::convert(UnifiedColor::Black))
            .stroke_width(1)
            .build();

        let bold_style = PrimitiveStyleBuilder::new()
            .stroke_color(T::convert(UnifiedColor::Black))
            .stroke_width(2)
            .build();

        let red_bold_style = PrimitiveStyleBuilder::new()
            .stroke_color(T::convert(UnifiedColor::Chromatic))
            .stroke_width(4)
            .build();

        let interval_style = PrimitiveStyleBuilder::new()
            .stroke_color(T::convert(UnifiedColor::Black))
            .stroke_width(2)
            .fill_color(T::convert(UnifiedColor::White))
            .build();

        ScheduleTable {
            top_left,
            size,
            header_height,
            time_col_width,
            y_pos_offset,
            current_time: nowline_time,
            time_range: time_range.into(),
            time_intervals,
            text_style_black,
            text_small_style_black,
            thin_style,
            bold_style,
            red_bold_style,
            interval_style,
            _phantom: PhantomData,
        }
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = T::Output>,
    {
        // Clear the display area for the table
        Rectangle::new(self.top_left, self.size)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(T::convert(UnifiedColor::White))
                    .build(),
            )
            .draw(display)?;

        let hours_range =
            ChronoRange::from(self.time_range.start().time()..=self.time_range.end().time());

        let display_width = self.size.width as i32;
        let display_height = self.size.height as i32;
        let date_col_width = (display_width - self.time_col_width)
            / self.time_range.iter(Duration::days(1)).count() as i32;
        let row_height = (display_height - self.header_height)
            / hours_range.iter(Duration::hours(1)).count() as i32;

        // Outer border (now relative to top_left of the table)
        Rectangle::new(self.top_left, self.size)
            .into_styled(self.thin_style)
            .draw(display)?;

        // Draw header line
        Line::new(
            Point::new(self.top_left.x, self.top_left.y + self.header_height),
            Point::new(
                self.top_left.x + display_width,
                self.top_left.y + self.header_height,
            ),
        )
        .into_styled(self.bold_style)
        .draw(display)?;

        // Draw horizontal lines for each hour
        for i in 1..hours_range.iter(Duration::hours(1)).count() as i32 {
            let y = self.top_left.y + self.header_height + i * row_height;
            Line::new(
                Point::new(self.top_left.x, y),
                Point::new(self.top_left.x + display_width, y),
            )
            .into_styled(self.thin_style)
            .draw(display)?;
        }

        // Draw vertical line for time column
        Line::new(
            Point::new(self.top_left.x + self.time_col_width, self.top_left.y),
            Point::new(
                self.top_left.x + self.time_col_width,
                self.top_left.y + display_height,
            ),
        )
        .into_styled(self.thin_style)
        .draw(display)?;

        // Draw vertical lines for each date column
        for i in 1..self.time_range.iter(Duration::days(1)).count() as i32 {
            let x = self.top_left.x + self.time_col_width + i * date_col_width;
            Line::new(
                Point::new(x, self.top_left.y),
                Point::new(x, self.top_left.y + display_height),
            )
            .into_styled(self.thin_style)
            .draw(display)?;
        }

        let mut header_texts = vec![TIME_ROW_HEADER.to_string()];
        header_texts.extend(
            self.time_range
                .iter(Duration::days(1))
                .map(|d| d.format("%d.%m.%Y").to_string()),
        );

        for (i, text) in header_texts.iter().enumerate() {
            let col_x = if i == 0 {
                self.top_left.x
            } else {
                self.top_left.x + self.time_col_width + (i - 1) as i32 * date_col_width
            };
            let col_width = if i == 0 {
                self.time_col_width
            } else {
                date_col_width
            };

            let text_width = text.len() as i32 * FONT_WIDTH;
            let x_pos = col_x + (col_width / 2) - (text_width / 2);
            let y_pos =
                self.top_left.y + (self.header_height / 2) - (FONT_HEIGHT / 2) + self.y_pos_offset;

            Text::new(
                text.as_str(),
                Point::new(x_pos, y_pos),
                self.text_style_black,
            )
            .draw(display)?;
        }

        // Time column texts
        for (i, hour) in hours_range.iter(Duration::hours(1)).enumerate() {
            let text = format!("{:02}:00", hour.hour());
            let row_y = self.top_left.y + self.header_height + i as i32 * row_height;

            let text_width = text.len() as i32 * FONT_WIDTH;
            let x_pos = self.top_left.x + (self.time_col_width / 2) - (text_width / 2);
            let y_pos = row_y + (row_height / 2) + self.y_pos_offset - FONT_HEIGHT;

            // Only draw if within data rows (0-indexed to num_data_rows-1)
            if i < hours_range.iter(Duration::hours(1)).count() {
                Text::new(&text, Point::new(x_pos, y_pos), self.text_style_black).draw(display)?;
            }
        }

        // Time intervals
        let radii = CornerRadiiBuilder::new().all(Size::new(10, 10)).build();
        let start_time = *self.time_range.start();
        for interval in self.time_intervals {
            let col_index = self
                .time_range
                .iter(Duration::days(1))
                .position(|d| d.date() == interval.start.date());
            let col_index = if let Some(index) = col_index {
                index as i32 + 1
            } else {
                continue;
            };

            let col_x = self.top_left.x + self.time_col_width + (col_index - 1) * date_col_width;

            let rel_start = (interval.start - start_time).num_hours() as f32;
            let rel_end = (interval.end - start_time).num_hours() as f32;

            let start_y =
                self.top_left.y + self.header_height + (rel_start * row_height as f32) as i32;
            let end_y = self.top_left.y + self.header_height + (rel_end * row_height as f32) as i32;

            // Ensure interval is within the bounds of the table
            if start_y < self.top_left.y + display_height
                && end_y > self.top_left.y + self.header_height
            {
                RoundedRectangle::new(
                    Rectangle::new(
                        Point::new(col_x + 4, start_y + 4),
                        Size::new(date_col_width as u32 - 8, (end_y - start_y) as u32 - 8),
                    ),
                    radii,
                )
                .into_styled(self.interval_style)
                .draw(display)?;

                if (rel_end - rel_start) >= 0.5 {
                    let text_width_approx = interval.label.len() as i32 * FONT_WIDTH;
                    let text_x = col_x + (date_col_width / 2) - (text_width_approx / 2);
                    let text_y =
                        start_y + (end_y - start_y) / 2 + self.y_pos_offset - (FONT_HEIGHT / 3);

                    Text::new(
                        interval.label,
                        Point::new(text_x, text_y),
                        self.text_style_black,
                    )
                    .draw(display)?;
                }

                let top_time_y = start_y + self.y_pos_offset - (SMALL_FONT_HEIGHT / 2);
                let bottom_time_y = end_y + self.y_pos_offset - (SMALL_FONT_HEIGHT);

                let start_time_str = interval.start.format("%H:%M").to_string();
                let end_time_str = interval.end.format("%H:%M").to_string();

                let start_time_x = col_x + (end_time_str.len() as i32 * SMALL_FONT_WIDTH / 3);

                let end_time_x = col_x
                    + (date_col_width - (end_time_str.len() as i32 * SMALL_FONT_WIDTH / 2))
                    - (end_time_str.len() as i32 * SMALL_FONT_WIDTH / 2)
                    - 8;

                Text::new(
                    &start_time_str,
                    Point::new(start_time_x, top_time_y),
                    self.text_small_style_black,
                )
                .draw(display)?;

                Text::new(
                    &end_time_str,
                    Point::new(end_time_x, bottom_time_y),
                    self.text_small_style_black,
                )
                .draw(display)?;
            }
        }

        // Current time line
        // let now_line_y = (self.top_left.y as f32
        //     + self.header_height as f32
        //     + (self.current_time - start_time_f32) * row_height as f32)
        //     as i32;

        // let line_end_x = self.top_left.x + self.time_col_width + date_col_width; // Line extends across 2 date columns

        // Line::new(
        //     Point::new(self.top_left.x, now_line_y),
        //     Point::new(line_end_x, now_line_y),
        // )
        // .into_styled(self.red_bold_style)
        // .draw(display)?;

        Ok(())
    }
}

// Utils

#[derive(Debug, Clone)]
struct ChronoRange<T>(RangeInclusive<T>);

#[derive(Debug, Clone)]
struct ChronoRangeIter<T> {
    step: chrono::Duration,
    current: T,
    end: T,
}

impl<T> Iterator for ChronoRangeIter<T>
where
    T: Copy + std::ops::AddAssign<chrono::Duration> + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let next = self.current;
            self.current += self.step;
            Some(next)
        }
    }
}

impl<T> ChronoRange<T>
where
    T: Copy + Sub<Output = chrono::Duration>,
{
    fn iter(&self, step: chrono::Duration) -> ChronoRangeIter<T> {
        ChronoRangeIter {
            current: *self.0.start(),
            end: *self.0.end(),
            step,
        }
    }

    fn start(&self) -> &T {
        self.0.start()
    }

    fn end(&self) -> &T {
        self.0.end()
    }
}

impl<T> From<RangeInclusive<T>> for ChronoRange<T> {
    fn from(range: RangeInclusive<T>) -> Self {
        ChronoRange(range)
    }
}
