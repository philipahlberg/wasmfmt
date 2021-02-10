(module
	(func (export "add") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.add
	)
	(func (export "sub") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.sub
	)
	(func (export "mul") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.mul
	)
	(func (export "div_s") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.div_s
	)
	(func (export "div_u") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.div_u
	)
	(func (export "rem_s") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.rem_s
	)
	(func (export "rem_u") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.rem_u
	)
	(func (export "and") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.and
	)
	(func (export "or") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.or
	)
	(func (export "xor") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.xor
	)
	(func (export "shl") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.shl
	)
	(func (export "shr_s") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.shr_s
	)
	(func (export "shr_u") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.shr_u
	)
	(func (export "rotl") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.rotl
	)
	(func (export "rotr") (param $x i64) (param $y i64) (result i64)
		(local.get $x)
		(local.get $y)
		i64.rotr
	)
	(func (export "clz") (param $x i64) (result i64)
		(local.get $x)
		i64.clz
	)
	(func (export "ctz") (param $x i64) (result i64)
		(local.get $x)
		i64.ctz
	)
	(func (export "popcnt") (param $x i64) (result i64)
		(local.get $x)
		i64.popcnt
	)
	(func (export "extend8_s") (param $x i64) (result i64)
		(local.get $x)
		i64.extend8_s
	)
	(func (export "extend16_s") (param $x i64) (result i64)
		(local.get $x)
		i64.extend16_s
	)
	(func (export "extend32_s") (param $x i64) (result i64)
		(local.get $x)
		i64.extend32_s
	)
	(func (export "eqz") (param $x i64) (result i32)
		(local.get $x)
		i64.eqz
	)
	(func (export "eq") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.eq
	)
	(func (export "ne") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.ne
	)
	(func (export "lt_s") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.lt_s
	)
	(func (export "lt_u") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.lt_u
	)
	(func (export "le_s") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.le_s
	)
	(func (export "le_u") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.le_u
	)
	(func (export "gt_s") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.gt_s
	)
	(func (export "gt_u") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.gt_u
	)
	(func (export "ge_s") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.ge_s
	)
	(func (export "ge_u") (param $x i64) (param $y i64) (result i32)
		(local.get $x)
		(local.get $y)
		i64.ge_u
	)
)
