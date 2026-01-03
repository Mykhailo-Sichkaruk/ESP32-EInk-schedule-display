use core::fmt;

#[derive(Debug)]
pub enum InvalidInterval {
    FirstDateNotFound,
    LastDateNotFound,
    EndBeforeStart,
}

impl InvalidInterval {
    pub fn description(&self) -> &str {
        match self {
            InvalidInterval::FirstDateNotFound => "First date not found",
            InvalidInterval::LastDateNotFound => "Last date not found",
            InvalidInterval::EndBeforeStart => "End time is before start time",
        }
    }
}

#[derive(Debug)]
pub enum ScheduleTableError {
    InvalidInterval(InvalidInterval),
}

impl fmt::Display for ScheduleTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduleTableError::InvalidInterval(interval) => {
                write!(f, "Invalid interval: {}", interval.description())
            }
        }
    }
}

impl std::error::Error for ScheduleTableError {}
