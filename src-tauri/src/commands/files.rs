use crate::error::UserVisibleError;
use tauri_plugin_dialog::{DialogExt, FilePath};

#[tauri::command]
pub async fn select_input_files(app: tauri::AppHandle) -> Result<Vec<String>, UserVisibleError> {
    let (sender, receiver) = tokio::sync::oneshot::channel::<Vec<String>>();

    app.dialog()
        .file()
        .add_filter("Excel", &["xls", "xlsx", "xlsm"])
        .pick_files(move |files| {
            let paths = files
                .unwrap_or_default()
                .into_iter()
                .filter_map(|file| match file {
                    FilePath::Path(path) => Some(path.to_string_lossy().to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let _ = sender.send(paths);
        });

    receiver.await.map_err(|_| UserVisibleError {
        code: "DIALOG_ERROR".to_string(),
        message: "文件选择对话框未正常返回。".to_string(),
    })
}
