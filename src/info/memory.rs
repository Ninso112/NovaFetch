use sysinfo::System;

use crate::info::bar;

pub fn get(sys: &System, show_bar: bool) -> (String, String) {
    let used = sys.used_memory();
    let total = sys.total_memory();
    let used_mib = used / (1024 * 1024);
    let total_mib = total / (1024 * 1024);
    let bar_str = if show_bar && total > 0 {
        bar::bar(used, total, 10)
    } else {
        String::new()
    };
    let value = if bar_str.is_empty() {
        format!("{} MiB / {} MiB", used_mib, total_mib)
    } else {
        format!("{} MiB / {} MiB [{}]", used_mib, total_mib, bar_str)
    };
    ("Memory".into(), value)
}
