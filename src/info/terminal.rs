//! Terminal emulator detection: env vars first, then parent process name via sysinfo.

use std::env;

use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

pub fn get() -> (String, String) {
    let name = detect_via_env().or_else(detect_via_parent_process).unwrap_or_else(|| "—".into());
    ("Terminal".into(), name)
}

fn detect_via_env() -> Option<String> {
    if let Ok(v) = env::var("TERM_PROGRAM") {
        if !v.is_empty() {
            return Some(v);
        }
    }
    if env::var("ALACRITTY_LOG").is_ok() {
        return Some("Alacritty".into());
    }
    if env::var("KITTY_PID").is_ok() {
        return Some("Kitty".into());
    }
    None
}

fn detect_via_parent_process() -> Option<String> {
    let current_pid = std::process::id();
    let pid = Pid::from_u32(current_pid);
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );

    let mut pid = Some(pid);
    for _ in 0..20 {
        let p = pid.and_then(|id| sys.process(id))?;
        let name = p.name().to_string_lossy();
        if is_likely_terminal(name.as_ref()) {
            return Some(clean_terminal_name(&name));
        }
        pid = p.parent();
    }
    None
}

fn is_likely_terminal(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("terminal")
        || lower.contains("alacritty")
        || lower.contains("kitty")
        || lower.contains("konsole")
        || lower.contains("gnome-terminal")
        || lower.contains("xfce4-terminal")
        || lower.contains("urxvt")
        || lower.contains("rxvt")
        || lower.contains("st ")
        || lower == "st"
        || lower.contains("wezterm")
        || lower.contains("foot")
        || lower.contains("wayst")
        || lower.contains("hyper")
}

fn clean_terminal_name(name: &str) -> String {
    let s = name.trim();
    if s.is_empty() {
        return "—".into();
    }
    if let Some(rest) = s.strip_suffix('-') {
        return rest.to_string();
    }
    s.to_string()
}
