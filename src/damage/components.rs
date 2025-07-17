use bevy::prelude::*;

use crate::attack::components::AttackInstance;

#[derive(Event)]
pub struct DamageEvent {
    pub attack_instance: AttackInstance,
    pub target_entity: Entity,
}

#[derive(Component)]
pub struct Health {
    pub current_health: i32,
    #[allow(dead_code)]
    pub max_health: i32,
    //Not read for now but eventually will be used
}

impl Health {
    pub fn shooter_pillar_default() -> Self {
        Self{current_health: 5, max_health: 5}
    }
    pub fn player_tower_default() -> Self {
        Self{current_health: 10, max_health: 10}
    }
}
