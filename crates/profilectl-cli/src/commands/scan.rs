use anyhow::Result;
use clap::Args;
#[derive(Args, Debug)]
pub struct ScanArgs {
    /// Output path (file or directory). Defaults to ./tools.md
    #[arg(long, short)]
    pub path: Option<std::path::PathBuf>,
}
pub fn run(_args: ScanArgs) -> Result<()> {
    todo!("profilectl scan: not yet implemented")
}
