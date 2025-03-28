use std::path::Path;

pub fn detect_os() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else if cfg!(target_os = "linux") {
        if std::path::Path::new("/etc/debian_version").exists() {
            "debian".to_string()
        } else if std::path::Path::new("/etc/redhat-release").exists() {
            "rhel".to_string()
        } else if std::path::Path::new("/etc/os-release").exists() {
            // fallback: 파싱도 가능
            "linux".to_string()
        } else {
            "unknown-linux".to_string()
        }
    } else {
        "unsupported".to_string()
    }
}
