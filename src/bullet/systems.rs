use bevy::prelude::*;
use crate::damage::components::DamageEvent;
use crate::shooter_pillar::components::ShooterPillar;
use crate::attack::components::{Attack, AttackInstance, AttackTimer, AttackType};
use super::bullet_mesh;
use super::components::{Bullet, BulletData, BulletType};
use crate::level_loader::Map;
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
            velocity = Vec3{x: bullet_data.speed, y: 0.0, z: 0.0};
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
                AttackType::Bullet(bullet_type) => {
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
    map: Res<Map>,
    time: Res<Time>,
    mut ev_writer: EventWriter<DamageEvent>,
) {
    for (mut bullet_transform, bullet, bullet_entity) in query_bullet.iter_mut() {
        if bullet_transform.translation.distance(bullet.bullet_origin) > bullet.bullet_data.range * 1.2 {
            commands.entity(bullet_entity).despawn();
        };
        let map_coordinate = bullet_transform.translation / Vec3::new(1.2, 1.2, 1.2);
        let hashmap_key = (map_coordinate.x.round() as i32, map_coordinate.y.round() as i32, map_coordinate.z.round() as i32);
        if map.tower_positions.contains_key(&hashmap_key) && bullet_transform.translation.distance(bullet.bullet_origin) > 1.2 {
            commands.entity(bullet_entity).despawn();
            if let Some(entity) = map.tower_positions.get(&hashmap_key) {
                ev_writer.send(DamageEvent { attack_instance: AttackInstance::Bullet(bullet.clone()), target_entity: *entity });
            }
        };
        bullet_transform.translation += bullet.velocity * Vec3{x: time.delta().as_secs_f32(),y: time.delta().as_secs_f32(),z: time.delta().as_secs_f32()};
    };
}
