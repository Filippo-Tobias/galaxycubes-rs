use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*, render::camera::RenderTarget, window::{CursorGrabMode, PrimaryWindow, WindowRef}};

use crate::level_loader::MapDragged;

//use crate::tower::{TowerHovered, TowerUnHovered};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        //let camera_systems_set = pan_camera;
        app.add_systems(Update, (
            //pan_camera.run_if(not(skip_next_frame)),
            pan_camera,
            //IntoSystem::into_system(reset_camera_skip).after(camera_systems_set),
        ));
        //app.insert_resource(LockingCamera{list: Vec::new()});
    }
}

// #[derive(PartialEq)]
// enum CameraPanState {
//     IsPanning,
//     NotPanning
// }

// fn skip_next_frame(
//     res_locking_camera: Res<LockingCamera>,
// ) -> bool {
//     return res_locking_camera.skipFrame;
// }

// fn reset_camera_skip(
//     mut res_locking_camera: ResMut<LockingCamera>,
// ) {
//     println!("{}", res_locking_camera.skipFrame);
//     res_locking_camera.skipFrame = false;
//     println!("{}", res_locking_camera.skipFrame);

// }

// #[derive(Resource)]
// pub struct LockingCamera{
//     pub list: Vec<Entity>,
// }

#[derive(Component)]
pub struct GameCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::new(0.0, 0.0, -6.0), Vec3::Y),
        Camera {
            order: 0,
            target: RenderTarget::Window(WindowRef::Primary),
            ..default()
        },
        GameCamera,
    ));
}

fn zoom_perspective(
    mut query_camera: Query<&mut Projection, With<GameCamera>>,
    zoom: f32
) {
    let Projection::Perspective(perspective) = query_camera.single_mut().into_inner() else {
        return;
    };
    // zoom in
    //println!("{}", perspective.fov);
    perspective.fov *= zoom;
    perspective.fov = perspective.fov.clamp(0.5, 1.0);
}

fn pan_camera(
    mut query_transform: Query<&mut Transform, With<GameCamera>>,
    query_camera_projection: Query<&mut Projection, With<GameCamera>>,
    mut query_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_event_reader: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut evr_map_drag: EventReader<MapDragged>,
) {
    let mut primary_window = query_windows.single_mut();
    if evr_map_drag.read().peekable().peek().is_none() {
        if mouse_buttons.pressed(MouseButton::Left) == false {
            cursor_ungrab(&mut primary_window);
        }
        return
    }

    let mut transform = query_transform.single_mut();
    let translation_multiplier: f32 = 0.005;
    let bounds = Vec3 { x: 8.0, y: 20.0, z: 8.0};
    let mut total_y_movement: f32 = 0.0;
    for e in mouse_wheel_events.read() {
        total_y_movement += e.y
    }
    zoom_perspective(query_camera_projection, 1.0 - total_y_movement*0.05);

    if mouse_buttons.pressed(MouseButton::Left) {
        cursor_grab(&mut primary_window);
        for event in mouse_event_reader.read() {
            let delta = event.delta;

            // Update camera translation
            transform.translation.x -= delta.x * translation_multiplier;
            transform.translation.z -= delta.y * translation_multiplier;

            // Clamp translation within bounds
            transform.translation.x = transform.translation.x.clamp(-bounds.x, bounds.x);
            transform.translation.z = transform.translation.z.clamp(-bounds.z, bounds.z);
        }
    } else {

    }
}


fn cursor_grab(
    window: &mut Mut<Window>,
) {
    let primary_window = window;
    primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor_options.visible = false;
}

fn cursor_ungrab(
    window: &mut Mut<Window>,
) {
    let primary_window = window;
    primary_window.cursor_options.grab_mode = CursorGrabMode::None;
    primary_window.cursor_options.visible = true;
}

/// Returns the point on the plane where the cursor is pointing at.
pub fn cursor_ray_to_plane(
    windows: &Query<&Window>,
    camera_query: &Query<&Camera, With<GameCamera>>,
    camera_transform_query: &Query<&GlobalTransform, With<GameCamera>>,
) -> Vec3 {
    let camera = camera_query.single();
    let camera_transform = camera_transform_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return Vec3::ZERO;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return Vec3::ZERO;
    };
    let Some(distance) = ray.intersect_plane(Vec3{x:0.0,y:0.0,z:0.0}, InfinitePlane3d::new(Vec3{x:0.0,y:1.0,z:0.0}))
    else {
        return Vec3::ZERO;
    };
    let point = ray.get_point(distance);
    point
}