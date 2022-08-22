use super::utils::{id_is_gensym, index_is_default};
use super::{Fmt, Formatter};
use wast::core::{Data, DataKind, DataVal, Memory, MemoryKind, MemoryType};

impl<'src> Fmt for &Memory<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(memory ");
        if let Some(id) = &self.id {
            if !id_is_gensym(id) {
                formatter.fmt(id);
                formatter.write(" ");
            }
        };
        if !self.exports.names.is_empty() {
            formatter.fmt(&self.exports);
            formatter.write(" ");
        };
        formatter.fmt(&self.kind);
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &MemoryKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            MemoryKind::Import { import, ty } => {
                formatter.fmt(import);
                formatter.write(" ");
                formatter.fmt(ty);
            }
            MemoryKind::Normal(ty) => {
                formatter.fmt(ty);
            }
            MemoryKind::Inline { is_32: true, data } => {
                if !data.is_empty() {
                    formatter.write("(data ");
                    formatter.fmt(data);
                    formatter.write(")");
                }
            }
            MemoryKind::Inline {
                is_32: false,
                data: _,
            } => {
                unimplemented!()
            }
        }
    }
}

impl<'src> Fmt for &Vec<DataVal<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        let mut iter = self.iter();
        if let Some(val) = iter.next() {
            formatter.fmt(val);
        }
        for val in iter {
            formatter.write(" ");
            formatter.fmt(val);
        }
    }
}

impl<'src> Fmt for &DataVal<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            DataVal::String(bytes) => {
                let string = std::str::from_utf8(bytes).expect("valid utf8");
                formatter.fmt(string);
            }
            DataVal::Integral(..) => {
                // https://github.com/WebAssembly/wat-numeric-values
                unimplemented!();
            }
        }
    }
}

impl Fmt for &MemoryType {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            MemoryType::B32 { limits, shared: _ } => {
                formatter.write(&limits.min.to_string());
                if let Some(max) = limits.max {
                    formatter.write(" ");
                    formatter.write(&max.to_string());
                }
            }
            MemoryType::B64 { limits, shared: _ } => {
                formatter.write(&limits.min.to_string());
                if let Some(max) = limits.max {
                    formatter.write(" ");
                    formatter.write(&max.to_string());
                }
            }
        }
    }
}

impl<'src> Fmt for &Data<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(data ");
        if let Some(id) = &self.id {
            if !id_is_gensym(id) {
                formatter.fmt(id);
                formatter.write(" ");
            }
        }
        formatter.fmt(&self.kind);
        if !self.data.is_empty() {
            formatter.write(" ");
            formatter.fmt(&self.data);
        }
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &DataKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            DataKind::Passive => todo!(),
            DataKind::Active { memory, offset } => {
                if !index_is_default(memory) {
                    formatter.fmt(memory);
                    formatter.write(" ");
                }
                formatter.fmt(offset);
            }
        }
    }
}
