use wast::{BlockType, BrTableIndices, Export, ExportKind, Expression, Func, FuncKind, FunctionType, Id, Index, Instruction, Local, MemArg, Module, ModuleField, ModuleKind, NameAnnotation, Type, TypeDef, TypeUse, ValType, Wat, parser::{self, ParseBuffer}};
use crate::utils::indent;

pub fn fmt(source: &str) -> String {
    let buffer = ParseBuffer::new(source).unwrap();
    let wat = parser::parse::<Wat>(&buffer).unwrap();
    fmt_wat(wat)
}

fn fmt_wat(wat: Wat) -> String {
    fmt_module(wat.module)
}

fn fmt_module(mut module: Module) -> String {
    // TODO: Handle error
    module.resolve().unwrap();
    let mut buf = String::new();
    buf.push_str("(module\n");
    match module.kind {
        ModuleKind::Text(fields) => {
            let fields = indent(&fmt_module_fields(fields));
            buf.push_str(&fields);
        },
        ModuleKind::Binary(..) => todo!(),
    }
    buf.push_str("\n)\n");
    buf
}

fn fmt_module_fields(fields: Vec<ModuleField>) -> String {
    let mut buf = String::new();
    for field in fields {
        buf.push_str(&fmt_module_field(field));
    }
    buf
}

fn fmt_module_field(field: ModuleField) -> String {
    match field {
        ModuleField::Type(ty) => {
            fmt_type(ty)
        },
        ModuleField::Func(func) => {
            fmt_func(func)
        },
        ModuleField::Global(..) => todo!(),
        ModuleField::Memory(..) => todo!(), 
        ModuleField::Table(..) => todo!(),
        ModuleField::Elem(..) => todo!(),
        ModuleField::Data(..) => todo!(),
        ModuleField::Export(export) => {
            fmt_export(export)
        },
        ModuleField::Import(..) => todo!(),
        ModuleField::Start(..) => todo!(),
        _ => unimplemented!(),
    }
}

fn fmt_type(ty: Type) -> String {
    let mut buf = String::new();
    buf.push_str("(type ");
    match ty.def {
        TypeDef::Func(functy) => {
            buf.push_str(&fmt_func_ty(&functy));
        },
        _ => unimplemented!(),
    };
    buf.push(')');
    buf
}


fn fmt_func_ty(functy: &FunctionType) -> String {
    let mut buf = String::new();
    buf.push_str("(func ");

    let params = fmt_params(&functy.params);
    buf.push_str(&params);

    buf.push(' ');

    let results = fmt_results(&functy.results);
    buf.push_str(&results);

    buf.push(')');

    buf
}

fn fmt_func(func: Func) -> String {
    let mut buf = String::new();
    buf.push_str("(func ");
    buf.push_str(&fmt_ty_use(&func.ty));
    buf.push('\n');
    buf.push_str(&indent(&fmt_func_kind(func.kind)));
    buf.push_str("\n)\n");
    buf
}

fn fmt_ty_use<'src>(ty_use: &TypeUse<'src, FunctionType>) -> String {
    let mut buf = String::new();
    if let Some(index) = ty_use.index {
        buf.push_str(&fmt_index(&index));
        buf.push(' ');
    };

    if let Some(functy) = &ty_use.inline {
        buf.push_str(&fmt_params(&functy.params));
        buf.push(' ');
        buf.push_str(&fmt_results(&functy.results));
    };

    buf
}

fn fmt_index(index: &Index) -> String {
    match index {
        Index::Num(n, ..) => n.to_string(),
        Index::Id(..) => todo!(),
    }
}

type Param<'src> = (Option<Id<'src>>, Option<NameAnnotation<'src>>, ValType<'src>);

fn fmt_params(params: &[Param]) -> String {
    let mut buf = String::new();
    if !params.is_empty() {
        buf.push_str("(param");
        for param in params.iter() {
            buf.push(' ');
            buf.push_str(fmt_valty(&param.2));
        }
        buf.push(')');
    }
    buf
}

fn fmt_results(results: &[ValType]) -> String {
    let mut buf = String::new();
    if !results.is_empty() {
        buf.push_str("(result");
        for result in results.iter() {
            buf.push(' ');
            buf.push_str(fmt_valty(result));
        }
        buf.push(')');
    }
    buf
}

fn fmt_valty<'src>(ty: &ValType<'src>) -> &'src str {
    match ty {
        ValType::I32 => "i32",
        ValType::I64 => "i64",
        ValType::F32 => "f32",
        ValType::F64 => "f64",
        _ => todo!(),
    }
}

fn fmt_func_kind(kind: FuncKind) -> String {
    let mut buf = String::new();
    match kind {
        FuncKind::Import(..) => todo!(),
        FuncKind::Inline {
            locals,
            expression,
        } => {
            buf.push_str(&fmt_locals(locals));
            buf.push_str(&fmt_expression(expression));
        },
    };
    buf
}

fn fmt_locals(locals: Vec<Local>) -> String {
    let mut buf = String::new();
    if !locals.is_empty() {
        buf.push_str("(local");
        for local in locals {
            buf.push(' ');
            buf.push_str(&fmt_valty(&local.ty));
        }
        buf.push_str(")\n");
    }
    buf
}

fn fmt_expression(expression: Expression) -> String {
    let mut buf = String::new();
    let mut indentation = 0;
    for instruction in expression.instrs.iter() {
        if is_block_end_instr(instruction) {
            indentation -= 1;
        }
        buf.push_str(&"\t".repeat(indentation));
        buf.push_str(&fmt_instr(instruction));
        buf.push('\n');
        if is_block_start_instr(instruction) {
            indentation += 1;
        }
    }
    buf
}

fn is_block_end_instr(instruction: &Instruction) -> bool {
    matches!(instruction, Instruction::Else(..) | Instruction::End(..))
}

fn is_block_start_instr(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::Block(..) |
        Instruction::If(..) |
        Instruction::Loop(..) |
        Instruction::Else(..),
    )
}

fn fmt_instr(instruction: &Instruction) -> String {
    match instruction {
        // Numeric instructions
        Instruction::I32Const(n) => {
            format!("i32.const {}", n)
        },
        Instruction::I64Const(n) => {
            format!("i64.const {}", n)
        },
        Instruction::F32Const(f) => {
            format!("f32.const {}", f32::from_bits(f.bits))
        },
        Instruction::F64Const(f) => {
            format!("f64.const {}", f64::from_bits(f.bits))
        },


        Instruction::I32Clz => {
            "i32.clz".to_owned()
        },
        Instruction::I32Ctz => {
            "i32.ctz".to_owned()
        },
        Instruction::I32Popcnt => {
            "i32.popcnt".to_owned()
        },
        Instruction::I32Add => {
            "i32.add".to_owned()
        },
        Instruction::I32Sub => {
            "i32.sub".to_owned()
        },
        Instruction::I32Mul => {
            "i32.mul".to_owned()
        },
        Instruction::I32DivS => {
            "i32.div_s".to_owned()
        },
        Instruction::I32DivU => {
            "i32.div_u".to_owned()
        },
        Instruction::I32RemS => {
            "i32.rem_s".to_owned()
        },
        Instruction::I32RemU => {
            "i32.rem_u".to_owned()
        },
        Instruction::I32And => {
            "i32.and".to_owned()
        },
        Instruction::I32Or => {
            "i32.or".to_owned()
        },
        Instruction::I32Xor => {
            "i32.xor".to_owned()
        },
        Instruction::I32Shl => {
            "i32.shl".to_owned()
        },
        Instruction::I32ShrS => {
            "i32.shr_s".to_owned()
        },
        Instruction::I32ShrU => {
            "i32.shr_u".to_owned()
        },
        Instruction::I32Rotl => {
            "i32.rotl".to_owned()
        },
        Instruction::I32Rotr => {
            "i32.rotr".to_owned()
        },


        Instruction::I64Clz => {
            "i64.clz".to_owned()
        },
        Instruction::I64Ctz => {
            "i64.ctz".to_owned()
        },
        Instruction::I64Popcnt => {
            "i64.popcnt".to_owned()
        },
        Instruction::I64Add => {
            "i64.add".to_owned()
        },
        Instruction::I64Sub => {
            "i64.sub".to_owned()
        },
        Instruction::I64Mul => {
            "i64.mul".to_owned()
        },
        Instruction::I64DivS => {
            "i64.div_s".to_owned()
        },
        Instruction::I64DivU => {
            "i64.div_u".to_owned()
        },
        Instruction::I64RemS => {
            "i64.rem_s".to_owned()
        },
        Instruction::I64RemU => {
            "i64.rem_u".to_owned()
        },
        Instruction::I64And => {
            "i64.and".to_owned()
        },
        Instruction::I64Or => {
            "i64.or".to_owned()
        },
        Instruction::I64Xor => {
            "i64.xor".to_owned()
        },
        Instruction::I64Shl => {
            "i64.shl".to_owned()
        },
        Instruction::I64ShrS => {
            "i64.shr_s".to_owned()
        },
        Instruction::I64ShrU => {
            "i64.shr_u".to_owned()
        },
        Instruction::I64Rotl => {
            "i64.rotl".to_owned()
        },
        Instruction::I64Rotr => {
            "i64.rotr".to_owned()
        },


        Instruction::F32Abs => {
            "f32.abs".to_owned()
        },
        Instruction::F32Neg => {
            "f32.neg".to_owned()
        },
        Instruction::F32Sqrt => {
            "f32.sqrt".to_owned()
        },
        Instruction::F32Ceil => {
            "f32.ceil".to_owned()
        },
        Instruction::F32Floor => {
            "f32.floor".to_owned()
        },
        Instruction::F32Trunc => {
            "f32.trunc".to_owned()
        },
        Instruction::F32Nearest => {
            "f32.nearest".to_owned()
        },
        Instruction::F32Add => {
            "f32.add".to_owned()
        },
        Instruction::F32Sub => {
            "f32.sub".to_owned()
        },
        Instruction::F32Div => {
            "f32.div".to_owned()
        },
        Instruction::F32Min => {
            "f32.min".to_owned()
        },
        Instruction::F32Max => {
            "f32.max".to_owned()
        },
        Instruction::F32Copysign => {
            "f32.copysign".to_owned()
        },


        Instruction::F64Abs => {
            "f32.abs".to_owned()
        },
        Instruction::F64Neg => {
            "f32.neg".to_owned()
        },
        Instruction::F64Sqrt => {
            "f32.sqrt".to_owned()
        },
        Instruction::F64Ceil => {
            "f32.ceil".to_owned()
        },
        Instruction::F64Floor => {
            "f32.floor".to_owned()
        },
        Instruction::F64Trunc => {
            "f32.trunc".to_owned()
        },
        Instruction::F64Nearest => {
            "f32.nearest".to_owned()
        },
        Instruction::F64Add => {
            "f32.add".to_owned()
        },
        Instruction::F64Sub => {
            "f32.sub".to_owned()
        },
        Instruction::F64Div => {
            "f32.div".to_owned()
        },
        Instruction::F64Min => {
            "f32.min".to_owned()
        },
        Instruction::F64Max => {
            "f32.max".to_owned()
        },
        Instruction::F64Copysign => {
            "f32.copysign".to_owned()
        },


        Instruction::I32Eqz => {
            "i32.eqz".to_owned()
        },
        Instruction::I32Eq => {
            "i32.eq".to_owned()
        },
        Instruction::I32Ne => {
            "i32.ne".to_owned()
        },
        Instruction::I32LtS => {
            "i32.lt_s".to_owned()
        },
        Instruction::I32LtU => {
            "i32.lt_u".to_owned()
        },
        Instruction::I32GtS => {
            "i32.gt_s".to_owned()
        },
        Instruction::I32GtU => {
            "i32.gt_u".to_owned()
        },
        Instruction::I32LeS => {
            "i32.le_s".to_owned()
        },
        Instruction::I32LeU => {
            "i32.le_u".to_owned()
        },
        Instruction::I32GeS => {
            "i32.ge_s".to_owned()
        },
        Instruction::I32GeU => {
            "i32.ge_u".to_owned()
        },


        Instruction::I64Eqz => {
            "i64.eqz".to_owned()
        },
        Instruction::I64Eq => {
            "i64.eq".to_owned()
        },
        Instruction::I64Ne => {
            "i64.ne".to_owned()
        },
        Instruction::I64LtS => {
            "i64.lt_s".to_owned()
        },
        Instruction::I64LtU => {
            "i64.lt_u".to_owned()
        },
        Instruction::I64GtS => {
            "i64.gt_s".to_owned()
        },
        Instruction::I64GtU => {
            "i64.gt_u".to_owned()
        },
        Instruction::I64LeS => {
            "i64.le_s".to_owned()
        },
        Instruction::I64LeU => {
            "i64.le_u".to_owned()
        },
        Instruction::I64GeS => {
            "i64.ge_s".to_owned()
        },
        Instruction::I64GeU => {
            "i64.ge_u".to_owned()
        },


        Instruction::F32Eq => {
            "f32.eq".to_owned()
        },
        Instruction::F32Ne => {
            "f32.ne".to_owned()
        },
        Instruction::F32Lt => {
            "f32.lt".to_owned()
        },
        Instruction::F32Gt => {
            "f32.gt".to_owned()
        },
        Instruction::F32Le => {
            "f32.le".to_owned()
        },
        Instruction::F32Ge => {
            "f32.ge".to_owned()
        },


        Instruction::F64Eq => {
            "f32.eq".to_owned()
        },
        Instruction::F64Ne => {
            "f32.ne".to_owned()
        },
        Instruction::F64Lt => {
            "f32.lt".to_owned()
        },
        Instruction::F64Gt => {
            "f32.gt".to_owned()
        },
        Instruction::F64Le => {
            "f32.le".to_owned()
        },
        Instruction::F64Ge => {
            "f32.ge".to_owned()
        },


        Instruction::I32WrapI64 => {
            "i32.wrap_i64".to_owned()
        },
        Instruction::I32TruncF32S => {
            "i32.trunc_f32_s".to_owned()
        },
        Instruction::I32TruncF32U => {
            "i32.trunc_f32_u".to_owned()
        },
        Instruction::I32TruncF64S => {
            "i32.trunc_f64_s".to_owned()
        },
        Instruction::I32TruncF64U => {
            "i32.trunc_f32_u".to_owned()
        },
        Instruction::I32TruncSatF32S => {
            "i32.trunc_sat_f32_s".to_owned()
        },
        Instruction::I32TruncSatF32U => {
            "i32.trunc_sat_f32_u".to_owned()
        },
        Instruction::I32TruncSatF64S => {
            "i32.trunc_sat_f64_s".to_owned()
        },
        Instruction::I32TruncSatF64U => {
            "i32.trunc_sat_f64_u".to_owned()
        },
        Instruction::I64ExtendI32S => {
            "i64.extend_i32_s".to_owned()
        },
        Instruction::I64ExtendI32U => {
            "i64.extend_i32_u".to_owned()
        },
        Instruction::I64TruncF32S => {
            "i64.trunc_f32_s".to_owned()
        },
        Instruction::I64TruncF32U => {
            "i64.trunc_f_32_u".to_owned()
        },
        Instruction::I64TruncF64S => {
            "i64.trunc_f64_s".to_owned()
        },
        Instruction::I64TruncF64U => {
            "i64.trunc_f64_u".to_owned()
        },
        Instruction::I64TruncSatF32S => {
            "i64.trunc_sat_f32_s".to_owned()
        },
        Instruction::I64TruncSatF32U => {
            "i64.trunc_sat_f32_u".to_owned()
        },
        Instruction::I64TruncSatF64S => {
            "i64.trunc_sat_f64_s".to_owned()
        },
        Instruction::I64TruncSatF64U => {
            "i64.trunc_sat_f64_u".to_owned()
        },
        Instruction::F32ConvertI32S => {
            "f32.convert_i32_s".to_owned()
        },
        Instruction::F32ConvertI32U => {
            "f32.convert_i32_u".to_owned()
        },
        Instruction::F32ConvertI64S => {
            "f32.convert_i64_s".to_owned()
        },
        Instruction::F32ConvertI64U => {
            "f32.convert_i64_u".to_owned()
        },
        Instruction::F32DemoteF64 => {
            "f32.demote_f64".to_owned()
        },
        Instruction::F64ConvertI32S => {
            "f64.convert_i32_s".to_owned()
        },
        Instruction::F64ConvertI32U => {
            "f64.convert_i32_u".to_owned()
        },
        Instruction::F64ConvertI64S => {
            "f64.convert_i64_s".to_owned()
        },
        Instruction::F64ConvertI64U => {
            "f64.convert_i64_u".to_owned()
        },
        Instruction::F64PromoteF32 => {
            "f64.promote_f32".to_owned()
        },
        Instruction::I32ReinterpretF32 => {
            "i32.reinterpret_f32".to_owned()
        },
        Instruction::I64ReinterpretF64 => {
            "i64.reinterpret_f64".to_owned()
        },
        Instruction::F32ReinterpretI32 => {
            "f32.reinterpret_i32".to_owned()
        },
        Instruction::F64ReinterpretI64 => {
            "f64.reinterpret_i64".to_owned()
        },


        Instruction::I32Extend8S => {
            "i32.extend_8_s".to_owned()
        },
        Instruction::I32Extend16S => {
            "i32.extend_16_s".to_owned()
        },


        Instruction::I64Extend8S => {
            "i64.extend_8_s".to_owned()
        },
        Instruction::I64Extend16S => {
            "i64.extend_16_s".to_owned()
        },
        Instruction::I64Extend32S => {
            "i64.extend_32_s".to_owned()
        },


        // Parametric instructions
        Instruction::Drop => {
            "drop".to_owned()
        },
        Instruction::Select(_types) => {
            "select".to_owned()
        },

        // Variable instructions
        Instruction::LocalGet(index) => {
            format!("local.get {}", fmt_index(index))
        },
        Instruction::LocalSet(index) => {
            format!("local.set {}", fmt_index(index))
        },
        Instruction::LocalTee(index) => {
            format!("local.tee {}", fmt_index(index))
        }
        Instruction::GlobalGet(index) => {
            format!("global.get {}", fmt_index(index))
        },
        Instruction::GlobalSet(index) => {
            format!("global.set {}", fmt_index(index))
        },

        // Memory instructions
        Instruction::I32Load(memarg) => {
            format!("i32.load {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Load(memarg) => {
            format!("i64.load {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::F32Load(memarg) => {
            format!("f32.load {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::F64Load(memarg) => {
            format!("f64.load {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I32Load8s(memarg) => {
            format!("i32.load_8_s {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I32Load8u(memarg) => {
            format!("i32.load_8_u {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I32Load16s(memarg) => {
            format!("i32.load_16_s {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I32Load16u(memarg) => {
            format!("i32.load_16_u {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Load8s(memarg) => {
            format!("i64.load_8_s {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Load8u(memarg) => {
            format!("i64.load_8_u {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Load16s(memarg) => {
            format!("i64.load_16_s {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Load16u(memarg) => {
            format!("i64.load_16_u {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Load32s(memarg) => {
            format!("i64.load_32_s {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Load32u(memarg) => {
            format!("i64.load_32_u {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I32Store(memarg) => {
            format!("i32.store {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Store(memarg) => {
            format!("i64.store {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::F32Store(memarg) => {
            format!("f32.store {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::F64Store(memarg) => {
            format!("f64.store {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I32Store8(memarg) => {
            format!("i32.store_8 {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I32Store16(memarg) => {
            format!("i32.store_16 {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Store8(memarg) => {
            format!("i64.store_8 {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Store16(memarg) => {
            format!("i64.store_16 {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::I64Store32(memarg) => {
            format!("i64.store_32 {}", fmt_memarg(&memarg))
                .trim()
                .to_owned()
        },
        Instruction::MemorySize(memory_arg) => {
            if let Index::Num(n, ..) = memory_arg.mem {
                if n != 0 {
                    unimplemented!()
                }
            };
            "memory.size".to_owned()
        },
        Instruction::MemoryGrow(memory_arg) => {
            if let Index::Num(n, ..) = memory_arg.mem {
                if n != 0 {
                    unimplemented!()
                }
            };
            "memory.grow".to_owned()
        },

        // Control instructions
        Instruction::Unreachable => {
            "unreachable".to_owned()
        },
        Instruction::Nop => {
            "nop".to_owned()
        },
        Instruction::Br(index) => {
            format!("br {}", fmt_index(index))
        },
        Instruction::BrIf(index) => {
            format!("br_if {}", fmt_index(index))
        },
        Instruction::BrTable(indices) => {
            format!("br_table {}", fmt_branch_indices(indices))
        },
        Instruction::Return => {
            "return".to_owned()
        },
        Instruction::Call(index) => {
            format!("call {}", fmt_index(index))
        },
        Instruction::CallIndirect(call_indirect) => {
            format!("call_indirect {}", fmt_ty_use(&call_indirect.ty))
        },
        Instruction::Block(bt) => {
            format!("block {}", fmt_blocktype(bt))
                .trim()
                .to_owned()
        },
        Instruction::Loop(bt) => {
            format!("loop {}", fmt_blocktype(bt))
                .trim()
                .to_owned()
        },
        Instruction::If(bt) => {
            format!("if {}", fmt_blocktype(bt))
                .trim()
                .to_owned()
        },
        // TODO: id
        Instruction::Else(..) => {
            "else".to_owned()
        },
        Instruction::End(..) => {
            "end".to_owned()
        },
        _ => unimplemented!(),
    }
}

// TODO: id
fn fmt_blocktype(blocktype: &BlockType) -> String {
    fmt_ty_use(&blocktype.ty)
}

fn fmt_branch_indices(indices: &BrTableIndices) -> String {
    let mut buf = String::new();
    for label in &indices.labels {
        buf.push_str(&fmt_index(label));
    }
    buf.push_str(&fmt_index(&indices.default));
    buf
}

fn fmt_export(export: Export) -> String {
    let mut buf = String::new();

    buf.push_str("(export ");
    buf.push_str(&fmt_string(export.name));
    buf.push(' ');
    buf.push_str(&fmt_export_kind(export.kind));
    buf.push_str(")\n");
    buf
}

fn fmt_export_kind(kind: ExportKind) -> String {
    match kind {
        ExportKind::Func(index) => {
            format!("(func {})", fmt_index(&index))
        },
        ExportKind::Type(..) => todo!(),
        ExportKind::Global(..) => todo!(),
        ExportKind::Instance(..) => todo!(),
        ExportKind::Memory(..) => todo!(),
        ExportKind::Table(..) => todo!(),
        ExportKind::Module(..) => todo!(),
        ExportKind::Event(..) => todo!(),
    }
}

fn fmt_string(string: &str) -> String {
    format!("\"{}\"", string)
}

fn fmt_memarg(memarg: &MemArg) -> String {
    format!("offset={offset} align={align}",
        offset = memarg.offset,
        align = memarg.align
    )
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
}
