use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use pointer::Location;

#[derive(Component)]
pub struct Tower;

#[derive(Event)]
pub struct TowerHovered {
    pub entity: Entity,
    pub position: Location,
}
#[derive(Event)]
pub struct TowerUnHovered {
    pub entity: Entity,
    pub position: Location,
}

impl From<ListenerInput<Pointer<Out>>> for TowerUnHovered {
    fn from(input: ListenerInput<Pointer<Out>>) -> Self {
        TowerUnHovered {
            entity: input.target,
            position: input.pointer_location.clone()
        }
    }
}

impl From<ListenerInput<Pointer<Over>>> for TowerHovered {
    fn from(input: ListenerInput<Pointer<Over>>) -> Self {
        TowerHovered {
            entity: input.target,
            position: input.pointer_location.clone()
        }
    }
}

impl Plugin for Tower{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_event::<TowerHovered>();
        app.add_event::<TowerUnHovered>();
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
    let cos_pi_8 = (2.0 + (2.0_f32).sqrt()).sqrt() / 2.0;
    let sin_pi_8 = (2.0 - (2.0_f32).sqrt()).sqrt() / 2.0;
    let rotation_quat = Quat::from_xyzw(cos_pi_8, 0.0, 0.0, sin_pi_8);
    let euler_angles: (f32, f32, f32) = rotation_quat.to_euler(EulerRot::YXZ);
    let tower_transform = Transform::from_xyz(
        0.0, 
        0.5, 
        -10.0);
        //tower_transform.rotation = rotation_quat;
    let rotation_x_radians: f32 = euler_angles.1;
    let _rotation_x_degrees: f32 = rotation_x_radians.to_degrees();
    //println!("{}",rotation_x_degrees);
    
    commands.spawn((
        PbrBundle {
            mesh: shape_handle,
            material: shape_material.clone(),
            transform: tower_transform,
            ..default()
        },
        On::<Pointer<Over>>::send_event::<TowerHovered>(),
        On::<Pointer<Out>>::send_event::<TowerUnHovered>(),
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.rotate_local_y(drag.delta.x / 50.0)
        }),
    ));
}