use std::{hash::Hash, ops::RangeInclusive};

use bevy::{math::DVec3, prelude::*, utils::{hashbrown::HashMap, HashSet}};
use crate::floor_tile_mesh;

#[derive(Resource)]
//Vec2 don't have a hash function, so we need to use integers instead.
//The interger positions are then multiplied by 1.2 to get the real position.
pub struct Map {
    pub tower_positions: HashMap<(i32, i32, i32), Entity>,
    pub available_positions: HashSet<(i32,i32,i32)>,
    pub map_ranges: Vec<(DVec3,DVec3)>,
}
impl Map {
    pub fn new() -> Self {
        Map{tower_positions: HashMap::new(), map_ranges: Vec::new() ,available_positions: HashSet::new()}
    }

    pub fn add_entity(
        &mut self,
        entity: Entity,
        position: (i32,i32,i32),
    ) {
        self.tower_positions.insert(position, entity);
        self.available_positions.remove(&position);
        //Insert takes v as V (v: V) meaning the entity passed will be copied since the entity trait implements the copy trait.
    }

    pub fn remove_entity(
        &mut self,
        position: (i32,i32,i32)
    ) {
        self.tower_positions.remove(&position);
    }
}

#[derive(Component)]
pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Map::new())
        .add_event::<MapDragged>();
    }
}

fn make_default_map (
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    map: ResMut<Map>,
) {
    let map_ranges = vec![(DVec3{x: -18.0,y: 0.0,z: -22.0}, DVec3{x: 18.0,y: 0.0,z: 7.0})];
    make_map(&mut commands, meshes, materials, map_ranges, map);
}

pub fn make_map(
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    map_ranges: Vec<(DVec3,DVec3)>,
    mut map: ResMut<Map>,
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
    map.map_ranges = map_ranges.clone();
    println!("{:?}", map.map_ranges);
    // for range in &map_ranges {
    //     for x in range.0.x as i32..range.1.x as i32{
    //         for z in range.0.z as i32..range.1.z as i32{
    //             map.available_positions.insert((x,range.0.y as i32,z));
    //             println!("{}", range.0.y);
    //         }
    //     }
    // }
    build_map(commands, meshes, materials, map_ranges);
}

fn build_map(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map_ranges: Vec<(DVec3,DVec3)>
) {
    let floor_tile_mesh = floor_tile_mesh::create_floor_tile_mesh(&mut meshes);
    for range in map_ranges {
        let spawn_positions = (DVec3{x: range.0.x * 1.2, y: range.0.y * 1.2, z: range.0.z * 1.2}, DVec3{x: range.1.x * 1.2,y: range.1.y * 1.2, z: range.1.z * 1.2});
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
                ))
                .observe(on_map_drag);
                //println!("x: {}, y: {}, z: {}, i: {}, j: {}, tiles_x: {}", position.x, position.y, position.z, i, j, tiles_x);
                position.z += 1.2;
            }
            position.z = spawn_positions.0.z;
            position.x += 1.2;
        }
    }
}

#[derive(Event)]
pub struct MapDragged{}

fn on_map_drag(_event: Trigger<Pointer<Drag>>, mut ev_hovered: EventWriter<MapDragged>) {
    ev_hovered.send(MapDragged{});
}
