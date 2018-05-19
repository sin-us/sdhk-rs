extern crate cgmath;

use std::ptr;
use self::cgmath::Vector3;

use corner::Corner;
use edge::Edge;

pub struct PlanetCoreMaterial {
    specific_heat: f64, // J / kg
    density: f64, // kg / m^3
    emissivity: f64,
}

impl PlanetCoreMaterial {
    pub const GRANITE: PlanetCoreMaterial = PlanetCoreMaterial {
        specific_heat: 280.0,
        density: 2750.0,
        emissivity: 0.9,
    };

    pub const WATER: PlanetCoreMaterial = PlanetCoreMaterial {
        specific_heat: 4180.0,
        density: 1000.0,
        emissivity: 0.92,
    };

    pub fn specific_heat(&self) -> f64 {
        self.specific_heat
    }

    pub fn density(&self) -> f64 {
        self.density
    }

    pub fn emissivity(&self) -> f64 {
        self.emissivity
    }
}

pub struct PlanetTile {
    pub grid_tile: GridTile,

    pub core_material: &'static PlanetCoreMaterial,
    pub height: f64,
    pub brightness: f32,
    pub temperature: f64,
    pub humidity: f64,
    pub has_water: bool,
    pub has_clouds: bool,
}

pub struct GridTile {
    pub id: usize,
    pub edge_count: u8,
    pub pos: Vector3<f32>,
    pub tiles: [*const GridTile; 6],
    pub corners: [*const Corner; 6],
    pub edges: [*const Edge; 6]
}


impl PlanetTile {
    pub fn new(grid_tile: GridTile) -> PlanetTile {
        PlanetTile {
            grid_tile: grid_tile,
            core_material: &PlanetCoreMaterial::GRANITE,
            height: 0.0,
            brightness: 0.0,
            temperature: 0.0,
            humidity: 0.0,
            has_water: false,
            has_clouds: false,
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