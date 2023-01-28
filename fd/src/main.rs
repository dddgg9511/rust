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

fn set_working_dir(opts: &Opts) -> Result<()> {
    if let Some(ref base_directory) = opts.base_directory {
        if !filesystem::is_existing_directory(base_directory) {
            return Err(anyhow!(
                "The '--base-directory' path '{}' is not a directory.",
                base_directory.to_string_lossy()
            ));
        }
        env::set_current_dir(base_directory).with_context(|| {
            format!(
                "Could not set '{}' as the current working directory",
                base_directory.to_string_lossy()
            )
        })?;
    }
    Ok(())
}
