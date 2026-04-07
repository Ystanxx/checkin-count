use std::path::{Path, PathBuf};
use thiserror::Error;

const ALLOWED_INPUT_EXTENSIONS: &[&str] = &["xls", "xlsx", "xlsm"];

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("未选择任何输入文件")]
    EmptyInput,
    #[error("输入文件格式不受支持: {0}")]
    UnsupportedInput(String),
    #[error("输出路径无效")]
    InvalidOutputPath,
    #[error("输出扩展名不匹配，期望 .{0}")]
    InvalidOutputExtension(String),
}

pub fn validate_input_paths(paths: &[String]) -> Result<Vec<PathBuf>, SecurityError> {
    if paths.is_empty() {
        return Err(SecurityError::EmptyInput);
    }

    let mut validated = Vec::new();
    for path in paths {
        let path_buf = PathBuf::from(path);
        let extension = path_buf
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .ok_or_else(|| SecurityError::UnsupportedInput(redact_path(&path_buf)))?;

        if !ALLOWED_INPUT_EXTENSIONS.contains(&extension.as_str()) {
            return Err(SecurityError::UnsupportedInput(redact_path(&path_buf)));
        }

        validated.push(path_buf);
    }

    Ok(validated)
}

pub fn prepare_output_path(
    raw_path: &str,
    expected_extension: &str,
) -> Result<PathBuf, SecurityError> {
    let input_path = PathBuf::from(raw_path);
    let parent = input_path
        .parent()
        .ok_or(SecurityError::InvalidOutputPath)?;
    let stem = input_path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("导出结果");

    let mut output_path = parent.join(sanitize_file_name(stem));
    output_path.set_extension(expected_extension);

    let extension = output_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if extension != expected_extension.to_ascii_lowercase() {
        return Err(SecurityError::InvalidOutputExtension(
            expected_extension.to_string(),
        ));
    }

    Ok(output_path)
}

pub fn redact_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<未知文件>")
        .to_string()
}

pub fn sanitize_file_name(input: &str) -> String {
    let sanitized: String = input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '(' | ')' | '[' | ']' | ' ') {
                ch
            } else if ('一'..='龥').contains(&ch) {
                ch
            } else {
                '_'
            }
        })
        .collect();

    let trimmed = sanitized.trim_matches('_').trim();
    if trimmed.is_empty() {
        "导出结果".to_string()
    } else {
        trimmed.to_string()
    }
}
