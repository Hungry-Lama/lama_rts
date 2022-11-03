use bevy::{window::close_on_esc, prelude::*};

fn main () {
    App::new()
      .insert_resource(Msaa { samples: 4 })
      .insert_resource(WindowDescriptor {
          title: "Bevy Game".to_string(),
          width: 1280.,
          height: 720.,
          ..Default::default()
      })
      .add_startup_system(setup)
      .add_system(close_on_esc)
      .add_plugins(DefaultPlugins)
    .run();
}

fn setup(
    mut commands: Commands,
) {
  commands
    .spawn_bundle(Camera3dBundle::default())
    .commands();
    // .spawn_bundle(UiCameraBundle::default());
}