use glam::{ivec3, vec3, IVec3};

use crate::bounding_box::BoundingBox;

use super::{block::Block, subchunk::Subchunk};

pub struct Chunk {
    pub subchunks: Vec<Subchunk>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            subchunks: Vec::new(),
        }
    }

    pub fn block(&self, x: usize, y: usize, z: usize) -> Block {
        let subchunk = y / Subchunk::SIZE;
        let y = y % Subchunk::SIZE;
        self.subchunks[subchunk].block(x, y, z)
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        let subchunk = y / Subchunk::SIZE;
        let y = y % Subchunk::SIZE;
        self.subchunks[subchunk].set_block(x, y, z, block);
    }

    pub fn blocks_intersecting(
        &self,
        bounding_box: BoundingBox,
    ) -> impl Iterator<Item = (IVec3, Block)> + '_ {
        self.subchunks_intersecting(bounding_box)
            .flat_map(move |(subchunk_y, subchunk)| {
                subchunk
                    .blocks_intersecting(bounding_box - 16.0 * vec3(0.0, subchunk_y as f32, 0.0))
                    .map(move |(p, block)| (ivec3(p.x, 16 * subchunk_y + p.y, p.z), block))
            })
    }

    fn subchunks_intersecting(
        &self,
        bounding_box: BoundingBox,
    ) -> impl Iterator<Item = (i32, &Subchunk)> + '_ {
        let min_subchunk_y = (bounding_box.min.y.floor() as i32).div_euclid(Subchunk::SIZE as i32);
        let max_subchunk_y = (bounding_box.max.y.floor() as i32).div_euclid(Subchunk::SIZE as i32);

        (min_subchunk_y..=max_subchunk_y)
            .filter(|y| (0..self.subchunks.len() as i32).contains(&y))
            .map(|y| (y, &self.subchunks[y as usize]))
    }
}
