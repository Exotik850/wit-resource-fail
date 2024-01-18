To create the test.wasm file:

```bat
cd examples/test-plugin

cargo build --target wasm32-wasi

cd ../..

wasm-tools component new ./target/wasm32-wasi/debug/test_plugin.wasm --adapt ./wasi_snapshot_preview1.wasm -o test.wasm
```

Running the host
```
cd examples/test-host
cargo run
```