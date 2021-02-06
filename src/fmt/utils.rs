use super::Formatter;
use wast::{
    BlockType, Expression, FuncKind, FunctionType, Id, Index, InlineImport, Instruction, MemoryArg,
    TypeUse,
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

pub fn func_ty_is_empty(func_ty: &FunctionType) -> bool {
    func_ty.params.is_empty() && func_ty.results.is_empty()
}

pub fn ty_use_is_empty<'a>(ty_use: &TypeUse<'a, FunctionType<'a>>) -> bool {
    ty_use.index.is_none()
        && ty_use
            .inline
            .as_ref()
            .map(|ty| func_ty_is_empty(&ty))
            .unwrap_or(false)
}

pub fn func_kind_is_empty(kind: &FuncKind) -> bool {
    match kind {
        FuncKind::Import(import) => inline_import_is_empty(import),
        FuncKind::Inline { locals, expression } => {
            locals.is_empty() && expression.instrs.is_empty()
        }
    }
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
