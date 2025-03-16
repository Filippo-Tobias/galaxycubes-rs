use bevy::prelude::*;

pub struct DropBarPlugin;

impl  Plugin for DropBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_event::<DroppableDropped>();
    }   
}

#[derive(Event)]
pub struct DroppableDropped {
    pub droppable_type: DroppableType,
    pub position: Vec3,
}

#[derive(PartialEq)]
pub enum DroppableType {
    Tower,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut transform = Transform::from_xyz(-490.0, -440.0, -1.0);
    transform.scale=Vec3{x: 0.6, y: 0.6, z: 1.0};
    commands.spawn((
        Sprite::from_image(asset_server.load("drop_bar.png")),
        transform,
    ));
    
}