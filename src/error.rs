use std::io;

#[derive(Debug)]
pub enum Error {
    ExtensionUnsupported(String),
    ExtensionMissing,
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
