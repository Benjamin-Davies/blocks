use crate::subchunk::Subchunk;

pub struct Chunk {
    pub subchunks: Vec<Subchunk>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            subchunks: (0..16)
                .map(|_| {
                    let mut subchunk = Subchunk::new();
                    subchunk.add_sphere();
                    subchunk.add_dirt();
                    subchunk
                })
                .collect(),
        }
    }
}
