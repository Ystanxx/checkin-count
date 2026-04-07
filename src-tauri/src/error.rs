use crate::domain::error::DomainError;
use crate::infrastructure::excel_reader::ExcelReadError;
use crate::infrastructure::export_csv::ExportCsvError;
use crate::infrastructure::export_xlsx::ExportXlsxError;
use crate::infrastructure::security::SecurityError;
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    State(String),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    ExcelRead(#[from] ExcelReadError),
    #[error(transparent)]
    ExportXlsx(#[from] ExportXlsxError),
    #[error(transparent)]
    ExportCsv(#[from] ExportCsvError),
    #[error(transparent)]
    Security(#[from] SecurityError),
}

#[derive(Debug, Clone, Serialize)]
pub struct UserVisibleError {
    pub code: String,
    pub message: String,
}

impl AppError {
    pub fn to_user_visible(&self) -> UserVisibleError {
        match self {
            Self::Validation(message) => UserVisibleError {
                code: "VALIDATION_ERROR".to_string(),
                message: message.clone(),
            },
            Self::State(message) => UserVisibleError {
                code: "STATE_ERROR".to_string(),
                message: message.clone(),
            },
            Self::Domain(error) => UserVisibleError {
                code: "DOMAIN_ERROR".to_string(),
                message: error.to_string(),
            },
            Self::ExcelRead(error) => UserVisibleError {
                code: "EXCEL_READ_ERROR".to_string(),
                message: error.user_message(),
            },
            Self::ExportXlsx(error) => UserVisibleError {
                code: "EXPORT_XLSX_ERROR".to_string(),
                message: error.user_message(),
            },
            Self::ExportCsv(error) => UserVisibleError {
                code: "EXPORT_CSV_ERROR".to_string(),
                message: error.user_message(),
            },
            Self::Security(error) => UserVisibleError {
                code: "SECURITY_ERROR".to_string(),
                message: error.to_string(),
            },
        }
    }
}
