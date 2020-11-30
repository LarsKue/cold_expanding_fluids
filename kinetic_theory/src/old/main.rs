#![feature(drain_filter)]
#![feature(iterator_fold_self)]

mod cluster;
mod utils;
mod particle_system;
mod clustered_particle_system;
mod vec3;
mod particle;

use std::sync::{Arc, Mutex};
use std::io::Write;
use std::fs::OpenOptions;

use rand_distr::{Distribution, Normal};
use rand::thread_rng;

use crate::particle_system::ParticleSystem;
use crate::particle::Particle;
use crate::vec3::Vec3;


fn main() -> Result<(), String> {

    // create normal variate distribution
    let mut rng = thread_rng();
    let normal = Normal::new(0.0, 2.0)
        .expect("Could not create normal distribution.");

    const N_PARTICLES : usize = 1000;

    // initialize particles with normal variate positions in the xy-plane
    let particles = (0..N_PARTICLES).map(|_| {
        Particle::new(
            Vec3::new(normal.sample(&mut rng), normal.sample(&mut rng), 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            1.0
        )
    }).collect();

    // wee need to wrap ps in a mutex and arc to pass it to timer
    let data = Arc::new(Mutex::new(Vec::<Vec::<Vec3>>::new()));
    let data_clone = data.clone();
    let ps = Arc::new(Mutex::new(ParticleSystem::new(particles)));

    const N_STEPS: usize = 10000;
    const H: f64 = 0.001;

    // time execution
    utils::timer(N_STEPS, move || -> Result<(), String> {
        let mut ps_guard = ps.lock().map_err(|e| e.to_string())?;
        (*ps_guard).step_kdk_leapfrog(H);
        let positions: Vec<Vec3>= (*ps_guard.positions().collect::<Vec<Vec3>>()).to_owned();
        let mut data_guard = data.lock().map_err(|e| e.to_string())?;
        (*data_guard).push(positions);
        Ok(())
    })?;

    // save positions to a file
    let filename = format!("data_h={}.json", H);
    let path = std::path::Path::new(filename.as_str());

    let str;

    {
        let guard = data_clone.lock().map_err(|e| e.to_string())?;

        str = serde_json::to_string(&*guard).map_err(|e| e.to_string())?;
    }

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
    file.write_all(str.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}

