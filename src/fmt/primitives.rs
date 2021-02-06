use super::{Fmt, Formatter};
use wast::{Float32, Float64};

impl Fmt for u32 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl Fmt for i32 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl Fmt for i64 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl Fmt for f32 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl Fmt for f64 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl Fmt for &Float32 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.fmt(f32::from_bits(self.bits));
    }
}

impl Fmt for &Float64 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.fmt(f64::from_bits(self.bits));
    }
}

impl<'src> Fmt for &'src str {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("\"");
        formatter.write(self);
        formatter.write("\"");
    }
}

impl<'src> Fmt for &Vec<&'src str> {
    fn fmt(&self, formatter: &mut Formatter) {
        let mut iter = self.iter();
        if let Some(name) = iter.next() {
            formatter.fmt(*name);
        }
        for name in iter {
            formatter.write(" ");
            formatter.fmt(*name);
        }
    }
}
