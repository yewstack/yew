#[derive(Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    pub fn add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub(&mut self, other: &Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn mul(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn div(&mut self, scalar: f64) {
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
