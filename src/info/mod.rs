mod bar;
mod cpu;
mod de_wm;
mod disk;
mod gpu;
mod kernel;
mod memory;
mod os;
mod packages;
mod shell;
mod terminal;
mod terminal_font;
mod uptime;
mod user_host;

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

pub fn cpu(sys: &System) -> InfoItem {
    cpu::get(sys)
}

/// Raw GPU name detection (OS-specific commands). Returns None on failure.
pub fn get_gpu_name() -> Option<String> {
    gpu::get_gpu_name()
}

pub fn gpu() -> InfoItem {
    let gpu_name = get_gpu_name().unwrap_or_else(|| "Generic GPU".into());
    ("GPU".into(), gpu_name)
}

pub fn memory(sys: &System, show_bar: bool) -> InfoItem {
    memory::get(sys, show_bar)
}

pub fn disk(show_bar: bool, label_prefix: &str) -> Vec<InfoItem> {
    disk::get(show_bar, label_prefix)
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
