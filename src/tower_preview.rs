use crate::{drop_bar::DroppableDropped, game_camera::{self, GameCamera}};
use bevy::prelude::*;
use crate::drop_bar::DroppableType;

pub struct TowerPreviewPlugin;

impl Plugin for TowerPreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_preview, check_for_drop));
    }
}

#[derive(Component)]
pub struct TowerPreview{
    pub droppable_type: DroppableType,
}

fn move_preview(
    windows: Query<&Window>,
    mut query_tower_preview_entity: Query<Entity, With<TowerPreview>>, // Query for TowerPreview entity
    query_tower_preview: Query<&TowerPreview>,
    mut query_tower_preview_transform: Query<&mut Transform>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
) {
    let point: Vec3 = game_camera::cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query);
    for tower_preview_entity in query_tower_preview_entity.iter_mut() {
        println!("Entity found: {:?}", tower_preview_entity);
        let tower_preview: &TowerPreview = query_tower_preview.get(tower_preview_entity).unwrap();
        if tower_preview.droppable_type == DroppableType::Tower {
            query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.x = (point.x / 1.2).round() * 1.2;
            query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.z = (point.z / 1.2).round() * 1.2;}
    }
} 

fn check_for_drop( //Check for drop event then delete the tower preview entity.
    mut ev_dropped: EventReader<DroppableDropped>,
    mut commands: Commands,
    mut query_tower_preview: Query<(Entity, &TowerPreview)>, // Query for TowerPreview entities
) {
    for event in ev_dropped.read() {
        if event.droppable_type == DroppableType::Tower {
            for (entity, _tower_preview) in query_tower_preview.iter_mut() {
                commands.entity(entity).despawn(); // Despawn the TowerPreview entity
            }
        }
    }
}

fn check_if_position_valid(
    mut commands: Commands,
    mut query_tower_preview: Query<(Entity, &TowerPreview)>, // Query for TowerPreview entities
) {
    for (entity, tower_preview) in query_tower_preview.iter_mut() {
        if tower_preview.droppable_type == DroppableType::Tower {
            // Check if the position is valid (e.g., not occupied by another entity)
            // If not valid, despawn the TowerPreview entity
            commands.entity(entity).despawn(); // Despawn the TowerPreview entity
        }
    }
}