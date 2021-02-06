use super::{Fmt, Formatter};
use wast::Index;

pub struct Start<'src> {
    index: Index<'src>,
}

impl<'src> Start<'src> {
    pub fn new(index: Index<'src>) -> Self {
        Start { index }
    }
}

impl<'src> Fmt for &Start<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(start ");
        formatter.fmt(&self.index);
        formatter.write(")");
        formatter.end_line();
    }
}
