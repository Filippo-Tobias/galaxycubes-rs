use bevy::{math::DVec3, prelude::*};
use crate::floor_tile_mesh;
#[derive(Component)]
pub struct LevelLoader;

impl Plugin for LevelLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_map, make_map));
    }
}

fn load_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //let spawn_positions =
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        transform: Transform::from_xyz(
            0.0,
            -0.1,
            -14.0,
        ),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

fn make_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("raaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaan");
    let spawn_positions = (DVec3{x: -14.4,y: 0.0,z: -14.4}, DVec3{x: 14.4,y: 0.0,z: 14.4});
    let floor_tile_mesh = floor_tile_mesh::create_floor_tile_mesh(&mut meshes);

    //Calculate the amount of tiles
    let tiles_x = ((spawn_positions.0.x - spawn_positions.1.x).abs() / 1.2).round() as i32;
    let tiles_z = ((spawn_positions.0.z - spawn_positions.1.z).abs() / 1.2).round() as i32;
    let mut position: DVec3 = spawn_positions.0;
    for i in 0..tiles_x {
        for j in 0..tiles_z{
            commands.spawn(PbrBundle {
                mesh: floor_tile_mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.7, 0.6),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(position.x as f32, position.y as f32, position.z as f32),
                ..Default::default()
            });
            println!("x: {}, y: {}, z: {}, i: {}, j: {}, tiles_x: {}", position.x, position.y, position.z, i, j, tiles_x);
            position.z += 1.2;
        }
        position.z = spawn_positions.0.z;
        position.x += 1.2;
    }
}
