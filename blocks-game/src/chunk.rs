use crate::subchunk::Subchunk;

pub struct Chunk {
    pub subchunks: Vec<Subchunk>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            subchunks: Vec::new(),
        }
    }
}
