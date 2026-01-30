//! Desktop Environment / Window Manager detection via env vars.

use std::env;

pub fn get() -> (String, String) {
    let de_wm = env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "N/A".into());
    ("DE/WM".into(), de_wm)
}
