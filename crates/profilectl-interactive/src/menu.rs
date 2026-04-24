use anyhow::Result;

/// Top-level interactive menu actions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
enum Action {
    Sync,
    Install,
    Link,
    Unlink,
    Scan,
    Diff,
    Check,
    Status,
    Exit,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Sync => "  sync     — apply symlinks + install tools",
            Self::Install => "  install  — install profile tools only",
            Self::Link => "  link     — create dotfile symlinks",
            Self::Unlink => "  unlink   — remove dotfile symlinks",
            Self::Scan => "  scan     — snapshot installed tools to tools.md",
            Self::Diff => "  diff     — compare profile tools vs installed",
            Self::Check => "  check    — verify symlinks and tools",
            Self::Status => "  status   — show current profile and machine state",
            Self::Exit => "  exit",
        };
        write!(f, "{label}")
    }
}

/// Entry point for full interactive mode (no subcommand given).
pub fn run_interactive(profile: &str) -> Result<()> {
    todo!(
        "profilectl interactive mode (profile='{profile}'): \
         inquire menu loop not yet implemented"
    )
}
