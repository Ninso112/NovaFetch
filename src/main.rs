mod config;
mod info;
mod ui;

use clap::Parser;
use std::path::PathBuf;

use config::Config;
use info::{
    cpu, de_wm, disk, distro_slug, gpu, kernel, memory, os, packages, shell, system_for_fetch,
    terminal, terminal_font, uptime, user_host, InfoItem,
};
use ui::logos;
use ui::{render, RenderOptions, SEPARATOR};

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
}

fn main() {
    let args = Args::parse();
    let config = Config::load(args.config.as_deref());

    let need_sys = config.memory.enabled || config.cpu.enabled;
    let sys = need_sys.then(system_for_fetch);

    let mut stats: Vec<InfoItem> = Vec::with_capacity(16);

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
            let (_, value) = memory(s, config.show_memory_bar);
            stats.push((config.memory.label.clone(), value));
        }
    }
    if config.disk.enabled {
        for item in disk(config.show_disk_bar, &config.disk.label) {
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

    let slug: String = args.logo.clone().unwrap_or_else(distro_slug);
    let slug = slug.trim();
    let slug = if slug.is_empty() { "fallback" } else { slug };
    let (logo_lines_vec, primary_color) = logos::get_logo(&slug);
    let logo_lines: &[&str] = logo_lines_vec.as_slice();

    render(RenderOptions {
        logo_lines,
        stats: &stats,
        primary_color: Some(primary_color),
        ascii_color: &config.colors.ascii,
        key_color: &config.colors.key,
        value_color: &config.colors.value,
        no_color: args.no_color,
        separator: SEPARATOR,
    });
}
