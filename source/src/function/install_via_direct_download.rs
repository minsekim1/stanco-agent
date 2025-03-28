use std::fs::File;
use std::io::copy;
use std::process::Command;
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::blocking::get;

pub fn install_via_direct_download() -> bool {
	println!("[ERROR] 현재는 직접 다운로드 방식 설치는 지원하지 않습니다.");
	false

    // println!("\n[INFO 5/12] APT 없이 Zabbix Agent 설치 (Rust 압축 해제 방식)");

    // let url = "https://cdn.zabbix.com/zabbix/binaries/stable/6.0/6.0.23/zabbix_agents-6.0.23-linux-amd64-static.tar.gz";
    // let archive_path = "/tmp/zabbix_agent.tar.gz";
    // let extract_dir = "/tmp/zabbix_agent";

    // // 1. 다운로드
    // println!("[INFO] Zabbix Agent 바이너리 다운로드 중...");
    // let mut response = match get(url) {
    //     Ok(resp) => resp,
    //     Err(_) => {
    //         eprintln!("[ERROR] 다운로드 실패: 네트워크 또는 URL 문제");
    //         return false;
    //     }
    // };

    // let mut out = File::create(archive_path).expect("파일 생성 실패");
    // if copy(&mut response, &mut out).is_err() {
    //     eprintln!("[ERROR] 파일 저장 실패");
    //     return false;
    // }

    // // 2. 압축 해제
    // println!("[INFO] 압축 해제 중...");
    // if let Err(e) = std::fs::create_dir_all(extract_dir) {
    //     eprintln!("[ERROR] 디렉터리 생성 실패: {}", e);
    //     return false;
    // }

    // let tar_gz = File::open(archive_path).expect("압축 파일 열기 실패");
    // let decompressed = GzDecoder::new(tar_gz);
    // let mut archive = Archive::new(decompressed);
    // if let Err(e) = archive.unpack(extract_dir) {
    //     eprintln!("[ERROR] 압축 해제 실패: {}", e);
    //     return false;
    // }

    // // 3. 실행 파일 복사
    // let agent_path = format!("{}/zabbix_agents-6.0.23-linux3.0-amd64-static/sbin/zabbix_agentd", extract_dir);
    // let target_path = "/usr/local/bin/zabbix_agentd";

    // println!("[INFO] 실행 파일 복사 중...");
    // let status = Command::new("sudo")
    //     .args(["cp", &agent_path, target_path])
    //     .status()
    //     .expect("cp 실행 실패");

    // if !status.success() {
    //     eprintln!("[ERROR] 실행 파일 복사 실패. sudo 권한을 확인하세요.");
    //     return false;
    // }

    // println!("\n[INFO 7/12] Zabbix Agent 직접 설치 완료.");
    true
}
