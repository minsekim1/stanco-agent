use std::process::Command;

pub fn get_zabbix_config_path() -> Option<String> {
    let output = Command::new("zabbix_agentd")
        .arg("-p")
        .output()
        .ok()?; // 실패 시 None 반환

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("config") && line.contains(".conf") {
            return Some(line.trim().split_whitespace().last()?.to_string());
        }
    }

    None
}
