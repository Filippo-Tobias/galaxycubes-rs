use bevy::prelude::*;

use super::systems;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::tick_timers);
    }
}
