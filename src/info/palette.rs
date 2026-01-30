//! ANSI color palette row (neofetch-style blocks at bottom of fetch).

/// Reset ANSI code.
const RESET: &str = "\x1b[0m";

/// Builds a string of colored blocks for the 8 standard (30–37) and optionally bright (90–97) ANSI colors.
/// Each block: `\x1b[{i}m██ \x1b[0m`. When `no_color` is true, returns plain blocks (no ANSI).
pub fn get_color_palette(no_color: bool) -> String {
    if no_color {
        return plain_blocks();
    }
    let mut s = String::new();
    for i in 30..=37 {
        s.push_str(&format!("\x1b[{}m██ {}", i, RESET));
    }
    s.push('\n');
    for i in 90..=97 {
        s.push_str(&format!("\x1b[{}m██ {}", i, RESET));
    }
    s
}

fn plain_blocks() -> String {
    let block = "██ ";
    let row: String = (0..8).map(|_| block).collect();
    format!("{}\n{}", row.trim_end(), row.trim_end())
}
