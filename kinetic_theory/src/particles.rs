use crate::utils::approx_equal;
use crate::vec3::Vec3;
use itertools::izip;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::vec::Vec;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

/// 4th-order Yoshida constants, these were pre-calculated with Wolfram-Alpha
///
/// see Wikipedia: https://en.wikipedia.org/wiki/Leapfrog_integration#Yoshida_algorithms
const C14: f64 = 0.675603595979828817023843904485730413460999688108572414164;
const C23: f64 = -0.17560359597982881702384390448573041346099968810857241416;
const D13: f64 = 1.351207191959657634047687808971460826921999376217144828328;
const D2: f64 = -1.70241438391931526809537561794292165384399875243428965665;

/// Potential Strength Constants
const POTENTIAL_ATTRACTING: f64 = 1.0;
const POTENTIAL_REPELLING: f64 = 1.0;

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Particles {
    positions: Vec<Vec3>,
    velocities: Vec<Vec3>,
    masses: Vec<f64>,
    potential: Option<PyObject>,
}

#[pymethods]
#[allow(dead_code)]
impl Particles {

    #[new]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_potential(&mut self, potential: PyObject) {
        self.potential = Some(potential);
    }

    pub fn unset_potential(&mut self) {
        self.potential = None;
    }

    pub fn add_particle(&mut self, x: Vec3, v: Vec3, m: f64) {
        self.particle(x, v, m);
    }

    pub fn run(&mut self, n: usize, h: f64) {
        use crate::utils::timer;

        timer(n, || {
            self.update_yoshida(h);
            Ok(())
        });
    }

    pub fn update_yoshida(&mut self, h: f64) {
        self.update_yoshida_positions(C14, h);
        self.update_yoshida_velocities(D13, h);
        self.update_yoshida_positions(C23, h);
        self.update_yoshida_velocities(D2, h);
        self.update_yoshida_positions(C23, h);
        self.update_yoshida_velocities(D13, h);
        self.update_yoshida_positions(C14, h);
    }

    pub fn positions(&self) -> Vec<Vec3> {
        self.positions.clone()
    }

    pub fn velocities(&self) -> Vec<Vec3> {
        self.velocities.clone()
    }

    pub fn masses(&self) -> Vec<f64> {
        self.masses.clone()
    }
}

/// Non-Python Methods
impl Particles {
    /// This struct is its own Builder
    pub fn particle(&mut self, x: Vec3, v: Vec3, m: f64) -> &mut Self {
        self.positions.push(x);
        self.velocities.push(v);
        self.masses.push(m);

        self
    }

    fn update_yoshida_positions(&mut self, c: f64, h: f64) {
    self.positions = self
        .positions
        .iter()
        .zip(self.velocities.iter())
        .map(|(p, v)| p + c * v * h)
        .collect();
    }

    fn update_yoshida_velocities(&mut self, d: f64, h: f64) {
        let forces = self.forces();
        self.velocities = self
            .velocities
            .iter()
            .zip(self.positions.iter().zip(forces.iter()))
            .map(|(v, (p, f))| v + d * f * h)
            .collect();
    }

    fn forces(&self) -> Vec<Vec3> {

        let potentials: Vec<Vec3> = match &self.potential {
            None => vec![Vec3::default(); self.positions.len()],
            Some(pot) => {
                let gil = Python::acquire_gil();
                let python: Python<'_> = gil.python();
                self.positions
                    .iter()
                    .map(|p1| {
                    // convert the vector to a python object tuple
                    let arg: PyObject = p1.into_py(python);
                    let args = (arg,);

                    // call the potential python function with args
                    let call_result: PyResult<PyObject> = pot.call1(python, args);
                    let obj: PyObject = call_result.unwrap();
                    let vec: Vec3 = FromPyObject::extract(obj.as_ref(python)).unwrap();
                    vec
                }).collect()
            }
        };

        self.positions
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
                        let mut f = POTENTIAL_ATTRACTING / r_sq.powf(3.0) - POTENTIAL_REPELLING / r_sq.powf(6.0);

                        f = crate::utils::cap(f, -3.0, 3.0);

                        r.unit() * f
                    })
                    .sum::<Vec3>() + pot
            })
            .collect()
    }

    fn thomas_fermi(p: &Vec3) -> Vec3 {
        let kx = 1.0 / 100.0;
        let ky = 3.0 / 100.0;
        let kz = 1.0 / 100.0;

        -p.unit() * (kx * p.x.powf(2.0) + ky * p.y.powf(2.0) + kz * p.z.powf(2.0)) / 2.0
    }
}
