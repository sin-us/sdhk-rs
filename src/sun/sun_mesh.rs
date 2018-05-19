extern crate rand;
extern crate cgmath;

use glfw::Key;
use gfx::camera::Camera;
use gfx::render_target::RenderTarget;
use gfx::shader_program::ShaderProgram;
use gfx::shader_target::ShaderTarget;
use gfx::mesh::Mesh;
use gfx::vertex::Vertex;
use self::cgmath::Vector3;

use planet_gen::grid::Grid;
use planet_gen::corner::CornerPos;

vertex_struct!( SunVertex {
    pos: [Vector3<f32>, "aPos"],
} );

impl SunVertex {
    pub fn new(pos: Vector3<f32>) -> SunVertex {
        SunVertex {
            pos: pos,
        }
    }
}

pub struct SunMesh<'a> {
    mesh: Mesh<SunVertex>,
    target: ShaderTarget<'a, SunVertex>,
}

impl<'a> SunMesh<'a> {
    const RADIUS: f32 = 1.0;

    pub fn create(grid: &Grid, shader_program: &'a ShaderProgram<'a, SunVertex>) -> SunMesh<'a> {
        let mut vertices: Vec<SunVertex> = Vec::with_capacity(grid.tiles.len() * 6);
        let mut indices: Vec<u32> = Vec::with_capacity(grid.tiles.len() * 12);

        let radius = SunMesh::RADIUS;

        let mut last_vertex_i = 0;

        for i in 0..grid.tiles.len() {
            let t = &grid.tiles[i];

            let corner0 = (&t.grid_tile.corners[0] as &CornerPos).pos() * radius;
            let corner1 = (&t.grid_tile.corners[1] as &CornerPos).pos() * radius;
            let corner2 = (&t.grid_tile.corners[2] as &CornerPos).pos() * radius;

            let mut vertex0 = SunVertex::new(corner0);
            vertices.push(vertex0);

            let mut vertex1 = SunVertex::new(corner1);
            vertices.push(vertex1);

            let mut vertex2 = SunVertex::new(corner2);
            vertices.push(vertex2);

            for j in 3..t.grid_tile.edge_count as usize {
                let corner = (&t.grid_tile.corners[j] as &CornerPos).pos() * radius;

                let mut vertex = SunVertex::new(corner);
                vertices.push(vertex);
            }

            for j in 0..t.grid_tile.edge_count as u32 - 2 {
                indices.push(0 + last_vertex_i);
                indices.push(j + 1 + last_vertex_i);
                indices.push(j + 2 + last_vertex_i);
            }

            last_vertex_i += t.grid_tile.edge_count as u32;
        }

        let mesh = Mesh::create(vertices, indices, Vec::new());

        SunMesh {
            mesh: mesh,
            target: ShaderTarget::create(shader_program)
        }
    }

    pub fn get_mesh(&self) -> &Mesh<SunVertex> {
        &self.mesh
    }
}

impl<'a> RenderTarget for SunMesh<'a> {
    fn update(&mut self, camera: &Camera, time: f32) {
        self.target.update(camera);
    }

    fn process_key_pressed(&mut self, key: Key) {
        
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