use std::env::consts::ARCH;

use serde::{Deserialize, Serialize};

use crate::platform::Platform;

/// Package managers that profilectl knows how to drive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    Other,
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
            Self::Other => "other",
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
fn candidates_for(platform: &Platform) -> Vec<PackageManager> {
    match platform {
        Platform::MacOs => vec![PackageManager::Brew, PackageManager::Mise],
        Platform::Linux => vec![
            PackageManager::Apt,
            PackageManager::Dnf,
            PackageManager::Pacman,
            PackageManager::Mise,
        ],
        Platform::Windows => vec![
            PackageManager::Winget,
            PackageManager::Choco,
            PackageManager::Scoop,
            PackageManager::Mise,
        ],
    }
}

fn detect_package_managers(platform: &Platform) -> Vec<PackageManager> {
    candidates_for(platform)
        .into_iter()
        .filter(|pm| pm.is_available())
        .collect()
}

/// Returns a human-readable OS/distro name.
///
/// - Linux: parses `PRETTY_NAME` from `/etc/os-release`; falls back to `"Linux"`.
/// - macOS: `"macOS"`
/// - Windows: `"Windows"`
fn detect_os_name(platform: &Platform) -> String {
    match platform {
        Platform::Linux => {
            read_os_release_name("/etc/os-release").unwrap_or_else(|| "Linux".to_string())
        }
        Platform::MacOs => "macOS".to_string(),
        Platform::Windows => "Windows".to_string(),
    }
}

/// Parses `PRETTY_NAME` from an `/etc/os-release`-formatted file path.
fn read_os_release_name(path: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    parse_pretty_name(&content)
}

/// Extracts `PRETTY_NAME` value from `/etc/os-release` content.
fn parse_pretty_name(content: &str) -> Option<String> {
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
    fn parse_pretty_name_extracts_quoted_value() {
        let input = "ID=ubuntu\nPRETTY_NAME=\"Ubuntu 24.04 LTS\"\nVERSION_ID=\"24.04\"\n";
        assert_eq!(
            parse_pretty_name(input).as_deref(),
            Some("Ubuntu 24.04 LTS")
        );
    }

    #[test]
    fn parse_pretty_name_extracts_unquoted_value() {
        let input = "ID=arch\nPRETTY_NAME=Arch Linux\n";
        assert_eq!(parse_pretty_name(input).as_deref(), Some("Arch Linux"));
    }

    #[test]
    fn parse_pretty_name_returns_none_when_absent() {
        let input = "ID=unknown\nVERSION_ID=1\n";
        assert_eq!(parse_pretty_name(input), None);
    }
}
