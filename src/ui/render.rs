use colored::{Color, Colorize};
use std::fmt::Display;

pub const SEPARATOR: &str = ": ";

/// Visible character width of a string, ignoring ANSI escape sequences.
pub fn display_width(s: &str) -> usize {
    let mut width = 0;
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'\x1b' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            i += 2;
            while i < bytes.len() && bytes[i] != b'm' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        if bytes[i] < 128 {
            width += 1;
            i += 1;
        } else {
            // UTF-8: advance to end of codepoint, count one column
            i += 1;
            while i < bytes.len() && (bytes[i] & 0xC0) == 0x80 {
                i += 1;
            }
            width += 1;
        }
    }
    width
}

/// Renders logo and info side-by-side. Logo is left-padded to `logo_width`, then `margin` spaces, then info.
/// Both vectors may contain ANSI-colored strings; padding uses display width (ANSI stripped).
pub fn render_side_by_side(
    logo_lines: &[String],
    info_lines: &[String],
    logo_width: usize,
    margin: usize,
) {
    let num_rows = logo_lines.len().max(info_lines.len());
    let gap = " ".repeat(margin);

    for i in 0..num_rows {
        let logo_part = logo_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let logo_visible = display_width(logo_part);
        let pad_spaces = logo_width.saturating_sub(logo_visible);
        print!("{}{}", logo_part, " ".repeat(pad_spaces));
        print!("{}", gap);

        if let Some(info) = info_lines.get(i) {
            print!("{}", info);
        }
        println!();
    }
}

pub struct RenderOptions<'a> {
    pub logo_lines: &'a [&'a str],
    pub stats: &'a [(String, String)],
    /// When set, used for ASCII art and keys (overrides config ascii/key colors).
    pub primary_color: Option<Color>,
    pub ascii_color: &'a str,
    pub key_color: &'a str,
    pub value_color: &'a str,
    pub no_color: bool,
    pub separator: &'a str,
}

fn apply_color(s: impl Display, color_name: &str, bold: bool) -> String {
    let s = s.to_string();
    if bold {
        match color_name.to_lowercase().as_str() {
            "cyan" => s.cyan().bold().to_string(),
            "yellow" => s.yellow().bold().to_string(),
            "white" => s.white().bold().to_string(),
            "blue" => s.blue().bold().to_string(),
            "bright_blue" => s.bright_blue().bold().to_string(),
            "green" => s.green().bold().to_string(),
            "red" => s.red().bold().to_string(),
            "magenta" => s.magenta().bold().to_string(),
            _ => s,
        }
    } else {
        match color_name.to_lowercase().as_str() {
            "cyan" => s.cyan().to_string(),
            "yellow" => s.yellow().to_string(),
            "white" => s.white().to_string(),
            "blue" => s.blue().to_string(),
            "bright_blue" => s.bright_blue().to_string(),
            "green" => s.green().to_string(),
            "red" => s.red().to_string(),
            "magenta" => s.magenta().to_string(),
            _ => s,
        }
    }
}

fn apply_color_plain(s: impl Display) -> String {
    s.to_string()
}

fn apply_color_enum(s: impl Display, color: Color, bold: bool) -> String {
    let s = s.to_string();
    if bold {
        s.color(color).bold().to_string()
    } else {
        s.color(color).to_string()
    }
}

/// Returns logo lines as Vec<String> with ANSI colors applied.
pub fn format_logo_lines(logo_lines: &[&str], opts: &RenderOptions) -> Vec<String> {
    let use_primary = opts.primary_color.is_some() && !opts.no_color;
    logo_lines
        .iter()
        .map(|&line| {
            if opts.no_color {
                apply_color_plain(line)
            } else if use_primary {
                apply_color_enum(line, opts.primary_color.unwrap(), false)
            } else {
                apply_color(line, opts.ascii_color, false)
            }
        })
        .collect()
}

/// Returns system info lines as Vec<String> (e.g. "GPU: ...") with ANSI colors applied.
pub fn format_info_lines(stats: &[(String, String)], opts: &RenderOptions) -> Vec<String> {
    let use_primary = opts.primary_color.is_some() && !opts.no_color;
    stats
        .iter()
        .map(|(key, value)| {
            if key.is_empty() {
                if opts.no_color {
                    apply_color_plain(value)
                } else {
                    apply_color(value, opts.value_color, false)
                }
            } else {
                let key_out = if opts.no_color {
                    apply_color_plain(key)
                } else if use_primary {
                    apply_color_enum(key, opts.primary_color.unwrap(), true)
                } else {
                    apply_color(key, opts.key_color, true)
                };
                let value_out = if opts.no_color {
                    apply_color_plain(value)
                } else {
                    apply_color(value, opts.value_color, false)
                };
                format!("{}{}{}", key_out, opts.separator, value_out)
            }
        })
        .collect()
}

/// Renders logo and system info side-by-side using pre-formatted Vec<String> (with colors).
pub fn render(opts: RenderOptions) {
    let logo_strings = format_logo_lines(opts.logo_lines, &opts);
    let info_strings = format_info_lines(opts.stats, &opts);

    let logo_width = logo_strings
        .iter()
        .map(|s| display_width(s))
        .max()
        .unwrap_or(0);
    const MARGIN: usize = 4;

    render_side_by_side(&logo_strings, &info_strings, logo_width, MARGIN);
}

/// Renders only the info lines (one per line). Used when an image is shown above.
pub fn render_info_only(opts: &RenderOptions) {
    let info_strings = format_info_lines(opts.stats, opts);
    for line in info_strings {
        println!("{}", line);
    }
}
