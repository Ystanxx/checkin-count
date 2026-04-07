pub mod application;
pub mod commands;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod state;

use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    infrastructure::logging::init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::select_input_files,
            commands::parse_attendance_preview,
            commands::build_summary,
            commands::build_notice_list,
            commands::export_summary_xlsx,
            commands::export_summary_csv,
            commands::export_notice_list
        ])
        .setup(|app| {
            let _ = app.emit("task://ready", serde_json::json!({ "ready": true }));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("tauri application run failed");
}
