
use std::default::Default;
use serde::{Serialize, Deserialize};

use crate::vec3::Vec3;
use crate::utils::approx_equal;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f64,
}

impl Default for Particle {
    /// The default Particle has a mass of 1
    fn default() -> Self {
        Self {
            position: Vec3::default(),
            velocity: Vec3::default(),
            mass: 1.0,
        }
    }
}

impl Particle {
    /// Returns a new Particle with given parameters
    pub fn new(position: Vec3, velocity: Vec3, mass: f64) -> Self {
        Self{ position, velocity, mass }
    }

    /// Returns the state self would be in after performing a
    /// Kick-Drift-Kick Style Leaprfog position step
    ///
    /// # Arguments
    /// `h` - The time step in arbitrary units
    pub fn step_kdk_pos(self, h: f64) -> Self {
        Self {
            position: self.position + h * self.velocity,
            velocity: self.velocity,
            mass: self.mass
        }
    }

    /// Returns the state self would be in after performing a
    /// Kick-Drift-Kick Style Leapfrog velocity step
    ///
    /// # Arguments
    /// `h` - The time step in arbitrary units
    /// `force` - The total force acting on self
    pub fn step_kdk_vel(self, h: f64, force: Vec3) -> Self {
        Self {
            position: self.position,
            velocity: self.velocity + (h / 2.0) * force,
            mass: self.mass
        }
    }

    /// Returns the force acting between two particles
    ///
    /// # Arguments
    /// `other` - The other particle
    pub fn force_with(self, other: Particle) -> Vec3 {
        let r = other.position - self.position;
        let r_sq = r.abs_sq();

        if approx_equal(r_sq, 0.0) {
            // self is most likely other
            return Vec3::new(0.0, 0.0, 0.0);
        }

        // Lennard-Jones Potential
        let mut force = r * (1.0 / r_sq.powf(6.0) - 1.0 / r_sq.powf(3.0));

        // let max_force = 3.0;
        //
        // if force.abs_sq() > max_force {
        //     force = max_force * force.unit();
        // }

        force
    }

    /// Returns the total force acting on a particle in a system
    ///
    /// # Arguments
    /// `others` - All other particles in the system
    /// # Remarks
    /// It's fine if `self` is in `others` since we return zero force for that case
    pub fn force(self, others: &Vec<Particle>) -> Vec3 {
        others.iter().map(|p| self.force_with(*p)).sum()
    }
}

