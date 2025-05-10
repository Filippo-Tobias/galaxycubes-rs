use bevy::prelude::*;

#[derive(Event)]
pub struct DroppableDropped {
    pub droppable_type: DroppableType,
    pub position: Vec3,
}

#[derive(PartialEq)]
pub enum DroppableType {
    Tower,
}

#[derive(Component)]
pub struct TowerDroppable {
    pub dragging: bool,
}

#[derive(Component)]
pub struct TowerPreview{
    pub droppable_type: DroppableType,
}
