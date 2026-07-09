use serde::{Deserialize, Serialize};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerStatus {
    pub available: bool,
    pub daemon_running: bool,
    pub version: Option<String>,
    pub compose_available: bool,
    pub user_in_docker_group: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerInfo {
    pub name: String,
    pub state: String,
    pub status: String,
    pub running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackStatus {
    pub running: bool,
    pub healthy: bool,
    pub containers: Vec<ContainerInfo>,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum DockerError {
    #[error("{0}")]
    Message(String),
}

pub fn check_docker_status() -> DockerStatus {
    let version = run_command("docker", &["--version"]);
    let compose = run_command("docker", &["compose", "version"]);
    let daemon = run_command("docker", &["info", "--format", "{{.ServerVersion}}"]);
    let user_in_group = current_user_in_docker_group();

    let available = version.is_ok();
    let daemon_running = daemon.is_ok();
    let compose_available = compose.is_ok();

    let message = if !available {
        "Docker CLI nicht gefunden. Bitte Docker Engine oder Docker Desktop installieren."
            .to_string()
    } else if !daemon_running {
        "Docker ist installiert, aber der Daemon läuft nicht. Starte den Docker-Dienst.".to_string()
    } else if !compose_available {
        "Docker Compose Plugin nicht gefunden.".to_string()
    } else if !user_in_group {
        "Docker läuft, aber dein Benutzer ist nicht in der docker-Gruppe. Nach der Installation neu anmelden.".to_string()
    } else {
        "Docker ist bereit.".to_string()
    };

    DockerStatus {
        available,
        daemon_running,
        version: version.ok(),
        compose_available,
        user_in_docker_group: user_in_group,
        message,
    }
}

pub fn current_user_in_docker_group() -> bool {
    let output = Command::new("id").arg("-nG").output();
    match output {
        Ok(out) if out.status.success() => {
            let groups = String::from_utf8_lossy(&out.stdout);
            groups.split_whitespace().any(|g| g == "docker")
        }
        _ => false,
    }
}

pub fn run_command(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("Befehl fehlgeschlagen: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(if stderr.is_empty() {
            format!("{program} {} fehlgeschlagen", args.join(" "))
        } else {
            stderr
        })
    }
}

async fn is_ui_reachable(port: u16) -> bool {
    let addr = format!("127.0.0.1:{port}");
    tokio::time::timeout(
        std::time::Duration::from_millis(800),
        tokio::net::TcpStream::connect(addr),
    )
    .await
    .map(|result| result.is_ok())
    .unwrap_or(false)
}

pub async fn get_stack_status_bollard(ui_port: u16) -> Result<StackStatus, DockerError> {
    let docker = bollard::Docker::connect_with_local_defaults()
        .map_err(|e| DockerError::Message(e.to_string()))?;

    let containers = docker
        .list_containers::<String>(Some(bollard::container::ListContainersOptions {
            all: true,
            ..Default::default()
        }))
        .await
        .map_err(|e| DockerError::Message(e.to_string()))?;

    let target_names = [
        "open-notebook-desktop-surrealdb",
        "open-notebook-desktop-app",
    ];
    let mut infos = Vec::new();

    for target in target_names {
        let found = containers.iter().find(|c| {
            c.names
                .as_ref()
                .is_some_and(|names| names.iter().any(|n| n.trim_start_matches('/') == target))
        });

        if let Some(container) = found {
            let state = container.state.as_deref().unwrap_or("unknown").to_string();
            let status = container.status.as_deref().unwrap_or("unknown").to_string();
            infos.push(ContainerInfo {
                name: target.to_string(),
                running: state == "running",
                state,
                status,
            });
        } else {
            infos.push(ContainerInfo {
                name: target.to_string(),
                running: false,
                state: "missing".to_string(),
                status: "missing".to_string(),
            });
        }
    }

    let running = infos.iter().all(|c| c.running);
    let healthy = running && is_ui_reachable(ui_port).await;

    let message = if running && healthy {
        "Open Notebook läuft.".to_string()
    } else if running {
        "Container laufen, Web-UI antwortet noch nicht.".to_string()
    } else {
        "Stack ist gestoppt.".to_string()
    };

    Ok(StackStatus {
        running,
        healthy,
        containers: infos,
        message,
    })
}

pub fn compose_up(data_dir: &str) -> Result<String, DockerError> {
    let output = Command::new("docker")
        .args(["compose", "up", "-d", "--remove-orphans"])
        .current_dir(data_dir)
        .output()
        .map_err(|e| DockerError::Message(e.to_string()))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(DockerError::Message(format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

pub fn compose_down(data_dir: &str) -> Result<String, DockerError> {
    let output = Command::new("docker")
        .args(["compose", "down"])
        .current_dir(data_dir)
        .output()
        .map_err(|e| DockerError::Message(e.to_string()))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(DockerError::Message(format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

pub fn compose_pull(data_dir: &str) -> Result<String, DockerError> {
    let output = Command::new("docker")
        .args(["compose", "pull"])
        .current_dir(data_dir)
        .output()
        .map_err(|e| DockerError::Message(e.to_string()))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(DockerError::Message(format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

pub fn compose_logs(data_dir: &str, tail: usize) -> Result<String, DockerError> {
    let tail_str = tail.to_string();
    let output = Command::new("docker")
        .args(["compose", "logs", "--no-color", "--tail", &tail_str])
        .current_dir(data_dir)
        .output()
        .map_err(|e| DockerError::Message(e.to_string()))?;

    Ok(format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ))
}

pub async fn wait_for_health_async(port: u16, timeout_secs: u64) -> bool {
    let deadline =
        tokio::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);

    while tokio::time::Instant::now() < deadline {
        if is_ui_reachable(port).await {
            return true;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    false
}

pub fn wait_for_health(_host: &str, port: u16, timeout_secs: u64) -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);
    let addr = format!("127.0.0.1:{port}");

    while std::time::Instant::now() < deadline {
        if std::net::TcpStream::connect_timeout(
            &addr.parse().expect("valid localhost address"),
            std::time::Duration::from_millis(800),
        )
        .is_ok()
        {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    false
}
