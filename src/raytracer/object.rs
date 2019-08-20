#![allow(dead_code)]
use super::math::vec3::*;
use super::ray::*;

// For storing hit data - important for proper layering
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

// Common traits for objects that can be queried for collisions etc
pub trait Object {
    fn check_hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct World {
    objects: Vec<Box<Object>>
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new()
        }
    }
    pub fn add_object(&mut self, obj: Box<Object>) {
        self.objects.push(obj);
    }
    pub fn pop_object(&mut self) -> Option<Box<Object>> {
        self.objects.pop()
    }
}

// TODO: impl Object for World

impl Object for World {
    fn check_hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp = HitRecord { // This is rather inefficient
            t: 0.0,
            p: Vec3::all(0.0),
            normal: Vec3::all(0.0)
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in &self.objects {
            if i.check_hit(r, t_min, closest_so_far, &mut temp) {
                hit_anything = true;
                closest_so_far = temp.t;
                rec.t = temp.t;
                rec.p = temp.p.copy();
                rec.normal = temp.normal.copy();
            }
        }
        return hit_anything;
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}

impl Object for Sphere {
    fn check_hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin.sub_by_vec(&self.center);
        let a = dot_product(&r.direction, &r.direction);
        let b = dot_product(&oc, &r.direction);
        let c = dot_product(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = rec.p.sub_by_vec(&self.center).div(self.radius);
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = rec.p.sub_by_vec(&self.center).div(self.radius);
                return true;
            }
        }
        return false;
    }
}