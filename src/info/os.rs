use sysinfo::System;

pub fn get() -> (String, String) {
    let os = System::long_os_version()
        .or_else(|| {
            System::name().and_then(|name| {
                System::os_version().map(|ver| format!("{} {}", name, ver))
            })
        })
        .unwrap_or_else(|| System::name().unwrap_or_else(|| "unknown".into()));
    let arch = System::cpu_arch();
    let full = if arch.is_empty() || os.to_lowercase().contains(&arch.to_lowercase()) {
        os
    } else {
        format!("{} {}", os, arch)
    };
    ("OS".into(), full)
}
