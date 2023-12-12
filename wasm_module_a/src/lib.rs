use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_data(array_size: usize) {
    let memory = wasm_bindgen::memory()
       .dyn_into::<js_sys::WebAssembly::Memory>()
       .unwrap()
       .buffer();

    let data = js_sys::Int32Array::new(&memory)
        .subarray(0, array_size.try_into().unwrap());

    for i in 0..data.length() {
        let value = data.get_index(i) + 1;
        data.set_index(i, value);
    }
}
