use crate::vec3::Vec3;
use crate::cluster::cluster_grid3;
use crate::utils::approx_equal;
use std::slice::Iter;
use std::iter::Map;
use serde::{Serialize, Deserialize};

/// 4th-order Yoshida constants, these were pre-calculated with Wolfram-Alpha
///
/// see Wikipedia: https://en.wikipedia.org/wiki/Leapfrog_integration#Yoshida_algorithms
const C14: f64 = 0.675603595979828817023843904485730413460999688108572414164;
const C23: f64 = -0.17560359597982881702384390448573041346099968810857241416;
const D13: f64 = 1.351207191959657634047687808971460826921999376217144828328;
const D2: f64 = -1.70241438391931526809537561794292165384399875243428965665;


/// Data oriented clustered particle system.
///
/// Particles only exist implicitly, positions are always clustered.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPS {
    pub clustered_positions: Vec<Vec<Vec3>>,
    pub velocities: Vec<Vec3>,
    pub masses: Vec<f64>,
}

impl CPS {
    // pub fn new(positions: Vec<Vec3>, velocities: Vec<Vec3>, masses: Vec<f64>) -> Self {
    //     Self {
    //         clustered_positions: cluster_method(positions),
    //         velocities,
    //         masses,
    //     }
    // }
    //
    // pub fn step_kdk_leapfrog(&mut self, h: f64) {
    //     self.step_kdk_velocity(h);
    //     self.step_kdk_position(h);
    //     self.step_kdk_velocity(h);
    // }
    //
    // fn step_kdk_velocity(&mut self, h: f64) {
    //
    // }
    //
    // fn step_kdk_position(&mut self, h: f64) {
    //
    // }
    //
    // pub fn step_yoshida(&mut self, h: f64) {
    //     // see wikipedia: https://en.wikipedia.org/wiki/Leapfrog_integration#Yoshida_algorithms
    //     let c14 = 0.675603595979828817023843904485730413460999688108572414164;
    //     let c23 = 1.526810787939486451071531713457191240382999064325717242493;
    //     let d13 = 1.351207191959657634047687808971460826921999376217144828328;
    //     let d2 = -1.70241438391931526809537561794292165384399875243428965665;
    //
    //
    // }


    pub fn new(positions: Vec<Vec3>, velocities: Vec<Vec3>, masses: Vec<f64>) -> Self {

    }



}





/// Data oriented Particle System.
///
/// Particles only exist implicitly.
pub struct DPS {
    pub positions: Vec<Vec3>,
    pub velocities: Vec<Vec3>,
    pub masses: Vec<f64>,
}


impl DPS {
    pub fn new(positions: Vec<Vec3>, velocities: Vec<Vec3>, masses: Vec<f64>) -> Self {
        Self {
            positions,
            velocities,
            masses
        }
    }

    pub fn step_yoshida(&mut self, h: f64) {
        self.step_yoshida_positions(C14, h);
        self.step_yoshida_velocities(D13, h);
        self.step_yoshida_positions(C23, h);
        self.step_yoshida_velocities(D2, h);
        self.step_yoshida_positions(C23, h);
        self.step_yoshida_velocities(D13, h);
        self.step_yoshida_positions(C14, h);
    }

    fn step_yoshida_positions(&mut self, c: f64, h: f64) {
        self.positions = self.positions.iter().zip(self.velocities.iter()).map(|(p, v)| p + c * v * h).collect();
    }

    fn step_yoshida_velocities(&mut self, d: f64, h: f64) {
        let forces = self.forces();
        self.velocities = self.velocities.iter().zip(self.positions.iter()).zip(forces).map(|((v, p), f)| v + d * f * h).collect();
    }

    fn forces(&self) -> Map<Iter<Vec3>, fn(Vec3) -> Vec3> {
        // Vec of all forces
        self.positions.iter().map(|p1| -> Vec3 {
            // total force on particle at position p1
            self.positions.iter().map(|p2| -> Vec3 {
                // single force between particle at position p1 and particle at position p2
                let r = p2 - p1;
                let r_sq = r.abs_sq();

                if approx_equal(r_sq, 0.0) {
                    return Vec3::new(0.0, 0.0, 0.0);
                }
                // Lennard-Jones Potential
                r * (1.0 / r_sq.powf(6.0) - 1.0 / r_sq.powf(3.0))
            }).sum()
        })
    }
}

