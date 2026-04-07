use crate::domain::attendance_schema::{
    AggregateOutput, AttendanceDetailRow, AttendanceSummaryRow, AttendanceWindow, NeedPunchDayRow,
    NormalizedAttendanceRecord,
};
use crate::domain::block_detector::is_reserved_person_name;
use crate::domain::error::DomainError;
use crate::domain::model::{unique_sorted_times, DailyWindowRecord};
use crate::domain::rules::AttendanceRules;
use chrono::NaiveDate;
use std::collections::{BTreeMap, BTreeSet};

pub fn aggregate_records(
    records: &[NormalizedAttendanceRecord],
    recognized_names: &[String],
    year: i32,
    month: u32,
    rules: &AttendanceRules,
) -> Result<AggregateOutput, DomainError> {
    let month_day_count = month_days(year, month)?;
    let need_days: Vec<u32> = (1..=month_day_count)
        .filter(|day| !rules.rest_days.contains(day))
        .collect();

    let need_day_rows = need_days
        .iter()
        .map(|day| NeedPunchDayRow {
            year,
            month,
            day: *day,
        })
        .collect::<Vec<_>>();

    let mut names = BTreeSet::new();
    for name in recognized_names {
        if !name.trim().is_empty() && !is_reserved_person_name(name) {
            names.insert(name.trim().to_string());
        }
    }
    for record in records {
        if !is_reserved_person_name(&record.person_name) {
            names.insert(record.person_name.clone());
        }
    }

    let need_day_set: BTreeSet<u32> = need_days.iter().copied().collect();
    let mut daily_maps: BTreeMap<String, BTreeMap<u32, DailyWindowRecord>> = BTreeMap::new();
    for record in records {
        if !need_day_set.contains(&record.day) {
            continue;
        }

        let name_entry = daily_maps.entry(record.person_name.clone()).or_default();
        let day_entry = name_entry.entry(record.day).or_default();

        match record.window {
            AttendanceWindow::Am => {
                day_entry.am_times = unique_sorted_times(
                    day_entry
                        .am_times
                        .iter()
                        .copied()
                        .chain(std::iter::once(record.normalized_time)),
                );
            }
            AttendanceWindow::Noon => {
                day_entry.noon_times = unique_sorted_times(
                    day_entry
                        .noon_times
                        .iter()
                        .copied()
                        .chain(std::iter::once(record.normalized_time)),
                );
            }
        }
    }

    let mut detail_rows = Vec::new();
    let mut summary_rows = Vec::new();

    for name in names {
        let day_map = daily_maps.get(&name).cloned().unwrap_or_default();
        let mut actual_punch_days = 0_u32;
        let mut actual_punch_count = 0_u32;
        let mut absent_dates = Vec::new();

        for day in &need_days {
            let daily = day_map.get(day).cloned().unwrap_or_default();
            let daily_count = daily.daily_count();
            if daily.hit_any() {
                actual_punch_days += 1;
            } else {
                absent_dates.push(*day);
            }
            actual_punch_count += daily_count;

            detail_rows.push(AttendanceDetailRow {
                name: name.clone(),
                date: format!("{year:04}-{month:02}-{day:02}"),
                day: *day,
                am_hit: !daily.am_times.is_empty(),
                noon_hit: !daily.noon_times.is_empty(),
                daily_count,
                am_times: daily
                    .am_times
                    .iter()
                    .map(|time| time.format("%H:%M:%S").to_string())
                    .collect(),
                noon_times: daily
                    .noon_times
                    .iter()
                    .map(|time| time.format("%H:%M:%S").to_string())
                    .collect(),
            });
        }

        let need_punch_days = need_days.len() as u32;
        let expected_punch_count = need_punch_days * 2;
        let absent_days = absent_dates.len() as u32;
        let absent_count = expected_punch_count.saturating_sub(actual_punch_count);

        summary_rows.push(AttendanceSummaryRow {
            name: name.clone(),
            need_punch_days,
            expected_punch_count,
            actual_punch_days,
            actual_punch_count,
            absent_days,
            absent_count,
            absent_dates,
        });
    }

    Ok(AggregateOutput {
        detail_rows,
        summary_rows,
        need_day_rows,
        daily_maps,
    })
}

pub fn month_days(year: i32, month: u32) -> Result<u32, DomainError> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| DomainError::YearMonth(format!("{year}-{month}")))?;

    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .ok_or_else(|| DomainError::YearMonth(format!("{year}-{month}")))?;

    Ok(next_month.signed_duration_since(first_day).num_days() as u32)
}
