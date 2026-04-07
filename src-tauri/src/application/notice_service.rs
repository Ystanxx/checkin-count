use crate::application::dto::{
    BuildNoticeFromSummaryFileRequest, BuildNoticeRequest, NoticeBuildResponse, NoticeRulesDto,
};
use crate::domain::attendance_schema::AttendanceSummaryRow;
use crate::domain::notice_filter::build_notice_rows;
use crate::domain::rules::{LogicalOperator, NoticeRules};
use crate::error::{AppError, AppResult};
use crate::infrastructure::security::validate_input_paths;
use crate::infrastructure::summary_importer::import_summary_rows;
use std::path::PathBuf;

pub fn build_notice_list(
    summary_rows: &[AttendanceSummaryRow],
    request: BuildNoticeRequest,
) -> AppResult<NoticeBuildResponse> {
    let rules = parse_notice_rules(&request.rules)?;
    Ok(NoticeBuildResponse {
        notice_rows: build_notice_rows(summary_rows, &rules),
    })
}

pub fn build_notice_list_from_summary_file(
    request: BuildNoticeFromSummaryFileRequest,
) -> AppResult<NoticeBuildResponse> {
    let rules = parse_notice_rules(&request.rules)?;
    let paths = validate_input_paths(&[request.input_file])?;
    let input_file = paths
        .into_iter()
        .next()
        .ok_or_else(|| AppError::Validation("请先选择汇总文件。".to_string()))?;
    let summary_rows =
        import_summary_rows(PathBuf::from(input_file)).map_err(AppError::Validation)?;

    Ok(NoticeBuildResponse {
        notice_rows: build_notice_rows(&summary_rows, &rules),
    })
}

fn parse_notice_rules(dto: &NoticeRulesDto) -> AppResult<NoticeRules> {
    let operator = match dto.operator.trim().to_ascii_uppercase().as_str() {
        "AND" => LogicalOperator::And,
        "OR" => LogicalOperator::Or,
        other => return Err(AppError::Validation(format!("不支持的逻辑运算符: {other}"))),
    };

    Ok(NoticeRules {
        absent_days_threshold: dto.absent_days_threshold,
        absent_count_threshold: dto.absent_count_threshold,
        operator,
    })
}
