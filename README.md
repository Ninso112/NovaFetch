# NovaFetch 🦀

A blazingly fast, highly configurable system information tool written in Rust.

![Screenshot](path/to/screenshot.png)

---

## Features

- **Side-by-side layout** — ASCII art (distro logo) on the left, system info on the right. A strict layout engine keeps everything aligned regardless of logo size.
- **Persistent configuration** — TOML config at `~/.config/novafetch/config.toml`. Enable/disable modules, customize labels, and choose byte units without recompiling.
- **Configurable byte units** — `unit_type` in config:
  - **`standard`** (default): Base 1024, labels "KB", "MB", "GB" (Windows style).
  - **`iec`**: Base 1024, labels "KiB", "MiB", "GiB" (Linux technical style).
  - **`si`**: Base 1000, labels "KB", "MB", "GB" (disk manufacturer style).
- **Rich system info** — Supported modules:
  - **OS** — Distribution and architecture  
  - **Kernel** — Long version string  
  - **Uptime** — Human-readable uptime  
  - **Shell** — Name and version  
  - **DE** — Desktop environment / window manager  
  - **CPU** — Model name  
  - **GPU** — Detected graphics (OS-specific)  
  - **Memory** — Used/total with optional ASCII bar (respects `unit_type`)  
  - **Disk** — Relevant mounts with filesystem type (e.g. ext4, btrfs), usage %, and size (virtual filesystems filtered out)  
  - **Terminal** — Detected via env vars or parent process  
  - **Terminal Font** — GNOME monospace font when available  
  - **Packages** — Count from one or more package managers (Pacman/Dpkg use file I/O on Linux for speed)  
  - **Resolution** — Monitor resolution and refresh rate (e.g. `1920x1080 @ 144Hz`)  
  - **Swap** — Used/total and percentage  
  - **OS Age** — Approximate install age (Linux: from root/installer dir birth time)
- **Multiple package managers** — Pacman, Dpkg, RPM, Flatpak, and Snap; counts shown per manager (e.g. `1251 (pacman), 12 (flatpak)`).
- **JSON output** — `--json` prints all info as JSON. Memory and swap include raw byte fields (`memory_used_bytes`, `memory_total_bytes`, `swap_used_bytes`, `swap_total_bytes`) for scripting.

---

## Installation

**Prerequisites:** [Rust](https://www.rust-lang.org/) and Cargo (install via [rustup](https://rustup.rs/)).

```bash
git clone <repo-url>
cd novafetch
cargo build --release
./target/release/novafetch
```

Optional: add `target/release/novafetch` to your `PATH` or copy it to a directory already in `PATH` (e.g. `~/.local/bin`).

---

## Configuration

On first run, if no config file exists, NovaFetch creates a default one at **`~/.config/novafetch/config.toml`**. You can then enable/disable modules, change labels, and set the byte unit type.

**Example:** disable Packages, rename DE, and use IEC units (KiB/MiB/GiB):

```toml
[de]
enabled = true
label = "Desktop"

[packages]
enabled = false
label = "Packages"

# Byte display: "standard" (KB/MB/GB, 1024), "iec" (KiB/MiB/GiB), "si" (KB/MB/GB, 1000)
unit_type = "iec"
```

Each module has `enabled` (bool) and `label` (string). Other options: `unit_type`, `show_memory_bar`, `show_disk_bar`, and `[colors]` for `ascii`, `key`, and `value` colors.

**CLI:** `--config /path/to/config.toml`, `--no-color`, `--logo <slug>` (e.g. `arch`, `ubuntu`, `fedora`, `macos`, `kali`, `gentoo`, `windows`, `fallback`), `--json` (output JSON and exit).

---

## Roadmap / Todo

- [ ] Add more ASCII logos (distros / variants)
- [ ] Broader support for macOS and Windows (detection and logos)
- [ ] Performance optimizations (e.g., fewer subprocess calls, cached results)

---

## License

GPLv3 — GNU General Public License v3.0 (see [LICENSE](LICENSE)).
