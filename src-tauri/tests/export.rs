use attendance_tauri_lib::domain::attendance_schema::{AttendanceSummaryRow, NoticeRow};
use attendance_tauri_lib::infrastructure::export_csv::{export_notice_csv, export_summary_csv};
use attendance_tauri_lib::infrastructure::export_xlsx::{export_notice_workbook, export_summary_workbook};
use tempfile::tempdir;

#[test]
fn export_summary_and_notice_smoke() {
    let dir = tempdir().expect("tempdir");
    let xlsx_path = dir.path().join("summary.xlsx");
    let csv_path = dir.path().join("summary.csv");
    let notice_path = dir.path().join("notice.xlsx");

    let summary_rows = vec![AttendanceSummaryRow {
        name: "张三".to_string(),
        need_punch_days: 22,
        expected_punch_count: 44,
        actual_punch_days: 20,
        actual_punch_count: 39,
        absent_days: 2,
        absent_count: 5,
        absent_dates: vec![2, 9],
    }];
    let notice_rows = vec![NoticeRow {
        name: "张三".to_string(),
        need_punch_days: 22,
        expected_punch_count: 44,
        actual_punch_days: 20,
        actual_punch_count: 39,
        absent_days: 2,
        absent_count: 5,
        absent_dates: vec![2, 9],
        trigger_reason: "缺勤次数>4".to_string(),
    }];

    export_summary_workbook(&xlsx_path, &summary_rows, None, None, Some(&notice_rows)).expect("xlsx");
    export_summary_csv(&csv_path, &summary_rows).expect("csv");
    export_notice_workbook(&notice_path, &notice_rows).expect("notice xlsx");
    export_notice_csv(&dir.path().join("notice.csv"), &notice_rows).expect("notice csv");

    assert!(xlsx_path.exists());
    assert!(csv_path.exists());
    assert!(notice_path.exists());
}
