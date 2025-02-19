use std::ops::{Add, Mul, Sub};

use glam::{ivec3, vec3, IVec2, IVec3, Vec3};

use super::{random::Random, seed_for_chunk};

pub struct PerlinNoise {
    points: Vec<Vec3>,
    scale: IVec3,
}

impl PerlinNoise {
    pub fn new(chunk_pos: IVec2, scale: IVec3) -> Self {
        let mut rand00 = Random::with_seed(seed_for_chunk(chunk_pos.x, chunk_pos.y));
        let mut rand01 = Random::with_seed(seed_for_chunk(chunk_pos.x + 1, chunk_pos.y));
        let mut rand10 = Random::with_seed(seed_for_chunk(chunk_pos.x, chunk_pos.y + 1));
        let mut rand11 = Random::with_seed(seed_for_chunk(chunk_pos.x + 1, chunk_pos.y + 1));

        let mut points = Vec::new();
        for z in 0..2 * scale.z {
            for _y in 0..2 * scale.y {
                for x in 0..2 * scale.x {
                    let rand = match (x >= scale.x, z >= scale.z) {
                        (false, false) => &mut rand00,
                        (true, false) => &mut rand01,
                        (false, true) => &mut rand10,
                        (true, true) => &mut rand11,
                    };
                    points.push(rand.next_normal_vec3());
                }
            }
        }

        PerlinNoise { points, scale }
    }

    fn vector(&self, cell: IVec3) -> Vec3 {
        let index = (cell.z * (2 * self.scale.y) + cell.y) * (2 * self.scale.x) + cell.x;
        self.points[index as usize]
    }

    pub fn sample(&self, p: Vec3) -> f32 {
        let cell = p.floor().as_ivec3();
        let p = p - cell.as_vec3();
        let u0 = self.vector(cell + ivec3(0, 0, 0));
        let u1 = self.vector(cell + ivec3(1, 0, 0));
        let u2 = self.vector(cell + ivec3(0, 1, 0));
        let u3 = self.vector(cell + ivec3(1, 1, 0));
        let u4 = self.vector(cell + ivec3(0, 0, 1));
        let u5 = self.vector(cell + ivec3(1, 0, 1));
        let u6 = self.vector(cell + ivec3(0, 1, 1));
        let u7 = self.vector(cell + ivec3(1, 1, 1));

        let v0 = p - vec3(0.0, 0.0, 0.0);
        let d0 = v0.dot(u0);
        let v1 = p - vec3(1.0, 0.0, 0.0);
        let d1 = v1.dot(u1);
        let v2 = p - vec3(0.0, 1.0, 0.0);
        let d2 = v2.dot(u2);
        let v3 = p - vec3(1.0, 1.0, 0.0);
        let d3 = v3.dot(u3);
        let v4 = p - vec3(0.0, 0.0, 1.0);
        let d4 = v4.dot(u4);
        let v5 = p - vec3(1.0, 0.0, 1.0);
        let d5 = v5.dot(u5);
        let v6 = p - vec3(0.0, 1.0, 1.0);
        let d6 = v6.dot(u6);
        let v7 = p - vec3(1.0, 1.0, 1.0);
        let d7 = v7.dot(u7);

        let u = smoothstep(p.x);
        let v = smoothstep(p.y);
        let w = smoothstep(p.z);

        let d01 = lerp(u, d0, d1);
        let d23 = lerp(u, d2, d3);
        let d45 = lerp(u, d4, d5);
        let d67 = lerp(u, d6, d7);

        let d0123 = lerp(v, d01, d23);
        let d4567 = lerp(v, d45, d67);

        lerp(w, d0123, d4567)
    }
}

fn lerp<T>(t: f32, a: T, b: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
    f32: Mul<T, Output = T>,
{
    a + t * (b - a)
}

fn smoothstep(t: f32) -> f32 {
    // The unique cubic polynomial that satisfies the requirements of smoothstep.
    t * t * (3.0 - 2.0 * t)
}
