use chrono::{Duration, NaiveDateTime};

use crate::errors::{InvalidInterval, ScheduleTableError};

#[derive(Debug, Clone)]
pub struct TimeInterval<'a> {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub label: &'a str,
}

impl<'a> TimeInterval<'a> {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        label: &'a str,
    ) -> Result<Self, ScheduleTableError> {
        if end <= start {
            return Err(ScheduleTableError::InvalidInterval(
                InvalidInterval::EndBeforeStart,
            ));
        }
        Ok(TimeInterval { start, end, label })
    }
}

impl<'a> PartialEq for TimeInterval<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end && self.label == other.label
    }
}

impl<'a> Eq for TimeInterval<'a> {}

impl<'a> PartialOrd for TimeInterval<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for TimeInterval<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| self.end.cmp(&other.end))
            .then_with(|| self.label.cmp(&other.label))
    }
}

// internal helper to split intervals that span multiple days
pub(crate) struct IntervalSplitter<'a> {
    original: &'a TimeInterval<'a>,
    current_start: Option<NaiveDateTime>,
    end: NaiveDateTime,
}

impl<'a> IntervalSplitter<'a> {
    pub(crate) fn new(interval: &'a TimeInterval<'a>) -> Self {
        Self {
            original: interval,
            current_start: Some(interval.start),
            end: interval.end,
        }
    }
}

impl<'a> Iterator for IntervalSplitter<'a> {
    type Item = TimeInterval<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_start = self.current_start?;

        if current_start > self.end {
            return None;
        }

        let current_date = current_start.date();
        let end_date = self.end.date();

        if current_date < end_date {
            let day_end =
                current_date.and_time(chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap());

            // next start is the beginning of the next day
            self.current_start = Some(
                current_date
                    .succ_opt()? // next day
                    .and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            );

            Some(TimeInterval {
                start: current_start,
                end: day_end,
                label: self.original.label,
            })
        } else {
            self.current_start = None;
            Some(TimeInterval {
                start: current_start,
                end: self.end,
                label: self.original.label,
            })
        }
    }
}
