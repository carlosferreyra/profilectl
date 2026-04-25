use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command(group = clap::ArgGroup::new("scope").args(["tools_only", "links_only"]))]
pub struct SyncArgs {
    /// Install tools only; skip the link phase.
    #[arg(long)]
    pub tools_only: bool,

    /// Create symlinks only; skip the install phase.
    #[arg(long)]
    pub links_only: bool,

    /// Pass --force to the link phase. Ignored under --tools-only.
    #[arg(long)]
    pub force: bool,
}

pub fn run(_args: SyncArgs) -> Result<()> {
    println!("profilectl sync: not yet implemented");
    Ok(())
}
