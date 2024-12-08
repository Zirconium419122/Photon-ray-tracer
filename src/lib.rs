extern crate console_error_panic_hook;

mod intersection;
mod random;
mod vector;

use crate::{
    intersection::{Intersection, IntersectionObject, Intersections},
    random::Random,
    vector::Vector,
};

use std::{cell::RefCell, ops::Add, rc::Rc};

use wasm_bindgen::{prelude::*, Clamped};
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
            let t = (-b - discriminant.sqrt()) / (2.0 * a);

            if t > 0.0 {
                let intersection_point = ray.point_at_parameter(t);
                return Some(Intersection {
                    t,
                    intersection_point,
                    intersection_object: *self,
                });
            }
        }

        // Ray does not intersect the sphere
        None
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
        let half_size = self.size / 2.0;

        // Calculate the minimum and maximum extents along each axis
        let min_bound = self.center - half_size;
        let max_bound = self.center + half_size;

        let inv_direction = Vector {
            x: 1.0 / ray.direction.x,
            y: 1.0 / ray.direction.y,
            z: 1.0 / ray.direction.z,
        };

        // Calculate the intersection distances along each axis
        let t1 = (min_bound.x - ray.origin.x) * inv_direction.x;
        let t2 = (max_bound.x - ray.origin.x) * inv_direction.x;

        let t_min = t1.min(t2);
        let t_max = t1.max(t2);

        let t3 = (min_bound.y - ray.origin.y) * inv_direction.y;
        let t4 = (max_bound.y - ray.origin.y) * inv_direction.y;

        let t_min = t3.min(t4).max(t_min);
        let t_max = t3.max(t4).min(t_max);

        let t5 = (min_bound.z - ray.origin.z) * inv_direction.z;
        let t6 = (max_bound.z - ray.origin.z) * inv_direction.z;

        let t_min = t5.min(t6).max(t_min);
        let t_max = t5.max(t6).min(t_max);

        // Check if there is a valid intersection
        if t_min <= t_max && t_min >= 0.0 {
            // Return the intersection point at the minmum distance
            let intersection_point = ray.point_at_parameter(t_min);
            return Some(Intersection {
                t: t_min,
                intersection_point,
                intersection_object: *self,
            });
        }

        // Ray does not intersect with the cube
        None
    }

    fn calculate_normal(&self, point: &Vector) -> Vector {
        // Calculate the differences between the point's coordinates and the cube's center
        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;
        let dz = point.z - self.center.z;

        // Identify the face closest to the point and assign the normal accordingly
        if dx.abs() > dy.abs() && dx.abs() > dz.abs() {
            // Point is on the face with the largest x-coordinate differance
            Vector {
                x: dx.signum(),
                y: 0.0,
                z: 0.0,
            }
        } else if dy.abs() > dz.abs() {
            // Point is on the face with the largest y-coordinate differance
            Vector {
                x: 0.0,
                y: dy.signum(),
                z: 0.0,
            }
        } else {
            // Point is on the face with the largest z-coordinate differance
            Vector {
                x: 0.0,
                y: 0.0,
                z: dz.signum(),
            }
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
        Settings {
            max_reflection_depth,
            num_samples,
            num_frames,
        }
    }
}

// Rust Scene struct
#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone)]
pub struct Renderer {
    canvas: HtmlCanvasElement,
    scene: Scene,
    settings: Settings,
    cumulative_image_data: Vec<u32>,
    current_frame: u32,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        scene: Scene,
        settings: Settings,
    ) -> Result<Renderer, JsValue> {
        init_panic_hook();

        Ok(Renderer {
            canvas: canvas.clone(),
            scene,
            settings,
            cumulative_image_data: vec![0; canvas.width() as usize * canvas.height() as usize * 4],
            current_frame: 0,
        })
    }

    pub fn run(&self) -> Result<(), JsValue> {
        Renderer::render_next_frame(Rc::new(RefCell::new(self.clone())))?;

        Ok(())
    }

    fn render_next_frame(self_rc: Rc<RefCell<Renderer>>) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let self_clone = self_rc.clone();
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let mut renderer = self_clone.borrow_mut();
            if renderer.current_frame < renderer.settings.num_frames {
                renderer.render_frame().unwrap();
                renderer.current_frame += 1;
                Renderer::render_next_frame(self_clone.clone()).unwrap();
            }
        }));

        window.request_animation_frame(closure.as_ref().unchecked_ref())?;

        closure.forget();

        Ok(())
    }

    fn render_frame(&mut self) -> Result<(), JsValue> {
        // Get canvas and context
        let context = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Create the new ImageData object for direct pixel manipulation
        let image_data = ImageData::new_with_sw(self.canvas.width(), self.canvas.height())?;

        // Access the pixel data array
        let mut data = image_data.data();

        let mut state;
        let max_state_value = 1e9;

        // Loop through each pixel on the canvas
        for y in 0..self.canvas.height() {
            for x in 0..self.canvas.width() {
                state = ((x + 349279) * (x * 213574) * (y + 784674) * (y * 426676))
                    % max_state_value as u32;
                let color = self.per_pixel(x as f64, y as f64, state);

                let i = ((y * self.canvas.width() + x) * 4) as usize;
                data[i] = (color.x * 255.0) as u8;
                data[i + 1] = (color.y * 255.0) as u8;
                data[i + 2] = (color.z * 255.0) as u8;
                data[i + 3] = 255; // Alpha channel
            }

            let bar_length: f32 = 50.0;
            let fraction: f32 = y as f32 / self.canvas.height() as f32;
            let filled_length: usize = (bar_length * fraction) as usize;
            let bar: String = format!(
                "[{}{}] {:.2}%",
                "#".repeat(filled_length),
                "-".repeat(bar_length as usize - filled_length),
                fraction * 100.0
            );
            console_log(&bar);
        }

        // Update cumulative_image_data
        for i in 0..data.len() {
            self.cumulative_image_data[i] =
                self.cumulative_image_data[i].saturating_add(data[i] as u32);
        }

        let averaged_image_data = ImageData::new_with_u8_clamped_array(
            Clamped(
                &(self
                    .cumulative_image_data
                    .iter()
                    .map(|x| x / (self.current_frame + 1))
                    .map(|x| x as u8)
                    .collect::<Vec<u8>>()),
            ),
            self.canvas.width(),
        )?;

        // Apply the frame data to the canvas
        context.put_image_data(&averaged_image_data, 0.0, 0.0)?;

        Ok(())
    }

    fn per_pixel(&self, x: f64, y: f64, state: u32) -> Vector {
        // Initialize the accumlateColor Vector
        let mut accumulated_color = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        for sample in 0..self.settings.num_samples {
            // Calculate the jittered sample position within the pixel
            let jitter_x: f64 = (random::<f64>() - 0.5) / 2.0;
            let jitter_y: f64 = (random::<f64>() - 0.5) / 2.0;

            // Calculate pixel coordinates for the jittered sample
            let sample_x: f64 = x + (sample as f64 + jitter_x) / self.settings.num_samples as f64;
            let sample_y: f64 = y + (sample as f64 + jitter_y) / self.settings.num_samples as f64;

            // Create a ray from the camera to the current pixel
            let aspect_ratio = self.canvas.width() as f64 / self.canvas.height() as f64;
            let ray_origin = Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let ray_direction = Vector {
                x: (sample_x / self.canvas.width() as f64) * 2.0 - 1.0,
                y: ((sample_y / self.canvas.height() as f64) * 2.0 - 1.0) / aspect_ratio,
                z: -1.0,
            };
            let mut ray = Ray::new(ray_origin, ray_direction);

            // Trace the ray to get the color
            let color = self.trace_ray(
                &mut ray,
                sample_x,
                sample_y,
                state,
                0,
                Vector {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            );

            // Accumulate the color
            accumulated_color += color;
        }

        accumulated_color / self.settings.num_samples.into()
    }

    fn trace_ray(
        &self,
        ray: &mut Ray,
        x: f64,
        y: f64,
        mut state: u32,
        depth: u32,
        mut ray_color: Vector,
    ) -> Vector {
        let num_pixels = self.canvas.width() * self.canvas.height();
        let pixel_index = (y * self.canvas.width() as f64 + x) as u32;
        state += num_pixels + pixel_index * 485732;

        if depth > self.settings.max_reflection_depth {
            return Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        let mut incoming_light = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let mut random = Random::new(state);

        let mut closest_intersection_sphere: Option<Intersection<Sphere>> = None;
        let mut closest_intersection_cube: Option<Intersection<Cube>> = None;

        for sphere in &self.scene.spheres {
            if let Some(intersection_result) = sphere.intersect(ray) {
                if closest_intersection_sphere.is_none()
                    || intersection_result.t < closest_intersection_sphere.unwrap().t
                {
                    closest_intersection_sphere = Some(intersection_result);
                }
            }
        }

        for cube in &self.scene.cubes {
            if let Some(intersection_result) = cube.intersect(ray) {
                if closest_intersection_cube.is_none()
                    || intersection_result.t < closest_intersection_cube.unwrap().t
                {
                    closest_intersection_cube = Some(intersection_result);
                }
            }
        }

        let mut closest_intersections =
            Intersections::new(closest_intersection_sphere, closest_intersection_cube);
        closest_intersections.determine_closer();

        match closest_intersections.get_closer_intersection() {
            IntersectionObject::Sphere(intersection_sphere) => {
                let sphere_intersection_point = intersection_sphere.unwrap().intersection_point;
                let sphere = intersection_sphere.unwrap().intersection_object;

                // Get the normal on the Sphere
                let normal = sphere.calculate_normal(&sphere_intersection_point);

                // Update the origin and direction of the ray for the next iteration
                ray.origin = sphere_intersection_point;
                ray.direction = random.random_hemisphere_direction(&normal);

                // Calculate the incoming light
                let emitted_light = sphere.material.emission_color * sphere.material.emission_power;
                let emission = emitted_light * ray_color;
                incoming_light += emission;

                ray_color *= sphere.material.color;

                if sphere.material.emission_power > 0.0 {
                    return incoming_light;
                }
            }
            IntersectionObject::Cube(intersection_cube) => {
                let cube_intersection_point = intersection_cube.unwrap().intersection_point;
                let cube = intersection_cube.unwrap().intersection_object;

                // Get the normal on the Cube
                let normal = cube.calculate_normal(&cube_intersection_point);

                // Update the origin and direction of the ray for the next iteration
                ray.origin = cube_intersection_point;
                ray.direction = random.random_hemisphere_direction(&normal);

                // Calculate the incoming light
                let emitted_light = cube.material.emission_color * cube.material.emission_power;
                let emission = emitted_light * ray_color;
                incoming_light += emission;

                ray_color *= cube.material.color;

                if cube.material.emission_power > 0.0 {
                    return incoming_light;
                }
            }
            IntersectionObject::None => {
                let background_color = ray.get_background_color();
                return ray_color * background_color;
            }
        };

        self.trace_ray(ray, x, y, state, depth + 1, ray_color)
    }
}
