(module
	(func (export "eq") (param $x f64) (param $y f64) (result i32)
		(local.get $x)
		(local.get $y)
		f64.eq
	)
	(func (export "ne") (param $x f64) (param $y f64) (result i32)
		(local.get $x)
		(local.get $y)
		f64.ne
	)
	(func (export "lt") (param $x f64) (param $y f64) (result i32)
		(local.get $x)
		(local.get $y)
		f64.lt
	)
	(func (export "le") (param $x f64) (param $y f64) (result i32)
		(local.get $x)
		(local.get $y)
		f64.le
	)
	(func (export "gt") (param $x f64) (param $y f64) (result i32)
		(local.get $x)
		(local.get $y)
		f64.gt
	)
	(func (export "ge") (param $x f64) (param $y f64) (result i32)
		(local.get $x)
		(local.get $y)
		f64.ge
	)
)
