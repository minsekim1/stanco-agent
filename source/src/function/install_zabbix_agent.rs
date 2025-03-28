use std::process::Command;

mod function;
use function::detect_os::detect_os;
use function::has_command::has_command;
use function::install_via_direct_download::install_via_direct_download;

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

pub fn install_zabbix_agent() -> bool {
    match detect_os().as_str() {
        "windows" => {
            println!("[ERROR] 현재는 Windows 설치는 지원하지 않습니다.");
            false
        }
        "debian" => install_via_apt(),
        "rhel" => install_via_yum(),
        "linux" => {
            if has_command("apt") {
                install_via_apt()
            } else if has_command("yum") {
                install_via_yum()
            } else if has_command("zypper") {
                install_via_zypper()
            } else {
                install_via_direct_download()
            }
        }
        _ => {
            println!("[ERROR] 지원하지 않는 운영체제입니다.");
            false
        }
    }
}