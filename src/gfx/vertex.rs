extern crate gl;
extern crate cgmath;

use std::os::raw::c_void;
use std::mem::{size_of};
use cgmath::{ Vector2, Vector3 };

#[derive(Debug)]
pub enum VertexAttribute {
    Float,
    Vector2,
    Vector3,
}

impl VertexAttribute {
    pub fn bind(&self, index: u32, offset: &mut usize, struct_size: i32) {
        match self {
            VertexAttribute::Float => VertexAttribute::bind_f32_attribute(index, offset, struct_size),
            VertexAttribute::Vector2 => VertexAttribute::bind_vec2_attribute(index, offset, struct_size),
            VertexAttribute::Vector3 => VertexAttribute::bind_vec3_attribute(index, offset, struct_size),
        }
    }

    pub fn get_shader_type(&self) -> &str {
        match self {
            VertexAttribute::Float => "float",
            VertexAttribute::Vector2 => "vec2",
            VertexAttribute::Vector3 => "vec3",
        }
    }

    fn bind_vec3_attribute(index: u32, offset: &mut usize, struct_size: i32) {
        Self::bind_vec_f32_attribute(index, 3, *offset, struct_size);
        *offset += size_of::<Vector3<f32>>()
    }

    fn bind_vec2_attribute(index: u32, offset: &mut usize, struct_size: i32) {
        Self::bind_vec_f32_attribute(index, 2, *offset, struct_size);
        *offset += size_of::<Vector2<f32>>()
    }

    fn bind_f32_attribute(index: u32, offset: &mut usize, struct_size: i32) {
        unsafe {
            gl::VertexAttribPointer(index, 1, gl::FLOAT, gl::FALSE, struct_size, *offset as *const c_void);
            gl::EnableVertexAttribArray(index);
        }
        *offset += size_of::<f32>()
    }

    fn bind_vec_f32_attribute(index: u32, size: i32, offset: usize, struct_size: i32) {
        unsafe {
            gl::VertexAttribPointer(index, size, gl::FLOAT, gl::FALSE, struct_size, offset as *const c_void);
            gl::EnableVertexAttribArray(index);
        }
    }
}

#[derive(Debug)]
pub struct VertexAttributeDescription {
    pub attribute_type: VertexAttribute,
    pub attribute_name: String
}

pub trait IntoVertexAttribute {
    fn into() -> VertexAttribute;
}

impl IntoVertexAttribute for f32 {
    fn into() -> VertexAttribute {
        VertexAttribute::Float
    }
}

impl IntoVertexAttribute for Vector2<f32> {
    fn into() -> VertexAttribute {
        VertexAttribute::Vector2
    }
}

impl IntoVertexAttribute for Vector3<f32> {
    fn into() -> VertexAttribute {
        VertexAttribute::Vector3
    }
}


pub trait Vertex: Sized + Copy + Clone {
    fn bind_attributes();

    fn get_attributes() -> Vec<VertexAttributeDescription>;
       
    fn bind_attribute(attribute_type: VertexAttribute, index: u32, offset: &mut usize) {
        use std::mem::size_of;
        attribute_type.bind(index, offset, size_of::<Self>() as i32);
    }
}

macro_rules! vertex_struct {
    ( 
        $struct_name:ident {
            $($field_name:ident : [$field_type:ty, $field_shader_name:expr]),*,
        } 
    ) => {
        
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        pub struct $struct_name {
            $($field_name : $field_type),*,
        }

        impl Vertex for $struct_name {
            #[allow(dead_code)]
            fn bind_attributes() {
                let mut index = 0;
                let mut offset = 0;
                $(
                    $struct_name::bind_attribute(<$field_type as ::vertex::IntoVertexAttribute>::into(), index, &mut offset);
                    index = index + 1;
                )*
            }

            fn get_attributes() -> Vec<::vertex::VertexAttributeDescription> {
                vec!($(
                        ::vertex::VertexAttributeDescription {
                            attribute_type: <$field_type as ::vertex::IntoVertexAttribute>::into(),
                            attribute_name: String::from($field_shader_name)
                        },
                    )*)
            }
        }
    };
}