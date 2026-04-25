use crate::commands;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "profilectl",
    version,
    about = "A modern, profile-based dotfiles manager",
    long_about = "profilectl manages your dotfiles, tools, and machine setup from a single git repo.\n\
                  Run without arguments to enter interactive mode."
)]
pub struct Cli {
    /// Profile to use (defaults to \"default\").
    #[arg(
        long,
        short,
        global = true,
        default_value = "default",
        env = "PCTL_PROFILE"
    )]
    pub profile: String,

    /// Path to the dotfiles repo (overrides $PCTL_HOME and ~/.dotfiles).
    #[arg(long, global = true, env = "PCTL_HOME")]
    pub home: Option<std::path::PathBuf>,

    /// Enable verbose output.
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Show what would happen without making changes.
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run the first-time setup wizard and write the global config.
    Init(commands::init::InitArgs),

    /// Apply symlinks and install tools for the active profile.
    Sync(commands::sync::SyncArgs),

    /// Create or refresh dotfile symlinks for the active profile.
    Link(commands::link::LinkArgs),

    /// Remove every symlink the active profile manages.
    Unlink(commands::unlink::UnlinkArgs),

    /// Wire up shell sourcing and materialize the rendered config tree.
    Bootstrap(commands::bootstrap::BootstrapArgs),

    /// Scan installed tools and write a tools.md (or .toml) snapshot.
    Scan(commands::scan::ScanArgs),

    /// Show drift between the active profile and the current machine.
    Diff(commands::diff::DiffArgs),

    /// Verify the active profile is fully applied (exits nonzero on drift).
    Check(commands::check::CheckArgs),

    /// Show current profile and machine state.
    Status(commands::status::StatusArgs),

    /// Manage profiles (list, show, switch).
    #[command(subcommand_value_name = "ACTION")]
    Profile(commands::profile::ProfileArgs),
}
