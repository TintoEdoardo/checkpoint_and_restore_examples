(module $fibonacci

  (import "host" "print" (func $print (param i32))

  (func $fibonacci (param $iter)
    (local $f_0 i32)
    (local $f_1 i32)
    (local $f_2 i32)
    
    (local $i i32)

    i32.const 0
    local.set $f_0

    i32.const 1
    local.set $f_1

    i32.const 0
    local.set $i

    (loop $loop_head
    
      local.get $f_0
      local.get $f_1
      i32.add
      local.set $f_2

      local.get $f_1
      local.set $f_0

      local.get $f_2
      local.set $f_1

      local.get $f_2
      call $print
      
      ;; Update the iteration number. 
      i32.const 1
      local.get $i
      i32.add
      local.set $i

      ;; Decide whether to continue or break. 
      local.get $i
      local.get $iter
      i32.lt_s
      br_if $loop_head
    
    )
  )

  (export "main_function" (func $fibonacci))
)