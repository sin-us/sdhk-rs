extern crate rand;
extern crate cgmath;


use gfx::camera::Camera;
use gfx::render_target::RenderTarget;
use gfx::shader_program::ShaderProgram;
use gfx::render_target::RenderableMesh;
use gfx::mesh::Mesh;
use self::cgmath::Vector3;

use planet_gen::grid::Grid;
use planet_gen::corner::CornerPos;

use mesh::Vertex;


pub struct SunVertex {
    pos: Vector3<f32>,
    color: Vector3<f32>,
}

impl SunVertex {
    pub fn new(pos: Vector3<f32>, color: Vector3<f32>) -> SunVertex {
        SunVertex {
            pos: pos,
            color: color
        }
    }
}

impl Vertex for SunVertex {
    fn bind_attributes() {
        let mut offset = 0;
        SunVertex::bind_vec3_attribute(0, &mut offset); // pos
        SunVertex::bind_vec3_attribute(1, &mut offset); // color
        SunVertex::bind_vec2_attribute(2, &mut offset); // tex
    }
}


pub struct SunMesh<'a> {
    renderable_mesh: RenderableMesh<'a, SunVertex>,
}

impl<'a> SunMesh<'a> {
    const RADIUS: f32 = 1.0;

    pub fn create(grid: &Grid, shader_program: &'a ShaderProgram<'a>) -> SunMesh<'a> {
        let mut vertices: Vec<SunVertex> = Vec::new();

        let radius = SunMesh::RADIUS;

        let vertices = {
            for i in 0..grid.tiles.len() {
                let t = &grid.tiles[i];

                for j in 0..t.grid_tile.edge_count as usize - 2 {

                    let corner0: &CornerPos = &t.grid_tile.corners[0];
                    let corner1: &CornerPos = &t.grid_tile.corners[j + 1];
                    let corner2: &CornerPos = &t.grid_tile.corners[j + 2];

                    let a = Vector3::new(corner2.x() * radius, corner2.y() * radius, corner2.z() * radius);
                    let b = Vector3::new(corner1.x() * radius, corner1.y() * radius, corner1.z() * radius);
                    let c = Vector3::new(corner0.x() * radius, corner0.y() * radius, corner0.z() * radius);

                    let mut vertex = SunVertex::new(c, Vector3::new(1.0, 0.0, 0.0));
                    vertices.push(vertex);

                    let mut vertex = SunVertex::new(b, Vector3::new(1.0, 0.0, 0.0));
                    vertices.push(vertex);

                    let mut vertex = SunVertex::new(a, Vector3::new(1.0, 0.0, 0.0));
                    vertices.push(vertex);
                }
            }

            vertices        
        };

        let mesh = Mesh::create(vertices, Vec::new());

        SunMesh {
            renderable_mesh: RenderableMesh::create(mesh, shader_program)
        }
    }

    pub fn get_mesh(&self) -> &Mesh<SunVertex> {
        self.renderable_mesh.get_mesh()
    }
}

impl<'a> RenderTarget for SunMesh<'a> {
    fn update(&mut self, camera: &Camera, time: f32) {
        self.renderable_mesh.update(camera, time);
    }
    fn render(&self) {
        self.renderable_mesh.render();
    }

    fn compile(&mut self) {
        self.renderable_mesh.compile();
    }

    fn set_pos(&mut self, pos: Vector3<f32>) {
        self.renderable_mesh.set_pos(pos);
    }
}