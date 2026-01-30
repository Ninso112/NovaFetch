use sysinfo::System;

pub fn get() -> (String, String) {
    let kernel = System::kernel_long_version();
    ("Kernel".into(), kernel)
}
