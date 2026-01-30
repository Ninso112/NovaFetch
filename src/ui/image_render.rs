//! Rendering of image/GIF logos via viuer (Sixel, Kitty, iTerm, or ANSI blocks).

use std::path::Path;

/// Default width in terminal cells when `width` is None (roughly 30–40 chars).
const DEFAULT_IMAGE_WIDTH: u32 = 36;

/// Prints an image (or GIF) from `path` to stdout using viu.
/// Uses Sixel/Kitty/iTerm if supported, otherwise ANSI blocks.
/// `width`: terminal cell width; if None, uses DEFAULT_IMAGE_WIDTH.
pub fn print_image(path: &str, width: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("image not found: {}", path.display()),
        )
        .into());
    }

    let w = width.unwrap_or(DEFAULT_IMAGE_WIDTH);
    let config = viuer::Config {
        width: Some(w),
        ..Default::default()
    };

    viuer::print_from_file(path, &config)
        .map(|_| ())
        .map_err::<Box<dyn std::error::Error>, _>(Into::into)
}
