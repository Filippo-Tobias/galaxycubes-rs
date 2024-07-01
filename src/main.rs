use bevy::prelude::*;
use bevy_editor_pls::*;
mod tower;
mod mini_editor;
mod game_camera;
mod level_loader;
mod floor_tile_mesh;
fn main() {
    App::new()
    .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),
        EditorPlugin::default(),
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        mini_editor::MiniEditor{editor_open: false},
        tower::Tower,
        game_camera::GameCamera,
        level_loader::LevelLoader,
        floor_tile_mesh::FloorTileMesh,
    ))
    .run()
}


