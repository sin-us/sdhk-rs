extern crate gl;
extern crate cgmath;

use gfx::vertex::Vertex;
use std::ffi::CString;
use std::os::raw::c_void;
use std::mem::{size_of};
use cgmath::{ Vector2, Vector3 };

use texture::Texture;
use shader_program::ShaderProgram;


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

            self.vbo = vbo;
            self.vao = vao;
            self.ebo = ebo;

            
            gl::BindVertexArray(self.vao);
            self.bind_vbo();

            V::bind_attributes();

            gl::BindVertexArray(0);
        };
    }

    pub fn update_vertices(&mut self, vertices: Vec<V>) {
        self.vertices = vertices;
        self.bind_vbo();
    }

    pub fn render(&self, shader_program: &ShaderProgram<V>) {
        unsafe {
            gl::UseProgram(shader_program.id());
            for i in 0..self.textures.len() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, self.textures[i].id());

                let uniform_texture = gl::GetUniformLocation(shader_program.id(), CString::new(format!("texture{}", i)).unwrap().as_ptr() as *const i8);
                gl::Uniform1i(uniform_texture, i as i32);
            }
           
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            gl::BindVertexArray(0);
        }
    }

    fn bind_vbo(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * size_of::<V>()) as isize, &self.vertices[0] as *const V as *const c_void, gl::STATIC_DRAW);
        }
        // V::bind_attributes();
    }
}
