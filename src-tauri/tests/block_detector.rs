use attendance_tauri_lib::domain::attendance_schema::{SheetSource, WorksheetData};
use attendance_tauri_lib::domain::block_detector::{collect_all_names, parse_person_blocks};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    sheet_name: String,
    rows: Vec<Vec<String>>,
}

#[test]
fn legacy_selfcheck_fixture_recognizes_two_people() {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "../../tests/fixtures/legacy_selfcheck_two_people.json"
    ))
    .expect("fixture parse");
    let worksheet = WorksheetData {
        source: SheetSource {
            file_name: "legacy.xlsx".to_string(),
            file_path: "legacy.xlsx".to_string(),
            sheet_name: fixture.sheet_name,
            row_count: fixture.rows.len(),
            column_count: fixture.rows.iter().map(|row| row.len()).max().unwrap_or(0),
        },
        rows: fixture.rows,
    };

    let names = collect_all_names(&[worksheet.clone()]);
    assert!(names.iter().any(|name| name == "张三"));
    assert!(names.iter().any(|name| name == "李四"));

    let blocks = parse_person_blocks(&worksheet, None);
    assert!(blocks.len() >= 2);
}

#[test]
fn short_sample_still_detects_block_without_five_date_columns() {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "../../tests/fixtures/short_sample_blocks.json"
    ))
    .expect("fixture parse");
    let worksheet = WorksheetData {
        source: SheetSource {
            file_name: "short.xlsx".to_string(),
            file_path: "short.xlsx".to_string(),
            sheet_name: fixture.sheet_name,
            row_count: fixture.rows.len(),
            column_count: fixture.rows.iter().map(|row| row.len()).max().unwrap_or(0),
        },
        rows: fixture.rows,
    };

    let blocks = parse_person_blocks(&worksheet, None);
    assert!(blocks.iter().any(|block| block.name == "王五"));
    assert!(blocks.iter().any(|block| block.name == "赵六"));
}

#[test]
fn exported_summary_header_is_not_treated_as_person_name() {
    let rows = vec![
        vec![
            "姓名".to_string(),
            "需要打卡日".to_string(),
            "应打卡次数".to_string(),
            "打卡天数".to_string(),
            "打卡次数".to_string(),
            "缺勤天数".to_string(),
            "缺勤次数".to_string(),
            "缺勤具体日期".to_string(),
        ],
        vec![
            "张三".to_string(),
            "22".to_string(),
            "44".to_string(),
            "20".to_string(),
            "39".to_string(),
            "2".to_string(),
            "5".to_string(),
            "2,9".to_string(),
        ],
    ];
    let worksheet = WorksheetData {
        source: SheetSource {
            file_name: "summary.xlsx".to_string(),
            file_path: "summary.xlsx".to_string(),
            sheet_name: "汇总".to_string(),
            row_count: rows.len(),
            column_count: rows.iter().map(|row| row.len()).max().unwrap_or(0),
        },
        rows,
    };

    let names = collect_all_names(&[worksheet.clone()]);
    assert!(!names.iter().any(|name| name == "需要打卡日"));
    assert!(!names.iter().any(|name| name == "日期"));
    assert!(parse_person_blocks(&worksheet, None).is_empty());
}
