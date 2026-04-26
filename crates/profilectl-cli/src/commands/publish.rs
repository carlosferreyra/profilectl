use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct PublishArgs {
    /// Remote URL to push the dotfiles repo to. If omitted, prompts (and
    /// pre-fills `https://github.com/<handle>/dotfiles` when `gh auth status`
    /// resolves a handle).
    pub url: Option<String>,
}

pub fn run(_args: PublishArgs) -> Result<()> {
    println!("profilectl publish: not yet implemented");
    Ok(())
}
