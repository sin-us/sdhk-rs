extern crate gl;

use std::ffi::CString;
use std::collections::HashMap;
use cgmath::{Matrix4, Vector3};
use gfx::shader_program::ShaderProgram;
use gfx::mesh::Mesh;
use gfx::vertex::Vertex;
use gfx::camera::Camera;




pub enum UniformValue {
    Float(f32),
    Vector3(Vector3<f32>),
    Matrix4(Matrix4<f32>)
}

impl From<Vector3<f32>> for UniformValue {
    fn from(value: Vector3<f32>) -> UniformValue {
        UniformValue::Vector3(value)
    }
}


pub struct Uniform {
    name: CString,
    value: UniformValue
}

impl Uniform {
    pub fn new(name: &str, value: UniformValue) -> Uniform {
        Uniform {
            name: CString::new(name).unwrap(),
            value: value
        }
    }

    pub fn new_f32(name: &str, value: f32) -> Uniform {
        Uniform {
            name: CString::new(name).unwrap(),
            value: UniformValue::Float(value)
        }
    }

    pub fn new_matrix4(name: &str, value: Matrix4<f32>) -> Uniform {
        Uniform {
            name: CString::new(name).unwrap(),
            value: UniformValue::Matrix4(value)
        }
    }

    pub fn apply(&self, shader_program: &ShaderProgram) {
        unsafe {
            let location = gl::GetUniformLocation(shader_program.id(), self.name.as_ptr() as *const i8);
            match self.value {
                UniformValue::Float(val) => gl::Uniform1f(location, val),
                UniformValue::Matrix4(val) => gl::UniformMatrix4fv(location, 1, gl::FALSE, &val as *const Matrix4<f32> as *const f32),
                UniformValue::Vector3(val) => gl::Uniform3fv(location, 1, &val as *const Vector3<f32> as *const f32)
            }
        }
    }
}



pub struct RenderableMesh<'a, V: 'a + Vertex> {
    mesh: Mesh<V>,
    uniforms: HashMap<&'a str, Uniform>,
    shader_program: &'a ShaderProgram<'a>,
    model_matrix: Matrix4<f32>,
}

impl<'a, V> RenderableMesh<'a, V> where V: Vertex {
    pub fn create(mesh: Mesh<V>, shader_program: &'a ShaderProgram<'a>) -> RenderableMesh<'a, V> {
        RenderableMesh {
            mesh: mesh,
            uniforms: HashMap::new(),
            shader_program: shader_program,
            model_matrix: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0))
        }
    }

    pub fn get_mesh(&self) -> &Mesh<V> {
        &self.mesh
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
}

pub trait RenderTarget {
    fn update(&mut self, camera: &Camera, time: f32);
    fn render(&self);
    fn compile(&mut self);
    fn set_pos(&mut self, pos: Vector3<f32>);
}

impl<'a, V> RenderTarget for RenderableMesh<'a, V>
            where V: Vertex {

    fn update(&mut self, camera: &Camera, time: f32) {
        self.set_uniform_f32("mixAmount", 0.5);

        let (projection, view, _) = camera.get_pvm();
        let model: Matrix4<f32> = self.model_matrix;

        self.set_uniform_matrix4("projection", projection);
        self.set_uniform_matrix4("view", view);
        self.set_uniform_matrix4("model", model);
    }

    fn render(&self) {
        unsafe {
            gl::UseProgram(self.shader_program.id());
        }
        for uniform in self.uniforms.values() {
            uniform.apply(&self.shader_program);
        }
        self.mesh.render(&self.shader_program);
    }

    fn compile(&mut self) {
        unsafe { (*(self.shader_program as *const ShaderProgram as *mut ShaderProgram)).compile(); }
        self.mesh.compile();
    }

    fn set_pos(&mut self, model_matrix: Vector3<f32>) {
        self.model_matrix = Matrix4::from_translation(model_matrix);
    }
}