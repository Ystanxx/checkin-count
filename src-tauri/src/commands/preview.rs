use crate::application::app_service;
use crate::application::dto::{
    BuildNoticeRequest, ParsePreviewRequest, SummaryBuildRequest, TaskProgressEvent,
};
use crate::application::notice_service;
use crate::error::UserVisibleError;
use crate::state::AppState;
use tauri::{Emitter, State};

#[tauri::command]
pub async fn parse_attendance_preview(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    request: ParsePreviewRequest,
) -> Result<crate::application::dto::PreviewResponse, UserVisibleError> {
    let app_handle = app.clone();
    let app_state = state.inner().clone();
    let response = tauri::async_runtime::spawn_blocking(move || {
        app_service::parse_preview(request, &|event| emit_progress(&app_handle, event))
    })
    .await
    .map_err(|error| UserVisibleError {
        code: "TASK_JOIN_ERROR".to_string(),
        message: format!("预览任务执行失败: {error}"),
    })
    .and_then(|result| result.map_err(|error| error.to_user_visible()))?;

    app_state.write(|runtime| {
        runtime.preview = Some(response.clone());
    });

    Ok(response)
}

#[tauri::command]
pub async fn build_summary(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    request: SummaryBuildRequest,
) -> Result<crate::application::dto::BuildSummaryResponse, UserVisibleError> {
    let app_handle = app.clone();
    let app_state = state.inner().clone();
    let response = tauri::async_runtime::spawn_blocking(move || {
        app_service::build_summary(request, &|event| emit_progress(&app_handle, event))
    })
    .await
    .map_err(|error| UserVisibleError {
        code: "TASK_JOIN_ERROR".to_string(),
        message: format!("汇总任务执行失败: {error}"),
    })
    .and_then(|result| result.map_err(|error| error.to_user_visible()))?;

    app_state.write(|runtime| {
        runtime.summary = Some(response.clone());
        runtime.notice = None;
    });

    Ok(response)
}

#[tauri::command]
pub async fn build_notice_list(
    state: State<'_, AppState>,
    request: BuildNoticeRequest,
) -> Result<crate::application::dto::NoticeBuildResponse, UserVisibleError> {
    let app_state = state.inner().clone();
    let summary = app_state.read(|runtime| runtime.summary.clone()).ok_or(UserVisibleError {
        code: "STATE_ERROR".to_string(),
        message: "请先生成汇总结果。".to_string(),
    })?;

    let response = tauri::async_runtime::spawn_blocking(move || {
        notice_service::build_notice_list(&summary.summary_rows, request)
    })
    .await
    .map_err(|error| UserVisibleError {
        code: "TASK_JOIN_ERROR".to_string(),
        message: format!("通报名单任务执行失败: {error}"),
    })
    .and_then(|result| result.map_err(|error| error.to_user_visible()))?;

    app_state.write(|runtime| {
        runtime.notice = Some(response.clone());
        if let Some(summary) = runtime.summary.as_mut() {
            summary.notice_rows = response.notice_rows.clone();
        }
    });

    Ok(response)
}

fn emit_progress(app: &tauri::AppHandle, event: TaskProgressEvent) {
    let _ = app.emit("task://progress", event);
}
