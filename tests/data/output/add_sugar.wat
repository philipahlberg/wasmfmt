(module
	(export "add" (func 0))
	(func $gensym (type 0) (param i32 i32) (result i32)
		(local.get 0)
		(local.get 1)
		i32.add
	)
	(type $gensym (func (param i32 i32) (result i32)))
)
