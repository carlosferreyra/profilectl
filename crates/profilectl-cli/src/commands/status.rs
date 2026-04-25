use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct StatusArgs {}
pub fn run(_args: StatusArgs) -> Result<()> {
    println!("profilectl status: not yet implemented");
    Ok(())
}
