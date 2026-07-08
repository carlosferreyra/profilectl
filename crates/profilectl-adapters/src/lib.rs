//! External system adapters for `profilectl`.
//!
//! Adapters isolate filesystem, shell, git, package manager, template, and
//! command-runner effects from the core planner.

use std::path::Path;

/// Filesystem effects needed by the executor.
pub trait FileSystem {
    /// Returns true when a path exists.
    fn exists(&self, path: &Path) -> bool;
}

/// Package manager effects needed by the executor.
pub trait PackageManager {
    /// Returns true when a package's executable is available.
    fn is_installed(&self, package: &str) -> bool;

    /// Install a package.
    fn install(&self, package: &str) -> Result<(), AdapterError>;
}

/// Error returned by an external adapter.
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    /// The external command failed.
    #[error("adapter command failed: {0}")]
    CommandFailed(String),
}
