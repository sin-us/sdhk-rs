extern crate cgmath;

use std::ptr;

use self::cgmath::{Vector3, InnerSpace};

use planet_gen::tile::{GridTile, PlanetTile};
use planet_gen::corner::Corner;
use planet_gen::edge::Edge;

pub struct Grid {
    size: u8,
    pub tiles: Vec<PlanetTile>,
    pub corners: Vec<Corner>,
    pub edges: Vec<Edge>
}

impl Grid {
    fn new(size: u8) -> Grid {

        let tile_count = Grid::get_tile_count(size);
        let corner_count = Grid::get_corner_count(size);
        let edge_count = Grid::get_edge_count(size);


        Grid {
            size: size,
            tiles: (0..tile_count).map(|i| PlanetTile::new( GridTile::new(i, if i < 12 { 5 } else { 6 })) ).collect(),
            corners: (0..corner_count).map(|i| Corner::new(i)).collect(),
            edges: (0..edge_count).map(|i| Edge::new(i)).collect(),
        }
    }


    pub fn create_size_n_grid(size: u8) -> Grid {
        if size == 0 {
            return unsafe { Grid::create_size_0_grid() };
        }
        else {
            return unsafe { Grid::create_size_n_grid(size - 1).create_subdivided_grid() }
        }
    }

    unsafe fn create_size_0_grid() -> Grid {
        let mut grid = Grid::new(0);
        let x = -0.525731112119133606;
        let z = -0.850650808352039932;

        let icos_tiles = [
            Vector3::new(-x, 0.0, z), Vector3::new( x, 0.0, z ), Vector3::new( -x, 0.0, -z ), Vector3::new( x, 0.0, -z ),
            Vector3::new( 0.0, z, x ), Vector3::new( 0.0, z, -x ), Vector3::new( 0.0, -z, x ), Vector3::new( 0.0, -z, -x ),
            Vector3::new( z, x, 0.0 ), Vector3::new( -z, x, 0.0 ), Vector3::new( z, -x, 0.0 ), Vector3::new( -z, -x, 0.0 )
        ];

        let icos_tiles_n = [
            [9, 4, 1, 6, 11], [4, 8, 10, 6, 0], [11, 7, 3, 5, 9],  [2, 7, 10, 8, 5],
            [9, 5, 8, 1, 0],  [2, 3, 8, 4, 9],  [0, 1, 10, 7, 11], [11, 6, 10, 3, 2],
            [5, 3, 10, 1, 4], [2, 5, 4, 0, 11], [3, 7, 6, 1, 8],   [7, 2, 9, 0, 6]
        ];

        for i in 0..grid.tiles.len() {
            let t = &mut grid.tiles[i].grid_tile as *mut GridTile;

            (*t).pos = icos_tiles[(*t).id];

            for k in 0..5 {
                (*t).tiles[k] = &grid.tiles[icos_tiles_n[(*t).id][k]].grid_tile as *const GridTile;
            }
        }

        for i in 0..5 {
            grid.add_corner(i, 0, icos_tiles_n[0][(i + 4) % 5], icos_tiles_n[0][i]);
        }
        
        for i in 0..5 {
            grid.add_corner(i + 5, 3, icos_tiles_n[3][(i + 4) % 5], icos_tiles_n[3][i]);
        }

        grid.add_corner(10, 10, 1, 8);
        grid.add_corner(11, 1, 10, 6);
        grid.add_corner(12, 6, 10, 7);
        grid.add_corner(13, 6, 7, 11);
        grid.add_corner(14, 11, 7, 2);
        grid.add_corner(15, 11, 2, 9);
        grid.add_corner(16, 9, 2, 5);
        grid.add_corner(17, 9, 5, 4);
        grid.add_corner(18, 4, 5, 8);
        grid.add_corner(19, 4, 8, 1);

        //_add corners to corners
        for i in 0..grid.corners.len() {

            let c = &mut *(&mut grid.corners[i] as *mut Corner);

            for k in 0..3 {
                c.corners[k] = (*c.tiles[k]).corners[ ( (*c.tiles[k]).get_corner_pos(c).unwrap() + 1) % 5 ];
            }
        }
        

        //new edges
        let mut next_edge_id = 0;
        for i in 0..grid.tiles.len() {

            let t = &grid.tiles[i].grid_tile as *const GridTile;

            for k in 0..5 {
                if (*t).edges[k] == ptr::null() {
                    grid.add_edge(next_edge_id, (*t).id, icos_tiles_n[(*t).id][k]);
                    next_edge_id += 1;
                }
            }
        }

        grid
    }


    unsafe fn create_subdivided_grid(&self) -> Grid {
        let mut grid = Grid::new(self.size + 1);

        let prev_tile_count = self.tiles.len();
        let prev_corner_count = self.corners.len();

        //old tiles
        for i in 0..prev_tile_count {
            grid.tiles[i].grid_tile.pos = self.tiles[i].grid_tile.pos;

            for k in 0..grid.tiles[i].grid_tile.edge_count as usize {
                grid.tiles[i].grid_tile.tiles[k] = &grid.tiles[(*self.tiles[i].grid_tile.corners[k]).id + prev_tile_count].grid_tile;
            }
        }

        //old corners become tiles
        for i in 0..prev_corner_count {
            grid.tiles[i + prev_tile_count].grid_tile.pos = self.corners[i].pos;
            for k in 0..3 {
                grid.tiles[i + prev_tile_count].grid_tile.tiles[2 * k] = &grid.tiles[(*self.corners[i].corners[k]).id + prev_tile_count].grid_tile;
                grid.tiles[i + prev_tile_count].grid_tile.tiles[2 * k + 1] = &grid.tiles[(*self.corners[i].tiles[k]).id].grid_tile;
            }
        }

        //new corners
        let mut next_corner_id = 0;
        for i in 0..self.tiles.len() {
            let n = &self.tiles[i].grid_tile as *const GridTile;

            let t = &grid.tiles[(*n).id].grid_tile as *const GridTile;
            for k in 0..(*t).edge_count as usize {
                grid.add_corner(next_corner_id, (*t).id, (*(*t).tiles[(k + (*t).edge_count as usize - 1) % (*t).edge_count as usize]).id, (*(*t).tiles[k]).id);
                next_corner_id += 1;
            }
        }

        //connect corners
        for i in 0..grid.corners.len() {
            let c = &grid.corners[i] as *const Corner as *mut Corner;
            for k in 0..3 {
                (*c).corners[k] = (*(*c).tiles[k]).corners[ ((*(*c).tiles[k]).get_corner_pos(c).unwrap() + 1) % (*(*c).tiles[k]).edge_count as usize ];
            }
        }

        // //new edges
        let mut next_edge_id = 0;
        for i in 0..grid.tiles.len() {
            let t = &grid.tiles[i].grid_tile as *const GridTile as *mut GridTile;

            for k in 0..(*t).edge_count as usize {
                if (*t).edges[k].is_null() {
                    grid.add_edge(next_edge_id, (*t).id, (*(*t).tiles[k]).id);
                    next_edge_id += 1;
                }
            }
        }

        grid
    }


    unsafe fn add_corner(&mut self, id: usize, t1: usize, t2: usize, t3: usize) {
        let c = &mut self.corners[id];
        let t = [ 
            &mut self.tiles[t1].grid_tile as *mut GridTile, 
            &mut self.tiles[t2].grid_tile as *mut GridTile,
            &mut self.tiles[t3].grid_tile as *mut GridTile 
        ];

        let v = (*t[0]).pos + (*t[1]).pos + (*t[2]).pos;
        c.pos = v.normalize();

        for i in 0..3 {
            (*t[i]).corners[ (*t[i]).get_tile_pos(t[(i + 2) % 3]).unwrap()] = c;
            c.tiles[i] = t[i];
        }
    }

    unsafe fn add_edge(&mut self, id: usize, t1: usize, t2: usize) {
        let e = &mut self.edges[id];
        let t = [
            &mut self.tiles[t1].grid_tile as *mut GridTile,
            &mut self.tiles[t2].grid_tile as *mut GridTile 
        ];

        let c = [
            &mut self.corners[ (*(*t[0]).corners[ (*t[0]).get_tile_pos(t[1]).unwrap() ]).id] as *mut Corner,
            &mut self.corners[ (*(*t[0]).corners[ ( (*t[0]).get_tile_pos(t[1]).unwrap() + 1) % (*t[0]).edge_count as usize ]).id] as *mut Corner
        ];

        for i in 0..2 {
            (*t[i]).edges[ (*t[i]).get_tile_pos( t[(i + 1) % 2] ).unwrap()] = e as *const Edge;
            e.tiles[i] = t[i];
            (*c[i]).edges[ (*c[i]).get_corner_pos( c[(i + 1) % 2]).unwrap()] = e as *const Edge;
            e.corners[i] = c[i];
        }
    }


    fn get_tile_count(size: u8) -> usize {
        10 * 3usize.pow(size as u32) + 2
    }

    fn get_corner_count(size: u8) -> usize {
        20 * 3usize.pow(size as u32)
    }

    fn get_edge_count(size: u8) -> usize {
        30 * 3usize.pow(size as u32)
    }
}