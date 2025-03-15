use bevy::prelude::*;
mod tower;
mod mini_editor;
mod game_camera;
mod level_loader;
mod floor_tile_mesh;
use bevy::render::settings::Backends;
use bevy::render::settings::WgpuSettings;
use bevy::render::settings::RenderCreation;
use bevy::render::RenderPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
fn main() {
    App::new()
    .add_plugins((   
        EmbeddedAssetPlugin{mode: bevy_embedded_assets::PluginMode::ReplaceDefault},
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        //mini_editor::MiniEditor{editor_open: false},
        tower::Tower,
        game_camera::GameCamera,
        level_loader::LevelLoader,
        MeshPickingPlugin,
        DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: {Some(Backends::BROWSER_WEBGPU); Some(Backends::VULKAN); Some(Backends::GL)},
                
                ..Default::default()
            }),
            ..Default::default()
        }),
        
        
    ))
    .run();
}
