extern crate glfw;
extern crate gl;
extern crate image;
extern crate cgmath;

use planet_gen::planet_mesh::PlanetVertex;
use cgmath::{ Zero, Vector3 };

#[macro_use]
mod gfx;
use gfx::*;

mod planet_gen;
use planet_gen::*;
use planet_gen::planet_mesh::PlanetOverlay;

mod sun;
use sun::*;

mod game_state;
use game_state::GameState;

const SCR_WIDTH: u32 = 1024;
const SCR_HEIGHT: u32 = 768;

pub fn main() {
    let surface_shader;
    let mut grid;
    let atmosphere_shader;
    let mut planet_mesh;

    let sun_shader;
    let mut sun_mesh;

    let mut game_window = GameWindow::<GameState>::create(SCR_WIDTH, SCR_HEIGHT, "Sdhk-rs", glfw::WindowMode::Windowed);
    let mut game: GameState = GameState::new();

    let mut camera = Camera::create_default();
    camera.set_viewport(0, 0, 1024, 768);
    camera.set_clipping(0.1, 1000.0);
    camera.set_position(Vector3::new(2.0, 0.0, 5.0));
    game.camera = camera;

    //planet_shader = ShaderProgram::create_with_geometry("assets/shaders/planet/basic.vert", "assets/shaders/planet/basic.frag", "assets/shaders/planet/basic.geom");
    surface_shader = ShaderProgram::create_basic("assets/shaders/planet/surface.vert", "assets/shaders/planet/surface.frag");
    atmosphere_shader = ShaderProgram::create_basic("assets/shaders/planet/atmosphere.vert", "assets/shaders/planet/atmosphere.frag");
    sun_shader = ShaderProgram::create_basic("assets/shaders/sun/sun.vert", "assets/shaders/sun/bright_sun.frag");

    
    grid = Grid::create_size_n_grid(7);
    planet_mesh = create_planet(grid, &surface_shader, &atmosphere_shader);
    sun_mesh = create_sun(&sun_shader);
   
    game.add_mesh(Box::new(planet_mesh));
    game.add_mesh(Box::new(sun_mesh));

    game_window.render(&mut game);
}

fn create_planet<'a>(grid: Grid, surface_shader: &'a ShaderProgram<PlanetVertex>, atmosphere_shader: &'a ShaderProgram<PlanetVertex>) -> PlanetMesh<'a> {
    let mut grid = grid;

    let light_direction = Vector3::new(2.0, 0.0, 0.0);
    Landscape::fill_heights(&mut grid, 1000.0, 500.0);

    let mut planet_mesh = PlanetMesh::create(grid, light_direction, surface_shader, atmosphere_shader);
    planet_mesh.compile();
    planet_mesh.set_pos(Vector3::zero());
    planet_mesh.set_light(light_direction, Vector3::new(1.0, 1.0, 1.0));
    planet_mesh.set_sea_level(500.0);

    planet_mesh
}

fn create_sun<'a>(sun_shader: &'a ShaderProgram<SunVertex>) -> SunMesh<'a> {
    let sun_pos = Vector3::new(10.0, 0.0, -100.0);
    let grid_6 = Grid::create_size_n_grid(6);

    //println!("{:?}", Vector3::zero() - sun_pos);
    let mut sun_mesh = SunMesh::create(&grid_6, &sun_shader);
    sun_mesh.compile();
    sun_mesh.set_pos(sun_pos);

    sun_mesh
}