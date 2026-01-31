//! ANSI color palette row (neofetch-style blocks at bottom of fetch).

/// Reset ANSI code.
const RESET: &str = "\x1b[0m";

/// Builds two strings (normal + bright colors) for the 8 standard (30–37) and 8 bright (90–97) ANSI colors.
/// Each block: `\x1b[{i}m██ \x1b[0m`. When `no_color` is true, returns plain blocks (no ANSI).
/// Returns Vec with two elements: [normal_colors, bright_colors].
pub fn get_color_palette(no_color: bool) -> Vec<String> {
    if no_color {
        return plain_blocks();
    }
    
    let mut normal = String::new();
    for i in 30..=37 {
        normal.push_str(&format!("\x1b[{}m██ {}", i, RESET));
    }
    
    let mut bright = String::new();
    for i in 90..=97 {
        bright.push_str(&format!("\x1b[{}m██ {}", i, RESET));
    }
    
    vec![normal, bright]
}

fn plain_blocks() -> Vec<String> {
    let block = "██ ";
    let row: String = (0..8).map(|_| block).collect();
    vec![row.trim_end().to_string(), row.trim_end().to_string()]
}
