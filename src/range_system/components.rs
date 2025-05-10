use bevy::prelude::*;
use std::ops::RangeInclusive;

#[derive(Component)]
pub struct DirtyPosition;

#[derive(Component)]
pub struct RangeArea{
    pub range: (RangeInclusive<i32>, RangeInclusive<i32>),
    pub entities: Vec<Entity>
}

