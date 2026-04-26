use profilectl_types::Platform;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level profile — lives at `profiles/<name>.toml` in the dotfiles repo.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,

    /// Optional profile to inherit from (must resolve to another profiles/*.toml).
    pub extends: Option<String>,

    /// Bundle fragments to merge before `extends`/own definition.
    /// Names refer to baked-in bundle TOML fragments (see `bundles/*.toml`).
    #[serde(default)]
    pub bundles: Vec<String>,

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
///
/// `mise` is the cross-platform default and covers polyglot language runtimes
/// plus most general-purpose tools. Per-OS lists are silently skipped on
/// non-matching platforms (filter at apply time).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolSet {
    /// Cross-platform default — polyglot version & tool manager.
    #[serde(default)]
    pub mise: Vec<String>,

    /// macOS (Homebrew, includes casks).
    #[serde(default)]
    pub brew: Vec<String>,

    /// Debian / Ubuntu.
    #[serde(default)]
    pub apt: Vec<String>,

    /// Fedora / RHEL.
    #[serde(default)]
    pub dnf: Vec<String>,

    /// Arch Linux.
    #[serde(default)]
    pub pacman: Vec<String>,

    /// Windows (official package manager).
    #[serde(default)]
    pub winget: Vec<String>,

    /// Windows (community package manager).
    #[serde(default)]
    pub choco: Vec<String>,

    /// Windows (CLI-focused package manager).
    #[serde(default)]
    pub scoop: Vec<String>,

    /// Catch-all for future package managers without a schema change.
    #[serde(default)]
    pub other: HashMap<String, Vec<String>>,
}

/// Which package manager a tool belongs to — used by the install/scan commands.
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
