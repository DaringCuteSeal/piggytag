use colored::Colorize;
use std::io::Write;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PiggytagError {
    #[error(transparent)]
    Io(#[from] ::std::io::Error),

    #[error("error on file {file_name}: {err}")]
    LoftyError {
        file_name: String,
        err: ::lofty::error::LoftyError,
    },

    #[error("{0}")]
    Msg(String),
}

impl From<&'static str> for PiggytagError {
    fn from(s: &'static str) -> Self {
        PiggytagError::Msg(s.to_owned())
    }
}

impl From<String> for PiggytagError {
    fn from(s: String) -> Self {
        PiggytagError::Msg(s)
    }
}

pub fn error_handler(error: PiggytagError, out: &mut dyn Write) -> ! {
    match error {
        PiggytagError::Io(err) => {
            if err.kind() == std::io::ErrorKind::BrokenPipe {
                std::process::exit(1)
            } else {
                writeln!(out, "{} {}", "[I/O error]".red(), err).ok();
                std::process::exit(1)
            }
        }
        PiggytagError::LoftyError { file_name, err } => {
            writeln!(
                out,
                "{} on file {}: {}",
                "[error]".red(),
                file_name.bold(),
                err
            )
            .ok();
            std::process::exit(1)
        }
        _ => {
            writeln!(out, "{} {}", "[error]".red(), error).ok();
            std::process::exit(1)
        }
    }
}
