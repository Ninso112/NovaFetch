use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub label: String,
}

impl ModuleConfig {
    pub fn new(enabled: bool, label: impl Into<String>) -> Self {
        Self {
            enabled,
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_user_host")]
    pub user_host: ModuleConfig,
    #[serde(default = "default_os")]
    pub os: ModuleConfig,
    #[serde(default = "default_kernel")]
    pub kernel: ModuleConfig,
    #[serde(default = "default_uptime")]
    pub uptime: ModuleConfig,
    #[serde(default = "default_shell")]
    pub shell: ModuleConfig,
    #[serde(default = "default_de")]
    pub de: ModuleConfig,
    #[serde(default = "default_cpu")]
    pub cpu: ModuleConfig,
    #[serde(default = "default_gpu")]
    pub gpu: ModuleConfig,
    #[serde(default = "default_memory")]
    pub memory: ModuleConfig,
    #[serde(default = "default_disk")]
    pub disk: ModuleConfig,
    #[serde(default = "default_terminal")]
    pub terminal: ModuleConfig,
    #[serde(default = "default_terminal_font")]
    pub terminal_font: ModuleConfig,
    #[serde(default = "default_packages")]
    pub packages: ModuleConfig,
    #[serde(default = "default_resolution")]
    pub resolution: ModuleConfig,
    #[serde(default = "default_swap")]
    pub swap: ModuleConfig,
    #[serde(default = "default_os_age")]
    pub os_age: ModuleConfig,
    #[serde(default = "default_unit_type")]
    pub unit_type: String,
    #[serde(default = "default_true")]
    pub show_memory_bar: bool,
    #[serde(default = "default_true")]
    pub show_disk_bar: bool,
    #[serde(default)]
    pub colors: ColorConfig,
}

fn default_unit_type() -> String {
    "standard".into()
}

fn default_user_host() -> ModuleConfig {
    ModuleConfig::new(true, "")
}
fn default_os() -> ModuleConfig {
    ModuleConfig::new(true, "OS")
}
fn default_kernel() -> ModuleConfig {
    ModuleConfig::new(true, "Kernel")
}
fn default_uptime() -> ModuleConfig {
    ModuleConfig::new(true, "Uptime")
}
fn default_shell() -> ModuleConfig {
    ModuleConfig::new(true, "Shell")
}
fn default_de() -> ModuleConfig {
    ModuleConfig::new(true, "DE")
}
fn default_cpu() -> ModuleConfig {
    ModuleConfig::new(true, "CPU")
}
fn default_gpu() -> ModuleConfig {
    ModuleConfig::new(true, "GPU")
}
fn default_memory() -> ModuleConfig {
    ModuleConfig::new(true, "Memory")
}
fn default_disk() -> ModuleConfig {
    ModuleConfig::new(true, "Disk")
}
fn default_terminal() -> ModuleConfig {
    ModuleConfig::new(true, "Terminal")
}
fn default_terminal_font() -> ModuleConfig {
    ModuleConfig::new(true, "Terminal Font")
}
fn default_packages() -> ModuleConfig {
    ModuleConfig::new(true, "Packages")
}
fn default_resolution() -> ModuleConfig {
    ModuleConfig::new(true, "Resolution")
}
fn default_swap() -> ModuleConfig {
    ModuleConfig::new(true, "Swap")
}
fn default_os_age() -> ModuleConfig {
    ModuleConfig::new(true, "OS Age")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_host: default_user_host(),
            os: default_os(),
            kernel: default_kernel(),
            uptime: default_uptime(),
            shell: default_shell(),
            de: default_de(),
            cpu: default_cpu(),
            gpu: default_gpu(),
            memory: default_memory(),
            disk: default_disk(),
            terminal: default_terminal(),
            terminal_font: default_terminal_font(),
            packages: default_packages(),
            resolution: default_resolution(),
            swap: default_swap(),
            os_age: default_os_age(),
            unit_type: default_unit_type(),
            show_memory_bar: true,
            show_disk_bar: true,
            colors: ColorConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColorConfig {
    #[serde(default = "default_ascii_color")]
    pub ascii: String,
    #[serde(default = "default_key_color")]
    pub key: String,
    #[serde(default = "default_value_color")]
    pub value: String,
}

fn default_ascii_color() -> String {
    "cyan".into()
}
fn default_key_color() -> String {
    "yellow".into()
}
fn default_value_color() -> String {
    "white".into()
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            ascii: default_ascii_color(),
            key: default_key_color(),
            value: default_value_color(),
        }
    }
}

impl Config {
    /// Default config path: `~/.config/novafetch/config.toml`
    pub fn default_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("novafetch").join("config.toml"))
    }

    /// Load config from path. If file does not exist, write default and return it.
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

    /// Write config to path. Creates parent dirs. Returns error on failure.
    pub fn write_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let s = toml::to_string_pretty(self).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, s)
    }
}
