use sysinfo::System;

use crate::info::bar;
use crate::info::utils;

pub fn get(sys: &System, show_bar: bool, unit_type: &str) -> (String, String) {
    let used = sys.used_memory();
    let total = sys.total_memory();
    let used_str = utils::format_bytes(used, unit_type);
    let total_str = utils::format_bytes(total, unit_type);
    let bar_str = if show_bar && total > 0 {
        bar::bar(used, total, 10)
    } else {
        String::new()
    };
    let value = if bar_str.is_empty() {
        format!("{} / {}", used_str, total_str)
    } else {
        format!("{} / {} [{}]", used_str, total_str, bar_str)
    };
    ("Memory".into(), value)
}
