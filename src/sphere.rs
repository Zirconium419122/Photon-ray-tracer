use wasm_bindgen::prelude::*;

use crate::{
    init_panic_hook, intersection::{Intersect, Intersection}, material::Material, ray::Ray, vector::Vector,
};

// Rust Sphere struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Intersect<Sphere> for Sphere {
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
}
