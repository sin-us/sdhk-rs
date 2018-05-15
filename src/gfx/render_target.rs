extern crate gl;

use cgmath::Vector3;
use gfx::camera::Camera;

pub trait RenderTarget {
    fn update(&mut self, camera: &Camera, time: f32);
    fn render(&self);
    fn compile(&mut self);
    fn set_pos(&mut self, pos: Vector3<f32>);
}