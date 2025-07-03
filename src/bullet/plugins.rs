use bevy::prelude::*;

use super::systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App) {
       app.add_systems(Update, check_timers); 
       app.add_systems(Update, move_bullets); 
    }
}
