use anyhow::Result;
use clap::Parser;
use dfiles_cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Init tracing — RUST_LOG controls verbosity, --verbose forces debug.
    let level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level)),
        )
        .without_time()
        .with_target(false)
        .init();

    // Override repo root if --home was passed.
    if let Some(ref home) = cli.home {
        std::env::set_var("DFILES_HOME", home);
    }

    match cli.command {
        // Subcommand given — run it directly, no interactive mode.
        Some(_) => dfiles_cli::run(cli),

        // No subcommand — drop into interactive TUI.
        None => dfiles_interactive::run_interactive(&cli.profile),
    }
}
