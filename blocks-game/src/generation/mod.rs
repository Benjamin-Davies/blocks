use glam::{ivec2, ivec3};

use crate::{block::Block, chunk::Chunk, subchunk::Subchunk};

use self::noise::PerlinNoise;

mod noise;
mod random;

pub fn generate_chunk(x: i32, z: i32) -> Chunk {
    let mut chunk = Chunk::new();
    let noise = PerlinNoise::new(ivec2(x, z), ivec3(1, 16, 1));

    for i in 0..16 {
        let mut subchunk = Subchunk::new();
        for x in 0..16 {
            for y in 0..16 {
                let y = y + i * 16;
                for z in 0..16 {
                    let p = ivec3(x, y, z).as_vec3() / 16.0;
                    let block = if noise.sample(p) > (y as f32 - 64.0) / 64.0 {
                        Block::STONE
                    } else {
                        Block::AIR
                    };
                    subchunk.set_block(x as usize, y as usize % 16, z as usize, block);
                }
            }
        }
        chunk.subchunks.push(subchunk);
    }

    chunk
}

fn seed_for_chunk(x: i32, z: i32) -> u64 {
    (x as u64) ^ (z as u64) << 16
}
