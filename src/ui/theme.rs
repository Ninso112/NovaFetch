//! Theming engine: gradient/solid labels, Nerd Font icons, value alignment.

use crate::config::{AppConfig, GeneralConfig, ThemeConfig};
use crate::ui::gradient;
use std::fmt::Write;

/// Nerd Font icon (Unicode private use) per layout key. Empty string = no icon.
fn nerd_icon_for_key(key: &str) -> &'static str {
    match key {
        "user_host" => "\u{f007}",  // fa-user
        "os" => "\u{f17c}",          // fa-linux
        "kernel" => "\u{f109}",      // fa-cog
        "uptime" => "\u{f017}",      // fa-clock-o
        "shell" => "\u{f489}",       // fa-terminal (alt)
        "de" => "\u{f1e6}",          // fa-desktop
        "cpu" => "\u{f0e4}",         // fa-microchip
        "gpu" => "\u{f108}",         // fa-microchip / display
        "memory" => "\u{f2db}",      // fa-memory
        "disk" => "\u{f0a0}",        // fa-hdd-o
        "terminal" => "\u{f120}",    // fa-terminal
        "terminal_font" => "\u{f031}", // fa-font
        "packages" => "\u{f187}",    // fa-cube
        "resolution" => "\u{f108}",  // fa-desktop
        "swap" => "\u{f2db}",        // fa-exchange
        "os_age" => "\u{f073}",      // fa-calendar
        "media" => "\u{f001}",       // fa-music
        "local_ip" => "\u{f0ac}",    // fa-globe
        _ => "",
    }
}

/// ANSI TrueColor for foreground: \x1b[38;2;R;G;Bm
fn ansi_rgb(rgb: [u8; 3]) -> String {
    format!("\x1b[38;2;{};{};{}m", rgb[0], rgb[1], rgb[2])
}

pub struct ThemeManager<'a> {
    pub general: &'a GeneralConfig,
    pub theme: &'a ThemeConfig,
    pub no_color: bool,
}

impl<'a> ThemeManager<'a> {
    pub fn new(config: &'a AppConfig, no_color: bool) -> Self {
        Self {
            general: &config.general,
            theme: &config.theme,
            no_color,
        }
    }

    /// Format label: optional Nerd icon + gradient or solid primary color.
    pub fn format_label(&self, key: &str, text: &str) -> String {
        if self.no_color {
            return text.to_string();
        }
        let mut out = String::new();
        if self.general.use_nerd_fonts {
            let icon = nerd_icon_for_key(key);
            if !icon.is_empty() {
                write!(out, "{} ", icon).ok();
            }
        }
        let labeled = if out.is_empty() {
            text.to_string()
        } else {
            format!("{}{}", out, text)
        };
        if self.theme.mode.to_lowercase() == "gradient" {
            let start = (
                self.theme.primary_color[0],
                self.theme.primary_color[1],
                self.theme.primary_color[2],
            );
            let end = (
                self.theme.secondary_color[0],
                self.theme.secondary_color[1],
                self.theme.secondary_color[2],
            );
            gradient::create_gradient_text(&labeled, start, end)
        } else {
            format!(
                "{}{}\x1b[0m",
                ansi_rgb(self.theme.primary_color),
                labeled
            )
        }
    }

    /// Format value in text color.
    pub fn format_value(&self, value: &str) -> String {
        if self.no_color {
            return value.to_string();
        }
        format!(
            "{}{}\x1b[0m",
            ansi_rgb(self.theme.text_color),
            value
        )
    }
}
