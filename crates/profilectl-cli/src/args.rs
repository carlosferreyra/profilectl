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
    /// First-time setup — clone or initialize the dotfiles repo.
    Init(commands::init::InitArgs),

    /// Apply the active profile: ensure shell sourcing, materialize files,
    /// create symlinks, and install tools. Idempotent.
    Apply(commands::apply::ApplyArgs),

    /// Configure a remote and push the local dotfiles repo.
    Publish(commands::publish::PublishArgs),

    /// Show drift between the active profile and the current machine state.
    Status(commands::status::StatusArgs),

    /// Like `status`, but exits 1 on drift (CI gate).
    Check(commands::check::CheckArgs),

    /// Remove profilectl-managed shell sourcing and symlinks.
    Uninstall(commands::uninstall::UninstallArgs),

    /// Capture the current machine's installed tools to a manifest file.
    Scan(commands::scan::ScanArgs),

    /// Manage profiles (list, show, switch).
    Profile(commands::profile::ProfileArgs),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::profile::ProfileCommand;
    use crate::commands::scope::Scope;
    use clap::Parser;

    #[test]
    fn no_subcommand_drops_into_interactive_mode() {
        let cli = Cli::try_parse_from(["profilectl"]).expect("bare invocation parses");
        assert!(cli.command.is_none());
        assert_eq!(cli.profile, "default");
    }

    #[test]
    fn apply_accepts_scope_pull_force_strict() {
        let cli = Cli::try_parse_from([
            "profilectl",
            "apply",
            "--scope",
            "tools",
            "--pull",
            "--force",
            "--strict",
        ])
        .expect("apply parses");
        let Some(Command::Apply(args)) = cli.command else {
            panic!("expected Apply, got {:?}", cli.command);
        };
        assert_eq!(args.scope, Scope::Tools);
        assert!(args.pull);
        assert!(args.force);
        assert!(args.strict);
    }

    #[test]
    fn init_accepts_repo_positional_and_comma_bundles() {
        let cli = Cli::try_parse_from([
            "profilectl",
            "init",
            "git@github.com:me/dotfiles.git",
            "--bundles",
            "zsh,git,rustup",
        ])
        .expect("init parses");
        let Some(Command::Init(args)) = cli.command else {
            panic!("expected Init, got {:?}", cli.command);
        };
        assert_eq!(args.repo.as_deref(), Some("git@github.com:me/dotfiles.git"));
        assert_eq!(args.bundles, vec!["zsh", "git", "rustup"]);
    }

    #[test]
    fn profile_use_takes_a_name() {
        let cli =
            Cli::try_parse_from(["profilectl", "profile", "use", "work"]).expect("profile parses");
        let Some(Command::Profile(args)) = cli.command else {
            panic!("expected Profile, got {:?}", cli.command);
        };
        let ProfileCommand::Use { name } = args.command else {
            panic!("expected Use");
        };
        assert_eq!(name, "work");
    }

    #[test]
    fn uninstall_purge_flag_parses() {
        let cli =
            Cli::try_parse_from(["profilectl", "uninstall", "--purge"]).expect("uninstall parses");
        let Some(Command::Uninstall(args)) = cli.command else {
            panic!("expected Uninstall");
        };
        assert!(args.purge);
    }

    #[test]
    fn status_default_scope_is_all() {
        let cli = Cli::try_parse_from(["profilectl", "status"]).expect("status parses");
        let Some(Command::Status(args)) = cli.command else {
            panic!("expected Status");
        };
        assert_eq!(args.scope, Scope::All);
    }

    #[test]
    fn removed_subcommands_are_rejected() {
        for removed in ["sync", "link", "unlink", "install", "diff", "bootstrap"] {
            assert!(
                Cli::try_parse_from(["profilectl", removed]).is_err(),
                "removed subcommand `{removed}` must not parse",
            );
        }
    }
}
