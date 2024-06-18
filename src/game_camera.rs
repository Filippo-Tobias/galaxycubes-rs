/* use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};
#[derive(Component)]
pub struct GameCamera;

impl Plugin for GameCamera{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, pan_camera);
    }
}

fn setup(
    mut commands: Commands
){
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        camera: Camera{
            target: RenderTarget::Window(WindowRef::Primary),
            ..default()
        },
        ..default()
    });
}

fn pan_camera(){

}
*/

use bevy::{input::mouse::MouseMotion, prelude::*, render::camera::RenderTarget, window::WindowRef};

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
            transform: Transform::from_xyz(0.0, 6.0, 6.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
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
            let delta = event.delta;
            transform.translation.x -= delta.x * 0.01;
            transform.translation.z -= delta.y * 0.01;
        }
    }
}