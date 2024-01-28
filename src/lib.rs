extern crate console_error_panic_hook;

use core::panic;

use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn console_log(s: &str) {
    log(s);
}

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
        init_panic_hook();

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

// Rust Intersection struct
#[derive(Debug, Copy, Clone)]
pub struct Intersection<T> {
    t: f64,
    intersection_point: Vector,
    intersection_object: T,
}

// Rust Material struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Material {
    color: Vector,  // RGB color/albedo of the material
    roughness: f64, // Reflection coefficient between 0 and 1, roughness zero means no reflections
    emission_color: Vector,
    emission_power: f64,
    // metallic: f64,       // Defines the splecularness of the Material
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
        Material {
            color,
            roughness,
            emission_color,
            emission_power,
        }
    }
}

// Rust Ray struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
}

#[wasm_bindgen]
impl Ray {
    #[wasm_bindgen(constructor)]
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector {
        self.origin.add(&self.direction.multiply(t))
    }
}

// Rust Sphere struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Material,
}

#[wasm_bindgen]
impl Sphere {
    #[wasm_bindgen(constructor)]
    pub fn new(center: Vector, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection<Sphere>> {
        let oc = ray.origin.subtract(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction) * 2.0;
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            // Ray intersects the sphere, calculate intersection point
            let t1 = -b - discriminant.sqrt() / (2.0 * a);
            let t2 = -b + discriminant.sqrt() / (2.0 * a);

            // Return the smaller positive intersection point
            let t = t1.min(t2);

            if t > 0.0 {
                let intersection_point = ray.point_at_parameter(t);
                Some(Intersection {
                    t,
                    intersection_point,
                    intersection_object: *self,
                })
            } else {
                // Ray does not intersect the sphere
                None
            }
        } else {
            // Ray does not intersect the sphere
            None
        }
    }

    fn calculate_normal(&self, point: &Vector) -> Vector {
        point.subtract(&self.center).normalize()
    }
}

// Rust Cube struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Cube {
    center: Vector,
    size: Vector,
    material: Material,
}

#[wasm_bindgen]
impl Cube {
    #[wasm_bindgen(constructor)]
    pub fn new(center: Vector, size: Vector, material: Material) -> Cube {
        Cube {
            center,
            size,
            material,
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection<Cube>> {
        let half_size = self.size.multiply(0.5);

        // Calculate the minimum and maximum extents along each axis
        let min_x = self.center.x - half_size.x;
        let min_y = self.center.y - half_size.y;
        let min_z = self.center.z - half_size.z;

        let max_x = self.center.x + half_size.x;
        let max_y = self.center.y + half_size.y;
        let max_z = self.center.z + half_size.z;

        // Calculate the intersection distances along each axis
        let t_min_x = (min_x - ray.origin.x) / ray.direction.x;
        let t_max_x = (max_x - ray.origin.x) / ray.direction.x;

        let t_min_y = (min_y - ray.origin.y) / ray.direction.y;
        let t_max_y = (max_y - ray.origin.y) / ray.direction.y;

        let t_min_z = (min_z - ray.origin.z) / ray.direction.z;
        let t_max_z = (max_z - ray.origin.z) / ray.direction.z;

        // Find the intersection intervals along each axis
        let t_min = t_min_x
            .max(t_max_x)
            .max(t_min_y.min(t_max_y))
            .max(t_min_z.min(t_max_z));
        let t_max = t_min_x
            .min(t_max_x)
            .min(t_min_y.min(t_max_y))
            .min(t_min_z.min(t_max_z));

        // Check if there is a valid intersection
        if t_min <= t_max && t_max > 0.0 {
            // Return the intersection point at the minmum distance
            let intersection_point = ray.point_at_parameter(t_min);
            Some(Intersection {
                t: t_min,
                intersection_point,
                intersection_object: *self,
            })
        } else {
            // Ray does not intersect with the cube
            None
        }
    }

    fn calculate_normal(&self, point: &Vector) -> Vector {
        // Calculate the differences between the point's coordinates and the cube's center
        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;
        let dz = point.z - self.center.z;

        // Identify the face closest to the point and assign the normal accordingly
        if dx.abs() > dy.abs() && dx.abs() > dz.abs() {
            // Point is on the face with the largest x-coordinate differance
            return Vector {
                x: dx.signum(),
                y: 0.0,
                z: 0.0,
            };
        } else if dy.abs() > dz.abs() {
            // Point is on the face with the largest y-coordinate differance
            return Vector {
                x: 0.0,
                y: dy.signum(),
                z: 0.0,
            };
        } else {
            // Point is on the face with the largest z-coordinate differance
            return Vector {
                x: 0.0,
                y: 0.0,
                z: dz.signum(),
            };
        }
    }
}

// Rust Scene struct
#[wasm_bindgen]
#[derive(Debug)]
pub struct Scene {
    spheres: Vec<Sphere>,
    cubes: Vec<Cube>,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene {
        Scene {
            spheres: Vec::new(),
            cubes: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
    }
}

// Rust Renderer struct
#[wasm_bindgen]
#[derive(Debug)]
pub struct Renderer {
    canvas: HtmlCanvasElement,
    scene: Scene,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, scene: Scene) -> Renderer {
        Renderer { canvas, scene }
    }

    pub fn render(&self, num_frames: u32) -> Result<ImageData, JsValue> {
        // Get canvas and context
        let context = self.canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Create the new ImageData object for direct pixel manipulation
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut []), self.canvas.width(), self.canvas.height())?;

        // Access the pixel data array
        let mut data = image_data.data();

        let mut state = 367380976;
        let max_state_value = 1e9;

        let mut cumulative_image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut []), self.canvas.width(), self.canvas.width())?; 

        // Recursively render the scene
        for frame in 0..num_frames {
            let mut i = 0;

            // Loop through each pixel on the canvas
            for y in 0..self.canvas.height() {
                for x in 0..self.canvas.width() {
                    // Get the state for the number generator
                    state = ((x + 349279) * (x * 213574) * (y + 784674) * (y * 426676) * (frame + 1)) as u32 % max_state_value as u32;

                    // Call the per_pixel function to get the color at the pixel
                    let color = self.per_pixel(x as f64, y as f64, state);

                    // Set the pixel color in ImageData
                    data[i] = (color.x * 255.0) as u8;
                    data[i + 1] = (color.y * 255.0) as u8;
                    data[i + 2] = (color.z * 255.0) as u8;
                    data[i + 4] = 255; // Alpha channel

                    i += 4;
                }
            }

            // Update the cumulativeImageData with averaging the pixel
            for i in 0..data.len() {
                cumulative_image_data.data()[i] = cumulative_image_data.data()[i] + (data[i] / num_frames as u8);
            }

            console_log(&format!("Frame: {} ended with this state: {}", frame, state));
        }

        // Put the modified ImageData back to the canvas
        context.put_image_data(&cumulative_image_data, 0.0, 0.0)?;

        Ok(cumulative_image_data)
    }

    pub fn per_pixel(&self, x: f64, y: f64, state: u32) -> Vector {
        todo!("Add implementation for PerPixel");
    }

    pub fn trace_ray(&self, ray: &Ray, x: f64, y: f64, state: u32) -> Vector {
        todo!("Add implementation for TraceRay");
    }
}
