use std::fs;
use std::io::{self, Write as _};
use std::path::PathBuf;
use structopt::StructOpt;
use wasmfmt::{fmt, Diff, Error, Options};

/// Format WebAssembly code.
#[derive(StructOpt)]
enum Cli {
    /// Format the input file in-place.
    Fix(Input),
    /// Check if the input file is formatted correctly.
    Check(Input),
    /// Print the formatted code to `stdout`.
    Print(Input),
}

#[derive(StructOpt)]
struct Input {
    /// The input file path.
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    #[structopt(flatten)]
    flags: Flags,
}

#[derive(StructOpt, Debug)]
struct Flags {
    /// Perform name resolution.
    #[structopt(short, long)]
    resolve_names: bool,
}

enum Command {
    Fix,
    Check,
    Print,
}

impl From<Cli> for (Command, Input) {
    fn from(cli: Cli) -> (Command, Input) {
        match cli {
            Cli::Print(input) => (Command::Print, input),
            Cli::Fix(input) => (Command::Fix, input),
            Cli::Check(input) => (Command::Check, input),
        }
    }
}

fn main() -> Result<(), Error> {
    let (command, input) = Cli::from_args().into();

    let source = fs::read_to_string(input.file.as_path())?;

    let formatted = fmt(
        &source,
        Options {
            resolve_names: input.flags.resolve_names,
        },
    );

    match command {
        Command::Fix => {
            fs::write(input.file, formatted)?;
            Ok(())
        }
        Command::Check => {
            if let Some(diff) = Diff::from(&source, &formatted) {
                io::stdout().write_fmt(format_args!("{}", diff))?;
                io::stdout().flush().unwrap();
                std::process::exit(1);
            };
            Ok(())
        }
        Command::Print => {
            io::stdout().write_all(formatted.as_bytes())?;
            Ok(())
        }
    }
}
