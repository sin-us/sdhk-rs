extern crate rand;
extern crate cgmath;

use glfw::Key;

use self::cgmath::{Deg, Vector2, Vector3, InnerSpace, Zero};

use gfx::camera::Camera;
use gfx::render_target::RenderTarget;
use gfx::shader_program::ShaderProgram;
use gfx::shader_target::{ Axis, ShaderTarget };
use gfx::mesh::Mesh;

use sphere_grid::corner::Corner;
use sphere_grid::grid::Grid;
use sphere_grid::grid_mesh::GridMesh;
use sphere_grid::tile::PlanetTile;

use planet_gen::landscape::Landscape;

use vertex::Vertex;

vertex_struct!(
    PlanetVertex {
        pos: [Vector3<f32>, "aPos"],
        normal: [Vector3<f32>, "aNormal"],
        color: [Vector3<f32>, "aColor"],
        tex: [Vector2<f32>, "aTexCoord"],

        height: [f32, "aHeight"],
        brightness: [f32, "aBrightness"],
        temperature: [f32, "aTemperature"],
        humidity: [f32, "aHumidity"],
        clouds: [f32, "aClouds"],
    }
);

impl PlanetVertex {
    pub fn new(pos: Vector3<f32>, normal: Vector3<f32>) -> PlanetVertex {
        PlanetVertex {
            pos: pos,
            normal: normal,
            color: Vector3::zero(),
            tex: Vector2::zero(),

            brightness: 0.0,
            temperature: 0.0,
            humidity: 0.0,
            height: 0.0,
            clouds: 0.0,
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlanetOverlay {
    Basic,
    Heights,
    Brightness,
    Temperature,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlanetType {
    Empty
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlanetMaterial {
    Granite
}



pub struct Planet<'a> {
    grid: Grid,
    mesh: Mesh<PlanetVertex>,
    surface_target: ShaderTarget<'a, PlanetVertex>,
    atmosphere_target: ShaderTarget<'a, PlanetVertex>,

    sun_pos: Vector3<f32>,
    rotation: Deg<f32>,

    sea_level: f32,

    last_frame: f32,
    planet_type: PlanetType,
    overlay: PlanetOverlay,
}

#[allow(dead_code)]
impl<'a> Planet<'a> {
    /* Tileset and textures are not currently used. Maybe in future */
    const TILESET_WIDTH: f32 = 1000.0;
    const TILESET_HEIGHT: f32 = 100.0;

    const TILESET_TILES_PER_ROW: f32 = 10.0;
    const TILESET_TILES_PER_COLUMN: f32 = 1.0;

    const TILESET_TILE_WIDTH: f32 = Planet::TILESET_WIDTH / Planet::TILESET_TILES_PER_ROW;
    const TILESET_TILE_HEIGHT: f32 = Planet::TILESET_HEIGHT / Planet::TILESET_TILES_PER_COLUMN;

    const TILE_WIDTH_NORMALIZED: f32 = Planet::TILESET_TILE_WIDTH / Planet::TILESET_WIDTH;
    const TILE_HEIGHT_NORMALIZED: f32 = Planet::TILESET_TILE_HEIGHT / Planet::TILESET_HEIGHT;

    const RADIUS: f32 = 1.0;

    const TEX_COORDS_HEXAGON: [[f32;2];6] = [
            [0.25 * Planet::TILE_WIDTH_NORMALIZED, 1.0 * Planet::TILE_HEIGHT_NORMALIZED],
            [0.75 * Planet::TILE_WIDTH_NORMALIZED, 1.0 * Planet::TILE_HEIGHT_NORMALIZED],
            [1.0  * Planet::TILE_WIDTH_NORMALIZED, 0.5 * Planet::TILE_HEIGHT_NORMALIZED],
            [0.75 * Planet::TILE_WIDTH_NORMALIZED, 0.0 * Planet::TILE_HEIGHT_NORMALIZED],
            [0.25 * Planet::TILE_WIDTH_NORMALIZED, 0.0 * Planet::TILE_HEIGHT_NORMALIZED],
            [0.0  * Planet::TILE_WIDTH_NORMALIZED, 0.5 * Planet::TILE_HEIGHT_NORMALIZED],
        ];

    pub fn create(grid: Grid,
                  sun_pos: Vector3<f32>,
                  surface_shader: &'a ShaderProgram<'a, PlanetVertex>,
                  atmosphere_shader: &'a ShaderProgram<'a, PlanetVertex>) -> Self {

        let surface_mesh  = Self::create_mesh(&grid);

        Planet {
            grid: grid,
            mesh: surface_mesh,
            surface_target: ShaderTarget::create(surface_shader),
            atmosphere_target: ShaderTarget::create(atmosphere_shader),

            sun_pos: sun_pos,
            rotation: Deg(0.0),

            sea_level: 0.0,

            last_frame: 0.0,
            overlay: PlanetOverlay::Basic,
            planet_type: PlanetType::Empty,
        }
    }

    pub fn create_mesh(grid: &Grid) -> Mesh<PlanetVertex> {
        GridMesh::create(grid, Planet::RADIUS, |pos, normal, tile: &PlanetTile| -> PlanetVertex {
            let mut vertex = PlanetVertex::new(pos, normal);            
            Self::fill_vertex(&mut vertex, tile);
            vertex
        })
    }

    fn fill_vertex(vertex: &mut PlanetVertex, tile: &PlanetTile) {
        vertex.brightness = tile.brightness;
        vertex.temperature = tile.temperature as f32;
        vertex.height = tile.height as f32;
        vertex.clouds = if tile.has_clouds { 1.0 } else { 0.0 };
    }

    fn update_vertices(&mut self) {
        {
            let vertices = self.mesh.get_mut_vertices();

            for i in 0..self.grid.tiles.len() {
                let tile = &self.grid.tiles[i];

                for j in 0..tile.grid_tile.edge_count as usize {
                    let corner_id = Corner::get_id(tile.grid_tile.corners[j]);
                    Self::fill_vertex(&mut vertices[corner_id], tile);
                }
            }
        }

        self.mesh.refill_vertices();
    }
    

    pub fn set_sea_level(&mut self, sea_level: f32) {
        self.sea_level = sea_level;
        self.surface_target.set_uniform_f32("sea_level", sea_level);
    }

    pub fn set_light(&mut self, direction: Vector3<f32>, color: Vector3<f32>) {
        self.surface_target.set_uniform_vec3("light_direction", direction.normalize());
        self.surface_target.set_uniform_vec3("light_color", color);
    }

    pub fn set_overlay(&mut self, overlay: PlanetOverlay) {
        self.overlay = overlay;
        self.surface_target.set_uniform_i32("overlay", overlay as i32)
    }

    fn get_color(t: &PlanetTile) -> Vector3<f32> {
        let is_water: bool = t.height < 300.0;

        return if is_water { [0.0, 0.0, 1.0].into() } else { [0.0, 1.0, 0.0].into() }
    }
}

impl<'a> RenderTarget for Planet<'a> {
    fn update(&mut self, camera: &Camera, time: f32) {

        let degrees_per_tick = 1.0;
        let seconds_per_tick = (24.0 * 60.0 * 60.0) / (360.0 / degrees_per_tick);

        self.rotation += Deg(degrees_per_tick);

        self.surface_target.set_rotation(Axis::Y, Deg(degrees_per_tick));
        self.atmosphere_target.set_rotation(Axis::Y, Deg(degrees_per_tick));

        self.surface_target.update(camera);
        self.atmosphere_target.update(camera);

        Landscape::heat(&mut self.grid, self.surface_target.get_model_matrix(), self.sun_pos, seconds_per_tick as f64);

        //if time - self.last_frame > 0.5 {
            self.update_vertices();
            self.last_frame = time;
        //}
    }

    fn process_key_pressed(&mut self, key: Key) {
        match key {
            Key::Num1 => { self.set_overlay(PlanetOverlay::Basic); },
            Key::Num2 => { self.set_overlay(PlanetOverlay::Brightness); },
            Key::Num3 => { self.set_overlay(PlanetOverlay::Temperature); },
            _ => {},
        }
    }

    fn render(&self) {
        self.surface_target.render(&self.mesh);
        //self.atmosphere_target.render(&self.mesh);
    }

    fn compile(&mut self) {
        self.mesh.compile();
        self.surface_target.compile();
        self.atmosphere_target.compile();
    }

    fn set_pos(&mut self, pos: Vector3<f32>) {
        self.surface_target.set_pos(pos);
        self.atmosphere_target.set_pos(pos);
    }
}