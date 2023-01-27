mod exit_codes;
mod cli;

use anyhow::{anyhow, bail, Context, Result};
use crate::exit_codes::ExitCode;
use crate::cli::Opts;
use clap::{CommandFactory, Parser};


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
    let opts = Opts::parse();

    ExitCode::Success.exit();
}