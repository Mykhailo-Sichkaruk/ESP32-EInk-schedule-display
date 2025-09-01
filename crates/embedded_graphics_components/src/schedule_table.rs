use std::ops::{Range, RangeInclusive};

use chrono::{Duration, prelude::*};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, Rectangle, RoundedRectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder};

use crate::errors::{InvalidInterval, ScheduleTableError};
use crate::schedule_table_style::ScheduleTableStyle;

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

pub struct ScheduleTable<'a, C>
where
    C: PixelColor,
{
    top_left: Point,
    size: Size,
    current_time: NaiveDateTime,
    date_range: Vec<NaiveDate>,
    time_intervals: &'a [TimeInterval<'a>],
    hours_to_show: i32, // FIXME: remove
    time_window_start: NaiveTime,

    // styles
    style: ScheduleTableStyle<'a, C>,
}

impl<'a, C> ScheduleTable<'a, C>
where
    C: PixelColor,
{
    const TIME_COL_HEADER: &'static str = "Time";

    #[allow(clippy::too_many_arguments)] // this many arguments are justified for a schedule table
    pub fn new(
        top_left: Point,
        size: Size,
        style: ScheduleTableStyle<'a, C>,
        current_time: NaiveDateTime,
        time_intervals: &'a [TimeInterval<'a>],
        hours_to_show: i32,
    ) -> Result<Self, ScheduleTableError> {
        let (first_date, last_date) = time_intervals
            .iter()
            .filter(|i| i.start.date() >= current_time.date())
            .try_fold((None, None), |(first, last), interval| {
                let start = interval.start.date();
                let end = interval.end.date();
                Ok((
                    Some(first.map_or(start, |f: NaiveDate| f.min(start))),
                    Some(last.map_or(end, |l: NaiveDate| l.max(end))),
                ))
            })
            .and_then(|(first, last)| {
                let first = first.ok_or(ScheduleTableError::InvalidInterval(
                    InvalidInterval::FirstDateNotFound,
                ))?;
                let last = last.ok_or(ScheduleTableError::InvalidInterval(
                    InvalidInterval::LastDateNotFound,
                ))?;
                Ok((first, last))
            })?;

        let half_window_hours = hours_to_show / 2;
        let current_hour = current_time.hour() as i32;

        // clamp window to [0, 23]
        let clamped_start_hour = (current_hour - half_window_hours).max(0)
            + -(current_hour + half_window_hours - 24).max(0);
        let clamped_end_hour =
            (current_hour + half_window_hours).min(23) + -(current_hour - half_window_hours).min(0);

        // minutes become 00 by construction
        let start_of_window = current_time.date().and_time(
            chrono::NaiveTime::from_hms_opt(clamped_start_hour as u32, 0, 0)
                .expect("Failed to create NaiveTime"),
        );

        let end_of_window = current_time.date().and_time(
            chrono::NaiveTime::from_hms_opt(clamped_end_hour as u32, 0, 0)
                .expect("Failed to create NaiveTime"),
        );

        Ok(ScheduleTable {
            top_left,
            size,
            current_time,
            date_range: Vec::from_iter(
                (0..=last_date.signed_duration_since(first_date).num_days())
                    .map(|d| first_date + Duration::days(d)),
            ),
            time_window_start: start_of_window.time(),
            time_intervals,
            hours_to_show,
            style,
        })
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // clear the display area for the table
        Rectangle::new(self.top_left, self.size)
            .into_styled(self.style.background)
            .draw(display)?;

        let days_count = self.date_range.len() as i32;

        // component positioning
        let component_width = self.size.width as i32;
        let component_height = self.size.height as i32;
        let component_left = self.top_left.x;
        let component_right = self.top_left.x + component_width;
        let component_top = self.top_left.y;
        let component_bottom = component_height + self.top_left.y;

        let header_height = self.style.text_body.font.character_size.height as i32 * 2; // two lines for header
        let time_col_width = self.style.text_body.font.character_size.width as i32 * 6; // width for time column (e.g., "HH:MM")
        let date_col_width = (component_width - time_col_width) / days_count;
        let row_height = (component_height - header_height) / self.hours_to_show;

        // content positioning (content starts below the header row and after the time column)
        let content_top = self.top_left.y + header_height;
        let content_bottom = component_bottom;
        let content_left = self.top_left.x + time_col_width;

        let body_font_height = self.style.text_body.font.character_size.height as i32;

        // draw outer border
        Rectangle::new(self.top_left, self.size)
            .into_styled(self.style.border)
            .draw(display)?;

        // draw header line
        Line::new(
            Point::new(component_left, self.top_left.y + header_height),
            Point::new(component_right, self.top_left.y + header_height),
        )
        .into_styled(self.style.header_line)
        .draw(display)?;

        // draw horizontal lines for each hour
        for i in 1..self.hours_to_show {
            let y = content_top + i * row_height;
            Line::new(
                Point::new(component_left, y),
                Point::new(component_right, y),
            )
            .into_styled(self.style.grid_line)
            .draw(display)?;
        }

        // draw vertical line for time column
        Line::new(
            Point::new(content_left, component_top),
            Point::new(content_left, component_bottom),
        )
        .into_styled(self.style.grid_line)
        .draw(display)?;

        // draw vertical lines for each date column
        for i in 1..days_count {
            let x = content_left + i * date_col_width;
            Line::new(
                Point::new(x, component_top),
                Point::new(x, component_bottom),
            )
            .into_styled(self.style.grid_line)
            .draw(display)?;
        }

        // draw time column text
        {
            let x_pos = component_left + (time_col_width / 2);
            let y_pos = component_top + body_font_height / 3;
            Text::with_text_style(
                Self::TIME_COL_HEADER,
                Point::new(x_pos, y_pos),
                self.style.text_body,
                TextStyleBuilder::new()
                    .alignment(Alignment::Center)
                    .baseline(Baseline::Top)
                    .build(),
            )
            .draw(display)?;
        }

        // draw date columns texts
        for (i, text) in self
            .date_range
            .iter()
            .map(|d| d.format("%d.%m.%Y").to_string())
            .enumerate()
            .map(|(i, text)| (i as i32 + 1, text))
        {
            let x_pos = content_left + (i - 1) * date_col_width + (date_col_width / 2);
            let y_pos = component_top + body_font_height / 3;

            Text::with_text_style(
                &text,
                Point::new(x_pos, y_pos),
                self.style.text_body,
                TextStyleBuilder::new()
                    .alignment(Alignment::Center)
                    .baseline(Baseline::Top)
                    .build(),
            )
            .draw(display)?;
        }

        // time column texts
        for (i, text) in (0..self.hours_to_show)
            .map(|h| self.time_window_start + Duration::hours(h as i64))
            .map(|dt| dt.format("%H:%M").to_string())
            .enumerate()
            .map(|(i, text)| (i as i32, text))
        {
            let x_pos = component_left + (time_col_width / 2);
            let y_pos = content_top + i * row_height;

            Text::with_text_style(
                &text,
                Point::new(x_pos, y_pos),
                self.style.text_body,
                TextStyleBuilder::new()
                    .alignment(Alignment::Center)
                    .baseline(Baseline::Top)
                    .build(),
            )
            .draw(display)?;
        }

        // Time intervals
        for interval in self
            .time_intervals
            .iter()
            .filter(|i| i.start.date() >= self.current_time.date())
        {
            let col_index = if let Some(index) = self
                .date_range
                .iter()
                .position(|d| interval.start.date() == *d)
                .map(|index| index as i32)
            {
                index
            } else {
                continue;
            };

            let col_x = content_left + col_index * date_col_width;

            let rel_start = interval.start.time() - self.time_window_start;
            let rel_end = interval.end.time() - self.time_window_start;

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

            let box_start_y = start_y + self.style.interval_box_margin;
            let box_start_x = col_x + self.style.interval_box_margin;
            let box_width = (date_col_width - self.style.interval_box_margin * 2).max(0);
            let box_height = (end_y - start_y - self.style.interval_box_margin * 2).max(0);

            RoundedRectangle::new(
                Rectangle::new(
                    Point::new(box_start_x, box_start_y),
                    Size::new(box_width as u32, box_height as u32),
                ),
                self.style.interval_box_radii,
            )
            .into_styled(self.style.interval_box)
            .draw(display)?;

            if (end_y - start_y) >= body_font_height {
                let text_x = col_x + (date_col_width / 2);
                let text_y = start_y + (end_y - start_y) / 2;
                Text::with_text_style(
                    interval.label,
                    Point::new(text_x, text_y),
                    self.style.text_body,
                    TextStyleBuilder::new()
                        .alignment(Alignment::Center)
                        .baseline(Baseline::Middle)
                        .build(),
                )
                .draw(display)?;
            }

            let start_time_str = interval.start.format("%H:%M").to_string();
            let end_time_str = interval.end.format("%H:%M").to_string();
            let text_start_x = col_x
                + self.style.interval_box_margin
                + self.style.interval_box_radii.top_left.width as i32;
            let text_end_x = col_x + date_col_width
                - self.style.interval_box_margin
                - self.style.interval_box_radii.bottom_right.width as i32;

            if let Some((style, x_range, y_range)) = &self.style.text_small_shadow {
                let offsets = x_range
                    .clone()
                    .flat_map(|x| y_range.clone().map(move |y| Point::new(x, y)));

                for offset in offsets {
                    Text::with_text_style(
                        &start_time_str,
                        Point::new(text_start_x + offset.x, box_start_y + offset.y),
                        *style,
                        TextStyleBuilder::new()
                            .alignment(Alignment::Left)
                            .baseline(Baseline::Middle)
                            .build(),
                    )
                    .draw(display)?;
                    Text::with_text_style(
                        &end_time_str,
                        Point::new(text_end_x + offset.x, box_start_y + box_height + offset.y),
                        *style,
                        TextStyleBuilder::new()
                            .alignment(Alignment::Right)
                            .baseline(Baseline::Middle)
                            .build(),
                    )
                    .draw(display)?;
                }
            }

            Text::with_text_style(
                &start_time_str,
                Point::new(text_start_x, box_start_y),
                self.style.text_small,
                TextStyleBuilder::new()
                    .alignment(Alignment::Left)
                    .baseline(Baseline::Middle)
                    .build(),
            )
            .draw(display)?;
            Text::with_text_style(
                &end_time_str,
                Point::new(text_end_x, box_start_y + box_height),
                self.style.text_small,
                TextStyleBuilder::new()
                    .alignment(Alignment::Right)
                    .baseline(Baseline::Middle)
                    .build(),
            )
            .draw(display)?;
        }

        // draw current time line
        let now_line_y = content_top
            + ((self.current_time.hour() as i32 - self.time_window_start.hour() as i32)
                * row_height)
            + (self.current_time.minute() as f32 * row_height as f32 / 60.0) as i32;

        let line_end_x = content_left + date_col_width;

        Line::new(
            Point::new(component_left, now_line_y),
            Point::new(line_end_x, now_line_y),
        )
        .into_styled(self.style.time_line)
        .draw(display)?;

        Ok(())
    }
}
