use std::ops::{Add, Sub};

use glam::{IVec3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn of_block(block: IVec3) -> Self {
        let min = block.as_vec3();
        let max = min + Vec3::ONE;
        Self { min, max }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        let intersects_x = self.min.x < other.max.x && self.max.x > other.min.x;
        let intersects_y = self.min.y < other.max.y && self.max.y > other.min.y;
        let intersects_z = self.min.z < other.max.z && self.max.z > other.min.z;
        intersects_x && intersects_y && intersects_z
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
