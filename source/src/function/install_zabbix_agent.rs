use std::process::Command;

use super::has_command::has_command;
use super::install_via_direct_download::install_via_direct_download;

fn install_via_yum() -> bool {
    println!("[INFO] YUM 기반 설치 진행 중...");
    let install_status = Command::new("sudo")
        .args(["yum", "install", "-y", "zabbix-agent"])
        .status()
        .expect("yum install 실행 실패");

    install_status.success()
}

fn install_via_zypper() -> bool {
    println!("[INFO] Zypper 기반 설치 진행 중...");
    let install_status = Command::new("sudo")
        .args(["zypper", "install", "-y", "zabbix-agent"])
        .status()
        .expect("zypper install 실행 실패");

    install_status.success()
}

fn install_via_apt() -> bool {
    

    let update_status = Command::new("sudo")
        .args(["apt", "update", "-qq"])
        .status()
        .expect("apt update 실행 실패");

    let upgrade_status = Command::new("sudo")
        .args(["apt", "upgrade", "-y", "--no-install-recommends"])
        .status()
        .expect("apt upgrade 실행 실패");

    if !update_status.success() || !upgrade_status.success() {
        eprintln!("\n[ERROR-4] 시스템 업데이트 실패. 네트워크 상태 또는 sudo 권한을 확인하세요.");
        return false;
    }

    println!("\n[INFO 5/12] 시스템 업데이트 완료.");
    println!("\n[INFO 6/12] Zabbix Agent 설치 중...");

    let install_status = Command::new("sudo")
        .args(["apt", "install", "-y", "zabbix-agent"])
        .status()
        .expect("zabbix-agent 설치 실행 실패");

    if !install_status.success() {
        eprintln!("\n[ERROR-7] Zabbix Agent 설치 실패. apt 상태나 인터넷 연결을 확인하세요.");
        return false;
    }

    println!("\n[INFO 7/12] Zabbix Agent 설치 완료.");
    true
}

fn install_via_brew() -> bool {
    println!("[INFO] Homebrew 기반 Zabbix Agent 설치 중...");
    let status = Command::new("brew")
        .args(["install", "zabbix-cli"])
        .status()
        .expect("brew 실행 실패");

    if status.success() {
        println!("[INFO] Zabbix Agent 설치 완료 (brew)");
        true
    } else {
        eprintln!("[ERROR] Zabbix Agent 설치 실패 (brew). Homebrew 상태를 확인하세요.");
        false
    }
}

fn detect_os() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    } else {
        "unsupported".to_string()
    }
}

pub fn install_zabbix_agent() -> bool {
    let os_type = detect_os();

    match os_type.as_str() {
        "windows" => {
            println!("[ERROR] 현재는 Windows 설치는 지원하지 않습니다.");
            return false;
        }
        "linux" => {
            if has_command("apt") {
                install_via_apt()
            } else if has_command("yum") {
                install_via_yum()
            } else if has_command("zypper") {
                install_via_zypper()
            } else {
                println!("[ERROR] 지원되지 않는 패키지 관리자입니다. apt, yum, zypper 중 하나를 설치해주세요.");
                return false;
            }
        }
        "macos" => {
            if has_command("brew") {
                install_via_brew()
            } else {
                println!("[ERROR] Homebrew가 설치되어 있지 않습니다.");
                return false;
            }
        }
        _ => {
            println!("[ERROR] 지원하지 않는 운영체제입니다: {}", os_type);
            return false;
        }
    }
}