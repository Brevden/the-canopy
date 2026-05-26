use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use std::process::Stdio;

// The shape podman search --format json returns.
// Fields are PascalCase in Podman 4.x+.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
    pub index: String,
    pub name: String,
    pub description: String,
    pub stars: u64,
    pub official: String,
}

pub async fn search_images(query: &str) -> Result<Vec<SearchResult>> {
    let output = Command::new("podman")
        .args(["search", query, "--format", "json", "--limit", "25"])
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("podman search failed: {}", stderr);
    }

    let results = serde_json::from_slice(&output.stdout).unwrap_or_default();
    Ok(results)
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub state: String,
    pub status: String,
}

pub async fn list_containers() -> Result<Vec<ContainerInfo>> {
    let output = Command::new("podman")
        .args(["ps", "--all", "--format", "json"])
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("podman ps failed: {}", stderr);
    }

    let results = serde_json::from_slice(&output.stdout).unwrap_or_default();
    Ok(results)
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub cpu: u8,
    pub memory_used_gb: f64,
    pub memory_total_gb: f64,
    pub disk_used_gb: f64,
    pub disk_total_gb: f64,
}

pub async fn system_info() -> Result<SystemInfo> {
    let hostname = std::fs::read_to_string("/etc/hostname")
        .unwrap_or_default()
        .trim()
        .to_string();

    // Memory from /proc/meminfo (values in kB)
    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut mem_total_kb = 0u64;
    let mut mem_available_kb = 0u64;
    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            mem_total_kb = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            mem_available_kb = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
        }
    }
    let mem_used_kb = mem_total_kb.saturating_sub(mem_available_kb);
    let kb_to_gb = |kb: u64| (kb as f64 / 1_048_576.0 * 100.0).round() / 100.0;

    // CPU: load1 / num_cpus as a percentage proxy
    let loadavg = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let load1: f64 = loadavg.split_whitespace().next().unwrap_or("0").parse().unwrap_or(0.0);
    let num_cpus = std::fs::read_to_string("/proc/cpuinfo")
        .unwrap_or_default()
        .lines()
        .filter(|l| l.starts_with("processor"))
        .count()
        .max(1);
    let cpu_pct = ((load1 / num_cpus as f64) * 100.0).round().min(100.0) as u8;

    // Disk: run `df -B1 /` and parse
    let disk_out = Command::new("df")
        .args(["-B1", "/"])
        .output()
        .await
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::default(),
            stdout: vec![],
            stderr: vec![],
        });
    let disk_str = String::from_utf8_lossy(&disk_out.stdout);
    let mut disk_total_b = 0u64;
    let mut disk_used_b = 0u64;
    if let Some(line) = disk_str.lines().nth(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() >= 3 {
            disk_total_b = cols[1].parse().unwrap_or(0);
            disk_used_b = cols[2].parse().unwrap_or(0);
        }
    }
    let b_to_gb = |b: u64| (b as f64 / 1_073_741_824.0 * 10.0).round() / 10.0;

    Ok(SystemInfo {
        hostname,
        cpu: cpu_pct,
        memory_used_gb: kb_to_gb(mem_used_kb),
        memory_total_gb: kb_to_gb(mem_total_kb),
        disk_used_gb: b_to_gb(disk_used_b),
        disk_total_gb: b_to_gb(disk_total_b),
    })
}

pub async fn pull_image(image: &str) -> Result<()> {
    let status = Command::new("podman")
        .args(["pull", image])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .status()
        .await?;

    if !status.success() {
        anyhow::bail!("podman pull failed for image: {}", image);
    }

    Ok(())
}
