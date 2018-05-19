use std::ptr;
use tile::GridTile;
use corner::Corner;

pub struct Edge {
    pub id: usize,
    pub tiles: [*const GridTile; 2],
    pub corners: [*const Corner; 2]
}

impl Edge {
    pub fn new(id: usize) -> Edge {
        Edge {
            id: id,
            tiles: [ptr::null(); 2],
            corners: [ptr::null(); 2],
        }
    }
}