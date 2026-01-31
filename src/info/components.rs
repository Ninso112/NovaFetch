//! Temperature from sysinfo Components (hwmon/thermal). Refreshed list used.

use sysinfo::Components;

/// Returns average temperature of components whose label contains any of the keywords.
/// Uses a refreshed components list. Returns None if no matching component or no valid reading.
pub fn get_temperature(keywords: &[&str]) -> Option<f32> {
    let components = Components::new_with_refreshed_list();
    let mut temps = Vec::new();
    for comp in components.iter() {
        let label = comp.label().to_lowercase();
        if !keywords.iter().any(|k| label.contains(&k.to_lowercase())) {
            continue;
        }
        if let Some(t) = comp.temperature() {
            if t.is_finite() && t > 0.0 && t < 200.0 {
                temps.push(t);
            }
        }
    }
    if temps.is_empty() {
        None
    } else {
        Some(temps.iter().sum::<f32>() / temps.len() as f32)
    }
}

/// GPU temperature from Components. Prefers "edge" or "composite" over "junction" (use junction only if alone).
pub fn get_gpu_temperature() -> Option<f32> {
    let components = Components::new_with_refreshed_list();
    let gpu_keywords = ["amdgpu", "nvidia", "radeon"];
    let mut edge_or_composite = Vec::new();
    let mut junction = Vec::new();
    let mut other = Vec::new();

    for comp in components.iter() {
        let label = comp.label().to_lowercase();
        if !gpu_keywords.iter().any(|k| label.contains(k)) {
            continue;
        }
        let Some(t) = comp.temperature() else { continue };
        if !t.is_finite() || t <= 0.0 || t >= 200.0 {
            continue;
        }
        if label.contains("edge") || label.contains("composite") {
            edge_or_composite.push(t);
        } else if label.contains("junction") {
            junction.push(t);
        } else {
            other.push(t);
        }
    }

    let chosen = if !edge_or_composite.is_empty() {
        &edge_or_composite
    } else if !other.is_empty() {
        &other
    } else if !junction.is_empty() {
        &junction
    } else {
        return None;
    };
    Some(chosen.iter().sum::<f32>() / chosen.len() as f32)
}

/// RAM/DIMM temperature from Components. Matches "dimm", "dram", "ddr", "memory" (case-insensitive),
/// excludes labels containing "gpu". Returns average if multiple sticks, None if none found.
pub fn get_ram_temperature() -> Option<f32> {
    let components = Components::new_with_refreshed_list();
    let ram_keywords = ["dimm", "dram", "ddr", "memory"];
    let mut temps = Vec::new();
    for comp in components.iter() {
        let label = comp.label().to_lowercase();
        if label.contains("gpu") {
            continue;
        }
        if !ram_keywords.iter().any(|k| label.contains(k)) {
            continue;
        }
        if let Some(t) = comp.temperature() {
            if t.is_finite() && t > 0.0 && t < 200.0 {
                temps.push(t);
            }
        }
    }
    if temps.is_empty() {
        None
    } else {
        Some(temps.iter().sum::<f32>() / temps.len() as f32)
    }
}
