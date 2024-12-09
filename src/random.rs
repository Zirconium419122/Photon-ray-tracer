use crate::{init_panic_hook, vector::Vector};

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
        self.state = self.state.wrapping_mul(747796405).wrapping_add(2891336453);
        let mut result =
            ((self.state >> ((self.state >> 28) + 4)) ^ self.state).wrapping_add(277803737);
        result = (result >> 22) ^ result;
        result as f64 / 4294967295.0
    }

    pub fn random_direction(&mut self) -> Vector {
        for _ in 0..100 {
            // Generate a random point in a cube
            let x = self.random_value() * 2.0 - 1.0;
            let y = self.random_value() * 2.0 - 1.0;
            let z = self.random_value() * 2.0 - 1.0;

            // Calculate the distance from the center of the cube
            let point_in_cube = Vector { x, y, z };
            let sqr_dst_from_center = point_in_cube.dot(&point_in_cube);

            // If point is inside sphere, scale it to lie on the surface (otherwise, keep trying)
            if sqr_dst_from_center <= 1.0 {
                return point_in_cube / f64::sqrt(sqr_dst_from_center);
            }
        }

        Vector::default()
    }

    pub fn random_hemisphere_direction(&mut self, normal: &Vector) -> Vector {
        let direction = self.random_direction();
        direction * normal.dot(&direction).signum()
    }
}
