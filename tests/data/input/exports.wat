(module
    (func)
    (export "func_a" (func 0))
    (func (export "func_b"))
    (func (export "func_c") (export "func_d"))
    (func (export "func_e") (export "func_f") (param i32))
    (func $func_g (export "func_g"))
    (func $func_h)
    (export "func_h" (func $func_h))

    (global i32 (i32.const 0))
    (export "global_a" (global 0))
    (global (export "global_b") i32 (i32.const 0))
    (global $global_c (export "global_c") i32 (i32.const 0))
    (global $global_d i32 (i32.const 0))
    (export "global_d" (global $global_d))

    (table 0 funcref)
    (export "table_a" (table 0))
    (table (export "table_b") 0 funcref)
    (table (export "table_c") 0 1 funcref)
    (table $table_d (export "table_d") 0 funcref)
    (table $table_e (export "table_e") 0 1 funcref)
    (table $table_f 0 funcref)
    (export "table_f" (table $table_f))

    (memory 0)
    (export "memory_a" (memory 0))
    (memory (export "memory_b") 0)
    (memory (export "memory_c") 0 1)
    (memory $memory_d (export "memory_d") 0)
    (memory $memory_e (export "memory_e") 0 1)
    (memory $memory_f 0)
    (export "memory_f" (memory $memory_f))
)