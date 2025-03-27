use clap::{Parser, Subcommand};

/// Stanco Agent CLI
#[derive(Parser)]
#[command(name = "stanco-agent")]
#[command(about = "Zabbix + Wazuh 기반 통합 에이전트", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// SBOM Key를 기반으로 에이전트 설치
    Install {
        /// 설치에 사용할 SBOM Key
        #[arg(long)]
        sbomkey: String,
    },
    /// 에이전트 제거
    Uninstall,
    /// 에이전트 상태 확인
    Status,
    /// 버전 정보 출력
    Version,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { sbomkey } => {
            println!("🔧 설치 시작 - SBOM Key: {}", sbomkey);
            // TODO: 설치 로직 구현
        }
        Commands::Uninstall => {
            println!("🧹 에이전트 제거 중...");
            // TODO: 제거 로직 구현
        }
        Commands::Status => {
            println!("📊 에이전트 상태 확인 중...");
            // TODO: 상태 확인 로직
        }
        Commands::Version => {
            println!("stanco-agent version 0.1.0");
        }
    }
}
