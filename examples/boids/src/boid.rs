use crate::vector::Vector;
use rand::Rng;
use std::f64::consts::PI;

const HEIGHT: f64 = 400.0;
const WIDTH: f64 = 600.0;
const VELOCITY_SIZE: f64 = 5.0;
const ALIGNMENT_RADIUS: f64 = 100.0;
const ALIGNMENT_WEIGHT: f64 = 3.0;
const COHESION_RADIUS: f64 = 200.0;
const COHESION_WEIGHT: f64 = 1.0;
const SEPARATION_RADIUS: f64 = 50.0;
const SEPARATION_WEIGHT: f64 = 1.0;

#[derive(Clone, PartialEq, Eq)]
pub struct Boid {
    pub position: Vector,
    pub velocity: Vector,
}

impl Boid {
    pub fn new(rng: &mut rand::rngs::ThreadRng) -> Boid {
        let theta = rng.gen::<f64>() * PI * 2.0;
        Boid {
            position: Vector::new(WIDTH * rng.gen::<f64>(), HEIGHT * rng.gen::<f64>()),
            velocity: Vector::new(theta.cos() * VELOCITY_SIZE, theta.sin() * VELOCITY_SIZE),
        }
    }

    fn calc_alignment(&self, boids: &[Boid]) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for other in boids {
            let mut position = other.position.clone();
            position -= self.position.clone();
            let position_size = position.size();
            if position_size == 0.0 || position_size > ALIGNMENT_RADIUS {
                continue;
            }

            ret += other.velocity.clone();
        }

        ret.normalize();
        ret *= ALIGNMENT_WEIGHT;
        ret
    }

    fn calc_cohesion(&self, boids: &[Boid]) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for other in boids {
            let mut position = other.position.clone();
            position -= self.position.clone();
            let position_size = position.size();
            if position_size == 0.0 || position_size > COHESION_RADIUS {
                continue;
            }

            ret += position;
        }

        ret.normalize();
        ret *= COHESION_WEIGHT;
        ret
    }

    fn calc_separation(&self, boids: &[Boid]) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for other in boids {
            let mut position = other.position.clone();
            position -= self.position.clone();
            let position_size = position.size();
            if position_size == 0.0 || position_size > SEPARATION_RADIUS {
                continue;
            }

            position /= position.size_square();
            ret -= position;
        }

        ret.normalize();
        ret *= SEPARATION_WEIGHT;
        ret
    }

    fn move_self(&mut self) {
        self.position += self.velocity.clone();
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
        acceleration += self.calc_separation(boids);
        acceleration += self.calc_cohesion(boids);
        acceleration += self.calc_alignment(boids);
        self.velocity += acceleration;
        self.velocity.normalize();
        self.velocity *= VELOCITY_SIZE;

        self.move_self();
    }
}
