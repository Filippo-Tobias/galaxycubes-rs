use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
pub fn create_bullet_mesh(meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
    let indices = Indices::U32(vec![
        // Indices for each face of the cube
        
        // // Bottom face (0, 1, 2, 3)
        // 0, 3, 1, 2, 3, 0, 

        // Top face (4, 5, 6, 7)
        4, 7, 5, 6, 7, 4, 

        // Front face (8, 9, 10, 11)
        8, 11, 9, 10, 11, 8,

        // // Back face (12, 13, 14, 15)
        // 12, 13, 14, 15, 14, 13,

        // Left face (16, 17, 18, 19)
        16, 18, 17, 17, 18, 19,

        // Right face (20, 21, 22, 23)
        20, 21, 22, 23, 22, 21
    ]);


    let vertices = vec![
        // Bottom face
        [0.1, 0.0, 0.1], // 0
        [-0.1, 0.0, 0.1], // 1
        [0.1, 0.0, -0.1], // 2
        [-0.1, 0.0, -0.1], // 3

        // Top face
        [0.1, 0.1, 0.1], // 4
        [-0.1, 0.1, 0.1], // 5
        [0.1, 0.1, -0.1], // 6
        [-0.1, 0.1, -0.1], // 7

        // Front face
        [0.1, 0.0, 0.1], // 8
        [-0.1, 0.0, 0.1], // 9
        [0.1, 0.1, 0.1], // 10
        [-0.1, 0.1, 0.1], // 11

        // Back face
        [0.1, 0.0, -0.1], // 12
        [-0.1, 0.0, -0.1], // 13
        [0.1, 0.1, -0.1], // 14
        [-0.1, 0.1, -0.1], // 15

        // Left face
        [-0.1, 0.0, 0.1], // 16
        [-0.1, 0.0, -0.1], // 17
        [-0.1, 0.1, 0.1], // 18
        [-0.1, 0.1, -0.1], // 19

        // Right face
        [0.1, 0.0, 0.1], // 20
        [0.1, 0.0, -0.1], // 21
        [0.1, 0.1, 0.1], // 22
        [0.1, 0.1, -0.1], // 23
    ];

    let normals = vec![
        // Normals for each vertex

        // Bottom face
        [0.0, -1.0, 0.0], // 0
        [0.0, -1.0, 0.0], // 1
        [0.0, -1.0, 0.0], // 2
        [0.0, -1.0, 0.0], // 3

        // Top face
        [0.0, 1.0, 0.0], // 4
        [0.0, 1.0, 0.0], // 5
        [0.0, 1.0, 0.0], // 6
        [0.0, 1.0, 0.0], // 7

        // Front face
        [0.0, 0.0, 1.0], // 8
        [0.0, 0.0, 1.0], // 9
        [0.0, 0.0, 1.0], // 10
        [0.0, 0.0, 1.0], // 11

        // Back face
        [0.0, 0.0, -1.0], // 12
        [0.0, 0.0, -1.0], // 13
        [0.0, 0.0, -1.0], // 14
        [0.0, 0.0, -1.0], // 15

        // Left face
        [-1.0, 0.0, 0.0], // 16
        [-1.0, 0.0, 0.0], // 17
        [-1.0, 0.0, 0.0], // 18
        [-1.0, 0.0, 0.0], // 19

        // Right face
        [1.0, 0.0, 0.0], // 20
        [1.0, 0.0, 0.0], // 21
        [1.0, 0.0, 0.0], // 22
        [1.0, 0.0, 0.0], // 23

    ];

    let uvs = vec![
        // UV coordinates
        // Bottom face
        [1.0, 1.0], // 0
        [1.0, 0.0], // 1
        [0.0, 0.0], // 2
        [0.0, 1.0], // 3

        // Top face
        [1.0, 1.0], // 4
        [1.0, 0.0], // 5
        [0.0, 0.0], // 6
        [0.0, 1.0], // 7

        // Front face
        [0.0, 1.0], // 8
        [1.0, 1.0], // 9
        [0.0, 0.0], // 10
        [1.0, 0.0], // 11

        // Back face
        [0.0, 1.0], // 12
        [1.0, 1.0], // 13
        [0.0, 0.0], // 14
        [1.0, 0.0], // 15

        // Left face
        [0.0, 1.0], // 16
        [1.0, 1.0], // 17
        [0.0, 0.0], // 18
        [1.0, 0.0], // 19

        // Right face
        [0.0, 1.0], // 20
        [1.0, 1.0], // 21
        [0.0, 0.0], // 22
        [1.0, 0.0], // 23
    ];


    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    meshes.add(mesh)
}