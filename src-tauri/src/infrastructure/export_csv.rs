use crate::domain::attendance_schema::{AttendanceSummaryRow, NoticeRow};
use csv::WriterBuilder;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExportCsvError {
    #[error("输出目录创建失败: {0}")]
    CreateDir(String),
    #[error("CSV 写入失败: {0}")]
    Write(String),
}

impl ExportCsvError {
    pub fn user_message(&self) -> String {
        match self {
            Self::CreateDir(_) => "无法创建 CSV 导出目录，请检查输出位置。".to_string(),
            Self::Write(_) => "CSV 导出失败，请确认文件未被占用。".to_string(),
        }
    }
}

pub fn export_summary_csv(output_path: &Path, rows: &[AttendanceSummaryRow]) -> Result<(), ExportCsvError> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| ExportCsvError::CreateDir(error.to_string()))?;
    }

    let mut file = File::create(output_path).map_err(|error| ExportCsvError::Write(error.to_string()))?;
    file.write_all(&[0xEF, 0xBB, 0xBF])
        .map_err(|error| ExportCsvError::Write(error.to_string()))?;

    let mut writer = WriterBuilder::new().from_writer(file);
    writer
        .write_record([
            "姓名",
            "需要打卡日",
            "应打卡次数",
            "打卡天数",
            "打卡次数",
            "缺勤天数",
            "缺勤次数",
            "缺勤具体日期",
        ])
        .map_err(|error| ExportCsvError::Write(error.to_string()))?;

    for row in rows {
        writer
            .write_record([
                row.name.as_str(),
                &row.need_punch_days.to_string(),
                &row.expected_punch_count.to_string(),
                &row.actual_punch_days.to_string(),
                &row.actual_punch_count.to_string(),
                &row.absent_days.to_string(),
                &row.absent_count.to_string(),
                &join_days(&row.absent_dates),
            ])
            .map_err(|error| ExportCsvError::Write(error.to_string()))?;
    }

    writer.flush().map_err(|error| ExportCsvError::Write(error.to_string()))?;
    Ok(())
}

pub fn export_notice_csv(output_path: &Path, rows: &[NoticeRow]) -> Result<(), ExportCsvError> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| ExportCsvError::CreateDir(error.to_string()))?;
    }

    let mut file = File::create(output_path).map_err(|error| ExportCsvError::Write(error.to_string()))?;
    file.write_all(&[0xEF, 0xBB, 0xBF])
        .map_err(|error| ExportCsvError::Write(error.to_string()))?;

    let mut writer = WriterBuilder::new().from_writer(file);
    writer
        .write_record([
            "姓名",
            "需要打卡日",
            "应打卡次数",
            "打卡天数",
            "打卡次数",
            "缺勤天数",
            "缺勤次数",
            "缺勤具体日期",
            "触发原因",
        ])
        .map_err(|error| ExportCsvError::Write(error.to_string()))?;

    for row in rows {
        writer
            .write_record([
                row.name.as_str(),
                &row.need_punch_days.to_string(),
                &row.expected_punch_count.to_string(),
                &row.actual_punch_days.to_string(),
                &row.actual_punch_count.to_string(),
                &row.absent_days.to_string(),
                &row.absent_count.to_string(),
                &join_days(&row.absent_dates),
                row.trigger_reason.as_str(),
            ])
            .map_err(|error| ExportCsvError::Write(error.to_string()))?;
    }

    writer.flush().map_err(|error| ExportCsvError::Write(error.to_string()))?;
    Ok(())
}

fn join_days(days: &[u32]) -> String {
    days.iter().map(u32::to_string).collect::<Vec<_>>().join(",")
}
