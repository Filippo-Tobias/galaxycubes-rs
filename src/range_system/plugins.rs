use bevy::prelude::*;
use super::systems; 
pub struct RangeSystemPlugin;

impl Plugin for RangeSystemPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Update, systems::check_ranges); 
    }
}

