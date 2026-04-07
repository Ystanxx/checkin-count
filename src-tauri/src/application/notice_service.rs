use crate::application::dto::{BuildNoticeRequest, NoticeBuildResponse, NoticeRulesDto};
use crate::domain::attendance_schema::AttendanceSummaryRow;
use crate::domain::notice_filter::build_notice_rows;
use crate::domain::rules::{LogicalOperator, NoticeRules};
use crate::error::{AppError, AppResult};

pub fn build_notice_list(
    summary_rows: &[AttendanceSummaryRow],
    request: BuildNoticeRequest,
) -> AppResult<NoticeBuildResponse> {
    let rules = parse_notice_rules(&request.rules)?;
    Ok(NoticeBuildResponse {
        notice_rows: build_notice_rows(summary_rows, &rules),
    })
}

fn parse_notice_rules(dto: &NoticeRulesDto) -> AppResult<NoticeRules> {
    let operator = match dto.operator.trim().to_ascii_uppercase().as_str() {
        "AND" => LogicalOperator::And,
        "OR" => LogicalOperator::Or,
        other => {
            return Err(AppError::Validation(format!(
                "不支持的逻辑运算符: {other}"
            )))
        }
    };

    Ok(NoticeRules {
        absent_days_threshold: dto.absent_days_threshold,
        absent_count_threshold: dto.absent_count_threshold,
        operator,
    })
}
