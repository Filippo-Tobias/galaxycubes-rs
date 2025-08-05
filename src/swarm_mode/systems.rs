use bevy::{math::DVec3, prelude::*};
use rand::Rng;
use crate::level_loader::{self, Map};
use crate::shooter_pillar::components::NewShooterPillar;
use super::components;
pub fn start(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut tower_ev_writer: EventWriter<NewShooterPillar>,
    map: ResMut<Map>
){
    let map_ranges = vec![(DVec3{x: -18.0,y: 0.0,z: -22.0}, DVec3{x: 18.0,y: 0.0,z: 7.0}), (DVec3{x: 0.0,y: 1.0,z: -22.0}, DVec3{x: 18.0,y: 1.0,z: 7.0})];
    level_loader::make_map(&mut commands, meshes, materials, map_ranges, map);
    tower_ev_writer.send(NewShooterPillar{
        location: Vec3 { x: 1.2, y: 0.5, z: 1.2 }
    });
    commands.spawn(components::EnemySpawnTimer(bevy::prelude::Timer::from_seconds(20.01, TimerMode::Repeating)));
}

pub fn spawn_random_enemies(
    mut tower_ev_writer: EventWriter<NewShooterPillar>,
    map: ResMut<Map>,
    mut query_enemy_spawn_timer: Query<&mut components::EnemySpawnTimer>,
    time: Res<Time>,
) {
    let mut enemy_spawn_timer = query_enemy_spawn_timer.single_mut();
    enemy_spawn_timer.0.tick(time.delta());
    let mut location: Vec3;
    let mut map_coord: (i32,i32,i32);
    if enemy_spawn_timer.0.finished() {
        loop {
            location = generate_random_position(Vec3{x: -18.0,y: 0.5,z: -22.0}, Vec3{x: 18.0,y: 0.5,z: 7.0});
            location.y /= 1.2;
            map_coord = (location.x as i32, location.y as i32, location.z as i32);
            if !map.tower_positions.contains_key(&map_coord) {
                location = location.map(|x| x*1.2);
                //println!("map_coord: {:?}, x: {}, y: {}, z: {},", map_coord, location.x, location.y, location.z);
                break;
            }
        }
        tower_ev_writer.send(NewShooterPillar{
            location 
        });
    }
}
fn generate_random_position(
    start: Vec3,
    end: Vec3
) -> Vec3 {
    let mut rng = rand::rng();
    let x: f32 = rng.random_range(start.x..=end.x).round();
    let z: f32 = rng.random_range(start.z..=end.z).round();
    let y = start.y;
    Vec3 { x, y, z }
}
