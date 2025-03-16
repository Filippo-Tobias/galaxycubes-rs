use bevy::prelude::*;
use crate::drop_bar::DroppableType;

#[derive(Component)]
pub struct TowerPreview{
    pub droppable_type: DroppableType,
}

impl Plugin for TowerPreview {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query_preview: Query<&mut TowerPreview, With<Transform>>,
    
) {
    for mut preview in query_preview.iter_mut() {
        if preview.droppable_type == DroppableType::Tower {
            let texture_handle = asset_server.load("Player1.png");

            let shape_material = materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle),
                ..default()
            });
            let shape_handle = meshes.add(Cuboid::default());

            let tower_transform = Transform::from_xyz(
                -490.0, 
                -440.0, 
                1.0).with_scale(Vec3::new(2.2, 2.2, 1.0));

            commands.spawn((
                Mesh3d(shape_handle.clone()),
                MeshMaterial3d(shape_material.clone()),
                tower_transform
            ));
        }
    }
} 