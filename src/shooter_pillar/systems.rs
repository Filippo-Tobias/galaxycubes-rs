use bevy::prelude::*;

use super::components::*;

use crate::attack::components::{Attack, AttackTimer, AttackType};
use crate::bullet::components::BulletType;
use crate::range_system::components::RangeArea;
use crate::level_loader::Map;

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
        Attack{attack_type: AttackType::Bullet { bullet_type: BulletType::ShooterPillar}},
        new_pillar_transform,
    ))
    .id();
    commands.entity(new_pillar_entity).insert(AttackTimer::new(Timer::from_seconds(2.0, TimerMode::Repeating)));
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
}
