use attendance_tauri_lib::domain::aggregator::aggregate_records;
use attendance_tauri_lib::domain::attendance_schema::{AttendanceWindow, NormalizedAttendanceRecord};
use attendance_tauri_lib::domain::notice_filter::build_notice_rows;
use attendance_tauri_lib::domain::rules::{AttendanceRules, LogicalOperator, NoticeRules};
use chrono::NaiveTime;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_aggregate(c: &mut Criterion) {
    let records = (1..=31)
        .flat_map(|day| {
            (0..200).map(move |index| NormalizedAttendanceRecord {
                person_name: format!("员工{index}"),
                day,
                normalized_time: NaiveTime::from_hms_opt(8, 35, 0).expect("time"),
                window: AttendanceWindow::Am,
                file_name: "bench.xlsx".to_string(),
                sheet_name: "打卡".to_string(),
            })
        })
        .collect::<Vec<_>>();
    let names = (0..200).map(|index| format!("员工{index}")).collect::<Vec<_>>();

    c.bench_function("aggregate_records_200_people", |b| {
        b.iter(|| aggregate_records(&records, &names, 2026, 4, &AttendanceRules::default()).expect("aggregate"));
    });
}

fn benchmark_notice(c: &mut Criterion) {
    let aggregate = aggregate_records(
        &(1..=31)
            .flat_map(|day| {
                (0..200).map(move |index| NormalizedAttendanceRecord {
                    person_name: format!("员工{index}"),
                    day,
                    normalized_time: NaiveTime::from_hms_opt(8, 35, 0).expect("time"),
                    window: AttendanceWindow::Am,
                    file_name: "bench.xlsx".to_string(),
                    sheet_name: "打卡".to_string(),
                })
            })
            .collect::<Vec<_>>(),
        &(0..200).map(|index| format!("员工{index}")).collect::<Vec<_>>(),
        2026,
        4,
        &AttendanceRules::default(),
    )
    .expect("aggregate");

    c.bench_function("build_notice_rows_200_people", |b| {
        b.iter(|| {
            build_notice_rows(
                &aggregate.summary_rows,
                &NoticeRules {
                    absent_days_threshold: Some(3),
                    absent_count_threshold: Some(5),
                    operator: LogicalOperator::Or,
                },
            )
        });
    });
}

criterion_group!(benches, benchmark_aggregate, benchmark_notice);
criterion_main!(benches);
