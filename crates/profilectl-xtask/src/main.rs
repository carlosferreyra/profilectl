mod flags;
mod tasks;

use std::{env, process::Command};

use anyhow::{bail, Result};

fn main() {
    let flags = flags::Xtask::from_env_or_exit();
    let result = match flags.subcommand {
        flags::XtaskCmd::Check(_) => tasks::check(),
        flags::XtaskCmd::Build(_) => tasks::build(),
        flags::XtaskCmd::Test(_) => tasks::test(),
    };
    if let Err(err) = result {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

pub fn cargo(args: &[&str]) -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let status = Command::new(&cargo).args(args).status()?;
    if !status.success() {
        bail!("`{cargo} {}` failed with {status}", args.join(" "));
    }
    Ok(())
}
