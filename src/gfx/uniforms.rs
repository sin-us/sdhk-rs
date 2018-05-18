extern crate gl;

use gfx::vertex::Vertex;
use gfx::shader_program::ShaderProgram;
use std::ffi::CString;
use cgmath::{Vector3, Matrix4};

pub enum UniformValue {
    Int(i32),
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
    pub value: UniformValue
}

impl Uniform {
    pub fn new(name: &str, value: UniformValue) -> Uniform {
        Uniform {
            name: CString::new(name).unwrap(),
            value: value
        }
    }

    pub fn new_i32(name: &str, value: i32) -> Uniform {
        Self::new(name, UniformValue::Int(value))
    }

    pub fn new_f32(name: &str, value: f32) -> Uniform {
        Self::new(name, UniformValue::Float(value))
    }

    pub fn new_matrix4(name: &str, value: Matrix4<f32>) -> Uniform {
        Self::new(name, UniformValue::Matrix4(value))
    }

    pub fn apply<V>(&self, shader_program: &ShaderProgram<V>) where V: Vertex {
        unsafe {
            let location = gl::GetUniformLocation(shader_program.id(), self.name.as_ptr() as *const i8);
            match self.value {
                UniformValue::Int(val) => gl::Uniform1i(location, val),
                UniformValue::Float(val) => gl::Uniform1f(location, val),
                UniformValue::Matrix4(val) => gl::UniformMatrix4fv(location, 1, gl::FALSE, &val as *const Matrix4<f32> as *const f32),
                UniformValue::Vector3(val) => gl::Uniform3fv(location, 1, &val as *const Vector3<f32> as *const f32)
            }
        }
    }
}
