//! Customizable progress bars for CPU/RAM/disk. Switch style via CURRENT_BAR_STYLE.

/// Bar style: change this constant to switch the look of all bars.
pub const CURRENT_BAR_STYLE: BarStyle = BarStyle::Round;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BarStyle {
    /// Filled `█`, Empty `░` -> `[████░░░░]`
    Classic,
    /// Filled `▰`, Empty `▱` -> `[▰▰▰▱▱▱]`
    Round,
    /// Filled `#`, Empty `.` -> `[####....]`
    Retro,
    /// Filled `●`, Empty `○` -> `●●●○○` (no brackets)
    Minimal,
}

impl BarStyle {
    fn chars(self) -> (char, char, bool) {
        match self {
            BarStyle::Classic => ('█', '░', true),
            BarStyle::Round => ('▰', '▱', true),
            BarStyle::Retro => ('#', '.', true),
            BarStyle::Minimal => ('●', '○', false),
        }
    }
}

/// Returns a bar string for the given style: used/total as filled/empty segments.
/// `width` = number of characters (e.g. 10). Minimal style has no brackets.
pub fn bar(used: u64, total: u64, width: u8) -> String {
    bar_with_style(used, total, width, CURRENT_BAR_STYLE)
}

/// Same as `bar` but with an explicit style (e.g. for tests or overrides).
pub fn bar_with_style(used: u64, total: u64, width: u8, style: BarStyle) -> String {
    if total == 0 || width == 0 {
        return String::new();
    }
    let w = width as usize;
    let filled = ((used as f64 / total as f64) * (w as f64)).round() as usize;
    let filled = filled.min(w);
    let empty = w.saturating_sub(filled);
    let (fc, ec, use_brackets) = style.chars();
    let s = format!("{}{}", fc.to_string().repeat(filled), ec.to_string().repeat(empty));
    if use_brackets {
        format!("[{}]", s)
    } else {
        s
    }
}
