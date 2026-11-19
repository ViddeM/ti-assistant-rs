use chrono::Duration;

pub fn format_duration(duration: &Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        duration.num_hours(),
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    )
}

pub fn format_std_duration(duration: std::time::Duration) -> String {
    format_duration(&Duration::from_std(duration).unwrap_or_default())
}
