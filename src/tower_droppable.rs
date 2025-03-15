use bevy::prelude::*;
use crate::drop_bar::DroppableDropped;
use crate::drop_bar::DroppableType;
use crate::game_camera::GameCamera;
pub struct TowerDroppable;

#[derive(Component)]
pub struct TowerDroppableProperties {
    pub dragging: bool,
}

impl Plugin for TowerDroppable {
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
    commands.entity(tower_droppable_id).insert(TowerDroppableProperties {
        dragging: false,
    });
}

fn on_dragged(
    _dragged_events: Trigger<Pointer<Drag>>,
    mut query: Query<&mut TowerDroppableProperties>,
) {
    query.single_mut().dragging = true;
}

fn check_if_dragging(
    windows: Query<&Window>,
    mut query: Query<&mut TowerDroppableProperties>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev_dropped: EventWriter<DroppableDropped>,
    camera_query: Query<&Camera, With<GameCamera>>,
    camera_transform_query: Query<&GlobalTransform, With<GameCamera>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let dragging = query.single_mut().dragging;
        let camera = camera_query.single();
        let camera_transform = camera_transform_query.single();
        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        let Some(distance) = ray.intersect_plane(Vec3{x:0.0,y:0.0,z:0.0}, InfinitePlane3d::new(Vec3{x:0.0,y:1.0,z:0.0}))
        else {
            return;
        };
        let point = ray.get_point(distance);
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
    for mut properties in query.iter_mut() {
        if properties.dragging == false {
            
        } else {
            properties.dragging = true;
        }
    }
}