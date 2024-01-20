# Steganography With Wasm

## Build Commands:

### Building the rust code to webassembly module(wasm)

```
cargo build --target wasm32-wasi
```

### Running the Steganography Function

```
wasmtime --dir=. mank-wasi-rust.wasm steg <image path> <text> <output path>
```

### Running the Desteganography Function

```
wasmtime --dir=. mank-wasi-rust.wasm unsteg <image path>
```
