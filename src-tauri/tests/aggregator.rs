use attendance_tauri_lib::domain::aggregator::aggregate_records;
use attendance_tauri_lib::domain::attendance_schema::{AttendanceWindow, NormalizedAttendanceRecord};
use attendance_tauri_lib::domain::rules::AttendanceRules;
use chrono::NaiveTime;

#[test]
fn keeps_zero_punch_people_in_summary() {
    let records = vec![NormalizedAttendanceRecord {
        person_name: "张三".to_string(),
        day: 1,
        normalized_time: NaiveTime::from_hms_opt(8, 30, 0).expect("time"),
        window: AttendanceWindow::Am,
        file_name: "a.xlsx".to_string(),
        sheet_name: "打卡".to_string(),
    }];

    let aggregate = aggregate_records(
        &records,
        &["张三".to_string(), "李四".to_string()],
        2026,
        4,
        &AttendanceRules::default(),
    )
    .expect("aggregate");

    assert!(aggregate.summary_rows.iter().any(|row| row.name == "李四" && row.actual_punch_count == 0));
}

#[test]
fn deduplicates_same_day_same_window_hits() {
    let time = NaiveTime::from_hms_opt(8, 30, 0).expect("time");
    let records = vec![
        NormalizedAttendanceRecord {
            person_name: "张三".to_string(),
            day: 1,
            normalized_time: time,
            window: AttendanceWindow::Am,
            file_name: "a.xlsx".to_string(),
            sheet_name: "打卡".to_string(),
        },
        NormalizedAttendanceRecord {
            person_name: "张三".to_string(),
            day: 1,
            normalized_time: time,
            window: AttendanceWindow::Am,
            file_name: "a.xlsx".to_string(),
            sheet_name: "打卡".to_string(),
        },
    ];

    let aggregate = aggregate_records(&records, &["张三".to_string()], 2026, 4, &AttendanceRules::default())
        .expect("aggregate");
    let summary = aggregate.summary_rows.iter().find(|row| row.name == "张三").expect("summary");
    assert_eq!(summary.actual_punch_count, 1);
}
