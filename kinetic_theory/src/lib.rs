#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod particles;
pub mod utils;
pub mod vec3;
mod prelude;

use pyo3::prelude::*;
pub use crate::prelude::*;

use rayon::prelude::*;

/// Build the Python Module for this crate
#[pymodule(particles)]
fn build_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {

    // rayon::ThreadPoolBuilder::new().num_threads(6).build_global().unwrap();

    m.add_class::<vec3::Vec3>()?;
    m.add_class::<particles::Particles>()?;

    Ok(())
}
