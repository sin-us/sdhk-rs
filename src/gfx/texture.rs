extern crate gl;
extern crate image;
extern crate cgmath;

use std::os::raw::c_void;
use image::GenericImage;
use std::path::Path;

pub struct Texture {
    id: u32
}

impl Texture {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn create(path: &str, has_alpha: bool) -> Texture {
        unsafe {
            let mut texture: u32 = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);

            let img = image::open(&Path::new(path))
                        .expect(&format!("Failed to load texture {}", path));
            let img = img.flipv();

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, img.width() as i32, img.height() as i32, 0, 
                            if has_alpha { gl::RGBA } else { gl::RGB }, 
                            gl::UNSIGNED_BYTE, 
                            img.raw_pixels().as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            Texture {
                id: texture
            }
        }
    }
}