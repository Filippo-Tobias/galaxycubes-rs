use bevy::prelude::*;
use super::components::*;
use super::systems::*;
pub struct DragAndDrop;

impl Plugin for DragAndDrop {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, check_if_dragging);
        app.add_event::<DroppableDropped>();
    }   
}

