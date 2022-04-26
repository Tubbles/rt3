use super::Holiday;
use chrono::Datelike;
use holidays_se::next_upcoming_holiday;

pub struct Holidays {}

impl Holiday for Holidays {
    fn is_holiday<D>(date: &D) -> Option<String>
    where
        D: Datelike,
    {
        let (next_holiday, next_date) = next_upcoming_holiday(date);
        if date.ordinal() == next_date.ordinal() {
            return Some(next_holiday.to_string());
        } else {
            None
        }
    }
}
