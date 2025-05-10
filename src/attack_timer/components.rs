use bevy::prelude::*;

#[derive(Component)]
pub struct AttackTimer(pub Timer);

impl  AttackTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}

