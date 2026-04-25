use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct SyncArgs {}
pub fn run(_args: SyncArgs) -> Result<()> {
    println!("profilectl sync: not yet implemented");
    Ok(())
}
