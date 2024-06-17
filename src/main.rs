use bevy::prelude::*;
use bevy_editor_pls::*;

mod card_container;
mod mini_editor;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins,
        EditorPlugin::default(),
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        card_container::CardContainer,
        mini_editor::MiniEditor{editor_open: false}
    ))
    .add_systems(Startup, setup_cam)
    .run()
}

fn setup_cam(
    mut commands: Commands,
) {
    let camera = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };
    commands.spawn(camera);
}