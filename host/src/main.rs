use std::fs::File;
use std::io::prelude::*;

#[cfg(feature = "wasmer_runtime")]
mod host {
    use std::mem;

    use wasmer::{imports, Instance, Module, Store, Memory, MemoryType, Value};
    use rand::{Rng, SeedableRng, rngs::StdRng};

    const MATRIX_SIZE: usize = 16; // Adjust the size as needed

    fn i32_slice_as_u8_slice(input: &[i32]) -> &[u8] {
        let ptr = input.as_ptr() as *const u8;
        let len = input.len() * std::mem::size_of::<i32>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    pub fn run() {
        // Create and populate the first array with random numbers
        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for reproducibility
        let first_array: Vec<i32> = (0..MATRIX_SIZE).map(|_| rng.gen_range(0..1025)).collect();
        let byte_slice = i32_slice_as_u8_slice(&first_array);

        println!("initial data");
        for i in &first_array {
            println!("{}", i);
        }

        // Create a zeroed array of the same size
        let second_array: Vec<i32> = vec![0; MATRIX_SIZE];
        let second_array_slice = i32_slice_as_u8_slice(&second_array);

        // Initialize Wasmer store and memory
        let mut store = Store::default();
        // Ensure enough memory is allocated
        let required_pages = ((MATRIX_SIZE * mem::size_of::<i32>() * 2) as f64 / 65536.0).ceil() as u32;
        let memory = Memory::new(&mut store, MemoryType::new(required_pages, None, false)).expect("create memory");

        // Copy first_array into WebAssembly memory
        let memory_view = memory.view(&store);
        memory_view.write(0, byte_slice).expect("failed to write first_array");

        // Calculate the offset for the second array
        let second_array_offset = MATRIX_SIZE * mem::size_of::<i32>();
        memory_view.write(second_array_offset.try_into().unwrap(), second_array_slice).expect("failed to write second_array");

        // Load wasm_module_a
        let module_wasm = super::read_wasm_file("/home/andy/src/me/rust-wasm-prototype/target/wasm32-unknown-unknown/debug/bindgen/wasm_module_a_bg.wasm");
        let module = Module::new(&store, &module_wasm).expect("failed to create module");

        // Create import object, if needed
        let import_object = imports! {
            // Define any needed imports here, such as functions or memory
        };

        // Instantiate the module
        let instance = Instance::new(&mut store, &module, &import_object).expect("failed to instantiate wasm module");

        // Call process_data() function
        let process_data = instance.exports.get_function("process_data").unwrap();
        process_data.call(&mut store, &[Value::I64((MATRIX_SIZE * mem::size_of::<i32>()) as i64)]).expect("failed to execute process_data");

        println!("final data");
        for i in &first_array {
            println!("{}", i);
        }
    }

    //fn load_module(store: &Store, file_path: &str, memory: &Memory) -> Module {
    //    // Load the module and set up the environment with shared memory
    //    // ...
    //}
}

fn read_wasm_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("failed to open wasm file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("failed to read wasm file");
    buffer
}

fn main() {
    #[cfg(feature = "wasmer_runtime")]
    host::run();
}
