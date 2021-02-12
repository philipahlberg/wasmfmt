mod diff;
mod error;
mod fmt;
mod options;
pub mod wast;
pub mod wat;

pub use diff::Diff;
pub use error::Error;
pub use fmt::{Fmt, Formatter};
pub use options::Options;
