mod ascii;
pub mod logos;
mod render;

pub use render::render;
pub use render::RenderOptions;
pub use render::SEPARATOR;
/// Low-level API: logo/info as `Vec<String>`, side-by-side layout.
#[allow(unused_imports)]
pub use render::{display_width, format_info_lines, format_logo_lines, render_side_by_side};