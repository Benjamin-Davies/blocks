use std::f32;

use blocks_game::player::Player;
use glam::{Mat4, Vec3};

pub struct Camera {
    eye: Vec3,
    dir: Vec3,
    up: Vec3,
    pub aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraBufferContents {
    view_projection: [f32; 4 * 4],
    position: [f32; 3],
    aspect: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            eye: Vec3::ZERO,
            dir: Vec3::Z,
            up: Vec3::Y,
            aspect,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn update(&mut self, player: &Player) {
        self.eye = player.head_position();
        self.dir = player.looking_direction();
        self.up = player.up_direction();
    }

    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_to_rh(self.eye, self.dir, self.up);
        let proj = Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);

        proj * view
    }

    pub fn buffer_contents(&self) -> CameraBufferContents {
        CameraBufferContents {
            view_projection: self.build_view_projection_matrix().to_cols_array(),
            position: self.eye.to_array(),
            aspect: self.aspect,
        }
    }
}
