use std::marker::PhantomData;
use std::num::Saturating;
use std::ops::{RangeInclusive, Sub};

use chrono::{Duration, TimeDelta, prelude::*};
use embedded_graphics::mono_font::ascii::{FONT_6X12, FONT_10X20};
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    CornerRadii, CornerRadiiBuilder, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle,
    RoundedRectangle,
};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};

use crate::unified_color::{IntoPixelColorConverter, UnifiedColor};

const FONT_HEIGHT: i32 = FONT_10X20.character_size.height as i32;
const FONT_WIDTH: i32 = FONT_10X20.character_size.width as i32;
const SMALL_FONT_HEIGHT: i32 = FONT_6X12.character_size.height as i32;
const SMALL_FONT_WIDTH: i32 = FONT_6X12.character_size.width as i32;

const TIME_COL_HEADER: &str = "Time";

#[derive(Debug, Clone)]
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
    current_time: NaiveDateTime,
    date_range: ChronoRange<NaiveDate>,
    time_intervals: Vec<TimeInterval<'a>>,
    hours_to_show: i32,
    hours_range: ChronoRange<NaiveDateTime>,

    // styles
    text_style_black: MonoTextStyle<'a, T::Output>,
    text_small_style_black: MonoTextStyle<'a, T::Output>,
    text_small_style_white: MonoTextStyle<'a, T::Output>,
    thin_style: PrimitiveStyle<T::Output>,
    bold_style: PrimitiveStyle<T::Output>,
    red_bold_style: PrimitiveStyle<T::Output>,
    interval_style: PrimitiveStyle<T::Output>,

    radii: CornerRadii,

    _phantom: PhantomData<T>,
}

impl<'a, T> ScheduleTable<'a, T>
where
    T: IntoPixelColorConverter,
    T::Output: PixelColor,
{
    #[allow(clippy::too_many_arguments)] // this many arguments are justified for a schedule table
    pub fn new(
        top_left: Point,
        size: Size,
        current_time: NaiveDateTime,
        time_intervals: Vec<TimeInterval<'a>>,
        hours_to_show: i32,
    ) -> Self {
        let text_style_black: MonoTextStyle<T::Output> = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(T::convert(UnifiedColor::Black))
            .build();

        let text_small_style_black: MonoTextStyle<T::Output> = MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(T::convert(UnifiedColor::Black))
            .build();

        let text_small_style_white: MonoTextStyle<T::Output> = MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(T::convert(UnifiedColor::White))
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

        let radii = CornerRadiiBuilder::new().all(Size::new(10, 10)).build();

        // filter intervals, keep only current and future dates
        let time_intervals = time_intervals
            .iter()
            .filter(|i| i.start.date() >= current_time.date())
            .cloned()
            .collect::<Vec<_>>();

        // calculate the date range
        let first_date = time_intervals
            .iter()
            .min_by_key(|i| i.start.date())
            .unwrap()
            .start
            .date();
        let last_date = time_intervals
            .iter()
            .max_by_key(|i| i.end.date())
            .unwrap()
            .end
            .date();

        // calculate the hours range
        let mut start_hours_range_add = 0;
        let mut end_hours_range_add = 0;

        let start_hours_range = if current_time.hour() as i32 - hours_to_show / 2 < 0 {
            end_hours_range_add += hours_to_show / 2 - current_time.hour() as i32;
            current_time
                .date()
                .and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        } else {
            current_time - Duration::hours((hours_to_show / 2) as i64)
        };

        let end_hours_range = if current_time.hour() as i32 + hours_to_show / 2 > 24 {
            start_hours_range_add += current_time.hour() as i32 + hours_to_show / 2 - 24;
            (current_time.date()).and_time(chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap())
        } else {
            current_time + Duration::hours((hours_to_show / 2) as i64)
        };

        let start_hours_range = start_hours_range - Duration::hours(start_hours_range_add as i64);
        let end_hours_range = end_hours_range + Duration::hours(end_hours_range_add as i64);

        let start_hours_range =
            start_hours_range - Duration::minutes(start_hours_range.minute() as i64);
        let end_hours_range = end_hours_range - Duration::minutes(end_hours_range.minute() as i64);

        ScheduleTable {
            top_left,
            size,
            current_time,
            date_range: ChronoRange::from(first_date..=last_date),
            hours_range: ChronoRange::from(start_hours_range..=end_hours_range),
            time_intervals,
            hours_to_show,
            text_style_black,
            text_small_style_black,
            text_small_style_white,
            thin_style,
            bold_style,
            red_bold_style,
            interval_style,
            radii,
            _phantom: PhantomData,
        }
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = T::Output>,
    {
        // clear the display area for the table
        Rectangle::new(self.top_left, self.size)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(T::convert(UnifiedColor::White))
                    .build(),
            )
            .draw(display)?;

        let days_count = self.date_range.iter_days().count() as i32;

        // component positioning
        let component_width = self.size.width as i32;
        let component_height = self.size.height as i32;
        let component_left = self.top_left.x;
        let component_right = self.top_left.x + component_width;
        let component_top = self.top_left.y;
        let component_bottom = component_height + self.top_left.y;

        let header_height = FONT_HEIGHT * 2; // two lines for header
        let time_col_width = FONT_WIDTH * 6; // width for time column (e.g., "HH:MM")
        let date_col_width = (component_width - time_col_width) / days_count;
        let row_height = (component_height - header_height) / self.hours_to_show;

        // content positioning (content starts below the header row and after the time column)
        let content_top = self.top_left.y + header_height;
        let content_bottom = component_bottom;
        let content_left = self.top_left.x + time_col_width;

        // draw outer border
        Rectangle::new(self.top_left, self.size)
            .into_styled(self.thin_style)
            .draw(display)?;

        // draw header line
        Line::new(
            Point::new(component_left, self.top_left.y + header_height),
            Point::new(component_right, self.top_left.y + header_height),
        )
        .into_styled(self.bold_style)
        .draw(display)?;

        // draw horizontal lines for each hour
        for i in 1..self.hours_to_show {
            let y = content_top + i * row_height;
            Line::new(
                Point::new(component_left, y),
                Point::new(component_right, y),
            )
            .into_styled(self.thin_style)
            .draw(display)?;
        }

        // draw vertical line for time column
        Line::new(
            Point::new(content_left, component_top),
            Point::new(content_left, component_bottom),
        )
        .into_styled(self.thin_style)
        .draw(display)?;

        // draw vertical lines for each date column
        for i in 1..days_count {
            let x = content_left + i * date_col_width;
            Line::new(
                Point::new(x, component_top),
                Point::new(x, component_bottom),
            )
            .into_styled(self.thin_style)
            .draw(display)?;
        }

        // draw time column text
        {
            let col_width = time_col_width;
            let text_width = TIME_COL_HEADER.len() as i32 * FONT_WIDTH;
            let x_pos = component_left + (col_width / 2) - (text_width / 2);
            let y_pos = component_top + FONT_HEIGHT / 3;
            Text::with_baseline(
                TIME_COL_HEADER,
                Point::new(x_pos, y_pos),
                self.text_style_black,
                Baseline::Top,
            )
            .draw(display)?;
        }

        // draw date columns texts
        for (i, text) in self
            .date_range
            .iter_days()
            .map(|d| d.format("%d.%m.%Y").to_string())
            .enumerate()
            .map(|(i, text)| (i as i32 + 1, text))
        {
            let col_x = content_left + (i - 1) * date_col_width;
            let text_width = text.len() as i32 * FONT_WIDTH;
            let x_pos = col_x + (date_col_width / 2) - (text_width / 2);
            let y_pos = component_top + FONT_HEIGHT / 3;

            Text::with_baseline(
                &text,
                Point::new(x_pos, y_pos),
                self.text_style_black,
                Baseline::Top,
            )
            .draw(display)?;
        }

        // time column texts
        for (i, text) in self
            .hours_range
            .iter_hours()
            .map(|dt| dt.time().format("%H:%M").to_string())
            .enumerate()
            .map(|(i, text)| (i as i32, text))
        {
            let row_y = content_top + i * row_height;
            let text_width = text.len() as i32 * FONT_WIDTH;
            let x_pos = component_left + (time_col_width / 2) - (text_width / 2);
            let y_pos = row_y + FONT_HEIGHT / 3;

            Text::with_baseline(
                &text,
                Point::new(x_pos, y_pos),
                self.text_style_black,
                Baseline::Top,
            )
            .draw(display)?;
        }

        // Time intervals
        for interval in self.time_intervals.iter() {
            let col_index = if let Some(index) = self
                .date_range
                .iter_days()
                .position(|d| interval.start.date() == d)
                .map(|index| index as i32)
            {
                index
            } else {
                continue;
            };

            let col_x = content_left + col_index * date_col_width;

            let rel_start = interval.start.time() - self.hours_range.start().time();
            let rel_end = interval.end.time() - self.hours_range.start().time();

            let start_y = content_top as f32
                + (rel_start.num_hours() as f32 * row_height as f32)
                + (rel_start.num_minutes() as f32 % 60.0 * row_height as f32 / 60.0);
            let end_y = content_top as f32
                + (rel_end.num_hours() as f32 * row_height as f32)
                + (rel_end.num_minutes() as f32 % 60.0 * row_height as f32 / 60.0);

            let mut start_y = start_y as i32;
            let mut end_y = end_y as i32;

            if start_y <= content_top {
                start_y = content_top;
            }
            if end_y >= content_bottom {
                end_y = content_bottom;
            }

            if start_y >= end_y {
                continue; // skip intervals that are not in the visible range
            }

            RoundedRectangle::new(
                Rectangle::new(
                    Point::new(col_x + 4, start_y + 4),
                    Size::new(date_col_width as u32 - 8, (end_y - start_y) as u32 - 8),
                ),
                self.radii,
            )
            .into_styled(self.interval_style)
            .draw(display)?;

            if (end_y - start_y) >= FONT_HEIGHT {
                let text_width_approx = interval.label.len() as i32 * FONT_WIDTH;
                let text_x = col_x + (date_col_width / 2) - (text_width_approx / 2);
                let text_y = start_y + (end_y - start_y) / 2;
                Text::with_baseline(
                    interval.label,
                    Point::new(text_x, text_y),
                    self.text_style_black,
                    Baseline::Middle,
                )
                .draw(display)?;
            }
            let top_time_y = start_y - 4;
            let bottom_time_y = end_y;
            let start_time_str = interval.start.format("%H:%M").to_string();
            let end_time_str = interval.end.format("%H:%M").to_string();
            let start_time_x = col_x + (end_time_str.len() as i32 * SMALL_FONT_WIDTH / 3);
            let end_time_x =
                col_x + date_col_width - (end_time_str.len() as i32 * SMALL_FONT_WIDTH) - 8;

            let offsets = (-1..=1)
                .flat_map(|x| (-1..=1).map(move |y| Point::new(x, y)))
                .collect::<Vec<_>>();

            for offset in offsets {
                Text::with_baseline(
                    &start_time_str,
                    Point::new(start_time_x + offset.x, top_time_y + offset.y),
                    self.text_small_style_white,
                    Baseline::Top,
                )
                .draw(display)?;
                Text::with_baseline(
                    &end_time_str,
                    Point::new(end_time_x + offset.x, bottom_time_y + offset.y),
                    self.text_small_style_white,
                    Baseline::Bottom,
                )
                .draw(display)?;
            }

            Text::with_baseline(
                &start_time_str,
                Point::new(start_time_x, top_time_y),
                self.text_small_style_black,
                Baseline::Top,
            )
            .draw(display)?;
            Text::with_baseline(
                &end_time_str,
                Point::new(end_time_x, bottom_time_y),
                self.text_small_style_black,
                Baseline::Bottom,
            )
            .draw(display)?;
        }

        // draw current time line
        let now_line_y = content_top
            + ((self.current_time.hour() as i32 - self.hours_range.start().hour() as i32)
                * row_height)
            + (self.current_time.minute() as f32 * row_height as f32 / 60.0) as i32;

        let line_end_x = content_left + date_col_width;

        Line::new(
            Point::new(component_left, now_line_y),
            Point::new(line_end_x, now_line_y),
        )
        .into_styled(self.red_bold_style)
        .draw(display)?;

        Ok(())
    }
}

// Utils

#[derive(Debug, Clone)]
pub struct ChronoRange<T>(RangeInclusive<T>);

#[derive(Debug, Clone)]
pub struct ChronoRangeIter<T> {
    step: chrono::Duration,
    current: T,
    end: T,
}

impl<T> Iterator for ChronoRangeIter<T>
where
    T: Copy + PartialOrd + std::ops::Add<chrono::Duration, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
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
    }
}

impl<T> ChronoRange<T>
where
    T: Copy,
{
    pub fn iter(&self, step: chrono::Duration) -> ChronoRangeIter<T> {
        ChronoRangeIter {
            current: *self.0.start(),
            end: *self.0.end(),
            step,
        }
    }

    pub fn iter_days(&self) -> ChronoRangeIter<T> {
        self.iter(Duration::days(1))
    }

    pub fn iter_hours(&self) -> ChronoRangeIter<T> {
        self.iter(Duration::hours(1))
    }

    pub fn start(&self) -> &T {
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
