use std::ops::{Add, Sub};

use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
}

impl Add<Vec3> for BoundingBox {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl Sub<Vec3> for BoundingBox {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            min: self.min - rhs,
            max: self.max - rhs,
        }
    }
}
