use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ScanArgs {
    /// Output path. Format is inferred from the extension (`.md`, `.toml`).
    /// Defaults to `./tools.md`.
    #[arg(long, short)]
    pub output: Option<PathBuf>,
}

pub fn run(_args: ScanArgs) -> Result<()> {
    println!("profilectl scan: not yet implemented");
    Ok(())
}
