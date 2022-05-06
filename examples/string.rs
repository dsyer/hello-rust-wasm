extern crate wasmtime;

use std::error;

use wasmtime::{Caller, Engine, Extern, Func, Instance, Module, Store};

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] =
    include_bytes!("../wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm");

fn main() -> Result<(), Box<dyn error::Error>> {

    let print_str2 = |_: u32, _: u32| {};
    let increment_shared = || {};

    let engine = Engine::default();
    let module = Module::new(&engine, WASM)?;
    let mut store = Store::new(&engine, {});
    let import_object = [
        Func::wrap(&mut store, print_str).into(),
        Func::wrap(&mut store, print_str2).into(),
        Func::wrap(&mut store, increment_shared).into(),
    ];

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.get_memory(&mut store, "memory").unwrap();

    let host_string = "from Rust!";

    // Write the string into the lineary memory
    for (cell, byte) in host_string.bytes().enumerate() {
        memory.data_mut(&mut store)[cell] = byte;
    }

    // Call our exported function!
    instance
        .get_typed_func(&mut store, "hello_string_from_rust")?
        .call(&mut store, (0, host_string.len() as i32))?;

    Ok(())
}

fn print_str(mut caller: Caller<'_, ()>, ptr: u32, len: u32) {
    let memory = match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => mem,
        _ => return,
    };
    let string: String = memory.data(&mut caller)[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(|cell| *cell as char)
        .collect();

    println!("{}", string);
}
