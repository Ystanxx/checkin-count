use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DailyWindowRecord {
    pub am_times: Vec<NaiveTime>,
    pub noon_times: Vec<NaiveTime>,
}

impl DailyWindowRecord {
    pub fn daily_count(&self) -> u32 {
        (u32::from(!self.am_times.is_empty())) + (u32::from(!self.noon_times.is_empty()))
    }

    pub fn hit_any(&self) -> bool {
        !self.am_times.is_empty() || !self.noon_times.is_empty()
    }
}

pub fn unique_sorted_times(times: impl IntoIterator<Item = NaiveTime>) -> Vec<NaiveTime> {
    let mut values: Vec<NaiveTime> = times.into_iter().collect();
    values.sort();
    values.dedup();
    values
}
