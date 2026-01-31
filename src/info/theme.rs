//! GTK theme, icon theme, and font detection from ~/.config/gtk-3.0/settings.ini
//! or gtk-4.0/settings.ini, with GTK_THEME env fallback.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Keys we look for (case-sensitive in the file).
const GTK_THEME_KEY: &str = "gtk-theme-name";
const GTK_ICON_THEME_KEY: &str = "gtk-icon-theme-name";
const GTK_FONT_KEY: &str = "gtk-font-name";

/// Returns path to `$HOME/.config/<subdir>/settings.ini`, or None if HOME is unset.
/// Does NOT use `~`; File::open does not expand tildes.
fn config_path(subdir: &str) -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join(".config").join(subdir).join("settings.ini"))
}

/// Strips surrounding double quotes from a value if present (e.g. `"Adwaita"` -> `Adwaita`).
fn unquote(s: &str) -> String {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        s[1..s.len() - 1].trim().to_string()
    } else {
        s.to_string()
    }
}

/// Parse an open settings.ini file and fill theme, icons, font. Keys may have spaces around `=`.
fn parse_settings_file(
    file: File,
    theme: &mut String,
    icons: &mut String,
    font: &mut String,
) {
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
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim();
            let value = unquote(v.trim());
            match key {
                GTK_THEME_KEY => *theme = value,
                GTK_ICON_THEME_KEY => *icons = value,
                GTK_FONT_KEY => *font = value,
                _ => {}
            }
        }
    }
}

/// Extracts GTK Theme, Icon Theme, and Font.
/// Tries `$HOME/.config/gtk-3.0/settings.ini` first, then `gtk-4.0/settings.ini`.
/// If both fail, uses `GTK_THEME` env var for theme name if set.
/// Values are trimmed and quotes stripped. Missing file/keys yield "Unknown".
pub fn get_theme_info() -> Vec<(String, String)> {
    let mut theme = String::from("Unknown");
    let mut icons = String::from("Unknown");
    let mut font = String::from("Unknown");

    let tried = [
        config_path("gtk-3.0"),
        config_path("gtk-4.0"),
    ];
    for path_opt in tried {
        let path = match path_opt {
            Some(p) => p,
            None => continue,
        };
        if let Ok(file) = File::open(&path) {
            parse_settings_file(file, &mut theme, &mut icons, &mut font);
            break;
        }
    }

    // Fallback: theme from environment if we still have Unknown and GTK_THEME is set
    if theme == "Unknown" {
        if let Ok(env_theme) = std::env::var("GTK_THEME") {
            let t = env_theme.trim().to_string();
            if !t.is_empty() {
                theme = t;
            }
        }
    }

    vec![
        ("Theme".into(), theme),
        ("Icons".into(), icons),
        ("Font".into(), font),
    ]
}
