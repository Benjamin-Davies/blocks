use glam::{vec3, Quat, Vec2, Vec3};

const WALK_SPEED: f32 = 5.0;

#[derive(Default)]
pub struct Player {
    pub position: Vec3,
    pub head_angle: Vec2,
    pub walk_vector: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: vec3(0.5, 16.0, 0.5),
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

    pub fn update(&mut self, delta_time: f32) {
        let walk_direction = self.walk_vector * WALK_SPEED * delta_time;
        let walk_rotation = Quat::from_rotation_y(self.head_angle.y.to_radians());
        self.position += walk_rotation * walk_direction;
    }
}
