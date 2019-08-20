#![allow(dead_code)]

extern crate rand;
use rand::Rng;

use super::math::vec3::*;
use super::ray::*;
use super::object::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
    fn copy(&self) -> Box<dyn Material>;
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    let half = n.mul(dot_product(v, n)).mul(2.0);
    v.sub_by_vec(&half)
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = v.as_unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let tmp = uv.sub_by_vec(&n.mul(dt)).mul(ni_over_nt).sub_by_vec(&n.mul(discriminant.sqrt()));
        refracted.x = tmp.x;
        refracted.y = tmp.y;
        refracted.z = tmp.z;
        true
    } else {
        false
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ( 1.0 - ref_idx ) / ( 1.0 + ref_idx );
    r0 * r0 + ( 1.0 - r0 ) * ( 1.0 - cosine ).powf(5.0)
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p.add_by_vec(&rec.normal).add_by_vec(&random_in_unit_sphere());
        scattered.origin = rec.p.copy();
        scattered.direction = target.sub_by_vec(&rec.p);
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        return true;
    }
    fn copy(&self) -> Box<dyn Material> {
        Box::new(Lambertian { albedo: self.albedo.copy() })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(&ray.direction.as_unit(), &rec.normal);
        scattered.origin = rec.p.copy();
        scattered.direction = reflected.add_by_vec(&random_in_unit_sphere().mul(self.fuzz));
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        dot_product(&scattered.direction, &rec.normal) > 0.0
    }
    fn copy(&self) -> Box<dyn Material> {
        Box::new(Metal { albedo: self.albedo.copy(), fuzz: self.fuzz })
    }
}

pub struct Dielectric {
    pub ref_idx: f32 // index of refraction
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3;
        let reflected = reflect(&ray.direction, &rec.normal);
        let ni_over_nt: f32;
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;
        let mut refracted = Vec3::all(0.0);
        let reflect_prob: f32;
        let cosine: f32;
        let mut rng = rand::thread_rng();
        if dot_product(&ray.direction, &rec.normal) > 0.0 {
            outward_normal = rec.normal.neg();
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot_product(&ray.direction, &rec.normal) / ray.direction.magnitude();
        } else {
            outward_normal = rec.normal.copy();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot_product(&ray.direction, &rec.normal) / ray.direction.magnitude();
        }
        if refract(&ray.direction, &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }
        if rng.gen::<f32>() < reflect_prob {
            scattered.origin = rec.p.copy();
            scattered.direction = reflected.copy();
        } else {
            scattered.origin = rec.p.copy();
            scattered.direction = refracted.copy();
        }
        return true;
    }
    fn copy(&self) -> Box<dyn Material> {
        Box::new(Dielectric { ref_idx: self.ref_idx })
    }
}