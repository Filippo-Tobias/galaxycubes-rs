use bevy::prelude::*;

#[derive(Component)]
pub struct ShooterPillar{
    health: i32,
    attack_cooldown: f32,
}

impl Default for ShooterPillar {
    fn default() -> Self {
        Self{
            health: 100,
            attack_cooldown: 2.0
        }
    }
}
