(module
	(func (export "add") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.add
	)
	(func (export "sub") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.sub
	)
	(func (export "mul") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.mul
	)
	(func (export "div_s") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.div_s
	)
	(func (export "div_u") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.div_u
	)
	(func (export "rem_s") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.rem_s
	)
	(func (export "rem_u") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.rem_u
	)
	(func (export "and") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.and
	)
	(func (export "or") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.or
	)
	(func (export "xor") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.xor
	)
	(func (export "shl") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.shl
	)
	(func (export "shr_s") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.shr_s
	)
	(func (export "shr_u") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.shr_u
	)
	(func (export "rotl") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.rotl
	)
	(func (export "rotr") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.rotr
	)
	(func (export "clz") (param $x i32) (result i32)
		(local.get $x)
		i32.clz
	)
	(func (export "ctz") (param $x i32) (result i32)
		(local.get $x)
		i32.ctz
	)
	(func (export "popcnt") (param $x i32) (result i32)
		(local.get $x)
		i32.popcnt
	)
	(func (export "extend8_s") (param $x i32) (result i32)
		(local.get $x)
		i32.extend8_s
	)
	(func (export "extend16_s") (param $x i32) (result i32)
		(local.get $x)
		i32.extend16_s
	)
	(func (export "eqz") (param $x i32) (result i32)
		(local.get $x)
		i32.eqz
	)
	(func (export "eq") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.eq
	)
	(func (export "ne") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.ne
	)
	(func (export "lt_s") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.lt_s
	)
	(func (export "lt_u") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.lt_u
	)
	(func (export "le_s") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.le_s
	)
	(func (export "le_u") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.le_u
	)
	(func (export "gt_s") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.gt_s
	)
	(func (export "gt_u") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.gt_u
	)
	(func (export "ge_s") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.ge_s
	)
	(func (export "ge_u") (param $x i32) (param $y i32) (result i32)
		(local.get $x)
		(local.get $y)
		i32.ge_u
	)
)
