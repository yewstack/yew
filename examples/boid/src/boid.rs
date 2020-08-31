use crate::vector::Vector;
use crate::rand::Rand;
use std::f64::consts::PI;

const HEIGHT: f64 = 400.0;
const WIDTH: f64 = 600.0;
const VELOCITY_SIZE: f64 = 5.0;
const ALIGNMENT_RADIOUS: f64 = 50.0;
const ALIGNMENT_WEIGHT: f64 = 2.0;
const COHENSION_RADIOUS: f64 = 100.0;
const COHENSION_WEIGHT: f64 = 2.0;
const SEPARATION_RADIOUS: f64 = 20.0;
const SEPRATION_WEIGHT: f64 = 2.0;

#[derive(Clone, PartialEq, Eq)]
pub struct Boid {
    pub position: Vector,
    pub velocity: Vector,
}

impl Boid {
    pub fn new(rng: &mut Rand) -> Boid {
        let theta = rng.next_f64() * PI * 2.0;
        Boid {
            position: Vector::new(WIDTH * rng.next_f64(), HEIGHT * rng.next_f64()),
            velocity: Vector::new(theta.cos() * VELOCITY_SIZE, theta.sin() * VELOCITY_SIZE),
        }
    }

    fn calc_alignment(&self, boids: &Vec<Boid>) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for other in boids {
            let mut position = other.position.clone();
            position.sub(&self.position);
            let position_size = position.size();
            if position_size == 0.0 || position_size > ALIGNMENT_RADIOUS {
                continue;
            }

            let mut velocity = other.velocity.clone();
            velocity.sub(&self.velocity);
            ret.add(&velocity);
        }

        ret.normalize();
        ret.mul(ALIGNMENT_WEIGHT);
        ret
    }

    fn calc_cohension(&self, boids: &Vec<Boid>) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for other in boids {
            let mut position = other.position.clone();
            position.sub(&self.position);
            let position_size = position.size();
            if position_size == 0.0 || position_size > COHENSION_RADIOUS {
                continue;
            }

            ret.add(&position);
        }

        ret.normalize();
        ret.mul(COHENSION_WEIGHT);
        ret
    }

    fn calc_separation(&self, boids: &Vec<Boid>) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for other in boids {
            let mut position = other.position.clone();
            position.sub(&self.position);
            let position_size = position.size();
            if position_size == 0.0 || position_size > SEPARATION_RADIOUS {
                continue;
            }

            ret.sub(&position);
        }

        ret.normalize();
        ret.mul(SEPRATION_WEIGHT);
        ret
    }
}
