extern crate rand;
extern crate cgmath;


use gfx::camera::Camera;
use gfx::render_target::RenderTarget;
use gfx::shader_program::ShaderProgram;
use gfx::render_target::RenderableMesh;
use gfx::mesh::Mesh;
use planet_gen::tile::PlanetTile;
use self::cgmath::{Vector2, Vector3, InnerSpace};

use planet_gen::grid::Grid;
use planet_gen::corner::CornerPos;

use mesh::Vertex;


pub struct PlanetVertex {
    pos: Vector3<f32>,
    normal: Vector3<f32>,
    color: Vector3<f32>,
    tex: Vector2<f32>,
    pub height: f32
}

impl PlanetVertex {
    pub fn new(pos: Vector3<f32>, normal: Vector3<f32>, color: Vector3<f32>, tex: Vector2<f32>) -> PlanetVertex {
        PlanetVertex {
            pos: pos,
            normal: normal,
            color: color,
            tex: tex,
            height: 0.0
        }
    }
}

impl Vertex for PlanetVertex {
    fn bind_attributes() {
        let mut offset = 0;
        PlanetVertex::bind_vec3_attribute(0, &mut offset); // pos
        PlanetVertex::bind_vec3_attribute(1, &mut offset); // normal
        PlanetVertex::bind_vec3_attribute(2, &mut offset); // color
        PlanetVertex::bind_vec2_attribute(3, &mut offset); // tex
        PlanetVertex::bind_f32_attribute(4, &mut offset); // height
    }
}


pub struct PlanetMesh<'a> {
    renderable_mesh: RenderableMesh<'a, PlanetVertex>,
    sea_level: f32,
}

impl<'a> PlanetMesh<'a> {
    const tileset_width: f32 = 1000.0;
    const tileset_height: f32 = 100.0;

    const tileset_tiles_per_row: f32 = 10.0;
    const tileset_tiles_per_column: f32 = 1.0;

    const tileset_tile_width: f32 = PlanetMesh::tileset_width / PlanetMesh::tileset_tiles_per_row;
    const tileset_tile_height: f32 = PlanetMesh::tileset_height / PlanetMesh::tileset_tiles_per_column;

    const tile_width_normalized: f32 = 1.0; // tileset_tile_width / tileset_width;
    const tile_height_normalized: f32 = 1.0; //tileset_tile_height / tileset_height;

    const radius: f32 = 1.0;

    const tex_coords_hexagon: [[f32;2];6] = [
            [0.25 * PlanetMesh::tile_width_normalized, 1.0 * PlanetMesh::tile_height_normalized],
            [0.75 * PlanetMesh::tile_width_normalized, 1.0 * PlanetMesh::tile_height_normalized],
            [1.0  * PlanetMesh::tile_width_normalized, 0.5 * PlanetMesh::tile_height_normalized],
            [0.75 * PlanetMesh::tile_width_normalized, 0.0 * PlanetMesh::tile_height_normalized],
            [0.25 * PlanetMesh::tile_width_normalized, 0.0 * PlanetMesh::tile_height_normalized],
            [0.0  * PlanetMesh::tile_width_normalized, 0.5 * PlanetMesh::tile_height_normalized],
        ];

    pub fn create(grid: &Grid, shader_program: &'a ShaderProgram<'a>) -> PlanetMesh<'a> {
        let mut vertices: Vec<PlanetVertex> = Vec::new();

        let mut rng = rand::thread_rng();

        let radius = PlanetMesh::radius;

        let vertices = {
            println!("tiles count: {}", grid.tiles.len());

            for i in 0..grid.tiles.len() {

                let starting_tile_vertice = vertices.len();

                {
                    let t = &grid.tiles[i];

                    let is_water: bool = t.height < 400.0;
                    let tile_x_offset = if is_water { PlanetMesh::tile_width_normalized } else {0.0};

                    // "0-level" tiles (bottom)
                    for j in 0..t.grid_tile.edge_count as usize - 2 {

                        let corner0: &CornerPos = &t.grid_tile.corners[0];
                        let corner1: &CornerPos = &t.grid_tile.corners[j + 1];
                        let corner2: &CornerPos = &t.grid_tile.corners[j + 2];

                        let a = Vector3::new(corner2.x() * radius, corner2.y() * radius, corner2.z() * radius);
                        let b = Vector3::new(corner1.x() * radius, corner1.y() * radius, corner1.z() * radius);
                        let c = Vector3::new(corner0.x() * radius, corner0.y() * radius, corner0.z() * radius);

                        let tex_coord_a: [f32;2];
                        let tex_coord_b: [f32;2];
                        let tex_coord_c: [f32;2];

                        if t.grid_tile.edge_count == 6 {
                            tex_coord_a = [PlanetMesh::tex_coords_hexagon[0][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[0][1]];
                            tex_coord_b = [PlanetMesh::tex_coords_hexagon[j + 1][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[j + 1][1]];
                            tex_coord_c = [PlanetMesh::tex_coords_hexagon[j + 2][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[j + 2][1]];
                        }
                        else
                        {
                            tex_coord_a = [PlanetMesh::tex_coords_hexagon[0][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[0][1]];
                            tex_coord_b = [PlanetMesh::tex_coords_hexagon[0][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[0][1]];
                            tex_coord_c = [PlanetMesh::tex_coords_hexagon[0][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[0][1]];
                        }

                        let normal = (a - b).cross(c - b);
                        let normal = normal.normalize();

                        let mut vertex = PlanetVertex::new(c, normal, PlanetMesh::get_color(t), /* normal: c.normalize().into(),*/ tex_coord_c.into());
                        vertex.height = t.height as f32;
                        vertices.push(vertex);

                        let mut vertex = PlanetVertex::new(b, normal, PlanetMesh::get_color(t), /* normal: b.normalize().into(),*/ tex_coord_b.into());
                        vertex.height = t.height as f32;
                        vertices.push(vertex);

                        let mut vertex = PlanetVertex::new(a, normal, PlanetMesh::get_color(t), /* normal: a.normalize().into(),*/ tex_coord_a.into());
                        vertex.height = t.height as f32;
                        vertices.push(vertex);

                        //vertices.push(a); //.Add(CreateVertex(a, normA, t.Height));
                        //vertices.push(b); //Add(CreateVertex(b, normB, t.Height));
                        //vertices.push(c); //Add(CreateVertex(c, normC, t.Height));
                    }
                }
            }

            vertices        
        };

        let mesh = Mesh::create(vertices, Vec::new());

        PlanetMesh {
            sea_level: 0.0,
            renderable_mesh: RenderableMesh::create(mesh, shader_program)
        }
    }

    pub fn create_vertices_for_tile(t: &PlanetTile) -> Vec<PlanetVertex> {
        let mut vertices: Vec<PlanetVertex> = Vec::new();

        let tile_x_offset = 0.0;

        let radius = 1.0001;

        // "0-level" tiles (bottom)
        for j in 0..t.grid_tile.edge_count as usize - 2 {

            let corner0: &CornerPos = &t.grid_tile.corners[0];
            let corner1: &CornerPos = &t.grid_tile.corners[j + 1];
            let corner2: &CornerPos = &t.grid_tile.corners[j + 2];

            let a = Vector3::new(corner2.x() * radius, corner2.y() * radius, corner2.z() * radius);
            let b = Vector3::new(corner1.x() * radius, corner1.y() * radius, corner1.z() * radius);
            let c = Vector3::new(corner0.x() * radius, corner0.y() * radius, corner0.z() * radius);

            let tex_coord_a: [f32;2];
            let tex_coord_b: [f32;2];
            let tex_coord_c: [f32;2];

            if t.grid_tile.edge_count == 6 {
                tex_coord_a = [PlanetMesh::tex_coords_hexagon[0][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[0][1]];
                tex_coord_b = [PlanetMesh::tex_coords_hexagon[j + 1][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[j + 1][1]];
                tex_coord_c = [PlanetMesh::tex_coords_hexagon[j + 2][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[j + 2][1]];
            }
            else
            {
                tex_coord_a = [PlanetMesh::tex_coords_hexagon[0][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[0][1]];
                tex_coord_b = [PlanetMesh::tex_coords_hexagon[0][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[0][1]];
                tex_coord_c = [PlanetMesh::tex_coords_hexagon[0][0] + tile_x_offset, PlanetMesh::tex_coords_hexagon[0][1]];
            }

            let normal = (a - b).cross(c - b);

            vertices.push(PlanetVertex::new(c, normal, PlanetMesh::get_color(t), tex_coord_c.into()));
            vertices.push(PlanetVertex::new(b, normal, PlanetMesh::get_color(t), tex_coord_b.into()));
            vertices.push(PlanetVertex::new(a, normal, PlanetMesh::get_color(t), tex_coord_a.into()));

            //vertices.push(a); //.Add(CreateVertex(a, normA, t.Height));
            //vertices.push(b); //Add(CreateVertex(b, normB, t.Height));
            //vertices.push(c); //Add(CreateVertex(c, normC, t.Height));
        }

        vertices
    }

    pub fn set_sea_level(&mut self, sea_level: f32) {
        self.sea_level = sea_level;
        self.renderable_mesh.set_uniform_f32("sea_level", sea_level);
    }

    pub fn set_light(&mut self, direction: Vector3<f32>, color: Vector3<f32>) {
        self.renderable_mesh.set_uniform_vec3("light_direction", direction.normalize());
        self.renderable_mesh.set_uniform_vec3("light_color", color);
    }

    pub fn get_mesh(&self) -> &Mesh<PlanetVertex> {
        self.renderable_mesh.get_mesh()
    }

    fn get_color(t: &PlanetTile) -> Vector3<f32> {
        let is_water: bool = t.height < 300.0;

        return if is_water { [0.0, 0.0, 1.0].into() } else { [0.0, 1.0, 0.0].into() }
    }
}

impl<'a> RenderTarget for PlanetMesh<'a> {
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