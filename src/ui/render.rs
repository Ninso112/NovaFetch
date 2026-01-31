//! Layout: side-by-side printing of logo and info lines.

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
            i += 1;
            while i < bytes.len() && (bytes[i] & 0xC0) == 0x80 {
                i += 1;
            }
            width += 1;
        }
    }
    width
}

/// Prints logo lines and info lines side-by-side. Logo is left-padded to a fixed width, then margin, then info.
/// When logo is shorter than info, remaining info lines are indented to maintain alignment.
pub fn print_final_result(
    logo_lines: &[String],
    info_lines: &[String],
    margin: usize,
) {
    // Calculate the maximum display width of the logo (ignoring ANSI codes)
    let logo_width = logo_lines
        .iter()
        .map(|s| display_width(s))
        .max()
        .unwrap_or(0);
    
    let num_rows = logo_lines.len().max(info_lines.len());
    let gap = " ".repeat(margin);
    let empty_logo_padding = " ".repeat(logo_width);

    for i in 0..num_rows {
        // Get logo line or use empty padding if logo is finished
        if let Some(logo_line) = logo_lines.get(i) {
            let logo_visible = display_width(logo_line);
            let pad_spaces = logo_width.saturating_sub(logo_visible);
            print!("{}{}", logo_line, " ".repeat(pad_spaces));
        } else {
            // Logo finished: print spaces equal to logo_width to maintain alignment
            print!("{}", empty_logo_padding);
        }
        
        // Print gap between logo and info
        print!("{}", gap);

        // Print info line if available
        if let Some(info) = info_lines.get(i) {
            print!("{}", info);
        }
        
        println!();
    }
}
