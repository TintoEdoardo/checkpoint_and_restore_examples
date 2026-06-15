(module $fibonacci

  (import "host" "print" (func $print (param i32)))
  (import "host" "checkpoint" (func $checkpoint (result i32)))
  (import "host" "resume" (func $resume (result i32)))
  (import "host" "restore_memory" (func $restore_mem))

  ;; Checkpoint organisation: 
  ;;  f_0 -> addr  0
  ;;  f_1 -> addr  4
  ;;  f_2 -> addr  8
  ;;  i   -> addr 12
  (memory $checkpoint_memory 1 1)

  (func $fibonacci (param $iter i32)
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

    ;; Resume procedure. 
    (block $to_block_end

      ;; Determine if we are resuming. 
      call $resume
      i32.eqz
      br_if $to_block_end

      ;; Restore the checkpoint memory. 
      call $restore_mem

      ;; Restore the values from the
      ;; checkpoint. 
      i32.const 0
      i32.load
      local.set $f_0
      i32.const 4
      i32.load
      local.set $f_1
      i32.const 8
      i32.load
      local.set $f_2
      i32.const 12
      i32.load
      local.set $i
    )

    (loop $loop_head

      ;; Checkpoint procedure. 
      (block $to_block_end

        ;; Determine if a checkpoint 
        ;; is required. 
        call $checkpoint
        i32.eqz 
        br_if $to_block_end

        ;; Build a checkpoint. 
        i32.const 0
        local.get $f_0
        i32.store
        i32.const 4
        local.get $f_1
        i32.store
        i32.const 8
        local.get $f_2
        i32.store
        i32.const 12
        local.get $i
        i32.store

        ;; Terminate here. 
        unreachable
      )
    
      ;; Body of the loop. 
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
  (export "memory" (memory $checkpoint_memory))
)