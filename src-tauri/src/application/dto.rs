use crate::domain::attendance_schema::{
    AttendanceDetailRow, AttendanceSummaryRow, NeedPunchDayRow, NoticeRow,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceRulesDto {
    pub am_start: String,
    pub am_end: String,
    pub noon_start: String,
    pub noon_end: String,
    pub rest_days: Vec<u32>,
}

impl Default for AttendanceRulesDto {
    fn default() -> Self {
        Self {
            am_start: "00:00:00".to_string(),
            am_end: "09:11:59".to_string(),
            noon_start: "11:00:00".to_string(),
            noon_end: "14:11:59".to_string(),
            rest_days: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsePreviewRequest {
    pub input_files: Vec<String>,
    pub start_row: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryBuildRequest {
    pub input_files: Vec<String>,
    pub year: i32,
    pub month: u32,
    pub start_row: Option<usize>,
    pub rules: AttendanceRulesDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeRulesDto {
    pub absent_days_threshold: Option<u32>,
    pub absent_count_threshold: Option<u32>,
    pub operator: String,
}

impl Default for NoticeRulesDto {
    fn default() -> Self {
        Self {
            absent_days_threshold: None,
            absent_count_threshold: None,
            operator: "OR".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildNoticeRequest {
    pub rules: NoticeRulesDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSummaryRequest {
    pub output_path: String,
    pub include_detail: bool,
    pub include_need_days: bool,
    pub include_notice: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportNoticeRequest {
    pub output_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewBlock {
    pub name: String,
    pub day_count: usize,
    pub token_count: usize,
    pub source_file: String,
    pub sheet_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksheetPreview {
    pub file_name: String,
    pub sheet_name: String,
    pub row_count: usize,
    pub column_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessStats {
    pub worksheet_count: usize,
    pub recognized_name_count: usize,
    pub block_count: usize,
    pub raw_token_count: usize,
    pub valid_record_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewResponse {
    pub recognized_names: Vec<String>,
    pub worksheet_previews: Vec<WorksheetPreview>,
    pub sample_blocks: Vec<PreviewBlock>,
    pub warnings: Vec<String>,
    pub stats: ProcessStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildSummaryResponse {
    pub summary_rows: Vec<AttendanceSummaryRow>,
    pub detail_rows: Vec<AttendanceDetailRow>,
    pub need_day_rows: Vec<NeedPunchDayRow>,
    pub notice_rows: Vec<NoticeRow>,
    pub warnings: Vec<String>,
    pub stats: ProcessStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeBuildResponse {
    pub notice_rows: Vec<NoticeRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgressEvent {
    pub task_id: String,
    pub stage: String,
    pub percent: u8,
    pub message: String,
}
