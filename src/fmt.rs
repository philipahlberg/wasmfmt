use wast::{
    parser::{self, ParseBuffer},
    BlockType, BrTableIndices, Export, ExportKind, Expression, Float32, Float64, Func, FuncKind,
    FunctionType, Global, GlobalKind, GlobalType, Id, Index, InlineExport, InlineImport,
    Instruction, Local, MemArg, Module, ModuleField, ModuleKind, NameAnnotation, Type, TypeDef,
    TypeUse, ValType, Wat,
};

/// A formatter used to format individual AST nodes.
pub struct Formatter {
    buffer: String,
    indentation: usize,
}

impl Formatter {
    /// Construct a new formatter with an empty buffer and zero initial indentation.
    pub fn new() -> Self {
        Self::default()
    }

    fn indent(&mut self) {
        self.indentation += 1;
    }

    fn deindent(&mut self) {
        self.indentation -= 1;
    }

    fn start_line(&mut self) {
        self.buffer.push_str(&"\t".repeat(self.indentation));
    }

    fn end_line(&mut self) {
        self.buffer.push('\n');
    }

    fn write(&mut self, string: &str) {
        self.buffer.push_str(string);
    }

    fn write_line(&mut self, string: &str) {
        self.start_line();
        self.write(string);
        self.end_line();
    }

    fn fmt<T: Fmt>(&mut self, v: T) {
        v.fmt(self);
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self {
            buffer: String::default(),
            indentation: usize::default(),
        }
    }
}

impl Into<String> for Formatter {
    fn into(self) -> String {
        self.buffer
    }
}

/// The `Fmt` trait allows individual WebAssembly AST nodes
/// to be formatted separately.
pub trait Fmt {
    fn fmt(&self, formatter: &mut Formatter);
}

/// Format `.wat` source code.
/// Uses tabs for indentation.
/// Resolves symbolic identifiers and unfolds instruction expressions.
/// Extracts inline exports and type definitions.
/// Encodes all number literals in decimal notation.
pub fn fmt(source: &str) -> String {
    let buffer = ParseBuffer::new(source).unwrap();
    let mut wat = parser::parse::<Wat>(&buffer).unwrap();
    // TODO: Handle error
    wat.module.resolve().unwrap();
    let mut formatter = Formatter::new();
    wat.fmt(&mut formatter);
    formatter.into()
}

impl<'src> Fmt for Wat<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        self.module.fmt(formatter)
    }
}

impl<'src> Fmt for Module<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write_line("(module");
        formatter.indent();
        formatter.fmt(&self.kind);
        formatter.deindent();
        formatter.write_line(")");
    }
}

impl<'src> Fmt for &ModuleKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ModuleKind::Text(fields) => {
                formatter.fmt(fields);
            }
            ModuleKind::Binary(..) => unimplemented!(),
        }
    }
}

impl<'src> Fmt for &Vec<ModuleField<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        for field in self.iter() {
            formatter.fmt(field);
        }
    }
}

impl<'src> Fmt for &ModuleField<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ModuleField::Type(ty) => formatter.fmt(ty),
            ModuleField::Func(func) => formatter.fmt(func),
            ModuleField::Global(global) => formatter.fmt(global),
            ModuleField::Memory(..) => todo!(),
            ModuleField::Table(..) => todo!(),
            ModuleField::Elem(..) => todo!(),
            ModuleField::Data(..) => todo!(),
            ModuleField::Export(export) => formatter.fmt(export),
            ModuleField::Import(..) => todo!(),
            ModuleField::Start(index) => formatter.fmt(&Start { index: *index }),
            ModuleField::Custom(..) => todo!(),
            ModuleField::ExportAll(..) => unimplemented!(),
            ModuleField::Event(..) => unimplemented!(),
            ModuleField::Instance(..) => unimplemented!(),
            ModuleField::NestedModule(..) => unimplemented!(),
            ModuleField::Alias(..) => unimplemented!(),
        };
    }
}

struct Start<'src> {
    index: Index<'src>,
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

impl<'src> Fmt for &Type<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(type ");
        formatter.fmt(&self.def);
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &Func<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(func ");
        formatter.fmt(&self.ty);
        formatter.end_line();
        formatter.indent();
        formatter.fmt(&self.kind);
        formatter.deindent();
        formatter.start_line();
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &Global<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(global ");
        if let Some(id) = &self.id {
            formatter.fmt(id);
            formatter.write(" ");
        };

        if !self.exports.names.is_empty() {
            formatter.fmt(&self.exports);
            formatter.write(" ");
        };

        if let GlobalKind::Import(inline_import) = &self.kind {
            formatter.fmt(inline_import);
        };

        formatter.fmt(&self.ty);
        if let GlobalKind::Inline(expression) = &self.kind {
            formatter.write(" ");
            formatter.fmt(expression);
        };
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &InlineImport<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(import ");
        formatter.write(&self.module);
        if let Some(field) = &self.field {
            formatter.write(" ");
            formatter.write(field);
        };
        formatter.write(")");
    }
}

impl<'src> Fmt for &InlineExport<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(export");
        for name in &self.names {
            formatter.write(" ");
            formatter.write(name);
        }
        formatter.write(")");
    }
}

impl<'src> Fmt for &GlobalType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        if self.mutable {
            formatter.write("(mut ");
            formatter.fmt(&self.ty);
            formatter.write(")");
        } else {
            formatter.fmt(&self.ty);
        }
    }
}

impl<'src> Fmt for &Export<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(export ");
        formatter.fmt(self.name);
        formatter.write(" ");
        formatter.fmt(&self.kind);
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &TypeDef<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            TypeDef::Func(functy) => functy.fmt(formatter),
            TypeDef::Struct(..) => unimplemented!(),
            TypeDef::Array(..) => unimplemented!(),
            TypeDef::Module(..) => unimplemented!(),
            TypeDef::Instance(..) => unimplemented!(),
        };
    }
}

impl<'src> Fmt for &FunctionType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(func");
        if !self.params.is_empty() {
            formatter.write(" ");
        }
        formatter.fmt(&*self.params);
        if !self.results.is_empty() {
            formatter.write(" ");
        }
        formatter.fmt(&*self.results);
        formatter.write(")");
    }
}

impl<'src> Fmt for &ExportKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ExportKind::Func(index) => {
                formatter.write("(func ");
                formatter.fmt(index);
                formatter.write(")");
            }
            ExportKind::Type(..) => todo!(),
            ExportKind::Global(..) => todo!(),
            ExportKind::Instance(..) => todo!(),
            ExportKind::Memory(..) => todo!(),
            ExportKind::Table(..) => todo!(),
            ExportKind::Module(..) => todo!(),
            ExportKind::Event(..) => todo!(),
        };
    }
}

impl<'src> Fmt for &TypeUse<'src, FunctionType<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        if let Some(index) = self.index {
            formatter.fmt(&index);
        };

        if let Some(functy) = &self.inline {
            if self.index.is_some() {
                formatter.write(" ");
            }
            formatter.fmt(&*functy.params);
            if !functy.params.is_empty() {
                formatter.write(" ");
            }
            formatter.fmt(&*functy.results);
        };
    }
}

impl<'src> Fmt for &Index<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            Index::Num(n, ..) => formatter.fmt(n),
            Index::Id(id) => formatter.fmt(id),
        };
    }
}

impl<'src> Fmt for &Id<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        if !id_is_gensym(self) {
            formatter.write("$");
            formatter.write(self.name());
        }
    }
}

fn id_is_gensym(id: &Id) -> bool {
    id.name() == "gensym"
}

impl<'src> Fmt for &u32 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl<'src> Fmt for &i32 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl<'src> Fmt for &i64 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl<'src> Fmt for &f32 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl<'src> Fmt for &f64 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&self.to_string());
    }
}

impl Fmt for &Float32 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.fmt(&f32::from_bits(self.bits));
    }
}

impl<'src> Fmt for &Float64 {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.fmt(&f64::from_bits(self.bits));
    }
}

type Param<'src> = (
    Option<Id<'src>>,
    Option<NameAnnotation<'src>>,
    ValType<'src>,
);

// TODO: ID, Name
impl<'src> Fmt for &[Param<'src>] {
    fn fmt(&self, formatter: &mut Formatter) {
        if !self.is_empty() {
            formatter.write("(param");
            for param in self.iter() {
                formatter.write(" ");
                formatter.fmt(&param.2);
            }
            formatter.write(")");
        }
    }
}

impl<'src> Fmt for &[ValType<'src>] {
    fn fmt(&self, formatter: &mut Formatter) {
        if !self.is_empty() {
            formatter.write("(result");
            for result in self.iter() {
                formatter.write(" ");
                formatter.fmt(result);
            }
            formatter.write(")");
        }
    }
}

impl<'src> Fmt for &ValType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ValType::I32 => formatter.write("i32"),
            ValType::I64 => formatter.write("i64"),
            ValType::F32 => formatter.write("f32"),
            ValType::F64 => formatter.write("f64"),
            ValType::V128 => unimplemented!(),
            ValType::Ref(..) => unimplemented!(),
            ValType::Rtt(..) => unimplemented!(),
        };
    }
}

impl<'src> Fmt for &FuncKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            FuncKind::Import(..) => todo!(),
            FuncKind::Inline { locals, expression } => {
                formatter.fmt(locals);
                fmt_long_expression(expression, formatter);
            }
        };
    }
}

impl<'src> Fmt for &Vec<Local<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        if !self.is_empty() {
            formatter.start_line();
            formatter.write("(local");
            for local in self.iter() {
                formatter.write(" ");
                formatter.fmt(&local.ty);
            }
            formatter.write(")");
            formatter.end_line();
        }
    }
}

impl<'src> Fmt for &Expression<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        let mut iter = self.instrs.into_iter();
        if let Some(instruction) = iter.next() {
            formatter.fmt(instruction);
        }
        for instruction in iter {
            formatter.write(" ");
            formatter.fmt(instruction);
        }
    }
}

fn fmt_long_expression<'src>(expression: &Expression<'src>, formatter: &mut Formatter) {
    for instruction in expression.instrs.iter() {
        if is_block_end_instr(instruction) {
            formatter.deindent();
        }
        formatter.start_line();
        formatter.fmt(instruction);
        formatter.end_line();
        if is_block_start_instr(instruction) {
            formatter.indent();
        }
    }
}

fn is_block_end_instr(instruction: &Instruction) -> bool {
    matches!(instruction, Instruction::Else(..) | Instruction::End(..))
}

fn is_block_start_instr(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::Block(..)
            | Instruction::If(..)
            | Instruction::Loop(..)
            | Instruction::Else(..),
    )
}

impl<'src> Fmt for &Instruction<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        validate_instr(self);
        let name = instr_name(self);
        let args = instr_args(self);

        if let Some(args) = args {
            formatter.write("(");
            formatter.write(name);
            formatter.write(" ");
            formatter.write(&args);
            formatter.write(")");
        } else {
            formatter.write(name);
        }
    }
}

fn validate_instr(instruction: &Instruction) {
    match instruction {
        Instruction::MemorySize(memory_arg) | Instruction::MemoryGrow(memory_arg) => {
            if let Index::Num(n, ..) = memory_arg.mem {
                if n != 0 {
                    unimplemented!()
                }
            };
        },
        _ => {},
    }
}

fn instr_args(instruction: &Instruction) -> Option<String> {
    let mut formatter = Formatter::new();
    match instruction {
        Instruction::I32Const(n) => {
            formatter.fmt(n);
        }
        Instruction::I64Const(n) => {
            formatter.fmt(n);
        }
        Instruction::F32Const(f) => {
            formatter.fmt(f);
        }
        Instruction::F64Const(f) => {
            formatter.fmt(f);
        }
        Instruction::LocalGet(index)
        | Instruction::LocalSet(index)
        | Instruction::LocalTee(index)
        | Instruction::GlobalGet(index)
        | Instruction::GlobalSet(index)
        | Instruction::Br(index)
        | Instruction::BrIf(index)
        | Instruction::Call(index) => {
            formatter.fmt(index);
        }
        Instruction::BrTable(indices) => {
            formatter.fmt(indices);
        }
        Instruction::CallIndirect(call_indirect) => {
            formatter.fmt(&call_indirect.ty);
        }
        Instruction::Block(bt) | Instruction::Loop(bt) | Instruction::If(bt) => {
            if bt_is_empty(bt) {
                return None;
            }
            formatter.fmt(bt);
        }
        Instruction::I32Load8s(memarg)
        | Instruction::I32Load8u(memarg)
        | Instruction::I64Load8s(memarg)
        | Instruction::I64Load8u(memarg)
        | Instruction::I32Store8(memarg)
        | Instruction::I64Store8(memarg) => {
            let access_size = 1;
            if memarg_is_default(memarg, access_size) {
                return None;
            }
            formatter.fmt(memarg);
        }
        Instruction::I32Load16s(memarg)
        | Instruction::I32Load16u(memarg)
        | Instruction::I64Load16s(memarg)
        | Instruction::I64Load16u(memarg)
        | Instruction::I32Store16(memarg)
        | Instruction::I64Store16(memarg) => {
            let access_size = 2;
            if memarg_is_default(memarg, access_size) {
                return None;
            }
            formatter.fmt(memarg);
        }
        Instruction::I32Load(memarg)
        | Instruction::F32Load(memarg)
        | Instruction::I64Load32s(memarg)
        | Instruction::I64Load32u(memarg)
        | Instruction::I32Store(memarg)
        | Instruction::F32Store(memarg)
        | Instruction::I64Store32(memarg) => {
            let access_size = 4;
            if memarg_is_default(memarg, access_size) {
                return None;
            }
            formatter.fmt(memarg);
        }
        Instruction::I64Load(memarg)
        | Instruction::F64Load(memarg)
        | Instruction::I64Store(memarg)
        | Instruction::F64Store(memarg) => {
            let access_size = 8;
            if memarg_is_default(memarg, access_size) {
                return None;
            }
            formatter.fmt(memarg);
        }
        _ => return None,
    };
    Some(formatter.into())
}

fn instr_name(instruction: &Instruction) -> &'static str {
    match instruction {
        // Numeric instructions
        Instruction::I32Const(..) => "i32.const",
        Instruction::I64Const(..) => "i64.const",
        Instruction::F32Const(..) => "f32.const",
        Instruction::F64Const(..) => "f64.const",

        Instruction::I32Clz => "i32.clz",
        Instruction::I32Ctz => "i32.ctz",
        Instruction::I32Popcnt => "i32.popcnt",
        Instruction::I32Add => "i32.add",
        Instruction::I32Sub => "i32.sub",
        Instruction::I32Mul => "i32.mul",
        Instruction::I32DivS => "i32.div_s",
        Instruction::I32DivU => "i32.div_u",
        Instruction::I32RemS => "i32.rem_s",
        Instruction::I32RemU => "i32.rem_u",
        Instruction::I32And => "i32.and",
        Instruction::I32Or => "i32.or",
        Instruction::I32Xor => "i32.xor",
        Instruction::I32Shl => "i32.shl",
        Instruction::I32ShrS => "i32.shr_s",
        Instruction::I32ShrU => "i32.shr_u",
        Instruction::I32Rotl => "i32.rotl",
        Instruction::I32Rotr => "i32.rotr",

        Instruction::I64Clz => "i64.clz",
        Instruction::I64Ctz => "i64.ctz",
        Instruction::I64Popcnt => "i64.popcnt",
        Instruction::I64Add => "i64.add",
        Instruction::I64Sub => "i64.sub",
        Instruction::I64Mul => "i64.mul",
        Instruction::I64DivS => "i64.div_s",
        Instruction::I64DivU => "i64.div_u",
        Instruction::I64RemS => "i64.rem_s",
        Instruction::I64RemU => "i64.rem_u",
        Instruction::I64And => "i64.and",
        Instruction::I64Or => "i64.or",
        Instruction::I64Xor => "i64.xor",
        Instruction::I64Shl => "i64.shl",
        Instruction::I64ShrS => "i64.shr_s",
        Instruction::I64ShrU => "i64.shr_u",
        Instruction::I64Rotl => "i64.rotl",
        Instruction::I64Rotr => "i64.rotr",

        Instruction::F32Abs => "f32.abs",
        Instruction::F32Neg => "f32.neg",
        Instruction::F32Sqrt => "f32.sqrt",
        Instruction::F32Ceil => "f32.ceil",
        Instruction::F32Floor => "f32.floor",
        Instruction::F32Trunc => "f32.trunc",
        Instruction::F32Nearest => "f32.nearest",
        Instruction::F32Add => "f32.add",
        Instruction::F32Sub => "f32.sub",
        Instruction::F32Div => "f32.div",
        Instruction::F32Min => "f32.min",
        Instruction::F32Max => "f32.max",
        Instruction::F32Copysign => "f32.copysign",

        Instruction::F64Abs => "f32.abs",
        Instruction::F64Neg => "f32.neg",
        Instruction::F64Sqrt => "f32.sqrt",
        Instruction::F64Ceil => "f32.ceil",
        Instruction::F64Floor => "f32.floor",
        Instruction::F64Trunc => "f32.trunc",
        Instruction::F64Nearest => "f32.nearest",
        Instruction::F64Add => "f32.add",
        Instruction::F64Sub => "f32.sub",
        Instruction::F64Div => "f32.div",
        Instruction::F64Min => "f32.min",
        Instruction::F64Max => "f32.max",
        Instruction::F64Copysign => "f32.copysign",

        Instruction::I32Eqz => "i32.eqz",
        Instruction::I32Eq => "i32.eq",
        Instruction::I32Ne => "i32.ne",
        Instruction::I32LtS => "i32.lt_s",
        Instruction::I32LtU => "i32.lt_u",
        Instruction::I32GtS => "i32.gt_s",
        Instruction::I32GtU => "i32.gt_u",
        Instruction::I32LeS => "i32.le_s",
        Instruction::I32LeU => "i32.le_u",
        Instruction::I32GeS => "i32.ge_s",
        Instruction::I32GeU => "i32.ge_u",

        Instruction::I64Eqz => "i64.eqz",
        Instruction::I64Eq => "i64.eq",
        Instruction::I64Ne => "i64.ne",
        Instruction::I64LtS => "i64.lt_s",
        Instruction::I64LtU => "i64.lt_u",
        Instruction::I64GtS => "i64.gt_s",
        Instruction::I64GtU => "i64.gt_u",
        Instruction::I64LeS => "i64.le_s",
        Instruction::I64LeU => "i64.le_u",
        Instruction::I64GeS => "i64.ge_s",
        Instruction::I64GeU => "i64.ge_u",

        Instruction::F32Eq => "f32.eq",
        Instruction::F32Ne => "f32.ne",
        Instruction::F32Lt => "f32.lt",
        Instruction::F32Gt => "f32.gt",
        Instruction::F32Le => "f32.le",
        Instruction::F32Ge => "f32.ge",

        Instruction::F64Eq => "f32.eq",
        Instruction::F64Ne => "f32.ne",
        Instruction::F64Lt => "f32.lt",
        Instruction::F64Gt => "f32.gt",
        Instruction::F64Le => "f32.le",
        Instruction::F64Ge => "f32.ge",

        Instruction::I32WrapI64 => "i32.wrap_i64",
        Instruction::I32TruncF32S => "i32.trunc_f32_s",
        Instruction::I32TruncF32U => "i32.trunc_f32_u",
        Instruction::I32TruncF64S => "i32.trunc_f64_s",
        Instruction::I32TruncF64U => "i32.trunc_f32_u",
        Instruction::I32TruncSatF32S => "i32.trunc_sat_f32_s",
        Instruction::I32TruncSatF32U => "i32.trunc_sat_f32_u",
        Instruction::I32TruncSatF64S => "i32.trunc_sat_f64_s",
        Instruction::I32TruncSatF64U => "i32.trunc_sat_f64_u",
        Instruction::I64ExtendI32S => "i64.extend_i32_s",
        Instruction::I64ExtendI32U => "i64.extend_i32_u",
        Instruction::I64TruncF32S => "i64.trunc_f32_s",
        Instruction::I64TruncF32U => "i64.trunc_f_32_u",
        Instruction::I64TruncF64S => "i64.trunc_f64_s",
        Instruction::I64TruncF64U => "i64.trunc_f64_u",
        Instruction::I64TruncSatF32S => "i64.trunc_sat_f32_s",
        Instruction::I64TruncSatF32U => "i64.trunc_sat_f32_u",
        Instruction::I64TruncSatF64S => "i64.trunc_sat_f64_s",
        Instruction::I64TruncSatF64U => "i64.trunc_sat_f64_u",
        Instruction::F32ConvertI32S => "f32.convert_i32_s",
        Instruction::F32ConvertI32U => "f32.convert_i32_u",
        Instruction::F32ConvertI64S => "f32.convert_i64_s",
        Instruction::F32ConvertI64U => "f32.convert_i64_u",
        Instruction::F32DemoteF64 => "f32.demote_f64",
        Instruction::F64ConvertI32S => "f64.convert_i32_s",
        Instruction::F64ConvertI32U => "f64.convert_i32_u",
        Instruction::F64ConvertI64S => "f64.convert_i64_s",
        Instruction::F64ConvertI64U => "f64.convert_i64_u",
        Instruction::F64PromoteF32 => "f64.promote_f32",
        Instruction::I32ReinterpretF32 => "i32.reinterpret_f32",
        Instruction::I64ReinterpretF64 => "i64.reinterpret_f64",
        Instruction::F32ReinterpretI32 => "f32.reinterpret_i32",
        Instruction::F64ReinterpretI64 => "f64.reinterpret_i64",

        Instruction::I32Extend8S => "i32.extend_8_s",
        Instruction::I32Extend16S => "i32.extend_16_s",

        Instruction::I64Extend8S => "i64.extend_8_s",
        Instruction::I64Extend16S => "i64.extend_16_s",
        Instruction::I64Extend32S => "i64.extend_32_s",

        // Parametric instructions
        Instruction::Drop => "drop",
        Instruction::Select(_types) => "select",

        // Variable instructions
        Instruction::LocalGet(..) => "local.get",
        Instruction::LocalSet(..) => "local.set",
        Instruction::LocalTee(..) => "local.tee",
        Instruction::GlobalGet(..) => "global.get",
        Instruction::GlobalSet(..) => "global.set",

        // Memory instructions
        Instruction::I32Load(..) => "i32.load",
        Instruction::I64Load(..) => "i64.load",
        Instruction::F32Load(..) => "f32.load",
        Instruction::F64Load(..) => "f64.load",
        Instruction::I32Load8s(..) => "i32.load_8_s",
        Instruction::I32Load8u(..) => "i32.load_8_u",
        Instruction::I32Load16s(..) => "i32.load_16_s",
        Instruction::I32Load16u(..) => "i32.load_16_u",
        Instruction::I64Load8s(..) => "i64.load_8_s",
        Instruction::I64Load8u(..) => "i64.load_8_u",
        Instruction::I64Load16s(..) => "i64.load_16_s",
        Instruction::I64Load16u(..) => "i64.load_16_u",
        Instruction::I64Load32s(..) => "i64.load_32_s",
        Instruction::I64Load32u(..) => "i64.load_32_u",
        Instruction::I32Store(..) => "i32.store",
        Instruction::I64Store(..) => "i64.store",
        Instruction::F32Store(..) => "f32.store",
        Instruction::F64Store(..) => "f64.store",
        Instruction::I32Store8(..) => "i32.store_8",
        Instruction::I32Store16(..) => "i32.store_16",
        Instruction::I64Store8(..) => "i64.store_8",
        Instruction::I64Store16(..) => "i64.store_16",
        Instruction::I64Store32(..) => "i64.store_32",
        Instruction::MemorySize(..) => "memory.size",
        Instruction::MemoryGrow(..) => "memory.grow",

        // Control instructions
        Instruction::Unreachable => "unreachable",
        Instruction::Nop => "nop",
        Instruction::Br(..) => "br",
        Instruction::BrIf(..) => "br_if",
        Instruction::BrTable(..) => "br_table",
        Instruction::Return => "return",
        Instruction::Call(..) => "call",
        Instruction::CallIndirect(..) => "call_indirect",
        Instruction::Block(..) => "block",
        Instruction::Loop(..) => "loop",
        Instruction::If(..) => "if",
        // TODO: id
        Instruction::Else(..) => "else",
        Instruction::End(..) => "end",
        _ => unimplemented!(),
    }
}

fn bt_is_empty(block_type: &BlockType) -> bool {
    block_type.label.is_none() && ty_use_is_empty(&block_type.ty)
}

fn ty_use_is_empty<'a>(ty_use: &TypeUse<'a, FunctionType<'a>>) -> bool {
    ty_use.index.is_none()
        && ty_use
            .inline
            .as_ref()
            .map(|ty| func_ty_is_empty(&ty))
            .unwrap_or(false)
}

fn func_ty_is_empty(func_ty: &FunctionType) -> bool {
    func_ty.params.is_empty() && func_ty.results.is_empty()
}

fn memarg_is_default(memarg: &MemArg, access_size: u32) -> bool {
    memarg.offset == 0 && memarg.align == access_size
}

// TODO: id
impl<'src> Fmt for &BlockType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        if let Some(label) = &self.label {
            formatter.fmt(label);
            formatter.write(" ");
        }
        formatter.fmt(&self.ty);
    }
}

impl<'src> Fmt for &BrTableIndices<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        for label in &self.labels {
            formatter.fmt(label);
        }
        formatter.fmt(&self.default);
    }
}

impl<'src> Fmt for &'src str {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("\"");
        formatter.write(self);
        formatter.write("\"");
    }
}

impl<'src> Fmt for &MemArg<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write(&format!(
            "offset={offset} align={align}",
            offset = self.offset,
            align = self.align
        ));
    }
}

#[cfg(test)]
mod test {
    use super::fmt;

    #[test]
    fn fmt_add_desugar() {
        let input = include_str!("../tests/data/input/add_desugar.wat");
        let expected = include_str!("../tests/data/output/add_desugar.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_sugar() {
        let input = include_str!("../tests/data/input/add_sugar.wat");
        let expected = include_str!("../tests/data/output/add_sugar.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn fac_desugar() {
        let input = include_str!("../tests/data/input/fac_desugar.wat");
        let expected = include_str!("../tests/data/output/fac_desugar.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn fac_sugar() {
        let input = include_str!("../tests/data/input/fac_sugar.wat");
        let expected = include_str!("../tests/data/output/fac_sugar.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn memory_grow() {
        let input = include_str!("../tests/data/input/memory_grow.wat");
        let expected = include_str!("../tests/data/output/memory_grow.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
    }
}
