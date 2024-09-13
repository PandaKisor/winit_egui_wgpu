// camera.rs

use glam::{Mat4, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub speed: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, speed: f32) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y, // Default up direction
            speed,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn move_forward(&mut self) {
        let direction = (self.target - self.position).normalize();
        self.position += direction * self.speed;
        self.target += direction * self.speed;
    }

    pub fn move_backward(&mut self) {
        let direction = (self.target - self.position).normalize();
        self.position -= direction * self.speed;
        self.target -= direction * self.speed;
    }

    pub fn strafe_left(&mut self) {
        let direction = (self.target - self.position).normalize();
        let strafe_direction = direction.cross(self.up).normalize();
        self.position -= strafe_direction * self.speed;
        self.target -= strafe_direction * self.speed;
    }

    pub fn strafe_right(&mut self) {
        let direction = (self.target - self.position).normalize();
        let strafe_direction = direction.cross(self.up).normalize();
        self.position += strafe_direction * self.speed;
        self.target += strafe_direction * self.speed;
    }
}
