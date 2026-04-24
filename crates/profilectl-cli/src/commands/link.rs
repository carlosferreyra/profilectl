use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct LinkArgs {
    /// Overwrite existing files/symlinks.
    #[arg(long)]
    pub force: bool,
}
pub fn run(_args: LinkArgs) -> Result<()> {
    todo!("profilectl link: not yet implemented")
}
