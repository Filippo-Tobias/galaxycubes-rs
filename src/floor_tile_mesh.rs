use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

#[derive(Component)]
pub struct FloorTileMesh;

impl Plugin for FloorTileMesh {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Define vertices for the custom mesh
    let vertices = vec![
        // Bottom
        [0.6, 0.0, 0.6,], // 0
        [-0.6, 0.0, 0.6], // 1
        [0.6, 0.0, -0.6], // 2
        [-0.6, 0.0, -0.6], // 3

        // Bottom Side Outer Flap (A)
        [0.6, 0.0, 0.6], // 4
        [-0.6, 0.0, 0.6], // 5
        [0.6, 0.2, 0.6], // 6
        [-0.6, 0.2, 0.6], // 7

        // Right Side Outer Flap (C)
        [0.6, 0.0, -0.6], // 8
        [0.6, 0.0, 0.6], // 9
        [0.6, 0.2, -0.6], // 10
        [0.6, 0.2, 0.6], // 11

        // Top Side Outer Flap (B)
        [0.6, 0.0, -0.6], // 12
        [-0.6, 0.0, -0.6], // 13
        [0.6, 0.2, -0.6], // 14
        [-0.6, 0.2, -0.6], // 15

        // Left Side Outer Flap (D)
        [-0.6, 0.0, 0.6], //16
        [-0.6, 0.0, -0.6], //17
        [-0.6, 0.2, 0.6], //18
        [-0.6, 0.2, -0.6], //19

        // Bottom Side Top Flap (A)
        [0.6, 0.2, 0.6], // 20
        [-0.6, 0.2, 0.6], // 21
        [0.6, 0.2, 0.5], // 22
        [-0.6, 0.2, 0.5], // 23

        // Right Side Top Flap (C)
        [0.6, 0.2, -0.6], // 24
        [0.6, 0.2, 0.6], // 25
        [0.5, 0.2, -0.6], // 26
        [0.5, 0.2, 0.6], // 27

        // Top Side Top Flap (B)
        [0.6, 0.2, -0.5], // 28
        [-0.6, 0.2, -0.5], // 29
        [0.6, 0.2, -0.6], // 30
        [-0.6, 0.2, -0.6], // 31

        // Left Side Top Flap (D)
        [-0.6, 0.2, 0.6], // 32
        [-0.6, 0.2, -0.6], // 33
        [-0.5, 0.2, 0.6], // 34
        [-0.5, 0.2, -0.6], // 35

        // Bottom Side Inner Flap (A)
        [0.6, 0.0, 0.5], // 36
        [-0.6, 0.0, 0.5], // 37
        [0.6, 0.2, 0.5], // 38
        [-0.6, 0.2, 0.5], // 39

        // Right Side Inner Flap (C)
        [0.5, 0.0, -0.6], // 40
        [0.5, 0.0, 0.6], // 41
        [0.5, 0.2, -0.6], // 42
        [0.5, 0.2, 0.6], // 43

        // Top Side Outer Flap (B)
        [0.6, 0.0, -0.5], // 44
        [-0.6, 0.0, -0.5], // 45
        [0.6, 0.2, -0.5], // 46
        [-0.6, 0.2, -0.5], // 47

        // Left Side Inner Flap (D)
        [-0.5, 0.0, 0.6], //48
        [-0.5, 0.0, -0.6], //49
        [-0.5, 0.2, 0.6], //50
        [-0.5, 0.2, -0.6], //51

    ];

    let normals = vec![
        // Normals for each vertex

        // Bottom
        [0.0, 1.0, 0.0], // 0
        [0.0, 1.0, 0.0], // 1
        [0.0, 1.0, 0.0], // 2
        [0.0, 1.0, 0.0], // 3

        // Bottom Side Outer Flap (A)
        [0.0, 0.0, -1.0], // 4
        [0.0, 0.0, -1.0], // 5
        [0.0, 0.0, -1.0], // 6
        [0.0, 0.0, -1.0], // 7

        // Right Side Outer Flap (C)
        [1.0, 0.0, 0.0], // 8
        [1.0, 0.0, 0.0], // 9
        [1.0, 0.0, 0.0], // 10
        [1.0, 0.0, 0.0], // 11

        // Top Side Outer Flap (B)
        [0.0, 0.0, 1.0], // 12
        [0.0, 0.0, 1.0], // 13
        [0.0, 0.0, 1.0], // 14
        [0.0, 0.0, 1.0], // 15

        // Left Side Outer Flap (D)
        [-1.0, 0.0, 0.0], //16
        [-1.0, 0.0, 0.0], //17
        [-1.0, 0.0, 0.0], //18
        [-1.0, 0.0, 0.0], //19

        // Bottom Side Top Flap (A)
        [0.0, 1.0, 0.0], // 20
        [0.0, 1.0, 0.0], // 21
        [0.0, 1.0, 0.0], // 22
        [0.0, 1.0, 0.0], // 23

        // Right Side Top Flap (A)
        [0.0, 1.0, 0.0], // 24
        [0.0, 1.0, 0.0], // 25
        [0.0, 1.0, 0.0], // 26
        [0.0, 1.0, 0.0], // 27

        // Top Side Top Flap (A)
        [0.0, 1.0, 0.0], // 28
        [0.0, 1.0, 0.0], // 29
        [0.0, 1.0, 0.0], // 30
        [0.0, 1.0, 0.0], // 31

        // Left Side Top Flap (A)
        [0.0, 1.0, 0.0], // 32
        [0.0, 1.0, 0.0], // 33
        [0.0, 1.0, 0.0], // 34
        [0.0, 1.0, 0.0], // 35

        // Bottom Side Inner Flap (A)
        [0.0, 0.0, 1.0], // 36
        [0.0, 0.0, -1.0], // 37
        [0.0, 0.0, -1.0], // 38
        [0.0, 0.0, -1.0], // 39

        // Right Side Inner Flap (C)
        [-1.0, 0.0, 0.0], // 40
        [-1.0, 0.0, 0.0], // 41
        [-1.0, 0.0, 0.0], // 42
        [-1.0, 0.0, 0.0], // 43

        // Top Side Inner Flap (B)
        [0.0, 0.0, 1.0], // 44
        [0.0, 0.0, 1.0], // 45
        [0.0, 0.0, 1.0], // 46
        [0.0, 0.0, 1.0], // 47

        // Left Side Inner Flap (D)
        [1.0, 0.0, 0.0], //48
        [1.0, 0.0, 0.0], //49
        [1.0, 0.0, 0.0], //50
        [1.0, 0.0, 0.0], //51
    ];

    let uvs = vec![
        // UV coordinates
        [1.0, 1.0], // 0
        [1.0, 0.0], // 1
        [0.0, 0.0], // 2
        [0.0, 1.0], // 3

        [1.0, 1.0], // 4
        [1.0, 0.0], // 5
        [0.0, 0.0], // 6
        [0.0, 1.0], // 7

        [1.0, 1.0], // 8
        [1.0, 0.0], // 9
        [0.0, 0.0], // 10
        [0.0, 1.0], // 11

        [1.0, 1.0], // 12
        [1.0, 0.0], // 13
        [0.0, 0.0], // 14
        [0.0, 1.0], // 15

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19

        [0.0, 1.0], // 16
        [0.0, 1.0], // 17
        [0.0, 1.0], // 18
        [0.0, 1.0], // 19
    ];

    let indices = Indices::U32(vec![
        0,3,1, 2,3,0,
        6,7,4, 4,7,5,
        8,11,9, 10,11,8,
        13,15,12, 12,15,14,
        16,19,17, 18,19,16,
        20,23,21, 22,23,20,
        24,27,25, 26,27,24,
        28,31,29, 30,31,28,
        32,35,33, 34,35,32,
        36,39,37, 38,39,36,
        41,43,40, 40,43,42,
        44,47,45, 46,47,44,
        49,51,48, 48,51,50,

    ]);

    // Create the mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    // Add mesh to the scene
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        ..Default::default()
    });

    // Add light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
