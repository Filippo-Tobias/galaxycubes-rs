use bevy::prelude::*;

use super::components::*;

use crate::attack_timer::components::AttackTimer;
use crate::range_system::components::RangeArea;
use crate::level_loader::Map;
use crate::shooter_pillar::bullet_mesh;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map: ResMut<Map>,
) {
    let texture_handle = asset_server.load("Enemies/Enemy2.png");
    let shape_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });
    let shape_handle = meshes.add(Cuboid::default());
    let new_pillar_transform= Transform::from_xyz(1.2, 0.5, 1.2);
    let second_pillar_transform= Transform::from_xyz(2.4, 0.5, 1.2);
    let new_pillar_entity = commands.spawn((
        RangeArea{range: (-4..=5,-4..=5), entities: vec![]},
        ShooterPillar,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        new_pillar_transform,
    ))
    .id();
    commands.entity(new_pillar_entity).insert(AttackTimer::new(Timer::from_seconds(10.0, TimerMode::Repeating)));
    map.tower_positions.insert(((new_pillar_transform.translation.x /1.2) as i32 , (new_pillar_transform.translation.z /1.2) as i32 ), new_pillar_entity);
    let second_pillar_entity = commands.spawn((
        RangeArea{range: (-4..=5,-4..=5), entities: vec![]},
        ShooterPillar,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        second_pillar_transform,
    ))
    .id();
    map.tower_positions.insert(((second_pillar_transform.translation.x /1.2) as i32 , (second_pillar_transform.translation.z /1.2) as i32 ), second_pillar_entity);
    spawn_bullet(&mut meshes, &mut commands, &mut materials);
}

pub fn check_timers(
    query_attack_timer: Query<&AttackTimer, With<ShooterPillar>>,
    mut res_meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    for attack_timer in query_attack_timer.iter() {
        if attack_timer.0.finished() {
            spawn_bullet(&mut res_meshes, &mut commands, &mut materials);
        }
    }
}

fn spawn_bullet(
    res_meshes: &mut ResMut<Assets<Mesh>>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let bullet_mesh = bullet_mesh::create_bullet_mesh(res_meshes);
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 1., 1.),
        ..Default::default()
    });
    commands.spawn((
        Mesh3d(bullet_mesh.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_translation(Vec3{x: 0., y: 6., z: 0.}),
        ShooterPillarBullet{velocity: Vec3 { x: 0.01, y: 0., z: 0. }}
    ));
}

pub fn move_bullets (
    mut query_bullet: Query<(&mut Transform, &ShooterPillarBullet)>
) {
    for (mut bullet_transform, bullet) in query_bullet.iter_mut() {
      bullet_transform.translation += bullet.velocity
    };
}
