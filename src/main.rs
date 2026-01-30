mod config;
mod info;
mod ui;

use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

use config::Config;
use info::{
    cpu, de_wm, disk, distro_slug, gpu, kernel, memory, os, os_age, packages, resolution, shell,
    swap, system_for_fetch, terminal, terminal_font, uptime, user_host, InfoItem,
};
use ui::image_render;
use ui::logos;
use ui::{render, render_info_only, RenderOptions, SEPARATOR};

use sysinfo::System;

#[derive(Parser, Debug)]
#[command(name = "novafetch")]
#[command(about = "A fast, configurable system fetch tool")]
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

/// Data collection phase: gather all enabled system info into a list of (label, value).
/// `sys` is used for memory, cpu, swap when enabled.
fn collect_system_info(config: &Config, sys: Option<&System>) -> Vec<InfoItem> {
    let unit = config.unit_type.as_str();
    let mut stats: Vec<InfoItem> = Vec::with_capacity(20);

    if config.user_host.enabled {
        let (_, value) = user_host();
        stats.push((config.user_host.label.clone(), value));
    }
    if config.os.enabled {
        let (_, value) = os();
        stats.push((config.os.label.clone(), value));
    }
    if config.kernel.enabled {
        let (_, value) = kernel();
        stats.push((config.kernel.label.clone(), value));
    }
    if config.uptime.enabled {
        let (_, value) = uptime();
        stats.push((config.uptime.label.clone(), value));
    }
    if config.shell.enabled {
        let (_, value) = shell();
        stats.push((config.shell.label.clone(), value));
    }
    if config.de.enabled {
        let (_, value) = de_wm();
        stats.push((config.de.label.clone(), value));
    }
    if config.cpu.enabled {
        if let Some(ref s) = sys {
            let (_, value) = cpu(s);
            stats.push((config.cpu.label.clone(), value));
        }
    }
    if config.gpu.enabled {
        let (_, value) = gpu();
        stats.push((config.gpu.label.clone(), value));
    }
    if config.memory.enabled {
        if let Some(ref s) = sys {
            let (_, value) = memory(s, config.show_memory_bar, unit);
            stats.push((config.memory.label.clone(), value));
        }
    }
    if config.disk.enabled {
        for item in disk(config.show_disk_bar, &config.disk.label, unit) {
            stats.push(item);
        }
    }
    if config.terminal.enabled {
        let (_, value) = terminal();
        stats.push((config.terminal.label.clone(), value));
    }
    if config.terminal_font.enabled {
        let (_, value) = terminal_font();
        stats.push((config.terminal_font.label.clone(), value));
    }
    if config.packages.enabled {
        let (_, value) = packages();
        stats.push((config.packages.label.clone(), value));
    }
    if config.resolution.enabled {
        let (_, value) = resolution();
        stats.push((config.resolution.label.clone(), value));
    }
    if config.swap.enabled {
        if let Some(ref s) = sys {
            let (_, value) = swap(s, unit);
            stats.push((config.swap.label.clone(), value));
        }
    }
    if config.os_age.enabled {
        let (_, value) = os_age();
        stats.push((config.os_age.label.clone(), value));
    }

    stats
}

/// Convert collected stats to a JSON map. Uses serde_json::Value so we can add raw byte integers.
/// Empty label becomes "user_host". When sys is present, adds memory_*_bytes and swap_*_bytes for scripting.
fn stats_to_json_map(
    stats: Vec<InfoItem>,
    sys: Option<&System>,
    config: &Config,
) -> HashMap<String, serde_json::Value> {
    let mut map: HashMap<String, serde_json::Value> = stats
        .into_iter()
        .map(|(k, v)| {
            let key = if k.is_empty() { "user_host".to_string() } else { k };
            (key, serde_json::Value::String(v))
        })
        .collect();

    if let Some(s) = sys {
        if config.memory.enabled {
            map.insert(
                "memory_used_bytes".to_string(),
                serde_json::Value::Number(serde_json::Number::from(s.used_memory())),
            );
            map.insert(
                "memory_total_bytes".to_string(),
                serde_json::Value::Number(serde_json::Number::from(s.total_memory())),
            );
        }
        if config.swap.enabled {
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
    let config = Config::load(args.config.as_deref());

    let need_sys = config.memory.enabled || config.cpu.enabled || config.swap.enabled;
    let sys = need_sys.then(system_for_fetch);
    let stats = collect_system_info(&config, sys.as_ref());

    if args.json {
        let map = stats_to_json_map(stats, sys.as_ref(), &config);
        match serde_json::to_string_pretty(&map) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("json error: {}", e),
        }
        return;
    }

    // Rendering phase: image logo or ASCII logo + info
    let opts = RenderOptions {
        logo_lines: &[],
        stats: &stats,
        primary_color: None,
        ascii_color: &config.colors.ascii,
        key_color: &config.colors.key,
        value_color: &config.colors.value,
        no_color: args.no_color,
        separator: SEPARATOR,
    };

    let use_image = config
        .image_path
        .as_ref()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty());

    if let Some(path) = use_image {
        match image_render::print_image(path, config.image_width) {
            Ok(()) => {
                println!();
                render_info_only(&opts);
            }
            Err(e) => {
                eprintln!("novafetch: image '{}': {} (using ASCII logo)", path, e);
                // Fallback: ASCII logo side-by-side
                let slug: String = args.logo.clone().unwrap_or_else(distro_slug);
                let slug = slug.trim();
                let slug = if slug.is_empty() { "fallback" } else { slug };
                let (logo_lines_vec, primary_color) = logos::get_logo(&slug);
                let logo_lines: &[&str] = logo_lines_vec.as_slice();
                render(RenderOptions {
                    logo_lines,
                    primary_color: Some(primary_color),
                    ..opts
                });
            }
        }
    } else {
        let slug: String = args.logo.clone().unwrap_or_else(distro_slug);
        let slug = slug.trim();
        let slug = if slug.is_empty() { "fallback" } else { slug };
        let (logo_lines_vec, primary_color) = logos::get_logo(&slug);
        let logo_lines: &[&str] = logo_lines_vec.as_slice();
        render(RenderOptions {
            logo_lines,
            primary_color: Some(primary_color),
            ..opts
        });
    }
}
