//! ASCII progress bar for memory/disk: [|||||.....]

/// Returns a bar string: `used/total` as filled/empty segments.
/// `width` = number of characters (e.g. 10 -> [|||||.....]).
pub fn bar(used: u64, total: u64, width: u8) -> String {
    if total == 0 || width == 0 {
        return String::new();
    }
    let w = width as usize;
    let filled = ((used as f64 / total as f64) * (w as f64)).round() as usize;
    let filled = filled.min(w);
    let empty = w.saturating_sub(filled);
    format!("{}{}", "|".repeat(filled), ".".repeat(empty))
}
