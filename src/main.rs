extern crate glfw;
extern crate gl;
extern crate image;
extern crate cgmath;

use cgmath::{ Zero, Vector3 };

#[macro_use]
mod gfx;
use gfx::*;

mod planet_gen;
use planet_gen::*;

mod sun;
use sun::*;

mod game_state;
use game_state::GameState;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
    let shader_program;
    let mut planet_mesh;

    let grid_6 = Grid::create_size_n_grid(6);
    let mut grid = Grid::create_size_n_grid(7);

    Landscape::fill_heights(&mut grid);

    shader_program = ShaderProgram::create_with_geometry("assets/shaders/planet/basic.vert", "assets/shaders/planet/basic.frag", "assets/shaders/planet/basic.geom");
    

    let shader_sun = ShaderProgram::create_basic("assets/shaders/sun/sun.vert", "assets/shaders/sun/bright_sun.frag");
    let mut sun_mesh = SunMesh::create(&grid_6, &shader_sun);

    let mut game_window = GameWindow::<GameState>::create(SCR_WIDTH, SCR_HEIGHT, "Sdhk-rs", glfw::WindowMode::Windowed);
    let mut game: GameState = GameState::new();

    planet_mesh = PlanetMesh::create(&grid, &shader_program);

    let mut camera = Camera::create_default();
    camera.set_viewport(0, 0, 800, 600);
    camera.set_clipping(0.1, 1000.0);
    camera.set_position(Vector3::new(2.0, 0.0, 5.0));


    game.camera = camera;

    let sun_pos = Vector3::new(10.0, 0.0, -100.0);

    planet_mesh.compile();
    planet_mesh.set_pos(Vector3::zero());
    planet_mesh.set_light(Vector3::new(2.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

    println!("{:?}", Vector3::zero() - sun_pos);

    planet_mesh.set_sea_level(500.0);

    sun_mesh.compile();
    sun_mesh.set_pos(sun_pos);

    game.add_mesh(&mut planet_mesh);
    game.add_mesh(&mut sun_mesh);

    game_window.render(&mut game);
}