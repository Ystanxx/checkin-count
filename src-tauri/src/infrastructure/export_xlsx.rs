use crate::domain::attendance_schema::{
    AttendanceDetailRow, AttendanceSummaryRow, NeedPunchDayRow, NoticeRow,
};
use rust_xlsxwriter::{Workbook, XlsxError};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExportXlsxError {
    #[error("输出目录创建失败: {0}")]
    CreateDir(String),
    #[error(transparent)]
    Writer(#[from] XlsxError),
}

impl ExportXlsxError {
    pub fn user_message(&self) -> String {
        match self {
            Self::CreateDir(_) => "无法创建导出目录，请检查输出位置。".to_string(),
            Self::Writer(_) => "导出 Excel 失败，请确认文件未被占用。".to_string(),
        }
    }
}

pub fn export_summary_workbook(
    output_path: &Path,
    summary_rows: &[AttendanceSummaryRow],
    detail_rows: Option<&[AttendanceDetailRow]>,
    need_day_rows: Option<&[NeedPunchDayRow]>,
    notice_rows: Option<&[NoticeRow]>,
) -> Result<(), ExportXlsxError> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| ExportXlsxError::CreateDir(error.to_string()))?;
    }

    let mut workbook = Workbook::new();
    write_summary_sheet(&mut workbook, summary_rows)?;
    if let Some(rows) = detail_rows {
        write_detail_sheet(&mut workbook, rows)?;
    }
    if let Some(rows) = need_day_rows {
        write_need_days_sheet(&mut workbook, rows)?;
    }
    if let Some(rows) = notice_rows {
        write_notice_sheet(&mut workbook, rows)?;
    }

    workbook.save(output_path)?;
    Ok(())
}

pub fn export_notice_workbook(
    output_path: &Path,
    notice_rows: &[NoticeRow],
) -> Result<(), ExportXlsxError> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| ExportXlsxError::CreateDir(error.to_string()))?;
    }

    let mut workbook = Workbook::new();
    write_notice_sheet(&mut workbook, notice_rows)?;
    workbook.save(output_path)?;
    Ok(())
}

fn write_summary_sheet(
    workbook: &mut Workbook,
    rows: &[AttendanceSummaryRow],
) -> Result<(), XlsxError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("汇总")?;
    let headers = [
        "姓名",
        "需要打卡日",
        "应打卡次数",
        "打卡天数",
        "打卡次数",
        "缺勤天数",
        "缺勤次数",
        "缺勤具体日期",
    ];
    write_headers(worksheet, &headers)?;

    for (row_index, row) in rows.iter().enumerate() {
        let line = [
            row.name.clone(),
            row.need_punch_days.to_string(),
            row.expected_punch_count.to_string(),
            row.actual_punch_days.to_string(),
            row.actual_punch_count.to_string(),
            row.absent_days.to_string(),
            row.absent_count.to_string(),
            join_days(&row.absent_dates),
        ];
        write_line(worksheet, row_index + 1, &line)?;
    }
    Ok(())
}

fn write_detail_sheet(
    workbook: &mut Workbook,
    rows: &[AttendanceDetailRow],
) -> Result<(), XlsxError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("明细")?;
    let headers = [
        "姓名",
        "日期",
        "日",
        "AM命中",
        "NOON命中",
        "当日计次",
        "AM时间列表",
        "NOON时间列表",
    ];
    write_headers(worksheet, &headers)?;

    for (row_index, row) in rows.iter().enumerate() {
        let line = [
            row.name.clone(),
            row.date.clone(),
            row.day.to_string(),
            bool_to_cn(row.am_hit),
            bool_to_cn(row.noon_hit),
            row.daily_count.to_string(),
            row.am_times.join(","),
            row.noon_times.join(","),
        ];
        write_line(worksheet, row_index + 1, &line)?;
    }
    Ok(())
}

fn write_need_days_sheet(
    workbook: &mut Workbook,
    rows: &[NeedPunchDayRow],
) -> Result<(), XlsxError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("需要打卡日")?;
    let headers = ["年份", "月份", "需要打卡日"];
    write_headers(worksheet, &headers)?;

    for (row_index, row) in rows.iter().enumerate() {
        let line = [
            row.year.to_string(),
            row.month.to_string(),
            row.day.to_string(),
        ];
        write_line(worksheet, row_index + 1, &line)?;
    }
    Ok(())
}

fn write_notice_sheet(workbook: &mut Workbook, rows: &[NoticeRow]) -> Result<(), XlsxError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("通报名单")?;
    let headers = [
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
    write_headers(worksheet, &headers)?;

    for (row_index, row) in rows.iter().enumerate() {
        let line = [
            row.name.clone(),
            row.need_punch_days.to_string(),
            row.expected_punch_count.to_string(),
            row.actual_punch_days.to_string(),
            row.actual_punch_count.to_string(),
            row.absent_days.to_string(),
            row.absent_count.to_string(),
            join_days(&row.absent_dates),
            row.trigger_reason.clone(),
        ];
        write_line(worksheet, row_index + 1, &line)?;
    }
    Ok(())
}

fn write_headers(
    worksheet: &mut rust_xlsxwriter::Worksheet,
    headers: &[&str],
) -> Result<(), XlsxError> {
    for (column_index, header) in headers.iter().enumerate() {
        worksheet.write_string(0, column_index as u16, *header)?;
    }
    Ok(())
}

fn write_line(
    worksheet: &mut rust_xlsxwriter::Worksheet,
    row_index: usize,
    values: &[String],
) -> Result<(), XlsxError> {
    for (column_index, value) in values.iter().enumerate() {
        worksheet.write_string(row_index as u32, column_index as u16, value)?;
    }
    Ok(())
}

fn join_days(days: &[u32]) -> String {
    days.iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn bool_to_cn(value: bool) -> String {
    if value {
        "是".to_string()
    } else {
        "否".to_string()
    }
}
