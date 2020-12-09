/// Constants for the Simulation are defined here.

pub mod yoshida {
    /// 4th-order Yoshida constants, these were pre-calculated with Wolfram-Alpha
    /// see Wikipedia: https://en.wikipedia.org/wiki/Leapfrog_integration#Yoshida_algorithms
    pub const C14: f64 = 0.675603595979828817023843904485730413460999688108572414164;
    pub const C23: f64 = -0.17560359597982881702384390448573041346099968810857241416;
    pub const D13: f64 = 1.351207191959657634047687808971460826921999376217144828328;
    pub const D2: f64 = -1.70241438391931526809537561794292165384399875243428965665;
}

pub mod potential {
    /// Potential Strength Constants in arbitrary units
    pub const ATTRACTING: f64 = 1.0;
    pub const REPELLING: f64 = 1.0;
}
