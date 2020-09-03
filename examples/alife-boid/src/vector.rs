use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    fn div(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }

    pub fn size(&mut self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let size = self.size();
        if size == 0.0 {
            return;
        }
        self.div(size)
    }
}

impl Eq for Vector {}
