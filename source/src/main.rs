use clap::Parser;
use std::process::{Command, exit};
use std::fs::File;
use uuid::Uuid;
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::blocking::get;
use std::io::copy;

mod function;
use function::has_command::has_command;
use function::check_network::check_network;

/// Stanco Agent Installer
#[derive(Parser)]
#[command(name = "stanco-agent")]
#[command(about = "Zabbix + Wazuh 통합 모니터링 에이전트 설치", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// 에이전트 설치
    Install {
        /// 회사명
        #[arg(long)]
        company: String,
        /// 제품명
        #[arg(long)]
        product: String,
        /// SBOM Key
        #[arg(long)]
        sbomkey: String,
        /// 위치정보 사용 여부 (true/false)
        #[arg(long, default_value = "false")]
        use_location: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install {
            company,
            product,
            sbomkey,
            use_location,
        } => {
            println!("\n[INFO 1/12] 모니터링 프로그램 설치 시작");

            // 2단계: 네트워크 연결 확인
            println!("\n[INFO 2/12] 네트워크 연결 확인 중...");
            if !check_network() {
                eprintln!("\n[ERROR-2] 네트워크 연결이 없습니다. 인터넷 상태를 확인하세요.");
                exit(1);
            }
            println!("[INFO 2/12] 네트워크 연결 확인 완료.");

            // 3단계: UUID 생성
            println!("\n[INFO 3/12] UUID 생성 중...");
            let uuid = Uuid::new_v4().to_string();
            println!("[INFO 3/12] UUID 생성 완료: {}", uuid);

            // 4단계: Host 정보 구성
            let hostname = format!("{}_{}_{}", company, product, uuid);
            let host_metadata = format!(
                "mode=production,company={},product={},sbom_key={}",
                company, product, sbomkey
            );

            println!("\n[INFO 4/12] 설정 값");
            println!("    >> 회사명: {}", company);
            println!("    >> 제품명: {}", product);
            println!("    >> SBOM Key: {}", sbomkey);
            println!("    >> 호스트명: {}", hostname);
            println!("    >> HostMetadata: {}", host_metadata);
            println!("    >> Use Location: {}", use_location);

            // 5~7단계: 시스템 업데이트 및 Zabbix 설치 
            // 명령어 확인
            if !install_zabbix_agent() {
                exit(1);
            } 
        }
    }
}



fn install_zabbix_agent() -> bool {
    if has_command("apt") {
        println!("\n[INFO] APT 기반 설치 시도");
        install_via_apt()
    } else {
        println!("\n[INFO] APT 없음 → 직접 설치 방식 사용");
        install_via_direct_download()
    }
}

fn install_via_apt() -> bool {
    println!("\n[INFO 5/12] 시스템 업데이트 중...");

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

fn install_via_direct_download() -> bool {
    println!("\n[INFO 5/12] APT 없이 Zabbix Agent 설치 (Rust 압축 해제 방식)");

    let url = "https://cdn.zabbix.com/zabbix/binaries/stable/6.0/6.0.23/zabbix_agents-6.0.23-linux-amd64-static.tar.gz";
    let archive_path = "/tmp/zabbix_agent.tar.gz";
    let extract_dir = "/tmp/zabbix_agent";

    // 1. 다운로드
    println!("[INFO] Zabbix Agent 바이너리 다운로드 중...");
    let mut response = match get(url) {
        Ok(resp) => resp,
        Err(_) => {
            eprintln!("[ERROR] 다운로드 실패: 네트워크 또는 URL 문제");
            return false;
        }
    };

    let mut out = File::create(archive_path).expect("파일 생성 실패");
    if copy(&mut response, &mut out).is_err() {
        eprintln!("[ERROR] 파일 저장 실패");
        return false;
    }

    // 2. 압축 해제
    println!("[INFO] 압축 해제 중...");
    if let Err(e) = std::fs::create_dir_all(extract_dir) {
        eprintln!("[ERROR] 디렉터리 생성 실패: {}", e);
        return false;
    }

    let tar_gz = File::open(archive_path).expect("압축 파일 열기 실패");
    let decompressed = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decompressed);
    if let Err(e) = archive.unpack(extract_dir) {
        eprintln!("[ERROR] 압축 해제 실패: {}", e);
        return false;
    }

    // 3. 실행 파일 복사
    let agent_path = format!("{}/zabbix_agents-6.0.23-linux3.0-amd64-static/sbin/zabbix_agentd", extract_dir);
    let target_path = "/usr/local/bin/zabbix_agentd";

    println!("[INFO] 실행 파일 복사 중...");
    let status = Command::new("sudo")
        .args(["cp", &agent_path, target_path])
        .status()
        .expect("cp 실행 실패");

    if !status.success() {
        eprintln!("[ERROR] 실행 파일 복사 실패. sudo 권한을 확인하세요.");
        return false;
    }

    println!("\n[INFO 7/12] Zabbix Agent 직접 설치 완료.");
    true
}


