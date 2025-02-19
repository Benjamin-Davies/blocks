use glam::{ivec3, IVec3};

use crate::bounding_box::BoundingBox;

use super::block::Block;

#[derive(Debug, Clone)]
pub struct Subchunk {
    pub blocks: [[[Block; Self::SIZE]; Self::SIZE]; Self::SIZE],
    pub dirty: bool,
}

impl Subchunk {
    pub const SIZE: usize = 16;

    pub fn new() -> Self {
        Self {
            blocks: [[[Block::AIR; Self::SIZE]; Self::SIZE]; Self::SIZE],
            dirty: true,
        }
    }

    pub fn block(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[z][y][x]
    }

    pub fn block_or_air(&self, x: isize, y: isize, z: isize) -> Block {
        if x < 0
            || y < 0
            || z < 0
            || x >= Self::SIZE as isize
            || y >= Self::SIZE as isize
            || z >= Self::SIZE as isize
        {
            return Block::AIR;
        }
        self.blocks[z as usize][y as usize][x as usize]
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[z][y][x] = block;
    }

    pub fn blocks_intersecting(
        &self,
        bounding_box: BoundingBox,
    ) -> impl Iterator<Item = (IVec3, Block)> + '_ {
        let min_block_x = bounding_box.min.x.floor() as i32;
        let min_block_x = min_block_x.clamp(0, Self::SIZE as i32);
        let max_block_x = bounding_box.max.x.ceil() as i32;
        let max_block_x = max_block_x.clamp(0, Self::SIZE as i32);
        let min_block_y = bounding_box.min.y.floor() as i32;
        let min_block_y = min_block_y.clamp(0, Self::SIZE as i32);
        let max_block_y = bounding_box.max.y.ceil() as i32;
        let max_block_y = max_block_y.clamp(0, Self::SIZE as i32);
        let min_block_z = bounding_box.min.z.floor() as i32;
        let min_block_z = min_block_z.clamp(0, Self::SIZE as i32);
        let max_block_z = bounding_box.max.z.ceil() as i32;
        let max_block_z = max_block_z.clamp(0, Self::SIZE as i32);

        (min_block_x..max_block_x).flat_map(move |x| {
            (min_block_y..max_block_y).flat_map(move |y| {
                (min_block_z..max_block_z).map(move |z| {
                    (
                        ivec3(x, y, z),
                        self.block(x as usize, y as usize, z as usize),
                    )
                })
            })
        })
    }
}

impl Default for Subchunk {
    fn default() -> Self {
        Self::new()
    }
}
