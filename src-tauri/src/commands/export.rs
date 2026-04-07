use crate::application::dto::{ExportNoticeRequest, ExportSummaryRequest};
use crate::error::UserVisibleError;
use crate::infrastructure::export_csv::{
    export_notice_csv as write_notice_csv, export_summary_csv as write_summary_csv,
};
use crate::infrastructure::export_xlsx::{
    export_notice_workbook as write_notice_workbook,
    export_summary_workbook as write_summary_workbook,
};
use crate::infrastructure::security::prepare_output_path;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn export_summary_xlsx(
    state: State<'_, AppState>,
    request: ExportSummaryRequest,
) -> Result<String, UserVisibleError> {
    let app_state = state.inner().clone();
    let summary = app_state
        .read(|runtime| runtime.summary.clone())
        .ok_or(UserVisibleError {
            code: "STATE_ERROR".to_string(),
            message: "请先生成汇总结果。".to_string(),
        })?;
    let notice_rows = app_state.read(|runtime| runtime.notice.clone());

    let output_path =
        prepare_output_path(&request.output_path, "xlsx").map_err(|error| UserVisibleError {
            code: "SECURITY_ERROR".to_string(),
            message: error.to_string(),
        })?;
    let output_path_for_task = output_path.clone();
    let notice_rows_for_export = if request.include_notice {
        notice_rows.map(|rows| rows.notice_rows)
    } else {
        None
    };

    tauri::async_runtime::spawn_blocking(move || {
        write_summary_workbook(
            &output_path_for_task,
            &summary.summary_rows,
            request
                .include_detail
                .then_some(summary.detail_rows.as_slice()),
            request
                .include_need_days
                .then_some(summary.need_day_rows.as_slice()),
            notice_rows_for_export.as_deref(),
        )
    })
    .await
    .map_err(|error| UserVisibleError {
        code: "TASK_JOIN_ERROR".to_string(),
        message: format!("导出任务执行失败: {error}"),
    })
    .and_then(|result| {
        result.map_err(|error| UserVisibleError {
            code: "EXPORT_XLSX_ERROR".to_string(),
            message: error.user_message(),
        })
    })?;

    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_summary_csv(
    state: State<'_, AppState>,
    request: ExportSummaryRequest,
) -> Result<String, UserVisibleError> {
    let app_state = state.inner().clone();
    let summary = app_state
        .read(|runtime| runtime.summary.clone())
        .ok_or(UserVisibleError {
            code: "STATE_ERROR".to_string(),
            message: "请先生成汇总结果。".to_string(),
        })?;
    let output_path =
        prepare_output_path(&request.output_path, "csv").map_err(|error| UserVisibleError {
            code: "SECURITY_ERROR".to_string(),
            message: error.to_string(),
        })?;
    let output_path_for_task = output_path.clone();

    tauri::async_runtime::spawn_blocking(move || {
        write_summary_csv(&output_path_for_task, &summary.summary_rows)
    })
    .await
    .map_err(|error| UserVisibleError {
        code: "TASK_JOIN_ERROR".to_string(),
        message: format!("导出任务执行失败: {error}"),
    })
    .and_then(|result| {
        result.map_err(|error| UserVisibleError {
            code: "EXPORT_CSV_ERROR".to_string(),
            message: error.user_message(),
        })
    })?;

    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_notice_list(
    state: State<'_, AppState>,
    request: ExportNoticeRequest,
) -> Result<String, UserVisibleError> {
    let app_state = state.inner().clone();
    let notice = app_state
        .read(|runtime| runtime.notice.clone())
        .ok_or(UserVisibleError {
            code: "STATE_ERROR".to_string(),
            message: "请先生成通报名单。".to_string(),
        })?;

    let use_csv = request.output_path.to_ascii_lowercase().ends_with(".csv");
    let output_path = if use_csv {
        prepare_output_path(&request.output_path, "csv")
    } else {
        prepare_output_path(&request.output_path, "xlsx")
    }
    .map_err(|error| UserVisibleError {
        code: "SECURITY_ERROR".to_string(),
        message: error.to_string(),
    })?;
    let output_path_for_task = output_path.clone();

    if use_csv {
        tauri::async_runtime::spawn_blocking(move || {
            write_notice_csv(&output_path_for_task, &notice.notice_rows)
        })
        .await
        .map_err(|error| UserVisibleError {
            code: "TASK_JOIN_ERROR".to_string(),
            message: format!("导出任务执行失败: {error}"),
        })
        .and_then(|result| {
            result.map_err(|error| UserVisibleError {
                code: "EXPORT_CSV_ERROR".to_string(),
                message: error.user_message(),
            })
        })?;
    } else {
        tauri::async_runtime::spawn_blocking(move || {
            write_notice_workbook(&output_path_for_task, &notice.notice_rows)
        })
        .await
        .map_err(|error| UserVisibleError {
            code: "TASK_JOIN_ERROR".to_string(),
            message: format!("导出任务执行失败: {error}"),
        })
        .and_then(|result| {
            result.map_err(|error| UserVisibleError {
                code: "EXPORT_XLSX_ERROR".to_string(),
                message: error.user_message(),
            })
        })?;
    }

    Ok(output_path.to_string_lossy().to_string())
}
