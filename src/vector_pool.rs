// VectorPool.rs
use core::panic;

use wasm_bindgen::prelude::*;

use crate::vector::Vector; // Import Vector from the Vector.rs file

use super::init_panic_hook;


// Rust VectorPool struct
#[wasm_bindgen]
pub struct VectorPool {
    pool: Vec<Vector>,
}

// Implement the methods for the VectorPool struct
#[wasm_bindgen]
impl VectorPool {
    // Create a new VectorPool with a specified capacity
    #[inline]
    #[wasm_bindgen(constructor)]
    pub fn new(capacity: usize) -> Self {
        init_panic_hook();

        let mut pool = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            // Initialize the pool with default Vector instances
            pool.push(Vector { x: 0.0, y: 0.0, z: 0.0 });
        }

        Self { pool }
    }

    // Get a Vector from the pool by index, or allocate a new one if the pool is empty
    #[inline]
    pub fn get(&self, index: usize) -> Vector {
        if self.is_valid_index(index) {
            self.pool[index]
        } else {
            // Optionally you could handle an out-of-bounds index
            panic!("Index out of bounds: {}", index);

        }
    }

    // Set a specific index to a Vector to update the values of a specific Vector in the pool
    #[inline]
    pub fn set(&mut self, index: usize, values: Vector) {
        if self.is_valid_index(index) {
            self.pool[index] = values;
        } else {
            // Optionally you could handle an out-of-bounds index
            panic!("Index out of bounds: {}", index);
        }
    }

    // Set a specific index to some new values to update the values of a specific Vector in the pool
    #[inline]
    pub fn set_values(&mut self, index: usize, x: f64, y: f64, z: f64) {
        if self.is_valid_index(index) {
            self.pool[index] = Vector { x, y, z };
        } else {
            // Optionally you could handle an out-of-bounds index
            panic!("Index out of bounds: {}", index);
        }
    }

    #[inline]
    pub fn is_valid_index(&self, index: usize) -> bool {
        !self.pool.is_empty() && index < self.pool.len()
    }
}