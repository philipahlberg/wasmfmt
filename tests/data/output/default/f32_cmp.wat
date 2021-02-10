(module
	(func (export "eq") (param $x f32) (param $y f32) (result i32)
		(local.get $x)
		(local.get $y)
		f32.eq
	)
	(func (export "ne") (param $x f32) (param $y f32) (result i32)
		(local.get $x)
		(local.get $y)
		f32.ne
	)
	(func (export "lt") (param $x f32) (param $y f32) (result i32)
		(local.get $x)
		(local.get $y)
		f32.lt
	)
	(func (export "le") (param $x f32) (param $y f32) (result i32)
		(local.get $x)
		(local.get $y)
		f32.le
	)
	(func (export "gt") (param $x f32) (param $y f32) (result i32)
		(local.get $x)
		(local.get $y)
		f32.gt
	)
	(func (export "ge") (param $x f32) (param $y f32) (result i32)
		(local.get $x)
		(local.get $y)
		f32.ge
	)
)
