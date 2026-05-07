use std::{
    env,
    process::{Command, ExitCode},
};

use anyhow::{bail, Result};

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("check") => check(),
        Some("build") => build(),
        Some("test") => test(),
        Some(unknown) => bail!("unknown task: {unknown}\n\nAvailable tasks: check, build, test"),
        None => bail!("no task specified\n\nAvailable tasks: check, build, test"),
    }
}

/// `cargo fmt --check` + `cargo check` + restrictive `cargo clippy`
fn check() -> Result<()> {
    cargo(&["fmt", "--all", "--", "--check"])?;
    cargo(&[
        "check",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--locked",
    ])?;
    cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--locked",
        "--",
        "-D",
        "warnings",
    ])
}

/// `cargo build` for the full workspace
fn build() -> Result<()> {
    cargo(&[
        "build",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--locked",
    ])
}

/// Full test suite via `cargo nextest`
fn test() -> Result<()> {
    cargo(&[
        "nextest",
        "run",
        "--workspace",
        "--all-features",
        "--locked",
    ])
}

fn cargo(args: &[&str]) -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let status = Command::new(&cargo).args(args).status()?;
    if !status.success() {
        bail!("`{cargo} {}` failed with {status}", args.join(" "));
    }
    Ok(())
}
