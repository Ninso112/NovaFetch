//! GTK theme, icon theme, and font detection from ~/.config/gtk-3.0/settings.ini.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Keys we look for in [Settings] (case-sensitive in the file).
const GTK_THEME_KEY: &str = "gtk-theme-name";
const GTK_ICON_THEME_KEY: &str = "gtk-icon-theme-name";
const GTK_FONT_KEY: &str = "gtk-font-name";

/// Returns path to `~/.config/gtk-3.0/settings.ini`, or None if HOME is unset.
fn gtk_settings_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config/gtk-3.0/settings.ini"))
}

/// Strips surrounding double quotes from a value if present.
fn unquote(s: &str) -> String {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        s[1..s.len() - 1].trim().to_string()
    } else {
        s.to_string()
    }
}

/// Extracts GTK Theme, Icon Theme, and Font from ~/.config/gtk-3.0/settings.ini.
/// Returns a list of (label, value) items, e.g. ["Theme: Nordic", "Icons: Papirus", "Font: JetBrains"].
/// Missing file or keys yield "Unknown" for that entry.
pub fn get_theme_info() -> Vec<(String, String)> {
    let path = match gtk_settings_path() {
        Some(p) => p,
        None => return default_theme_lines(),
    };
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return default_theme_lines(),
    };
    let mut theme = String::from("Unknown");
    let mut icons = String::from("Unknown");
    let mut font = String::from("Unknown");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let line = line.trim();
        if line.is_empty() || line.starts_with('[') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = unquote(value.trim());
            match key {
                GTK_THEME_KEY => theme = value,
                GTK_ICON_THEME_KEY => icons = value,
                GTK_FONT_KEY => font = value,
                _ => {}
            }
        }
    }
    vec![
        ("Theme".into(), theme),
        ("Icons".into(), icons),
        ("Font".into(), font),
    ]
}

fn default_theme_lines() -> Vec<(String, String)> {
    vec![
        ("Theme".into(), "Unknown".into()),
        ("Icons".into(), "Unknown".into()),
        ("Font".into(), "Unknown".into()),
    ]
}
