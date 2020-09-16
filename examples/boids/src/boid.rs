use crate::linalg::Vector2D;
use crate::settings::Settings;
use rand::Rng;
use std::f64::consts::{FRAC_PI_3, PI};
use std::ops::{AddAssign, DivAssign};
use yew::{html, Html};

// at the time of writing the TAU constant is still unstable
const TAU: f64 = 2.0 * PI;
const FRAC_TAU_3: f64 = 2.0 * FRAC_PI_3;

struct Other<'a> {
    boid: &'a Boid,
    offset: Vector2D,
    distance: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Boid {
    position: Vector2D,
    velocity: Vector2D,
}

impl Boid {
    pub fn new_random(size: Vector2D, max_velocity: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: Vector2D::new(rng.gen::<f64>() * size.x, rng.gen::<f64>() * size.y),
            velocity: Vector2D::new_direction(rng.gen::<f64>() * TAU, max_velocity),
        }
    }

    fn iter_visible_boids<'it, 'item: 'it>(
        &'it self,
        settings: &'it Settings,
        boids: &'item [Self],
    ) -> impl Iterator<Item = Other<'item>> + Clone + 'it {
        boids
            .iter()
            .filter(move |boid| *boid != self)
            .filter_map(move |boid| {
                let offset = boid.position - self.position;
                let distance = offset.magnitude();

                if distance > settings.visible_range {
                    None
                } else {
                    Some(Other {
                        boid,
                        offset,
                        distance,
                    })
                }
            })
    }

    fn mean<T>(it: impl IntoIterator<Item = T>) -> T
    where
        T: AddAssign + Default + DivAssign<f64>,
    {
        let (mut avg, count) = it
            .into_iter()
            .fold((T::default(), 0), |(mut sum, count), value| {
                sum += value;
                (sum, count + 1)
            });
        if count > 0 {
            avg /= count as f64;
            avg
        } else {
            T::default()
        }
    }

    fn coherence<'a>(
        &self,
        visible_boids: impl Iterator<Item = Other<'a>>,
        factor: f64,
    ) -> Vector2D {
        let avg_pos = Self::mean(visible_boids.map(|other| other.boid.position));
        (avg_pos - self.position) * factor
    }

    fn separation<'a>(
        &self,
        visible_boids: impl Iterator<Item = Other<'a>>,
        min_distance: f64,
        factor: f64,
    ) -> Vector2D {
        let accel = visible_boids
            .filter_map(|other| {
                if other.distance > min_distance {
                    None
                } else {
                    Some(-other.offset)
                }
            })
            .sum::<Vector2D>();
        accel * factor
    }

    fn alignment<'a>(
        &self,
        visible_boids: impl Iterator<Item = Other<'a>>,
        factor: f64,
    ) -> Vector2D {
        let avg_vel = Self::mean(visible_boids.map(|other| other.boid.velocity));
        (avg_vel - self.velocity) * factor
    }

    fn keep_in_bounds(&self, min: Vector2D, max: Vector2D, turn_speed: f64) -> Vector2D {
        let mut v = Vector2D::default();

        let turn_speed = self.velocity.magnitude() * turn_speed;
        let pos = self.position;
        if pos.x < min.x {
            v.x += turn_speed;
        }
        if pos.x > max.x {
            v.x -= turn_speed
        }

        if pos.y < min.y {
            v.y += turn_speed;
        }
        if pos.y > max.y {
            v.y -= turn_speed;
        }

        v
    }

    fn update_velocity(&mut self, settings: &Settings, size: Vector2D, boids: &[Self]) {
        let visible_boids = self.iter_visible_boids(settings, boids);
        let v = self.velocity
            + self.coherence(visible_boids.clone(), settings.cohesion_factor)
            + self.separation(
                visible_boids.clone(),
                settings.min_distance,
                settings.separation_factor,
            )
            + self.alignment(visible_boids, settings.alignment_factor);
        // self.velocity = v.clamp_magnitude(settings.max_speed);
        self.velocity = v / v.magnitude() * settings.max_speed;

        let min = Vector2D::new(settings.border_margin, settings.border_margin);
        let max = size - min;
        self.velocity += self.keep_in_bounds(min, max, settings.turn_velocity_ratio);
    }

    pub fn update(&mut self, settings: &Settings, size: Vector2D, boids: &[Self]) {
        self.update_velocity(settings, size, boids);
        self.position += self.velocity;
    }

    pub fn render(&self) -> Html {
        const SIZE: f64 = 10.0;

        let mut points = String::new();
        for offset in triangle_offsets(SIZE, self.velocity.direction()) {
            let Vector2D { x, y } = self.position + offset;
            points.push_str(&format!("{},{} ", x, y));
        }

        html! { <polygon points=points /> }
    }
}

fn triangle_offsets(radius: f64, rotation: f64) -> impl Iterator<Item = Vector2D> {
    (0..3).map(move |i| Vector2D::new_direction(rotation + i as f64 * FRAC_TAU_3, radius))
}
