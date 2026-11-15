use chrono::{DateTime, FixedOffset, Utc};
use serde_json::Value;
use std::collections::HashMap;

/// Escapes HTML special characters to prevent injection in generated email bodies.
pub fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 8);
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Loads `KEY=VALUE` pairs from a dotenv-style file if present.
pub fn load_env_file(path: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let Ok(raw) = std::fs::read_to_string(path) else {
        return map;
    };
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = trimmed.split_once('=') {
            let val = v.trim().trim_matches('"').trim_matches('\'').to_string();
            map.insert(k.trim().to_string(), val);
        }
    }
    map
}

/// Reads a Cal.com response field that may be either a plain string or `{ value, label }`.
pub fn cal_field_str(val: Option<&Value>) -> String {
    match val {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Object(obj)) => obj
            .get("value")
            .or_else(|| obj.get("label"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        _ => String::new(),
    }
}

/// Formats Cal.com ISO start/end timestamps into `(DD.MM.YYYY, HH:MM – HH:MM Uhr)`.
pub fn format_cal_window(start: &str, end: &str, with_uhr_suffix: bool) -> (String, String) {
    let parse = |s: &str| DateTime::parse_from_rfc3339(s).ok();
    if let (Some(sd), Some(ed)) = (parse(start), parse(end)) {
        let date = sd.format("%d.%m.%Y").to_string();
        let range = if with_uhr_suffix {
            format!("{} – {} Uhr", sd.format("%H:%M"), ed.format("%H:%M"))
        } else {
            format!("{} - {}", sd.format("%H:%M"), ed.format("%H:%M"))
        };
        return (date, range);
    }
    let date = start.get(0..10).unwrap_or("").to_string();
    let time = start.get(11..16).unwrap_or("").to_string();
    (date, time)
}

/// Returns current Berlin (CET/CEST approx +01/+02 via local offset fallback) formatted timestamp.
pub fn now_ddmmyyyy_hhmm() -> String {
    let now = Utc::now();
    // Server runs in UTC or Europe/Berlin; Local::now() preserves host timezone like Python datetime.now().
    let local: DateTime<chrono::Local> = now.with_timezone(&chrono::Local);
    local.format("%d.%m.%Y %H:%M").to_string()
}

/// Returns RFC 2822 timestamp for SMTP `Date` headers.
pub fn now_rfc2822() -> String {
    let fixed: DateTime<FixedOffset> = Utc::now().into();
    fixed.to_rfc2822()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>'a' & \"b\"</script>"), "&lt;script&gt;&#39;a&#39; &amp; &quot;b&quot;&lt;/script&gt;");
    }

    #[test]
    fn test_cal_field_str() {
        assert_eq!(cal_field_str(Some(&json!({"value": "+49123"}))), "+49123");
        assert_eq!(cal_field_str(Some(&json!("Max"))), "Max");
        assert_eq!(cal_field_str(None), "");
    }

    #[test]
    fn test_format_cal_window() {
        let (d, t) = format_cal_window("2026-11-15T14:30:00Z", "2026-11-15T15:00:00Z", true);
        assert_eq!(d, "15.11.2026");
        assert_eq!(t, "14:30 – 15:00 Uhr");
    }
}
