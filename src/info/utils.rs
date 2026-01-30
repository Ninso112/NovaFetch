//! Byte-formatting helper: configurable unit type (standard / IEC / SI).

/// Format bytes according to `unit_type`:
/// - `"standard"` (default): Base 1024, labels "KB", "MB", "GB" (Windows style).
/// - `"iec"`: Base 1024, labels "KiB", "MiB", "GiB" (Linux technical style).
/// - `"si"`: Base 1000, labels "KB", "MB", "GB" (disk manufacturer style).
///
/// Example: `format_bytes(1073741824, "standard")` → `"1.00 GB"`.
pub fn format_bytes(bytes: u64, unit_type: &str) -> String {
    let (base, units): (u64, &[&str]) = match unit_type.to_lowercase().as_str() {
        "iec" => (1024, &["B", "KiB", "MiB", "GiB", "TiB"]),
        "si" => (1000, &["B", "KB", "MB", "GB", "TB"]),
        _ => (1024, &["B", "KB", "MB", "GB", "TB"]), // standard
    };

    if bytes == 0 {
        return format!("0 {}", units[0]);
    }

    let mut idx = 0;
    let mut b = bytes as f64;
    while b >= base as f64 && idx < units.len() - 1 {
        b /= base as f64;
        idx += 1;
    }
    format!("{:.2} {}", b, units[idx])
}
