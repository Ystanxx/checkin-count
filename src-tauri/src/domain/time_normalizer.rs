use chrono::NaiveTime;

pub fn to_ascii_fullwidth(input: &str) -> String {
    input
        .chars()
        .map(|ch| match ch {
            '０'..='９' => char::from_u32(ch as u32 - '０' as u32 + '0' as u32).unwrap_or(ch),
            '：' => ':',
            '，' | '、' => ',',
            '；' => ';',
            '／' => '/',
            '　' => ' ',
            _ => ch,
        })
        .collect()
}

pub fn normalize_time_token(token: &str) -> Option<NaiveTime> {
    let normalized = to_ascii_fullwidth(token).trim().to_string();
    if normalized.is_empty() {
        return None;
    }

    if let Some(time) = parse_colon_time(&normalized) {
        return Some(time);
    }

    if let Some(time) = parse_compact_digits(&normalized) {
        return Some(time);
    }

    if let Some(time) = parse_excel_fraction(&normalized) {
        return Some(time);
    }

    None
}

fn parse_colon_time(value: &str) -> Option<NaiveTime> {
    let parts: Vec<&str> = value.split(':').collect();
    if !(parts.len() == 2 || parts.len() == 3) {
        return None;
    }

    let hour = parts[0].parse::<u32>().ok()?;
    let minute = parts[1].parse::<u32>().ok()?;
    let second = if parts.len() == 3 {
        parts[2].parse::<u32>().ok()?
    } else {
        0
    };

    NaiveTime::from_hms_opt(hour, minute, second)
}

fn parse_compact_digits(value: &str) -> Option<NaiveTime> {
    if !value.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }

    match value.len() {
        3 => {
            let hour = value[0..1].parse::<u32>().ok()?;
            let minute = value[1..3].parse::<u32>().ok()?;
            NaiveTime::from_hms_opt(hour, minute, 0)
        }
        4 => {
            let hour = value[0..2].parse::<u32>().ok()?;
            let minute = value[2..4].parse::<u32>().ok()?;
            NaiveTime::from_hms_opt(hour, minute, 0)
        }
        5 => {
            let hour = value[0..1].parse::<u32>().ok()?;
            let minute = value[1..3].parse::<u32>().ok()?;
            let second = value[3..5].parse::<u32>().ok()?;
            NaiveTime::from_hms_opt(hour, minute, second)
        }
        6 => {
            let hour = value[0..2].parse::<u32>().ok()?;
            let minute = value[2..4].parse::<u32>().ok()?;
            let second = value[4..6].parse::<u32>().ok()?;
            NaiveTime::from_hms_opt(hour, minute, second)
        }
        _ => None,
    }
}

fn parse_excel_fraction(value: &str) -> Option<NaiveTime> {
    let number = value.parse::<f64>().ok()?;
    if !(0.0..1.0).contains(&number) {
        return None;
    }

    let total_seconds = (number * 24.0 * 60.0 * 60.0).round() as u32;
    let hour = (total_seconds / 3600) % 24;
    let minute = (total_seconds % 3600) / 60;
    let second = total_seconds % 60;
    NaiveTime::from_hms_opt(hour, minute, second)
}
