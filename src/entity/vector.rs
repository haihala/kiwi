use std::ops::AddAssign;

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn square_magnitude(&self) -> f32 {
        // Non-square rooted magnitude is sufficient for most things.
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn magnitude(&self) -> f32 {
        self.square_magnitude().sqrt()
    }

    pub fn resize(&mut self, new_lenght: f32) -> f32 {
        let old_lenght = self.square_magnitude();
        if old_lenght.is_normal() {
            let ratio: f32 = new_lenght/old_lenght;
            self.x *= ratio;
            self.y *= ratio;
            return new_lenght;
        }
        0.0
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector{x: 0.0, y: 0.0}
    }
}

