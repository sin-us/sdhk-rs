extern crate gl;
extern crate cgmath;

use std::ffi::CString;
use std::os::raw::c_void;
use std::mem::{size_of};
use cgmath::{ Vector2, Vector3 };

use texture::Texture;
use shader_program::ShaderProgram;

pub trait Vertex: Sized {
    fn bind_attributes();

    fn bind_vec3_attribute(index: u32, offset: &mut usize) {
        Self::bind_vec_f32_attribute(index, 3, *offset);
        *offset += size_of::<Vector3<f32>>()
    }

    fn bind_vec2_attribute(index: u32, offset: &mut usize) {
        Self::bind_vec_f32_attribute(index, 2, *offset);
        *offset += size_of::<Vector2<f32>>()
    }

    fn bind_f32_attribute(index: u32, offset: &mut usize) {
        unsafe {
            gl::VertexAttribPointer(index, 1, gl::FLOAT, gl::FALSE, size_of::<Self>() as i32, *offset as *const c_void);
            gl::EnableVertexAttribArray(index);
        }
        *offset += size_of::<f32>()
    }

    fn bind_vec_f32_attribute(index: u32, size: i32, offset: usize) {
        unsafe {
            gl::VertexAttribPointer(index, size, gl::FLOAT, gl::FALSE, size_of::<Self>() as i32, offset as *const c_void);
            gl::EnableVertexAttribArray(index);
        }
    }
}

pub struct Mesh<V: Vertex> {
    vertices: Vec<V>,
    textures: Vec<Texture>,
    vbo: u32,
    vao: u32,
    ebo: u32
}

impl<V> Mesh<V>
            where V: Vertex {
    pub fn create(vertices: Vec<V>, textures: Vec<Texture>) -> Mesh<V> {
        Mesh {
            vertices: vertices,
            textures: textures,
            vbo: 0,
            vao: 0,
            ebo: 0
        }
    }

    pub fn compile(&mut self) {
        unsafe {
            let mut vbo: u32 = 0;// Vertex Buffer Objects
            let mut vao: u32 = 0;
            let mut ebo: u32 = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);        
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * size_of::<V>()) as isize, &self.vertices[0] as *const V as *const c_void, gl::STATIC_DRAW);

            V::bind_attributes();

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            self.vbo = vbo;
            self.vao = vao;
            self.ebo = ebo;
        };
    }

    pub fn render(&self, shader_program: &ShaderProgram) {
        unsafe {
            gl::UseProgram(shader_program.id());
            for i in 0..self.textures.len() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, self.textures[i].id());

                let uniform_texture = gl::GetUniformLocation(shader_program.id(), CString::new(format!("texture{}", i)).unwrap().as_ptr() as *const i8);
                gl::Uniform1i(uniform_texture, i as i32);
            }
           
            gl::UseProgram(shader_program.id());
            gl::BindVertexArray(self.vao);

            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
        }
    }
}
