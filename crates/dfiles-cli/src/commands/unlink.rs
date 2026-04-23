use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct UnlinkArgs {
    /// Remove targets even if they are regular files.
    #[arg(long)]
    pub force: bool,
}
pub fn run(_args: UnlinkArgs) -> Result<()> {
    todo!("dfiles unlink: not yet implemented")
}
