//! RGB text gradients via ANSI TrueColor.

/// Builds a string with ANSI TrueColor escape codes so each character
/// is interpolated between `start_rgb` and `end_rgb`.
/// Reset is appended at the end.
pub fn create_gradient_text(
    text: &str,
    start_rgb: (u8, u8, u8),
    end_rgb: (u8, u8, u8),
) -> String {
    if text.is_empty() {
        return String::new();
    }
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(
        n * (2 + 3 * 4 + 5) + 4,
    ); // rough: escape + "38;2;R;G;Bm" per char + reset
    for (i, &c) in chars.iter().enumerate() {
        let t = if n <= 1 {
            1.0
        } else {
            i as f64 / (n - 1) as f64
        };
        let r = lerp_u8(start_rgb.0, end_rgb.0, t);
        let g = lerp_u8(start_rgb.1, end_rgb.1, t);
        let b = lerp_u8(start_rgb.2, end_rgb.2, t);
        out.push_str(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, c));
    }
    out.push_str("\x1b[0m");
    out
}

fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    let v = (1.0 - t) * f64::from(a) + t * f64::from(b);
    v.round().clamp(0.0, 255.0) as u8
}
