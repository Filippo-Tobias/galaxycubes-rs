use bevy::prelude::*;

use super::systems::*;

pub struct ShooterPillarPlugin;
impl Plugin for ShooterPillarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, move_bullets);
        app.add_systems(Update, check_timers);
    }
}
