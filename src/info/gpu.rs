//! GPU detection via OS-specific commands (lspci, wmic, system_profiler).

use std::io::Read;
use std::process::Command;

/// Returns cleaned GPU name(s), or "Generic GPU" on failure. Never panics.
/// Prefer using `info::gpu()` which calls `get_gpu_name()`.
#[allow(dead_code)]
pub fn get() -> (String, String) {
    let name = get_gpu_name().unwrap_or_else(|| "Generic GPU".into());
    ("GPU".into(), name)
}

/// Detects GPU name(s) using OS-specific commands. Returns None on any failure.
pub fn get_gpu_name() -> Option<String> {
    #[cfg(target_os = "linux")]
    return gpu_linux();

    #[cfg(target_os = "windows")]
    return gpu_windows();

    #[cfg(target_os = "macos")]
    return gpu_macos();

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    None
}

#[cfg(target_os = "linux")]
fn gpu_linux() -> Option<String> {
    if let Some(s) = gpu_linux_lspci() {
        return Some(crate::info::clean_gpu_name(&s));
    }
    if let Some(s) = gpu_linux_sysfs() {
        return Some(crate::info::clean_gpu_name(&s));
    }
    None
}

#[cfg(target_os = "linux")]
fn gpu_linux_lspci() -> Option<String> {
    let out = Command::new("lspci").arg("-mm").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    let mut gpus: Vec<String> = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || (!line.contains("VGA") && !line.contains("3D") && !line.contains("Display")) {
            continue;
        }
        let quoted: Vec<&str> = line.split('"').collect();
        let mut best: Option<String> = None;
        for i in (1..quoted.len()).step_by(2) {
            let seg = quoted[i].trim();
            if seg.is_empty() {
                continue;
            }
            if seg.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            if seg.starts_with("Device ") && seg.len() < 20 {
                continue;
            }
            best = Some(seg.to_string());
        }
        if let Some(b) = best {
            gpus.push(b);
        }
    }
    prefer_dedicated_or_join(gpus)
}

#[cfg(target_os = "linux")]
fn gpu_linux_sysfs() -> Option<String> {
    for i in 0..8u32 {
        for name in ["product_name", "model", "name"] {
            let path = format!("/sys/class/drm/card{}/device/{}", i, name);
            if let Some(s) = read_file_trim(&path) {
                if !s.is_empty() {
                    return Some(s);
                }
            }
        }
    }
    read_file_trim("/sys/class/drm/card0/device/product_name")
        .or_else(|| read_file_trim("/sys/class/drm/card0/device/model"))
        .or_else(|| read_file_trim("/sys/class/drm/card0/device/name"))
}

#[cfg(target_os = "windows")]
fn gpu_windows() -> Option<String> {
    let out = Command::new("wmic")
        .args(["path", "win32_videocontroller", "get", "name"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    let mut names: Vec<String> = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if line.eq_ignore_ascii_case("name") || line.is_empty() {
            continue;
        }
        if !line.is_empty() {
            names.push(line.to_string());
        }
    }
    prefer_dedicated_or_join(names).map(|s| crate::info::clean_gpu_name(&s))
}

#[cfg(target_os = "macos")]
fn gpu_macos() -> Option<String> {
    let out = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    for line in s.lines() {
        let line = line.trim();
        if line.starts_with("Chipset Model:") {
            let name = line.strip_prefix("Chipset Model:").unwrap_or("").trim();
            if !name.is_empty() {
                return Some(crate::info::clean_gpu_name(name));
            }
        }
    }
    None
}

fn read_file_trim(path: &str) -> Option<String> {
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).ok()?;
    let s = buf.trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Prefer dedicated GPU; if multiple, join with ", ".
fn prefer_dedicated_or_join(mut gpus: Vec<String>) -> Option<String> {
    if gpus.is_empty() {
        return None;
    }
    let integrated = ["Intel UHD", "Intel HD", "Intel Graphics", "AMD Radeon Graphics", "Mesa"];
    let dedicated = gpus
        .iter()
        .find(|s| !integrated.iter().any(|i| s.contains(i)));
    if let Some(d) = dedicated {
        return Some(d.clone());
    }
    gpus.dedup();
    Some(gpus.join(", "))
}

