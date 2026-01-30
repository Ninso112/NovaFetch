//! Enhanced GPU detection: sysinfo components first, then lspci fallback.

use std::process::Command;

/// Tries sysinfo components (e.g. hwmon/thermal labels, macOS TG0P), then lspci on Linux.
/// Returns None on failure or if result is too generic.
pub fn get_gpu_name() -> Option<String> {
    if let Some(name) = try_sysinfo_components() {
        if !is_generic(&name) {
            return Some(name);
        }
    }
    try_lspci()
}

fn is_generic(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower.is_empty()
        || lower == "gpu"
        || lower.contains("generic")
        || lower.contains("unknown")
        || (lower.len() < 4)
}

fn try_sysinfo_components() -> Option<String> {
    use sysinfo::Components;
    let components = Components::new_with_refreshed_list();
    for comp in components.iter() {
        let label = comp.label().to_lowercase();
        let id = comp.id().unwrap_or("").to_lowercase();
        if label.contains("gpu")
            || id.contains("gpu")
            || id == "tg0p"
            || label.contains("nvidia")
            || label.contains("amd")
            || label.contains("radeon")
        {
            let name = comp.label().trim().to_string();
            if !name.is_empty() && !is_generic(&name) {
                return Some(name);
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn try_lspci() -> Option<String> {
    let out = Command::new("lspci")
        .output()
        .ok()
        .filter(|o| o.status.success())?;
    let s = String::from_utf8_lossy(&out.stdout);
    for line in s.lines() {
        let line = line.trim();
        if line.to_lowercase().contains("vga")
            || line.to_lowercase().contains("3d")
            || line.to_lowercase().contains("display")
        {
            // lspci format: "xx:xx.x VGA compatible controller: NVIDIA ..."
            let after_first = line.splitn(2, ':').nth(1).unwrap_or(line).trim();
            let name = after_first
                .splitn(2, ':')
                .nth(1)
                .unwrap_or(after_first)
                .trim();
            if !name.is_empty() && !is_generic(name) {
                return Some(clean_gpu_string(name));
            }
        }
    }
    None
}

#[cfg(not(target_os = "linux"))]
fn try_lspci() -> Option<String> {
    None
}

fn clean_gpu_string(s: &str) -> String {
    let mut t = s.trim().to_string();
    for word in ["Corporation", "Inc.", "Co.", "Ltd.", "Limited"] {
        t = t.replace(word, "");
    }
    t = t.replace("  ", " ");
    t.trim().to_string()
}
