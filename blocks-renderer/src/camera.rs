use std::f32;

use glam::{Mat4, Quat, Vec3};

pub struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    pub aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            eye: 5.0 * Vec3::Z,
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.eye = rotation * self.eye;
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(
            self.fovy / 180.0 * f32::consts::PI,
            self.aspect,
            self.znear,
            self.zfar,
        );

        proj * view
    }
}
