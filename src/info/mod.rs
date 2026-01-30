mod bar;
mod components;
mod cpu;
mod de_wm;
mod disk;
mod gpu;
mod kernel;
mod memory;
mod os;
mod os_age;
mod packages;
mod resolution;
mod shell;
mod swap;
mod terminal;
mod terminal_font;
mod theme;
mod uptime;
mod user_host;
mod utils;

pub use bar::bar;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

/// One line of fetch output: (key, value). Empty key = header line (e.g. user@host).
pub type InfoItem = (String, String);

pub fn user_host() -> InfoItem {
    user_host::get()
}

pub fn os() -> InfoItem {
    os::get()
}

pub fn kernel() -> InfoItem {
    kernel::get()
}

pub fn uptime() -> InfoItem {
    uptime::get()
}

pub fn shell() -> InfoItem {
    shell::get()
}

pub fn de_wm() -> InfoItem {
    de_wm::get()
}

pub fn cpu(sys: &System, show_bar: bool) -> InfoItem {
    cpu::get(sys, show_bar)
}

/// Raw GPU name detection (OS-specific commands). Returns None on failure.
pub fn get_gpu_name() -> Option<String> {
    gpu::get_gpu_name()
}

/// GPU temperature from sysinfo Components (prefers edge/composite over junction). None if unavailable.
pub fn get_gpu_temperature() -> Option<f32> {
    components::get_gpu_temperature()
}

pub fn gpu() -> InfoItem {
    let gpu_name = get_gpu_name().unwrap_or_else(|| "Generic GPU".into());
    let value = match get_gpu_temperature() {
        Some(t) => format!("{} ({:.1}°C)", gpu_name, t),
        None => gpu_name,
    };
    ("GPU".into(), value)
}

pub fn memory(sys: &System, show_bar: bool, unit_type: &str) -> InfoItem {
    memory::get(sys, show_bar, unit_type)
}

pub fn disk(show_bar: bool, label_prefix: &str, unit_type: &str) -> Vec<InfoItem> {
    disk::get(show_bar, label_prefix, unit_type)
}

pub fn terminal() -> InfoItem {
    terminal::get()
}

pub fn terminal_font() -> InfoItem {
    terminal_font::get()
}

pub fn packages() -> InfoItem {
    packages::get()
}

pub fn resolution() -> InfoItem {
    resolution::get()
}

pub fn swap(sys: &System, unit_type: &str) -> InfoItem {
    swap::get(sys, unit_type)
}

pub fn os_age() -> InfoItem {
    os_age::get()
}

/// GTK theme, icon theme, and font from ~/.config/gtk-3.0/settings.ini.
/// Returns one (label, value) per line: Theme, Icons, Font.
pub fn theme() -> Vec<InfoItem> {
    theme::get_theme_info()
}

/// Builds a System with only memory and CPU (name) refreshed for performance.
pub fn system_for_fetch() -> System {
    System::new_with_specifics(
        RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::everything()),
    )
}

/// Detected distribution ID from the OS (e.g. "arch", "ubuntu").
/// Used for logo selection when no --logo override is given.
pub fn distro_slug() -> String {
    System::distribution_id()
}

/// Universal GPU name cleaner for lspci-style raw strings (AMD, NVIDIA, Intel).
/// Removes (rev ...), extracts marketing name from last [...], normalizes vendor prefix.
pub fn clean_gpu_name(raw: &str) -> String {
    let mut s = raw.trim();
    if let Some(i) = s.rfind("(rev ") {
        s = s[..i].trim();
    }
    let from_brackets = extract_last_bracketed(s);
    let mut cleaned = if let Some(name) = from_brackets {
        name.to_string()
    } else {
        remove_corporate_noise(s)
    };
    cleaned = normalize_vendor_prefix(&cleaned, raw);
    collapse_spaces(cleaned.trim())
}

fn extract_last_bracketed(s: &str) -> Option<&str> {
    let start = s.rfind('[')?;
    let rest = &s[start + 1..];
    let end = rest.find(']')?;
    let inner = rest[..end].trim();
    if inner.is_empty() {
        None
    } else {
        Some(inner)
    }
}

fn remove_corporate_noise(s: &str) -> String {
    let mut t = s.to_string();
    for word in [
        "Corporation",
        "Inc.",
        "Co.",
        "Ltd.",
        "Limited",
        "Advanced Micro Devices, ",
        "Advanced Micro Devices",
    ] {
        t = t.replace(word, "");
    }
    t = t.replace("  ", " ");
    t.trim().to_string()
}

fn normalize_vendor_prefix(cleaned: &str, raw: &str) -> String {
    let lower_c = cleaned.to_lowercase();
    let lower_r = raw.to_lowercase();
    if (lower_r.contains("nvidia") || lower_c.contains("geforce") || lower_c.contains("rtx") || lower_c.contains("gtx"))
        && !lower_c.starts_with("nvidia")
    {
        return format!("NVIDIA {}", cleaned.trim());
    }
    if (lower_r.contains("advanced micro devices")
        || lower_r.contains("ati")
        || lower_c.contains("radeon")
        || lower_c.contains("navi"))
        && !lower_c.starts_with("amd")
    {
        return format!("AMD {}", cleaned.trim());
    }
    if (lower_r.contains("intel") || lower_c.contains("arc") || lower_c.contains("iris") || lower_c.contains("uhd") || lower_c.contains("hd graphics"))
        && !lower_c.starts_with("intel")
    {
        return format!("Intel {}", cleaned.trim());
    }
    cleaned.trim().to_string()
}

fn collapse_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ").trim().to_string()
}
