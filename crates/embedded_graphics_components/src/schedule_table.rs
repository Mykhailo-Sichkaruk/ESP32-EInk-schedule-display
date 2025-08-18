use std::marker::PhantomData;

use chrono::prelude::*;
use embedded_graphics::mono_font::ascii::{FONT_6X12, FONT_10X20};
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    CornerRadiiBuilder, Line, PrimitiveStyleBuilder, Rectangle, RoundedRectangle,
};
use embedded_graphics::text::Text;

use crate::unified_color::{IntoPixelColorConverter, UnifiedColor};

const FONT_HEIGHT: i32 = FONT_10X20.character_size.height as i32;
const FONT_WIDTH: i32 = FONT_10X20.character_size.width as i32;
const SMALL_FONT_HEIGHT: i32 = FONT_6X12.character_size.height as i32;
const SMALL_FONT_WIDTH: i32 = FONT_6X12.character_size.width as i32;

const TIME_ROW_HEADER: &str = "Time";

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
    header_texts: &'a [&'a str],
    time_range: core::ops::RangeInclusive<u8>,
    time_intervals: &'a [(&'a str, f32, f32, &'a str)],

    _phantom_converter: PhantomData<T>, // Используем PhantomData для типа Color
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
        header_texts: &'a [&'a str],
        time_range: core::ops::RangeInclusive<u8>,
        time_intervals: &'a [(&'a str, f32, f32, &'a str)],
    ) -> Self {
        ScheduleTable {
            top_left,
            size,
            header_height,
            time_col_width,
            y_pos_offset,
            current_time: nowline_time,
            header_texts,
            time_range,
            time_intervals,
            //
            _phantom_converter: PhantomData,
        }
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = T::Output>,
    {
        let display_width = self.size.width as i32;
        let display_height = self.size.height as i32;

        let date_col_width = (display_width - self.time_col_width) / self.header_texts.len() as i32;
        let row_height = (display_height - self.header_height) / self.time_range.len() as i32;

        // Clear the area of the schedule table with white
        Rectangle::new(self.top_left, self.size)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(T::convert(UnifiedColor::White))
                    .build(),
            )
            .draw(display)?;

        let text_style_black: MonoTextStyle<T::Output> = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(T::convert(UnifiedColor::Black))
            .build();

        let text_small_style_black: MonoTextStyle<T::Output> = MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(T::convert(UnifiedColor::Black))
            .background_color(T::convert(UnifiedColor::White))
            .build();

        let base_style = PrimitiveStyleBuilder::new()
            .stroke_color(T::convert(UnifiedColor::Black))
            .stroke_width(1)
            .build();

        let bold_line_style = PrimitiveStyleBuilder::new()
            .stroke_color(T::convert(UnifiedColor::Black))
            .stroke_width(2)
            .build();

        let now_line_style = PrimitiveStyleBuilder::new()
            .stroke_color(T::convert(UnifiedColor::Chromatic))
            .stroke_width(4)
            .build();

        let interval_style = PrimitiveStyleBuilder::new()
            .stroke_color(T::convert(UnifiedColor::Black))
            .stroke_width(2)
            .fill_color(T::convert(UnifiedColor::White))
            .build();

        // Outer border (now relative to top_left of the table)
        Rectangle::new(self.top_left, self.size)
            .into_styled(base_style)
            .draw(display)?;

        // Horizontal lines
        Line::new(
            Point::new(self.top_left.x, self.top_left.y + self.header_height),
            Point::new(
                self.top_left.x + display_width,
                self.top_left.y + self.header_height,
            ),
        )
        .into_styled(bold_line_style)
        .draw(display)?;

        for i in 1..self.time_range.len() as i32 {
            let y = self.top_left.y + self.header_height + i * row_height;
            // Only draw lines within the table's defined height
            if y < self.top_left.y + display_height {
                Line::new(
                    Point::new(self.top_left.x, y),
                    Point::new(self.top_left.x + display_width, y),
                )
                .into_styled(base_style)
                .draw(display)?;
            }
        }

        // Vertical lines
        Line::new(
            Point::new(self.top_left.x + self.time_col_width, self.top_left.y),
            Point::new(
                self.top_left.x + self.time_col_width,
                self.top_left.y + display_height,
            ),
        )
        .into_styled(base_style)
        .draw(display)?;

        for i in 1..self.header_texts.len() as i32 {
            let x = self.top_left.x + self.time_col_width + i * date_col_width;
            Line::new(
                Point::new(x, self.top_left.y),
                Point::new(x, self.top_left.y + display_height),
            )
            .into_styled(base_style)
            .draw(display)?;
        }

        // Header texts
        for (i, &text) in [TIME_ROW_HEADER]
            .iter()
            .chain(self.header_texts.iter())
            .enumerate()
        {
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

            Text::new(text, Point::new(x_pos, y_pos), text_style_black).draw(display)?;
        }

        // Time column texts
        for (i, hour) in self.time_range.clone().enumerate() {
            let text = format!("{hour:02}:00");
            let row_y = self.top_left.y + self.header_height + i as i32 * row_height;

            let text_width = text.len() as i32 * FONT_WIDTH;
            let x_pos = self.top_left.x + (self.time_col_width / 2) - (text_width / 2);
            let y_pos = row_y + (row_height / 2) + self.y_pos_offset - FONT_HEIGHT;

            // Only draw if within data rows (0-indexed to num_data_rows-1)
            if i < self.time_range.len() {
                Text::new(&text, Point::new(x_pos, y_pos), text_style_black).draw(display)?;
            }
        }

        // Time intervals
        let radii = CornerRadiiBuilder::new().all(Size::new(10, 10)).build();
        let start_time_f32 = *self.time_range.start() as f32;
        for (date, start, end, text) in self.time_intervals {
            let col_index = match *date {
                "01.01.2025" => 1,
                "02.01.2025" => 2,
                "03.01.2025" => 3,
                _ => continue,
            };

            let col_x = self.top_left.x + self.time_col_width + (col_index - 1) * date_col_width;

            let rel_start = start - start_time_f32;
            let rel_end = end - start_time_f32;

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
                .into_styled(interval_style)
                .draw(display)?;

                if (end - start) >= 0.5 {
                    let text_width_approx = text.len() as i32 * FONT_WIDTH;
                    let text_x = col_x + (date_col_width / 2) - (text_width_approx / 2);
                    let text_y =
                        start_y + (end_y - start_y) / 2 + self.y_pos_offset - (FONT_HEIGHT / 3);

                    Text::new(text, Point::new(text_x, text_y), text_style_black).draw(display)?;
                }

                let top_time_y = start_y + self.y_pos_offset - (SMALL_FONT_HEIGHT / 2);
                let bottom_time_y = end_y + self.y_pos_offset - (SMALL_FONT_HEIGHT);

                let start_time_str =
                    format!("{:02}:{:02}", *start as i32, (*start * 60.0) as i32 % 60);
                let end_time_str = format!("{:02}:{:02}", *end as i32, (*end * 60.0) as i32 % 60);

                let start_time_x = col_x + (end_time_str.len() as i32 * SMALL_FONT_WIDTH / 3);

                let end_time_x = col_x
                    + (date_col_width - (end_time_str.len() as i32 * SMALL_FONT_WIDTH / 2))
                    - (end_time_str.len() as i32 * SMALL_FONT_WIDTH / 2)
                    - 8;

                Text::new(
                    &start_time_str,
                    Point::new(start_time_x, top_time_y),
                    text_small_style_black,
                )
                .draw(display)?;

                Text::new(
                    &end_time_str,
                    Point::new(end_time_x, bottom_time_y),
                    text_small_style_black,
                )
                .draw(display)?;
            }
        }

        // Current time line
        let now_line_y = (self.top_left.y as f32
            + self.header_height as f32
            + (self.current_time - start_time_f32) * row_height as f32)
            as i32;

        let line_end_x = self.top_left.x + self.time_col_width + date_col_width; // Line extends across 2 date columns

        Line::new(
            Point::new(self.top_left.x, now_line_y),
            Point::new(line_end_x, now_line_y),
        )
        .into_styled(now_line_style)
        .draw(display)?;

        Ok(())
    }
}
