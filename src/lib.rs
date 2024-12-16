extern crate console_error_panic_hook;

mod cube;
mod intersection;
mod material;
mod random;
mod ray;
mod sphere;
mod vector;

use crate::{
    cube::Cube,
    intersection::{Intersectable, Intersection},
    random::Random,
    ray::Ray,
    sphere::Sphere,
    vector::Vector,
};

use std::{cell::RefCell, rc::Rc};

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
    random: Random,
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
            random: Random::new(367380976),
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

        // Loop through each pixel on the canvas
        for y in 0..self.canvas.height() {
            for x in 0..self.canvas.width() {
                let color = self.per_pixel(x as f64, y as f64);

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

    fn per_pixel(&mut self, x: f64, y: f64) -> Vector {
        // Initialize the accumlateColor Vector
        let mut accumulated_color = Vector::default();

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
                self.settings.max_reflection_depth,
                Vector {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            );

            // Accumulate the color
            accumulated_color += color;
        }

        accumulated_color / self.settings.num_samples as f64
    }

    fn trace_ray(&mut self, ray: &mut Ray, depth: u32, mut ray_color: Vector) -> Vector {
        if depth <= 0 {
            return Vector::default();
        }

        let mut incoming_light = Vector::default();

        let random = &mut self.random;

        let mut closest_intersection: Option<Intersection> = None;

        for sphere in &self.scene.spheres {
            if let Some(intersection_result) = sphere.intersect(ray) {
                if closest_intersection.is_none() {
                    closest_intersection = Some(intersection_result);
                } else if intersection_result.t
                    < unsafe { &closest_intersection.clone().unwrap_unchecked().t }
                {
                    closest_intersection = Some(intersection_result);
                }
            }
        }

        for cube in &self.scene.cubes {
            if let Some(intersection_result) = cube.intersect(ray) {
                if closest_intersection.is_none() {
                    closest_intersection = Some(intersection_result);
                } else if intersection_result.t
                    < unsafe { closest_intersection.clone().unwrap_unchecked().t }
                {
                    closest_intersection = Some(intersection_result);
                }
            }
        }

        match closest_intersection {
            Some(intersection) => {
                let intersection_point = intersection.intersection_point;

                // Get the normal on the Sphere
                let normal = intersection
                    .intersection_object
                    .calculate_normal(&intersection_point);

                // Update the origin and direction of the ray for the next iteration
                ray.origin = intersection_point;
                ray.direction = random.random_hemisphere_direction(&normal);

                // Calculate the incoming light
                let emission_color = intersection
                    .intersection_object
                    .get_material()
                    .emission_color;
                let emission_power = intersection
                    .intersection_object
                    .get_material()
                    .emission_power;
                let emitted_light = emission_color * emission_power;
                let emission = emitted_light * ray_color;
                incoming_light += emission;

                ray_color *= intersection.intersection_object.get_material().color;

                if intersection
                    .intersection_object
                    .get_material()
                    .emission_power
                    > 0.0
                {
                    return incoming_light;
                }
            }
            // IntersectionObject::Sphere(intersection_sphere) => {
            //     let sphere_intersection_point = intersection_sphere.intersection_point;
            //     let sphere = intersection_sphere.intersection_object;

            //     // Get the normal on the Sphere
            //     let normal = sphere.calculate_normal(&sphere_intersection_point);

            //     // Update the origin and direction of the ray for the next iteration
            //     ray.origin = sphere_intersection_point;
            //     ray.direction = random.random_hemisphere_direction(&normal);

            //     // Calculate the incoming light
            //     let emitted_light = sphere.get_material().emission_color * sphere.get_material().emission_power;
            //     let emission = emitted_light * ray_color;
            //     incoming_light += emission;

            //     ray_color *= sphere.get_material().color;

            //     if sphere.get_material().emission_power > 0.0 {
            //         return incoming_light;
            //     }
            // }
            // IntersectionObject::Cube(intersection_cube) => {
            //     let cube_intersection_point = intersection_cube.intersection_point;
            //     let cube = intersection_cube.intersection_object;

            //     // Get the normal on the Cube
            //     let normal = cube.calculate_normal(&cube_intersection_point);

            //     // Update the origin and direction of the ray for the next iteration
            //     ray.origin = cube_intersection_point;
            //     ray.direction = random.random_hemisphere_direction(&normal);

            //     // Calculate the incoming light
            //     let emitted_light = cube.get_material().emission_color * cube.get_material().emission_power;
            //     let emission = emitted_light * ray_color;
            //     incoming_light += emission;

            //     ray_color *= cube.get_material().color;

            //     if cube.get_material().emission_power > 0.0 {
            //         return incoming_light;
            //     }
            // }
            None => {
                let background_color = ray.get_background_color();
                return ray_color * background_color;
            }
        };

        self.trace_ray(ray, depth + 1, ray_color)
    }
}
