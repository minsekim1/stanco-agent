use clap::Parser;
use std::process::{exit};
use uuid::Uuid;

mod function;
use function::check_network::check_network;
use function::install_zabbix_agent::install_zabbix_agent;

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
            println!("\n[INFO 5/12] 시스템 업데이트 중...");
            if !install_zabbix_agent() {
                eprintln!("\n[ERROR-5] Zabbix Agent 설치 실패. 네트워크 또는 권한 문제가 있을 수 있습니다.");
                exit(1);
            } 
        }
    }
}








