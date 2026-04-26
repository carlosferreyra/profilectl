pub mod apply;
pub mod check;
pub mod init;
pub mod profile;
pub mod publish;
pub mod scan;
pub mod scope;
pub mod status;
pub mod uninstall;

use crate::args::Command;
use anyhow::Result;

pub fn dispatch(cmd: Command) -> Result<()> {
    match cmd {
        Command::Init(args) => init::run(args),
        Command::Apply(args) => apply::run(args),
        Command::Publish(args) => publish::run(args),
        Command::Status(args) => status::run(args),
        Command::Check(args) => check::run(args),
        Command::Uninstall(args) => uninstall::run(args),
        Command::Scan(args) => scan::run(args),
        Command::Profile(args) => profile::run(args),
    }
}
