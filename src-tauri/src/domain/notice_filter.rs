use crate::domain::attendance_schema::{AttendanceSummaryRow, NoticeRow};
use crate::domain::block_detector::is_reserved_person_name;
use crate::domain::rules::{LogicalOperator, NoticeRules};

pub fn build_notice_rows(
    summary_rows: &[AttendanceSummaryRow],
    rules: &NoticeRules,
) -> Vec<NoticeRow> {
    let mut notices = Vec::new();

    for row in summary_rows {
        if is_reserved_person_name(&row.name) {
            continue;
        }

        let by_days = rules
            .absent_days_threshold
            .map(|threshold| row.absent_days > threshold)
            .unwrap_or(false);
        let by_count = rules
            .absent_count_threshold
            .map(|threshold| row.absent_count > threshold)
            .unwrap_or(false);

        let triggered = match (
            rules.absent_days_threshold.is_some(),
            rules.absent_count_threshold.is_some(),
            rules.operator,
        ) {
            (true, true, LogicalOperator::And) => by_days && by_count,
            (true, true, LogicalOperator::Or) => by_days || by_count,
            (true, false, _) => by_days,
            (false, true, _) => by_count,
            (false, false, _) => false,
        };

        if !triggered {
            continue;
        }

        let mut reasons = Vec::new();
        if by_days {
            if let Some(threshold) = rules.absent_days_threshold {
                reasons.push(format!("缺勤天数>{threshold}"));
            }
        }
        if by_count {
            if let Some(threshold) = rules.absent_count_threshold {
                reasons.push(format!("缺勤次数>{threshold}"));
            }
        }

        notices.push(NoticeRow {
            name: row.name.clone(),
            need_punch_days: row.need_punch_days,
            expected_punch_count: row.expected_punch_count,
            actual_punch_days: row.actual_punch_days,
            actual_punch_count: row.actual_punch_count,
            absent_days: row.absent_days,
            absent_count: row.absent_count,
            absent_dates: row.absent_dates.clone(),
            trigger_reason: reasons.join(" / "),
        });
    }

    notices.sort_by(|left, right| {
        right
            .absent_days
            .cmp(&left.absent_days)
            .then(right.absent_count.cmp(&left.absent_count))
            .then(left.name.cmp(&right.name))
    });

    notices
}
