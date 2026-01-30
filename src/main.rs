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
            out.push((key.to_string(), String::new(), get_color_palette(no_color)));
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
    let sep = &config.general.separator;

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
            // Align: compute max label width (formatted)
            let max_label_width = if config.general.align_values && !args.no_color {
                lines
                    .iter()
                    .map(|(k, l, _)| ThemeManager::display_width(&theme.format_label(k, l)))
                    .max()
                    .unwrap_or(0)
            } else {
                0
            };
            for (key, label, value) in &lines {
                let formatted_label = theme.format_label(key, label);
                let formatted_value = theme.format_value(value);
                if label.is_empty() {
                    println!("{}", formatted_value);
                } else {
                    let label_part = if config.general.align_values && max_label_width > 0 {
                        format!("{}{}", theme.pad_label_and_sep(&formatted_label, max_label_width), sep)
                    } else {
                        format!("{}{}", formatted_label, sep)
                    };
                    println!("{}{}", label_part, formatted_value);
                }
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

        // Build info lines (formatted strings)
        let max_label_width = if config.general.align_values && !args.no_color {
            lines
                .iter()
                .map(|(k, l, _)| ThemeManager::display_width(&theme.format_label(k, l)))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        let mut info_lines = Vec::new();
        for (key, label, value) in &lines {
            let formatted_label = theme.format_label(key, label);
            let formatted_value = theme.format_value(value);
            if label.is_empty() {
                info_lines.push(formatted_value);
            } else {
                let label_part = if config.general.align_values && max_label_width > 0 {
                    format!("{}{}", theme.pad_label_and_sep(&formatted_label, max_label_width), sep)
                } else {
                    format!("{}{}", formatted_label, sep)
                };
                info_lines.push(format!("{}{}", label_part, formatted_value));
            }
        }

        ui::print_final_result(&logo_lines, &info_lines, 4);
    } else {
        // No ASCII: print info lines only
        let max_label_width = if config.general.align_values && !args.no_color {
            lines
                .iter()
                .map(|(k, l, _)| ThemeManager::display_width(&theme.format_label(k, l)))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        for (key, label, value) in &lines {
            let formatted_label = theme.format_label(key, label);
            let formatted_value = theme.format_value(value);
            if label.is_empty() {
                println!("{}", formatted_value);
            } else {
                let label_part = if config.general.align_values && max_label_width > 0 {
                    format!("{}{}", theme.pad_label_and_sep(&formatted_label, max_label_width), sep)
                } else {
                    format!("{}{}", formatted_label, sep)
                };
                println!("{}{}", label_part, formatted_value);
            }
        }
    }
}
