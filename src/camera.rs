use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],  // Combined view-projection matrix
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        let view = camera.view_matrix();
        let proj = Mat4::perspective_rh(camera.config.fovy, camera.config.aspect, camera.config.znear, camera.config.zfar);
        let view_proj = proj * view;
        self.view_proj = view_proj.to_cols_array_2d();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CameraConfig {
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub speed: f32,
}

impl CameraConfig {
    pub fn new(aspect: f32, fovy: f32, znear: f32, zfar: f32, speed: f32) -> Self {
        Self { aspect, fovy, znear, zfar, speed }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub yaw: f32,   // Rotation around the Y axis (left/right)
    pub pitch: f32, // Rotation around the X axis (up/down)
    pub config: CameraConfig,
}

impl Camera {
    pub fn new(
        position: Vec3,
        yaw: f32,
        pitch: f32,
        config: CameraConfig,
        up: Vec3,
    ) -> Self {
        let target = position + Vec3::new(yaw.cos(), pitch.sin(), yaw.sin()).normalize();
        Self {
            position,
            yaw,
            pitch,
            config,
            target,
            up,
        }
    }

    /// Returns the view matrix of this [`Camera`].
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Moves the camera based on a direction and speed.
    pub fn move_camera(&mut self, direction: Vec3, speed: f32) {
        let normalized_dir = direction.normalize();
        self.position += normalized_dir * speed;
        self.target += normalized_dir * speed;
    }

    pub fn move_forward(&mut self) {
        let forward_dir = (self.target - self.position).normalize();
        self.move_camera(forward_dir, self.config.speed);
    }

    pub fn move_backward(&mut self) {
        let backward_dir = (self.position - self.target).normalize();
        self.move_camera(backward_dir, self.config.speed);
    }

    pub fn strafe_left(&mut self) {
        let forward_dir = (self.target - self.position).normalize();
        let strafe_dir = forward_dir.cross(self.up).normalize();
        self.move_camera(-strafe_dir, self.config.speed);
    }

    pub fn strafe_right(&mut self) {
        let forward_dir = (self.target - self.position).normalize();
        let strafe_dir = forward_dir.cross(self.up).normalize();
        self.move_camera(strafe_dir, self.config.speed);
    }

    /// Zoom in by reducing the field of view (fovy).
    pub fn zoom_in(&mut self) {
        self.config.fovy -= 0.1;  // Adjust the value as necessary
        if self.config.fovy < 0.1 {
            self.config.fovy = 0.1;
        }
    }

    /// Zoom out by increasing the field of view (fovy).
    pub fn zoom_out(&mut self) {
        self.config.fovy += 0.1;
        if self.config.fovy > 3.14 {
            self.config.fovy = 3.14;
        }
    }
}
