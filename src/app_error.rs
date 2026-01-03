use core::fmt;

use crate::schedule_api::ScheduleParseError;

#[derive(Debug)]
pub enum AppError {
    Init(anyhow::Error),
    WifiInit(anyhow::Error),
    WifiConnect(anyhow::Error),
    FetchSchedule(anyhow::Error),
    ParseSchedule(ScheduleParseError),
    Render(anyhow::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Init(err) => write!(f, "init failed: {err}"),
            AppError::WifiInit(err) => write!(f, "wifi init failed: {err}"),
            AppError::WifiConnect(err) => write!(f, "wifi connect failed: {err}"),
            AppError::FetchSchedule(err) => write!(f, "fetch schedule failed: {err}"),
            AppError::ParseSchedule(err) => write!(f, "parse schedule failed: {err}"),
            AppError::Render(err) => write!(f, "render failed: {err}"),
        }
    }
}

impl std::error::Error for AppError {}
