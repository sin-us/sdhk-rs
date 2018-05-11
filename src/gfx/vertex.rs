extern crate gl;
extern crate cgmath;

use std::os::raw::c_void;
use std::mem::{size_of};
use cgmath::{ Vector2, Vector3 };


pub trait VertexAttribute {
    fn bind(index: u32, offset: &mut usize, struct_size: i32);
}

impl VertexAttribute for f32 {
    fn bind(index: u32, offset: &mut usize, struct_size: i32) {
        unsafe {
            gl::VertexAttribPointer(index, 1, gl::FLOAT, gl::FALSE, struct_size, *offset as *const c_void);
            gl::EnableVertexAttribArray(index);
        }
        *offset += size_of::<f32>()
    }
}

impl VertexAttribute for Vector3<f32> {
    fn bind(index: u32, offset: &mut usize, struct_size: i32) {
        unsafe {
            gl::VertexAttribPointer(index, 3, gl::FLOAT, gl::FALSE, struct_size, *offset as *const c_void);
            gl::EnableVertexAttribArray(index);
        }
        *offset += size_of::<Vector3<f32>>()
    }
}

impl VertexAttribute for Vector2<f32> {
    fn bind(index: u32, offset: &mut usize, struct_size: i32) {
        unsafe {
            gl::VertexAttribPointer(index, 2, gl::FLOAT, gl::FALSE, struct_size, *offset as *const c_void);
            gl::EnableVertexAttribArray(index);
        }
        *offset += size_of::<Vector2<f32>>()
    }
}

pub trait Vertex: Sized {
    fn bind_attributes();
       
    fn bind_attribute<T: VertexAttribute>(index: u32, offset: &mut usize) {
        use std::mem::size_of;
        T::bind(index, offset, size_of::<Self>() as i32);
    }
}

macro_rules! vertex_struct {
    ( 
        $struct_name:ident {
            $($field_name:ident : [$field_type:ty, $field_shader_name:expr]),*,
        } 
    ) => {
        
        #[allow(dead_code)]
        pub struct $struct_name {
            $($field_name : $field_type),*,
        }

        impl Vertex for $struct_name {
            #[allow(dead_code)]
            fn bind_attributes() {
                let mut index = 0;
                let mut offset = 0;
                $(
                    $struct_name::bind_attribute::<$field_type>(index, &mut offset);
                    index = index + 1;
                )*
            }
        }
    };
}