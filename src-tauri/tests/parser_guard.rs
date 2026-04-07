use attendance_tauri_lib::application::app_service;
use attendance_tauri_lib::application::dto::{
    AttendanceRulesDto, ParsePreviewRequest, SummaryBuildRequest,
};
use rust_xlsxwriter::Workbook;
use tempfile::tempdir;

#[test]
fn raw_attendance_build_summary_only_contains_real_people() {
    let dir = tempdir().expect("tempdir");
    let input_path = dir.path().join("raw.xlsx");
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name("打卡").expect("sheet name");

    let rows = vec![
        vec!["姓名", "张三", "", ""],
        vec!["1", "2", "3", ""],
        vec!["08:55", "12:03", "", ""],
        vec!["", "12:05", "08:59", ""],
        vec!["姓名", "李四", "", ""],
        vec!["1", "2", "3", ""],
        vec!["09:00", "", "12:10", ""],
    ];

    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            sheet
                .write_string(row_index as u32, column_index as u16, *cell)
                .expect("write cell");
        }
    }
    workbook.save(&input_path).expect("save workbook");

    let response = app_service::build_summary(
        SummaryBuildRequest {
            input_files: vec![input_path.to_string_lossy().to_string()],
            year: 2026,
            month: 3,
            start_row: None,
            rules: AttendanceRulesDto::default(),
        },
        &|_| {},
    )
    .expect("build summary");

    let names = response
        .summary_rows
        .iter()
        .map(|row| row.name.as_str())
        .collect::<Vec<_>>();

    assert!(names.contains(&"张三"));
    assert!(names.contains(&"李四"));
    assert!(!names.contains(&"日期"));
    assert!(!names.contains(&"需要打卡日"));
}

#[test]
fn exported_summary_workbook_is_rejected_by_preview_and_summary() {
    let dir = tempdir().expect("tempdir");
    let input_path = dir.path().join("summary.xlsx");
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name("汇总").expect("sheet name");

    let rows = vec![
        vec![
            "姓名",
            "需要打卡日",
            "应打卡次数",
            "打卡天数",
            "打卡次数",
            "缺勤天数",
            "缺勤次数",
            "缺勤具体日期",
        ],
        vec!["张三", "22", "44", "20", "39", "2", "5", "2,9"],
    ];

    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            sheet
                .write_string(row_index as u32, column_index as u16, *cell)
                .expect("write cell");
        }
    }
    workbook.save(&input_path).expect("save workbook");

    let preview_error = app_service::parse_preview(
        ParsePreviewRequest {
            input_files: vec![input_path.to_string_lossy().to_string()],
            start_row: None,
        },
        &|_| {},
    )
    .expect_err("preview should reject exported workbook");
    assert!(preview_error
        .to_user_visible()
        .message
        .contains("程序导出的汇总/明细文件"));

    let summary_error = app_service::build_summary(
        SummaryBuildRequest {
            input_files: vec![input_path.to_string_lossy().to_string()],
            year: 2026,
            month: 3,
            start_row: None,
            rules: AttendanceRulesDto::default(),
        },
        &|_| {},
    )
    .expect_err("summary should reject exported workbook");
    assert!(summary_error
        .to_user_visible()
        .message
        .contains("请重新选择原始刷卡 Excel"));
}
