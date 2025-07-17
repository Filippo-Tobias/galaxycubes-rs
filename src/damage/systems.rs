use bevy::prelude::*;
use super::components;
use crate::attack::components::AttackInstance;

use super::components::Health;

pub fn read_damage_events(
    mut ev: EventReader<components::DamageEvent>,
    mut health_query: Query<&mut Health>
) {
    for event in ev.read() {
        let damage;
        match &event.attack_instance {
            AttackInstance::Bullet(bullet) => {
                damage = bullet.bullet_data.damage;
            },
        }
        let result_health = health_query.get_mut(event.target_entity);
        if let Ok(mut health) = result_health {
            health.current_health -= damage
        }
    }
}
