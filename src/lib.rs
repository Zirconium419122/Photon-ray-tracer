extern crate console_error_panic_hook;

use core::panic;

use wasm_bindgen::prelude::*;

// Rust Vector struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Implementation of methods for the Vector struct
#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    // Method to set a vector to specific values
    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    // Method to add another vector
    pub fn add(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    // Method to subtract another vector
    pub fn subtract(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    // Method to multiply with a scalar
    pub fn multiply(&self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    // Method to multiply with a Vector
    pub fn multiply_elementwise(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }

    // Method to divide by a scalar
    pub fn divide(&self, scalar: f64) -> Vector {
        // Check for division by zero to avoid errors
        if scalar != 0.0 {
            Vector {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
            }
        } else {
            // Handle division by zero gracefully
            panic!("Division by zero!");
        }
    }

    // Method to calculate the dot product with another vector
    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    // Method to calculate the cross product with another vector
    pub fn cross(&self, v: &Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    // Method to calculate the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Method to normalize the vector (make it a unit vector)
    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

// Rust VectorPool struct
#[wasm_bindgen]
pub struct VectorPool {
    pool: Vec<Vector>,
}

// Implement the methods for the VectorPool struct
#[wasm_bindgen]
impl VectorPool {
    // Create a new VectorPool with a specified capacity
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
    pub fn get(&self, index: usize) -> Vector {
        if !self.pool.is_empty() && index < self.pool.len() {
            self.pool[index]
        } else {
            // Optionally you could handle an out-of-bounds index
            panic!("Index out of bounds: {}", index);

        }
    }

    // Set a specific index to a Vector to update the values of a specific Vector in the pool
    pub fn set(&mut self, index: usize, values: Vector) {
        if !self.pool.is_empty() && index < self.pool.len() {
            self.pool[index] = values;
        } else {
            // Optionally you could handle an out-of-bounds index
            panic!("Index out of bounds: {}", index);
        }
    }

    // Set a specific index to some new values to update the values of a specific Vector in the pool
    pub fn set_values(&mut self, index: usize, x: f64, y: f64, z: f64) {
        if !self.pool.is_empty() && index < self.pool.len() {
            self.pool[index].x = x;
            self.pool[index].y = y;
            self.pool[index].z = z;
        } else {
            // Optionally you could handle an out-of-bounds index
            panic!("Index out of bounds: {}", index);
        }
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}