# NovaFetch 🦀

A blazingly fast, highly configurable system information tool written in Rust.

![Screenshot](path/to/screenshot.png)

---

## Features

- **Side-by-side layout** — ASCII art (distro logo) on the left, system info on the right. A strict layout engine keeps everything aligned regardless of logo size.
- **Persistent configuration** — TOML config at `~/.config/novafetch/config.toml`. Enable/disable modules and customize labels without recompiling.
- **Rich system info** — Supported modules:
  - **OS** — Distribution and architecture  
  - **Kernel** — Long version string  
  - **Uptime** — Human-readable uptime  
  - **Shell** — Name and version  
  - **Display** — Desktop environment / window manager (DE)  
  - **CPU** — Model name  
  - **GPU** — Detected graphics (OS-specific)  
  - **Memory** — Used/total with optional ASCII bar  
  - **Disk** — All relevant mounts (NVMe/SSD/HDD); virtual filesystems (squashfs, tmpfs, overlay, loop) filtered out  
  - **Terminal** — Detected via env vars or parent process  
  - **Terminal Font** — GNOME monospace font when available  
  - **Packages** — Count from one or more package managers
- **Multiple package managers** — Pacman, Dpkg, RPM, Flatpak, and Snap are detected; counts are shown per manager (e.g. `1251 (pacman), 12 (flatpak)`).

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

On first run, if no config file exists, NovaFetch creates a default one at **`~/.config/novafetch/config.toml`**. You can then enable/disable modules and change labels.

**Example:** disable the Packages line and rename "DE" to "Desktop":

```toml
[de]
enabled = true
label = "Desktop"

[packages]
enabled = false
label = "Packages"
```

Each module has `enabled` (bool) and `label` (string). Other options include `show_memory_bar`, `show_disk_bar`, and `[colors]` for `ascii`, `key`, and `value` colors.

**Override config path:** run with `--config /path/to/config.toml`. Disable colors with `--no-color`. Override the logo with `--logo <slug>` (e.g. `arch`, `ubuntu`, `fedora`, `macos`, `fallback`).

---

## Roadmap / Todo

- [ ] Add more ASCII logos (distros / variants)
- [ ] Broader support for macOS and Windows (detection and logos)
- [ ] Performance optimizations (e.g., fewer subprocess calls, cached results)
- [ ] Optional JSON/text output for scripting

---

## License

GPLv3 — GNU General Public License v3.0 (see [LICENSE](LICENSE)).
