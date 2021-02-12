#[derive(Clone)]
pub struct Options {
    pub resolve_names: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            resolve_names: false,
        }
    }
}
