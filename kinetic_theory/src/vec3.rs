use serde::{Deserialize, Serialize};
use std::default::Default;
use std::ops::{Div, DivAssign, Mul, MulAssign};

use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use pyo3::prelude::*;

#[pyclass]
#[derive(
    Debug, Copy, Clone, PartialEq, Default, Neg, Add, AddAssign, Sub, SubAssign, Serialize, Deserialize,
)]
pub struct Vec3 {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
    #[pyo3(get, set)]
    pub z: f64,
}

#[pymethods]
impl Vec3 {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn abs_sq(&self) -> f64 {
        // we need self as reference because it is implicitly wrapped in a PyRef
        self * self
    }

    pub fn abs(&self) -> f64 {
        self.abs_sq().sqrt()
    }

    pub fn unit(&self) -> Self {
        self / self.abs()
    }
}

// impl<'source> FromPyObject<'source> for Vec3 {
//     fn extract(obj: &'source PyObjectRef) -> PyResult<Self> {
//         let gil = Python::acquire_gil();
//         let py = gil.python();
//
//         Ok(
//             Self {
//                 x: obj.getattr(py, "x")?.extract(py)?,
//                 y: obj.getattr(py, "y")?.extract(py)?,
//                 z: obj.getattr(py, "z")?.extract(py)?
//             }
//         )
//     }
// }

// impl IntoPy<PyObject> for Vec3 {
//     fn into_py(self, py: Python<'_>) -> PyObject {
//         Py::new(py, self).unwrap()
//     }
// }

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        -*self
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        *self + *other
    }
}

impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        self + *other
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        *self + other
    }
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        *self - *other
    }
}

impl std::ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        self - *other
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        *self - other
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: &Self) -> f64 {
        self * *other
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = f64;

    fn mul(self, other: Vec3) -> f64 {
        *self * other
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = f64;

    fn mul(self, other: &Vec3) -> f64 {
        *self * *other
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        *self * other
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        self * *other
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        *self / other
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut result = Vec3::new(0.0, 0.0, 0.0);
        for v in iter {
            result += v;
        }
        result
    }
}
