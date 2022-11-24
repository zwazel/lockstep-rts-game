use bevy::prelude::*;
use bevy::render::mesh::{self, Indices, PrimitiveTopology};
use ds_heightmap::Runner;

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

pub fn setup_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let width = 10;
    let height = 10;
    let depth = 10.0;
    let rough = 1.0;
    let mut runner = Runner::new();
    runner.set_width(width);
    runner.set_height(height);
    runner.set_depth(depth); // q: what is this? a: depth of the terrain, how high the terrain is, how far the terrain goes up
    runner.set_rough(rough); // q: what is this? a: the roughness of the terrain, 0.0 is flat, 1.0 is very rough. Default: 1.0.

    let output = runner.ds();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for x in 0..width {
        for y in 0..height {
            let position = Vec3::new(x as f32, y as f32, output.data[x][y]);
            let normal = Vec3::new(0.0, 0.0, 1.0);
            let uv = Vec2::new(x as f32, y as f32);
            vertices.push(Vertex { position, normal, uv });
        }
    }

    for x in 0..width - 1 {
        for y in 0..height - 1 {
            let a = x + y * width;
            let b = (x + 1) + y * width;
            let c = x + (y + 1) * width;
            let d = (x + 1) + (y + 1) * width;
            indices.push(a as u32);
            indices.push(b as u32);
            indices.push(c as u32);
            indices.push(b as u32);
            indices.push(d as u32);
            indices.push(c as u32);
        }
    }

    let positions = vertices.iter().map(|v| v.position).collect::<Vec<_>>();
    let normals = vertices.iter().map(|v| v.normal).collect::<Vec<_>>();
    let uvs = vertices.iter().map(|v| v.uv).collect::<Vec<_>>();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.8, 0.8),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: material_handle,
            ..Default::default()
        });
}
