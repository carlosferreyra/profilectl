use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct InstallArgs {}
pub fn run(_args: InstallArgs) -> Result<()> {
    println!("profilectl install: not yet implemented");
    Ok(())
}
