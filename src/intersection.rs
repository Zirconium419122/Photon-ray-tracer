use crate::{cube::Cube, init_panic_hook, ray::Ray, sphere::Sphere, vector::Vector};

pub(crate) trait Intersect<T> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection<T>>;
    fn calculate_normal(&self, point: &Vector) -> Vector;
}

// Rust Intersection struct
#[derive(Debug, Copy, Clone)]
pub struct Intersection<T> {
    pub t: f64,
    pub intersection_point: Vector,
    pub intersection_object: T,
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
    Sphere(Intersection<Sphere>),
    Cube(Intersection<Cube>),
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
    pub fn new(
        sphere_intersection: Option<Intersection<Sphere>>,
        cube_intersection: Option<Intersection<Cube>>,
    ) -> Intersections {
        init_panic_hook();

        Intersections {
            sphere_intersection,
            cube_intersection,
            closer_type: None,
        }
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
            }
            (Some(_), None) => Some(IntersectionType::Sphere),
            (None, Some(_)) => Some(IntersectionType::Cube),
            (None, None) => None,
        };
    }

    // Method / Function to get the closer intersection
    pub fn get_closer_intersection(&self) -> IntersectionObject {
        match self.closer_type {
            Some(IntersectionType::Sphere) => IntersectionObject::Sphere(self.sphere_intersection.unwrap()),
            Some(IntersectionType::Cube) => IntersectionObject::Cube(self.cube_intersection.unwrap()),
            None => IntersectionObject::None,
        }
    }
}
