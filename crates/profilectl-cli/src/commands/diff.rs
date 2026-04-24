use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct DiffArgs {}
pub fn run(_args: DiffArgs) -> Result<()> {
    println!("profilectl diff: not yet implemented");
    Ok(())
}
