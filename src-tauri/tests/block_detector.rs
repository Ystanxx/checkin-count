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
    let fixture: Fixture = serde_json::from_str(include_str!("../../tests/fixtures/legacy_selfcheck_two_people.json"))
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
    let fixture: Fixture = serde_json::from_str(include_str!("../../tests/fixtures/short_sample_blocks.json"))
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
