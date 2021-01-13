use std::fmt::{Display, Error as FormatError, Formatter};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use wasmfmt::{fmt, Diff, Error};

/// Format WebAssembly Text Format code according to a set of style rules.
#[derive(StructOpt)]
pub struct Options {
    /// Specify the operation mode.
    ///
    /// In `fix` mode, the formatted code
    /// is written to the output destination.
    /// In `check` mode, the formatted code
    /// is compared to the source code. If
    /// any difference is found, an error is
    /// written to `stdout`, and the process
    /// exits with code 1. If no difference is
    /// found, the process exits with code 0.
    #[structopt(short, long)]
    mode: Option<Mode>,

    /// Specify the output file path.
    ///
    /// The formatted code will be written to this file.
    /// If no path is provided, the formatted code
    /// is written directly to `stdout`.
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// Specify the input file path.
    ///
    /// The source code will be read from this file.
    /// If no path is provided, source code
    /// will be read directly from `stdin`.
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

enum Mode {
    Fix,
    Check,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FormatError> {
        match self {
            Mode::Fix => f.write_str("fix"),
            Mode::Check => f.write_str("check"),
        }
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fix" => Ok(Mode::Fix),
            "check" => Ok(Mode::Check),
            _ => Err(format!("unknown --mode value: {}", s)),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Fix
    }
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();

    let source = match options.file {
        Some(file) => fs::read_to_string(file.as_path())?,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    let formatted = fmt(&source);

    match options.mode.unwrap_or_default() {
        Mode::Fix => {
            match options.output {
                Some(path) => {
                    fs::write(path, formatted)?;
                }
                None => {
                    io::stdout().write_all(formatted.as_bytes())?;
                }
            };
            Ok(())
        }
        Mode::Check => {
            if let Some(diff) = Diff::from(&source, &formatted) {
                io::stdout().write_fmt(format_args!("{}", diff))?;
                io::stdout().flush().unwrap();
                std::process::exit(1);
            };
            Ok(())
        }
    }
}
