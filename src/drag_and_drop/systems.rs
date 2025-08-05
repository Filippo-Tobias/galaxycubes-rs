use bevy::prelude::*;
use crate::damage::components::Health;
use crate::level_loader::Map;
use crate::drag_and_drop::components::{DroppableDropped, DroppableType};
use crate::game_camera;
use crate::game_camera::components::GameCamera;
use crate::game_camera::systems::cursor_ray_to_plane;
use crate::drag_and_drop::components::TowerPreview;
use super::components::TowerDroppable;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut transform = Transform::from_xyz(-490.0, -440.0, -1.0);
    //let mut transform = Transform::from_xyz(-100.0, -250.0, -1.0);
    transform.scale=Vec3{x: 0.6, y: 0.6, z: 1.0};
    commands.spawn((
        Sprite::from_image(asset_server.load("drop_bar.png")),
        transform,
        println!("dropped bar")
    ));
    let tower_droppable_id = commands.spawn((
        Sprite::from_image(asset_server.load("Player1.png")),
        Transform::from_xyz(-490.0, -440.0, 1.0).with_scale(Vec3::new(2.2, 2.2, 1.0)),
        //Transform::from_xyz(-450.0, -250.0, 1.0).with_scale(Vec3::new(2.2, 2.2, 1.0)),
    )).id();
    commands.entity(tower_droppable_id).observe(on_dragged);
    commands.entity(tower_droppable_id).insert(TowerDroppable {
        dragging: false,
    });
    println!("spawned droppable");
}

#[allow(clippy::too_many_arguments)]
fn on_dragged(
    _dragged_events: Trigger<Pointer<Drag>>,
    mut query: Query<&mut TowerDroppable>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
    map: ResMut<Map>
) {
    //Spawn a cube as a preview of the tower if the drag just started.
    if !query.single_mut().dragging {
        let texture_handle = asset_server.load("Player1.png");
        let shape_material = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            base_color: Color::srgba(0.5, 0.5, 0.5, 0.25),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        let mut tower_transform = cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query, &map);
        tower_transform.y = 0.5; // Fixed y position for the cube
        let shape_handle = meshes.add(Cuboid::default());
        commands.spawn((
            Mesh3d(shape_handle.clone()),
            MeshMaterial3d(shape_material.clone()),
            Transform::from_translation(tower_transform),
            TowerPreview{
                droppable_type: DroppableType::Tower,
            },
        ));
    }
    // After the preview is spawned, set the dragging state to true.
    query.single_mut().dragging = true;
}

pub fn check_if_dragging(
    windows: Query<&Window>,
    mut query: Query<&mut TowerDroppable>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev_dropped: EventWriter<DroppableDropped>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
    map: ResMut<Map>,
) {
    let dragging = query.single_mut().dragging;
    if buttons.just_released(MouseButton::Left) {
        let point = cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query, &map);
        let x = (point.x / 1.2).round() * 1.2;
        let y = (point.y / 1.2).round() * 1.2 + 0.5;
        let z = (point.z / 1.2).round() * 1.2;
        let new_point = Vec3::new(x, y, z);
        if dragging {
            query.single_mut().dragging = false;
            ev_dropped.send(DroppableDropped {
                droppable_type: DroppableType::Tower,
                position: new_point,
            });
        }
    }
}

//Tower Preview
#[allow(clippy::too_many_arguments)]
pub fn move_preview(
    windows: Query<&Window>,
    mut query_tower_preview_entity: Query<Entity, With<TowerPreview>>, // Query for TowerPreview entity
    query_tower_preview: Query<&TowerPreview>,
    mut query_tower_preview_transform: Query<&mut Transform>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
    query_material: Query<&mut MeshMaterial3d<StandardMaterial>, With<TowerPreview>>,
    mut assets_standardmaterial: ResMut<Assets<StandardMaterial>>,
    map: ResMut<Map>, // Resource containing tower positions
) {
    let point: Vec3 = game_camera::systems::cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query, &map);
    for tower_preview_entity in query_tower_preview_entity.iter_mut() {
        let tower_preview: &TowerPreview = query_tower_preview.get(tower_preview_entity).unwrap();
        if tower_preview.droppable_type == DroppableType::Tower {
            query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.x = (point.x / 1.2).round() * 1.2;
            query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.y = (point.y / 1.2).round() * 1.2 + 0.5;
            query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.z = (point.z / 1.2).round() * 1.2;
            if map.tower_positions.contains_key(&((point.x / 1.2).round() as i32, (point.y / 1.2).round() as i32, (point.z / 1.2).round() as i32)) {
            //Converting the mouse position to the tile position in multiples of 1.2.
                let material_handle = query_material.get(tower_preview_entity).unwrap();
                let material = assets_standardmaterial.get_mut(material_handle);
                match material {
                    Some(standard_material) => {
                        standard_material.base_color = Color::srgba(1.0, 0.3, 0.3, 0.4)
                    }
                    None => print!("MeshMaterial3d doesn't contain material (shouldn't happen)")
                }
                query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.y += 1.0;
                //Make the new tower appear on top
            } else {
                //Converting the mouse position to the tile position in multiples of 1.2.
                let material_handle = query_material.get(tower_preview_entity).unwrap();
                let material = assets_standardmaterial.get_mut(material_handle);
                match material {
                    Some(standard_material) => {
                        standard_material.base_color = Color::srgba(0.5, 0.5, 0.5, 0.25)
                    }
                    None => print!("MeshMaterial3d doesn't contain material (shouldn't happen)")
                }
                query_tower_preview_transform.get_mut(tower_preview_entity).unwrap().translation.y = (point.y / 1.2).round() * 1.2 + 0.5;
                //Reset tower position back to normal
            }

        }
    }
} 

pub fn check_for_drop( //Check for drop event then delete the tower preview entity.
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

