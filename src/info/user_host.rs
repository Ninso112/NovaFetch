pub fn get() -> (String, String) {
    let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".into());
    let user_host = format!("{}@{}", whoami::username(), hostname);
    (String::new(), user_host)
}
