use bevy::prelude::*;
#[derive(Component)]
pub struct MiniEditor{
    pub editor_open: bool,
}

impl Plugin for MiniEditor{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_editor);
        app.add_systems(Update, launch_editor);
    }
}

fn initialize_editor(
    mut commands: Commands
) {
    commands.spawn(MiniEditor{editor_open: false});
}

fn launch_editor(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MiniEditor>,
) {
    if keys.just_pressed(KeyCode::KeyA){
        for mut editor in query.iter_mut() {
            /* if editor.editor_open == true {
                editor.editor_open = false;
            } else {
                editor.editor_open = true;
            }
            */
            editor.editor_open = !editor.editor_open;
            println!("{}", editor.editor_open.to_string());
        }
    }
}