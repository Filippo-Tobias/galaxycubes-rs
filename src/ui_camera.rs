use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};

pub struct UICameraPlugin;

impl Plugin for UICameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands
) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            target: RenderTarget::Window(WindowRef::Primary),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        IsDefaultUiCamera,
    ));
}
