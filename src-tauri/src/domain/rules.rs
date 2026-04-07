use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowRange {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceRules {
    pub am_window: WindowRange,
    pub noon_window: WindowRange,
    pub rest_days: BTreeSet<u32>,
}

impl Default for AttendanceRules {
    fn default() -> Self {
        Self {
            am_window: WindowRange {
                start: NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"),
                end: NaiveTime::from_hms_opt(9, 11, 59).expect("valid time"),
            },
            noon_window: WindowRange {
                start: NaiveTime::from_hms_opt(11, 0, 0).expect("valid time"),
                end: NaiveTime::from_hms_opt(14, 11, 59).expect("valid time"),
            },
            rest_days: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeRules {
    pub absent_days_threshold: Option<u32>,
    pub absent_count_threshold: Option<u32>,
    pub operator: LogicalOperator,
}

impl Default for NoticeRules {
    fn default() -> Self {
        Self {
            absent_days_threshold: None,
            absent_count_threshold: None,
            operator: LogicalOperator::Or,
        }
    }
}
