#![allow(dead_code)]
extern crate rand;
use rand::Rng;

use super::math::vec3::*;
use super::ray::*;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    while {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0).mul(2.0).sub_by_vec(&Vec3::new(1.0, 1.0, 0.0)); 
        p.dot(&p) >= 1.0
    } {}
    p
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32,  aperture: f32, focus_dist: f32) -> Camera {
        let theta = (vfov * std::f32::consts::PI)/180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = lookfrom.sub_by_vec(&lookat).as_unit();
        let u = vup.cross(&w).as_unit();
        let v = w.cross(&u);
        let p1 = lookfrom.sub_by_vec(&u.mul(half_width * focus_dist));
        let p2 = &v.mul(half_height * focus_dist);
        let p3 = &w.mul(focus_dist);
        let llc = p1.sub_by_vec(&p2).sub_by_vec(&p3);
        Camera {
            lower_left_corner: llc,
            horizontal: u.mul(half_width * focus_dist * 2.0),
            vertical: v.mul(half_height * focus_dist * 2.0),
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
            w: w
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disk().mul(self.lens_radius);
        let offset = self.u.mul(rd.x).add_by_vec(&self.v.mul(rd.y));
        let u_component = self.horizontal.mul(u);
        let v_component = self.vertical.mul(v);
        let direction = self.lower_left_corner.add_by_vec(
            &u_component.add_by_vec(&v_component)
        ).sub_by_vec(&self.origin).sub_by_vec(&offset);
        Ray::new(
            self.origin.add_by_vec(&offset),
            direction
        )
    }
}