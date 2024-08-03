use chrono::{Datelike, NaiveDate, ParseError};

pub fn parse_date(date: &str) -> Result<NaiveDate, ParseError> {
    let trimmed = &date[1..];
    Ok(NaiveDate::parse_from_str(&trimmed, "%Y-%m-%d")?)
}

pub fn add_days(date: NaiveDate, days: i64) -> NaiveDate {
    date + chrono::Duration::days(days)
}

pub fn substract_days(date: NaiveDate, days: i64) -> NaiveDate {
    date - chrono::Duration::days(days)
}

pub fn substract_dates(date1: NaiveDate, date2: NaiveDate) -> i64 {
    (date1 - date2).num_days()
}

pub fn day_of_week(date: NaiveDate) -> i64 {
    date.weekday().num_days_from_sunday() as i64
}

pub fn day_of_year(date: NaiveDate) -> i64 {
    date.ordinal() as i64
}
pub fn day_of_month(date: NaiveDate) -> i64 {
    date.day() as i64
}

pub fn week_of_month(date: NaiveDate) -> i64 {
    ((date.day() - 1) / 7 + 1).into()
}

pub fn get_today() -> NaiveDate {
    chrono::Local::now().naive_local().date()
}

pub fn week_of_year(date: NaiveDate) -> i64 {
    date.iso_week().week() as i64
}

pub fn month(date: NaiveDate) -> i64 {
    date.month() as i64
}

pub fn year(date: NaiveDate) -> i64 {
    date.year() as i64
}
