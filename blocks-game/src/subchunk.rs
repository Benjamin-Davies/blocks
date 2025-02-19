use crate::block::Block;

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
}

impl Default for Subchunk {
    fn default() -> Self {
        Self::new()
    }
}
