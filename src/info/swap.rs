//! Swap memory (used/total and percentage) via sysinfo.

use sysinfo::System;

use crate::info::utils;

/// Returns swap line: "Used / Total (Percentage%)" with unit_type formatting.
pub fn get(sys: &System, unit_type: &str) -> (String, String) {
    let total = sys.total_swap();
    let used = sys.used_swap();
    let used_str = utils::format_bytes(used, unit_type);
    let total_str = utils::format_bytes(total, unit_type);
    let pct = if total > 0 {
        (used as f64 / total as f64 * 100.0).round() as u32
    } else {
        0
    };
    let value = format!("{} / {} ({}%)", used_str, total_str, pct);
    ("Swap".into(), value)
}
