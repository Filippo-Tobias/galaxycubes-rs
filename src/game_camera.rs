use bevy::{input::mouse::MouseMotion, prelude::*, render::camera::RenderTarget, window::{CursorGrabMode, PrimaryWindow, WindowRef}};

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
            transform: Transform::from_xyz(0.0, 20.0, 9.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
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
) {
    let mut transform = query.single_mut();
    if mouse_buttons.pressed(MouseButton::Left) {
        for event in mouse_event_reader.read() {
            let mut translation_multiplier_x: f32 = 0.01;
            let mut translation_multiplier_z: f32 = 0.01;
            let default_translation_multiplier: f32 = 0.01;
            let e: f32 = 2.7;
    
            if transform.translation.x > 0. {
                if transform.translation.x >= 8. {
                    translation_multiplier_x = 0.01 * e.powf(-(transform.translation.x - 8.0));
                }
            } else {
                if transform.translation.x <= -8. {
                    translation_multiplier_x = 0.01 * e.powf(-(-transform.translation.x - 8.0));
                }
            }

            if transform.translation.z > 0. {
                if transform.translation.z >= 16. {
                    translation_multiplier_z = 0.01 * e.powf(-(transform.translation.z - 16.0));
                }
            } else {
                if transform.translation.z <= -1. {
                    translation_multiplier_z = 0.01 * e.powf(-(-transform.translation.z - 1.0));
                }
            }

            let delta = event.delta;
            if (delta.x + transform.translation.x).abs() > transform.translation.x.abs() { //this checks if going in opposite direction to use the normal speed
                transform.translation.x -= delta.x * default_translation_multiplier;
            }
            if (delta.y + transform.translation.z).abs() > transform.translation.z.abs() { //this checks if going in opposite direction to use the normal speed
                transform.translation.z -= delta.y * default_translation_multiplier;
            }
            transform.translation.x -= delta.x * translation_multiplier_x;
            transform.translation.z -= delta.y * translation_multiplier_z;
            println!("{}", transform.translation);
        }
    }

}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

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
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;
}