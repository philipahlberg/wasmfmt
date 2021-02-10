(module
	(func (export "abs") (param $x f64) (result f64)
		(local.get $x)
		f64.abs
	)
	(func (export "neg") (param $x f64) (result f64)
		(local.get $x)
		f64.neg
	)
	(func (export "copysign") (param $x f64) (param $y f64) (result f64)
		(local.get $x)
		(local.get $y)
		f64.copysign
	)
)
