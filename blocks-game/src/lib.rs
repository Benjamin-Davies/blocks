use crate::{player::Player, terrain::Terrain};

pub mod bounding_box;
pub mod player;
pub mod terrain;
pub mod util;

const MAX_DELTA_TIME: f32 = 0.03;

pub struct Game {
    pub player: Player,
    pub terrain: Terrain,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            terrain: Terrain::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let delta_time = delta_time.min(MAX_DELTA_TIME);

        self.terrain.generate(self.player.position.as_ivec3());

        self.player.update(delta_time);
        self.player.collide_with_terrain(&self.terrain);
    }
}
