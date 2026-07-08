//! Public library and executable dispatch for `profilectl`.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::ffi::OsString;
use std::process::ExitCode;

use clap::Parser;
use profilectl_cli::{Cli, Command, ProfileCommand};

/// Returns a starter message.
#[must_use]
pub const fn hello() -> &'static str {
    "Hello from profilectl!"
}

/// Parse command-line arguments and dispatch the selected command.
pub fn main(args: impl IntoIterator<Item = OsString>) -> ExitCode {
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .try_init();
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(error) => {
            let exit_code = error.exit_code();
            let _ = error.print();
            return ExitCode::from(u8::try_from(exit_code).unwrap_or(2));
        }
    };
    match cli.command {
        None => println!("profilectl interactive mode is planned"),
        Some(Command::Init(_)) => println!("profilectl init is planned"),
        Some(Command::Plan(_)) => println!("profilectl plan is planned"),
        Some(Command::Apply(_)) => println!("profilectl apply is planned"),
        Some(Command::Status(_)) => println!("profilectl status is planned"),
        Some(Command::Check(_)) => println!("profilectl check is planned"),
        Some(Command::Scan(_)) => println!("profilectl scan is planned"),
        Some(Command::Profile(profile)) => match profile.command {
            ProfileCommand::List => println!("profilectl profile list is planned"),
            ProfileCommand::Show { name } => {
                let profile_name = name.unwrap_or(cli.profile);
                println!("profilectl profile show {profile_name} is planned");
            }
            ProfileCommand::Use { name } => {
                println!("profilectl profile use {name} is planned");
            }
        },
        Some(Command::Publish(_)) => println!("profilectl publish is planned"),
        Some(Command::Uninstall(_)) => println!("profilectl uninstall is planned"),
    }
    ExitCode::SUCCESS
}
