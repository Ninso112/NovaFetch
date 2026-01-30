// Legacy logo module; distro logos are in logos.rs.
#![allow(dead_code)]

fn default_logo() -> &'static [&'static str] {
    &[
        "  _____  ",
        " |     | ",
        " |  *  | ",
        " |     | ",
        "  \\___/  ",
        "   / \\   ",
        "  (___)  ",
        "         ",
    ]
}

fn arch_logo() -> &'static [&'static str] {
    &[
        "    /\\     ",
        "   /  \\    ",
        "  /\\   \\   ",
        " /  \\   \\  ",
        "/    \\   \\ ",
        "\\    \\   / ",
        " \\   \\  /  ",
        "  \\   \\/   ",
    ]
}

fn ubuntu_logo() -> &'static [&'static str] {
    &[
        "  ____  ",
        " / __ \\ ",
        "| |  | |",
        "| |  | |",
        "| |__| |",
        " \\____/ ",
        "        ",
        "        ",
    ]
}

fn penguin_logo() -> &'static [&'static str] {
    &[
        "   .---.   ",
        "  /     \\  ",
        " | o   o | ",
        "  \\  ~  /  ",
        "   \\___/   ",
        "  (     )  ",
        "   (   )   ",
        "    ( )    ",
    ]
}

/// Returns ASCII art lines for the given logo name (case-insensitive).
/// Unknown names fall back to the default logo.
pub fn get_logo(name: &str) -> &'static [&'static str] {
    let name = name.to_lowercase();
    match name.as_str() {
        "arch" => arch_logo(),
        "ubuntu" => ubuntu_logo(),
        "penguin" | "linux" => penguin_logo(),
        _ => default_logo(),
    }
}

/// List of built-in logo names for help text.
#[allow(dead_code)]
pub fn logo_names() -> Vec<&'static str> {
    vec!["default", "arch", "ubuntu", "penguin"]
}
