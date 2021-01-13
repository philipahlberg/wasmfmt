use crate::error::Error;
use std::io::{self, Write};

/// Check for any differences between source code and formatted code.
/// If any differences are found, the two strings are printed to `stdout`
/// and `1` is returned.
/// If no differences are found, `0` is returned.
pub fn check(source: &str, formatted: &str) -> Result<i32, Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if source != formatted {
        handle.write_all(b"Difference found.\n")?;
        handle.write_all(b"Source:\n")?;
        handle.write_all(source.as_bytes())?;
        handle.write_all(b"Formatted:\n")?;
        handle.write_all(formatted.as_bytes())?;
        Ok(1)
    } else {
        Ok(0)
    }
}
