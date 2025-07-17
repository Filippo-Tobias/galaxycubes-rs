use bevy::prelude::*;
use crate::bullet::components::{Bullet, BulletType};
#[derive(PartialEq)]
pub enum AttackType{
    Bullet(BulletType)
}

#[derive(Component)]
pub struct AttackTimer(pub Timer);

#[derive(Component)]
pub struct Attack{
    pub attack_type: AttackType,
}

pub enum AttackInstance {
    Bullet(Bullet),
}

impl AttackTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}

