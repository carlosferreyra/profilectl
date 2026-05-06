use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Repository to initialize from. Accepts a clone URL, a local path, or
    /// (when omitted) `git init` in `~/.dotfiles` with bundled starters.
    pub repo: Option<String>,

    /// Comma-separated bundles to enable for non-interactive init
    /// (e.g. `--bundles zsh,git,rustup`).
    #[arg(long, value_delimiter = ',')]
    pub bundles: Vec<String>,

    /// Overwrite an existing dotfiles repo at the target location.
    #[arg(long)]
    pub force: bool,

    /// Skip all prompts; fail if a required answer is missing.
    #[arg(long)]
    pub non_interactive: bool,
}

pub fn run(_args: InitArgs) -> Result<()> {
    println!("profilectl init: not yet implemented");
    Ok(())
}
