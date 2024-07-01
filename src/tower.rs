use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::SQRT_2;
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
    let cos_pi_8 = (2.0 + (2.0_f32).sqrt()).sqrt() / 2.0;
    let sin_pi_8 = (2.0 - (2.0_f32).sqrt()).sqrt() / 2.0;
    let rotation_quat = Quat::from_xyzw(cos_pi_8, 0.0, 0.0, sin_pi_8);
    let euler_angles: (f32, f32, f32) = rotation_quat.to_euler(EulerRot::YXZ);
    let mut tower_transform = Transform::from_xyz(
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
    ));
}