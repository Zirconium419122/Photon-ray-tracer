use wasm_bindgen::prelude::*;

use crate::{init_panic_hook, vector::Vector};

// Rust Ray struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

#[wasm_bindgen]
impl Ray {
    #[wasm_bindgen(constructor)]
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        init_panic_hook();

        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector {
        self.origin + (self.direction * t)
    }

    pub fn get_background_color(&self) -> Vector {
        let t = 0.5 * (self.direction.y + 1.0);

        let white = Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let blue = Vector {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };

        white * (1.0 - t) + (blue * t)
    }
}
