use bevy::{picking::{pointer::Location, prelude::*}, prelude::*};
use crate::{game_camera::{self, GameCamera}, level_loader::Map};
use crate::drop_bar::{DroppableDropped, DroppableType};

pub struct TowerPlugin;

impl Plugin for TowerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_cube, spawn_cube_on_drop));
        app.add_event::<TowerHovered>();
        app.add_event::<TowerUnHovered>();
        app.add_event::<TowerDragged>();
    }
}

#[derive(Component)]
pub struct Tower;

#[derive(Event)]
#[allow(dead_code)]
pub struct TowerHovered {
    pub entity: Entity,
    pub position: Location,
}
#[derive(Event)]
#[allow(dead_code)]
pub struct TowerUnHovered {
    pub entity: Entity,
    pub position: Location,
}
#[derive(Event)]
pub struct TowerDragged{
    pub entity: Entity,
}

fn on_tower_hover(event: Trigger<Pointer<Over>>, mut ev_hovered: EventWriter<TowerHovered>) {
    ev_hovered.send(TowerHovered{entity: event.target, position: event.pointer_location.clone()});
}

fn on_tower_unhover(event: Trigger<Pointer<Out>>, mut ev_hovered: EventWriter<TowerUnHovered>) {
    ev_hovered.send(TowerUnHovered{entity: event.target, position: event.pointer_location.clone()});
}

fn on_tower_dragged(event: Trigger<Pointer<Drag>>, mut ev_hovered: EventWriter<TowerDragged>, mut res_locking_camera: ResMut<game_camera::LockingCamera>) {
    ev_hovered.send(TowerDragged{entity: event.target});
    res_locking_camera.list.push(event.target);
}

fn setup(
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
        Tower,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        tower_transform
    ))
    .observe(on_tower_hover)
    .observe(on_tower_unhover)
    .observe(on_tower_dragged)
    .id();

    map.tower_positions.insert(((tower_transform.translation.x / 1.2) as i32 , (tower_transform.translation.z / 1.2) as i32), new_tower_entity);
    //Insert takes v as V (v: V) meaning the entity passed will be copied since the entity trait implements the copy trait.

    let new_tower_entity = commands.spawn((
        Tower,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        tower_transform2
    ))
    .id();
    commands.entity(new_tower_entity).observe(on_tower_hover);
    commands.entity(new_tower_entity).observe(on_tower_unhover);
    commands.entity(new_tower_entity).observe(on_tower_dragged);
    map.tower_positions.insert(((tower_transform2.translation.x / 1.2) as i32 , (tower_transform2.translation.z / 1.2) as i32), new_tower_entity);

    
} 

fn move_cube (
    windows: Query<&Window>,
    mut dragged_events: EventReader<TowerDragged>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
    mut tower_query: Query<&mut Transform, With<Tower>>,
    mut tower_dragged: EventReader<TowerDragged>,
    mut map: ResMut<Map>, // Resource containing tower positions
    mut res_locking_camera: ResMut<game_camera::LockingCamera>,
    query_tower_entity: Query<Entity, With<Tower>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let mut dragging = false;
    for _event in dragged_events.read() {
        dragging = true;
    };
    if dragging == true{
        let point: Vec3 = game_camera::cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query);
        let mut option_entity: Option<Entity> = None;
        for event in tower_dragged.read() {
            option_entity = Some(event.entity)
        };
        let mut transform_tower: Mut<Transform> = tower_query.get_mut(option_entity.unwrap()).unwrap();
        map.tower_positions.remove(&((transform_tower.translation.x / 1.2) as i32,(transform_tower.translation.z / 1.2) as i32));
        transform_tower.translation.x = (point.x / 1.2).round() * 1.2;
        transform_tower.translation.z = (point.z / 1.2).round() * 1.2;
        map.tower_positions.insert(((transform_tower.translation.x / 1.2) as i32,(transform_tower.translation.z / 1.2) as i32), option_entity.unwrap());
        println!("{}", transform_tower.translation);
    } else if mouse_buttons.pressed(MouseButton::Left) == false { //if not dragging stop locking camera.
        for entity in query_tower_entity.iter(){
            res_locking_camera.list.retain(|x| x != &entity);
        }
    }
}

fn spawn_cube_on_drop(
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
        let point = game_camera::cursor_ray_to_plane(&query_window, &query_camera, &query_camera_transform);
        if drop.droppable_type == DroppableType::Tower && map.tower_positions.contains_key(&((point.x / 1.2).round() as i32, (point.z / 1.2).round() as i32)) == false {
            {
            let new_tower_entity = commands.spawn((
                Tower,
                Mesh3d(shape_handle.clone()),
                MeshMaterial3d(shape_material.clone()),
                Transform::from_translation(drop.position)
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
    