use glam::{Mat4, Vec2, Vec3};
use winit::dpi::PhysicalSize;

pub struct Camera {
    pub focus_position: Vec2,
    pub zoom: f32,
    pub window_size: PhysicalSize<u32>,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let left = self.focus_position.x - self.window_size.width as f32 / 2.;
        let right = self.focus_position.x + self.window_size.width as f32 / 2.;
        let top = self.focus_position.y - self.window_size.height as f32 / 2.;
        let bottom = self.focus_position.y + self.window_size.height as f32 / 2.;

        let orth = Mat4::orthographic_rh(left, right, bottom, top, 0., 1.);
        let zoom = Mat4::from_scale(Vec3::splat(self.zoom));

        orth * zoom
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: Mat4,
}

impl CameraUniform {
    pub fn new(camera: &Camera, window_size: &PhysicalSize<u32>) -> Self {
        Self {
            view_proj: camera.build_view_projection_matrix(),
        }
    }
}

pub struct CameraController {
    speed: f32,
    direction: Vec2,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            direction: Vec2::ZERO,
        }
    }
}
