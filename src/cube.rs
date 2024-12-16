use wasm_bindgen::prelude::*;

use crate::{
    init_panic_hook,
    intersection::{Intersectable, Intersection},
    material::Material,
    ray::Ray,
    vector::Vector,
};

// Rust Cube struct
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Cube {
    pub center: Vector,
    pub size: Vector,
    pub material: Material,
}

impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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
                intersection_object: Box::new(*self),
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

    fn get_material(&self) -> &Material {
        &self.material
    }
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
}
