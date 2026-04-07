use crate::domain::attendance_schema::AttendanceSummaryRow;
use crate::domain::time_normalizer::to_ascii_fullwidth;
use crate::infrastructure::excel_reader::read_workbooks;
use crate::infrastructure::exported_workbook_detector::detect_exported_sheet_kind;
use std::path::PathBuf;

pub fn import_summary_rows(path: PathBuf) -> Result<Vec<AttendanceSummaryRow>, String> {
    let read_result = read_workbooks(&[path]).map_err(|error| error.user_message())?;
    let worksheet = read_result
        .worksheets
        .iter()
        .find(|sheet| sheet.source.sheet_name == "汇总")
        .or_else(|| {
            read_result.worksheets.iter().find(|sheet| {
                detect_exported_sheet_kind(sheet)
                    == Some(crate::infrastructure::exported_workbook_detector::ExportedSheetKind::Summary)
            })
        })
        .ok_or_else(|| "未找到“汇总”sheet，无法从汇总文件生成通报名单。".to_string())?;

    let header_row_index = worksheet
        .rows
        .iter()
        .position(|row| {
            let normalized = row
                .iter()
                .map(|cell| normalize_cell(cell))
                .filter(|cell| !cell.is_empty())
                .collect::<Vec<_>>();
            normalized.starts_with(&[
                "姓名".to_string(),
                "需要打卡日".to_string(),
                "应打卡次数".to_string(),
                "打卡天数".to_string(),
                "打卡次数".to_string(),
                "缺勤天数".to_string(),
                "缺勤次数".to_string(),
                "缺勤具体日期".to_string(),
            ])
        })
        .ok_or_else(|| "汇总文件缺少正确表头，无法导入。".to_string())?;

    let mut rows = Vec::new();
    for row in worksheet.rows.iter().skip(header_row_index + 1) {
        let values = row
            .iter()
            .map(|cell| normalize_cell(cell))
            .collect::<Vec<_>>();
        if values.iter().all(|value| value.is_empty()) {
            continue;
        }

        let name = values.first().cloned().unwrap_or_default();
        if name.is_empty() {
            continue;
        }

        rows.push(AttendanceSummaryRow {
            name,
            need_punch_days: parse_u32(values.get(1))?,
            expected_punch_count: parse_u32(values.get(2))?,
            actual_punch_days: parse_u32(values.get(3))?,
            actual_punch_count: parse_u32(values.get(4))?,
            absent_days: parse_u32(values.get(5))?,
            absent_count: parse_u32(values.get(6))?,
            absent_dates: parse_days(values.get(7)),
        });
    }

    Ok(rows)
}

fn normalize_cell(value: &str) -> String {
    to_ascii_fullwidth(value).trim().to_string()
}

fn parse_u32(value: Option<&String>) -> Result<u32, String> {
    value
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .ok_or_else(|| "汇总文件字段缺失，无法导入。".to_string())?
        .parse::<u32>()
        .map_err(|_| "汇总文件字段格式非法，无法导入。".to_string())
}

fn parse_days(value: Option<&String>) -> Vec<u32> {
    value
        .map(|item| {
            item.split([',', '，', ' ', ';', '；'])
                .filter_map(|token| token.trim().parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}
