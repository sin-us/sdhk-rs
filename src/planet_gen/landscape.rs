extern crate cgmath;
extern crate noise;

use std::f64;
use planet_gen::grid::Grid;
use self::noise::{Fbm, NoiseFn, Point3, Seedable, MultiFractal};

pub struct Landscape;

impl Landscape {
    pub fn fill_heights(grid: &mut Grid) {
        let perlin_surface = Fbm::default().set_octaves(7);

        let perlin_clouds = Fbm::default().set_seed(9043).set_octaves(4);

        let max_height = 1000.0;

        let mut min_noise: f64 = f64::MAX;
        let mut max_noise: f64 = f64::MIN;

        for i in 0..grid.tiles.len() {
            let tile = &mut grid.tiles[i];
            let pos = tile.grid_tile.pos;
            let point3: Point3<f64> = [pos.x as f64, pos.y as f64, pos.z as f64].into();
            
            tile.height = perlin_surface.get(point3);

            tile.has_clouds = perlin_clouds.get(point3) > 0.2;

            min_noise = f64::min(min_noise, tile.height);
            max_noise = f64::max(max_noise, tile.height);
        }

        for i in 0..grid.tiles.len() {
            let tile = &mut grid.tiles[i];

            tile.height = (tile.height - min_noise) / (max_noise - min_noise) * max_height;
        }
    }
}