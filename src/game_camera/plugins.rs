use bevy::prelude::*;
use super::systems::setup_game_camera;
use super::systems::setup_ui_camera;
use super::systems::pan_camera;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_camera);
        //let camera_systems_set = pan_camera;
        app.add_systems(Update, (
            //pan_camera.run_if(not(skip_next_frame)),
            pan_camera,
            //IntoSystem::into_system(reset_camera_skip).after(camera_systems_set),
        ));
        //app.insert_resource(LockingCamera{list: Vec::new()});
    }
}

pub struct UICameraPlugin;

impl Plugin for UICameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_camera);
    }
}
