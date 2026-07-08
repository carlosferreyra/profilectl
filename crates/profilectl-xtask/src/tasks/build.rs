use anyhow::Result;

use crate::cargo;

pub fn build() -> Result<()> {
    cargo(&[
        "build",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--locked",
    ])
}
