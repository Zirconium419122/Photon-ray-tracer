extern crate console_error_panic_hook;

use core::panic;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div};

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use rand::random;

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

// Rust Random struct
pub struct Random {
    state: u32,
}

impl Random {
    pub fn new(seed: u32) -> Random {
        init_panic_hook();
        
        Random { state: seed }
    }

    pub fn random_value(&mut self) -> f64 {
        self.state = self.state * 747796405 + 2891336453;
        let mut result = ((self.state >> ((self.state >> 28) + 4)) ^ self.state) * 277803737;
        result = (result >> 22) ^ result;
        result as f64 / 4294967295.0
    }

    pub fn random_direction(&mut self) -> Vector {
        for _ in 0..100 {
            let x = self.random_value() * 2.0 - 1.0;
            let y = self.random_value() * 2.0 - 1.0;
            let z = self.random_value() * 2.0 - 1.0;

            let point_in_cube = Vector { x, y, z };
            let sqr_dst_from_center = point_in_cube.dot(&point_in_cube);

            // If point is inside sphere, scale it to lie on the surface (otherwise, keep trying)
            if sqr_dst_from_center <= 1.0 {
                return point_in_cube / f64::sqrt(sqr_dst_from_center);
            }
        }

        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn random_hemisphere_direction(&mut self, normal: &Vector) -> Vector {
        let direction = self.random_direction();
        direction * normal.dot(&direction).signum()
    }
}

// Rust Intersection struct
#[derive(Debug, Copy, Clone)]
pub struct Intersection<T> {
    t: f64,
    intersection_point: Vector,
    intersection_object: T,
}

// Rust IntersectionType enum
#[derive(Debug, Copy, Clone)]
pub enum IntersectionType {
    Sphere,
    Cube,
}

// Rust IntersectionObject enum
#[derive(Debug, Copy, Clone)]
pub enum IntersectionObject {
    Sphere(Option<Intersection<Sphere>>),
    Cube(Option<Intersection<Cube>>),
    None,
}

// Rust Intersections struct for holding the closest intersection of each type
#[derive(Debug, Copy, Clone)]
pub struct Intersections {
    sphere_intersection: Option<Intersection<Sphere>>,
    cube_intersection: Option<Intersection<Cube>>,
    closer_type: Option<IntersectionType>,
}

impl Intersections {
    // Method / Function to create a new Intersections struct
    pub fn new(sphere_intersection: Option<Intersection<Sphere>>, cube_intersection: Option<Intersection<Cube>>) -> Intersections {
        init_panic_hook();
        
        Intersections { sphere_intersection, cube_intersection, closer_type: None }
    }
    // Method / Function to determine the which intersection is closer
    pub fn determine_closer(&mut self) {
        self.closer_type = match (&self.sphere_intersection, &self.cube_intersection) {
            (Some(sphere_intersection), Some(cube_intersection)) => {
                if sphere_intersection.t < cube_intersection.t {
                    Some(IntersectionType::Sphere)
                } else {
                    Some(IntersectionType::Cube)
                }
            },
            (Some(_), None) => Some(IntersectionType::Sphere),
            (None, Some(_)) => Some(IntersectionType::Cube),
            (None, None) => None,
        };
    }

    // Method / Function to get the closer intersection
    pub fn get_closer_intersection(&self) -> IntersectionObject {
        match self.closer_type {
            Some(IntersectionType::Sphere) => IntersectionObject::Sphere(self.sphere_intersection),
            Some(IntersectionType::Cube) => IntersectionObject::Cube(self.cube_intersection),
            None => IntersectionObject::None,
        }
    }
}


// Rust Material struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
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
        init_panic_hook();
        
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
        init_panic_hook();
        
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector {
        self.origin.add(self.direction * t)
    }

    pub fn get_background_color(&self) -> Vector {
        let t = 0.5 * (self.direction.y + 1.0);
    
        let white = Vector::new(1.0, 1.0, 1.0);
        let blue = Vector::new(0.5, 0.7, 1.0);
    
        let gradient = white * (1.0 - t) + (blue * t);
    
        gradient
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
        init_panic_hook();
        
        Sphere {
            center,
            radius,
            material,
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection<Sphere>> {
        let oc = ray.origin - self.center;
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
        (*point - self.center).normalize()
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
        init_panic_hook();

        Cube {
            center,
            size,
            material,
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection<Cube>> {
        let half_size = self.size * 0.5;

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

// Rust Settings struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Settings {
    max_reflection_depth: u32,
    num_samples: u32,
    num_frames: u32,
}

#[wasm_bindgen]
impl Settings {
    #[wasm_bindgen(constructor)]
    pub fn new(max_reflection_depth: u32, num_samples: u32, num_frames: u32) -> Settings {
        Settings { max_reflection_depth, num_samples, num_frames }
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
        init_panic_hook();

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
    settings: Settings,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, scene: Scene, settings: Settings) -> Renderer {
        init_panic_hook();

        Renderer { canvas, scene, settings }
    }

    pub fn render(&self) -> Result<ImageData, JsValue> {
        // Get canvas and context
        let context = self
            .canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Create the new ImageData object for direct pixel manipulation
        let image_data = ImageData::new_with_sw(
            self.canvas.width(),
            self.canvas.height(),
        )?;

        // Access the pixel data array
        let mut data = image_data.data();

        let mut state = 367380976;
        let max_state_value = 1e9;

        let cumulative_image_data = ImageData::new_with_sw(
            self.canvas.width(),
            self.canvas.width(),
        )?;

        let mut cumulative_data = cumulative_image_data.data();

        // Recursively render the scene
        for frame in 0..self.settings.num_frames {
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
                    data[i + 3] = 255; // Alpha channel

                    i += 4;
                }

                console_log(
                    &format!("Row number {} is complete", y)
                );
            }

            // Update the cumulativeImageData with averaging the pixel
            for i in 0..data.len() {
                cumulative_data[i] = cumulative_data[i] + (data[i] / self.settings.num_frames as u8);
            }

            console_log(&format!(
                "Frame: {} ended with this state: {}", frame, state
            ));
        }

        let cumulative_image_data = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(cumulative_data.as_slice()),
            self.canvas.width(),
            self.canvas.height()
        )?;

        // Put the modified ImageData back to the canvas
        context.put_image_data(&cumulative_image_data, 0.0, 0.0)?;

        Ok(cumulative_image_data)
    }

    pub fn per_pixel(&self, x: f64, y: f64, state: u32) -> Vector {
        // Initialize the accumlateColor Vector
        let mut accumulated_color = Vector::new(0.0, 0.0, 0.0);

        for sample in 0..self.settings.num_samples {
            // Calculate the jittered sample position within the pixel
            let jitter_x: f64 = (random::<f64>() - 0.5) / 2.0;
            let jitter_y: f64 = (random::<f64>() - 0.5) / 2.0;

            // Calculate pixel coordinates for the jittered sample
            let sample_x: f64 = x + (sample as f64 + jitter_x) / 10 as f64;
            let sample_y: f64 = y + (sample as f64 + jitter_y) / 10 as f64;

            // Create a ray from the camera to the current pixel
            let aspect_ratio = self.canvas.width() as f64 / self.canvas.height() as f64;
            let ray_origin = Vector::new(0.0, 0.0, 0.0);
            let ray_direction = Vector::new(
                (sample_x / self.canvas.width() as f64) * 2.0 - 1.0,
                ((sample_y / self.canvas.height() as f64) * 2.0 - 1.0) / aspect_ratio,
                -1.0,
            );
            let mut ray = Ray::new(ray_origin, ray_direction);

            // Trace the ray to get the color
            let color = self.trace_ray(&mut ray, sample_x, sample_y, state);

            // Accumulate the color
            accumulated_color = accumulated_color + color;
        }

        accumulated_color / self.settings.num_samples.into()
    }

    pub fn trace_ray(&self, ray: &mut Ray, x: f64, y: f64, state: u32) -> Vector {
        // Create seed for random number generator
        let num_pixels = self.canvas.width() * self.canvas.height();
        let pixel_index = (y * self.canvas.width() as f64 + x) as u32;
        let state = state + num_pixels + pixel_index * 485732;

        let mut incoming_light = Vector::new(0.0, 0.0, 0.0);
        let mut ray_color = Vector::new(1.0, 1.0, 1.0);

        let mut random = Random::new(state);

        let mut closest_intersection_sphere: Option<Intersection<Sphere>> = None;
        let mut closest_intersection_cube: Option<Intersection<Cube>> = None;

        // Recursively reflect the ray
        for _ in 0..self.settings.max_reflection_depth {
            // Test for intersection with objects in the scene
            for sphere in &self.scene.spheres {
                if let Some(intersection_result) = sphere.intersect(&ray) {
                    if closest_intersection_sphere.is_none()
                        || intersection_result.t < closest_intersection_sphere.unwrap().t
                    {
                        closest_intersection_sphere = Some(intersection_result);
                    }
                }
            }

            for cube in &self.scene.cubes {
                if let Some(intersection_result) = cube.intersect(&ray) {
                    if closest_intersection_cube.is_none()
                        || intersection_result.t < closest_intersection_cube.unwrap().t
                    {
                        closest_intersection_cube = Some(intersection_result);
                    }
                }
            }

            let mut closest_intersections: Intersections = Intersections::new(closest_intersection_sphere, closest_intersection_cube);
            closest_intersections.determine_closer();

            match closest_intersections.get_closer_intersection() {
                IntersectionObject::Sphere(intersection_sphere) => {
                    let intersection_point = intersection_sphere.unwrap().intersection_point;
                    let object = &intersection_sphere.unwrap().intersection_object;

                    // Get the normal on the object
                    let normal = object.calculate_normal(&intersection_point);

                    // Update the origin and direction of the ray for the next iteration
                    ray.origin = intersection_point;
                    ray.direction = random.random_hemisphere_direction(&normal);

                    // Calculate the incoming light
                    let emitted_light = object.material.emission_color * object.material.emission_power;
                    let emission = emitted_light * ray_color;
                    incoming_light += emission;

                    ray_color *= object.material.color;

                    if object.material.emission_power > 0.0 {
                        return incoming_light;
                    }
                },
                IntersectionObject::Cube(intersection_cube) => {
                    let intersection_point = intersection_cube.unwrap().intersection_point;
                    let object = &intersection_cube.unwrap().intersection_object;

                    // Get the normal on the object
                    let normal = object.calculate_normal(&intersection_point);

                    // Update the origin and direction of the ray for the next iteration
                    ray.origin = intersection_point;
                    ray.direction = random.random_hemisphere_direction(&normal);

                    // Calculate the incoming light
                    let emitted_light = object.material.emission_color * object.material.emission_power;
                    let emission = emitted_light * ray_color;
                    incoming_light += emission;

                    ray_color *= object.material.color;

                    if object.material.emission_power > 0.0 {
                        return incoming_light;
                    }
                },
                IntersectionObject::None => {
                    let background_color = ray.get_background_color();
                    return ray_color * background_color;
                },
            };
        }

        incoming_light
    }
}
