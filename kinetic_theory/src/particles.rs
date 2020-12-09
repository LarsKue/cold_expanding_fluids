/// This is the actual Particle Simulation Class file

use crate::utils::approx_equal;
use crate::vec3::Vec3;
use itertools::izip;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::vec::Vec;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

// Tell PyO3 to make this class accessible from Python
#[pyclass]
// Tell Rust to automatically generate the Debug, Clone and Default Trait
#[derive(Debug, Clone, Default)]
/// This struct represents an N-Particle Simulation
// it uses a data-oriented layout, individual Particles exist only implicitly
pub struct Particles {
    // Each Particle has a position, velocity and mass
    positions: Vec<Vec3>,
    velocities: Vec<Vec3>,
    masses: Vec<f64>,
    // This is the optionally given external Potential
    potential: Option<PyObject>,
}

// These are Python-exposed methods
#[pymethods]
// We don't want compiler warnings for when these methods are not used in Rust Code,
// they are intended to be called from Python
#[allow(dead_code)]
impl Particles {

    /// This is the Python constructor for the class.
    #[new]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the external potential to some Python Function.
    ///
    /// # Arguments
    ///
    /// * `potential` - A Python Callable that calculates the external
    /// vectorial potential acting on a particle at some point in space.
    ///
    /// # Examples
    ///
    /// Python:
    /// ```python
    /// def potential(v):
    ///     return -v
    ///
    /// particles.set_potential(potential)
    /// ```
    ///
    /// Typed Rust Analog:
    /// ```rust
    /// use crate::vec3::Vec3;
    /// fn potential(v: Vec3) -> Vec3 {
    ///     -v
    /// }
    /// ```
    ///
    pub fn set_potential(&mut self, potential: PyObject) {
        self.potential = Some(potential);
    }

    /// Check if there is an external potential set
    pub fn has_potential(&self) -> bool {
        self.potential.is_some()
    }

    /// Unset the external potential.
    pub fn unset_potential(&mut self) {
        self.potential = None;
    }

    /// Add a particle to the Simulation.
    ///
    /// # Arguments
    ///
    /// * `x` - The [`Vec3`](../vec3/struct.Vec3.html) describing the position of the particle.
    /// * `v` - The [`Vec3`](../vec3/struct.Vec3.html) describing the velocity of the particle.
    /// * `m` - A `float` describing the mass of the particle.
    ///
    /// # Examples
    ///
    /// Python:
    /// ```python
    /// particles.add_particle(
    ///     Vec3(0.0, 0.0, 0.0),
    ///     Vec3(0.0, 0.0, 0.0),
    ///     1.0
    /// )
    /// ```
    ///
    pub fn add_particle(&mut self, x: Vec3, v: Vec3, m: f64) {
        self.particle(x, v, m);
    }


    /// Run the simulation with the specified number of steps and a fixed time step.
    /// This method is preferred to manually updating in a Python `for`-loop, since
    /// it prevents swapping between Rust and Python Execution in every step.
    ///
    ///
    /// # Arguments
    ///
    /// * `n` - The number of time steps to perform
    /// * `h` - The fixed size of the time step
    ///
    pub fn run(&mut self, n: usize, h: f64) -> PyResult<()> {
        for _ in 0..n {
            self.update_yoshida(h)?;
        }

        Ok(())
    }

    // hidden doc until this function is actually implemented
    // currently using the utils::timer function conflicts with the python gil,
    // which can never be acquired from a threaded context
    // but acquiring the gil is necessary to call the python-defined potential
    #[doc(hidden)]
    /// Run the simulation and display a continually updating timer.
    /// See also:
    /// - [run](#method.run)
    /// - [utils::timer](../utils/fn.timer.html)
    ///
    pub fn run_timer(&mut self, n: usize, h: f64) -> PyResult<()> {
        unimplemented!()
    }

    /// Update the state of the simulation by performing a single time step of given size.
    /// The state will be integrated using the
    /// [Yoshida Leapfrog Algorithm](https://en.wikipedia.org/wiki/Leapfrog_integration#Yoshida_algorithms).
    ///
    /// This method is preferred if you have to record the state of the simulation every step
    /// or if you have to manually alter it in some way.
    /// If you want to run multiple steps in
    /// succession *without* doing the above, use [run](#method.run).
    ///
    /// # Arguments
    ///
    /// * `h` - The size of the time step
    ///
    pub fn update_yoshida(&mut self, h: f64) -> PyResult<()> {
        use crate::constants::yoshida::{C14, D13, C23, D2};

        self.update_yoshida_positions(C14, h);
        self.update_yoshida_velocities(D13, h)?;
        self.update_yoshida_positions(C23, h);
        self.update_yoshida_velocities(D2, h)?;
        self.update_yoshida_positions(C23, h);
        self.update_yoshida_velocities(D13, h)?;
        self.update_yoshida_positions(C14, h);

        Ok(())
    }

    /// Query the positions of all particles in the simulation.
    /// Positions are returned in the same ordering as particles were originally defined.
    pub fn positions(&self) -> Vec<Vec3> {
        self.positions.clone()
    }

    /// Analogous to [positions](#method.positions).
    pub fn velocities(&self) -> Vec<Vec3> {
        self.velocities.clone()
    }

    /// Analogous to [positions](#method.positions).
    pub fn masses(&self) -> Vec<f64> {
        self.masses.clone()
    }

    /// Query the number of particles currently present in the simulation
    pub fn num_particles(&self) -> usize {
        self.positions.len()
    }
}

// Non-Python (Rust-only) Methods
// do not create docs to avoid confusing python-only users
#[doc(hidden)]
impl Particles {

    #[doc(hidden)]
    pub fn particle(&mut self, x: Vec3, v: Vec3, m: f64) -> &mut Self {
        // this struct is its own builder
        self.positions.push(x);
        self.velocities.push(v);
        self.masses.push(m);

        self
    }

    #[doc(hidden)]
    fn update_yoshida_positions(&mut self, c: f64, h: f64) {
    self.positions = self
        .positions
        .iter()
        .zip(self.velocities.iter())
        .map(|(p, v)| p + c * v * h)
        .collect();
    }

    #[doc(hidden)]
    fn update_yoshida_velocities(&mut self, d: f64, h: f64) -> PyResult<()> {
        let forces = self.forces()?;
        self.velocities = self
            .velocities
            .iter()
            .zip(forces.iter())
            .map(|(v, f)| v + d * f * h)
            .collect();
        Ok(())
    }


    fn potentials(&self) -> PyResult<Vec<Vec3>> {
        match &self.potential {
            None => Ok(vec![Vec3::default(); self.positions.len()]),
            Some(pot) => {
                // acquire the python global interpreter lock
                let gil = pyo3::Python::acquire_gil();
                // acquire the respective python instance
                let python: Python<'_> = gil.python();
                self.positions
                    .iter()
                    .map(|p1| {
                        // convert the vector to a python object tuple
                        let arg: PyObject = p1.into_py(python);
                        let args = (arg,);

                        // call the potential python function with args
                        // if we encounter an error in Python, move the error up
                        let call_result: PyResult<PyObject> = pot.call1(python, args);
                        let obj: PyObject = call_result?;
                        let vec: Vec3 = FromPyObject::extract(obj.as_ref(python))?;

                        Ok(vec)
                    }).collect()
            }
        }
    }

    #[doc(hidden)]
    fn forces(&self) -> PyResult<Vec<Vec3>> {

        let potentials: Vec<Vec3> = self.potentials()?;

        let forces = self.positions
            .par_iter()
            .zip(potentials.par_iter())
            .map(|(p1, pot)| {
                // regular iter because the thread creation overhead
                // outweighs its benefit in the inner loop
                self.positions
                    .iter()
                    .map(|p2| {
                        let r = p2 - p1;
                        let r_sq = r.abs_sq();

                        if approx_equal(r_sq, 0.0) {
                            return Vec3::default();
                        }

                        // Lennard-Jones Potential
                        use crate::constants::potential::{ATTRACTING, REPELLING};
                        let mut f = ATTRACTING / r_sq.powf(3.0) - REPELLING / r_sq.powf(6.0);

                        f = crate::utils::cap(f, -3.0, 3.0);

                        r.unit() * f
                    })
                    .sum::<Vec3>() + pot
            })
            .collect();

        Ok(forces)
    }
}
