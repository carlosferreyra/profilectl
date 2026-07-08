//! Clap command model for `profilectl`.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// `profilectl` command-line interface.
#[derive(Debug, Parser)]
#[command(name = "profilectl", version, about)]
pub struct Cli {
    /// Active profile name.
    #[arg(long, global = true, env = "PCTL_PROFILE", default_value = "default")]
    pub profile: String,

    /// Dotfiles repository root.
    #[arg(long, global = true, env = "PCTL_HOME")]
    pub home: Option<PathBuf>,

    /// Emit verbose diagnostic output.
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Command to run.
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Available commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize local profile storage.
    Init(InitArgs),

    /// Show the operations profilectl would perform.
    Plan(ScopeArgs),

    /// Apply the active profile to the current machine.
    Apply(ApplyArgs),

    /// Show drift between desired state and machine state.
    Status(ScopeArgs),

    /// CI-friendly drift check.
    Check(ScopeArgs),

    /// Inspect the current machine and write a migration report.
    Scan(ScanArgs),

    /// Manage profiles.
    Profile(ProfileArgs),

    /// Configure a remote for the dotfiles repository.
    Publish(PublishArgs),

    /// Remove profilectl-managed artifacts.
    Uninstall(UninstallArgs),
}

/// Arguments for `profilectl init`.
#[derive(Debug, clap::Args)]
pub struct InitArgs {
    /// Existing repository URL or local path.
    pub repo: Option<String>,

    /// Bundles to include in the starter profile.
    #[arg(long, value_delimiter = ',')]
    pub bundles: Vec<String>,

    /// Replace an existing destination when safe to do so.
    #[arg(long)]
    pub force: bool,
}

/// Arguments shared by read-only scoped commands.
#[derive(Debug, clap::Args)]
pub struct ScopeArgs {
    /// Desired-state area to inspect.
    #[arg(long, value_enum, default_value_t = Scope::All)]
    pub scope: Scope,
}

/// Arguments for `profilectl apply`.
#[derive(Debug, clap::Args)]
pub struct ApplyArgs {
    /// Desired-state area to apply.
    #[arg(long, value_enum, default_value_t = Scope::All)]
    pub scope: Scope,

    /// Pull the dotfiles repository before planning.
    #[arg(long)]
    pub pull: bool,

    /// Allow replacement of conflicting targets.
    #[arg(long)]
    pub force: bool,

    /// Stop on the first failed operation.
    #[arg(long)]
    pub strict: bool,
}

/// Desired-state area for profilectl commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum Scope {
    /// File links and rendered templates.
    Links,

    /// Tool and package-manager operations.
    Tools,

    /// Shell bootstrap and environment wiring.
    Shell,

    /// Explicit profile tasks.
    Tasks,

    /// Every supported area.
    #[default]
    All,
}

/// Arguments for `profilectl scan`.
#[derive(Debug, clap::Args)]
pub struct ScanArgs {
    /// Output path for the migration report.
    #[arg(long, short)]
    pub output: Option<PathBuf>,
}

/// Arguments for `profilectl profile`.
#[derive(Debug, clap::Args)]
pub struct ProfileArgs {
    /// Profile action to run.
    #[command(subcommand)]
    pub command: ProfileCommand,
}

/// Profile management commands.
#[derive(Debug, Subcommand)]
pub enum ProfileCommand {
    /// List available profiles.
    List,

    /// Show a resolved profile.
    Show {
        /// Profile name; defaults to the active profile.
        name: Option<String>,
    },

    /// Set the active profile.
    Use {
        /// Profile name to activate.
        name: String,
    },
}

/// Arguments for `profilectl publish`.
#[derive(Debug, clap::Args)]
pub struct PublishArgs {
    /// Remote URL to configure.
    pub url: Option<String>,
}

/// Arguments for `profilectl uninstall`.
#[derive(Debug, clap::Args)]
pub struct UninstallArgs {
    /// Also remove tools when profilectl can prove ownership.
    #[arg(long)]
    pub purge: bool,
}
