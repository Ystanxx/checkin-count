use attendance_tauri_lib::domain::attendance_schema::{AttendanceSummaryRow, NoticeRow};
use attendance_tauri_lib::infrastructure::export_csv::{export_notice_csv, export_summary_csv};
use attendance_tauri_lib::infrastructure::export_xlsx::{
    export_notice_workbook, export_summary_workbook,
};
use calamine::{open_workbook_auto, Reader};
use std::fs;
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

    export_summary_workbook(&xlsx_path, &summary_rows, None, None, Some(&notice_rows))
        .expect("xlsx");
    export_summary_csv(&csv_path, &summary_rows).expect("csv");
    export_notice_workbook(&notice_path, &notice_rows).expect("notice xlsx");
    export_notice_csv(&dir.path().join("notice.csv"), &notice_rows).expect("notice csv");

    assert!(xlsx_path.exists());
    assert!(csv_path.exists());
    assert!(notice_path.exists());
}

#[test]
fn exported_notice_files_only_contain_real_person_names() {
    let dir = tempdir().expect("tempdir");
    let notice_xlsx = dir.path().join("notice.xlsx");
    let notice_csv = dir.path().join("notice.csv");
    let notice_rows = vec![
        NoticeRow {
            name: "张三".to_string(),
            need_punch_days: 22,
            expected_punch_count: 44,
            actual_punch_days: 18,
            actual_punch_count: 38,
            absent_days: 4,
            absent_count: 6,
            absent_dates: vec![1, 2, 3, 4],
            trigger_reason: "缺勤天数>3 / 缺勤次数>5".to_string(),
        },
        NoticeRow {
            name: "李四".to_string(),
            need_punch_days: 22,
            expected_punch_count: 44,
            actual_punch_days: 22,
            actual_punch_count: 38,
            absent_days: 0,
            absent_count: 6,
            absent_dates: Vec::new(),
            trigger_reason: "缺勤次数>5".to_string(),
        },
    ];

    export_notice_workbook(&notice_xlsx, &notice_rows).expect("notice xlsx");
    export_notice_csv(&notice_csv, &notice_rows).expect("notice csv");

    let mut workbook = open_workbook_auto(&notice_xlsx).expect("open notice xlsx");
    let range = workbook.worksheet_range("通报名单").expect("notice sheet");
    let xlsx_names = range
        .rows()
        .skip(1)
        .filter_map(|row| row.first())
        .map(|cell| cell.to_string())
        .collect::<Vec<_>>();

    let csv_text = fs::read(&notice_csv).expect("read csv");
    let csv_text = String::from_utf8(csv_text[3..].to_vec()).expect("utf8 csv");
    let csv_names = csv_text
        .lines()
        .skip(1)
        .filter_map(|line| line.split(',').next())
        .map(str::to_string)
        .collect::<Vec<_>>();

    assert_eq!(xlsx_names, vec!["张三".to_string(), "李四".to_string()]);
    assert_eq!(csv_names, vec!["张三".to_string(), "李四".to_string()]);
    assert!(!xlsx_names
        .iter()
        .any(|name| name == "日期" || name == "需要打卡日"));
    assert!(!csv_names
        .iter()
        .any(|name| name == "日期" || name == "需要打卡日"));
}
