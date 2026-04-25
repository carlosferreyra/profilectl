use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct CheckArgs {}
pub fn run(_args: CheckArgs) -> Result<()> {
    println!("profilectl check: not yet implemented");
    Ok(())
}
