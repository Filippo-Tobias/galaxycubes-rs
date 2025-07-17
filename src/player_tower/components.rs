use bevy::{picking::pointer::Location, prelude::*};

#[derive(Component)]
pub struct Tower;

#[derive(Event)]
#[allow(dead_code)]
pub struct TowerHovered {
    pub entity: Entity,
    pub position: Location,
}
#[derive(Event)]
#[allow(dead_code)]
pub struct TowerUnHovered {
    pub entity: Entity,
    pub position: Location,
}
#[derive(Event)]
pub struct TowerDragged{
    pub entity: Entity,
}

