use anyhow::Result;
use clap::Args;
use profilectl_types::MachineInfo;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Repository to initialize from. Accepts a clone URL, a local path, or
    /// (when omitted) `git init` in `~/.dotfiles` with bundled starters.
    pub repo: Option<String>,

    /// Comma-separated bundles to enable for non-interactive init
    /// (e.g. `--bundles zsh,git,rustup`).
    #[arg(long, value_delimiter = ',')]
    pub bundles: Vec<String>,

    /// Overwrite an existing dotfiles repo at the target location.
    #[arg(long)]
    pub force: bool,

    /// Skip all prompts; fail if a required answer is missing.
    #[arg(long)]
    pub non_interactive: bool,
}

pub fn run(_args: InitArgs) -> Result<()> {
    let info = MachineInfo::detect();
    print_machine_info(&info);

    println!("\nprofilectl init: not yet implemented");
    Ok(())
}

/// Prints detected machine info. Always shown during `init` (first-run setup context).
pub fn print_machine_info(info: &MachineInfo) {
    println!("Detected system:");
    println!("  OS:   {} ({})", info.os_name, info.platform);
    println!("  Arch: {}", info.arch);

    if info.package_managers.is_empty() {
        println!("  Package managers: none detected");
    } else {
        let names: Vec<String> = info
            .package_managers
            .iter()
            .map(|pm| pm.to_string())
            .collect();
        println!("  Package managers: {}", names.join(", "));
    }
}
