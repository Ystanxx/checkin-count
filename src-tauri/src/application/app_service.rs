use crate::application::dto::{
    AttendanceRulesDto, ParsePreviewRequest, PreviewBlock, PreviewResponse, ProcessStats,
    SummaryBuildRequest, TaskProgressEvent, WorksheetPreview, BuildSummaryResponse,
};
use crate::domain::aggregator::aggregate_records;
use crate::domain::attendance_schema::{NormalizedAttendanceRecord, PersonBlock};
use crate::domain::block_detector::{collect_all_names, parse_person_blocks};
use crate::domain::error::DomainError;
use crate::domain::rules::AttendanceRules;
use crate::domain::time_normalizer::normalize_time_token;
use crate::domain::window_classifier::classify_window;
use crate::error::{AppError, AppResult};
use crate::infrastructure::excel_reader::read_workbooks;
use crate::infrastructure::security::validate_input_paths;
use std::collections::BTreeSet;

pub fn parse_preview(
    request: ParsePreviewRequest,
    reporter: &impl Fn(TaskProgressEvent),
) -> AppResult<PreviewResponse> {
    reporter(progress_event("preview", 5, "开始读取输入文件"));
    let paths = validate_input_paths(&request.input_files)?;
    let read_result = read_workbooks(&paths)?;
    if read_result.worksheets.is_empty() {
        return Err(AppError::Validation("未读取到任何可用工作表。".to_string()));
    }

    reporter(progress_event("preview", 40, "扫描人员姓名与人员块"));
    let recognized_names = collect_all_names(&read_result.worksheets);
    let start_row = normalize_start_row(request.start_row);
    let mut sample_blocks = Vec::new();
    let mut block_count = 0_usize;
    let mut raw_token_count = 0_usize;

    for worksheet in &read_result.worksheets {
        for block in parse_person_blocks(worksheet, start_row) {
            raw_token_count += block.day_to_tokens.values().map(Vec::len).sum::<usize>();
            block_count += 1;
            if sample_blocks.len() < 12 {
                sample_blocks.push(PreviewBlock {
                    name: block.name.clone(),
                    day_count: block.day_to_tokens.len(),
                    token_count: block.day_to_tokens.values().map(Vec::len).sum::<usize>(),
                    source_file: block.provenance.file_name.clone(),
                    sheet_name: block.provenance.sheet_name.clone(),
                });
            }
        }
    }

    reporter(progress_event("preview", 100, "预览分析完成"));

    Ok(PreviewResponse {
        recognized_names: recognized_names.clone(),
        worksheet_previews: read_result
            .worksheets
            .iter()
            .map(|sheet| WorksheetPreview {
                file_name: sheet.source.file_name.clone(),
                sheet_name: sheet.source.sheet_name.clone(),
                row_count: sheet.source.row_count,
                column_count: sheet.source.column_count,
            })
            .collect(),
        sample_blocks,
        warnings: read_result.warnings,
        stats: ProcessStats {
            worksheet_count: read_result.worksheets.len(),
            recognized_name_count: recognized_names.len(),
            block_count,
            raw_token_count,
            valid_record_count: 0,
        },
    })
}

pub fn build_summary(
    request: SummaryBuildRequest,
    reporter: &impl Fn(TaskProgressEvent),
) -> AppResult<BuildSummaryResponse> {
    reporter(progress_event("summary", 5, "开始读取输入文件"));
    let paths = validate_input_paths(&request.input_files)?;
    let read_result = read_workbooks(&paths)?;
    if read_result.worksheets.is_empty() {
        return Err(AppError::Validation("未读取到任何可用工作表。".to_string()));
    }

    reporter(progress_event("summary", 20, "构建规则与姓名全集"));
    let attendance_rules = build_attendance_rules(&request.rules)?;
    let recognized_names = collect_all_names(&read_result.worksheets);
    let start_row = normalize_start_row(request.start_row);

    reporter(progress_event("summary", 45, "解析人员块与时间 token"));
    let blocks = read_result
        .worksheets
        .iter()
        .flat_map(|sheet| parse_person_blocks(sheet, start_row))
        .collect::<Vec<_>>();

    let raw_token_count = blocks
        .iter()
        .map(|block| block.day_to_tokens.values().map(Vec::len).sum::<usize>())
        .sum::<usize>();

    let records = build_normalized_records(&blocks, &attendance_rules);

    reporter(progress_event("summary", 75, "执行逐日聚合与汇总计算"));
    let aggregate_output = aggregate_records(
        &records,
        &recognized_names,
        request.year,
        request.month,
        &attendance_rules,
    )?;

    reporter(progress_event("summary", 100, "汇总构建完成"));

    Ok(BuildSummaryResponse {
        summary_rows: aggregate_output.summary_rows,
        detail_rows: aggregate_output.detail_rows,
        need_day_rows: aggregate_output.need_day_rows,
        notice_rows: Vec::new(),
        warnings: read_result.warnings,
        stats: ProcessStats {
            worksheet_count: read_result.worksheets.len(),
            recognized_name_count: recognized_names.len(),
            block_count: blocks.len(),
            raw_token_count,
            valid_record_count: records.len(),
        },
    })
}

fn build_attendance_rules(dto: &AttendanceRulesDto) -> AppResult<AttendanceRules> {
    let am_start = normalize_time_token(&dto.am_start)
        .ok_or_else(|| AppError::Domain(DomainError::InvalidRule("AM 开始时间非法".to_string())))?;
    let am_end = normalize_time_token(&dto.am_end)
        .ok_or_else(|| AppError::Domain(DomainError::InvalidRule("AM 结束时间非法".to_string())))?;
    let noon_start = normalize_time_token(&dto.noon_start)
        .ok_or_else(|| AppError::Domain(DomainError::InvalidRule("NOON 开始时间非法".to_string())))?;
    let noon_end = normalize_time_token(&dto.noon_end)
        .ok_or_else(|| AppError::Domain(DomainError::InvalidRule("NOON 结束时间非法".to_string())))?;

    if am_start > am_end {
        return Err(AppError::Domain(DomainError::InvalidRule(
            "AM 窗口开始时间不能晚于结束时间".to_string(),
        )));
    }
    if noon_start > noon_end {
        return Err(AppError::Domain(DomainError::InvalidRule(
            "NOON 窗口开始时间不能晚于结束时间".to_string(),
        )));
    }

    Ok(AttendanceRules {
        am_window: crate::domain::rules::WindowRange {
            start: am_start,
            end: am_end,
        },
        noon_window: crate::domain::rules::WindowRange {
            start: noon_start,
            end: noon_end,
        },
        rest_days: dto.rest_days.iter().copied().collect::<BTreeSet<u32>>(),
    })
}

fn build_normalized_records(
    blocks: &[PersonBlock],
    rules: &AttendanceRules,
) -> Vec<NormalizedAttendanceRecord> {
    let mut records = Vec::new();

    for block in blocks {
        for (day, tokens) in &block.day_to_tokens {
            for token in tokens {
                let Some(time) = normalize_time_token(token) else {
                    continue;
                };
                let Some(window) = classify_window(time, rules) else {
                    continue;
                };
                records.push(NormalizedAttendanceRecord {
                    person_name: block.name.clone(),
                    day: *day,
                    normalized_time: time,
                    window,
                    file_name: block.provenance.file_name.clone(),
                    sheet_name: block.provenance.sheet_name.clone(),
                });
            }
        }
    }

    records
}

fn normalize_start_row(start_row: Option<usize>) -> Option<usize> {
    match start_row {
        Some(value) if value > 1 => Some(value - 1),
        _ => None,
    }
}

fn progress_event(stage: &str, percent: u8, message: &str) -> TaskProgressEvent {
    TaskProgressEvent {
        task_id: format!("task-{stage}"),
        stage: stage.to_string(),
        percent,
        message: message.to_string(),
    }
}

