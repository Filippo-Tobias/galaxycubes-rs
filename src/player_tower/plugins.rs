use super::systems;
use super::components;

use bevy::prelude::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::spawn_tower_on_event);
        app.add_systems(Update, (systems::move_cube, systems::spawn_cube_on_drop));
        app.add_event::<components::TowerHovered>();
        app.add_event::<components::TowerUnHovered>();
        app.add_event::<components::TowerDragged>();
        app.add_event::<components::NewTower>();
    }
}

