use structopt::StructOpt;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error as FormatError};
use wasmfmt::{fmt, diff};

#[derive(StructOpt)]
pub struct Options {
    #[structopt(short, long)]
    mode: Option<Mode>,

    #[structopt(short, long)]
    output: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

pub enum Mode {
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

#[derive(Debug)]
enum Error {
    Io(io::Error),
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();

    let source = match options.file {
        // If there is a file specified, read its' content
        Some(file) => {
            fs::read_to_string(file.as_path())
                .map_err(Error::Io)?
        },
        // If there is no file, read from stdin
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                .map_err(Error::Io)?;
            buffer
        }
    };

    let formatted = fmt(&source);

    match options.mode.unwrap_or_default() {
        // When fixing, write the results to a destination
        Mode::Fix => {
            match options.output {
                // If there is an output file specified, write the result to it
                Some(path) => {
                    fs::write(path, formatted)
                        .map_err(Error::Io)?;
                },
                // If there is no file, write to stdout
                None => {
                    io::stdout().write_all(formatted.as_bytes())
                        .map_err(Error::Io)?;
                },
            };
        },
        // When checking, highlight any mismatch and exit
        // with an error code if there are any mismatches
        Mode::Check => {
            if source != formatted {
                let diff = diff(&source, &formatted);
                io::stdout().write_all(diff.as_bytes())
                        .map_err(Error::Io)?;
            }
        }
    };

    Ok(())
}
