use bevy::prelude::*;
use super::components::NewShooterPillar;
use super::systems::*;

pub struct ShooterPillarPlugin;
impl Plugin for ShooterPillarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_event::<NewShooterPillar>();
        app.add_systems(Update, spawn_pillar_on_event);
    }
}
