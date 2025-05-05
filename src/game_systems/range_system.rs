use std::ops::RangeInclusive;

use bevy::prelude::*;

pub struct RangeSystemPlugin;

impl Plugin for RangeSystemPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Update, check_ranges); 
    }
}
#[derive(Component)]

pub struct RangeArea{
    pub range: (RangeInclusive<i32>, RangeInclusive<i32>),
    pub entities: Vec<Entity>
}

#[derive(Component)]
pub struct DirtyPosition;

fn check_ranges(
    mut commands: Commands,
    query_entity_transform: Query<(Entity, &Transform), With<DirtyPosition>>,
    mut query_range_areas: Query<&mut RangeArea>,
){
    for (entity, transform) in query_entity_transform.iter() {
        let mut flag_remove_dirty = false;
        for mut range_area in query_range_areas.iter_mut(){
            if range_area.range.0.contains(&((transform.translation.x/1.2) as i32))
            & range_area.range.1.contains(&((transform.translation.z/1.2) as i32)) {
                flag_remove_dirty = true;
                range_area.entities.push(entity);
                println!("within range")
            } else {
                if range_area.entities.contains(&entity){
                    println!("left range")
                }
                range_area.entities.retain(|x| *x != entity);
            }
        }
        if flag_remove_dirty {
            commands.entity(entity).remove::<DirtyPosition>();
        }
    } 
}
