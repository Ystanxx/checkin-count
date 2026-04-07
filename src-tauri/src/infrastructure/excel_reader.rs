use crate::domain::attendance_schema::{SheetSource, WorksheetData};
use crate::infrastructure::security::redact_path;
use calamine::{open_workbook_auto, Reader};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadWorkbookResult {
    pub worksheets: Vec<WorksheetData>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ExcelReadError {
    #[error("未提供输入文件")]
    EmptySelection,
}

impl ExcelReadError {
    pub fn user_message(&self) -> String {
        match self {
            Self::EmptySelection => "请先选择至少一个 Excel 文件。".to_string(),
        }
    }
}

pub fn read_workbooks(paths: &[PathBuf]) -> Result<ReadWorkbookResult, ExcelReadError> {
    if paths.is_empty() {
        return Err(ExcelReadError::EmptySelection);
    }

    let mut worksheets = Vec::new();
    let mut warnings = Vec::new();

    for path in paths {
        let file_name = redact_path(path);
        let mut workbook = match open_workbook_auto(path) {
            Ok(workbook) => workbook,
            Err(error) => {
                warnings.push(format!("无法打开文件 {file_name}: {error}"));
                continue;
            }
        };

        for sheet_name in workbook.sheet_names().to_owned() {
            match workbook.worksheet_range(&sheet_name) {
                Ok(range) => {
                    let rows: Vec<Vec<String>> = range
                        .rows()
                        .map(|row| row.iter().map(|cell| cell.to_string()).collect())
                        .collect();
                    let column_count = rows.iter().map(|row| row.len()).max().unwrap_or(0);
                    let row_count = rows.len();

                    worksheets.push(WorksheetData {
                        source: SheetSource {
                            file_name: file_name.clone(),
                            file_path: path.to_string_lossy().to_string(),
                            sheet_name: sheet_name.clone(),
                            row_count,
                            column_count,
                        },
                        rows,
                    });
                }
                Err(error) => {
                    warnings.push(format!("无法读取工作表 {file_name}/{sheet_name}: {error}"));
                }
            }
        }
    }

    Ok(ReadWorkbookResult { worksheets, warnings })
}
