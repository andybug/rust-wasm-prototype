cargo_component_bindings::generate!();

use crate::bindings::clegane::mapgen::mapgen_api::update_tile;
use crate::bindings::exports::clegane::mapgen::controller::Guest;

use rand::{rngs::StdRng, Rng, SeedableRng};

struct Component;

impl Guest for Component {
    fn run(size: u64) {
        let mut map = vec![0.0f32; size.try_into().unwrap()].into_boxed_slice();

        let seed = [0u8; 32];
        let mut rng = StdRng::from_seed(seed);

        for i in 0..size {
            map[i as usize] = rng.gen::<f32>();
            update_tile(i, map[i as usize]);
            println!("wasm update-tile {} = {}", i, map[i as usize]);
        }

    }
}
