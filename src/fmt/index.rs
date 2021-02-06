use super::{Fmt, Formatter};
use wast::{Id, Index};

impl<'src> Fmt for &Vec<Index<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        let mut iter = self.iter();
        if let Some(index) = iter.next() {
            formatter.fmt(index);
        }
        for index in iter {
            formatter.write(" ");
            formatter.fmt(index);
        }
    }
}

impl<'src> Fmt for &Index<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            Index::Num(n, ..) => formatter.fmt(*n),
            Index::Id(id) => formatter.fmt(id),
        };
    }
}

impl<'src> Fmt for &Id<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("$");
        formatter.write(self.name());
    }
}
