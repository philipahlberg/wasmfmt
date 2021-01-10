use wast::{BlockType, BrTableIndices, Export, ExportKind, Expression, Func, FuncKind, FunctionType, Id, Index, Instruction, Local, MemArg, Module, ModuleField, ModuleKind, NameAnnotation, Type, TypeDef, TypeUse, ValType, Wat, parser::{self, ParseBuffer}};
use crate::utils::indent;

pub fn fmt<'src>(source: &'src str) -> String {
    let buffer = ParseBuffer::new(source).unwrap();
    let wat = parser::parse::<Wat>(&buffer).unwrap();
    fmt_wat(wat)
}

fn fmt_wat<'src>(wat: Wat<'src>) -> String {
    fmt_module(wat.module)
}

fn fmt_module<'src>(mut module: Module<'src>) -> String {
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

fn fmt_module_fields<'src>(fields: Vec<ModuleField<'src>>) -> String {
    let mut buf = String::new();
    for field in fields {
        buf.push_str(&fmt_module_field(field));
    }
    buf
}

fn fmt_module_field<'src>(field: ModuleField<'src>) -> String {
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

fn fmt_type<'src>(ty: Type<'src>) -> String {
    let mut buf = String::new();
    buf.push_str("(type ");
    match ty.def {
        TypeDef::Func(functy) => {
            buf.push_str(&fmt_func_ty(&functy));
        },
        _ => unimplemented!(),
    };
    buf.push_str(")");
    buf
}


fn fmt_func_ty<'src>(functy: &FunctionType<'src>) -> String {
    let mut buf = String::new();
    buf.push_str("(func ");

    let params = fmt_params(&functy.params);
    buf.push_str(&params);

    buf.push_str(" ");

    let results = fmt_results(&functy.results);
    buf.push_str(&results);

    buf.push_str(")");

    buf
}

fn fmt_func<'src>(func: Func<'src>) -> String {
    let mut buf = String::new();
    buf.push_str("(func ");
    buf.push_str(&fmt_ty_use(&func.ty));
    buf.push_str("\n");
    buf.push_str(&indent(&fmt_func_kind(func.kind)));
    buf.push_str("\n)\n");
    buf
}

fn fmt_ty_use<'src>(ty_use: &TypeUse<'src, FunctionType>) -> String {
    let mut buf = String::new();
    if let Some(index) = ty_use.index {
        buf.push_str(&fmt_index(&index));
        buf.push_str(" ");
    };

    if let Some(functy) = &ty_use.inline {
        buf.push_str(&fmt_params(&functy.params));
        buf.push_str(" ");
        buf.push_str(&fmt_results(&functy.results));
    };

    buf
}

fn fmt_index<'src>(index: &Index<'src>) -> String {
    match index {
        Index::Num(n, ..) => n.to_string(),
        Index::Id(..) => todo!(),
    }
}

type Param<'src> = (Option<Id<'src>>, Option<NameAnnotation<'src>>, ValType<'src>);

fn fmt_params<'src>(params: &[Param<'src>]) -> String {
    let mut buf = String::new();
    if params.len() > 0 {
        buf.push_str("(param");
        for param in params.into_iter() {
            buf.push_str(" ");
            buf.push_str(fmt_valty(&param.2));
        }
        buf.push_str(")");
    }
    buf
}

fn fmt_results<'src>(results: &[ValType<'src>]) -> String {
    let mut buf = String::new();
    if results.len() > 0 {
        buf.push_str("(result");
        for result in results.into_iter() {
            buf.push_str(" ");
            buf.push_str(fmt_valty(result));
        }
        buf.push_str(")");
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

fn fmt_func_kind<'src>(kind: FuncKind<'src>) -> String {
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
            buf.push_str(" ");
            buf.push_str(&fmt_valty(&local.ty));
        }
        buf.push_str(")\n");
    }
    buf
}

fn fmt_expression<'src>(expression: Expression<'src>) -> String {
    let mut buf = String::new();
    let mut indentation = 0;
    for instruction in expression.instrs.into_iter() {
        if is_block_end_instr(instruction) {
            indentation -= 1;
        }
        buf.push_str(&"\t".repeat(indentation));
        buf.push_str(&fmt_instr(instruction));
        buf.push_str("\n");
        if is_block_start_instr(instruction) {
            indentation += 1;
        }
    }
    buf
}

fn is_block_end_instr<'src>(instruction: &Instruction<'src>) -> bool {
    match instruction {
        Instruction::Else(..) | Instruction::End(..) => true,
        _ => false,
    }
}

fn is_block_start_instr<'src>(instruction: &Instruction<'src>) -> bool {
    match instruction {
        Instruction::Block(..) |
        Instruction::If(..) |
        Instruction::Loop(..) |
        Instruction::Else(..) => true,
        _ => false,
    }
}

fn fmt_instr<'src>(instruction: &Instruction<'src>) -> String {
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
            format!("i32.clz")
        },
        Instruction::I32Ctz => {
            format!("i32.ctz")
        },
        Instruction::I32Popcnt => {
            format!("i32.popcnt")
        },
        Instruction::I32Add => {
            format!("i32.add")
        },
        Instruction::I32Sub => {
            format!("i32.sub")
        },
        Instruction::I32Mul => {
            format!("i32.mul")
        },
        Instruction::I32DivS => {
            format!("i32.div_s")
        },
        Instruction::I32DivU => {
            format!("i32.div_u")
        },
        Instruction::I32RemS => {
            format!("i32.rem_s")
        },
        Instruction::I32RemU => {
            format!("i32.rem_u")
        },
        Instruction::I32And => {
            format!("i32.and")
        },
        Instruction::I32Or => {
            format!("i32.or")
        },
        Instruction::I32Xor => {
            format!("i32.xor")
        },
        Instruction::I32Shl => {
            format!("i32.shl")
        },
        Instruction::I32ShrS => {
            format!("i32.shr_s")
        },
        Instruction::I32ShrU => {
            format!("i32.shr_u")
        },
        Instruction::I32Rotl => {
            format!("i32.rotl")
        },
        Instruction::I32Rotr => {
            format!("i32.rotr")
        },


        Instruction::I64Clz => {
            format!("i64.clz")
        },
        Instruction::I64Ctz => {
            format!("i64.ctz")
        },
        Instruction::I64Popcnt => {
            format!("i64.popcnt")
        },
        Instruction::I64Add => {
            format!("i64.add")
        },
        Instruction::I64Sub => {
            format!("i64.sub")
        },
        Instruction::I64Mul => {
            format!("i64.mul")
        },
        Instruction::I64DivS => {
            format!("i64.div_s")
        },
        Instruction::I64DivU => {
            format!("i64.div_u")
        },
        Instruction::I64RemS => {
            format!("i64.rem_s")
        },
        Instruction::I64RemU => {
            format!("i64.rem_u")
        },
        Instruction::I64And => {
            format!("i64.and")
        },
        Instruction::I64Or => {
            format!("i64.or")
        },
        Instruction::I64Xor => {
            format!("i64.xor")
        },
        Instruction::I64Shl => {
            format!("i64.shl")
        },
        Instruction::I64ShrS => {
            format!("i64.shr_s")
        },
        Instruction::I64ShrU => {
            format!("i64.shr_u")
        },
        Instruction::I64Rotl => {
            format!("i64.rotl")
        },
        Instruction::I64Rotr => {
            format!("i64.rotr")
        },


        Instruction::F32Abs => {
            format!("f32.abs")
        },
        Instruction::F32Neg => {
            format!("f32.neg")
        },
        Instruction::F32Sqrt => {
            format!("f32.sqrt")
        },
        Instruction::F32Ceil => {
            format!("f32.ceil")
        },
        Instruction::F32Floor => {
            format!("f32.floor")
        },
        Instruction::F32Trunc => {
            format!("f32.trunc")
        },
        Instruction::F32Nearest => {
            format!("f32.nearest")
        },
        Instruction::F32Add => {
            format!("f32.add")
        },
        Instruction::F32Sub => {
            format!("f32.sub")
        },
        Instruction::F32Div => {
            format!("f32.div")
        },
        Instruction::F32Min => {
            format!("f32.min")
        },
        Instruction::F32Max => {
            format!("f32.max")
        },
        Instruction::F32Copysign => {
            format!("f32.copysign")
        },


        Instruction::F64Abs => {
            format!("f32.abs")
        },
        Instruction::F64Neg => {
            format!("f32.neg")
        },
        Instruction::F64Sqrt => {
            format!("f32.sqrt")
        },
        Instruction::F64Ceil => {
            format!("f32.ceil")
        },
        Instruction::F64Floor => {
            format!("f32.floor")
        },
        Instruction::F64Trunc => {
            format!("f32.trunc")
        },
        Instruction::F64Nearest => {
            format!("f32.nearest")
        },
        Instruction::F64Add => {
            format!("f32.add")
        },
        Instruction::F64Sub => {
            format!("f32.sub")
        },
        Instruction::F64Div => {
            format!("f32.div")
        },
        Instruction::F64Min => {
            format!("f32.min")
        },
        Instruction::F64Max => {
            format!("f32.max")
        },
        Instruction::F64Copysign => {
            format!("f32.copysign")
        },


        Instruction::I32Eqz => {
            format!("i32.eqz")
        },
        Instruction::I32Eq => {
            format!("i32.eq")
        },
        Instruction::I32Ne => {
            format!("i32.ne")
        },
        Instruction::I32LtS => {
            format!("i32.lt_s")
        },
        Instruction::I32LtU => {
            format!("i32.lt_u")
        },
        Instruction::I32GtS => {
            format!("i32.gt_s")
        },
        Instruction::I32GtU => {
            format!("i32.gt_u")
        },
        Instruction::I32LeS => {
            format!("i32.le_s")
        },
        Instruction::I32LeU => {
            format!("i32.le_u")
        },
        Instruction::I32GeS => {
            format!("i32.ge_s")
        },
        Instruction::I32GeU => {
            format!("i32.ge_u")
        },


        Instruction::I64Eqz => {
            format!("i64.eqz")
        },
        Instruction::I64Eq => {
            format!("i64.eq")
        },
        Instruction::I64Ne => {
            format!("i64.ne")
        },
        Instruction::I64LtS => {
            format!("i64.lt_s")
        },
        Instruction::I64LtU => {
            format!("i64.lt_u")
        },
        Instruction::I64GtS => {
            format!("i64.gt_s")
        },
        Instruction::I64GtU => {
            format!("i64.gt_u")
        },
        Instruction::I64LeS => {
            format!("i64.le_s")
        },
        Instruction::I64LeU => {
            format!("i64.le_u")
        },
        Instruction::I64GeS => {
            format!("i64.ge_s")
        },
        Instruction::I64GeU => {
            format!("i64.ge_u")
        },


        Instruction::F32Eq => {
            format!("f32.eq")
        },
        Instruction::F32Ne => {
            format!("f32.ne")
        },
        Instruction::F32Lt => {
            format!("f32.lt")
        },
        Instruction::F32Gt => {
            format!("f32.gt")
        },
        Instruction::F32Le => {
            format!("f32.le")
        },
        Instruction::F32Ge => {
            format!("f32.ge")
        },


        Instruction::F64Eq => {
            format!("f32.eq")
        },
        Instruction::F64Ne => {
            format!("f32.ne")
        },
        Instruction::F64Lt => {
            format!("f32.lt")
        },
        Instruction::F64Gt => {
            format!("f32.gt")
        },
        Instruction::F64Le => {
            format!("f32.le")
        },
        Instruction::F64Ge => {
            format!("f32.ge")
        },


        Instruction::I32WrapI64 => {
            format!("i32.wrap_i64")
        },
        Instruction::I32TruncF32S => {
            format!("i32.trunc_f32_s")
        },
        Instruction::I32TruncF32U => {
            format!("i32.trunc_f32_u")
        },
        Instruction::I32TruncF64S => {
            format!("i32.trunc_f64_s")
        },
        Instruction::I32TruncF64U => {
            format!("i32.trunc_f32_u")
        },
        Instruction::I32TruncSatF32S => {
            format!("i32.trunc_sat_f32_s")
        },
        Instruction::I32TruncSatF32U => {
            format!("i32.trunc_sat_f32_u")
        },
        Instruction::I32TruncSatF64S => {
            format!("i32.trunc_sat_f64_s")
        },
        Instruction::I32TruncSatF64U => {
            format!("i32.trunc_sat_f64_u")
        },
        Instruction::I64ExtendI32S => {
            format!("i64.extend_i32_s")
        },
        Instruction::I64ExtendI32U => {
            format!("i64.extend_i32_u")
        },
        Instruction::I64TruncF32S => {
            format!("i64.trunc_f32_s")
        },
        Instruction::I64TruncF32U => {
            format!("i64.trunc_f_32_u")
        },
        Instruction::I64TruncF64S => {
            format!("i64.trunc_f64_s")
        },
        Instruction::I64TruncF64U => {
            format!("i64.trunc_f64_u")
        },
        Instruction::I64TruncSatF32S => {
            format!("i64.trunc_sat_f32_s")
        },
        Instruction::I64TruncSatF32U => {
            format!("i64.trunc_sat_f32_u")
        },
        Instruction::I64TruncSatF64S => {
            format!("i64.trunc_sat_f64_s")
        },
        Instruction::I64TruncSatF64U => {
            format!("i64.trunc_sat_f64_u")
        },
        Instruction::F32ConvertI32S => {
            format!("f32.convert_i32_s")
        },
        Instruction::F32ConvertI32U => {
            format!("f32.convert_i32_u")
        },
        Instruction::F32ConvertI64S => {
            format!("f32.convert_i64_s")
        },
        Instruction::F32ConvertI64U => {
            format!("f32.convert_i64_u")
        },
        Instruction::F32DemoteF64 => {
            format!("f32.demote_f64")
        },
        Instruction::F64ConvertI32S => {
            format!("f64.convert_i32_s")
        },
        Instruction::F64ConvertI32U => {
            format!("f64.convert_i32_u")
        },
        Instruction::F64ConvertI64S => {
            format!("f64.convert_i64_s")
        },
        Instruction::F64ConvertI64U => {
            format!("f64.convert_i64_u")
        },
        Instruction::F64PromoteF32 => {
            format!("f64.promote_f32")
        },
        Instruction::I32ReinterpretF32 => {
            format!("i32.reinterpret_f32")
        },
        Instruction::I64ReinterpretF64 => {
            format!("i64.reinterpret_f64")
        },
        Instruction::F32ReinterpretI32 => {
            format!("f32.reinterpret_i32")
        },
        Instruction::F64ReinterpretI64 => {
            format!("f64.reinterpret_i64")
        },


        Instruction::I32Extend8S => {
            format!("i32.extend_8_s")
        },
        Instruction::I32Extend16S => {
            format!("i32.extend_16_s")
        },


        Instruction::I64Extend8S => {
            format!("i64.extend_8_s")
        },
        Instruction::I64Extend16S => {
            format!("i64.extend_16_s")
        },
        Instruction::I64Extend32S => {
            format!("i64.extend_32_s")
        },


        // Parametric instructions
        Instruction::Drop => {
            format!("drop")
        },
        Instruction::Select(_types) => {
            format!("select")
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
            format!("memory.size")
        },
        Instruction::MemoryGrow(memory_arg) => {
            if let Index::Num(n, ..) = memory_arg.mem {
                if n != 0 {
                    unimplemented!()
                }
            };
            format!("memory.grow")
        },

        // Control instructions
        Instruction::Unreachable => {
            format!("unreachable")
        },
        Instruction::Nop => {
            format!("nop")
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
            format!("return")
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
            format!("else")
        },
        Instruction::End(..) => {
            format!("end")
        },
        _ => todo!("other instruction"),
    }
}

// TODO: id
fn fmt_blocktype<'src>(blocktype: &BlockType<'src>) -> String {
    fmt_ty_use(&blocktype.ty)
}

fn fmt_branch_indices<'src>(indices: &BrTableIndices<'src>) -> String {
    let mut buf = String::new();
    for label in &indices.labels {
        buf.push_str(&fmt_index(label));
    }
    buf.push_str(&fmt_index(&indices.default));
    buf
}

fn fmt_export<'src>(export: Export<'src>) -> String {
    let mut buf = String::new();

    buf.push_str("(export ");
    buf.push_str(&fmt_string(export.name));
    buf.push_str(" ");
    buf.push_str(&fmt_export_kind(export.kind));
    buf.push_str(")\n");
    buf
}

fn fmt_export_kind<'src>(kind: ExportKind<'src>) -> String {
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

fn fmt_string<'src>(string: &str) -> String {
    format!("\"{}\"", string)
}

fn fmt_memarg<'src>(memarg: &MemArg<'src>) -> String {
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
