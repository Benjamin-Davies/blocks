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

    pub fn add_sphere(&mut self) {
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                for z in 0..Self::SIZE {
                    let dx = x as f32 + 0.5 - 8.0;
                    let dy = y as f32 + 0.5 - 8.0;
                    let dz = z as f32 + 0.5 - 8.0;
                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                    if distance < 8.0 {
                        self.set_block(x, y, z, Block::STONE);
                    }
                }
            }
        }
    }

    pub fn add_dirt(&mut self) {
        for x in 0..Self::SIZE {
            for z in 0..Self::SIZE {
                let mut depth = 0;
                for y in (0..Self::SIZE).rev() {
                    match self.block(x, y, z) {
                        Block::AIR => depth = 0,
                        Block::STONE => {
                            if depth == 0 {
                                self.set_block(x, y, z, Block::GRASS);
                            } else if depth < 3 {
                                self.set_block(x, y, z, Block::DIRT);
                            }
                            depth += 1;
                        }
                        _ => depth += 1,
                    }
                }
            }
        }
    }
}

impl Default for Subchunk {
    fn default() -> Self {
        Self::new()
    }
}
