use glam::{ivec2, ivec3};

use super::{block::Block, chunk::Chunk, subchunk::Subchunk};

use self::noise::PerlinNoise;

mod noise;
mod random;

pub fn generate_chunk(x: i32, z: i32) -> Chunk {
    let mut chunk = base_land_mass(x, z);
    add_grass(&mut chunk);
    chunk
}

fn base_land_mass(x: i32, z: i32) -> Chunk {
    let mut chunk = Chunk::new();
    let noise = PerlinNoise::new(ivec2(x, z), ivec3(1, 16, 1));

    for i in 0..16 {
        let mut subchunk = Subchunk::new();
        for x in 0..16 {
            for y in 0..16 {
                let y = y + i * 16;
                for z in 0..16 {
                    let p = ivec3(x, y, z).as_vec3() / 16.0;
                    let block = if noise.sample(p) > (y as f32 - 64.0) / 16.0 {
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

fn add_grass(chunk: &mut Chunk) {
    for x in 0..16 {
        for z in 0..16 {
            let mut y = 255;
            while y > 0 && chunk.block(x, y, z) == Block::AIR {
                y -= 1;
            }
            if y > 0 && chunk.block(x, y, z) == Block::STONE {
                chunk.set_block(x, y, z, Block::GRASS);
                y -= 1;
            }
            for _ in 0..3 {
                if y > 0 && chunk.block(x, y, z) == Block::STONE {
                    chunk.set_block(x, y, z, Block::DIRT);
                    y -= 1;
                }
            }
        }
    }
}

fn seed_for_chunk(x: i32, z: i32) -> u64 {
    (x as u64) ^ (z as u64) << 16
}
