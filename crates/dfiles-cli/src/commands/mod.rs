pub mod check;
pub mod diff;
pub mod install;
pub mod link;
pub mod profiles;
pub mod scan;
pub mod status;
pub mod sync;
pub mod unlink;

use crate::args::Command;
use anyhow::Result;

pub fn dispatch(cmd: Command) -> Result<()> {
    match cmd {
        Command::Sync(args)     => sync::run(args),
        Command::Install(args)  => install::run(args),
        Command::Link(args)     => link::run(args),
        Command::Unlink(args)   => unlink::run(args),
        Command::Scan(args)     => scan::run(args),
        Command::Diff(args)     => diff::run(args),
        Command::Check(args)    => check::run(args),
        Command::Profiles(args) => profiles::run(args),
        Command::Status(args)   => status::run(args),
    }
}
