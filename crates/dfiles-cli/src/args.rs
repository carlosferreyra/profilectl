use crate::commands;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "dfiles",
    version,
    about = "A modern, profile-based dotfiles manager",
    long_about = "dfiles manages your dotfiles, tools, and machine setup from a single git repo.\n\
                  Run without arguments to enter interactive mode."
)]
pub struct Cli {
    /// Profile to use (defaults to \"default\").
    #[arg(
        long,
        short,
        global = true,
        default_value = "default",
        env = "DFILES_PROFILE"
    )]
    pub profile: String,

    /// Path to the dotfiles repo (overrides $DFILES_HOME and ~/.dotfiles).
    #[arg(long, global = true, env = "DFILES_HOME")]
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
    /// Apply symlinks and install tools for a profile (link + install).
    Sync(commands::sync::SyncArgs),

    /// Install tools declared in the profile without touching symlinks.
    Install(commands::install::InstallArgs),

    /// Create or update dotfile symlinks for the active profile.
    Link(commands::link::LinkArgs),

    /// Remove all symlinks managed by the active profile.
    Unlink(commands::unlink::UnlinkArgs),

    /// Scan installed tools and write a tools.md snapshot.
    Scan(commands::scan::ScanArgs),

    /// Show diff between profile tools and currently installed tools.
    Diff(commands::diff::DiffArgs),

    /// Verify all profile symlinks are in place and tools are installed.
    Check(commands::check::CheckArgs),

    /// List available profiles.
    Profiles(commands::profiles::ProfilesArgs),

    /// Show current profile and machine state.
    Status(commands::status::StatusArgs),
}
