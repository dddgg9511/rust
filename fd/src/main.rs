mod exit_codes;

use anyhow::{anyhow, bail, Context, Result};
use crate::exit_codes::ExitCode;


fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            exit_code.exit();
        }
        Err(err) => {
            eprintln!("[fd error]: {:#}", err);
            ExitCode::GeneralError.exit();
        }
    }
}

fn run() -> Result<ExitCode> {
    ExitCode::Success.exit();
}