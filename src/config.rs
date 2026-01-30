//! Rice-ready configuration: AppConfig with general, theme, layout, and ascii.
//! Loads from ~/.config/novafetch/config.toml (XDG); generates default if missing.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

fn default_true() -> bool {
    true
}

fn default_separator() -> String {
    "  ".into() // Rice-style; e.g. " -> ", " :: ", or "  " (two spaces)
}

fn default_layout() -> Vec<String> {
    vec![
        "user_host".into(),
        "os".into(),
        "kernel".into(),
        "uptime".into(),
        "shell".into(),
        "de".into(),
        "cpu".into(),
        "gpu".into(),
        "memory".into(),
        "disk".into(),
        "terminal".into(),
        "terminal_font".into(),
        "packages".into(),
        "resolution".into(),
        "swap".into(),
        "os_age".into(),
        "theme".into(),
        "media".into(),
        "local_ip".into(),
    ]
}

/// Primary RGB: blue. Used for gradient start / solid labels.
fn default_primary_color() -> [u8; 3] {
    [59, 130, 246]
}

/// Secondary RGB: purple. Used for gradient end.
fn default_secondary_color() -> [u8; 3] {
    [147, 51, 234]
}

/// Text/value color: white.
fn default_text_color() -> [u8; 3] {
    [255, 255, 255]
}

fn default_theme_mode() -> String {
    "gradient".into()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralConfig {
    #[serde(default = "default_separator")]
    pub separator: String,
    #[serde(default = "default_true")]
    pub use_nerd_fonts: bool,
    #[serde(default = "default_true")]
    pub align_values: bool,
    #[serde(default)]
    pub unit_type: String,
    #[serde(default = "default_true")]
    pub show_memory_bar: bool,
    #[serde(default = "default_true")]
    pub show_cpu_bar: bool,
    #[serde(default = "default_true")]
    pub show_disk_bar: bool,
    #[serde(default)]
    pub image_path: Option<String>,
    #[serde(default)]
    pub image_width: Option<u32>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            separator: default_separator(),
            use_nerd_fonts: true,
            align_values: true,
            unit_type: "standard".into(),
            show_memory_bar: true,
            show_cpu_bar: true,
            show_disk_bar: true,
            image_path: None,
            image_width: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeConfig {
    #[serde(default = "default_primary_color")]
    pub primary_color: [u8; 3],
    #[serde(default = "default_secondary_color")]
    pub secondary_color: [u8; 3],
    #[serde(default = "default_text_color")]
    pub text_color: [u8; 3],
    #[serde(default = "default_theme_mode")]
    pub mode: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            primary_color: default_primary_color(),
            secondary_color: default_secondary_color(),
            text_color: default_text_color(),
            mode: default_theme_mode(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AsciiConfig {
    #[serde(default)]
    pub distro_override: Option<String>,
    #[serde(default = "default_true")]
    pub print_ascii: bool,
}

impl Default for AsciiConfig {
    fn default() -> Self {
        Self {
            distro_override: None,
            print_ascii: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default = "default_layout")]
    pub layout: Vec<String>,
    #[serde(default)]
    pub ascii: AsciiConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            theme: ThemeConfig::default(),
            layout: default_layout(),
            ascii: AsciiConfig::default(),
        }
    }
}

impl AppConfig {
    /// Default config path: `~/.config/novafetch/config.toml` (XDG).
    pub fn default_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("novafetch").join("config.toml"))
    }

    /// Load config from path. If file is missing, write default and return it.
    pub fn load(path: Option<&Path>) -> Self {
        let path_buf = path
            .map(|p| p.to_path_buf())
            .or_else(Self::default_path);

        match path_buf {
            Some(p) => {
                match std::fs::read_to_string(&p) {
                    Ok(s) => toml::from_str(&s).unwrap_or_else(|_| {
                        let def = Self::default();
                        let _ = def.write_to(&p);
                        def
                    }),
                    Err(_) => {
                        let def = Self::default();
                        let _ = def.write_to(&p);
                        def
                    }
                }
            }
            None => Self::default(),
        }
    }

    /// Write config to path. Creates parent dirs.
    pub fn write_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let s = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, s)
    }
}
