use bevy::prelude::*;
use crate::drop_bar::DroppableDropped;
use crate::drop_bar::DroppableType;
use crate::game_camera;
use crate::game_camera::GameCamera;
use crate::tower_preview::TowerPreview;
pub struct TowerDroppablePlugin;

#[derive(Component)]
pub struct TowerDroppable {
    pub dragging: bool,
}

impl Plugin for TowerDroppablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, check_if_dragging);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tower_droppable_id = commands.spawn((
        Sprite::from_image(asset_server.load("Player1.png")),
        Transform::from_xyz(-490.0, -440.0, 1.0).with_scale(Vec3::new(2.2, 2.2, 1.0)),
    )).id();
    commands.entity(tower_droppable_id).observe(on_dragged);
    commands.entity(tower_droppable_id).insert(TowerDroppable {
        dragging: false,
    });
}

fn on_dragged(
    _dragged_events: Trigger<Pointer<Drag>>,
    mut query: Query<&mut TowerDroppable>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
) {
    //Spawn a cube as a preview of the tower if the drag just started.
    if query.single_mut().dragging == false {
        let texture_handle = asset_server.load("Player1.png");
        let shape_material = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            base_color: Color::srgba(0.5, 0.5, 0.5, 0.25),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        let mut tower_transform = game_camera::cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query);
        tower_transform.y = 0.5; // Fixed y position for the cube
        let shape_handle = meshes.add(Cuboid::default());
        commands.spawn((
            Mesh3d(shape_handle.clone()),
            MeshMaterial3d(shape_material.clone()),
            Transform::from_translation(tower_transform),
            TowerPreview{
                droppable_type: DroppableType::Tower,
            },
        ));
    }
    // After the preview is spawned, set the dragging state to true.
    query.single_mut().dragging = true;
}

fn check_if_dragging(
    windows: Query<&Window>,
    mut query: Query<&mut TowerDroppable>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev_dropped: EventWriter<DroppableDropped>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let dragging = query.single_mut().dragging;
        let point = game_camera::cursor_ray_to_plane(&windows, &camera_query, &camera_transform_query);
        let x = (point.x / 1.2).round() * 1.2;
        let y = 0.5; // Fixed y position for the cube
        let z = (point.z / 1.2).round() * 1.2;
        let new_point = Vec3::new(x, y, z);
        if dragging == true {
            query.single_mut().dragging = false;
            ev_dropped.send(DroppableDropped {
                droppable_type: DroppableType::Tower,
                position: new_point,
            });
            println!("dropped");
        } else {
            println!("not dropped");
        }
    }
    for mut tower_droppable in query.iter_mut() {
        if tower_droppable.dragging == false {
            
        } else {
            tower_droppable.dragging = true;
        }
    }
}