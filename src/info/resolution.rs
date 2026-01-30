//! Monitor resolution and refresh rate via display-info.

use display_info::DisplayInfo;

/// Returns resolution string, e.g. "1920x1080 @ 144Hz" or "1920x1080 @ 60Hz, 2560x1440 @ 60Hz".
pub fn get() -> (String, String) {
    let value = match DisplayInfo::all() {
        Ok(displays) if !displays.is_empty() => displays
            .iter()
            .map(|d| {
                let hz = if d.frequency > 0.0 {
                    format!(" @ {}Hz", d.frequency as u32)
                } else {
                    String::new()
                };
                format!("{}x{}{}", d.width, d.height, hz)
            })
            .collect::<Vec<_>>()
            .join(", "),
        _ => "—".into(),
    };
    ("Resolution".into(), value)
}
