use crate::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn fix(formatted: &str, output: Option<PathBuf>) -> Result<i32, Error> {
    match output {
        Some(path) => {
            fs::write(path, formatted)?;
        }
        None => {
            io::stdout().write_all(formatted.as_bytes())?;
        }
    };
    Ok(0)
}
