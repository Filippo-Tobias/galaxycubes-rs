use bevy::prelude::*;
use crate::shooter_pillar::components::ShooterPillar;
use crate::attack::components::{Attack, AttackTimer, AttackType};
use super::bullet_mesh;
use super::components::{Bullet, BulletData, BulletType};

fn spawn_bullet(
    res_meshes: &mut ResMut<Assets<Mesh>>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    bullet_type: BulletType,
    bullet_data: BulletData,
) {
    let material_handle: Handle<StandardMaterial>;
    let bullet_mesh: Handle<Mesh>; 
    let velocity: Vec3;
    match bullet_type {
        BulletType::ShooterPillar => {
            bullet_mesh = bullet_mesh::create_bullet_mesh(res_meshes);
            material_handle = materials.add(StandardMaterial {
                base_color: Color::srgb(1., 1., 1.),
                ..Default::default()
            });
            velocity = Vec3{x: 0.01, y: 0.00, z: 0.00};
        }
    }
    commands.spawn((
        Mesh3d(bullet_mesh.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_translation(position),
        Bullet{bullet_type, velocity, bullet_data, bullet_origin: position}
    ));
}

pub fn check_timers(
    query_components: Query<(&AttackTimer, &Transform, &Attack), With<ShooterPillar>>,
    mut res_meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    for components in query_components.iter() {
        let attack_timer = components.0;
        let transform = components.1;
        if attack_timer.0.finished() {
            match &components.2.attack_type {
                AttackType::Bullet { bullet_type } => {
                    match bullet_type {
                        BulletType::ShooterPillar =>{
                            spawn_bullet(&mut res_meshes, &mut commands, &mut materials, transform.translation, BulletType::ShooterPillar, BulletData::shooter_pillar_default());
                        }
                    }
                }
            };
        }
    }
}

pub fn move_bullets (
    mut query_bullet: Query<(&mut Transform, &Bullet, Entity)>,
    mut commands: Commands,
) {
    for (mut bullet_transform, bullet, bullet_entity) in query_bullet.iter_mut() {
        if bullet_transform.translation.distance(bullet.bullet_origin) > bullet.bullet_data.range * 1.2 {
            commands.entity(bullet_entity).despawn();
        }
        bullet_transform.translation += bullet.velocity
    };
}
