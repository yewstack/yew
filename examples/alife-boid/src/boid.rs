use crate::rand::Rand;
use crate::vector::Vector;
use std::f64::consts::PI;

const HEIGHT: f64 = 400.0;
const WIDTH: f64 = 600.0;
const VELOCITY_SIZE: f64 = 5.0;
const ALIGNMENT_RADIOUS: f64 = 100.0;
const ALIGNMENT_WEIGHT: f64 = 3.0;
const COHENSION_RADIOUS: f64 = 200.0;
const COHENSION_WEIGHT: f64 = 1.0;
const SEPARATION_RADIOUS: f64 = 50.0;
const SEPARATION_WEIGHT: f64 = 1.0;

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

    fn calc_alignment(&self, boids: &[Boid]) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for other in boids {
            let mut position = other.position.clone();
            position.sub(&self.position);
            let position_size = position.size();
            if position_size == 0.0 || position_size > ALIGNMENT_RADIOUS {
                continue;
            }

            /*
            let mut velocity = other.velocity.clone();
            velocity.sub(&self.velocity);
            */
            ret.add(&other.velocity);
        }

        ret.normalize();
        ret.mul(ALIGNMENT_WEIGHT);
        ret
    }

    fn calc_cohension(&self, boids: &[Boid]) -> Vector {
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

    fn calc_separation(&self, boids: &[Boid]) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for other in boids {
            let mut position = other.position.clone();
            position.sub(&self.position);
            let position_size = position.size();
            if position_size == 0.0 || position_size > SEPARATION_RADIOUS {
                continue;
            }

            let size = position.size();
            position.div(size * size);

            ret.sub(&position);
        }

        ret.normalize();
        ret.mul(SEPARATION_WEIGHT);
        ret
    }

    fn move_self(&mut self) {
        self.position.add(&self.velocity);
        if self.position.x < 0.0 {
            self.position.x += WIDTH;
        } else if self.position.x > WIDTH {
            self.position.x -= WIDTH;
        }

        if self.position.y < 0.0 {
            self.position.y += HEIGHT;
        } else if self.position.y > HEIGHT {
            self.position.y -= HEIGHT;
        }
    }

    pub fn next_state(&mut self, boids: &[Boid]) {
        let mut acceleration = Vector::new(0.0, 0.0);
        acceleration.add(&self.calc_separation(boids));
        acceleration.add(&self.calc_cohension(boids));
        acceleration.add(&self.calc_alignment(boids));
        self.velocity.add(&acceleration);
        self.velocity.normalize();
        self.velocity.mul(VELOCITY_SIZE);

        self.move_self();
    }
}
