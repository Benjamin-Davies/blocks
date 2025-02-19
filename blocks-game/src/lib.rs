use std::collections::BTreeMap;

use generation::generate_chunk;
use glam::{ivec3, IVec3};

use crate::{chunk::Chunk, player::Player, subchunk::Subchunk};

pub mod block;
pub mod chunk;
pub mod generation;
pub mod player;
pub mod subchunk;

pub struct Game {
    pub player: Player,
    pub chunks: BTreeMap<(i32, i32), Chunk>,
}

impl Game {
    pub fn new() -> Self {
        let mut chunks = BTreeMap::new();
        for x in -1..=1 {
            for z in -1..=1 {
                chunks.insert((x, z), generate_chunk(x, z));
            }
        }

        Self {
            player: Player::new(),
            chunks,
        }
    }

    pub fn subchunks_mut(&mut self) -> impl Iterator<Item = (IVec3, &mut Subchunk)> {
        self.chunks
            .iter_mut()
            .flat_map(|(&(chunk_x, chunk_z), chunk)| {
                chunk
                    .subchunks
                    .iter_mut()
                    .enumerate()
                    .map(move |(subchunk_y, subchunk)| {
                        (ivec3(chunk_x, subchunk_y as i32, chunk_z), subchunk)
                    })
            })
    }

    pub fn dirty_subchunks_mut(&mut self) -> impl Iterator<Item = (IVec3, &mut Subchunk)> {
        self.subchunks_mut().filter(|(_, subchunk)| subchunk.dirty)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.player.update(delta_time);
    }
}
