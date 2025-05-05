use bevy::prelude::*;

use crate::game_systems::range_system::RangeArea;
use crate::level_loader::Map;
use crate::shooter_pillar::bullet_mesh;

pub struct ShooterPillarPlugin;
impl Plugin for ShooterPillarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, move_bullets);
    }
}

#[derive(Component)]
pub struct ShooterPillar;

fn setup(
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
    spawn_bullet(meshes, commands, materials);
}

fn spawn_bullet(
    mut res_meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let bullet_mesh = bullet_mesh::create_bullet_mesh(&mut res_meshes);
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

#[derive(Component)]
pub struct ShooterPillarBullet{
    velocity: Vec3
}

fn move_bullets (
    mut query_bullet: Query<(&mut Transform, &ShooterPillarBullet)>
) {
    for (mut bullet_transform, bullet) in query_bullet.iter_mut() {
      bullet_transform.translation += bullet.velocity
    };
}
