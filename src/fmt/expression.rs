use super::{Fmt, Formatter};
use wast::core::Expression;

impl<'src> Fmt for &Expression<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        let mut iter = self.instrs.iter();
        if let Some(instruction) = iter.next() {
            formatter.fmt(instruction);
        }
        for instruction in iter {
            formatter.write(" ");
            formatter.fmt(instruction);
        }
    }
}
