use bevy::prelude::*;
use super::systems::setup_game_camera;
use super::systems::setup_ui_camera;
use super::systems::pan_camera;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_camera);
        app.add_systems(Update, (
            pan_camera,
        ));
    }
}

pub struct UICameraPlugin;

impl Plugin for UICameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_camera);
    }
}
