(module
	(export "fac" (func 0))
	(func (param i32) (result i32)
		(local i32)
		(i32.const 1)
		(local.set 1)
		block
			(local.get 0)
			(i32.const 2)
			i32.lt_s
			(br_if 0)
			loop
				(local.get 1)
				(local.get 0)
				i32.mul
				(local.set 1)
				(local.get 0)
				(i32.const -1)
				i32.add
				(local.set 0)
				(local.get 0)
				(i32.const 1)
				i32.gt_s
				(br_if 0)
			end
		end
		(local.get 1)
	)
)
