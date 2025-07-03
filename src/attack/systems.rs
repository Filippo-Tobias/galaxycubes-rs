use bevy::prelude::*;
use super::components::AttackTimer;
pub fn tick_timers(
    mut query_attack_timers: Query<&mut AttackTimer>,
    time: Res<Time>
) {
    for mut attack_timer in query_attack_timers.iter_mut() {
        attack_timer.0.tick(time.delta());
    }
}
