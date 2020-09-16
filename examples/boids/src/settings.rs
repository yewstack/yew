#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// amount of boids
    pub boids: usize,
    // time between each simulation tick
    pub tick_interval_ms: u64,
    /// how many units a boid can see
    pub visible_range: f64,
    /// distance boids try to keep between each other
    pub min_distance: f64,
    /// max speed
    pub max_speed: f64,
    /// force multiplier for pulling boids together
    pub cohesion_factor: f64,
    /// force multiplier for separating boids
    pub separation_factor: f64,
    /// force multiplier for matching velocity of other boids
    pub alignment_factor: f64,
    /// controls turn speed to avoid leaving boundary
    pub turn_velocity_ratio: f64,
    /// distance to the boundary before boid starts turning
    pub border_margin: f64,
}

const DEFAULT: Settings = Settings {
    boids: 500,
    tick_interval_ms: 50,
    visible_range: 50.0,
    min_distance: 10.0,
    max_speed: 15.0,
    alignment_factor: 0.15,
    cohesion_factor: 0.05,
    separation_factor: 0.5,
    turn_velocity_ratio: 0.5,
    border_margin: 50.0,
};

pub fn load() -> Settings {
    DEFAULT.clone()
}
