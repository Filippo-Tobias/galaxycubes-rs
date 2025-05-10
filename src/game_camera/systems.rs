use bevy::prelude::*;

use bevy::render::camera::ScalingMode;
use bevy::{input::mouse::{MouseMotion, MouseWheel}, render::camera::RenderTarget, window::{CursorGrabMode, PrimaryWindow, WindowRef}};
use super::components::GameCamera;
use crate::level_loader::MapDragged;
 // GameCamera
pub fn setup_game_camera(mut commands: Commands) {
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

pub fn pan_camera(
    mut query_transform: Query<&mut Transform, With<GameCamera>>,
    query_camera_projection: Query<&mut Projection, With<GameCamera>>,
    mut query_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_event_reader: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut evr_map_drag: EventReader<MapDragged>,
) {
    let mut transform = query_transform.single_mut();
    let translation_multiplier: f32 = 0.005;
    let bounds = Vec3 { x: 8.0, y: 20.0, z: 8.0};
    let mut total_y_movement: f32 = 0.0;
    for ev in mouse_wheel_events.read() {
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                println!("line")
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => {
                println!("pixel")
            }
        }
        total_y_movement += ev.y
    }
    zoom_perspective(query_camera_projection, 1.0 - total_y_movement*0.05);

    let mut primary_window = query_windows.single_mut();
    if evr_map_drag.read().peekable().peek().is_none() {
        if !mouse_buttons.pressed(MouseButton::Left) {
            cursor_ungrab(&mut primary_window);
        }
        return
    }

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
    ray.get_point(distance)
}

//UI Camera

pub fn setup_ui_camera(
    mut commands: Commands
) {
    let camera2d = (
        Camera2d,
        Camera {
            order: 1,
            target: RenderTarget::Window(WindowRef::Primary),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        IsDefaultUiCamera,
        OrthographicProjection{
            scaling_mode: ScalingMode::Fixed { width: 1920.0 , height: 1080.0 }, 
            ..OrthographicProjection::default_2d()
        }
    );

    commands.spawn(camera2d);
}
