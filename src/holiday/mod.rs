use chrono::Datelike;

pub trait Holiday {
    fn is_holiday<D>(date: &D) -> Option<String>
    where
        D: Datelike;
}

pub mod se;
