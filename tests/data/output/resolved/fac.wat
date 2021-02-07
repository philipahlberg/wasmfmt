(module
	(export "fac-rec" (func 0))
	(func (type 0) (param i64) (result i64)
		(local.get 0)
		(i64.const 0)
		i64.eq
		if (result i64)
			(i64.const 1)
		else
			(local.get 0)
			(local.get 0)
			(i64.const 1)
			i64.sub
			(call 0)
			i64.mul
		end
	)
	(export "fac-rec-named" (func 1))
	(func $fac-rec-named (type 0) (param $n i64) (result i64)
		(local.get 0)
		(i64.const 0)
		i64.eq
		if (result i64)
			(i64.const 1)
		else
			(local.get 0)
			(local.get 0)
			(i64.const 1)
			i64.sub
			(call 1)
			i64.mul
		end
	)
	(export "fac-iter" (func 2))
	(func (type 0) (param i64) (result i64)
		(local i64 i64)
		(local.get 0)
		(local.set 1)
		(i64.const 1)
		(local.set 2)
		block
			loop
				(local.get 1)
				(i64.const 0)
				i64.eq
				if
					(br 2)
				else
					(local.get 1)
					(local.get 2)
					i64.mul
					(local.set 2)
					(local.get 1)
					(i64.const 1)
					i64.sub
					(local.set 1)
				end
				(br 0)
			end
		end
		(local.get 2)
	)
	(export "fac-iter-named" (func 3))
	(func (type 0) (param $n i64) (result i64)
		(local $i i64)
		(local $res i64)
		(local.get 0)
		(local.set 1)
		(i64.const 1)
		(local.set 2)
		block $done
			loop $loop
				(local.get 1)
				(i64.const 0)
				i64.eq
				if
					(br 2)
				else
					(local.get 1)
					(local.get 2)
					i64.mul
					(local.set 2)
					(local.get 1)
					(i64.const 1)
					i64.sub
					(local.set 1)
				end
				(br 0)
			end
		end
		(local.get 2)
	)
	(export "fac-opt" (func 4))
	(func (type 0) (param i64) (result i64)
		(local i64)
		(i64.const 1)
		(local.set 1)
		block
			(local.get 0)
			(i64.const 2)
			i64.lt_s
			(br_if 0)
			loop
				(local.get 1)
				(local.get 0)
				i64.mul
				(local.set 1)
				(local.get 0)
				(i64.const -1)
				i64.add
				(local.set 0)
				(local.get 0)
				(i64.const 1)
				i64.gt_s
				(br_if 0)
			end
		end
		(local.get 1)
	)
	(func $pick0 (type 1) (param i64) (result i64 i64)
		(local.get 0)
		(local.get 0)
	)
	(func $pick1 (type 2) (param i64 i64) (result i64 i64 i64)
		(local.get 0)
		(local.get 1)
		(local.get 0)
	)
	(export "fac-ssa" (func 7))
	(func (type 0) (param i64) (result i64)
		(i64.const 1)
		(local.get 0)
		loop $l (type 3)
			(call 6)
			(call 6)
			i64.mul
			(call 6)
			(i64.const 1)
			i64.sub
			(call 5)
			(i64.const 0)
			i64.gt_u
			(br_if 0)
			drop
			return
		end
	)
	(type (func (param i64) (result i64)))
	(type (func (param i64) (result i64 i64)))
	(type (func (param i64 i64) (result i64 i64 i64)))
	(type (func (param i64 i64) (result i64)))
)
