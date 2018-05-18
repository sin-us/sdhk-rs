extern crate gl;

use cgmath::Vector3;
use glfw::Key;
use gfx::camera::Camera;

pub trait RenderTarget {
    fn update(&mut self, camera: &Camera, time: f32);
    fn process_key_pressed(&mut self, key: Key);
    fn render(&self);
    fn compile(&mut self);
    fn set_pos(&mut self, pos: Vector3<f32>);
}