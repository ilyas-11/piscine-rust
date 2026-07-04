use chrono::*;

pub fn middle_day(year: u32) -> Option<Weekday> {
    let first = NaiveDate::from_ymd_opt(year as i32, 1, 1)?;

    if first.leap_year() {
        return None;
    }

    let middle = first + Duration::days(182);

    Some(middle.weekday())
}
