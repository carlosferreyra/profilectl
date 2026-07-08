use crate::commands::scope::Scope;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct CheckArgs {
    /// Subset of profile state to verify.
    #[arg(long, value_enum, default_value_t = Scope::All)]
    pub scope: Scope,
}

pub fn run(_args: CheckArgs) -> Result<()> {
    println!("profilectl check: not yet implemented");
    Ok(())
}
