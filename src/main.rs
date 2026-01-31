mod config;
mod info;
mod modules;
mod ui;

use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

use config::AppConfig;
use info::{
    cpu, de_wm, disk, distro_slug, get_color_palette, get_gpu_name as info_gpu_name,
    get_gpu_temperature, gpu as info_gpu, kernel, memory, os, os_age, packages, resolution, shell,
    swap, system_for_fetch, terminal, terminal_font, theme, uptime, user_host,
};
use sysinfo::System;
use ui::image_render;
use ui::logos;
use ui::theme::ThemeManager;

#[derive(Parser, Debug)]
#[command(name = "novafetch")]
#[command(about = "A fast, rice-ready system fetch tool")]
struct Args {
    /// Override ASCII art logo (e.g. arch, ubuntu, fedora, macos, fallback)
    #[arg(long, value_name = "NAME")]
    logo: Option<String>,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Path to config file (default: ~/.config/novafetch/config.toml)
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Output system info as JSON (skips ASCII art and rendering)
    #[arg(long)]
    json: bool,
}

/// Fetch one or more (label, value) lines for a layout key. Returns (key, label, value) for each line.
fn fetch_module(
    key: &str,
    config: &AppConfig,
    sys: Option<&System>,
    no_color: bool,
) -> Vec<(String, String, String)> {
    let unit = config.general.unit_type.as_str();
    let mut out = Vec::new();

    match key {
        "user_host" => {
            let (_, v) = user_host();
            out.push((key.to_string(), String::new(), v));
        }
        "os" => {
            let (l, v) = os();
            out.push((key.to_string(), l, v));
        }
        "kernel" => {
            let (l, v) = kernel();
            out.push((key.to_string(), l, v));
        }
        "uptime" => {
            let (l, v) = uptime();
            out.push((key.to_string(), l, v));
        }
        "shell" => {
            let (l, v) = shell();
            out.push((key.to_string(), l, v));
        }
        "de" => {
            let (l, v) = de_wm();
            out.push((key.to_string(), l, v));
        }
        "cpu" => {
            if let Some(s) = sys {
                let (l, v) = cpu(s, config.general.show_cpu_bar);
                out.push((key.to_string(), l, v));
            }
        }
        "gpu" => {
            let name = modules::gpu::get_gpu_name()
                .or_else(info_gpu_name)
                .unwrap_or_else(|| "Generic GPU".into());
            let value = match get_gpu_temperature() {
                Some(t) => format!("{} ({:.1}°C)", name, t),
                None => name,
            };
            let (default_l, _) = info_gpu();
            out.push((key.to_string(), default_l, value));
        }
        "memory" => {
            if let Some(s) = sys {
                let (l, v) = memory(s, config.general.show_memory_bar, unit);
                out.push((key.to_string(), l, v));
            }
        }
        "disk" => {
            for (l, v) in disk(config.general.show_disk_bar, "Disk", unit) {
                out.push((key.to_string(), l, v));
            }
        }
        "terminal" => {
            let (l, v) = terminal();
            out.push((key.to_string(), l, v));
        }
        "terminal_font" => {
            let (l, v) = terminal_font();
            out.push((key.to_string(), l, v));
        }
        "packages" => {
            let (l, v) = packages();
            out.push((key.to_string(), l, v));
        }
        "resolution" => {
            let (l, v) = resolution();
            out.push((key.to_string(), l, v));
        }
        "swap" => {
            if let Some(s) = sys {
                let (l, v) = swap(s, unit);
                out.push((key.to_string(), l, v));
            }
        }
        "os_age" => {
            let (l, v) = os_age();
            out.push((key.to_string(), l, v));
        }
        "theme" => {
            for (l, v) in theme() {
                out.push((key.to_string(), l, v));
            }
        }
        "media" => {
            if let Some(v) = modules::media::get_media_status() {
                out.push((key.to_string(), "Media".into(), v));
            }
        }
        "local_ip" => {
            if let Some(v) = modules::net::get_local_ip() {
                out.push((key.to_string(), "Local IP".into(), v));
            }
        }
        "palette" => {
            // Palette returns Vec<String> with two lines (normal + bright colors)
            for palette_line in get_color_palette(no_color) {
                out.push((key.to_string(), String::new(), palette_line));
            }
        }
        _ => {}
    }
    out
}

/// Collect all lines (key, label, value) following config.layout.
fn collect_lines(
    config: &AppConfig,
    sys: Option<&System>,
    no_color: bool,
) -> Vec<(String, String, String)> {
    let mut lines = Vec::new();
    for key in &config.layout {
        let key = key.trim();
        if key.is_empty() {
            continue;
        }
        for (k, label, value) in fetch_module(key, config, sys, no_color) {
            lines.push((k, label, value));
        }
    }
    lines
}

/// Category definitions for tree layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Hardware,
    Software,
    Status,
}

impl Category {
    fn name(&self) -> &str {
        match self {
            Category::Hardware => "Hardware",
            Category::Software => "Software",
            Category::Status => "Status",
        }
    }

    fn header(&self, width: usize) -> String {
        let name = self.name();
        let total_dashes = width.saturating_sub(name.len());
        let left_dashes = total_dashes / 2;
        let right_dashes = total_dashes - left_dashes;
        format!(
            "{}{}{}",
            "─".repeat(left_dashes),
            name,
            "─".repeat(right_dashes)
        )
    }
}

/// Categorize a key into Hardware, Software, or Status.
fn categorize_key(key: &str) -> Option<Category> {
    match key {
        "user_host" | "cpu" | "gpu" | "memory" | "disk" | "resolution" | "swap" => {
            Some(Category::Hardware)
        }
        "os" | "kernel" | "de" | "shell" | "terminal" | "terminal_font" | "packages"
        | "theme" | "os_age" => Some(Category::Software),
        "uptime" | "local_ip" | "media" => Some(Category::Status),
        "palette" => None, // Palette is special, shown at the very end
        _ => None,
    }
}

/// Format a group of items with tree structure.
/// First item uses no prefix (root), middle items use ├─, last item uses └─.
fn format_group(
    items: &[(String, String)],
    theme: &ThemeManager,
    tree_color: &str,
) -> Vec<String> {
    let mut result = Vec::new();
    let count = items.len();
    if count == 0 {
        return result;
    }

    for (i, (label, value)) in items.iter().enumerate() {
        let is_first = i == 0;
        let is_last = i == count - 1;

        let formatted_value = theme.format_value(value);

        if is_first {
            // First item: no tree prefix, just the label
            let formatted_label = theme.format_label("", label);
            result.push(format!(" {}: {}", formatted_label, formatted_value));
        } else {
            let prefix = if is_last { " └─ " } else { " ├─ " };
            let colored_prefix = format!("{}{}\x1b[0m", tree_color, prefix);
            let formatted_label = theme.format_label("", label);
            result.push(format!("{}{}: {}", colored_prefix, formatted_label, formatted_value));
        }
    }
    result
}

/// Build tree-structured output from categorized lines.
fn build_tree_output(
    lines: &[(String, String, String)],
    theme: &ThemeManager,
    no_color: bool,
) -> Vec<String> {
    let mut result = Vec::new();
    let tree_color = if no_color {
        ""
    } else {
        "\x1b[38;5;214m" // Orange/Yellow for tree structure
    };

    // Group lines by category
    let mut hardware = Vec::new();
    let mut software = Vec::new();
    let mut status = Vec::new();
    let mut palette_lines = Vec::new();

    for (key, label, value) in lines {
        if key == "palette" {
            palette_lines.push(value.clone());
            continue;
        }
        if let Some(cat) = categorize_key(key) {
            let display_label = if label.is_empty() {
                // For user_host, use a default label
                if key == "user_host" {
                    "Host".to_string()
                } else {
                    key.to_string()
                }
            } else {
                label.clone()
            };
            match cat {
                Category::Hardware => hardware.push((display_label, value.clone())),
                Category::Software => software.push((display_label, value.clone())),
                Category::Status => status.push((display_label, value.clone())),
            }
        }
    }

    // Format each category
    let header_width = 24; // Width for category headers

    if !hardware.is_empty() {
        let header = Category::Hardware.header(header_width);
        let colored_header = if no_color {
            header
        } else {
            format!("{}{}\x1b[0m", tree_color, header)
        };
        result.push(colored_header);
        result.extend(format_group(&hardware, theme, tree_color));
        result.push(String::new()); // Empty line between categories
    }

    if !software.is_empty() {
        let header = Category::Software.header(header_width);
        let colored_header = if no_color {
            header
        } else {
            format!("{}{}\x1b[0m", tree_color, header)
        };
        result.push(colored_header);
        result.extend(format_group(&software, theme, tree_color));
        result.push(String::new());
    }

    if !status.is_empty() {
        let header = Category::Status.header(header_width);
        let colored_header = if no_color {
            header
        } else {
            format!("{}{}\x1b[0m", tree_color, header)
        };
        result.push(colored_header);
        result.extend(format_group(&status, theme, tree_color));
        result.push(String::new());
    }

    // Add palette lines at the very end if present
    if !palette_lines.is_empty() {
        for palette_line in palette_lines {
            result.push(theme.format_value(&palette_line));
        }
    }

    result
}

/// Convert lines to JSON map (for --json).
fn lines_to_json(
    lines: &[(String, String, String)],
    sys: Option<&System>,
    config: &AppConfig,
) -> HashMap<String, serde_json::Value> {
    let mut map: HashMap<String, serde_json::Value> = lines
        .iter()
        .map(|(_k, l, v)| {
            let key = if l.is_empty() {
                "user_host".to_string()
            } else {
                l.to_string()
            };
            (key, serde_json::Value::String(v.clone()))
        })
        .collect();
    if let Some(s) = sys {
        if config.layout.contains(&"memory".to_string()) {
            map.insert(
                "memory_used_bytes".to_string(),
                serde_json::Value::Number(serde_json::Number::from(s.used_memory())),
            );
            map.insert(
                "memory_total_bytes".to_string(),
                serde_json::Value::Number(serde_json::Number::from(s.total_memory())),
            );
        }
        if config.layout.contains(&"swap".to_string()) {
            map.insert(
                "swap_used_bytes".to_string(),
                serde_json::Value::Number(serde_json::Number::from(s.used_swap())),
            );
            map.insert(
                "swap_total_bytes".to_string(),
                serde_json::Value::Number(serde_json::Number::from(s.total_swap())),
            );
        }
    }
    map
}

fn main() {
    let args = Args::parse();
    let config = AppConfig::load(args.config.as_deref());

    let need_sys = config.layout.iter().any(|k| {
        k.as_str() == "memory" || k.as_str() == "cpu" || k.as_str() == "swap"
    });
    let mut sys = need_sys.then(system_for_fetch);
    if let Some(ref mut s) = sys {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        s.refresh_cpu_usage();
    }
    let lines = collect_lines(&config, sys.as_ref(), args.no_color);

    if args.json {
        let map = lines_to_json(&lines, sys.as_ref(), &config);
        match serde_json::to_string_pretty(&map) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("json error: {}", e),
        }
        return;
    }

    let theme = ThemeManager::new(&config, args.no_color);

    // Optional: image logo
    let use_image = config
        .general
        .image_path
        .as_ref()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty());

    if let Some(path) = use_image {
        if let Ok(()) = image_render::print_image(path, config.general.image_width) {
            println!();
            let info_lines = build_tree_output(&lines, &theme, args.no_color);
            for line in info_lines {
                println!("{}", line);
            }
            return;
        }
        eprintln!("novafetch: image '{}' failed, using ASCII logo", path);
    }

    // ASCII logo (if enabled)
    let slug: String = args
        .logo
        .clone()
        .or(config.ascii.distro_override.clone())
        .unwrap_or_else(distro_slug);
    let slug = slug.trim();
    let slug = if slug.is_empty() { "fallback" } else { slug };

    if config.ascii.print_ascii {
        let (logo_lines_vec, _primary) = logos::get_logo(slug);
        let logo_lines: Vec<String> = logo_lines_vec
            .iter()
            .map(|s| {
                if args.no_color {
                    s.to_string()
                } else {
                    let rgb = config.theme.primary_color;
                    crate::ui::gradient::create_gradient_text(
                        s,
                        (rgb[0], rgb[1], rgb[2]),
                        (
                            config.theme.secondary_color[0],
                            config.theme.secondary_color[1],
                            config.theme.secondary_color[2],
                        ),
                    )
                }
            })
            .collect();

        // Build tree-structured info lines
        let info_lines = build_tree_output(&lines, &theme, args.no_color);

        ui::print_final_result(&logo_lines, &info_lines, 4);
    } else {
        // No ASCII: print tree-structured info lines only
        let info_lines = build_tree_output(&lines, &theme, args.no_color);
        for line in info_lines {
            println!("{}", line);
        }
    }
}
