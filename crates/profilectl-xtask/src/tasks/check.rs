use anyhow::Result;

use crate::cargo;

pub fn check() -> Result<()> {
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
