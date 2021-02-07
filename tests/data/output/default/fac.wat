(module
	(func (export "fac-rec") (param i64) (result i64)
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
	(func $fac-rec-named (export "fac-rec-named") (param $n i64) (result i64)
		(local.get $n)
		(i64.const 0)
		i64.eq
		if (result i64)
			(i64.const 1)
		else
			(local.get $n)
			(local.get $n)
			(i64.const 1)
			i64.sub
			(call $fac-rec-named)
			i64.mul
		end
	)
	(func (export "fac-iter") (param i64) (result i64)
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
	(func (export "fac-iter-named") (param $n i64) (result i64)
		(local $i i64)
		(local $res i64)
		(local.get $n)
		(local.set $i)
		(i64.const 1)
		(local.set $res)
		block $done
			loop $loop
				(local.get $i)
				(i64.const 0)
				i64.eq
				if
					(br $done)
				else
					(local.get $i)
					(local.get $res)
					i64.mul
					(local.set $res)
					(local.get $i)
					(i64.const 1)
					i64.sub
					(local.set $i)
				end
				(br $loop)
			end
		end
		(local.get $res)
	)
	(func (export "fac-opt") (param i64) (result i64)
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
	(func $pick0 (param i64) (result i64 i64)
		(local.get 0)
		(local.get 0)
	)
	(func $pick1 (param i64 i64) (result i64 i64 i64)
		(local.get 0)
		(local.get 1)
		(local.get 0)
	)
	(func (export "fac-ssa") (param i64) (result i64)
		(i64.const 1)
		(local.get 0)
		loop $l (param i64 i64) (result i64)
			(call $pick1)
			(call $pick1)
			i64.mul
			(call $pick1)
			(i64.const 1)
			i64.sub
			(call $pick0)
			(i64.const 0)
			i64.gt_u
			(br_if $l)
			drop
			return
		end
	)
)
