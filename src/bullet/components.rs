use bevy::prelude::*;

#[derive(PartialEq)]
pub struct BulletData {
    pub range: f32,
    pub damage: i32,
}

#[derive(PartialEq, Eq)]
pub enum BulletType {
    ShooterPillar,
} 

impl BulletData {
    pub fn shooter_pillar_default() -> Self {
        Self { range: 5., damage: 1 }
    }
}

#[derive(Component)]
pub struct Bullet {
    pub bullet_type: BulletType,
    pub velocity: Vec3,
    pub bullet_data: BulletData,
    pub bullet_origin: Vec3,
}

