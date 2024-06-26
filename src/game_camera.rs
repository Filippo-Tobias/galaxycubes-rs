use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*, render::camera::RenderTarget, window::{CursorGrabMode, PrimaryWindow, WindowRef}};

#[derive(Component)]
pub struct GameCamera;

impl Plugin for GameCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, pan_camera);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::new(0.0, 0.0, -6.0), Vec3::Y),
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Primary),
                ..default()
            },
            ..default()
        },
        GameCamera,
    ));
}

fn pan_camera(
    mut query: Query<&mut Transform, With<GameCamera>>,
    mut mouse_event_reader: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let mut transform = query.single_mut();
    let primary_window = q_windows.single_mut();
    let translation_multiplier: f32 = 0.005;
    let mut bounds = Vec3 { x: 8.0, y: 20.0, z: 8.0};
    let rotation = transform.rotation;
    let angle_to_y = rotation.to_euler(EulerRot::YXZ).1;

    for e in mouse_wheel_events.read() {
        let delta_y = e.y * angle_to_y.sin(); // Calculate the change in y and z based on the mouse wheel input
        let delta_z = e.y * angle_to_y.cos();
        let potential_y = transform.translation.y + delta_y; // Calculate the potential new y position
        if potential_y >= 6. && potential_y <= bounds.y { // Check if the potential new y position is within bounds
            transform.translation.y = potential_y; // If within bounds, apply both y and z movements
            transform.translation.z -= delta_z;
        } else {
            let max_y_movement = if potential_y < 6. { // If out of bounds, determine the maximum allowed movement on the y axis
                6. - transform.translation.y
            } else {
                bounds.y - transform.translation.y
            };
            let proportion = max_y_movement / delta_y; // Calculate the proportion of e.y that is allowed
            transform.translation.y += max_y_movement; // Apply the maximum allowed movement on the y axis
            transform.translation.z -= proportion * delta_z; // Adjust the z axis proportionally
        }
        transform.translation.y = transform.translation.y.clamp(6., bounds.y); // Clamp the y position to ensure it stays within bounds
    }
    
    if mouse_buttons.pressed(MouseButton::Left) {
        cursor_grab(primary_window);
        for event in mouse_event_reader.read() {
            let delta = event.delta;

            // Update camera translation
            bounds.z = (80.0 / (f32::max(transform.translation.y/2.0, 4.0))).abs();
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

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor.visible = false;
}

fn cursor_ungrab(
    window: Mut<Window>,
) {
    let mut primary_window = window;
    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;
}