use std::process::Command;

pub fn check_network() -> bool {
    Command::new("ping")
        .args(["-c", "3", "8.8.8.8"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}