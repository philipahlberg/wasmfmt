(module
	(export "abs" (func 0))
	(func (type 0) (param $x f32) (result f32)
		(local.get 0)
		f32.abs
	)
	(export "neg" (func 1))
	(func (type 0) (param $x f32) (result f32)
		(local.get 0)
		f32.neg
	)
	(export "copysign" (func 2))
	(func (type 1) (param $x f32) (param $y f32) (result f32)
		(local.get 0)
		(local.get 1)
		f32.copysign
	)
	(type (func (param f32) (result f32)))
	(type (func (param f32 f32) (result f32)))
)
