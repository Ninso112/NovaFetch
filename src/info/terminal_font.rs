//! Terminal font: gsettings (GNOME/GTK monospace) or fallback.

use std::process::Command;

pub fn get() -> (String, String) {
    let value = get_gsettings_monospace().unwrap_or_else(|| "Unknown (Terminal-specific)".into());
    ("Terminal Font".into(), value)
}

fn get_gsettings_monospace() -> Option<String> {
    let out = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "monospace-font-name"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    let s = s.trim().trim_matches('\'');
    if s.is_empty() {
        return None;
    }
    Some(s.to_string())
}
