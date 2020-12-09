#![warn(missing_debug_implementations, rust_2018_idioms)]
#[doc(hidden)]

// This is the crate's main library file
// Refer to the [Rust Documentation](https://doc.rust-lang.org/rust-by-example/crates/lib.html)


// Include files belonging to this library crate
pub mod particles;
pub mod utils;
pub mod vec3;
pub mod constants;
mod prelude;

// use statements shorten syntax, analogous to C++'s "using"
use pyo3::prelude::*;
use rayon::prelude::*;
pub use crate::prelude::*;



// Build the Python Module for this crate
#[pymodule(particles)]
fn build_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {

    // set the number of threads to be used by the program
    // set to None to use all threads and Some(n) to use n threads.
    let num_threads = None;

    if let Some(n) = num_threads {
        // actually set the number of threads, if applicable
        rayon::ThreadPoolBuilder::new().num_threads(n).build_global().unwrap();
    }

    // these build the python module, by telling PyO3 to create Python classes
    // the ? operator tells the current function to exit
    // with an error if an error happened in the preceding statement
    m.add_class::<vec3::Vec3>()?;
    m.add_class::<particles::Particles>()?;

    // if everything went fine, return the Ok Result
    Ok(())
}
