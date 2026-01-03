use chrono::{NaiveDateTime};
use embedded_graphics_components::schedule_table::TimeInterval;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParsedSchedule {
    pub current_time: NaiveDateTime,
    pub items: Vec<ScheduleItem>,
}

#[derive(Debug)]
pub struct ScheduleItem {
    pub label: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
struct ScheduleResponse {
    server_time: i64,
    items: Vec<ScheduleItemRaw>,
}

#[derive(Debug, Deserialize)]
struct ScheduleItemRaw {
    label: String,
    start: i64,
    end: i64,
}

#[derive(Debug)]
pub enum ScheduleParseError {
    Json(serde_json::Error),
    InvalidTimestamp { field: &'static str, value: i64 },
}

impl fmt::Display for ScheduleParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduleParseError::Json(err) => write!(f, "json parse error: {err}"),
            ScheduleParseError::InvalidTimestamp { field, value } => {
                write!(f, "invalid timestamp for {field}: {value}")
            }
        }
    }
}

impl Error for ScheduleParseError {}

impl From<serde_json::Error> for ScheduleParseError {
    fn from(err: serde_json::Error) -> Self {
        ScheduleParseError::Json(err)
    }
}

impl ParsedSchedule {
    pub fn time_intervals(&self) -> Vec<TimeInterval<'_>> {
        self.items
            .iter()
            .map(|item| TimeInterval::new(item.start, item.end, item.label.as_str()))
            .collect()
    }
}

pub fn parse_schedule(json: &str) -> Result<ParsedSchedule, ScheduleParseError> {
    let response: ScheduleResponse = serde_json::from_str(json)?;
    let current_time = parse_epoch(response.server_time, "server_time")?;
    let mut items = Vec::with_capacity(response.items.len());

    for item in response.items {
        let start = parse_epoch(item.start, "start")?;
        let end = parse_epoch(item.end, "end")?;
        items.push(ScheduleItem {
            label: item.label,
            start,
            end,
        });
    }

    Ok(ParsedSchedule {
        current_time,
        items,
    })
}

fn parse_epoch(value: i64, field: &'static str) -> Result<NaiveDateTime, ScheduleParseError> {
    // Accept seconds or milliseconds since epoch.
    let (secs, nanos) = if value.abs() >= 1_000_000_000_000 {
        let secs = value.div_euclid(1000);
        let millis = value.rem_euclid(1000) as u32;
        (secs, millis * 1_000_000)
    } else {
        (value, 0)
    };

    chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nanos)
        .map(|dt| dt.naive_utc())
        .ok_or(ScheduleParseError::InvalidTimestamp { field, value })
}
