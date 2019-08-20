#![allow(dead_code)]
extern crate rand;
use rand::Rng;

use super::math::vec3::*;
use super::object::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }
    // p(t) = Origin + Direction * t
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin.add_by_vec(&self.direction.mul(t))
    }
    // Get the color for a ray
    pub fn get_color(&self, world: &dyn Object) -> Vec3 {
        // Check hits
        let mut temp = HitRecord {
            t: 0.0,
            p: Vec3::all(0.0),
            normal: Vec3::all(0.0)
        };

        let hit = world.check_hit(&self, 0.001, std::f32::MAX, &mut temp);

        if hit {
            let target = temp.p.add_by_vec(&temp.normal).add_by_vec(&random_in_unit_sphere());
            let mut sub_color = Ray::new(temp.p.copy(), target.sub_by_vec(&temp.p)).get_color(world);
            sub_color.mul_eq(0.5);
            sub_color
        } else {
            // If there wasn't a hit, just show the background color
            let unit_direction = self.direction.as_unit();
            let t = (unit_direction.y + 1.0) * 0.5;
            // Saving memory by modifying vectors in place
            let mut step1 = Vec3::new(0.5, 0.7, 1.0);
            step1.mul_eq(t);
            let mut res = Vec3::all(1.0);
            res.mul_eq(1.0 - t);
            res.add_by_vec_eq(&step1);
            res
        }
    }
}

// For calculating the direction of a randomly bouncing ray
pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::all(0.0);
    let ones = Vec3::all(1.0); // Save memory
    loop {
        // Random point in unit cube, saving memory
        p.x = rng.gen::<f32>();
        p.y = rng.gen::<f32>();
        p.z = rng.gen::<f32>();
        // Check if it's within the unit sphere
        p.sub_by_vec_eq(&ones);
        p.mul_eq(2.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}