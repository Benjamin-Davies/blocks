use glam::{vec3, IVec3, Quat, Vec2, Vec3};

use crate::{
    bounding_box::BoundingBox,
    terrain::{block::Block, Terrain},
    util::TotalOrd,
};

const GRAVITY: f32 = 20.0;
const JUMP_VELOCITY: f32 = 10.0;
const WALK_SPEED: f32 = 5.0;
/// The amount of overlap past which a collision will not be resolved.
const OVERLAP_THRESHOLD: f32 = 0.5;

#[derive(Default)]
pub struct Player {
    pub position: Vec3,
    pub head_angle: Vec2,
    pub velocity: Vec3,
    pub on_ground: bool,
    pub walk_vector: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 72.0, 0.0),
            ..Default::default()
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            self.position + vec3(-0.875, 0.0, -0.875),
            self.position + vec3(0.875, 2.75, 0.875),
        )
    }

    pub fn head_position(&self) -> Vec3 {
        self.position + vec3(0.0, 2.5, 0.0)
    }

    pub fn looking_direction(&self) -> Vec3 {
        let pitch = self.head_angle.x.to_radians();
        let yaw = self.head_angle.y.to_radians();
        vec3(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            yaw.cos() * pitch.cos(),
        )
    }

    pub fn up_direction(&self) -> Vec3 {
        let pitch = self.head_angle.x.to_radians();
        let yaw = self.head_angle.y.to_radians();
        vec3(
            yaw.sin() * -pitch.sin(),
            pitch.cos(),
            yaw.cos() * -pitch.sin(),
        )
    }

    pub fn update(&mut self, delta_time: f32) {
        self.constrain_head_angle();

        let walk_rotation = Quat::from_rotation_y(self.head_angle.y.to_radians());
        let walk_velocity = walk_rotation * self.walk_vector * WALK_SPEED;
        self.velocity.x = walk_velocity.x;
        self.velocity.z = walk_velocity.z;

        self.velocity.y -= GRAVITY * delta_time;

        self.position += self.velocity * delta_time;
    }

    fn constrain_head_angle(&mut self) {
        if self.head_angle.x > 90.0 {
            self.head_angle.x = 90.0;
        } else if self.head_angle.x < -90.0 {
            self.head_angle.x = -90.0;
        }

        self.head_angle.y %= 360.0;
        if self.head_angle.y > 180.0 {
            self.head_angle.y -= 360.0;
        } else if self.head_angle.y <= -180.0 {
            self.head_angle.y += 360.0;
        }
    }

    pub fn collide_with_terrain(&mut self, terrain: &Terrain) {
        self.on_ground = false;

        for (block_pos, _) in terrain
            .blocks_intersecting(self.bounding_box())
            .filter(|&(_, b)| b != Block::AIR)
        {
            self.collide_with_block(block_pos, terrain);
        }
    }

    /// Moves the player by the smallest amount necessary to not collide with
    /// the block at `block_pos`. This method assumes that the player currently
    /// intersects the block.
    fn collide_with_block(&mut self, block_pos: IVec3, terrain: &Terrain) {
        let block = BoundingBox::new(block_pos.as_vec3(), block_pos.as_vec3() + Vec3::ONE);
        let player = self.bounding_box();

        // Top, bottom, etc. are faces of the block
        let west = block.max.x - player.min.x;
        let east = player.max.x - block.min.x;
        let top = block.max.y - player.min.y;
        let bottom = player.max.y - block.min.y;
        let north = block.max.z - player.min.z;
        let south = player.max.z - block.min.z;

        if let Some((depth, direction)) = [
            (west, Vec3::X),
            (east, -Vec3::X),
            (top, Vec3::Y),
            (bottom, -Vec3::Y),
            (north, Vec3::Z),
            (south, -Vec3::Z),
        ]
        .into_iter()
        .filter(|&(d, _)| d < OVERLAP_THRESHOLD)
        .filter(|(_, v)| terrain.block(block_pos + v.as_ivec3()) == Block::AIR)
        .min_by_key(|&(d, _)| TotalOrd(d))
        {
            self.position += depth * direction;

            let normal_velocity = self.velocity.dot(direction);
            if normal_velocity < 0.0 {
                self.velocity -= normal_velocity * direction;
            }

            if direction == Vec3::Y {
                self.on_ground = true;
            }
        }
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.on_ground = false;
            self.velocity.y = JUMP_VELOCITY;
        }
    }
}
