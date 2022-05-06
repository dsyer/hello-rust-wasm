# Rust Embedder App Example

This repo showcases how to use [wasmtime](https://crates.io/crates/wasmtime/) from Rust.

See [`src/main.rs`](./src/main.rs) for the example implementation.

The `wasm-sample-app` directory contains an example rust wasm app to run in the embedder app.

## Requirements
- Rust target `wasm32-unknown-unknown` - install using `rustup target add wasm32-unknown-unknown`

## Running

Building the `wasm-sample-app`:

```bash
cd wasm-sample-app && cargo build --release && cd ..
```

Running the wasm sample from Rust

```bash
cargo run
```

or 

```bash
cargo run --example string
```
