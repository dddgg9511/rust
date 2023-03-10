use std::process;

#[cfg(unix)]
use nix::sys::signal::{raise, signal, SigHandler, Signal};

#[derive(PartialEq)]
pub enum ExitCode {
    Success,
    HasResults(bool),
    GeneralError,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        match code {
            ExitCode::Success => 0,
            ExitCode::HasResults(has_results) => !has_results as i32,
            ExitCode::GeneralError => 1,
            ExitCode::KilledBySigint => 130,
        }
    }
}

impl ExitCode {
    fn is_error(self) -> bool {
        i32::from(self) != 0
    }

    /// Exit the process with the appropriate code.
    pub fn exit(self) -> ! {
        #[cfg(unix)]
        if self == ExitCode::KilledBySigint {
            // Get rid of the SIGINT handler, if present, and raise SIGINT
            unsafe {
                if signal(Signal::SIGINT, SigHandler::SigDfl).is_ok() {
                    let _ = raise(Signal::SIGINT);
                }
            }
        }

        process::exit(self.into())
    }
}
