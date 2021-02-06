use super::utils::{bt_is_empty, is_block_start_instr, is_valid_memory_arg, ty_use_is_empty};
use super::{Fmt, Formatter};
use wast::{BlockType, BrTableIndices, Instruction, MemArg};

impl<'src> Fmt for &Instruction<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        if !instr_is_valid(self) {
            unimplemented!();
        }
        let name = instr_name(self);
        let args = instr_args(self);
        if let Some(args) = args {
            if is_block_start_instr(self) {
                formatter.write(name);
                formatter.write(" ");
                formatter.write(&args);
            } else {
                formatter.write("(");
                formatter.write(name);
                formatter.write(" ");
                formatter.write(&args);
                formatter.write(")");
            }
        } else {
            formatter.write(name);
        }
    }
}

fn instr_is_valid(instruction: &Instruction) -> bool {
    match instruction {
        Instruction::MemorySize(arg) | Instruction::MemoryGrow(arg) => is_valid_memory_arg(arg),
        _ => true,
    }
}

fn instr_args(instruction: &Instruction) -> Option<String> {
    let mut formatter = Formatter::new();
    match instruction {
        Instruction::I32Const(n) => {
            formatter.fmt(*n);
        }
        Instruction::I64Const(n) => {
            formatter.fmt(*n);
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
        | Instruction::Br(index)
        | Instruction::BrIf(index) => {
            formatter.fmt(index);
        }
        Instruction::GlobalGet(index_or_ref) | Instruction::GlobalSet(index_or_ref) => {
            let item_ref = &index_or_ref.0;
            let index = item_ref.unwrap_index();
            formatter.fmt(index);
        }
        Instruction::Call(index_or_ref) => {
            let item_ref = &index_or_ref.0;
            let index = item_ref.unwrap_index();
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

        Instruction::I32Extend8S => "i32.extend8_s",
        Instruction::I32Extend16S => "i32.extend16_s",

        Instruction::I64Extend8S => "i64.extend8_s",
        Instruction::I64Extend16S => "i64.extend16_s",
        Instruction::I64Extend32S => "i64.extend32_s",

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
        Instruction::I32Load8s(..) => "i32.load8_s",
        Instruction::I32Load8u(..) => "i32.load8_u",
        Instruction::I32Load16s(..) => "i32.load16_s",
        Instruction::I32Load16u(..) => "i32.load16_u",
        Instruction::I64Load8s(..) => "i64.load8_s",
        Instruction::I64Load8u(..) => "i64.load8_u",
        Instruction::I64Load16s(..) => "i64.load16_s",
        Instruction::I64Load16u(..) => "i64.load16_u",
        Instruction::I64Load32s(..) => "i64.load32_s",
        Instruction::I64Load32u(..) => "i64.load32_u",
        Instruction::I32Store(..) => "i32.store",
        Instruction::I64Store(..) => "i64.store",
        Instruction::F32Store(..) => "f32.store",
        Instruction::F64Store(..) => "f64.store",
        Instruction::I32Store8(..) => "i32.store8",
        Instruction::I32Store16(..) => "i32.store16",
        Instruction::I64Store8(..) => "i64.store8",
        Instruction::I64Store16(..) => "i64.store16",
        Instruction::I64Store32(..) => "i64.store32",
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

impl<'src> Fmt for &BlockType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        if let Some(label) = &self.label {
            formatter.fmt(label);
            if !ty_use_is_empty(&self.ty) {
                formatter.write(" ");
            }
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

impl<'src> Fmt for &MemArg<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("offset=");
        formatter.fmt(self.offset);
        formatter.write(" align=");
        formatter.fmt(self.align);
    }
}

fn memarg_is_default(memarg: &MemArg, access_size: u32) -> bool {
    memarg.offset == 0 && memarg.align == access_size
}
