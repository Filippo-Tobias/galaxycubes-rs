use bevy::prelude::*;
use bevy_editor_pls::*;
use bevy_mod_picking::prelude::*;
mod tower;
mod mini_editor;
mod game_camera;
mod level_loader;
mod floor_tile_mesh;
use bevy_mod_picking::prelude::*;
use bevy::render::settings::Backends;
use bevy::render::settings::WgpuSettings;
use bevy::render::settings::RenderCreation;
use bevy::render::RenderPlugin;
fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..Default::default()
            }),
            ..Default::default()
        }),
        EditorPlugin::default(),
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        mini_editor::MiniEditor{editor_open: false},
        tower::Tower,
        game_camera::GameCamera,
        level_loader::LevelLoader,
        dropbar::DropBar,
        dropable::Dropable,
        DefaultPickingPlugins,
        
    ))
    .run()
}


