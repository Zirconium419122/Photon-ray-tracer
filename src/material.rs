use wasm_bindgen::prelude::*;

use crate::{init_panic_hook, vector::Vector};

// Rust Material struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Material {
    pub color: Vector,  // RGB color/albedo of the material
    pub roughness: f64, // Reflection coefficient between 0 and 1, roughness zero means just reflections
    pub emission_color: Vector,
    pub emission_power: f64,
    // pub metallic: f64,       // Defines the splecularness of the Material
}

#[wasm_bindgen]
impl Material {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: Vector,
        roughness: f64,
        emission_color: Vector,
        emission_power: f64,
    ) -> Material {
        init_panic_hook();

        Material {
            color,
            roughness,
            emission_color,
            emission_power,
        }
    }
}
