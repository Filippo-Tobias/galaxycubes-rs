use bevy::prelude::*;
mod player_tower;
mod shooter_pillar;
mod range_system;
mod attack_timer;
use player_tower::tower;
mod mini_editor;
mod game_camera;
mod game_camera_old;
mod level_loader;
mod floor_tile_mesh;
mod drag_and_drop;
use player_tower::tower_preview;
use drag_and_drop::plugins::*;
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
        game_camera::plugins::GameCameraPlugin,
        level_loader::LevelLoaderPlugin,
        MeshPickingPlugin,
        game_camera::plugins::UICameraPlugin,
        tower_preview::TowerPreviewPlugin,
        shooter_pillar::plugins::ShooterPillarPlugin,
        range_system::plugins::RangeSystemPlugin,
        attack_timer::plugins::AttackTimerPlugin,
        DragAndDrop,


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
