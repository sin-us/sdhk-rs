extern crate cgmath;

use std::ptr;
use self::cgmath::Vector3;

use planet_gen::corner::Corner;
use planet_gen::edge::Edge;

pub struct PlanetTile {
    pub grid_tile: GridTile,
    pub gl_tile: GLTile,
    pub height: f64
}

pub struct GridTile {
    pub id: usize,
    pub edge_count: u8,
    pub pos: Vector3<f32>,
    pub tiles: [*const GridTile; 6],
    pub corners: [*const Corner; 6],
    pub edges: [*const Edge; 6]
}

pub struct GLTile {
    pub vertice_indices: [usize; 6]
}

impl PlanetTile {
    pub fn new(grid_tile: GridTile) -> PlanetTile {
        PlanetTile {
            grid_tile: grid_tile,
            gl_tile: GLTile { vertice_indices: [0; 6] },
            height: 0.0
        }
    }
}

#[allow(dead_code)]
impl GridTile {
    pub fn new(id: usize, edge_count: u8) -> GridTile {
        GridTile {
            id: id,
            edge_count: edge_count,
            pos: Vector3::new(0.0, 0.0, 0.0),
            tiles: [ptr::null(); 6],
            corners: [ptr::null(); 6],
            edges: [ptr::null(); 6]
        }
    }

    pub fn get_tile_pos(&self, n: *const GridTile) -> Option<usize> {
        for i in 0..self.edge_count as usize {
            if self.tiles[i] == n {
                return Some(i);
            }
        }
        return None;
    }

    pub fn get_corner_pos(&self, n: *const Corner) -> Option<usize> {
        for i in 0..self.edge_count as usize {
            if self.corners[i] == n {
                return Some(i);
            }
        }
        return None;
    }

    pub fn get_edge_pos(&self, n: *const Edge) -> Option<usize> {
        for i in 0..self.edge_count as usize {
            if self.edges[i] == n {
                return Some(i);
            }
        }
        return None;
    }

}