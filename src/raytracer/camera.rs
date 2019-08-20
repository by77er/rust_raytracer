#![allow(dead_code)]
use super::math::vec3::*;
use super::ray::*;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        // Hard-coded size for now
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0)
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let u_component = self.horizontal.mul(u);
        let v_component = self.vertical.mul(v);
        let direction = self.lower_left_corner.add_by_vec(
            &u_component.add_by_vec(&v_component)
        );
        Ray::new(
            self.origin.copy(),
            direction
        )
    }
}