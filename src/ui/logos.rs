//! Distro-specific ASCII art and primary colors.
//! Data lives in ascii_data.rs; this module handles matching and colors.

use colored::Color;

use super::ascii_data;

fn lines_from_raw(raw: &'static str) -> Vec<&'static str> {
    raw.trim_matches('\n').split('\n').collect()
}

/// Normalize a distro slug for lookup: lowercase, no spaces.
pub fn normalize_slug(s: &str) -> String {
    s.to_lowercase().replace(' ', "")
}

/// Returns ASCII art lines and primary color for the given distro slug.
/// Uses family fallback (e.g. "kubuntu" -> Ubuntu) and finally Tux fallback.
pub fn get_logo(slug: &str) -> (Vec<&'static str>, Color) {
    let slug = normalize_slug(slug);

    // Direct match: slug -> (art constant, color)
    let (raw, color) = match slug.as_str() {
        // Major families
        "arch" | "archlinux" => (ascii_data::ARCH, Color::Cyan),
        "debian" => (ascii_data::DEBIAN, Color::Red),
        "ubuntu" => (ascii_data::UBUNTU, Color::BrightRed),
        "fedora" => (ascii_data::FEDORA, Color::Blue),
        "opensuse" | "suse" | "sles" => (ascii_data::OPENSUSE, Color::Green),
        "gentoo" => (ascii_data::GENTOO, Color::Magenta),
        "slackware" => (ascii_data::SLACKWARE, Color::Blue),
        "rhel" | "redhat" => (ascii_data::RHEL, Color::Red),
        // Top derivatives
        "linuxmint" | "mint" => (ascii_data::MINT, Color::Green),
        "manjaro" => (ascii_data::MANJARO, Color::Green),
        "endeavouros" | "endeavour" => (ascii_data::ENDEAVOUROS, Color::Magenta),
        "popos" | "pop_os" | "pop!_os" => (ascii_data::POP_OS, Color::Cyan),
        "mxlinux" | "mx" => (ascii_data::MX_LINUX, Color::Green),
        "zorinos" | "zorin" => (ascii_data::ZORIN, Color::Blue),
        "elementary" | "elementaryos" => (ascii_data::ELEMENTARY, Color::Blue),
        "kali" | "kalilinux" => (ascii_data::KALI, Color::Blue),
        "parrot" | "parrotos" => (ascii_data::PARROT, Color::BrightWhite),
        "garuda" | "garudalinux" => (ascii_data::GARUDA, Color::Cyan),
        "nobara" | "nobaralinux" => (ascii_data::NOBARA, Color::Green),
        // Server / Enterprise
        "almalinux" | "alma" => (ascii_data::ALMALINUX, Color::Red),
        "rocky" | "rockylinux" => (ascii_data::ROCKY, Color::Blue),
        "centos" => (ascii_data::CENTOS, Color::Blue),
        "alpine" => (ascii_data::ALPINE, Color::Blue),
        "oraclelinux" | "oracle" => (ascii_data::ORACLE_LINUX, Color::Red),
        // Others
        "nixos" | "nix" => (ascii_data::NIXOS, Color::Cyan),
        "void" | "voidlinux" => (ascii_data::VOID, Color::Green),
        "solus" => (ascii_data::SOLUS, Color::Blue),
        "puppy" | "puppylinux" => (ascii_data::PUPPY, Color::BrightYellow),
        "freebsd" | "bsd" => (ascii_data::FREEBSD, Color::Red),
        "raspbian" | "raspberrypi" | "raspi" => (ascii_data::RASPBIAN, Color::Red),
        // Windows / macOS
        "windows10" | "windows8" | "windows7" => (ascii_data::WINDOWS10, Color::BrightBlue),
        "windows11" | "windows" => (ascii_data::WINDOWS11, Color::BrightBlue),
        "macos" | "darwin" | "apple" | "mac" => (ascii_data::MACOS, Color::BrightWhite),
        "fallback" => (ascii_data::FALLBACK, Color::Cyan),
        _ => {
            // Family fallback: slug contains family name -> use that logo
            let slug_lower = slug.as_str();
            if slug_lower.contains("arch") {
                (ascii_data::ARCH, Color::Cyan)
            } else if slug_lower.contains("debian") {
                (ascii_data::DEBIAN, Color::Red)
            } else if slug_lower.contains("ubuntu") || slug_lower.contains("kubuntu") || slug_lower.contains("xubuntu") || slug_lower.contains("lubuntu") {
                (ascii_data::UBUNTU, Color::BrightRed)
            } else if slug_lower.contains("fedora") || slug_lower.contains("rhel") || slug_lower.contains("redhat") {
                (ascii_data::FEDORA, Color::Blue)
            } else if slug_lower.contains("suse") || slug_lower.contains("sles") {
                (ascii_data::OPENSUSE, Color::Green)
            } else if slug_lower.contains("gentoo") {
                (ascii_data::GENTOO, Color::Magenta)
            } else if slug_lower.contains("mint") {
                (ascii_data::MINT, Color::Green)
            } else if slug_lower.contains("manjaro") {
                (ascii_data::MANJARO, Color::Green)
            } else if slug_lower.contains("centos") || slug_lower.contains("rocky") || slug_lower.contains("alma") {
                (ascii_data::RHEL, Color::Blue)
            } else if slug_lower.contains("alpine") {
                (ascii_data::ALPINE, Color::Blue)
            } else if slug_lower.contains("kali") {
                (ascii_data::KALI, Color::Blue)
            } else if slug_lower.contains("nix") {
                (ascii_data::NIXOS, Color::Cyan)
            } else if slug_lower.contains("void") {
                (ascii_data::VOID, Color::Green)
            } else if slug_lower.contains("freebsd") || slug_lower.contains("bsd") {
                (ascii_data::FREEBSD, Color::Red)
            } else {
                (ascii_data::FALLBACK, Color::Cyan)
            }
        }
    };

    (lines_from_raw(raw), color)
}

/// List of supported distro slug names for help/CLI.
#[allow(dead_code)]
pub fn supported_slugs() -> &'static [&'static str] {
    &[
        "arch",
        "debian",
        "ubuntu",
        "fedora",
        "opensuse",
        "gentoo",
        "slackware",
        "rhel",
        "mint",
        "manjaro",
        "endeavouros",
        "pop_os",
        "mx",
        "zorin",
        "elementary",
        "kali",
        "parrot",
        "garuda",
        "nobara",
        "almalinux",
        "rocky",
        "centos",
        "alpine",
        "oracle",
        "nixos",
        "void",
        "solus",
        "puppy",
        "freebsd",
        "raspbian",
        "windows",
        "windows11",
        "macos",
        "fallback",
    ]
}
