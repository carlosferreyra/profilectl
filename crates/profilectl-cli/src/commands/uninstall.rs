use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct UninstallArgs {
    /// Also uninstall packages that the active profile installed.
    /// Without this flag, only the shell-sourcing block and managed symlinks
    /// are removed.
    #[arg(long)]
    pub purge: bool,
}

pub fn run(_args: UninstallArgs) -> Result<()> {
    println!("profilectl uninstall: not yet implemented");
    Ok(())
}
