use profilectl_types::Platform;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level profile — lives at `profiles/<name>.toml` in the dotfiles repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,

    /// Optional profile to inherit from (must resolve to another profiles/*.toml).
    pub extends: Option<String>,

    /// Human-readable description of this profile's purpose.
    pub description: Option<String>,

    /// Dotfile symlinks: repo-relative source -> absolute target (~ expanded).
    #[serde(default)]
    pub links: Vec<Link>,

    /// Tools to install, grouped by package manager.
    #[serde(default)]
    pub tools: ToolSet,

    /// Arbitrary key/value env vars to export on this machine.
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Platform filter — if set, this profile only applies on the listed platforms.
    #[serde(default)]
    pub platforms: Vec<Platform>,
}

/// A single dotfile symlink entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// Path relative to the repo root (e.g. `config/zsh/.zshrc`).
    pub source: String,
    /// Absolute destination path, `~` is expanded at runtime (e.g. `~/.zshrc`).
    pub target: String,
    /// Skip silently if the source doesn't exist (useful for optional configs).
    #[serde(default)]
    pub optional: bool,
}

/// Tool lists grouped by package manager.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolSet {
    #[serde(default)]
    pub brew: Vec<String>,

    #[serde(default)]
    pub cargo: Vec<String>,

    #[serde(default)]
    pub uv: Vec<String>,

    #[serde(default)]
    pub npm: Vec<String>,

    #[serde(default)]
    pub bun: Vec<String>,

    /// Catch-all for future package managers without a schema change.
    #[serde(default)]
    pub other: HashMap<String, Vec<String>>,
}

/// Which package manager a tool belongs to — used by the install/scan commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Brew,
    Cargo,
    Uv,
    Npm,
    Bun,
    Other,
}
