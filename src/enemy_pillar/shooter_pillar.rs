use bevy::prelude::*;

use crate::level_loader::Map;

pub struct ShooterPillarPlugin;
impl Plugin for ShooterPillarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
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
    let new_pillar_entity = commands.spawn((
        ShooterPillar,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        new_pillar_transform,
    ))
    .id();
    map.tower_positions.insert(((new_pillar_transform.translation.x /1.2) as i32 , (new_pillar_transform.translation.z /1.2) as i32 ), new_pillar_entity);




}





