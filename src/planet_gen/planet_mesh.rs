extern crate rand;
extern crate cgmath;

use gfx::camera::Camera;
use gfx::render_target::RenderTarget;
use gfx::shader_program::ShaderProgram;
use gfx::shader_target::ShaderTarget;
use gfx::mesh::Mesh;
use planet_gen::tile::PlanetTile;
use self::cgmath::{Vector2, Vector3, InnerSpace, Zero};

use planet_gen::grid::Grid;
use planet_gen::corner::CornerPos;

use vertex::Vertex;

vertex_struct!(
    PlanetVertex {
        pos: [Vector3<f32>, "aPos"],
        normal: [Vector3<f32>, "aNormal"],
        color: [Vector3<f32>, "aColor"],
        tex: [Vector2<f32>, "aTexCoord"],
        height: [f32, "aHeight"],
        clouds: [f32, "aClouds"],
    }
);

impl PlanetVertex {
    pub fn new(pos: Vector3<f32>, normal: Vector3<f32>, color: Vector3<f32>) -> PlanetVertex {
        PlanetVertex {
            pos: pos,
            normal: normal,
            color: color,
            tex: Vector2::zero(),
            height: 0.0,
            clouds: 0.0,
        }
    }
}


pub struct PlanetMesh<'a> {
    grid: Grid,
    mesh: Mesh<PlanetVertex>,
    surface_target: ShaderTarget<'a, PlanetVertex>,
    atmosphere_target: ShaderTarget<'a, PlanetVertex>,
    sea_level: f32,
    last_frame: f32,
}

#[allow(dead_code)]
impl<'a> PlanetMesh<'a> {
    const TILESET_WIDTH: f32 = 1000.0;
    const TILESET_HEIGHT: f32 = 100.0;

    const TILESET_TILES_PER_ROW: f32 = 10.0;
    const TILESET_TILES_PER_COLUMN: f32 = 1.0;

    const TILESET_TILE_WIDTH: f32 = PlanetMesh::TILESET_WIDTH / PlanetMesh::TILESET_TILES_PER_ROW;
    const TILESET_TILE_HEIGHT: f32 = PlanetMesh::TILESET_HEIGHT / PlanetMesh::TILESET_TILES_PER_COLUMN;

    const TILE_WIDTH_NORMALIZED: f32 = PlanetMesh::TILESET_TILE_WIDTH / PlanetMesh::TILESET_WIDTH;
    const TILE_HEIGHT_NORMALIZED: f32 = PlanetMesh::TILESET_TILE_HEIGHT / PlanetMesh::TILESET_HEIGHT;

    const RADIUS: f32 = 1.0;

    const TEX_COORDS_HEXAGON: [[f32;2];6] = [
            [0.25 * PlanetMesh::TILE_WIDTH_NORMALIZED, 1.0 * PlanetMesh::TILE_HEIGHT_NORMALIZED],
            [0.75 * PlanetMesh::TILE_WIDTH_NORMALIZED, 1.0 * PlanetMesh::TILE_HEIGHT_NORMALIZED],
            [1.0  * PlanetMesh::TILE_WIDTH_NORMALIZED, 0.5 * PlanetMesh::TILE_HEIGHT_NORMALIZED],
            [0.75 * PlanetMesh::TILE_WIDTH_NORMALIZED, 0.0 * PlanetMesh::TILE_HEIGHT_NORMALIZED],
            [0.25 * PlanetMesh::TILE_WIDTH_NORMALIZED, 0.0 * PlanetMesh::TILE_HEIGHT_NORMALIZED],
            [0.0  * PlanetMesh::TILE_WIDTH_NORMALIZED, 0.5 * PlanetMesh::TILE_HEIGHT_NORMALIZED],
        ];

    pub fn recalc_vertices(&mut self) {
        println!("Surface Vertices recalculated");
        let vertices = Self::create_surface_vertices(&self.grid);
        self.mesh.update_vertices(vertices);
    }

    pub fn create(grid: Grid, surface_shader: &'a ShaderProgram<'a, PlanetVertex>, atmosphere_shader: &'a ShaderProgram<'a, PlanetVertex>) -> Self {
        let surface_vertices = Self::create_surface_vertices(&grid);
        let surface_mesh = Mesh::create(surface_vertices, Vec::new());

        PlanetMesh {
            grid: grid,
            mesh: surface_mesh,
            surface_target: ShaderTarget::create(surface_shader),
            atmosphere_target: ShaderTarget::create(atmosphere_shader),
            sea_level: 0.0,
            last_frame: 0.0,
        }
    }

    pub fn create_surface_vertices(grid: &Grid) -> Vec<PlanetVertex> {
        PlanetMesh::create_vertices(grid, PlanetMesh::RADIUS, |v,t| {
            v.height = t.height as f32;
            v.clouds = if t.has_clouds { 1.0 } else { 0.0 };
        })
    }

    pub fn create_vertices<VAction>(grid: &Grid, radius: f32, vertex_action: VAction) -> Vec<PlanetVertex>
                                                         where VAction: Fn(&mut PlanetVertex, &PlanetTile) {
        let mut vertices: Vec<PlanetVertex> = Vec::new();

        let vertices = {
            println!("tiles count: {}", grid.tiles.len());

            for i in 0..grid.tiles.len() {

                {
                    let t = &grid.tiles[i];

                    for j in 0..t.grid_tile.edge_count as usize - 2 {

                        let corner0: &CornerPos = &t.grid_tile.corners[0];
                        let corner1: &CornerPos = &t.grid_tile.corners[j + 1];
                        let corner2: &CornerPos = &t.grid_tile.corners[j + 2];

                        let a = Vector3::new(corner2.x() * radius, corner2.y() * radius, corner2.z() * radius);
                        let b = Vector3::new(corner1.x() * radius, corner1.y() * radius, corner1.z() * radius);
                        let c = Vector3::new(corner0.x() * radius, corner0.y() * radius, corner0.z() * radius);

                        // let tex_coord_a: [f32;2];
                        // let tex_coord_b: [f32;2];
                        // let tex_coord_c: [f32;2];

                        // if t.grid_tile.edge_count == 6 {
                        //     tex_coord_a = [PlanetMesh::TEX_COORDS_HEXAGON[0][0] + tile_x_offset, PlanetMesh::TEX_COORDS_HEXAGON[0][1]];
                        //     tex_coord_b = [PlanetMesh::TEX_COORDS_HEXAGON[j + 1][0] + tile_x_offset, PlanetMesh::TEX_COORDS_HEXAGON[j + 1][1]];
                        //     tex_coord_c = [PlanetMesh::TEX_COORDS_HEXAGON[j + 2][0] + tile_x_offset, PlanetMesh::TEX_COORDS_HEXAGON[j + 2][1]];
                        // }
                        // else
                        // {
                        //     tex_coord_a = [PlanetMesh::TEX_COORDS_HEXAGON[0][0] + tile_x_offset, PlanetMesh::TEX_COORDS_HEXAGON[0][1]];
                        //     tex_coord_b = [PlanetMesh::TEX_COORDS_HEXAGON[0][0] + tile_x_offset, PlanetMesh::TEX_COORDS_HEXAGON[0][1]];
                        //     tex_coord_c = [PlanetMesh::TEX_COORDS_HEXAGON[0][0] + tile_x_offset, PlanetMesh::TEX_COORDS_HEXAGON[0][1]];
                        // }

                        let normal = (a - b).cross(c - b);
                        let normal = normal.normalize();

                        let mut vertex = PlanetVertex::new(c, normal, PlanetMesh::get_color(t));
                        vertex_action(&mut vertex, t);
                        vertices.push(vertex);

                        let mut vertex = PlanetVertex::new(b, normal, PlanetMesh::get_color(t));
                        vertex_action(&mut vertex, t);
                        vertices.push(vertex);

                        let mut vertex = PlanetVertex::new(a, normal, PlanetMesh::get_color(t));
                        vertex_action(&mut vertex, t);
                        vertices.push(vertex);
                    }
                }
            }

            vertices        
        };

        vertices
    }

    pub fn create_vertices_for_tile(t: &PlanetTile) -> Vec<PlanetVertex> {
        let mut vertices: Vec<PlanetVertex> = Vec::new();

        let radius = 1.0001;

        for j in 0..t.grid_tile.edge_count as usize - 2 {

            let corner0: &CornerPos = &t.grid_tile.corners[0];
            let corner1: &CornerPos = &t.grid_tile.corners[j + 1];
            let corner2: &CornerPos = &t.grid_tile.corners[j + 2];

            let a = Vector3::new(corner2.x() * radius, corner2.y() * radius, corner2.z() * radius);
            let b = Vector3::new(corner1.x() * radius, corner1.y() * radius, corner1.z() * radius);
            let c = Vector3::new(corner0.x() * radius, corner0.y() * radius, corner0.z() * radius);

            let normal = (a - b).cross(c - b);

            vertices.push(PlanetVertex::new(c, normal, PlanetMesh::get_color(t)));
            vertices.push(PlanetVertex::new(b, normal, PlanetMesh::get_color(t)));
            vertices.push(PlanetVertex::new(a, normal, PlanetMesh::get_color(t)));
        }

        vertices
    }

    pub fn set_sea_level(&mut self, sea_level: f32) {
        self.sea_level = sea_level;
        self.surface_target.set_uniform_f32("sea_level", sea_level);
    }

    pub fn set_light(&mut self, direction: Vector3<f32>, color: Vector3<f32>) {
        self.surface_target.set_uniform_vec3("light_direction", direction.normalize());
        self.surface_target.set_uniform_vec3("light_color", color);
    }

    fn get_color(t: &PlanetTile) -> Vector3<f32> {
        let is_water: bool = t.height < 300.0;

        return if is_water { [0.0, 0.0, 1.0].into() } else { [0.0, 1.0, 0.0].into() }
    }
}

impl<'a> RenderTarget for PlanetMesh<'a> {
    fn update(&mut self, camera: &Camera, time: f32) {
        self.surface_target.update(camera);
        self.atmosphere_target.update(camera);

        let mut i = 0;

        for t in &mut self.grid.tiles {
            t.height = t.height + 0.1;
            t.has_clouds = t.height < 900.0;
        }

        if time - self.last_frame > 0.5 {
            self.recalc_vertices();
            self.last_frame = time;
        }
    }

    fn render(&self) {
        self.surface_target.render(&self.mesh);
        self.atmosphere_target.render(&self.mesh);
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