use anyhow::Result;

use crate::cargo;

pub fn test() -> Result<()> {
    cargo(&[
        "nextest",
        "run",
        "--workspace",
        "--all-features",
        "--locked",
    ])
}
