//! Disk usage for relevant mounts. Uses sysinfo::Disks.
//! Filters out squashfs (snaps), tmpfs, overlay, read-only loops; keeps real storage (NVMe, SSD, HDD, USB).

use std::ffi::OsStr;
use std::path::Path;
use sysinfo::{DiskKind, DiskRefreshKind, Disks};

use super::utils;
use super::InfoItem;
use crate::info::bar;

/// Returns one (label, value) per relevant disk. Label uses `label_prefix` e.g. "Disk (/)", "Disk (/home)".
/// `unit_type` controls byte display: "standard" (KB/MB/GB), "iec" (KiB/MiB/GiB), "si" (1000-based).
pub fn get(show_bar: bool, label_prefix: &str, unit_type: &str) -> Vec<InfoItem> {
    let disks = Disks::new_with_refreshed_list_specifics(
        DiskRefreshKind::nothing().with_kind().with_storage(),
    );

    let mut items = Vec::new();
    for d in disks.list() {
        if !is_relevant_disk(d.mount_point(), d.name(), d.kind()) {
            continue;
        }
        let total = d.total_space();
        let avail = d.available_space();
        let used = total.saturating_sub(avail);
        let used_str = utils::format_bytes(used, unit_type);
        let total_str = utils::format_bytes(total, unit_type);
        let pct = if total > 0 {
            (used as f64 / total as f64 * 100.0).round() as u32
        } else {
            0
        };
        let fs_type = d.file_system().to_string_lossy();
        let fs_display = if fs_type.is_empty() {
            "?".into()
        } else {
            fs_type.into_owned()
        };
        let bar_str = if show_bar && total > 0 {
            bar::bar(used, total, 10)
        } else {
            String::new()
        };
        let value = if bar_str.is_empty() {
            format!("{}% ({} / {})", pct, used_str, total_str)
        } else {
            format!("{}% ({} / {}) [{}]", pct, used_str, total_str, bar_str)
        };
        let mount_display = mount_display_string(d.mount_point());
        let label = if label_prefix.is_empty() {
            format!("{} ({})", mount_display, fs_display)
        } else {
            format!("{} ({}, {})", label_prefix, mount_display, fs_display)
        };
        items.push((label, value));
    }

    // Stable order: prefer root / and Windows C: first, then sort by mount path
    items.sort_by(|a, b| {
        let order = |s: &str| {
            if s.contains("(/)") || s.eq_ignore_ascii_case("C:") || s.contains("(C:)") {
                0
            } else {
                1
            }
        };
        let oa = order(&a.0);
        let ob = order(&b.0);
        oa.cmp(&ob).then_with(|| a.0.cmp(&b.0))
    });

    items
}

fn mount_display_string(p: &Path) -> String {
    let s = p.to_string_lossy();
    if s.is_empty() || s == "/" {
        "/".into()
    } else {
        s.into_owned()
    }
}

/// Exclude virtual/temporary mounts and loop devices; keep real storage (SSD, HDD, USB).
fn is_relevant_disk(mount_point: &Path, name: &OsStr, kind: DiskKind) -> bool {
    let mount = mount_point.to_string_lossy();
    let name_str = name.to_string_lossy();

    // Skip typical tmpfs/squashfs/overlay mount points
    if mount.contains("snap") {
        return false;
    }
    let mount_lower = mount.to_lowercase();
    if mount_lower == "/tmp"
        || mount_lower == "/run"
        || mount_lower == "/dev/shm"
        || mount_lower.starts_with("/sys")
        || mount_lower.starts_with("/proc")
        || mount_lower.contains("/run/")
        || mount_lower.contains("overlay")
    {
        return false;
    }

    // Skip loop devices (e.g. snaps, read-only images) unless they look like real storage
    if name_str.to_lowercase().contains("loop") {
        return matches!(kind, DiskKind::SSD | DiskKind::HDD);
    }

    true
}

