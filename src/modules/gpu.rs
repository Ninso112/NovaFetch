//! Enhanced GPU detection: lspci first (cleanest model name), then sysinfo components as fallback.

use std::process::Command;

/// Prefer lspci (or platform equivalent) for the cleanest model name; fall back to sysinfo components.
/// Sysinfo fallback skips sensor labels (junction, edge, mem, sensor, fan) so "amdgpu junction" is never shown.
pub fn get_gpu_name() -> Option<String> {
    if let Some(name) = try_lspci() {
        return Some(name);
    }
    if let Some(name) = try_sysinfo_components() {
        if !is_generic(&name) {
            return Some(name);
        }
    }
    None
}

fn is_generic(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower.is_empty()
        || lower == "gpu"
        || lower.contains("generic")
        || lower.contains("unknown")
        || (lower.len() < 4)
}

/// Sensor/subsystem labels to ignore when using sysinfo Components (hwmon thermal labels).
fn is_sensor_or_subsystem_label(label: &str) -> bool {
    let lower = label.to_lowercase();
    lower.contains("junction")
        || lower.contains("edge")
        || lower.contains("mem")
        || lower.contains("sensor")
        || lower.contains("fan")
        || lower.contains("temp")
        || lower.contains("power")
}

fn try_sysinfo_components() -> Option<String> {
    use sysinfo::Components;
    let components = Components::new_with_refreshed_list();
    for comp in components.iter() {
        let label = comp.label().trim();
        let label_lower = label.to_lowercase();
        if is_sensor_or_subsystem_label(label) {
            continue;
        }
        let id = comp.id().unwrap_or("").to_lowercase();
        if label_lower.contains("gpu")
            || id.contains("gpu")
            || id == "tg0p"
            || (label_lower.contains("nvidia") && !is_sensor_or_subsystem_label(label))
            || (label_lower.contains("radeon") && !is_sensor_or_subsystem_label(label))
        {
            let name = label.to_string();
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
            let after_first = line.splitn(2, ':').nth(1).unwrap_or(line).trim();
            let name = after_first
                .splitn(2, ':')
                .nth(1)
                .unwrap_or(after_first)
                .trim();
            if !name.is_empty() && !is_generic(name) {
                return Some(crate::info::clean_gpu_name(name));
            }
        }
    }
    None
}

#[cfg(not(target_os = "linux"))]
fn try_lspci() -> Option<String> {
    None
}
