(module
	(func (export "fac") (param $n i32) (result i32)
		(local $r i32)
		(local.set $r (i32.const 1))
		(block
			(br_if 0 (i32.lt_s (local.get 0) (i32.const 2)))
			(loop
				(local.set $r (i32.mul (local.get $r) (local.get $n)))
				(local.set $n (i32.add (local.get $n) (i32.const -1)))
				(br_if 0 (i32.gt_s (local.get $n) (i32.const 1)))))
		(local.get $r)
	)
)
