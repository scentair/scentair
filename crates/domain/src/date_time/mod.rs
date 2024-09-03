#[macro_export]
macro_rules! date_time {
    ($year:expr, $month:expr, $day:expr, $hour:expr, $minute:expr, $second:expr) => {
        chrono::NaiveDateTime::new(ymd!($year, $month, $day), hms!($hour, $minute, $second))
    };
}

macro_rules! with_unwrap {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => std::unreachable!(),
        }
    };
}

#[macro_export]
macro_rules! ymd {
    ($year:expr, $month:expr, $day:expr) => {
        with_unwrap!(chrono::NaiveDate::from_ymd_opt($year, $month, $day))
    };
}

#[macro_export]
macro_rules! hms {
    ($hour:expr, $minute:expr, $second:expr) => {
        with_unwrap!(chrono::NaiveTime::from_hms_opt($hour, $minute, $second))
    };
}

pub mod deleted_at {
    use chrono::{DateTime, NaiveDate, NaiveDateTime};

    pub const DEFAULT: NaiveDateTime = date_time!(2199, 12, 31, 0, 0, 0);
}
