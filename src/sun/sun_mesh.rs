extern crate rand;
extern crate cgmath;

use cgmath::Vector3;
use glfw::Key;

use gfx::camera::Camera;
use gfx::render_target::RenderTarget;
use gfx::shader_program::ShaderProgram;
use gfx::shader_target::ShaderTarget;
use gfx::mesh::Mesh;
use gfx::vertex::Vertex;

use sphere_grid::grid::Grid;
use sphere_grid::grid_mesh::GridMesh;

vertex_struct!( SunVertex {
    pos: [Vector3<f32>, "aPos"],
} );


pub struct SunMesh<'a> {
    mesh: Mesh<SunVertex>,
    target: ShaderTarget<'a, SunVertex>,
}

impl<'a> SunMesh<'a> {
    const RADIUS: f32 = 1.0;

    pub fn create(grid: &Grid, shader_program: &'a ShaderProgram<'a, SunVertex>) -> SunMesh<'a> {
        let mesh = GridMesh::create(grid, SunMesh::RADIUS, |pos, _, _| SunVertex { pos: pos });

        SunMesh {
            mesh: mesh,
            target: ShaderTarget::create(shader_program)
        }
    }
}

impl<'a> RenderTarget for SunMesh<'a> {
    fn update(&mut self, camera: &Camera, _time: f32) {
        self.target.update(camera);
    }

    fn process_key_pressed(&mut self, _key: Key) {
        
    }
    
    fn render(&self) {
        self.target.render(&self.mesh);
    }

    fn compile(&mut self) {
        self.mesh.compile();
        self.target.compile();
    }

    fn set_pos(&mut self, pos: Vector3<f32>) {
        self.target.set_pos(pos);
    }
}