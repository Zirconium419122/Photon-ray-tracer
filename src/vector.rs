use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

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

// Method to add another vector
impl Add for Vector {
    type Output = Self;

    fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

// Method to add another vector and assign it
impl AddAssign for Vector {
    fn add_assign(&mut self, v: Self) {
        *self = Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

// Method to subtract another vector
impl Sub for Vector {
    type Output = Self;

    fn sub(self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

// Method to subtract another vector and assign it
impl SubAssign for Vector {
    fn sub_assign(&mut self, v: Self) {
        *self = Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

// Method to multiply with a scalar
impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Method to multiply with a Vector
impl Mul for Vector {
    type Output = Self;

    fn mul(self, v: Self) -> Self {
        Self {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

// Method to multiply another vector and assign it
impl MulAssign for Vector {
    fn mul_assign(&mut self, v: Self) {
        *self = Self {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

// Method to divide by a scalar
impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        // Check for division by zero to avoid errors
        if scalar != 0.0 {
            Self {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
            }
        } else {
            // Handle division by zero gracefully
            panic!("Division by zero!");
        }
    }
}
