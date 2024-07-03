use std::ptr::null;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use pointer::Location;

use crate::game_camera::GameCamera;

#[derive(Component)]
pub struct Tower;

#[derive(Event)]
pub struct TowerHovered {
    pub entity: Entity,
    pub position: Location,
}
#[derive(Event)]
pub struct TowerUnHovered {
    pub entity: Entity,
    pub position: Location,
}
#[derive(Event)]
pub struct TowerDragged{
    pub entity: Entity,
}

#[derive(Component)]
pub struct TowerMovement {
    moving: bool
}

impl From<ListenerInput<Pointer<Out>>> for TowerUnHovered {
    fn from(input: ListenerInput<Pointer<Out>>) -> Self {
        TowerUnHovered {
            entity: input.target,
            position: input.pointer_location.clone()
        }
    }
}

impl From<ListenerInput<Pointer<Over>>> for TowerHovered {
    fn from(input: ListenerInput<Pointer<Over>>) -> Self {
        TowerHovered {
            entity: input.target,
            position: input.pointer_location.clone()
        }
    }
}

impl From<ListenerInput<Pointer<Drag>>> for TowerDragged {
    fn from(input: ListenerInput<Pointer<Drag>>) -> Self {
        TowerDragged {
            entity: input.target,
        }
    }
}

impl Plugin for Tower{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_cube);
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
    windows: Query<&Window>,
) {
    let texture_handle = asset_server.load("Player1.png");

    let shape_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });
    

    let shape_handle = 
        meshes.add(Cuboid::default());
    let cos_pi_8 = (2.0 + (2.0_f32).sqrt()).sqrt() / 2.0;
    let sin_pi_8 = (2.0 - (2.0_f32).sqrt()).sqrt() / 2.0;
    let rotation_quat = Quat::from_xyzw(cos_pi_8, 0.0, 0.0, sin_pi_8);
    let euler_angles: (f32, f32, f32) = rotation_quat.to_euler(EulerRot::YXZ);
    let tower_transform = Transform::from_xyz(
        0.0, 
        0.5, 
        -10.0);
    let tower_transform2 = Transform::from_xyz(
        5.0, 
        0.5, 
        -10.0);
    commands.spawn((
        PbrBundle {
            mesh: shape_handle.clone(),
            material: shape_material.clone(),
            transform: tower_transform,
            ..default()
        }, Tower,
        On::<Pointer<Over>>::send_event::<TowerHovered>(),
        On::<Pointer<Out>>::send_event::<TowerUnHovered>(),
        On::<Pointer<Drag>>::send_event::<TowerDragged>(),
    ));
        
    let mut tower = commands.spawn((
        PbrBundle {
            mesh: shape_handle.clone(),
            material: shape_material.clone(),
            transform: tower_transform2,
            ..default()
        }, Tower,
        On::<Pointer<Over>>::send_event::<TowerHovered>(),
        On::<Pointer<Out>>::send_event::<TowerUnHovered>(),
        On::<Pointer<Drag>>::send_event::<TowerDragged>(),
    ));
    tower.insert(TowerMovement{moving: false}); 
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
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        let Some(distance) = ray.intersect_plane(Vec3{x:0.0,y:0.0,z:0.0}, Plane3d::new(Vec3{x:0.0,y:1.0,z:0.0}))
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