use bevy::prelude::*;

#[derive(PartialEq, Eq)]
pub enum BulletType {
    ShooterPillar,
} 

#[derive(Component)]
pub struct Bullet {
    pub bullet_type: BulletType,
    pub velocity: Vec3,
}

