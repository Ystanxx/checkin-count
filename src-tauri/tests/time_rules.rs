use attendance_tauri_lib::domain::attendance_schema::AttendanceWindow;
use attendance_tauri_lib::domain::rules::AttendanceRules;
use attendance_tauri_lib::domain::time_normalizer::normalize_time_token;
use attendance_tauri_lib::domain::window_classifier::classify_window;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TokenFixture {
    tokens: Vec<String>,
}

#[test]
fn normalizes_fullwidth_compact_and_fractional_times() {
    let fixture: TokenFixture =
        serde_json::from_str(include_str!("../../tests/fixtures/time_tokens.json"))
            .expect("fixture parse");

    let normalized = fixture
        .tokens
        .iter()
        .filter_map(|token| normalize_time_token(token))
        .map(|time| time.format("%H:%M:%S").to_string())
        .collect::<Vec<_>>();

    assert!(normalized.contains(&"08:35:00".to_string()));
    assert!(normalized.contains(&"12:00:00".to_string()));
    assert!(normalized.contains(&"09:04:59".to_string()));
}

#[test]
fn default_rule_treats_1405_as_valid_noon() {
    let rules = AttendanceRules::default();
    let time = normalize_time_token("14:05").expect("normalized time");
    assert_eq!(classify_window(time, &rules), Some(AttendanceWindow::Noon));
}
