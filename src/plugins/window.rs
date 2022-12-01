use bevy::{prelude::*, window::close_on_esc};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Bevy Game".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(close_on_esc);

    }
}

fn setup(
    mut window: ResMut<Windows>,
) {
    window.primary_mut().set_cursor_lock_mode(true);
}