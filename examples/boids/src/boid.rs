use std::fmt::Write;
use std::iter;

use rand::Rng;
use yew::{html, Html};

use crate::math::{self, Mean, Vector2D, WeightedMean};
use crate::settings::Settings;
use crate::simulation::SIZE;

#[derive(Clone, Debug, PartialEq)]
pub struct Boid {
    position: Vector2D,
    velocity: Vector2D,
    radius: f64,
    hue: f64,
}

impl Boid {
    pub fn new_random(settings: &Settings) -> Self {
        let mut rng = rand::thread_rng();

        let max_radius = settings.min_distance / 2.0;
        let min_radius = max_radius / 6.0;
        // by using the third power large boids become rarer
        let radius = min_radius + rng.gen::<f64>().powi(3) * (max_radius - min_radius);

        Self {
            position: Vector2D::new(rng.gen::<f64>() * SIZE.x, rng.gen::<f64>() * SIZE.y),
            velocity: Vector2D::from_polar(rng.gen::<f64>() * math::TAU, settings.max_speed),
            radius,
            hue: rng.gen::<f64>() * math::TAU,
        }
    }

    fn coherence(&self, boids: VisibleBoidIter, factor: f64) -> Vector2D {
        Vector2D::weighted_mean(
            boids.map(|other| (other.boid.position, other.boid.radius * other.boid.radius)),
        )
        .map(|mean| (mean - self.position) * factor)
        .unwrap_or_default()
    }

    fn separation(&self, boids: VisibleBoidIter, settings: &Settings) -> Vector2D {
        let accel = boids
            .filter_map(|other| {
                if other.distance > settings.min_distance {
                    None
                } else {
                    Some(-other.offset)
                }
            })
            .sum::<Vector2D>();
        accel * settings.separation_factor
    }

    fn alignment(&self, boids: VisibleBoidIter, factor: f64) -> Vector2D {
        Vector2D::mean(boids.map(|other| other.boid.velocity))
            .map(|mean| (mean - self.velocity) * factor)
            .unwrap_or_default()
    }

    fn adapt_color(&mut self, boids: VisibleBoidIter, factor: f64) {
        let mean = f64::mean(boids.filter_map(|other| {
            if other.boid.radius > self.radius {
                Some(math::smallest_angle_between(self.hue, other.boid.hue))
            } else {
                None
            }
        }));
        if let Some(avg_hue_offset) = mean {
            self.hue += avg_hue_offset * factor;
        }
    }

    fn keep_in_bounds(&mut self, settings: &Settings) {
        let min = SIZE * settings.border_margin;
        let max = SIZE - min;

        let mut v = Vector2D::default();

        let turn_speed = self.velocity.magnitude() * settings.turn_speed_ratio;
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

        self.velocity += v;
    }

    fn update_velocity(&mut self, settings: &Settings, boids: VisibleBoidIter) {
        let v = self.velocity
            + self.coherence(boids.clone(), settings.cohesion_factor)
            + self.separation(boids.clone(), settings)
            + self.alignment(boids, settings.alignment_factor);
        self.velocity = v.clamp_magnitude(settings.max_speed);
    }

    fn update(&mut self, settings: &Settings, boids: VisibleBoidIter) {
        self.adapt_color(boids.clone(), settings.color_adapt_factor);
        self.update_velocity(settings, boids);
        self.keep_in_bounds(settings);
        self.position += self.velocity;
    }

    pub fn update_all(settings: &Settings, boids: &mut [Self]) {
        for i in 0..boids.len() {
            let (before, after) = boids.split_at_mut(i);
            let (boid, after) = after.split_first_mut().unwrap();
            let visible_boids =
                VisibleBoidIter::new(before, after, boid.position, settings.visible_range);

            boid.update(settings, visible_boids);
        }
    }

    pub fn render(&self) -> Html {
        let color = format!("hsl({:.3}rad, 100%, 50%)", self.hue);

        let mut points = String::new();
        for offset in iter_shape_points(self.radius, self.velocity.angle()) {
            let Vector2D { x, y } = self.position + offset;

            // Write to string will never fail.
            let _ = write!(points, "{x:.2},{y:.2} ");
        }

        html! { <polygon {points} fill={color} /> }
    }
}

fn iter_shape_points(radius: f64, rotation: f64) -> impl Iterator<Item = Vector2D> {
    const SHAPE: [(f64, f64); 3] = [
        (0. * math::FRAC_TAU_3, 2.0),
        (1. * math::FRAC_TAU_3, 1.0),
        (2. * math::FRAC_TAU_3, 1.0),
    ];
    SHAPE
        .iter()
        .copied()
        .map(move |(angle, radius_mul)| Vector2D::from_polar(angle + rotation, radius_mul * radius))
}

#[derive(Debug)]
struct VisibleBoid<'a> {
    boid: &'a Boid,
    offset: Vector2D,
    distance: f64,
}

#[derive(Clone, Debug)]
struct VisibleBoidIter<'boid> {
    // Pay no mind to this mess of a type.
    // It's just `before` and `after` joined together.
    it: iter::Chain<std::slice::Iter<'boid, Boid>, std::slice::Iter<'boid, Boid>>,
    position: Vector2D,
    visible_range: f64,
}
impl<'boid> VisibleBoidIter<'boid> {
    fn new(
        before: &'boid [Boid],
        after: &'boid [Boid],
        position: Vector2D,
        visible_range: f64,
    ) -> Self {
        Self {
            it: before.iter().chain(after),
            position,
            visible_range,
        }
    }
}
impl<'boid> Iterator for VisibleBoidIter<'boid> {
    type Item = VisibleBoid<'boid>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            ref mut it,
            position,
            visible_range,
        } = *self;

        it.find_map(move |other| {
            let offset = other.position - position;
            let distance = offset.magnitude();

            if distance > visible_range {
                None
            } else {
                Some(VisibleBoid {
                    boid: other,
                    offset,
                    distance,
                })
            }
        })
    }
}
