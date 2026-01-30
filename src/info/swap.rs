//! Swap memory (used/total and percentage) via sysinfo.

use sysinfo::System;

/// Returns swap line: "Used MiB / Total MiB (Percentage%)".
pub fn get(sys: &System) -> (String, String) {
    let total = sys.total_swap();
    let used = sys.used_swap();
    let total_mib = total / (1024 * 1024);
    let used_mib = used / (1024 * 1024);
    let pct = if total > 0 {
        (used as f64 / total as f64 * 100.0).round() as u32
    } else {
        0
    };
    let value = format!("{} MiB / {} MiB ({}%)", used_mib, total_mib, pct);
    ("Swap".into(), value)
}
