//! Package count from pacman, dpkg, rpm, flatpak, snap.

use std::process::Command;

/// Returns package count string, e.g. "1234 (pacman), 12 (flatpak)".
pub fn get() -> (String, String) {
    let mut parts = Vec::new();

    if let Some(n) = count_pacman() {
        parts.push(format!("{} (pacman)", n));
    }
    if let Some(n) = count_dpkg() {
        parts.push(format!("{} (dpkg)", n));
    }
    if let Some(n) = count_rpm() {
        parts.push(format!("{} (rpm)", n));
    }
    if let Some(n) = count_flatpak() {
        parts.push(format!("{} (flatpak)", n));
    }
    if let Some(n) = count_snap() {
        parts.push(format!("{} (snap)", n));
    }

    let value = if parts.is_empty() {
        "—".into()
    } else {
        parts.join(", ")
    };
    ("Packages".into(), value)
}

fn count_lines(cmd: &mut Command) -> Option<u32> {
    let out = cmd.output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    let n = s.lines().filter(|l| !l.trim().is_empty()).count() as u32;
    Some(n)
}

/// Like count_lines but skip the first line (header, e.g. flatpak list / snap list).
fn count_lines_skip_header(cmd: &mut Command) -> Option<u32> {
    let out = cmd.output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    let mut lines = s.lines().filter(|l| !l.trim().is_empty());
    let _ = lines.next(); // skip header
    let n = lines.count() as u32;
    Some(n)
}

fn count_pacman() -> Option<u32> {
    count_lines(Command::new("pacman").args(["-Qq"]))
}

fn count_dpkg() -> Option<u32> {
    count_lines(
        Command::new("dpkg-query").args(["-f", "${binary:Package}\n", "-W"]),
    )
}

fn count_rpm() -> Option<u32> {
    count_lines(Command::new("rpm").args(["-qa"]))
}

fn count_flatpak() -> Option<u32> {
    count_lines_skip_header(Command::new("flatpak").args(["list"]))
}

fn count_snap() -> Option<u32> {
    count_lines_skip_header(Command::new("snap").args(["list"]))
}
