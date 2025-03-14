use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{tower::Tower, ui::UI};

#[derive(Component)]
pub struct Dropable;
impl Plugin for Dropable {
    fn build(&self, app: &mut App) {
         app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    assetserver: Res<AssetServer>
) {
    
    let dropable_transform = Transform::from_xyz(
        -125.0,
        -280.5,
        1.0).with_scale(Vec3 { x: 3.0, y: 3.0, z: 3.0 });

    let _dropable = commands.spawn((
            SpriteBundle{texture: assetserver.load("Player1.png"), transform: dropable_transform, ..default()},
            Dropable,
    ));
}


fn check_hit(
    pointer_query: Query<&PointerInteraction>,
    tower_query: Query<&Tower>,
    ui_query: Query<&UI>,
) {
    for pointer_interaction in pointer_query.iter() {
        if let Some((entity, _hit_data)) = pointer_interaction.get_nearest_hit() {
            // Check if the nearest hit entity has `MyComponent`
            if let Ok(_tower) = tower_query.get(*entity) {
                // Entity has `MyComponent`, so you can process it here
                println!("Entity {:?} with 'Tower' was hit!", entity);
            }
        }
    }
}
