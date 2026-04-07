use attendance_tauri_lib::application::dto::{BuildNoticeFromSummaryFileRequest, NoticeRulesDto};
use attendance_tauri_lib::application::notice_service::build_notice_list_from_summary_file;
use attendance_tauri_lib::domain::attendance_schema::AttendanceSummaryRow;
use attendance_tauri_lib::domain::notice_filter::build_notice_rows;
use attendance_tauri_lib::domain::rules::{LogicalOperator, NoticeRules};
use attendance_tauri_lib::infrastructure::export_xlsx::export_summary_workbook;
use tempfile::tempdir;

fn create_reference_summary_rows() -> Vec<AttendanceSummaryRow> {
    let mut rows = Vec::new();

    for index in 1..=72 {
        rows.push(AttendanceSummaryRow {
            name: format!("员工{index:02}"),
            need_punch_days: 22,
            expected_punch_count: 44,
            actual_punch_days: 18,
            actual_punch_count: 38,
            absent_days: 4,
            absent_count: 6,
            absent_dates: vec![1, 2, 3, 4],
        });
    }

    for index in 73..=80 {
        rows.push(AttendanceSummaryRow {
            name: format!("员工{index:02}"),
            need_punch_days: 22,
            expected_punch_count: 44,
            actual_punch_days: 22,
            actual_punch_count: 38,
            absent_days: 0,
            absent_count: 6,
            absent_dates: Vec::new(),
        });
    }

    rows.push(AttendanceSummaryRow {
        name: "徐启林".to_string(),
        need_punch_days: 22,
        expected_punch_count: 44,
        actual_punch_days: 21,
        actual_punch_count: 39,
        absent_days: 1,
        absent_count: 5,
        absent_dates: vec![2],
    });

    rows
}

#[test]
fn imported_summary_workbook_builds_expected_notice_counts() {
    let dir = tempdir().expect("tempdir");
    let summary_path = dir.path().join("汇总_202603.xlsx");
    let summary_rows = create_reference_summary_rows();

    export_summary_workbook(&summary_path, &summary_rows, None, None, None)
        .expect("export summary");

    let or_result = build_notice_list_from_summary_file(BuildNoticeFromSummaryFileRequest {
        input_file: summary_path.to_string_lossy().to_string(),
        rules: NoticeRulesDto {
            absent_days_threshold: Some(3),
            absent_count_threshold: Some(5),
            operator: "OR".to_string(),
        },
    })
    .expect("build OR notice");
    assert_eq!(or_result.notice_rows.len(), 80);
    assert!(!or_result.notice_rows.iter().any(|row| row.name == "徐启林"));
    assert!(!or_result.notice_rows.iter().any(|row| row.name == "日期"));
    assert!(!or_result
        .notice_rows
        .iter()
        .any(|row| row.name == "需要打卡日"));

    let and_result = build_notice_list_from_summary_file(BuildNoticeFromSummaryFileRequest {
        input_file: summary_path.to_string_lossy().to_string(),
        rules: NoticeRulesDto {
            absent_days_threshold: Some(3),
            absent_count_threshold: Some(5),
            operator: "AND".to_string(),
        },
    })
    .expect("build AND notice");
    assert_eq!(and_result.notice_rows.len(), 72);
    assert!(!and_result.notice_rows.iter().any(|row| row.name == "日期"));
    assert!(!and_result
        .notice_rows
        .iter()
        .any(|row| row.name == "需要打卡日"));
}

#[test]
fn notice_filter_ignores_reserved_export_header_names() {
    let mut rows = create_reference_summary_rows();
    rows.push(AttendanceSummaryRow {
        name: "日期".to_string(),
        need_punch_days: 22,
        expected_punch_count: 44,
        actual_punch_days: 0,
        actual_punch_count: 0,
        absent_days: 22,
        absent_count: 44,
        absent_dates: (1..=22).collect(),
    });
    rows.push(AttendanceSummaryRow {
        name: "需要打卡日".to_string(),
        need_punch_days: 22,
        expected_punch_count: 44,
        actual_punch_days: 0,
        actual_punch_count: 0,
        absent_days: 22,
        absent_count: 44,
        absent_dates: (1..=22).collect(),
    });

    let notices = build_notice_rows(
        &rows,
        &NoticeRules {
            absent_days_threshold: Some(3),
            absent_count_threshold: Some(5),
            operator: LogicalOperator::Or,
        },
    );

    assert!(!notices.iter().any(|row| row.name == "日期"));
    assert!(!notices.iter().any(|row| row.name == "需要打卡日"));
}
