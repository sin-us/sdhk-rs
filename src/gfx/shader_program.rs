extern crate gl;

use gfx::shader_constructor::{ShaderType, ShaderSource};
use gfx::vertex::Vertex;
use gl::types::GLenum;
use gl::types::GLchar;
use std::ffi::{CString, CStr};
use std::ptr;

use gfx::shader_constructor::ShaderConstructor;

pub struct ShaderProgram<'a, V: Vertex> {
    id: u32,
    shaders: Vec<ShaderSource<'a, V>>,
}

impl<'a, V> ShaderProgram<'a, V> where V: Vertex {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn compile(&mut self) {
        unsafe {
            let shader_program = gl::CreateProgram();

            let mut compiled_shaders: Vec<u32> = Vec::with_capacity(self.shaders.len());

            for shader in self.shaders.iter() {
                let shader_source = ShaderConstructor::create_shader_from::<V>(shader);
                let compiled_shader = compile_shader(&shader_source, shader.shader_type);
                compiled_shaders.push(compiled_shader);
                gl::AttachShader(shader_program, compiled_shader);
            }
            
            gl::LinkProgram(shader_program);

            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as i32 {
                let mut error_string: [u8; 512] = [0; 512];
                let mut error_string_len = 0;
                gl::GetProgramInfoLog(shader_program, 512, &mut error_string_len, error_string.as_mut_ptr() as *mut GLchar);
                println!("Shader Program linking error: {}", CStr::from_bytes_with_nul(&error_string[0..error_string_len as usize + 1]).unwrap().to_string_lossy());
            }

            for compiled_shader in compiled_shaders.iter() {
                gl::DeleteShader(*compiled_shader);
            }

            self.id = shader_program;
        }
    }

    pub fn create_basic(vertex_shader_path: &'a str, fragment_shader_path: &'a str) -> Self {
        let vertex_shader = ShaderSource::new(vertex_shader_path, ShaderType::Vertex);
        let fragment_shader = ShaderSource::new(fragment_shader_path, ShaderType::Fragment);

        ShaderProgram {
            id: 0,
            shaders: vec!(vertex_shader, fragment_shader),
        }
    }

    pub fn create_with_geometry(vertex_shader_path: &'a str, fragment_shader_path: &'a str, geometry_shader_path: &'a str) -> Self {

        let vertex_shader = ShaderSource::new(vertex_shader_path, ShaderType::Vertex);
        let fragment_shader = ShaderSource::new(fragment_shader_path, ShaderType::Fragment);
        let geometry_shader = ShaderSource::new(geometry_shader_path, ShaderType::Geometry);

        ShaderProgram {
            id: 0,
            shaders: vec!(vertex_shader, fragment_shader, geometry_shader),
        }
    }
}


fn compile_shader(shader_source: &str, shader_type: ShaderType) -> u32 {
    unsafe {
        let shader_source = CString::new(shader_source).unwrap();
        let shader = gl::CreateShader(get_gl_type(&shader_type));
        gl::ShaderSource(shader, 1, &shader_source.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = 0;

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as i32 {
            let mut error_string: [u8; 512] = [0; 512];
            let mut error_string_len = 0;
            gl::GetShaderInfoLog(shader, 512, &mut error_string_len, error_string.as_mut_ptr() as *mut GLchar);
            println!("{:?} Shader compile error: {}", shader_type, CStr::from_bytes_with_nul(&error_string[0..error_string_len as usize + 1 ]).unwrap().to_string_lossy());
        }

        shader
    }
}

fn get_gl_type(shader_type: &ShaderType) -> GLenum {
    match shader_type {
        ShaderType::Vertex => gl::VERTEX_SHADER,
        ShaderType::Fragment => gl::FRAGMENT_SHADER,
        ShaderType::Geometry => gl::GEOMETRY_SHADER
    }
}