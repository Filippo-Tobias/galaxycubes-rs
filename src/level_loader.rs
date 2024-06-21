use bevy::prelude::*;
#[derive(Component)]
pub struct LevelLoader{
    pub editor_open: bool,
}

impl Plugin for LevelLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map);
    }
}

fn load_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        transform: Transform::from_xyz(
            0.0,
            0.0,
            -14.0,
        ),
        ..default()
    });
}
