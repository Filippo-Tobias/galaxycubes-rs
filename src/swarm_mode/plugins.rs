use bevy::prelude::*;
use super::systems;

pub struct SwarmModePlugin;

impl Plugin for SwarmModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::start);
        app.add_systems(Update, systems::spawn_random_enemies);
    }
}
