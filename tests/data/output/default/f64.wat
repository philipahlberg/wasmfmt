(module
	(func (export "add") (param $x f64) (param $y f64) (result f64)
		(local.get $x)
		(local.get $y)
		f64.add
	)
	(func (export "sub") (param $x f64) (param $y f64) (result f64)
		(local.get $x)
		(local.get $y)
		f64.sub
	)
	(func (export "mul") (param $x f64) (param $y f64) (result f64)
		(local.get $x)
		(local.get $y)
		f64.mul
	)
	(func (export "div") (param $x f64) (param $y f64) (result f64)
		(local.get $x)
		(local.get $y)
		f64.div
	)
	(func (export "sqrt") (param $x f64) (result f64)
		(local.get $x)
		f64.sqrt
	)
	(func (export "min") (param $x f64) (param $y f64) (result f64)
		(local.get $x)
		(local.get $y)
		f64.min
	)
	(func (export "max") (param $x f64) (param $y f64) (result f64)
		(local.get $x)
		(local.get $y)
		f64.max
	)
	(func (export "ceil") (param $x f64) (result f64)
		(local.get $x)
		f64.ceil
	)
	(func (export "floor") (param $x f64) (result f64)
		(local.get $x)
		f64.floor
	)
	(func (export "trunc") (param $x f64) (result f64)
		(local.get $x)
		f64.trunc
	)
	(func (export "nearest") (param $x f64) (result f64)
		(local.get $x)
		f64.nearest
	)
)
