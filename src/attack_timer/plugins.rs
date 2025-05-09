use bevy::prelude::*;

use super::systems;

pub struct AttackTimerPlugin;

impl Plugin for AttackTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::tick_timers);
    }
}
