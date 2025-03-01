use std::{collections::BTreeMap, i32};

use block::Block;
use glam::{ivec3, vec3, IVec3};

use self::{chunk::Chunk, generation::generate_chunk, subchunk::Subchunk};
use crate::bounding_box::BoundingBox;

pub mod block;
pub mod chunk;
pub mod subchunk;

mod generation;

pub struct Terrain {
    pub chunks: BTreeMap<(i32, i32), Chunk>,
}

impl Terrain {
    pub fn new() -> Self {
        let mut chunks = BTreeMap::new();
        for x in -1..=1 {
            for z in -1..=1 {
                chunks.insert((x, z), generate_chunk(x, z));
            }
        }

        Self { chunks }
    }

    pub fn block(&self, block_pos: IVec3) -> Block {
        let subchunk_x = block_pos.x.div_euclid(Subchunk::SIZE as i32);
        let subchunk_y = block_pos.y.div_euclid(Subchunk::SIZE as i32);
        let subchunk_z = block_pos.z.div_euclid(Subchunk::SIZE as i32);

        self.chunks
            .get(&(subchunk_x, subchunk_z))
            .map_or(Block::default(), |chunk| {
                chunk
                    .subchunks
                    .get(subchunk_y as usize)
                    .map_or(Block::default(), |subchunk| {
                        subchunk.block(
                            block_pos.x.rem_euclid(Subchunk::SIZE as i32) as usize,
                            block_pos.y.rem_euclid(Subchunk::SIZE as i32) as usize,
                            block_pos.z.rem_euclid(Subchunk::SIZE as i32) as usize,
                        )
                    })
            })
    }

    pub fn subchunk_exists(&self, subchunk_pos: IVec3) -> bool {
        self.chunks
            .get(&(subchunk_pos.x, subchunk_pos.z))
            .is_some_and(|c| (subchunk_pos.y as usize) < c.subchunks.len())
    }

    pub fn subchunk_mut(&mut self, subchunk_pos: IVec3) -> Option<&mut Subchunk> {
        self.chunks
            .get_mut(&(subchunk_pos.x, subchunk_pos.z))
            .and_then(|c| c.subchunks.get_mut(subchunk_pos.y as usize))
    }

    pub fn blocks_intersecting(
        &self,
        bounding_box: BoundingBox,
    ) -> impl Iterator<Item = (IVec3, block::Block)> + '_ {
        self.chunks_intersecting(bounding_box)
            .flat_map(move |(chunk_x, chunk_z, chunk)| {
                chunk
                    .blocks_intersecting(
                        bounding_box - 16.0 * vec3(chunk_x as f32, 0.0, chunk_z as f32),
                    )
                    .map(move |(p, block)| {
                        (ivec3(16 * chunk_x + p.x, p.y, 16 * chunk_z + p.z), block)
                    })
            })
    }

    fn chunks_intersecting(
        &self,
        bounding_box: BoundingBox,
    ) -> impl Iterator<Item = (i32, i32, &Chunk)> + '_ {
        let min_chunk_x = (bounding_box.min.x.floor() as i32).div_euclid(Subchunk::SIZE as i32);
        let max_chunk_x = (bounding_box.max.x.floor() as i32).div_euclid(Subchunk::SIZE as i32);
        let min_chunk_z = (bounding_box.min.z.floor() as i32).div_euclid(Subchunk::SIZE as i32);
        let max_chunk_z = (bounding_box.max.z.floor() as i32).div_euclid(Subchunk::SIZE as i32);

        (min_chunk_x..=max_chunk_x)
            .flat_map(move |x| self.chunks.range((x, min_chunk_z)..=(x, max_chunk_z)))
            .map(move |(&(x, z), chunk)| (x, z, chunk))
    }
}
