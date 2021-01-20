use chrono::{DateTime, TimeZone};
use regex::Regex;

pub mod alias;
pub mod event;
pub mod health;
pub mod license;
pub mod navigation;
pub mod notifications;
pub mod subscriptions;
pub mod threads;
pub mod user;
pub mod uuid;

pub fn format_datetime<Tz: TimeZone>(datetime: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    // The datetime in database is persisted as UTC but is actually in local time. So we reinterpreted it here.
    let naive_datetime = datetime.naive_utc();
    chrono_tz::Europe::Berlin
        .from_local_datetime(&naive_datetime)
        .unwrap()
        .to_rfc3339()
}

pub fn format_alias(prefix: Option<&str>, id: i32, suffix: Option<&str>) -> String {
    let prefix = prefix
        .map(|p| format!("/{}", slugify(p)))
        .unwrap_or_else(|| "".to_string());
    let suffix = suffix.map(slugify).unwrap_or_else(|| "".to_string());
    format!("{}/{}/{}", prefix, id, suffix)
}

fn slugify(segment: &str) -> String {
    let segment = Regex::new(r#"['"`=+*&^%$#@!<>?]"#)
        .unwrap()
        .replace_all(&segment, "");
    let segment = Regex::new(r"[\[\]{}() ,;:/|\-]+")
        .unwrap()
        .replace_all(&segment, "-");
    segment.to_lowercase().trim_matches('-').to_string()
}

#[cfg(test)]
mod test {
    use super::slugify;

    #[test]
    fn format_alias_double_dash() {
        assert_eq!(
            slugify("Flächen- und Volumenberechnung mit Integralen"),
            "flächen-und-volumenberechnung-mit-integralen"
        )
    }
}
