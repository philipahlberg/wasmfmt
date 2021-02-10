(module
	(export "eq" (func 0))
	(func (type 0) (param $x f64) (param $y f64) (result i32)
		(local.get 0)
		(local.get 1)
		f64.eq
	)
	(export "ne" (func 1))
	(func (type 0) (param $x f64) (param $y f64) (result i32)
		(local.get 0)
		(local.get 1)
		f64.ne
	)
	(export "lt" (func 2))
	(func (type 0) (param $x f64) (param $y f64) (result i32)
		(local.get 0)
		(local.get 1)
		f64.lt
	)
	(export "le" (func 3))
	(func (type 0) (param $x f64) (param $y f64) (result i32)
		(local.get 0)
		(local.get 1)
		f64.le
	)
	(export "gt" (func 4))
	(func (type 0) (param $x f64) (param $y f64) (result i32)
		(local.get 0)
		(local.get 1)
		f64.gt
	)
	(export "ge" (func 5))
	(func (type 0) (param $x f64) (param $y f64) (result i32)
		(local.get 0)
		(local.get 1)
		f64.ge
	)
	(type (func (param f64 f64) (result i32)))
)
