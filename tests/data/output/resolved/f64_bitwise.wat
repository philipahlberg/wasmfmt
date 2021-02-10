(module
	(export "abs" (func 0))
	(func (type 0) (param $x f64) (result f64)
		(local.get 0)
		f64.abs
	)
	(export "neg" (func 1))
	(func (type 0) (param $x f64) (result f64)
		(local.get 0)
		f64.neg
	)
	(export "copysign" (func 2))
	(func (type 1) (param $x f64) (param $y f64) (result f64)
		(local.get 0)
		(local.get 1)
		f64.copysign
	)
	(type (func (param f64) (result f64)))
	(type (func (param f64 f64) (result f64)))
)
