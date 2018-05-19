use cgmath::{Vector3, InnerSpace};

use gfx::mesh::Mesh;
use gfx::vertex::Vertex;

use sphere_grid::corner::Corner;
use sphere_grid::tile::PlanetTile;
use sphere_grid::grid::Grid;

pub struct GridMesh;

impl GridMesh {
    pub fn create<V, VCreator>(grid: &Grid, radius: f32, vertex_creator: VCreator) -> Mesh<V>
                                                         where V: Vertex, VCreator: Fn(Vector3<f32>, Vector3<f32>, &PlanetTile) -> V {
        
        let mut vertices: Vec<V> = (0..grid.corners.len()).map(|_| V::default()).collect();
        let mut indices: Vec<u32> = Vec::with_capacity(grid.tiles.len() * 12);

        for i in 0..grid.tiles.len() {
            let t = &grid.tiles[i];

            let corner0_pos = Corner::get_pos(t.grid_tile.corners[0]) * radius;
            let corner1_pos = Corner::get_pos(t.grid_tile.corners[1]) * radius;
            let corner2_pos = Corner::get_pos(t.grid_tile.corners[2]) * radius;

            let corner0_id = Corner::get_id(t.grid_tile.corners[0]);
            let corner1_id = Corner::get_id(t.grid_tile.corners[1]);
            let corner2_id = Corner::get_id(t.grid_tile.corners[2]);

            let normal = (corner2_pos - corner1_pos).cross(corner0_pos - corner1_pos);
            let normal = normal.normalize();

            let mut vertex0 = vertex_creator(corner0_pos, normal, t);
            vertices[corner0_id] = vertex0;

            let mut vertex1 = vertex_creator(corner1_pos, normal, t);
            vertices[corner1_id] = vertex1;

            let mut vertex2 = vertex_creator(corner2_pos, normal, t);
            vertices[corner2_id] = vertex2;

            for j in 3..t.grid_tile.edge_count as usize {
                let corner_pos = Corner::get_pos(t.grid_tile.corners[j]) * radius;
                let corner_id = Corner::get_id(t.grid_tile.corners[j]);

                let vertex = vertex_creator(corner_pos, normal, t);
                vertices[corner_id] = vertex;
            }

            for j in 0..t.grid_tile.edge_count as usize - 2 {
                indices.push(Corner::get_id(t.grid_tile.corners[0]) as u32);
                indices.push(Corner::get_id(t.grid_tile.corners[j + 1]) as u32);
                indices.push(Corner::get_id(t.grid_tile.corners[j + 2]) as u32);
            }
        }

        Mesh::create(vertices, indices, Vec::new())
    }
}