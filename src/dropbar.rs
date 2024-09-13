use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};
use bevy_mod_picking::prelude::*;

pub struct DropBar;
impl Plugin for DropBar {
    fn build(&self, app: &mut App) {
         app.add_systems(Startup, setup);
         //app.add_event::<UnhoveredUI>();
         //app.add_event::<HoveredUI>();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let dropbar_transform = Transform::from_xyz(
        -125.0,
        -280.5,
        0.0).with_scale(Vec3 { x: 2.0, y: 2.0, z: 2.0 });

    commands.spawn(Camera2dBundle{
        camera: Camera{
            is_active: true, order: 1, target: RenderTarget::Window(WindowRef::Primary), ..default()
        }, ..default()
    });

    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("dropbar.png"), transform: dropbar_transform.with_scale(Vec3 { x: 2.0, y: 2.0, z: 2.0 }), ..default() 
        },

        //On::<Pointer<Over>>::send_event::<HoveredUI>(),
        //On::<Pointer<Out>>::send_event::<UnhoveredUI>(),

    ));
}

