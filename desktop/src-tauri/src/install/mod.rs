use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistroInfo {
    pub id: String,
    pub name: String,
    pub version_id: Option<String>,
    pub family: String,
}

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("{0}")]
    Message(String),
}

#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use linux::*;
#[cfg(windows)]
pub use windows::*;

pub fn current_platform() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else {
        "linux"
    }
}
