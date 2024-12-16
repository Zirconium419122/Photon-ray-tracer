use crate::{material::Material, ray::Ray, vector::Vector};

pub(crate) trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn calculate_normal(&self, point: &Vector) -> Vector;
    fn get_material(&self) -> &Material;
}

// pub(crate) trait IntersectableClone {
//     fn clone_box(&self) -> Box<dyn Intersectable>;
// }

// impl<T> IntersectableClone for T
// where
//     T: 'static + Intersectable + Clone
// {
//     fn clone_box(&self) -> Box<dyn Intersectable> {
//         Box::new(self.clone())
//     }
// }

// impl IntersectableClone for Box<dyn Intersectable> {
//     fn clone_box(&self) -> Box<dyn Intersectable> {
//         self.
//     }
// }

// impl Clone for Box<dyn Intersectable> {
//     fn clone(&self) -> Box<dyn Intersectable> {
//         self.clone()
//     }
// }

// Rust Intersection struct
pub struct Intersection {
    pub t: f64,
    pub intersection_point: Vector,
    pub intersection_object: Box<dyn Intersectable>,
}
