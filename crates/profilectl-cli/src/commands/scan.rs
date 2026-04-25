use anyhow::Result;
use clap::{Args, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ScanFormat {
    Md,
    Toml,
}

#[derive(Args, Debug)]
pub struct ScanArgs {
    /// Output path. Defaults to ./tools.md (or ./tools.toml when --format toml).
    #[arg(long, short)]
    pub path: Option<std::path::PathBuf>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = ScanFormat::Md)]
    pub format: ScanFormat,
}

pub fn run(_args: ScanArgs) -> Result<()> {
    println!("profilectl scan: not yet implemented");
    Ok(())
}
