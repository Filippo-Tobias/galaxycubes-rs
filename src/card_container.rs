use bevy::prelude::*;

#[derive(Component)]
pub struct CardContainer;

impl Plugin for CardContainer {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_card_container);
    }
}

fn spawn_card_container(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sprite = asset_server.load("Player1.png");
    commands.spawn((SpriteBundle {
        texture: sprite,
        ..Default::default()
    }, CardContainer));
}
