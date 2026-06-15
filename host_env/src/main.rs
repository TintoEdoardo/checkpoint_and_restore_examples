use wasmtime::anyhow::Result;
use std::env; 

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();


    // Initialisation of the WebAssemòy runtime. 

    println!("Initialisation of the runtime started...");

    let engine : wasmtime::Engine = wasmtime::Engine::default();
    struct ApplicationState {
        start_time: std::time::Instant,
    }
    let application_state = ApplicationState {
        start_time: std::time::Instant::now(),
    };

    let mut store: wasmtime::Store<ApplicationState> =
        wasmtime::Store::new(&engine, application_state);

    let module   : wasmtime::Module    =
        wasmtime::Module::from_file(store.engine(), &args[1])?;

    let print    : wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, |param: i32| -> () {
            // Suspend for a sec. 
            std::thread::sleep(std::time::Duration::new(1, 0));
            
            println!("new value is {} ", param);
        });
    let checkpoint: wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, move |caller: wasmtime::Caller<'_, ApplicationState>| -> i32 {
            let start_time = caller.data().start_time;
            let mut result : i32 = 0;
            if start_time.elapsed().as_secs() > 5 {
                result = 1;
            }
            return result;
        });
    
    let resume   : wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, || -> i32 {
            return 0;
        });
    
    let restore_mem   : wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, || -> () {
            // Nothing to do;
        });

    let instance : wasmtime::Instance  =
        wasmtime::Instance::new(&mut store, 
                                &module, 
                                &[print.into(), checkpoint.into(), resume.into(), restore_mem.into()])?;

    let wasm_import_function : wasmtime::TypedFunc<i32,()> =
        instance.get_typed_func::<i32,()>(&mut store, "main_function")?;

    println!("Initialisation of the runtime completed. ");

    // Function execution. 

    println!("The execution start. ");

    let _result : wasmtime::Result<()> =
        wasm_import_function.call(&mut store, 18);

    println!("The execution is over. ");

    let module_lin_mem : wasmtime::Memory =
        instance.get_memory(&mut store, "memory").expect("Failed to load memory");

    let mut checkpoint_vec : Vec<i32> = Vec::new();
    unsafe {
        checkpoint_vec.push(*(module_lin_mem.data_ptr(&store).wrapping_add(0)) as i32);
        checkpoint_vec.push(*(module_lin_mem.data_ptr(&store).wrapping_add(4)) as i32);
        checkpoint_vec.push(*(module_lin_mem.data_ptr(&store).wrapping_add(8)) as i32);
        checkpoint_vec.push(*(module_lin_mem.data_ptr(&store).wrapping_add(12)) as i32);
    }
    println!("The content of the checkpoint memory is {:?}", checkpoint_vec);

    // -----------------------
    // Resume the execution. 
    // -----------------------

    println!("Re-initialisation of the runtime started...");

    let engine : wasmtime::Engine = wasmtime::Engine::default();

    let application_state = ApplicationState {
        start_time: std::time::Instant::now(),
    };

    let mut store: wasmtime::Store<ApplicationState> =
        wasmtime::Store::new(&engine, application_state);

    let module   : wasmtime::Module    =
        wasmtime::Module::from_file(store.engine(), &args[1])?;

    let print    : wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, |param: i32| -> () {
            // Suspend for a sec. 
            std::thread::sleep(std::time::Duration::new(1, 0));
            
            println!("new value is {} ", param);
        });
    let checkpoint: wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, move |caller: wasmtime::Caller<'_, ApplicationState>| -> i32 {
            let start_time = caller.data().start_time;
            let mut result : i32 = 0;
            if start_time.elapsed().as_secs() > 5 {
                result = 1;
            }
            return result;
        });
    
    let resume   : wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, || -> i32 {
            return 1;
        });

    let memory_export = module.get_export_index("memory").unwrap();

    // The checkpoint vec should be read as an array of bytes. 
    let mut checkpoint_bytes : Vec<u8> = Vec::new();
    for i in 0..checkpoint_vec.len() 
    {
        let byte = checkpoint_vec[i].to_le_bytes();
        for j in 0..4 {
            checkpoint_bytes.push(byte[j]);
        }
    }

    let restore_mem: wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, move |mut caller: wasmtime::Caller<'_, ApplicationState>| -> () {
            
            let memory = match caller.get_module_export(&memory_export)
                {
                    Some (wasmtime::Extern::Memory(mem)) => mem,
                    _ => panic!("Failed to find host memory. "),
                };
            let memory_ptr = memory.data_ptr(&caller);

            for addr in 0..(checkpoint_bytes.len()) {
                unsafe {
                    *(memory_ptr.wrapping_add(addr)) = checkpoint_bytes[addr];
                }
            }
            
        });

    let instance : wasmtime::Instance  =
        wasmtime::Instance::new(&mut store, 
                                &module, 
                                &[print.into(), checkpoint.into(), resume.into(), restore_mem.into()])?;

    let wasm_import_function : wasmtime::TypedFunc<i32,()> =
        instance.get_typed_func::<i32,()>(&mut store, "main_function")?;

    println!("Initialisation of the runtime completed. ");

    // Function execution. 

    println!("The execution start. ");

    let _result : wasmtime::Result<()> =
        wasm_import_function.call(&mut store, 18);

    println!("The execution is over. ");

    Ok(())
}