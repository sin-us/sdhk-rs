use std::marker::PhantomData;
use gfx::vertex::Vertex;
use std::fs::File;
use std::io::Read;



#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry
}

pub struct ShaderSource<'a, V: Vertex> {
    pub file_path: &'a str,
    pub shader_type: ShaderType,
    phantom_vertex: PhantomData<V>
}

impl<'a, V> ShaderSource<'a, V> where V: Vertex {
    pub fn new(file_path: &'a str, shader_type: ShaderType) -> Self {
        ShaderSource {
            file_path: file_path,
            shader_type: shader_type,
            phantom_vertex: PhantomData
        }
    }
}

pub struct ShaderConstructor {

}

impl ShaderConstructor {
    pub fn create_shader_from<V: Vertex>(source: &ShaderSource<V>) -> String {
        let mut result: String = String::new();
        result.push_str("#version 330 core\n");

        let attributes = V::get_attributes();

        let file_contents = ShaderConstructor::read_file(source.file_path);

        match source.shader_type {
            ShaderType::Vertex => {
                for i in 0..attributes.len() {
                    result.push_str(&format!("layout (location = {}) in {} {};\n", i, attributes[i].attribute_type.get_shader_type(), attributes[i].attribute_name));
                }
            },
            ShaderType::Fragment => {},
            ShaderType::Geometry => {},
        }

        result.push_str(&file_contents);

        result
    }

    fn read_file(path: &str) -> String {
        let mut result = String::new();
        File::open(path)
                .expect(&format!("Failed to open {}", path))
            .read_to_string(&mut result)
                .expect(&format!("Failed to read shader: {}", path));

        result
    }
}