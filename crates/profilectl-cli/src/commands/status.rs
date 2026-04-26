use crate::commands::scope::Scope;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct StatusArgs {
    /// Subset of profile state to report on.
    #[arg(long, value_enum, default_value_t = Scope::All)]
    pub scope: Scope,
}

pub fn run(_args: StatusArgs) -> Result<()> {
    println!("profilectl status: not yet implemented");
    Ok(())
}
