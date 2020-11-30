
use std::default::Default;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;

use crate::particle::Particle;
use crate::vec3::Vec3;
use std::io::Write;
use std::slice::Iter;
use std::iter::Map;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl Default for ParticleSystem {
    /// The default ParticleSystem contains no particles
    fn default() -> Self {
        Self{
            particles: Vec::new()
        }
    }
}

impl ParticleSystem {
    /// Returns a new ParticleSystem with the given particles
    ///
    /// # Arguments
    /// `particles` - A collection of particles to initialize with
    pub fn new(particles: Vec<Particle>) -> Self {
        Self { particles }
    }

    /// Returns the number of particles currently present in the system
    pub fn num_particles(&self) -> usize {
        self.particles.len()
    }

    /// Performs a full Kick-Drift-Kick Style Leapfrog step
    ///
    /// # Arguments
    /// `h` - The time step in arbitrary units
    pub fn step_kdk_leapfrog(&mut self, h: f64) {
        self.step_kdk_velocity(h);
        self.step_kdk_position(h);
        self.step_kdk_velocity(h);
    }

    /// Performs a Kick-Drift-Kick Style Leapfrog velocity step
    ///
    /// # Arguments
    /// `h` - The time step in arbitrary units
    fn step_kdk_velocity(&mut self, h: f64) {
        // calculate the total force on each particle
        let forces: Vec<Vec3> = self.particles
            .iter()
            .map(|p| p.force(&self.particles))
            .collect();
        // overwrite the particles with new particles,
        // each having performed its velocity step
        // with the force calculated above
        self.particles = self.particles
            .iter()
            .zip(forces.iter())
            .map(|pf| pf.0.step_kdk_vel(h, *pf.1))
            .collect();
    }

    /// Performs a Kick-Drift-Kick Style Leapfrog position step
    ///
    /// # Arguments
    /// `h` - The time step in arbitrary units
    fn step_kdk_position(&mut self, h: f64) {
        // overwrite the particles with new particles,
        // each having performed its position step
        self.particles = self.particles
            .iter()
            .map(|p| p.step_kdk_pos(h))
            .collect();
    }

    /// Returns an iterator over particle positions
    pub fn positions(&self) -> Map<Iter<Particle>, fn(&Particle) -> Vec3> {
        self.particles.iter().map(|p| p.position)
    }

    /// Returns an iterator over particle velocities
    pub fn velocities(&self) -> Map<Iter<Particle>, fn(&Particle) -> Vec3> {
        self.particles.iter().map(|p| p.velocity)
    }

    /// Returns an iterator over particle masses
    pub fn masses(&self) -> Map<Iter<Particle>, fn(&Particle) -> f64> {
        self.particles.iter().map(|p| p.mass)
    }

    /// Serializes the struct into a json file
    ///
    /// # Arguments
    /// * `path` - The target file path
    pub fn save_json(&self, path: &std::path::Path) -> Result<(), String> {
        // serialize the content into a string
        let contents = serde_json::to_string(self)
        .map_err(|e| e.to_string())?;

        // open the file
        // erase it if it was previously present
        // otherwise create it
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        // write file contents
        file.write_all(contents.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}