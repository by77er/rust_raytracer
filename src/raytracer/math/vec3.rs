#![allow(dead_code)]

// Represents a 3D vector
#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

// Instead of implementing the + - / etc operators, I'm using fns
// this is because doing otherwise requires much more copying than
// is reasonable

// I'm supplying two versions of each function: one that borrows and
// returns a newly allocated Vec3, and one that modifies the original
// Vec3 in place. I'll try to use the latter when possible for greater
// speed and memory efficiency
impl Vec3 {
    // Just create a new vector the usual way
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }
    // Create a new vector with x, y, and z set to the same value
    pub fn all(all: f32) -> Vec3 {
        Vec3 {
            x: all,
            y: all,
            z: all
        }
    }
    // Create a duplicate vector
    pub fn copy(&self) -> Vec3 {
        Vec3::new(
            self.x,
            self.y,
            self.z
        )
    }
    // Dot product of two vectors
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    // Cross product of two vectors
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x 
        }
    }
    // Halfway to magnitude
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    // Length of vector
    pub fn magnitude(&self) -> f32 {
        self.squared_length().sqrt()
    }
    // Creates a new unit vector corresponding to the calling Vec3
    pub fn as_unit(&self) -> Vec3 {
        let mag = self.magnitude();
        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }
    pub fn add(&self, num: f32) -> Vec3 {
        Vec3::new(self.x + num, self.y + num, self.z + num)
    }
    pub fn add_eq(&mut self, num: f32) {
        self.x += num;
        self.y += num;
        self.z += num;
    }
    pub fn add_by_vec(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    pub fn add_by_vec_eq(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    pub fn sub(&self, num: f32) -> Vec3 {
        Vec3::new(self.x - num, self.y - num, self.z - num)
    }
    pub fn sub_eq(&mut self, num: f32) {
        self.x -= num;
        self.y -= num;
        self.z -= num;
    }
    pub fn sub_by_vec(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    pub fn sub_by_vec_eq(&mut self, other: &Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
    pub fn div(&self, num: f32) -> Vec3 {
        Vec3::new(self.x / num, self.y / num, self.z / num)
    }
    pub fn div_eq(&mut self, num: f32) {
        self.x /= num;
        self.y /= num;
        self.z /= num;
    }
    pub fn div_by_vec(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
    pub fn div_by_vec_eq(&mut self, other: &Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
    pub fn mul(&self, num: f32) -> Vec3 {
        Vec3::new(self.x * num, self.y * num, self.z * num)
    }
    pub fn mul_eq(&mut self, num: f32) {
        self.x *= num;
        self.y *= num;
        self.z *= num;
    }
    pub fn mul_by_vec(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
    pub fn mul_by_vec_eq(&mut self, other: &Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
    pub fn neg(&self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
    pub fn neg_eq(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}

impl std::ops::Index<usize> for Vec3 {
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

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bounds: {}", index)
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

pub fn dot_product(vec1: &Vec3, vec2: &Vec3) -> f32 {
    vec1.dot(vec2)
}

pub fn cross_product(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
    vec1.cross(vec2)
}