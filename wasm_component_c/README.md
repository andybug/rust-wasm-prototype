# compnent_c

The goal for this component is to consume an array from linear memory and call
an update function on the host.

```
cargo component build --release

wasm-tools component wit ./target/wasm32-wasi/release/wasm_component_c.wasm
```

You can see the generated bindings in `target/bindings`.
