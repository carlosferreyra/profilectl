//! Thin process entrypoint for `profilectl`.

use std::process::ExitCode;

fn main() -> ExitCode {
    profilectl::main(std::env::args_os())
}
