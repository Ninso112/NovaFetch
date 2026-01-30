//! OS install age: approximate "X years, Y months, Z days" from root or installer dir birth time.

#[cfg(target_os = "linux")]
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns OS age string, e.g. "2 years, 3 months, 4 days" or "Unknown".
pub fn get() -> (String, String) {
    let value = detect_os_age().unwrap_or_else(|| "Unknown".into());
    ("OS Age".into(), value)
}

fn detect_os_age() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        let created = fs_created("/")
            .or_else(|| fs_created("/var/log/installer"))
            .or_else(|| fs_created("/etc"))
            .or_else(|| fs_created("/var/log"));
        created.and_then(format_duration_since)
    }

    #[cfg(not(target_os = "linux"))]
    None
}

#[cfg(target_os = "linux")]
fn fs_created(path: &str) -> Option<SystemTime> {
    std::fs::metadata(path).ok().and_then(|m| m.created().ok())
}

#[cfg(target_os = "linux")]
fn format_duration_since(system_time: SystemTime) -> Option<String> {
    let created_secs = system_time.duration_since(UNIX_EPOCH).ok()?.as_secs();
    let now_secs = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let secs = now_secs.saturating_sub(created_secs);
    let days = secs / 86400;
    let years = days / 365;
    let rem = days % 365;
    let months = rem / 30;
    let days_rem = rem % 30;
    let parts: Vec<String> = [
        (years, "year", "years"),
        (months, "month", "months"),
        (days_rem, "day", "days"),
    ]
    .into_iter()
    .filter(|(n, _, _)| *n > 0)
    .map(|(n, sing, plur)| {
        if n == 1 {
            format!("1 {}", sing)
        } else {
            format!("{} {}", n, plur)
        }
    })
    .collect();
    if parts.is_empty() {
        Some("Less than a day".into())
    } else {
        Some(parts.join(", "))
    }
}
