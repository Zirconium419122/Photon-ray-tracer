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

    // Method to add another vector
    #[wasm_bindgen]
    pub fn add(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    // Method to subtract another vector
    #[wasm_bindgen]
    pub fn subtract(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }

    // Metod to multiply with a scalar
    #[wasm_bindgen]
    pub fn multiply(&self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    // Method to divide by a scalar
    #[wasm_bindgen]
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
    #[wasm_bindgen]
    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    // Method to calculate the cross product with another vector
    #[wasm_bindgen]
    pub fn cross(&self, v: &Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    // Method to calculate the magnitude of the vector
    #[wasm_bindgen]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Method to normalize the vector (make it a unit vector)
    #[wasm_bindgen]
    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}