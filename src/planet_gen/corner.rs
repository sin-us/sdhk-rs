extern crate cgmath;

use self::cgmath::Vector3;

use std::ptr;
use planet_gen::tile::GridTile;
use planet_gen::edge::Edge;

pub trait CornerPos {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

impl CornerPos for *const Corner {
    fn x(&self) -> f32 {
        unsafe { (**self).pos.x }
    }

    fn y(&self) -> f32 {
        unsafe { (**self).pos.y }
    }

    fn z(&self) -> f32 {
        unsafe { (**self).pos.z }
    }
}

pub struct Corner {
    pub id: usize,
    pub pos: Vector3<f32>,
    pub tiles: [*const GridTile; 3],
    pub corners: [*const Corner; 3],
    pub edges: [*const Edge; 3]
}

impl Corner {
    pub fn new(id: usize) -> Corner {
        Corner {
            id: id,
            pos: Vector3::new(0.0, 0.0, 0.0),
            tiles: [ptr::null(); 3],
            corners: [ptr::null(); 3],
            edges: [ptr::null(); 3]
        }
    }

    pub fn get_corner_pos(&self, n: *const Corner) -> Option<usize> {
        for i in 0..self.corners.len() {
            if self.corners[i] == n {
                return Some(i);
            }
        }
        return None;
    }
}