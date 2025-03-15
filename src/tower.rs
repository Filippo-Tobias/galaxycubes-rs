use bevy::{picking::{pointer::Location, prelude::*}, prelude::*};
use crate::game_camera::GameCamera;
use crate::drop_bar::{DroppableDropped, DroppableType};
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
    println!("hovered");
    ev_hovered.send(TowerHovered{entity: event.target, position: event.pointer_location.clone()});
}

fn on_tower_unhover(event: Trigger<Pointer<Out>>, mut ev_hovered: EventWriter<TowerUnHovered>) {
    ev_hovered.send(TowerUnHovered{entity: event.target, position: event.pointer_location.clone()});
    println!("unhovered")

}

fn on_tower_dragged(event: Trigger<Pointer<Drag>>, mut ev_hovered: EventWriter<TowerDragged>) {
    ev_hovered.send(TowerDragged{entity: event.target});
    println!("dragged")

}


impl Plugin for Tower{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_cube, spawn_cube_on_drop));
        app.add_event::<TowerHovered>();
        app.add_event::<TowerUnHovered>();
        app.add_event::<TowerDragged>();
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        -10.0);
    let tower_transform2 = Transform::from_xyz(
        5.0, 
        0.5, 
        -10.0);

    commands.spawn((
        Tower,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        tower_transform
    ))
    .observe(on_tower_hover)
    .observe(on_tower_unhover)
    .observe(on_tower_dragged);

    let tower_id = commands.spawn((
        Tower,
        Mesh3d(shape_handle.clone()),
        MeshMaterial3d(shape_material.clone()),
        tower_transform2
    ))
    .id();
    commands.entity(tower_id).observe(on_tower_hover);
    commands.entity(tower_id).observe(on_tower_unhover);
    commands.entity(tower_id).observe(on_tower_dragged);
} 

fn move_cube (
    windows: Query<&Window>,
    mut dragged_events: EventReader<TowerDragged>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
    mut tower_query: Query<&mut Transform, With<Tower>>,
    mut tower_dragged: EventReader<TowerDragged>
) {
    let mut dragging = false;
    for _event in dragged_events.read() {
        dragging = true
    };
    if dragging == true{
        let camera = camera_query.single();
        let camera_transform = camera_transform_query.single();
        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        let Some(distance) = ray.intersect_plane(Vec3{x:0.0,y:0.0,z:0.0}, InfinitePlane3d::new(Vec3{x:0.0,y:1.0,z:0.0}))
        else {
            return;
        };
        let point = ray.get_point(distance);
        let mut entity: Option<Entity> = None;
        for event in tower_dragged.read() {
            entity = Some(event.entity)
        };
        let mut tower: Mut<Transform> = tower_query.get_mut(entity.unwrap()).unwrap();
        tower.translation.x = (point.x / 1.2).round() * 1.2;
        tower.translation.z = (point.z / 1.2).round() * 1.2;
        println!("{}", tower.translation);
    }
}

fn spawn_cube_on_drop(
    mut commands: Commands,
    mut ev_dropped: EventReader<DroppableDropped>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for drop in ev_dropped.read() {
        let shape_handle = 
        meshes.add(Cuboid::default());
        let texture_handle = asset_server.load("Player1.png");
        let shape_material = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            ..default()
        });
        if drop.droppable_type == DroppableType::Tower {
            
            commands.spawn((
                Tower,
                Mesh3d(shape_handle.clone()),
                MeshMaterial3d(shape_material.clone()),
                Transform::from_translation(drop.position)
            ))
            .observe(on_tower_hover)
            .observe(on_tower_unhover)
            .observe(on_tower_dragged);
        }
    }
}
            
    
    