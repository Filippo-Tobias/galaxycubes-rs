use bevy::prelude::*;
use crate::bullet::components::BulletType;
#[derive(PartialEq, Eq)]
pub enum AttackType{
    Bullet{
        bullet_type: BulletType,
    },
}

#[derive(Component)]
pub struct AttackTimer(pub Timer);

#[derive(Component)]
pub struct Attack{
    pub attack_type: AttackType,
}


impl AttackTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}

