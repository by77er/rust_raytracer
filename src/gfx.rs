#![allow(dead_code)]
use std::ops;

extern crate rand;
use rand::Rng;


#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr,  $z:expr) => {
        Vec3 {
            x: $x,
            y: $y,
            z: $z
        }
    };
}

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x 
        }
    }
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn as_unit(&self) -> Vec3 {
        let mag = self.magnitude();
        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, input: f32) -> Vec3 {
        Vec3 {
            x: self.x * input,
            y: self.y * input,
            z: self.z * input
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, input: f32) {
        self.x *= input;
        self.y *= input;
        self.z *= input;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, input: f32) -> Vec3 {
        Vec3 {
            x: self.x / input,
            y: self.y / input,
            z: self.z / input
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, input: f32) {
        self.x /= input;
        self.y /= input;
        self.z /= input;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of bounds: {}", index)
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bounds: {}", index)
        }
    }
}


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
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin.clone() + self.direction.clone() * t
    }
}

#[derive(Clone)]
pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3
}

// Calculates normal of generic object
pub trait Hitable {
    fn check_hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

#[derive(Clone)]
pub enum Shapes {
    Sphere(Sphere),
    HitableList(HitableList)
}

#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Shapes>
}

impl Hitable for HitableList {
    fn check_hit(&self, r: &Ray, _t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            t: 0.0,
            p: vec3!(0.0, 0.0, 0.0),
            normal: vec3!(0.0, 0.0, 0.0)
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for x in self.list.iter() {
            let ret = match x {
                Shapes::Sphere(v) => v.check_hit(r, 0.0, closest_so_far, &mut temp_rec),
                Shapes::HitableList(v)  => v.check_hit(r, 0.0, closest_so_far, &mut temp_rec),
            };
            if ret {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.normal = temp_rec.normal.clone();
                rec.p = temp_rec.p.clone();
                rec.t = temp_rec.t.clone();
            }
        }
        return hit_anything;
    }
}

impl Hitable for Sphere {
    fn check_hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin.clone() - self.center.clone();
        let a  = r.direction.dot(r.direction.clone());
        let b  = oc.dot(r.direction.clone());
        let c  = oc.dot(oc.clone()) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t.clone());
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t.clone());
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                return true;
            }
        }
        false
    }
}

pub fn random_unit_in_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();
    loop {
        p = vec3!(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - vec3!(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}

pub fn ray_color(r: &Ray, world: Shapes) -> Vec3 {
    let mut rec = HitRecord {
        t: 0.0,
        p: vec3!(0.0, 0.0, 0.0),
        normal: vec3!(0.0, 0.0, 0.0)
    };
    let ret = match world.clone() {
        Shapes::Sphere(v) => v.check_hit(r, 0.001, std::f32::MAX, &mut rec),
        Shapes::HitableList(v)  => v.check_hit(r, 0.0, std::f32::MAX, &mut rec)
    };
    if ret {
        let target = rec.p.clone() + rec.normal.clone() + random_unit_in_sphere();
        // vec3!(rec.normal.clone().x + 1.0, rec.normal.clone().y + 1.0, rec.normal.clone().z + 1.0) * 0.5
        ray_color(&Ray::new(rec.p.clone(), target - rec.p.clone()), world) * 0.5
    } else {
        let unit_direction = r.direction.as_unit();
        let t = (unit_direction.y + 1.0) * 0.5;
        vec3!(1.0, 1.0, 1.0) * (1.0 - t) + vec3!(0.5, 0.7, 1.0) * t
    }
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: vec3!(-2.0, -1.0, -1.0),
            horizontal: vec3!(4.0, 0.0, 0.0),
            vertical: vec3!(0.0, 2.0, 0.0),
            origin: vec3!(0.0, 0.0, 0.0)
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin.clone(), self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v)
    }
}