use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command(group = clap::ArgGroup::new("scope").args(["tools_only", "links_only"]))]
pub struct DiffArgs {
    /// Only diff tools.
    #[arg(long)]
    pub tools_only: bool,

    /// Only diff symlinks.
    #[arg(long)]
    pub links_only: bool,
}

pub fn run(_args: DiffArgs) -> Result<()> {
    println!("profilectl diff: not yet implemented");
    Ok(())
}
