use super::components;
use bevy::prelude::*;


pub fn check_ranges(
    mut commands: Commands,
    query_entity_transform: Query<(Entity, &Transform), With<components::DirtyPosition>>,
    mut query_range_areas: Query<&mut components::RangeArea>,
){
    for (entity, transform) in query_entity_transform.iter() {
        let mut flag_remove_dirty = false;
        for mut range_area in query_range_areas.iter_mut(){
            if range_area.range.0.contains(&((transform.translation.x/1.2) as i32))
            & range_area.range.1.contains(&((transform.translation.z/1.2) as i32)) {
                flag_remove_dirty = true;
                range_area.entities.push(entity);
            } else {
                range_area.entities.retain(|x| *x != entity);
                flag_remove_dirty = true;
            }
        }
        if flag_remove_dirty {
            commands.entity(entity).remove::<components::DirtyPosition>();
        }
    } 
}
