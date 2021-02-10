(module
	(func (export "add") (param $x f32) (param $y f32) (result f32)
		(local.get $x)
		(local.get $y)
		f32.add
	)
	(func (export "sub") (param $x f32) (param $y f32) (result f32)
		(local.get $x)
		(local.get $y)
		f32.sub
	)
	(func (export "mul") (param $x f32) (param $y f32) (result f32)
		(local.get $x)
		(local.get $y)
		f32.mul
	)
	(func (export "div") (param $x f32) (param $y f32) (result f32)
		(local.get $x)
		(local.get $y)
		f32.div
	)
	(func (export "sqrt") (param $x f32) (result f32)
		(local.get $x)
		f32.sqrt
	)
	(func (export "min") (param $x f32) (param $y f32) (result f32)
		(local.get $x)
		(local.get $y)
		f32.min
	)
	(func (export "max") (param $x f32) (param $y f32) (result f32)
		(local.get $x)
		(local.get $y)
		f32.max
	)
	(func (export "ceil") (param $x f32) (result f32)
		(local.get $x)
		f32.ceil
	)
	(func (export "floor") (param $x f32) (result f32)
		(local.get $x)
		f32.floor
	)
	(func (export "trunc") (param $x f32) (result f32)
		(local.get $x)
		f32.trunc
	)
	(func (export "nearest") (param $x f32) (result f32)
		(local.get $x)
		f32.nearest
	)
)
