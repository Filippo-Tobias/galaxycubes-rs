use bevy::prelude::*;

#[derive(Component)]
pub struct ShooterPillar;


#[derive(Component)]
pub struct ShooterPillarBullet{
    pub velocity: Vec3
}
