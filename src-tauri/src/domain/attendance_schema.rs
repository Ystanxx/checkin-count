use crate::domain::model::DailyWindowRecord;
use crate::domain::rules::LogicalOperator;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetSource {
    pub file_name: String,
    pub file_path: String,
    pub sheet_name: String,
    pub row_count: usize,
    pub column_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksheetData {
    pub source: SheetSource,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProvenance {
    pub file_name: String,
    pub sheet_name: String,
    pub start_row: usize,
    pub date_row: usize,
    pub end_row: usize,
    pub consumed_rows: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonBlock {
    pub name: String,
    pub day_to_tokens: BTreeMap<u32, Vec<String>>,
    pub provenance: BlockProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttendanceWindow {
    Am,
    Noon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedAttendanceRecord {
    pub person_name: String,
    pub day: u32,
    pub normalized_time: NaiveTime,
    pub window: AttendanceWindow,
    pub file_name: String,
    pub sheet_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceDetailRow {
    pub name: String,
    pub date: String,
    pub day: u32,
    pub am_hit: bool,
    pub noon_hit: bool,
    pub daily_count: u32,
    pub am_times: Vec<String>,
    pub noon_times: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceSummaryRow {
    pub name: String,
    pub need_punch_days: u32,
    pub expected_punch_count: u32,
    pub actual_punch_days: u32,
    pub actual_punch_count: u32,
    pub absent_days: u32,
    pub absent_count: u32,
    pub absent_dates: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeedPunchDayRow {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeRow {
    pub name: String,
    pub need_punch_days: u32,
    pub expected_punch_count: u32,
    pub actual_punch_days: u32,
    pub actual_punch_count: u32,
    pub absent_days: u32,
    pub absent_count: u32,
    pub absent_dates: Vec<u32>,
    pub trigger_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateOutput {
    pub detail_rows: Vec<AttendanceDetailRow>,
    pub summary_rows: Vec<AttendanceSummaryRow>,
    pub need_day_rows: Vec<NeedPunchDayRow>,
    pub daily_maps: BTreeMap<String, BTreeMap<u32, DailyWindowRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeEvaluation {
    pub by_days: bool,
    pub by_count: bool,
    pub operator: LogicalOperator,
}
