use attendance_tauri_lib::domain::attendance_schema::AttendanceSummaryRow;
use attendance_tauri_lib::domain::notice_filter::build_notice_rows;
use attendance_tauri_lib::domain::rules::{LogicalOperator, NoticeRules};

#[test]
fn filters_notice_rows_with_and_logic() {
    let rows = vec![AttendanceSummaryRow {
        name: "张三".to_string(),
        need_punch_days: 22,
        expected_punch_count: 44,
        actual_punch_days: 10,
        actual_punch_count: 18,
        absent_days: 12,
        absent_count: 26,
        absent_dates: vec![1, 2, 3],
    }];

    let notices = build_notice_rows(
        &rows,
        &NoticeRules {
            absent_days_threshold: Some(5),
            absent_count_threshold: Some(10),
            operator: LogicalOperator::And,
        },
    );

    assert_eq!(notices.len(), 1);
    assert!(notices[0].trigger_reason.contains("缺勤天数>5"));
    assert!(notices[0].trigger_reason.contains("缺勤次数>10"));
}
