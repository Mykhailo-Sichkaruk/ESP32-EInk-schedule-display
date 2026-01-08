use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::app_error::AppError;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub widgets: ResponseWidgets,
    pub system: ResponseSystem,
}

#[derive(Debug, Deserialize)]
pub struct ResponseWidgets {
    pub schedule: ResponseWidgetSchedule,
}

#[derive(Debug, Deserialize)]
pub struct ResponseWidgetSchedule {
    pub events: Vec<ResponseWidgetScheduleEvent>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseSystem {
    #[serde(with = "chrono::naive::serde::ts_milliseconds")]
    pub server_time_unix: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ResponseWidgetScheduleEvent {
    pub label: String,
    #[serde(with = "chrono::naive::serde::ts_milliseconds")]
    pub start_unix: NaiveDateTime,
    #[serde(with = "chrono::naive::serde::ts_milliseconds")]
    pub end_unix: NaiveDateTime,
}

pub fn parse(json: &str) -> Result<Response, AppError> {
    serde_json::from_str(json).map_err(|_| AppError::JsonDeserializationFailed)
}
