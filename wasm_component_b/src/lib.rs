cargo_component_bindings::generate!();

use crate::bindings::exports::clegane::mapgen::map_generator::Guest;

struct Component;

impl Guest for Component {
    fn run(mut floats: Vec<f32>, val: f32) {
        for f in floats.iter_mut() {
            *f *= val;
        }

        floats[0] = 1234.0;

        for f in floats.iter() {
            println!("wasm {}", f);
        }
    }
}
