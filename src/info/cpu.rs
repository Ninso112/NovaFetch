//! CPU model name from sysinfo (first CPU, cleaned). Optionally appends temperature.

use sysinfo::System;

use crate::info::components;

/// Collapse multiple spaces into one and trim.
fn clean_cpu_name(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn get(sys: &System) -> (String, String) {
    let name = sys
        .cpus()
        .first()
        .map(|c| {
            let b = clean_cpu_name(c.brand());
            let n = clean_cpu_name(c.name());
            if !b.is_empty() && b != n {
                b
            } else {
                n
            }
        })
        .unwrap_or_else(|| "N/A".into());
    let value = match components::get_temperature(&["k10temp", "coretemp", "package", "die"]) {
        Some(t) => format!("{} ({:.1}°C)", name, t),
        None => name,
    };
    ("CPU".into(), value)
}
