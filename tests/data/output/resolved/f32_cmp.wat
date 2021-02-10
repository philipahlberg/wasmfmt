(module
	(export "eq" (func 0))
	(func (type 0) (param $x f32) (param $y f32) (result i32)
		(local.get 0)
		(local.get 1)
		f32.eq
	)
	(export "ne" (func 1))
	(func (type 0) (param $x f32) (param $y f32) (result i32)
		(local.get 0)
		(local.get 1)
		f32.ne
	)
	(export "lt" (func 2))
	(func (type 0) (param $x f32) (param $y f32) (result i32)
		(local.get 0)
		(local.get 1)
		f32.lt
	)
	(export "le" (func 3))
	(func (type 0) (param $x f32) (param $y f32) (result i32)
		(local.get 0)
		(local.get 1)
		f32.le
	)
	(export "gt" (func 4))
	(func (type 0) (param $x f32) (param $y f32) (result i32)
		(local.get 0)
		(local.get 1)
		f32.gt
	)
	(export "ge" (func 5))
	(func (type 0) (param $x f32) (param $y f32) (result i32)
		(local.get 0)
		(local.get 1)
		f32.ge
	)
	(type (func (param f32 f32) (result i32)))
)
