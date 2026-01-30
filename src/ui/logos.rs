//! Distro-specific ASCII art and primary colors.
//! Art extracted from Neofetch (https://github.com/dylanaraps/neofetch), Bash variables removed.

use colored::Color;

// --- Arch Linux (neofetch arch_small) ---
const ARCH_ASCII: &str = r#"
      /\
     /  \
    /\   \
   /      \
  /   ,,   \
 /   |  |  -\
/_-''    ''-_\
"#;

// --- Debian (neofetch Debian full logo) ---
const DEBIAN_ASCII: &str = r#"
       _,met$$$$$gg.
    ,g$$$$$$$$$$$$$$$P.
  ,g$$P"        """Y$$.".
 ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,$P"'   .    $$$
 $$P      d$'     ,    $$P
 $$:      $$.   -    ,d$$'
 $$;      Y$b._   _,d$P'
 Y$$.    `.`"Y$$$$P"'
 `$$b      "-.__
  `Y$$
   `Y$$.
     `$$b.
       `Y$$b.
          `"Y$b._
              `"""
"#;

// --- Ubuntu (neofetch Ubuntu) ---
const UBUNTU_ASCII: &str = r#"
            .-/+oossssoo+-.
        ´:+ssssssssssssssssss+:`
      -+ssssssssssssssssssyyssss+-
    .ossssssssssssssssssdMMMNysssso.
   /ssssssssssshdmmNNmmyNMMMMhssssss\
  +ssssssssshmydMMMMMMMNddddyssssssss+
 /sssssssshNMMMyhhyyyhmNMMMNhssssssss\
.ssssssssdMMMNhsssssssssshNMMMdssssssss.
+sssshhhyNMMNyssssssssssssyNMMMysssssss+
ossyNMMMNyMMhsssssssssssssshmmmhssssssso
ossyNMMMNyMMhsssssssssssssshmmmhssssssso
+sssshhhyNMMNyssssssssssssyNMMMysssssss+
.ssssssssdMMMNhsssssssssshNMMMdssssssss.
 \sssssssshNMMMyhhyyyhdNMMMNhssssssss/
  +sssssssssdmydMMMMMMMMddddyssssssss+
   \ssssssssssshdmNNNNmyNMMMMhssssss/
    .ossssssssssssssssssdMMMNysssso.
      -+sssssssssssssssssyyyssss+-
        `:+ssssssssssssssssss+:`
            .-\+oossssoo+/-.
"#;

// --- Fedora (neofetch Fedora) ---
const FEDORA_ASCII: &str = r#"
             .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
  .;ccccccccccccc;.:dddl:.;ccccccc;.
 .:ccccccccccccc;OWMKOOXMWd;ccccccc:.
.:ccccccccccccc;KMMc;cc;xMMc;ccccccc:.
,cccccccccccccc;MMM.;cc;;WW:;cccccccc,
:cccccccccccccc;MMM.;cccccccccccccc:
:ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:
cccccc;0MMKxdd:;MMMkddc.;cccccccccccc;
ccccc;XM0';cccc;MMM.;cccccccccccccccc'
ccccc;MMo;ccccc;MMW.;ccccccccccccccc;
ccccc;0MNc.ccc.xMMd;ccccccccccccccc;
cccccc;dNMWXXXWM0:;cccccccccccccc:,
cccccccc;.:odl:.;cccccccccccccc:,.
:cccccccccccccccccccccccccccc:'.
.:cccccccccccccccccccccc:;,..
  '::cccccccccccccc::;,.
"#;

// --- Linux Mint (neofetch LinuxMint) ---
const MINT_ASCII: &str = r#"
  \_____/
   \   /
    \ /
   _/ \_
  (     )
   \___/
  (     )
 (       )
"#;

// --- Windows 10 (neofetch Windows 10) ---
const WINDOWS10_ASCII: &str = r#"
                                ..,
                    ....,,:;+ccllll
      ...,,+:;  cllllllllllllllllll
,cclllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll

llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
`'ccllllllllll  lllllllllllllllllll
       `' \*::  :ccllllllllllllllll
                       ````''*::cll
                                 ``
"#;

// --- Windows 11 (neofetch Windows 11) ---
const WINDOWS11_ASCII: &str = r#"

################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################

################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
"#;

// --- macOS / Darwin (neofetch mac/Darwin) ---
const MACOS_ASCII: &str = r#"
                    c.'
                 ,xNMM.
               .OMMMMo
               lMM"
     .;loddo:.  .olloddol;.
   cKMMMMMMMMMMNWMMMMMMMMMM0:
 .KMMMMMMMMMMMMMMMMMMMMMMMWd.
 XMMMMMMMMMMMMMMMMMMMMMMMX.
;MMMMMMMMMMMMMMMMMMMMMMMM:
:MMMMMMMMMMMMMMMMMMMMMMMM:
.MMMMMMMMMMMMMMMMMMMMMMMMMX.
 kMMMMMMMMMMMMMMMMMMMMMMMMWd.
 'XMMMMMMMMMMMMMMMMMMMMMMMMMMk
  'XMMMMMMMMMMMMMMMMMMMMMMMMK.
    kMMMMMMMMMMMMMMMMMMMMMMd
     ;KMMMMMMMWXXWMMMMMMMk.
       "cooc*"    "*coo'"
"#;

// --- Fallback (Tux-style, simple) ---
const FALLBACK_ASCII: &str = r#"
   .---.
  /     \
 | .   . |
  \  ~  /
   \_/
  (   )
   ( )
"#;

fn lines_from_raw(raw: &'static str) -> Vec<&'static str> {
    raw.trim_matches('\n').split('\n').collect()
}

fn arch_lines() -> Vec<&'static str> {
    lines_from_raw(ARCH_ASCII)
}
fn debian_lines() -> Vec<&'static str> {
    lines_from_raw(DEBIAN_ASCII)
}
fn ubuntu_lines() -> Vec<&'static str> {
    lines_from_raw(UBUNTU_ASCII)
}
fn fedora_lines() -> Vec<&'static str> {
    lines_from_raw(FEDORA_ASCII)
}
fn mint_lines() -> Vec<&'static str> {
    lines_from_raw(MINT_ASCII)
}
fn windows10_lines() -> Vec<&'static str> {
    lines_from_raw(WINDOWS10_ASCII)
}
fn windows11_lines() -> Vec<&'static str> {
    lines_from_raw(WINDOWS11_ASCII)
}
fn macos_lines() -> Vec<&'static str> {
    lines_from_raw(MACOS_ASCII)
}
fn fallback_lines() -> Vec<&'static str> {
    lines_from_raw(FALLBACK_ASCII)
}

/// Normalize a distro slug for lookup: lowercase, no spaces.
pub fn normalize_slug(s: &str) -> String {
    s.to_lowercase().replace(' ', "")
}

/// Returns ASCII art lines and primary color for the given distro slug.
/// Unknown slugs return the fallback (Tux-style) logo with default color.
pub fn get_logo(slug: &str) -> (Vec<&'static str>, Color) {
    let slug = normalize_slug(slug);
    let (lines, color) = match slug.as_str() {
        "arch" | "archlinux" => (arch_lines(), Color::Cyan),
        "debian" => (debian_lines(), Color::Red),
        "ubuntu" => (ubuntu_lines(), Color::Red),
        "fedora" => (fedora_lines(), Color::Blue),
        "linuxmint" | "mint" => (mint_lines(), Color::Green),
        "windows10" | "windows8" | "windows" => (windows10_lines(), Color::BrightBlue),
        "windows11" => (windows11_lines(), Color::BrightBlue),
        "macos" | "darwin" | "apple" | "mac" => (macos_lines(), Color::BrightWhite),
        _ => (fallback_lines(), Color::Cyan),
    };
    (lines, color)
}

/// List of supported distro slug names for help/CLI.
#[allow(dead_code)]
pub fn supported_slugs() -> &'static [&'static str] {
    &[
        "arch",
        "debian",
        "ubuntu",
        "fedora",
        "mint",
        "windows",
        "windows10",
        "windows11",
        "macos",
        "fallback",
    ]
}
