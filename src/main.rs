use bevy::{window::close_on_esc, prelude::*};
// Exemple to I18n import
use rust_i18n::t;

rust_i18n::i18n!("locales");

fn main () {
    App::new()
      .insert_resource(Msaa { samples: 4 })
      .insert_resource(WindowDescriptor {
          title: "Bevy Game".to_string(),
          width: 1280.,
          height: 720.,
          ..Default::default()
      })
      .add_startup_system(spawn_basic_scene)
      .add_startup_system(setup)
      .add_system_set(
        SystemSet::new()
        .with_system(move_camera),
      )
      .add_system(close_on_esc)
      .add_plugins(DefaultPlugins)
    .run();
}

fn setup(
    mut commands: Commands,
) {
  commands
    .spawn_bundle(Camera3dBundle{
      transform: Transform::from_xyz(0.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .commands();
    // .spawn_bundle(UiCameraBundle::default());
}

fn spawn_basic_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      ..default()
  });

  commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
  });
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
    transform: Transform::from_xyz(2.0, 0.5, 2.0),
    ..default()
  });
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
    transform: Transform::from_xyz(-1.0, 0.5, 1.0),
    ..default()
  });
}

fn move_camera(
  keyboard_input: Res<Input<KeyCode>>,
  mut transforms: Query<&mut Transform, With<Camera3d>>,
  time: Res<Time>,
) {
  let mut translation = Vec3::default();

  if keyboard_input.pressed(KeyCode::Up) {
      translation.z -= 1.0;
  }
  if keyboard_input.pressed(KeyCode::Down) {
      translation.z += 1.0;
  }
  if keyboard_input.pressed(KeyCode::Right) {
      translation.x += 1.0;
  }
  if keyboard_input.pressed(KeyCode::Left) {
      translation.x -= 1.0;
  }

  for mut transform in transforms.iter_mut() {
    transform.translation += translation;

  }
}