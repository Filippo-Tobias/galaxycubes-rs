use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*, render::camera::RenderTarget, window::{CursorGrabMode, PrimaryWindow, WindowRef}};
use bevy_mod_picking::pointer::PointerInteraction;

use crate::{tower::{Tower, TowerHovered, TowerUnHovered}, ui::UI};

#[derive(Component)]
pub struct GameCamera;

impl Plugin for GameCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (pan_camera, check_hit));
    }
}

#[derive(Component)]
struct HoverState {
    hovering: bool,
    locked: bool,
}

fn setup(mut commands: Commands) {
    let mut camera_instance = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::new(0.0, 0.0, -6.0), Vec3::Y),
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Primary),
                order: 0,
                ..default()
            },
            ..default()
        },
        GameCamera,
    ));
    camera_instance.insert(HoverState{hovering: false, locked: false});   
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

fn check_if_hovering_element(
    mut query_hover: Query<&mut HoverState, With<GameCamera>>,
    mut tower_hovered_events: EventReader<TowerHovered>,
    mut tower_unhovered_events: EventReader<TowerUnHovered>,
    mut ui_hovered_events: EventReader<HoveredUI>,
    mut ui_unhovered_events: EventReader<UnhoveredUI>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let mut hover = query_hover.single_mut();

    for _event in tower_hovered_events.read() {
        if !mouse_buttons.pressed(MouseButton::Left) {
            hover.hovering = true
        }
    }

    for _event in ui_hovered_events.read() {
        if !mouse_buttons.pressed(MouseButton::Left) {
            hover.hovering = true
        }
    }

    for _event in tower_unhovered_events.read() {
        if !mouse_buttons.pressed(MouseButton::Left) {
            hover.hovering = false;
            println!("unhovered")
        } else {
            hover.locked = true;
            println!("locked")
        }
    }

    for _event in ui_unhovered_events.read() {
        if !mouse_buttons.pressed(MouseButton::Left) {
            hover.hovering = false;
            println!("unhovered")
        } else {
            hover.locked = true;
            println!("locked")
        }
    }

    if !mouse_buttons.pressed(MouseButton::Left) && hover.locked {
        hover.hovering = false;
        hover.locked = false;
        println!("unhovered, unlocked")
    }
}

fn pan_camera(
    mut query_transform: Query<&mut Transform, With<GameCamera>>,
    query_hover: Query<&mut HoverState, With<GameCamera>>,
    query_camera_projection: Query<&mut Projection, With<GameCamera>>,
    mut query_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_event_reader: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let hover_iter = query_hover.iter();
    let mut hover: bool = false;
    let mut transform = query_transform.single_mut();
    let primary_window = query_windows.single_mut();
    let translation_multiplier: f32 = 0.005;
    let bounds = Vec3 { x: 8.0, y: 20.0, z: 8.0};
    let mut total_y_movement: f32 = 0.0;
    for e in mouse_wheel_events.read() {
        total_y_movement += e.y
    }
    zoom_perspective(query_camera_projection, 1.0 - total_y_movement*0.05);
    for hover_state in hover_iter {
        if hover_state.hovering {
            hover = true
        }
    }
    if mouse_buttons.pressed(MouseButton::Left) && !hover{
        cursor_grab(primary_window);
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
        cursor_ungrab(primary_window);
    }
}


fn cursor_grab(
    window: Mut<Window>,
) {
    let mut primary_window = window;
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn cursor_ungrab(
    window: Mut<Window>,
) {
    let mut primary_window = window;
    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;
}

fn check_hit(
    pointer_query: Query<&PointerInteraction>,
    tower_query: Query<&Tower>,
    ui_query: Query<&UI>,
) {
    let mut entity_count 
    for pointer_interaction in pointer_query.iter() {
        if let Some((entity, _hit_data)) = pointer_interaction.get_nearest_hit() {
            // Check if the nearest hit entity has `MyComponent`
            if let Ok(_tower) = tower_query.get(*entity) {
                // Entity has `MyComponent`, so you can process it here
            }
        }
    }
}
