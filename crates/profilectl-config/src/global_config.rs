use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Global profilectl configuration at `~/.config/profilectl/config.toml`.
/// Absence of this file means first-time setup (run `profilectl init`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    /// Path to the dotfiles repository.
    pub home: Option<PathBuf>,

    /// Active profile name.
    pub profile: Option<String>,
}

impl GlobalConfig {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("profilectl").join("config.toml"))
    }

    /// Returns `true` when no global config file exists (first-time setup).
    pub fn is_first_run() -> bool {
        Self::config_path().map(|p| !p.exists()).unwrap_or(true)
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path().context("could not resolve config directory")?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        toml::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path().context("could not resolve config directory")?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        let contents = toml::to_string_pretty(self).context("failed to serialize config")?;
        std::fs::write(&path, contents)
            .with_context(|| format!("failed to write {}", path.display()))
    }
}
