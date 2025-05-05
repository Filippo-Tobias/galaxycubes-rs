use bevy::prelude::*;
mod player_tower;
mod shooter_pillar;
mod game_systems;
use game_systems::range_system;
use player_tower::tower;
mod mini_editor;
mod game_camera;
mod ui_camera;
mod level_loader;
mod floor_tile_mesh;
mod drop_bar;
use player_tower::tower_droppable;
use player_tower::tower_preview;
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
        game_camera::GameCameraPlugin,
        level_loader::LevelLoaderPlugin,
        MeshPickingPlugin,
        drop_bar::DropBarPlugin,
        ui_camera::UICameraPlugin,
        tower_droppable::TowerDroppablePlugin,
        tower_preview::TowerPreviewPlugin,
        shooter_pillar::pillar::ShooterPillarPlugin,
        range_system::RangeSystemPlugin,


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
                backends: {Some(Backends::VULKAN)},
                ..Default::default()
            }),
            ..Default::default()
        }),
        
        
    ))
    .run();
}
