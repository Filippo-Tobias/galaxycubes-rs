use super::systems;
use super::components;

use bevy::prelude::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup);
        app.add_systems(Update, (systems::move_cube, systems::spawn_cube_on_drop));
        app.add_event::<components::TowerHovered>();
        app.add_event::<components::TowerUnHovered>();
        app.add_event::<components::TowerDragged>();
    }
}

