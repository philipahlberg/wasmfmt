use super::Formatter;
use wast::{
    BlockType, Expression, FunctionType, Id, Index, InlineExport, InlineImport, Instruction,
    MemoryArg, TypeUse,
};

pub fn expr_is_const(expression: &Expression) -> bool {
    expression.instrs.len() == 1 && instr_is_const(&expression.instrs[0])
}

pub fn instr_is_const(instruction: &Instruction) -> bool {
    matches!(instruction, Instruction::I32Const(..))
}

pub fn index_is_default(index: &Index) -> bool {
    matches!(index, Index::Num(0, ..))
}

pub fn inline_import_is_empty(import: &InlineImport) -> bool {
    import.field.is_none()
}

pub fn inline_export_is_empty(export: &InlineExport) -> bool {
    export.names.is_empty()
}

pub fn func_ty_is_empty(func_ty: &FunctionType) -> bool {
    func_ty.params.is_empty() && func_ty.results.is_empty()
}

pub fn ty_use_is_empty<'a>(ty_use: &TypeUse<'a, FunctionType<'a>>) -> bool {
    ty_use.index.is_none()
        && ty_use
            .inline
            .as_ref()
            .map(|ty| func_ty_is_empty(&ty))
            .unwrap_or(true)
}

pub fn id_is_gensym(id: &Id) -> bool {
    id.name() == "gensym"
}

pub fn fmt_long_expression<'src>(expression: &Expression<'src>, formatter: &mut Formatter) {
    for instruction in expression.instrs.iter() {
        if instr_is_block_end(instruction) {
            formatter.deindent();
        }
        formatter.start_line();
        formatter.fmt(instruction);
        formatter.end_line();
        if instr_is_block_start(instruction) {
            formatter.indent();
        }
    }
}

pub fn instr_is_block_end(instruction: &Instruction) -> bool {
    matches!(instruction, Instruction::Else(..) | Instruction::End(..))
}

pub fn instr_is_block_start(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::Block(..)
            | Instruction::If(..)
            | Instruction::Loop(..)
            | Instruction::Else(..),
    )
}

pub fn memory_arg_is_valid(memory_arg: &MemoryArg) -> bool {
    memory_index_is_valid(&memory_arg.mem.unwrap_index())
}

pub fn memory_index_is_valid(index: &Index) -> bool {
    matches!(index, Index::Num(0, ..))
}

pub fn bt_is_empty(block_type: &BlockType) -> bool {
    block_type.label.is_none() && ty_use_is_empty(&block_type.ty)
}

pub fn to_byte_string(slice: &[u8]) -> String {
    let mut string = String::new();
    for &byte in slice {
        string.push('\\');
        string.push(to_hex_char(high_four_bits(byte)));
        string.push(to_hex_char(low_four_bits(byte)));
    }
    string
}

fn to_hex_char(v: u8) -> char {
    std::char::from_digit(v as u32, 16).unwrap()
}

const fn high_four_bits(byte: u8) -> u8 {
    (byte & 0xF0) >> 4
}

const fn low_four_bits(byte: u8) -> u8 {
    byte & 0xF
}
