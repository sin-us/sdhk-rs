extern crate cgmath;
extern crate noise;

use std::f64;
use cgmath::{ Vector3, Matrix4, InnerSpace, Transform };
use self::noise::{Fbm, NoiseFn, Point3, Seedable, MultiFractal};

use sphere_grid::grid::Grid;
use sphere_grid::tile::PlanetCoreMaterial;

pub struct Landscape;

impl Landscape {
    pub fn fill_heights(grid: &mut Grid, max_height: f64, sea_level: f64) {
        let perlin_surface = Fbm::default().set_octaves(7);

        let perlin_clouds = Fbm::default().set_seed(9043).set_octaves(4);

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
            tile.has_water = tile.height < sea_level;

            if tile.has_water {
                tile.core_material = &PlanetCoreMaterial::WATER;
            }
        }
    }

    pub fn heat(grid: &mut Grid, model_matrix: Matrix4<f32>, sun_pos: Vector3<f32>, delta_t: f64) {
        let mut max_temp: f64 = 0.0;
        let mut min_temp: f64 = 0.0;
        for t in &mut grid.tiles {
            t.brightness = model_matrix.transform_vector(t.grid_tile.pos).dot(sun_pos);

            let stephen_bolzman_const = 5.67036713 * 0.00000001;

            let q_sun = 1000.0; // W ~ J / s
            let q_absorbed: f64;

            if t.brightness <= 0.0 {
                q_absorbed = 0.0;
            } else {
                q_absorbed = q_sun * t.core_material.emissivity() * t.brightness as f64;
            }

            let q_emitted = (273.0 + t.temperature).powf(4.0) * t.core_material.emissivity() * stephen_bolzman_const;
            let delta_q = (q_absorbed - q_emitted) * delta_t; // J

            let delta_temperature = delta_q / (t.core_material.density() * t.core_material.specific_heat());

            t.temperature = t.temperature + delta_temperature;

            max_temp = f64::max(max_temp, t.temperature);
            min_temp = f64::min(min_temp, t.temperature);
        }

        println!("Max temp: {:?}    Min Temp: {:?}", max_temp, min_temp);
    }
}