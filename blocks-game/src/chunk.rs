use crate::{block::Block, subchunk::Subchunk};

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
}
