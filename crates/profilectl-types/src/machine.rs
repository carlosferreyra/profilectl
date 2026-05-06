use std::env::consts::ARCH;

use serde::{Deserialize, Serialize};

use crate::platform::Platform;

/// Package managers that profilectl knows how to drive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Mise,
    Brew,
    Apt,
    Dnf,
    Pacman,
    Winget,
    Choco,
    Scoop,
}

impl PackageManager {
    /// The binary name used for PATH probing and version queries.
    fn binary(&self) -> &'static str {
        match self {
            Self::Mise => "mise",
            Self::Brew => "brew",
            Self::Apt => "apt",
            Self::Dnf => "dnf",
            Self::Pacman => "pacman",
            Self::Winget => "winget",
            Self::Choco => "choco",
            Self::Scoop => "scoop",
        }
    }

    /// Returns true if the binary is on PATH and responds to a version query.
    pub fn is_available(&self) -> bool {
        let Ok(path) = which::which(self.binary()) else {
            return false;
        };
        std::process::Command::new(path)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn as_str(&self) -> &'static str {
        self.binary()
    }
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.binary())
    }
}

/// Runtime information about the current machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineInfo {
    pub platform: Platform,
    /// Value of `std::env::consts::ARCH` at runtime (e.g. `"x86_64"`, `"aarch64"`).
    pub arch: String,
    /// Package managers found on PATH that also respond to `--version`.
    pub package_managers: Vec<PackageManager>,
    /// Human-readable OS name from `/etc/os-release` (Linux) or the platform name elsewhere.
    pub os_name: String,
}

impl MachineInfo {
    /// Detect the current machine's platform, arch, OS name, and available package managers.
    ///
    /// On Linux, reads `/etc/os-release` for a human-readable distro name.
    /// Package manager availability is confirmed by both PATH presence and `--version` success.
    pub fn detect() -> Self {
        let platform = Platform::current();
        let arch = ARCH.to_string();
        let os_name = detect_os_name(&platform);
        let package_managers = detect_package_managers(&platform);

        Self {
            platform,
            arch,
            package_managers,
            os_name,
        }
    }
}

/// Candidate managers per platform — probed in this order.
fn candidates_for(platform: &Platform) -> &'static [PackageManager] {
    match platform {
        Platform::MacOs => &[PackageManager::Brew, PackageManager::Mise],
        Platform::Linux => &[
            PackageManager::Apt,
            PackageManager::Dnf,
            PackageManager::Pacman,
            PackageManager::Mise,
        ],
        Platform::Windows => &[
            PackageManager::Winget,
            PackageManager::Choco,
            PackageManager::Scoop,
            PackageManager::Mise,
        ],
    }
}

fn detect_package_managers(platform: &Platform) -> Vec<PackageManager> {
    candidates_for(platform)
        .iter()
        .filter(|pm| pm.is_available())
        .cloned()
        .collect()
}

/// Returns a human-readable OS/distro name.
///
/// - Linux: parses `PRETTY_NAME` from `/etc/os-release`; falls back to `"Linux"`.
/// - macOS: `"macOS"`
/// - Windows: `"Windows"`
fn detect_os_name(platform: &Platform) -> String {
    match platform {
        Platform::Linux => parse_os_release().unwrap_or_else(|| "Linux".to_string()),
        Platform::MacOs => "macOS".to_string(),
        Platform::Windows => "Windows".to_string(),
    }
}

/// Parses `PRETTY_NAME` from `/etc/os-release`.
fn parse_os_release() -> Option<String> {
    let content = std::fs::read_to_string("/etc/os-release").ok()?;
    for line in content.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            return Some(value.trim_matches('"').to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_consistent_platform() {
        let info = MachineInfo::detect();
        assert_eq!(info.platform, Platform::current());
    }

    #[test]
    fn detect_arch_is_nonempty() {
        let info = MachineInfo::detect();
        assert!(!info.arch.is_empty());
    }

    #[test]
    fn detect_os_name_is_nonempty() {
        let info = MachineInfo::detect();
        assert!(!info.os_name.is_empty());
    }

    #[test]
    fn package_manager_display() {
        assert_eq!(PackageManager::Brew.to_string(), "brew");
        assert_eq!(PackageManager::Apt.to_string(), "apt");
        assert_eq!(PackageManager::Mise.to_string(), "mise");
    }

    #[test]
    fn parse_os_release_extracts_pretty_name() {
        // Simulate a minimal /etc/os-release payload.
        let input = "ID=ubuntu\nPRETTY_NAME=\"Ubuntu 24.04 LTS\"\nVERSION_ID=\"24.04\"\n";
        let result: Option<String> = {
            let mut found = None;
            for line in input.lines() {
                if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                    found = Some(value.trim_matches('"').to_string());
                    break;
                }
            }
            found
        };
        assert_eq!(result.as_deref(), Some("Ubuntu 24.04 LTS"));
    }
}
