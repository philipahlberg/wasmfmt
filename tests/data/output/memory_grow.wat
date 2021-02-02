(module
	(export "load_at_zero" (func 0))
	(func (type 0) (result i32)
		(i32.const 0)
		i32.load
	)
	(export "store_at_zero" (func 1))
	(func (type 1)
		(i32.const 0)
		(i32.const 2)
		i32.store
	)
	(export "load_at_page_size" (func 2))
	(func (type 0) (result i32)
		(i32.const 65536)
		i32.load
	)
	(export "store_at_page_size" (func 3))
	(func (type 1)
		(i32.const 65536)
		(i32.const 3)
		i32.store
	)
	(export "grow" (func 4))
	(func (type 2) (param i32) (result i32)
		(local.get 0)
		memory.grow
	)
	(export "size" (func 5))
	(func (type 0) (result i32)
		memory.size
	)
	(type (func (result i32)))
	(type (func))
	(type (func (param i32) (result i32)))
)
