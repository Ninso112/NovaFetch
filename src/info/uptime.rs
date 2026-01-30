use sysinfo::System;

pub fn get() -> (String, String) {
    let secs = System::uptime();
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 {
        parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
    }
    if mins > 0 || parts.is_empty() {
        parts.push(format!("{} min{}", mins, if mins == 1 { "" } else { "s" }));
    }
    let uptime = parts.join(", ");
    ("Uptime".into(), uptime)
}
