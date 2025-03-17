use bevy::{math::DVec3, prelude::*, utils::{hashbrown::HashMap, HashSet}};
use crate::floor_tile_mesh;
#[derive(Component)]
pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.
        add_systems(Startup, (load_map, make_map))
        .insert_resource(Map {
            tower_positions: HashMap::new(),
        })
        ;
    }
}

fn load_map(
    mut commands: Commands,
) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 1000.0,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));
}

fn make_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_positions = (DVec3{x: -21.6,y: 0.0,z: -26.4}, DVec3{x: 21.6,y: 0.0,z: 8.4});
    //let spawn_positions = (DVec3{x: -3.6,y: 0.0,z: -3.6}, DVec3{x: 3.6,y: 0.0,z: 3.6});
    //let spawn_positions = (DVec3{x: 0.0,y: 0.0,z: 0.0}, DVec3{x: 1.2,y: 0.0,z: 1.2});
    let floor_tile_mesh = floor_tile_mesh::create_floor_tile_mesh(&mut meshes);

    //Calculate the amount of tiles
    let tiles_x = ((spawn_positions.0.x - spawn_positions.1.x).abs() / 1.2).round() as i32;
    let tiles_z = ((spawn_positions.0.z - spawn_positions.1.z).abs() / 1.2).round() as i32;
    let mut position: DVec3 = spawn_positions.0;
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2),
        ..Default::default()
    });
    for _i in 0..tiles_x {
        for _j in 0..tiles_z{
            commands.spawn((
                Mesh3d(floor_tile_mesh.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(position.x as f32, position.y as f32, position.z as f32),
            )); //Find way to declare bundle once and clone it inside the loop.
            //println!("x: {}, y: {}, z: {}, i: {}, j: {}, tiles_x: {}", position.x, position.y, position.z, i, j, tiles_x);
            position.z += 1.2;
        }
        position.z = spawn_positions.0.z;
        position.x += 1.2;
    }
}

#[derive(Resource)]
pub struct Map {
    tower_positions: HashMap<Vec2, Entity>,
}