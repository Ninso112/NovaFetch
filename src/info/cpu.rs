//! CPU model name, optional bar, usage %, frequency, and temperature.

use sysinfo::System;

use crate::info::bar;
use crate::info::components;

fn clean_cpu_name(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn get(sys: &System, show_bar: bool) -> (String, String) {
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
    let temp_str = match components::get_temperature(&["k10temp", "coretemp", "package", "die"]) {
        Some(t) => format!(" ({:.1}°C)", t),
        None => String::new(),
    };
    let name_and_temp = format!("{}{}", name, temp_str);

    if !show_bar {
        return ("CPU".into(), name_and_temp);
    }

    let usage_pct = (sys.global_cpu_usage() as u64).min(100);
    let bar_str = bar::bar(usage_pct, 100, 10);
    let freq_mhz = sys.cpus().first().map(|c| c.frequency());
    let freq_str = freq_mhz
        .map(|mhz| {
            if mhz >= 1000 {
                format!("{:.1}GHz", mhz as f32 / 1000.0)
            } else {
                format!("{}MHz", mhz)
            }
        })
        .unwrap_or_else(|| "?".into());
    let value = format!(
        "{} {}% @ {}  {}",
        bar_str, usage_pct, freq_str, name_and_temp
    );
    ("CPU".into(), value)
}
