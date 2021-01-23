use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew_services::storage::{Area, StorageService};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    /// amount of boids
    pub boids: usize,
    // time between each simulation tick
    pub tick_interval_ms: u64,
    /// view distance of a boid
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
    pub turn_speed_ratio: f64,
    /// percentage of the size to the boundary at which a boid starts turning away
    pub border_margin: f64,
    /// factor for adapting the average color of the swarm
    pub color_adapt_factor: f64,
}
impl Settings {
    const KEY: &'static str = "yew.boids.settings";

    pub fn load() -> Self {
        StorageService::new(Area::Local)
            .ok()
            .and_then(|storage| {
                storage
                    .restore::<Json<anyhow::Result<Settings>>>(Self::KEY)
                    .0
                    .ok()
            })
            .unwrap_or_default()
    }

    pub fn remove() {
        if let Ok(mut storage) = StorageService::new(Area::Local) {
            storage.remove(Self::KEY);
        }
    }

    pub fn store(&self) {
        if let Ok(mut storage) = StorageService::new(Area::Local) {
            storage.store(Self::KEY, Json(self))
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            boids: 300,
            tick_interval_ms: 50,
            visible_range: 80.0,
            min_distance: 15.0,
            max_speed: 20.0,
            alignment_factor: 0.15,
            cohesion_factor: 0.05,
            separation_factor: 0.6,
            turn_speed_ratio: 0.25,
            border_margin: 0.1,
            color_adapt_factor: 0.05,
        }
    }
}
