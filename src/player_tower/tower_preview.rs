use crate::{drop_bar::DroppableDropped, game_camera::{self, GameCamera}, level_loader::Map};
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

#[allow(clippy::too_many_arguments)]
fn move_preview(
    windows: Query<&Window>,
    mut query_tower_preview_entity: Query<Entity, With<TowerPreview>>, // Query for TowerPreview entity
    query_tower_preview: Query<&TowerPreview>,
    mut query_tower_preview_transform: Query<&mut Transform>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
    query_material: Query<&mut MeshMaterial3d<StandardMaterial>, With<TowerPreview>>,
    mut assets_standardmaterial: ResMut<Assets<StandardMaterial>>,
    map: Res<Map>, // Resource containing tower positions
) {
    let point: Vec3 = game_camera::cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query);
    for tower_preview_entity in query_tower_preview_entity.iter_mut() {
        let tower_preview: &TowerPreview = query_tower_preview.get(tower_preview_entity).unwrap();
        if tower_preview.droppable_type == DroppableType::Tower {
            query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.x = (point.x / 1.2).round() * 1.2;
            query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.z = (point.z / 1.2).round() * 1.2;
            if map.tower_positions.contains_key(&((point.x / 1.2).round() as i32, (point.z / 1.2).round() as i32)) {
            //Converting the mouse position to the tile position in multiples of 1.2.
                let material_handle = query_material.get(tower_preview_entity).unwrap();
                let material = assets_standardmaterial.get_mut(material_handle);
                match material {
                    Some(standard_material) => {
                        standard_material.base_color = Color::srgba(1.0, 0.3, 0.3, 0.4)
                    }
                    None => print!("MeshMaterial3d doesn't contain material (shouldn't happen)")
                }
                query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.y = 1.5;
                //Make the new tower appear on top
            } else {
                //Converting the mouse position to the tile position in multiples of 1.2.
                let material_handle = query_material.get(tower_preview_entity).unwrap();
                let material = assets_standardmaterial.get_mut(material_handle);
                match  material {
                    Some(standard_material) => {
                        standard_material.base_color = Color::srgba(0.5, 0.5, 0.5, 0.25)
                    }
                    None => print!("MeshMaterial3d doesn't contain material (shouldn't happen)")
                }
                query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.y = 0.5;
                //Reset tower position back to normal
            }

        }
    }
} 

fn check_for_drop( //Check for drop event then delete the tower preview entity.
    mut ev_dropped: EventReader<DroppableDropped>,
    mut commands: Commands,
    mut query_tower_preview: Query<(Entity, &TowerPreview)>, // Query for TowerPreview entities
    //mut res_locking_camera: ResMut<game_camera::LockingCamera>,
) {
    for event in ev_dropped.read() {
        if event.droppable_type == DroppableType::Tower {
            for (entity, _tower_preview) in query_tower_preview.iter_mut() {
                //res_locking_camera.list.retain(|x| x != &entity);
                commands.entity(entity).despawn(); // Despawn the TowerPreview entity
            }
        }
    }
}
