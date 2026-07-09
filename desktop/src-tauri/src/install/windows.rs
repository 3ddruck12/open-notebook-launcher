use super::{DistroInfo, InstallError};
use crate::docker::check_docker_status;
use std::process::Command;

const DOCKER_DESKTOP_URL: &str = "https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe";

pub fn detect_distro() -> DistroInfo {
    DistroInfo {
        id: "windows".to_string(),
        name: "Windows".to_string(),
        version_id: None,
        family: "windows".to_string(),
    }
}

pub fn get_manual_install_instructions(_distro: &DistroInfo) -> String {
    format!(
        "Manuelle Installation (Windows):\n\
        1. Installiere Docker Desktop von {DOCKER_DESKTOP_URL}\n\
        2. Starte Docker Desktop und warte, bis es bereit ist\n\
        3. Aktiviere WSL 2, falls der Installer danach fragt\n\
        4. Starte Open Notebook Desktop erneut"
    )
}

pub fn install_docker_engine(_distro: &DistroInfo) -> Result<String, InstallError> {
    Err(InstallError::Message(
        "Unter Windows wird Docker Desktop empfohlen. Bitte „Docker Desktop installieren“ verwenden.".to_string(),
    ))
}

pub fn install_docker_desktop(_distro: &DistroInfo) -> Result<String, InstallError> {
    let winget = Command::new("winget")
        .args([
            "install",
            "-e",
            "--id",
            "Docker.DockerDesktop",
            "--accept-package-agreements",
            "--accept-source-agreements",
        ])
        .output();

    match winget {
        Ok(output) if output.status.success() => Ok(
            "Docker Desktop wird über winget installiert. Bitte den Installer abschließen und Docker Desktop starten.".to_string(),
        ),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            Err(InstallError::Message(format!(
                "Automatische Installation fehlgeschlagen.\nLade Docker Desktop manuell herunter:\n{DOCKER_DESKTOP_URL}\n\n{stdout}{stderr}"
            )))
        }
        Err(_) => Err(InstallError::Message(format!(
            "winget nicht verfügbar. Bitte Docker Desktop manuell installieren:\n{DOCKER_DESKTOP_URL}"
        ))),
    }
}

pub fn verify_installation() -> String {
    let status = check_docker_status();
    if status.available && status.daemon_running && status.compose_available {
        "Docker Desktop ist einsatzbereit.".to_string()
    } else {
        status.message
    }
}

pub fn start_docker_service() -> Result<String, InstallError> {
    let candidates = [
        r"C:\Program Files\Docker\Docker\Docker Desktop.exe",
        r"C:\Program Files (x86)\Docker\Docker\Docker Desktop.exe",
    ];

    for candidate in candidates {
        if std::path::Path::new(candidate).exists() {
            Command::new(candidate)
                .spawn()
                .map_err(|e| InstallError::Message(e.to_string()))?;
            return Ok("Docker Desktop wird gestartet. Bitte warte, bis es bereit ist.".to_string());
        }
    }

    let _ = Command::new("cmd")
        .args(["/C", "start", "", "Docker Desktop"])
        .spawn();

    Ok("Versuche Docker Desktop zu starten. Falls nichts passiert, starte es manuell aus dem Startmenü.".to_string())
}
