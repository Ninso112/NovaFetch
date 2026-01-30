# NovaFetch

**Fast, customizable, rice-ready system fetch written in Rust.**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

---

## Features

- **TOML configuration** — Single config at `~/.config/novafetch/config.toml`. Reorder modules, set separator, theme, and ASCII options. Config is auto-generated on first run.
- **RGB gradients** — Labels use a configurable gradient (primary → secondary) or solid color. Values stay in text color for readability.
- **Modular layout** — Define exactly which modules appear and in what order via the `layout` array (e.g. `["os", "kernel", "uptime", "gpu", "media"]`).
- **Nerd Fonts support** — Optional icons per module (OS, CPU, GPU, Memory, Disk, Terminal, Media, etc.) when `use_nerd_fonts = true`.
- **Value alignment** — When `align_values = true`, separators line up vertically for a clean block of info.
- **Image / ASCII logos** — Use a custom image (PNG, JPG, GIF) as logo, or distro ASCII art side-by-side with info. Control via `ascii.print_ascii`, `ascii.distro_override`, and `general.image_path`.
- **Rich system info** — OS, Kernel, Uptime, Shell, DE, CPU, GPU, Memory, Disk, Terminal, Packages, Resolution, Swap, OS Age, Media (MPRIS), Local IP.
- **JSON output** — `--json` prints all collected info as JSON for scripting.

---

## Installation

**Prerequisites:** [Rust](https://www.rust-lang.org/) and Cargo (e.g. via [rustup](https://rustup.rs/)).

```bash
git clone <repo-url>
cd novafetch
cargo install --path .
```

Then run `novafetch` from anywhere (Cargo installs to `~/.cargo/bin` by default). Alternatively:

```bash
cargo build --release
./target/release/novafetch
```

---

## Configuration

On **first run**, if no config exists, NovaFetch creates a default config at:

- **`~/.config/novafetch/config.toml`** (Linux/macOS, XDG)
- Or the platform-appropriate config directory (e.g. Windows).

You can then edit this file to change colors, reorder the layout, set the separator, or override the ASCII distro.

### Example config

```toml
[general]
separator = "  "       # Between label and value (e.g. "  ", " -> ", " :: ")
use_nerd_fonts = true # Prepend Nerd Font icons to labels
align_values = true   # Align separators vertically
unit_type = "standard" # "standard" | "iec" | "si" for bytes
show_memory_bar = true
show_disk_bar = true
# image_path = "/path/to/logo.png"
# image_width = 36

[theme]
# RGB [R, G, B]. Primary = label start / solid; secondary = gradient end; text = value color
primary_color = [59, 130, 246]   # Blue
secondary_color = [147, 51, 234] # Purple
text_color = [255, 255, 255]     # White
mode = "gradient"                # "gradient" or "solid"

[ascii]
print_ascii = true               # Show distro ASCII art (or image if image_path set)
distro_override = null           # e.g. "arch", "ubuntu", "fedora", "macos", "fallback"

# Order and subset of modules. Only these keys are shown, in this order.
layout = [
  "user_host", "os", "kernel", "uptime", "shell", "de",
  "cpu", "gpu", "memory", "disk", "terminal", "terminal_font",
  "packages", "resolution", "swap", "os_age", "media", "local_ip"
]
```

### Layout keys

- `user_host` — User@host (no label)
- `os`, `kernel`, `uptime`, `shell`, `de`
- `cpu`, `gpu`, `memory`, `disk` (can emit multiple lines per mount)
- `terminal`, `terminal_font`, `packages`, `resolution`, `swap`, `os_age`
- `media` — MPRIS (e.g. Spotify) now playing
- `local_ip` — Local IPv4

Remove or reorder entries in `layout` to customize what you see.

### ASCII / distro override

- **`ascii.print_ascii`** — If `true`, show ASCII art (or image if `general.image_path` is set). If `false`, only the info block is printed.
- **`ascii.distro_override`** — Force a logo: `"arch"`, `"ubuntu"`, `"fedora"`, `"macos"`, `"kali"`, `"gentoo"`, `"windows"`, `"fallback"`, etc. If unset, the tool detects the distro.

**CLI overrides:** `--logo <name>` overrides the logo for this run; `--config /path/to/config.toml` uses a different config; `--no-color` disables colors; `--json` outputs JSON and exits.

---

## License

GPLv3 — GNU General Public License v3.0. See [LICENSE](LICENSE).
