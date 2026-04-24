use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct ProfilesArgs {}
pub fn run(_args: ProfilesArgs) -> Result<()> {
    println!("profilectl profiles: not yet implemented");
    Ok(())
}
