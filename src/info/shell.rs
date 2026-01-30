use std::env;
use std::process::Command;

pub fn get() -> (String, String) {
    let shell_path = env::var("SHELL").unwrap_or_else(|_| "unknown".into());
    let name = shell_path
        .rsplit('/')
        .next()
        .unwrap_or(shell_path.as_str());
    let version = get_shell_version(&shell_path);
    let shell = if version.is_empty() {
        name.to_string()
    } else {
        format!("{} {}", name, version)
    };
    ("Shell".into(), shell)
}

fn get_shell_version(shell_path: &str) -> String {
    let out = Command::new(shell_path).arg("--version").output();
    let out = match out {
        Ok(o) if o.status.success() => o,
        _ => return String::new(),
    };
    let s = String::from_utf8_lossy(&out.stdout);
    let first_line = s.lines().next().unwrap_or("");
    let version = first_line
        .split_whitespace()
        .find(|w| w.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false));
    version
        .map(|v| {
            v.chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect::<String>()
        })
        .filter(|v| !v.is_empty())
        .unwrap_or_default()
}
