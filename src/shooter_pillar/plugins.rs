use bevy::prelude::*;

use super::systems::*;

pub struct ShooterPillarPlugin;
impl Plugin for ShooterPillarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
