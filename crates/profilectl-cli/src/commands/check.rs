use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command(group = clap::ArgGroup::new("scope").args(["tools_only", "links_only"]))]
pub struct CheckArgs {
    /// Only check tools.
    #[arg(long)]
    pub tools_only: bool,

    /// Only check symlinks.
    #[arg(long)]
    pub links_only: bool,
}

pub fn run(_args: CheckArgs) -> Result<()> {
    println!("profilectl check: not yet implemented");
    Ok(())
}
