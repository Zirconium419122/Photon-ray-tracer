use wasm_bindgen::prelude::*;

// Rust Vector struct
#[wasm_bindgen]
#[repr(C)]
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

    // Metod to multiply with a scalar
    pub fn multiply(&self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
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
        let pool = Vec::with_capacity(capacity);
        Self { pool }
    }

    // Get a Vector from the pool by index, or allocate a new one if the pool is empty
    pub fn get(&mut self, index: usize) -> Vector {
        if index < self.pool.len() {
            self.pool[index]
        } else {
            // You might want to handle the case when the pool is empty differently
            Vector { x: 0.0, y: 0.0, z: 0.0 }
        }
    }

    // Set a specific index to a Vector to update the values of a specific Vector in the pool
    pub fn set(&mut self, index: usize, values: Vector) {
        if index < self.pool.len() {
            self.pool[index] = values;
        } else {
            // Optionally we could handle an out-of-bounds index
            println!("Index out of bounds: {}", index);
        }
    }

    // Set a specific index to some new values to update the values of a specific Vector in the pool
    pub fn set_values(&mut self, index: usize, x: f64, y: f64, z: f64) {
        if index < self.pool.len() {
            let vector = &mut self.pool[index];
            vector.x = x;
            vector.y = y;
            vector.z = z;
        } else {
            // Optionally we could handle an out-of-bounds index
            println!("Index out of bounds: {}", index);
        }
    }

    // // Return a Vector to the pool by pushing it back
    // pub fn return_to_pool(&mut self, vector: Vector) {
    //     self.pool.push(vector);
    // }
}