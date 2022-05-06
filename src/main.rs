extern crate wasmtime;

use std::sync::{Arc, Mutex};
use std::error;
use wasmtime::{Engine, Instance, Module, Store, Func, Caller, Extern};

static WASM: &'static [u8] =
    include_bytes!("../wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm");

fn main() -> Result<(), Box<dyn error::Error>> {
    let shared_data = Arc::new(Mutex::new(0usize));

    let data = Arc::clone(&shared_data);
    let print_str = |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
        let memory = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => return,
        };
        let string: String = memory.data(&mut caller)[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(|cell| *cell as char)
        .collect();
        println!("{}", string);
    };
    let print_str2 = move |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
        let memory = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => return,
        };
        let string: String = memory.data(&mut caller)[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(|cell| *cell as char)
        .collect();

        let guard = data.lock().unwrap();
        println!("{}: {}", guard, string);
    };

    let data = Arc::clone(&shared_data);
    let increment_shared = move || {
        let mut guard = data.lock().unwrap();
        *guard += 1;
    };
    let engine = Engine::default();
    let module = Module::new(&engine, WASM)?;
    let mut store = Store::new(&engine, {});
    let import_object = [Func::wrap(&mut store, print_str).into(),
            Func::wrap(&mut store, print_str2).into(),
            Func::wrap(&mut store, increment_shared).into(),
    ];

    let instance = Instance::new(&mut store, &module, &import_object)?;

    instance.get_typed_func(&mut store, "hello_wasm")?.call(&mut store, ())?;

    Ok(())
}

