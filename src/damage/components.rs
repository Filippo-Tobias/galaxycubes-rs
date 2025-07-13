use bevy::prelude::*;

#[derive(Component)]
pub struct Damageable {
    pub immune: bool,
}

#[derive(Component)]
pub struct Health {
    pub current_health: i32,
    pub max_health: i32,
}

impl Health {
    pub fn shooter_pillar_default() -> Self {
        Self{current_health: 5, max_health: 5}
    }
}
