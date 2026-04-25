pub mod bootstrap;
pub mod check;
pub mod diff;
pub mod init;
pub mod link;
pub mod profile;
pub mod scan;
pub mod status;
pub mod sync;
pub mod unlink;

use crate::args::Command;
use anyhow::Result;

pub fn dispatch(cmd: Command) -> Result<()> {
    match cmd {
        Command::Init(args) => init::run(args),
        Command::Sync(args) => sync::run(args),
        Command::Link(args) => link::run(args),
        Command::Unlink(args) => unlink::run(args),
        Command::Bootstrap(args) => bootstrap::run(args),
        Command::Scan(args) => scan::run(args),
        Command::Diff(args) => diff::run(args),
        Command::Check(args) => check::run(args),
        Command::Status(args) => status::run(args),
        Command::Profile(args) => profile::run(args),
    }
}
