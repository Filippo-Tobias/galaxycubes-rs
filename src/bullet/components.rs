use bevy::prelude::*;

#[derive(PartialEq, Clone)]
pub struct BulletData {
    pub range: f32,
    pub damage: i32,
    pub speed: f32
}

#[derive(PartialEq, Eq, Clone)]
pub enum BulletType {
    ShooterPillar,
} 

impl BulletData {
    pub fn shooter_pillar_default() -> Self {
        Self { range: 5., damage: 1, speed: 10.0 }
    }
}

#[derive(Component, PartialEq, Clone)]
pub struct Bullet {
    #[allow(dead_code)]
    pub bullet_type: BulletType,
    //For now bullet type isn't used but it will likely be used by something eventually
    pub velocity: Vec3,
    pub bullet_data: BulletData,
    pub bullet_origin: Vec3,
}

