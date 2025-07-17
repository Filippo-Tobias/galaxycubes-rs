use bevy::prelude::*;

use std::collections::HashSet;
use super::components;
use crate::{damage::components::Health, game_camera::components::GameCamera, level_loader::Map, range_system::components::DirtyPosition};
use crate::drag_and_drop::components::{DroppableDropped, DroppableType};
use crate::game_camera;

fn on_tower_hover(event: Trigger<Pointer<Over>>, mut ev_hovered: EventWriter<components::TowerHovered>) {
    ev_hovered.send(components::TowerHovered{entity: event.target, position: event.pointer_location.clone()});
}

fn on_tower_unhover(event: Trigger<Pointer<Out>>, mut ev_hovered: EventWriter<components::TowerUnHovered>) {
    ev_hovered.send(components::TowerUnHovered{entity: event.target, position: event.pointer_location.clone()});
}
fn on_tower_dragged(event: Trigger<Pointer<Drag>>, mut ev_hovered: EventWriter<components::TowerDragged>) {
    ev_hovered.send(components::TowerDragged{entity: event.target});
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map: ResMut<Map>
) {
    let texture_handle = asset_server.load("Player1.png");

    let shape_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });
    

    let shape_handle = 
        meshes.add(Cuboid::default());
    let tower_transform = Transform::from_xyz(
        0.0, 
        0.5, 
        -12.0);
    let tower_transform2 = Transform::from_xyz(
        6.0, 
        0.5, 
        -12.0);

    let new_tower_entity = commands.spawn((
        components::Tower,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        tower_transform,
        Health::player_tower_default()
    ))
    .observe(on_tower_hover)
    .observe(on_tower_unhover)
    .observe(on_tower_dragged)
    .id();

    map.tower_positions.insert(((tower_transform.translation.x / 1.2) as i32 , (tower_transform.translation.z / 1.2) as i32), new_tower_entity);
    //Insert takes v as V (v: V) meaning the entity passed will be copied since the entity trait implements the copy trait.

    let new_tower_entity = commands.spawn((
        components::Tower,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        tower_transform2,
        Health::player_tower_default()
    ))
    .id();
    commands.entity(new_tower_entity).observe(on_tower_hover);
    commands.entity(new_tower_entity).observe(on_tower_unhover);
    commands.entity(new_tower_entity).observe(on_tower_dragged);
    map.tower_positions.insert(((tower_transform2.translation.x / 1.2) as i32 , (tower_transform2.translation.z / 1.2) as i32), new_tower_entity);
} 

#[allow(clippy::too_many_arguments)]
pub fn move_cube (
    windows: Query<&Window>,
    mut dragged_events: EventReader<components::TowerDragged>,
    mut commands: Commands,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
    mut tower_query: Query<&mut Transform, With<components::Tower>>,
    query_dirty_position: Query<&DirtyPosition>,
    mut tower_dragged: EventReader<components::TowerDragged>,
    mut map: ResMut<Map>, // Resource containing tower positions
) {
    let mut dragging = false;
    for event in dragged_events.read() {
        dragging = true;
        if !query_dirty_position.contains(event.entity) {
            commands.entity(event.entity).insert(DirtyPosition);
        }
    };
    if dragging {
        let point: Vec3 = game_camera::systems::cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query);
        let entities: HashSet<Entity> = tower_dragged.read().map(|event| event.entity).collect();
        for entity in entities {
            if let Ok(mut tower_transform) = tower_query.get_mut(entity){
                map.tower_positions.remove(&((tower_transform.translation.x / 1.2) as i32,(tower_transform.translation.z / 1.2) as i32));
                tower_transform.translation.x = (point.x / 1.2).round() * 1.2;
                tower_transform.translation.z = (point.z / 1.2).round() * 1.2;
                map.tower_positions.insert(((tower_transform.translation.x / 1.2) as i32,(tower_transform.translation.z / 1.2) as i32), entity);
            };
        }

    } 
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_cube_on_drop(
    mut commands: Commands,
    mut ev_dropped: EventReader<DroppableDropped>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map: ResMut<Map>,
    query_window: Query<&Window>,
    query_camera: Query<&Camera, With<GameCamera>>,
    query_camera_transform: Query<&GlobalTransform, With<GameCamera>>,
) {
    for drop in ev_dropped.read() {
        let shape_handle = meshes.add(Cuboid::default());
        let texture_handle = asset_server.load("Player1.png");
        let shape_material = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            ..default()
        });
        let point = game_camera::systems::cursor_ray_to_plane(&query_window, &query_camera, &query_camera_transform);
        if drop.droppable_type == DroppableType::Tower && !map.tower_positions.contains_key(&((point.x / 1.2).round() as i32, (point.z / 1.2).round() as i32)) {
            {
                let new_tower_entity = commands.spawn((
                    components::Tower,
                    Mesh3d(shape_handle.clone()),
                    MeshMaterial3d(shape_material.clone()),
                    Transform::from_translation(drop.position),
                    Health{max_health: 10, current_health: 10},

                ))
                .observe(on_tower_hover)
                .observe(on_tower_unhover)
                .observe(on_tower_dragged)
                .id();
                map.tower_positions.insert(((drop.position.x / 1.2) as i32 , (drop.position.z / 1.2) as i32), new_tower_entity);
            }
        }
    }
            
}
    

