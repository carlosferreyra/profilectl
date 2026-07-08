//! Desired-state planning engine for `profilectl`.
//!
//! This crate owns the domain model that every user-facing command should share:
//! resolve a profile, observe the machine, build a plan, then execute or report
//! that plan.

use std::path::PathBuf;

/// Fully resolved machine intent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DesiredState {
    /// Active profile name used to build this state.
    pub profile: String,
    /// Operations implied by the resolved profile.
    pub operations: Vec<Operation>,
}

/// Executable plan built from desired and observed state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Plan {
    /// Ordered operations to report or execute.
    pub operations: Vec<Operation>,
}

impl Plan {
    /// Returns true when no machine changes are needed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

/// A typed operation profilectl can report, check, or execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// Link a repository file into a machine target path.
    LinkFile {
        /// Repository-relative source path.
        source: PathBuf,
        /// Machine target path.
        target: PathBuf,
    },
    /// Render a template into profilectl's rendered output directory.
    RenderTemplate {
        /// Repository-relative template path.
        source: PathBuf,
        /// Rendered output path.
        target: PathBuf,
    },
    /// Ensure the shell startup file sources profilectl's rendered files.
    EnsureShellBootstrap {
        /// Shell startup file to update.
        target: PathBuf,
    },
    /// Install a missing tool through a package manager adapter.
    InstallTool {
        /// Package manager name.
        manager: String,
        /// Package name.
        package: String,
    },
    /// Run an explicit task from a profile.
    RunTask {
        /// Task name.
        name: String,
    },
    /// Remove a previously managed artifact.
    RemoveManagedItem {
        /// Path to remove.
        path: PathBuf,
    },
}
