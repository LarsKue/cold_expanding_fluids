
use std::default::Default;
use std::ops::{Mul, Div};
use serde::{Serialize, Deserialize};

use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Add, AddAssign, Sub, SubAssign, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn abs_sq(self) -> f64 {
        self * self
    }

    pub fn abs(self) -> f64 {
        self.abs_sq().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.abs()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}


impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item = Vec3>
    {
        let mut result = Vec3::new(0.0, 0.0, 0.0);
        for v in iter {
            result += v;
        }
        result
    }
}
