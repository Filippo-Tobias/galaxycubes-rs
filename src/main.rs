use bevy::prelude::*;
mod tower;
mod mini_editor;
mod game_camera;
mod ui_camera;
mod level_loader;
mod floor_tile_mesh;
mod drop_bar;
mod tower_droppable;
mod tower_preview;
use bevy::render::settings::Backends;
use bevy::render::settings::WgpuSettings;
use bevy::render::settings::RenderCreation;
use bevy::render::RenderPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin; // Currently not used, but can be used to load assets from the binary. Use when building executable.
fn main() {
    App::new()
    .add_plugins((   
        EmbeddedAssetPlugin{mode: bevy_embedded_assets::PluginMode::ReplaceDefault},
        //mini_editor::MiniEditor{editor_open: false},
        tower::TowerPlugin,
        game_camera::GameCamera,
        level_loader::LevelLoaderPlugin,
        MeshPickingPlugin,
        drop_bar::DropBarPlugin,
        ui_camera::UICameraPlugin,
        tower_droppable::TowerDroppablePlugin,
        DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: {Some(Backends::BROWSER_WEBGPU); Some(Backends::VULKAN)},
                ..Default::default()
            }),
            ..Default::default()
        }),
        
        
    ))
    .run();
}
