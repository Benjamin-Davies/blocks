use glam::{vec3, Vec3};

/// Random number generator using the same algorithm as the Java `Random` class.
///
/// See also: https://docs.oracle.com/javase/8/docs/api/java/util/Random.html
pub struct Random {
    seed: u64,
}

impl Random {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed: seed ^ 0x5_DEEC_E66D & 0xFFFF_FFFF_FFFF,
        }
    }

    fn next_bits(&mut self, bits: u32) -> u32 {
        self.seed = (self.seed.wrapping_mul(0x5_DEEC_E66D) + 0xB) & 0xFFFF_FFFF_FFFF;
        (self.seed >> (48 - bits)) as u32
    }

    pub fn next_float(&mut self) -> f32 {
        self.next_bits(24) as f32 / (1 << 24) as f32
    }

    pub fn next_normal_vec3(&mut self) -> Vec3 {
        vec3(
            self.next_float() * 2.0 - 1.0,
            self.next_float() * 2.0 - 1.0,
            self.next_float() * 2.0 - 1.0,
        )
        .normalize()
    }
}
