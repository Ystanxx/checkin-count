use crate::domain::attendance_schema::AttendanceWindow;
use crate::domain::rules::AttendanceRules;
use chrono::NaiveTime;

pub fn classify_window(time: NaiveTime, rules: &AttendanceRules) -> Option<AttendanceWindow> {
    if (rules.am_window.start..=rules.am_window.end).contains(&time) {
        return Some(AttendanceWindow::Am);
    }

    if (rules.noon_window.start..=rules.noon_window.end).contains(&time) {
        return Some(AttendanceWindow::Noon);
    }

    None
}
