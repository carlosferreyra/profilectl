use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct StatusArgs {}
pub fn run(_args: StatusArgs) -> Result<()> {
    todo!("dfiles status: not yet implemented")
}
