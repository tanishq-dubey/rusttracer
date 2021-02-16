use std::ops;

use crate::utils::{random_f64, random_f64_range};

#[derive(Copy, Clone, Debug)]
pub struct Vec3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl Vec3<f64> {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3<f64> {
        Vec3 {x, y, z}
    }

    pub fn new_random() -> Vec3<f64> {
        Vec3 {x: random_f64(), y: random_f64(), z: random_f64()}
    }

    pub fn new_random_range(min: f64, max: f64) -> Vec3<f64> {
        Vec3 {x: random_f64_range(min, max), y: random_f64_range(min, max), z: random_f64_range(min, max)}
    }

    pub fn new_random_in_unit_sphere() -> Vec3<f64> {
        loop {
            let p = Vec3::new_random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: Vec3<f64>) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3::new(self.y * rhs.z - self.z * rhs.y, 
                  self.z * rhs.x - self.x * rhs.z,
                  self.x * rhs.y - self.y * rhs.x)
    }

    pub fn unit_vector(self) -> Vec3<f64> {
        self / self.length()
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}


impl ops::Add<Vec3<f64>> for Vec3<f64> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vec3<f64>> for Vec3<f64> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vec3<f64> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }

}

impl ops::Div<f64> for Vec3<f64> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self * (1.0/rhs)
    }
}


pub type Point3 = Vec3<f64>;
pub type Color = Vec3<f64>;

