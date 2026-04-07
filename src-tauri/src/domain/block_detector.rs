use crate::domain::attendance_schema::{BlockProvenance, PersonBlock, WorksheetData};
use crate::domain::time_normalizer::normalize_time_token;
use crate::domain::time_normalizer::to_ascii_fullwidth;
use crate::infrastructure::exported_workbook_detector::is_export_header_term;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NameEvidence {
    Inline,
    LabelPair,
}

#[derive(Debug, Clone)]
struct NameCandidate {
    value: String,
    evidence: NameEvidence,
}

pub fn collect_all_names(worksheets: &[WorksheetData]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut names = Vec::new();

    for sheet in worksheets {
        for row in &sheet.rows {
            if let Some(name) = extract_name_from_row(row) {
                if seen.insert(name.clone()) {
                    names.push(name);
                }
            }
        }
    }

    names
}

pub fn parse_person_blocks(
    worksheet: &WorksheetData,
    start_row: Option<usize>,
) -> Vec<PersonBlock> {
    let mut blocks = Vec::new();
    let mut cursor = start_row.unwrap_or(0);

    while cursor < worksheet.rows.len() {
        match parse_person_block(worksheet, cursor) {
            Some((block, next_cursor)) => {
                blocks.push(block);
                cursor = next_cursor.max(cursor + 1);
            }
            None => {
                cursor += 1;
            }
        }
    }

    blocks
}

fn parse_person_block(worksheet: &WorksheetData, start_row: usize) -> Option<(PersonBlock, usize)> {
    let name_candidate = extract_name_candidate(worksheet.rows.get(start_row)?)?;
    if looks_like_export_header_row(worksheet.rows.get(start_row)?) {
        return None;
    }

    let (date_row_idx, column_days) = detect_best_date_row(worksheet, start_row)?;
    let (day_to_tokens, consumed_rows, end_row) =
        collect_time_rows(worksheet, start_row, date_row_idx, &column_days);
    let legal_time_token_count = count_legal_time_tokens(&day_to_tokens);

    if legal_time_token_count == 0
        && (name_candidate.evidence == NameEvidence::LabelPair
            || looks_like_export_structure(worksheet, start_row, end_row))
    {
        return None;
    }

    Some((
        PersonBlock {
            name: name_candidate.value,
            day_to_tokens,
            provenance: BlockProvenance {
                file_name: worksheet.source.file_name.clone(),
                sheet_name: worksheet.source.sheet_name.clone(),
                start_row,
                date_row: date_row_idx,
                end_row,
                consumed_rows,
            },
        },
        end_row + 1,
    ))
}

fn extract_name_from_row(row: &[String]) -> Option<String> {
    extract_name_candidate(row).map(|candidate| candidate.value)
}

fn extract_name_candidate(row: &[String]) -> Option<NameCandidate> {
    let inline_pattern = Regex::new(r"姓\s*名\s*[：:]\s*([^\s:：\-\|]+)").expect("name regex");

    for cell in row {
        let value = to_ascii_fullwidth(cell).trim().to_string();
        if value.is_empty() {
            continue;
        }
        if let Some(capture) = inline_pattern.captures(&value) {
            let name = capture.get(1)?.as_str().trim().to_string();
            if looks_like_person_name(&name) {
                return Some(NameCandidate {
                    value: name,
                    evidence: NameEvidence::Inline,
                });
            }
        }
    }

    for (index, cell) in row.iter().enumerate() {
        let value = to_ascii_fullwidth(cell)
            .replace(['：', ':'], "")
            .trim()
            .to_string();
        if value == "姓名" {
            for candidate in row.iter().skip(index + 1) {
                let trimmed = candidate.trim();
                if looks_like_person_name(trimmed) {
                    return Some(NameCandidate {
                        value: trimmed.to_string(),
                        evidence: NameEvidence::LabelPair,
                    });
                }
            }
        }
    }

    None
}

fn detect_best_date_row(
    worksheet: &WorksheetData,
    name_row: usize,
) -> Option<(usize, BTreeMap<usize, u32>)> {
    let mut best_row: Option<(usize, BTreeMap<usize, u32>, i32)> = None;
    let strong_name_evidence =
        extract_name_candidate(worksheet.rows.get(name_row)?)?.evidence == NameEvidence::Inline;

    for offset in 1..=4 {
        let row_idx = name_row + offset;
        let Some(row) = worksheet.rows.get(row_idx) else {
            break;
        };
        let map = parse_date_columns(row);
        if map.is_empty() {
            continue;
        }

        let nearby_time_token_count = count_nearby_legal_time_tokens(worksheet, row_idx, &map);
        if nearby_time_token_count == 0 && !strong_name_evidence {
            continue;
        }

        let unique_days = map.values().copied().collect::<BTreeSet<u32>>().len() as i32;
        let duplicate_penalty = map.len() as i32 - unique_days;
        let proximity_bonus = 5 - offset as i32;
        let time_bonus = nearby_time_token_count.min(6) as i32 * 4;
        let score = unique_days * 10 + proximity_bonus + time_bonus - duplicate_penalty * 2;

        match &best_row {
            Some((_, _, current_score)) if *current_score >= score => {}
            _ => best_row = Some((row_idx, map, score)),
        }
    }

    best_row.map(|(row_idx, map, _)| (row_idx, map))
}

fn parse_date_columns(row: &[String]) -> BTreeMap<usize, u32> {
    let mut result = BTreeMap::new();

    for (column_index, cell) in row.iter().enumerate() {
        if let Some(day) = parse_day(cell) {
            result.insert(column_index, day);
        }
    }

    result
}

pub fn is_reserved_person_name(name: &str) -> bool {
    let normalized = normalize_name(name);
    normalized.is_empty() || is_export_header_term(&normalized)
}

fn looks_like_person_name(name: &str) -> bool {
    let normalized = normalize_name(name);
    if normalized.is_empty() || is_reserved_person_name(&normalized) {
        return false;
    }
    if normalized.chars().count() > 20 {
        return false;
    }

    normalized.chars().all(|ch| {
        ch.is_ascii_alphabetic()
            || ch.is_ascii_whitespace()
            || matches!(ch, '·' | '•' | '.')
            || ('\u{4e00}'..='\u{9fff}').contains(&ch)
    })
}

fn normalize_name(value: &str) -> String {
    to_ascii_fullwidth(value)
        .replace(['：', ':'], "")
        .trim()
        .to_string()
}

pub fn parse_day(input: &str) -> Option<u32> {
    let ascii = to_ascii_fullwidth(input).trim().to_string();
    if ascii.is_empty() {
        return None;
    }

    let cleaned = ascii
        .trim_matches(|ch: char| !ch.is_ascii_digit() && ch != '.')
        .trim_end_matches(".0")
        .to_string();

    let day = cleaned.parse::<u32>().ok()?;
    if (1..=31).contains(&day) {
        Some(day)
    } else {
        None
    }
}

fn collect_time_rows(
    worksheet: &WorksheetData,
    start_row: usize,
    date_row_idx: usize,
    column_days: &BTreeMap<usize, u32>,
) -> (BTreeMap<u32, Vec<String>>, Vec<usize>, usize) {
    let separators = Regex::new(r"[\s,，;；/／\\、()（）–—-]+").expect("time separator regex");
    let mut day_to_tokens: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    let mut consumed_rows = vec![start_row, date_row_idx];
    let mut end_row = date_row_idx;
    let column_mapping = expand_date_columns(column_days, worksheet.source.column_count);

    for row_idx in (date_row_idx + 1)..worksheet.rows.len() {
        let row = &worksheet.rows[row_idx];

        if row_idx > date_row_idx + 4 {
            break;
        }

        if extract_name_from_row(row).is_some() {
            break;
        }

        let next_date_map = parse_date_columns(row);
        if !next_date_map.is_empty() && row_idx != date_row_idx + 1 {
            break;
        }

        let mut row_has_token = false;
        for (column_index, cell) in row.iter().enumerate() {
            let Some(day) = column_mapping.get(&column_index).copied() else {
                continue;
            };

            let ascii = to_ascii_fullwidth(cell);
            let tokens: Vec<String> = separators
                .split(&ascii)
                .map(str::trim)
                .filter(|token| !token.is_empty())
                .map(ToOwned::to_owned)
                .collect();

            if tokens.is_empty() {
                continue;
            }

            row_has_token = true;
            day_to_tokens.entry(day).or_default().extend(tokens);
        }

        if row_has_token {
            consumed_rows.push(row_idx);
            end_row = row_idx;
        } else if row_idx > date_row_idx + 1 {
            break;
        }
    }

    (day_to_tokens, consumed_rows, end_row)
}

fn count_nearby_legal_time_tokens(
    worksheet: &WorksheetData,
    date_row_idx: usize,
    column_days: &BTreeMap<usize, u32>,
) -> usize {
    let (day_to_tokens, _, _) = collect_time_rows(
        worksheet,
        date_row_idx.saturating_sub(1),
        date_row_idx,
        column_days,
    );
    count_legal_time_tokens(&day_to_tokens)
}

fn count_legal_time_tokens(day_to_tokens: &BTreeMap<u32, Vec<String>>) -> usize {
    day_to_tokens
        .values()
        .flat_map(|tokens| tokens.iter())
        .filter(|token| normalize_time_token(token).is_some())
        .count()
}

fn looks_like_export_structure(
    worksheet: &WorksheetData,
    start_row: usize,
    end_row: usize,
) -> bool {
    worksheet.rows[start_row..=end_row]
        .iter()
        .any(|row| looks_like_export_header_row(row))
}

fn looks_like_export_header_row(row: &[String]) -> bool {
    let normalized = row
        .iter()
        .map(|cell| normalize_name(cell))
        .filter(|cell| !cell.is_empty())
        .collect::<Vec<_>>();

    normalized
        .iter()
        .filter(|cell| is_export_header_term(cell))
        .count()
        >= 2
}

fn expand_date_columns(
    column_days: &BTreeMap<usize, u32>,
    total_columns: usize,
) -> BTreeMap<usize, u32> {
    let mut full_map = BTreeMap::new();
    let columns: Vec<(usize, u32)> = column_days.iter().map(|(col, day)| (*col, *day)).collect();

    for (index, (start_col, day)) in columns.iter().enumerate() {
        let end_col = columns
            .get(index + 1)
            .map(|(col, _)| col.saturating_sub(1))
            .unwrap_or(total_columns.saturating_sub(1));

        for current_col in *start_col..=end_col {
            full_map.insert(current_col, *day);
        }
    }

    full_map
}
