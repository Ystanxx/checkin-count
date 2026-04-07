use crate::domain::attendance_schema::WorksheetData;
use crate::domain::time_normalizer::to_ascii_fullwidth;

const SUMMARY_HEADERS: [&str; 8] = [
    "姓名",
    "需要打卡日",
    "应打卡次数",
    "打卡天数",
    "打卡次数",
    "缺勤天数",
    "缺勤次数",
    "缺勤具体日期",
];

const DETAIL_HEADERS: [&str; 8] = [
    "姓名",
    "日期",
    "日",
    "AM命中",
    "NOON命中",
    "当日计次",
    "AM时间列表",
    "NOON时间列表",
];

const NEED_DAYS_HEADERS: [&str; 3] = ["年份", "月份", "需要打卡日"];

const NOTICE_HEADERS: [&str; 9] = [
    "姓名",
    "需要打卡日",
    "应打卡次数",
    "打卡天数",
    "打卡次数",
    "缺勤天数",
    "缺勤次数",
    "缺勤具体日期",
    "触发原因",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportedSheetKind {
    Summary,
    Detail,
    NeedDays,
    Notice,
}

impl ExportedSheetKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Summary => "汇总",
            Self::Detail => "明细",
            Self::NeedDays => "需要打卡日",
            Self::Notice => "通报名单",
        }
    }
}

pub fn detect_exported_workbook(worksheets: &[WorksheetData]) -> Option<ExportedSheetKind> {
    worksheets.iter().find_map(detect_exported_sheet_kind)
}

pub fn detect_exported_sheet_kind(worksheet: &WorksheetData) -> Option<ExportedSheetKind> {
    worksheet.rows.iter().find_map(|row| {
        let normalized = normalize_row(row);
        if row_starts_with_headers(&normalized, &SUMMARY_HEADERS) {
            return Some(ExportedSheetKind::Summary);
        }
        if row_starts_with_headers(&normalized, &DETAIL_HEADERS) {
            return Some(ExportedSheetKind::Detail);
        }
        if row_starts_with_headers(&normalized, &NEED_DAYS_HEADERS) {
            return Some(ExportedSheetKind::NeedDays);
        }
        if row_starts_with_headers(&normalized, &NOTICE_HEADERS) {
            return Some(ExportedSheetKind::Notice);
        }
        None
    })
}

pub fn is_export_header_term(value: &str) -> bool {
    let normalized = normalize_cell(value);
    SUMMARY_HEADERS
        .iter()
        .chain(DETAIL_HEADERS.iter())
        .chain(NEED_DAYS_HEADERS.iter())
        .chain(NOTICE_HEADERS.iter())
        .any(|header| normalized == *header)
}

fn row_starts_with_headers(row: &[String], headers: &[&str]) -> bool {
    row.len() >= headers.len()
        && headers
            .iter()
            .enumerate()
            .all(|(index, header)| row.get(index).map(|cell| cell.as_str()) == Some(*header))
}

fn normalize_row(row: &[String]) -> Vec<String> {
    row.iter()
        .map(|cell| normalize_cell(cell))
        .filter(|cell| !cell.is_empty())
        .collect()
}

fn normalize_cell(value: &str) -> String {
    to_ascii_fullwidth(value)
        .replace(['：', ':'], "")
        .trim()
        .to_string()
}
