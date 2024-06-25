use bevy::prelude::*;

#[derive(Component)]
pub struct Tower;


impl Plugin for Tower{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("Player1.png");

    let shape_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });
    

    let shape_handle = 
        meshes.add(Cuboid::default());

    commands.spawn((
        PbrBundle {
            mesh: shape_handle,
            material: shape_material.clone(),
            transform: Transform::from_xyz(
                0.0,
                0.5,
                -14.0,
            ),
            ..default()
        },
    ));
}