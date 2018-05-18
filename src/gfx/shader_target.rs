extern crate gl;

use cgmath::Deg;
use std::collections::HashMap;
use cgmath::{Matrix4, Vector3};

use gfx::uniforms::{Uniform, UniformValue};
use gfx::shader_program::ShaderProgram;
use gfx::mesh::Mesh;
use gfx::vertex::Vertex;
use gfx::camera::Camera;

#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z
}

pub struct ShaderTarget<'a, V: 'a + Vertex> {
    uniforms: HashMap<&'a str, Uniform>,
    shader_program: &'a ShaderProgram<'a, V>,
    model_matrix: Matrix4<f32>,
}

impl<'a, V> ShaderTarget<'a, V> where V: Vertex {
    pub fn create(shader_program: &'a ShaderProgram<'a, V>) -> Self {
        ShaderTarget {
            uniforms: HashMap::new(),
            shader_program: shader_program,
            model_matrix: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0))
        }
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    pub fn shader_program(&self) -> &ShaderProgram<V> {
        &self.shader_program
    }

    pub fn set_uniform_i32(&mut self, uniform_name: &'a str, uniform_value: i32) {
        if let Some(uniform) = self.uniforms.get_mut(uniform_name) {
            uniform.value = UniformValue::Int(uniform_value);
            return;
        }
        
        self.uniforms.insert(uniform_name, Uniform::new_i32(uniform_name, uniform_value));
    }

    pub fn set_uniform_f32(&mut self, uniform_name: &'a str, uniform_value: f32) {
        if let Some(uniform) = self.uniforms.get_mut(uniform_name) {
            uniform.value = UniformValue::Float(uniform_value);
            return;
        }
        
        self.uniforms.insert(uniform_name, Uniform::new_f32(uniform_name, uniform_value));
    }

    pub fn set_uniform_vec3(&mut self, uniform_name: &'a str, uniform_value: Vector3<f32>) {
        if let Some(uniform) = self.uniforms.get_mut(uniform_name) {
            uniform.value = UniformValue::Vector3(uniform_value);
            return;
        }
        
        self.uniforms.insert(uniform_name, Uniform::new(uniform_name, uniform_value.into()));
    }

    pub fn set_uniform_matrix4(&mut self, uniform_name: &'a str, uniform_value: Matrix4<f32>) {
        if let Some(uniform) = self.uniforms.get_mut(uniform_name) {
            uniform.value = UniformValue::Matrix4(uniform_value);
            return;
        }
        
        self.uniforms.insert(uniform_name, Uniform::new_matrix4(uniform_name, uniform_value));
    }

    pub fn update(&mut self, camera: &Camera) {
        let (projection, view, _) = camera.get_pvm();
        let model: Matrix4<f32> = self.model_matrix;

        self.set_uniform_matrix4("projection", projection);
        self.set_uniform_matrix4("view", view);
        self.set_uniform_matrix4("model", model);
    }

    pub fn render(&self, mesh: &Mesh<V>) {
        unsafe { gl::UseProgram(self.shader_program.id()); }

        for uniform in self.uniforms.values() {
            uniform.apply(&self.shader_program);
        }

        mesh.render(&self.shader_program);
    }

    pub fn compile(&mut self) {
        unsafe { (*(self.shader_program as *const ShaderProgram<V> as *mut ShaderProgram<V>)).compile(); }
    }

    pub fn set_pos(&mut self, model_matrix: Vector3<f32>) {
        self.model_matrix = Matrix4::from_translation(model_matrix);
    }

    pub fn set_rotation(&mut self, axis: Axis, deg: Deg<f32>) {
        match axis {
            Axis::X => {},
            Axis::Y => self.model_matrix = self.model_matrix * Matrix4::from_angle_y(deg),
            Axis::Z => {},
        }
    }
}
