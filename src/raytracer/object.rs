#![allow(dead_code)]
extern crate rand;
use rand::Rng;

use super::math::vec3::*;
use super::ray::*;
use super::material::*;

// For initializing HitRecords with default values
const DEFAULT_MATERIAL: Lambertian = Lambertian {
                                        albedo: Vec3 {
                                            x: 0.0,
                                            y: 0.0,
                                            z: 0.0
                                        } 
                                    };

// For storing hit data - important for proper layering
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<Material>
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::all(0.0),
            normal: Vec3::all(0.0),
            material: Box::new(DEFAULT_MATERIAL)
        }
    }
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
    pub fn random() -> World {
        let mut rng = rand::thread_rng();
        let mut world = World::new();
        // Add the ground
        world.add_object(Box::new(
            Sphere::new(Vec3::new(0.0, -1000.0, 0.0),
                1000.0, Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))
            )
        ));
        // Add a bunch of random spheres
        for _ in 0..100 {
            world.add_object(Box::new(
                Sphere::new(Vec3::new(25.0 * (rng.gen::<f32>() - 0.5), 0.2, 25.0 * (rng.gen::<f32>() - 0.5)),
                    0.2, Box::new(Lambertian::new(Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())))
                )
            ));
        }
        // Add the three big spheres
        world.add_object(Box::new(
            Sphere::new(Vec3::new(0.0, 1.0, 0.0),
                1.0, Box::new(Dielectric { ref_idx: 1.5 })
            )
        ));
        world.add_object(Box::new(
            Sphere::new(Vec3::new(-4.0, 1.0, 0.0),
                1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))
            )
        ));
        world.add_object(Box::new(
            Sphere::new(Vec3::new(4.0, 1.0, 0.0),
                1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))
            )
        ));
        world
    }
}

// TODO: impl Object for World

impl Object for World {
    fn check_hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in &self.objects {
            if i.check_hit(r, t_min, closest_so_far, &mut temp) {
                hit_anything = true;
                closest_so_far = temp.t;
                rec.t = temp.t;
                rec.p = temp.p.copy();
                rec.normal = temp.normal.copy();
                rec.material = temp.material.copy()
            }
        }
        return hit_anything;
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material
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
                rec.material = self.material.copy();
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = rec.p.sub_by_vec(&self.center).div(self.radius);
                rec.material = self.material.copy();
                return true;
            }
        }
        return false;
    }
}