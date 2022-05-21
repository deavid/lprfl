#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn vector_to(&self, other: &Self) -> Vector {
        Vector {
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector {
    pub dx: f32,
    pub dy: f32,
}

impl Vector {
    pub fn distance_2(&self) -> f32 {
        self.dx.powi(2) + self.dy.powi(2)
    }
    pub fn distance(&self) -> f32 {
        self.distance_2().sqrt()
    }
    pub fn scale(&self, s: f32) -> Vector {
        Vector {
            dx: self.dx * s,
            dy: self.dy * s,
        }
    }
}
