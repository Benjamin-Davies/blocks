use glam::{vec3, Quat, Vec2, Vec3};

const GRAVITY: f32 = 20.0;
const JUMP_VELOCITY: f32 = 8.0;
const WALK_SPEED: f32 = 5.0;

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
            position: vec3(0.5, 72.0, 0.5),
            ..Default::default()
        }
    }

    pub fn head_position(&self) -> Vec3 {
        self.position + vec3(0.0, 1.7, 0.0)
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

        self.collide_with_terrain();
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

    fn collide_with_terrain(&mut self) {
        self.on_ground = false;

        if self.position.y < 64.0 {
            self.position.y = 64.0;
            self.velocity.y = 0.0;

            self.on_ground = true;
        }
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.velocity.y = JUMP_VELOCITY;
        }
    }
}
