use bevy::prelude::*;
use super::systems;
use super::components;
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::read_damage_events);
        app.add_event::<components::DamageEvent>();
    }
}
