(module
	(func (export "load_at_zero") (result i32)
		(i32.const 0)
		i32.load
	)
	(func (export "store_at_zero")
		(i32.const 0)
		(i32.const 2)
		i32.store
	)
	(func (export "load_at_page_size") (result i32)
		(i32.const 65536)
		i32.load
	)
	(func (export "store_at_page_size")
		(i32.const 65536)
		(i32.const 3)
		i32.store
	)
	(func (export "grow") (param $sz i32) (result i32)
		(local.get $sz)
		memory.grow
	)
	(func (export "size") (result i32)
		memory.size
	)
)
